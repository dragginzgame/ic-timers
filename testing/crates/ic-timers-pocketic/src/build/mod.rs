//! Consumer recipes; IC Build owns Cargo execution and exact Wasm artifacts.

use ic_build::artifacts::{
    CacheRetentionLimits, WasmBuildSpec, build_wasm_canisters_cached, resolve_cargo_build_inputs,
};
use ic_build::cargo::selection::{CargoFeatureSelection, CargoSelection};
use ic_host_process::{
    child::CleanupPolicy,
    tool::{OutputLimit, OutputLimits, ToolError, run_group_command},
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
    if let (Some(output), Some(baseline)) = (&report_path, &baseline)
        && output.canonicalize().ok().as_ref() == Some(&baseline.canonicalize()?)
    {
        return Err("report output must not overwrite the baseline".into());
    }
    let mut tests = Command::new("cargo");
    tests
        .current_dir(&root)
        .args(["test", "-p", "ic-timers-pocketic", "--locked"]);
    let mut builds = Vec::new();
    let mut inputs = Vec::new();
    let mut report = crate::report::Report::new(&root)?;
    let recipes = if mode == "watchdog" {
        tests.arg("tests::");
        vec![(
            "runtime",
            "IC_TIMERS_PROBE_WASM",
            "ic-timers-runtime-probe",
            None,
        )]
    } else {
        tests.args([
            "comparable_policy_cohorts_report_size_and_instruction_subjects",
            "--",
            "--nocapture",
        ]);
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
    if let Some(path) = &report_path {
        tests.env("IC_TIMERS_COHORT_REPORT", path);
        tests.env("IC_TIMERS_COHORT_CONTEXT", serde_json::to_string(&report)?);
    }
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
    if let (Some(path), Some(baseline)) = (&report_path, &baseline) {
        crate::report::compare(path, baseline)?;
    }
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
