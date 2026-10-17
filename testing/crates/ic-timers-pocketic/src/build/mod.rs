//! Consumer recipes; IC Build owns Cargo execution and exact Wasm artifacts.

use ic_build::artifacts::{ArtifactCachePrunePolicy, WasmBuildSpec, build_wasm_canisters_cached};
use ic_build::cargo::selection::{CargoFeatureSelection, CargoSelection};
use std::{path::Path, process::Command, time::Duration};

pub(crate) const COHORTS: [(&str, &str); 4] = [
    ("baseline", "IC_TIMERS_COHORT_BASELINE_WASM"),
    ("once", "IC_TIMERS_COHORT_ONCE_WASM"),
    ("after-completion", "IC_TIMERS_COHORT_AFTER_COMPLETION_WASM"),
    ("watchdog", "IC_TIMERS_COHORT_WATCHDOG_WASM"),
];

pub(crate) fn run(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()?;
    let mut tests = Command::new("cargo");
    tests
        .current_dir(&root)
        .args(["test", "-p", "ic-timers-pocketic", "--locked"]);
    let mut builds = Vec::new();
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
        let spec = WasmBuildSpec::new(&root, &target, &[package])
            .with_isolated_builds()
            .with_prune_policy(
                ArtifactCachePrunePolicy::new()
                    .with_max_age(Duration::from_secs(7 * 24 * 60 * 60))
                    .with_max_size_bytes(512 * 1024 * 1024),
            )
            .with_cargo_program("cargo")
            .with_rustc_identity_program("rustc")
            .with_cargo_selection(selection);
        let built = build_wasm_canisters_cached(&spec)?;
        let [artifact] = built.record().artifacts() else {
            return Err(format!("expected exactly one Wasm artifact for {label}").into());
        };
        tests.env(variable, artifact);
        // Retain exact artifacts until the child tests finish reading them.
        builds.push(built);
    }
    let status = tests.status()?;
    drop(builds);
    if !status.success() {
        return Err(format!("{mode} PocketIC subjects failed: {status}").into());
    }
    Ok(())
}
