use super::*;

#[test]
fn receipt_round_trip_preserves_costs_and_rejects_incomparable_contexts() {
    let mut baseline = Report {
        schema: 1,
        context: BTreeMap::from([("rustc".into(), "qualified compiler".into())]),
        artifacts: crate::build::COHORTS
            .iter()
            .map(|(cohort, _)| ((*cohort).into(), BTreeMap::new()))
            .collect(),
        measurements: BTreeMap::from([(
            "once".into(),
            BTreeMap::from([("wasm_bytes".into(), 100)]),
        )]),
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
