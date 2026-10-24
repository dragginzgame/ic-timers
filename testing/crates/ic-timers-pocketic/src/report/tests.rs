use super::*;

fn fixture_metrics(subject: &str) -> BTreeMap<String, u128> {
    let keys = if subject.starts_with("watchdog-calibration-") {
        CALIBRATION_METRICS
    } else {
        POLICY_METRICS
    };
    keys.iter().map(|key| ((*key).into(), 100)).collect()
}

#[test]
fn missing_or_incomplete_candidate_never_replaces_a_previous_report() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "timer-report-admission")
            .unwrap();
    let output = fixture.path().join("previous.json");
    let candidate = fixture.path().join("new-invocation.json");
    let expected = Report {
        schema: 1,
        context: BTreeMap::from([("lock_sha256".into(), "this invocation".into())]),
        artifacts: crate::build::COHORTS
            .iter()
            .map(|(name, _)| {
                (
                    (*name).into(),
                    BTreeMap::from([("wasm_sha256".into(), format!("{name} bytes"))]),
                )
            })
            .collect(),
        measurements: BTreeMap::new(),
    };
    let mut complete: Report =
        serde_json::from_slice(&serde_json::to_vec(&expected).unwrap()).unwrap();
    for subject in crate::build::COHORTS.iter().map(|(name, _)| *name).chain([
        "watchdog-calibration-minimal",
        "watchdog-calibration-representative",
    ]) {
        complete
            .measurements
            .insert(subject.into(), fixture_metrics(subject));
    }
    let previous = serde_json::to_vec_pretty(&complete).unwrap();
    fs::write(&output, &previous).unwrap();
    // Even a previous report with identical source context is never a fallback
    // for this invocation's missing evidence.
    assert!(Report::read_candidate(&candidate, &expected).is_err());
    for change in [
        "empty",
        "missing-subject",
        "missing-metric",
        "unknown-metric",
        "empty-metrics",
        "context",
        "artifact",
        "schema",
        "malformed",
    ] {
        let mut changed: Report = serde_json::from_slice(&previous).unwrap();
        match change {
            "empty" => changed.measurements.clear(),
            "missing-subject" => {
                changed.measurements.remove("watchdog-calibration-minimal");
            }
            "empty-metrics" => changed.measurements.get_mut("once").unwrap().clear(),
            "missing-metric" => {
                changed
                    .measurements
                    .get_mut("once")
                    .unwrap()
                    .remove("wasm_bytes");
            }
            "unknown-metric" => {
                changed
                    .measurements
                    .get_mut("once")
                    .unwrap()
                    .insert("instructions".into(), 1);
            }
            "context" => {
                changed
                    .context
                    .insert("lock_sha256".into(), "other invocation".into());
            }
            "artifact" => {
                changed
                    .artifacts
                    .get_mut("once")
                    .unwrap()
                    .insert("wasm_sha256".into(), "other bytes".into());
            }
            "schema" => changed.schema = 2,
            _ => {}
        }
        let bytes = if change == "malformed" {
            b"not JSON".to_vec()
        } else {
            serde_json::to_vec(&changed).unwrap()
        };
        fs::write(&candidate, bytes).unwrap();
        assert!(
            Report::read_candidate(&candidate, &expected).is_err(),
            "{change}"
        );
        assert_eq!(fs::read(&output).unwrap(), previous);
    }
    fs::write(&candidate, &previous).unwrap();
    let admitted = Report::read_candidate(&candidate, &expected).unwrap();
    admitted.publish(&output).unwrap();
    assert_eq!(fs::read(&output).unwrap(), previous);
}

#[test]
fn report_publication_replaces_the_entry_without_truncating_hard_linked_bytes() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &std::env::temp_dir(),
        "timer-report-publication",
    )
    .unwrap();
    let baseline = fixture.path().join("baseline.json");
    let output = fixture.path().join("candidate.json");
    let before = b"reviewed baseline";
    fs::write(&baseline, before).unwrap();
    fs::hard_link(&baseline, &output).unwrap();
    let report = Report {
        schema: 1,
        context: BTreeMap::new(),
        artifacts: BTreeMap::new(),
        measurements: BTreeMap::new(),
    };
    report.publish(&output).unwrap();
    assert_eq!(fs::read(&baseline).unwrap(), before);
    assert_eq!(
        fs::read(&output).unwrap(),
        serde_json::to_vec_pretty(&report).unwrap()
    );
    let decoded: Report = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(decoded.schema, 1);
}

#[test]
fn receipt_round_trip_preserves_costs_and_rejects_incomparable_contexts() {
    let mut baseline = Report {
        schema: 1,
        context: BTreeMap::from([("rustc".into(), "qualified compiler".into())]),
        artifacts: crate::build::COHORTS
            .iter()
            .map(|(cohort, _)| ((*cohort).into(), BTreeMap::new()))
            .collect(),
        measurements: crate::build::COHORTS
            .iter()
            .map(|(name, _)| *name)
            .chain([
                "watchdog-calibration-minimal",
                "watchdog-calibration-representative",
            ])
            .map(|name| (name.into(), fixture_metrics(name)))
            .collect(),
    };
    let mut current: Report =
        serde_json::from_slice(&serde_json::to_vec(&baseline).unwrap()).unwrap();
    current
        .measurements
        .get_mut("once")
        .unwrap()
        .insert("wasm_bytes".into(), 110);
    assert_eq!(current.deltas(&baseline).unwrap()["once/wasm_bytes"], 10);
    // Source/artifact identities change with the code under comparison. They
    // remain attributable without making every code change incomparable.
    current
        .artifacts
        .get_mut("once")
        .unwrap()
        .insert("wasm_sha256".into(), "new bytes".into());
    assert!(current.deltas(&baseline).is_ok());
    for key in [
        "rustc",
        "cargo",
        "server_sha256",
        "lock_sha256",
        "host",
        "profile",
        "target",
    ] {
        let previous = baseline
            .context
            .insert(key.into(), "different context".into());
        assert!(current.deltas(&baseline).is_err(), "must reject {key}");
        if let Some(previous) = previous {
            baseline.context.insert(key.into(), previous);
        } else {
            baseline.context.remove(key);
        }
    }
    baseline
        .measurements
        .get_mut("once")
        .unwrap()
        .remove("wasm_bytes");
    assert!(current.deltas(&baseline).is_err());
    baseline.measurements.clone_from(&current.measurements);
    baseline.artifacts.remove("once");
    assert!(current.deltas(&baseline).is_err());
    baseline.artifacts.clone_from(&current.artifacts);
    baseline.schema = 2;
    assert!(current.deltas(&baseline).is_err());
}
