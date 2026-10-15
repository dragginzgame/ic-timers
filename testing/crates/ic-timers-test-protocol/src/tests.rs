use super::{CohortObservation, ProbeMemorySummary, ProbeSnapshot};
use candid::{CandidType, Decode, Encode};

#[test]
fn borrowed_cohort_label_decodes_into_owned_text_with_wide_totals() {
    assert_eq!(
        CohortObservation::<&str>::ty(),
        CohortObservation::<String>::ty()
    );
    let observed = CohortObservation {
        cohort: "watchdog",
        scheduler_instruction_total: u128::from(u64::MAX) + 1,
        work_instruction_total: u128::MAX,
        ..CohortObservation::default()
    };
    let encoded = Encode!(&observed).expect("encode borrowed cohort observation");
    let decoded =
        Decode!(&encoded, CohortObservation<String>).expect("decode owned cohort observation");
    assert_eq!(decoded.cohort, "watchdog");
    assert_eq!(
        decoded.scheduler_instruction_total,
        u128::from(u64::MAX) + 1
    );
    assert_eq!(decoded.work_instruction_total, u128::MAX);
}

#[test]
fn snapshot_round_trip_preserves_wide_totals_and_optional_memory() {
    let observed = ProbeSnapshot {
        registration_id: Some((7, 11, 13)),
        registered: true,
        next_deadline_ns: Some(u64::MAX),
        scheduler_instruction_total: u128::from(u64::MAX) + 1,
        work_instruction_total: u128::MAX,
        scheduler_memory: Some(ProbeMemorySummary {
            samples: 1,
            latest_wasm_start_pages: 2,
            latest_wasm_end_pages: 3,
            latest_stable_start_pages: 5,
            latest_stable_end_pages: 8,
            maximum_wasm_growth_pages: 1,
            maximum_stable_growth_pages: 3,
        }),
        work_memory: Some(ProbeMemorySummary {
            samples: 2,
            latest_wasm_start_pages: 3,
            latest_wasm_end_pages: 5,
            latest_stable_start_pages: 8,
            latest_stable_end_pages: 13,
            maximum_wasm_growth_pages: 2,
            maximum_stable_growth_pages: 5,
        }),
        ..ProbeSnapshot::default()
    };
    let encoded = Encode!(&observed).expect("encode wide probe snapshot");
    assert_eq!(Decode!(&encoded, ProbeSnapshot).unwrap(), observed);
}
