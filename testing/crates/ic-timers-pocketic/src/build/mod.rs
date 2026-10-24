//! Consumer recipes; IC Build owns Cargo execution and exact Wasm artifacts.

use ic_build::artifacts::{
    CacheRetentionLimits, WasmBuildSpec, build_wasm_canisters_cached, resolve_cargo_build_inputs,
};
use ic_build::cargo::selection::{CargoFeatureSelection, CargoSelection};
use ic_host_process::{
    child::CleanupPolicy,
    tool::{OutputLimit, OutputLimits, ToolError, capture_command, run_group_command},
};
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

pub(crate) const COHORTS: [(&str, &str); 4] = [
    ("baseline", "IC_TIMERS_COHORT_BASELINE_WASM"),
    ("once", "IC_TIMERS_COHORT_ONCE_WASM"),
    ("after-completion", "IC_TIMERS_COHORT_AFTER_COMPLETION_WASM"),
    ("watchdog", "IC_TIMERS_COHORT_WATCHDOG_WASM"),
];

const COHORT_SUBJECT: &str =
    "cohorts::comparable_policy_cohorts_report_size_and_instruction_subjects";

// These subjects anchor the maintained recovery contract, not incidental test counts.
const REQUIRED_WATCHDOG_SUBJECTS: &[&str] = &[
    "tests::trapped_work_keeps_committed_successor_and_executor_is_private",
    "tests::instruction_exhaustion_leaves_successor_for_later_progress",
    "tests::insufficient_cycles_defer_due_work_until_top_up",
    "tests::dispatch_response_trap_cleanup_preserves_pending_work_for_retry",
    "tests::ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery",
    "tests::ordinary_continuation_trap_retires_running_work_and_releases_expired_authority",
    "tests::accepted_ordinary_work_survives_dispatch_timeout_without_replay",
    "tests::shared_watchdog_survives_trap_and_upgrade",
    "tests::maximum_capacity_dispatch_preserves_every_survivor_under_pressure_and_await",
];

fn test_listing(command: &mut Command) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // The owning worker supplies the elapsed deadline and process group. Host
    // bounds discovery output and preserves a failed producer's status/evidence.
    let output = capture_command(
        command,
        OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(256 * 1024),
            timeout: None,
        },
    )?
    .require_success()?;
    Ok(output.stdout)
}

fn discover_subjects(
    command: &mut Command,
    mode: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let output = test_listing(command)?;
    let names = std::str::from_utf8(&output)?
        .lines()
        .map(|line| line.strip_suffix(": test").ok_or("invalid test listing"))
        .collect::<Result<Vec<_>, _>>()?;
    let admitted = match mode {
        "watchdog" => REQUIRED_WATCHDOG_SUBJECTS
            .iter()
            .all(|name| names.contains(name)),
        "cohorts" => names == [COHORT_SUBJECT],
        _ => false,
    };
    if !admitted {
        return Err(
            format!("missing {mode} qualification subjects in successful test listing").into(),
        );
    }
    // Listing includes ignored tests. Refuse ignored selected subjects rather than
    // silently qualifying a partial run or overriding an intentional ignore.
    let ignored = test_listing(command.arg("--ignored"))?;
    if !ignored.is_empty() {
        return Err("qualification selection contains ignored subjects".into());
    }
    Ok(output)
}

pub(crate) fn run(
    mode: &str,
    report_path: Option<&PathBuf>,
    baseline: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()?;
    let report_path = report_path.map(|path| root.join(path));
    let baseline = baseline.map(|path| root.join(path));
    if let (Some(output), Some(baseline)) = (&report_path, &baseline) {
        let baseline = baseline.canonicalize()?;
        ic_host_fs::durable::validate_non_aliasing(output, &[&baseline], 40)?;
    }
    let filter = match mode {
        "watchdog" => "tests::",
        "cohorts" if report_path.is_some() => COHORT_SUBJECT,
        _ => return Err("select watchdog or cohorts with a report output".into()),
    };
    // Retain failures (including a completed child's unadmitted report) without
    // ever consulting the previous final output. Host owns unique allocation.
    let mut scratch = ic_host_fs::owned_directory::OwnedDirectory::new(
        ic_host_fs::directory::create_unique_directory(
            &std::env::temp_dir().canonicalize()?,
            "timer-qualification",
            0o700,
            std::num::NonZeroUsize::new(128),
        )?,
    );
    scratch.disarm();
    eprintln!("qualification diagnostics: {}", scratch.path().display());
    let candidate = scratch.path().join("cohorts.json");
    let mut tests = Command::new("cargo");
    tests.current_dir(&root).args([
        "test",
        "-p",
        "ic-timers-pocketic",
        "--bin",
        "ic-timers-pocketic",
        "--locked",
        filter,
    ]);
    let mut discovery = Command::new(tests.get_program());
    discovery
        .current_dir(&root)
        .args(tests.get_args())
        .args(["--", "--list", "--format", "terse"]);
    if mode == "cohorts" {
        discovery.arg("--exact");
        tests.args(["--", "--exact", "--nocapture"]);
    }
    std::fs::write(
        scratch.path().join("subjects.txt"),
        discover_subjects(&mut discovery, mode)?,
    )?;
    let mut builds = Vec::new();
    let mut inputs = Vec::new();
    let mut report = crate::report::Report::new(&root)?;
    let recipes = if mode == "watchdog" {
        vec![(
            "runtime",
            "IC_TIMERS_PROBE_WASM",
            "ic-timers-runtime-probe",
            None,
        )]
    } else {
        COHORTS
            .iter()
            .map(|&(cohort, variable)| (cohort, variable, "ic-timers-size-probe", Some(cohort)))
            .collect()
    };
    for (label, variable, package, feature) in recipes {
        let target = root.join("testing/target").join(format!("probe-{label}"));
        let selection = CargoSelection {
            profile: Some("timer-probe".to_owned()),
            locked: true,
            features: CargoFeatureSelection {
                features: feature.map(str::to_owned).into_iter().collect(),
                default_features: feature.is_none(),
                ..CargoFeatureSelection::default()
            },
            ..CargoSelection::default()
        };
        let spec = WasmBuildSpec::new(&root, &target, &[package], std::env::vars_os())
            .with_isolated_builds()
            .with_prune_policy(
                CacheRetentionLimits::new()
                    .with_max_age(Duration::from_secs(7 * 24 * 60 * 60))
                    .with_max_size_bytes(512 * 1024 * 1024),
            )
            .with_cargo_program("cargo")
            .with_rustc_identity_program("rustc")
            .with_cargo_selection(selection);
        let before = resolve_cargo_build_inputs(&spec)?;
        let built =
            build_wasm_canisters_cached(&spec, ic_host_process::child::OwnedChild::spawn_direct)?;
        let [artifact] = built.record().artifacts() else {
            return Err(format!("expected exactly one Wasm artifact for {label}").into());
        };
        tests.env(variable, artifact);
        if mode == "cohorts" {
            report.record_build(label, built.record(), artifact)?;
        }
        // Retain exact artifacts until the child tests finish reading them.
        inputs.push((spec, before));
        builds.push(built);
    }
    if report_path.is_some() {
        tests.env("IC_TIMERS_COHORT_REPORT", &candidate);
        tests.env("IC_TIMERS_COHORT_CONTEXT", serde_json::to_string(&report)?);
    }
    tests.env("IC_TIMERS_DIAGNOSTICS", scratch.path());
    let status = tests.status()?;
    for ((spec, before), built) in inputs.iter().zip(&builds) {
        let after = resolve_cargo_build_inputs(spec)?;
        if before.validation_digest() != after.validation_digest()
            || before.fingerprint() != built.record().fingerprint()
            || before.fingerprint() != after.fingerprint()
        {
            return Err(
                "qualification inputs changed during measurement; discard the cohort report".into(),
            );
        }
    }
    report.verify_context(&root)?;
    drop(builds);
    if !status.success() {
        return Err(format!("{mode} PocketIC subjects failed: {status}").into());
    }
    if let Some(path) = &report_path {
        let completed = crate::report::Report::read_candidate(&candidate, &report)?;
        if let Some(baseline) = &baseline {
            crate::report::compare(&candidate, baseline)?;
        }
        completed.publish(path)?;
    }
    scratch.cleanup()?;
    Ok(())
}

// One parent process group covers builds, Cargo tests and synchronous instance
// deletion. Separately grouped Testkit servers also have their own hard TTL.
pub(crate) fn qualify(command: &mut Command, timeout: Duration) -> Result<(), ToolError> {
    run_group_command(
        command,
        OutputLimits {
            stdout: OutputLimit::Truncate(16 * 1024),
            stderr: OutputLimit::Truncate(16 * 1024),
            timeout: Some(timeout),
        },
        CleanupPolicy::TermThenKill {
            grace: Duration::from_secs(2),
            reap_timeout: Duration::from_secs(5),
        },
        None,
        || false,
    )?
    .require_success()?;
    Ok(())
}

#[cfg(all(test, unix))]
mod tests;
