use super::{CohortObservation, CounterObservation, ProbeMemorySummary, ProbeSnapshot};
use ic_platform::candid::{
    DecodeLimits,
    contract::{CandidType, encode_args},
    decode_args,
};

// The protocol subjects use the same finite ceilings as the native probe driver.
const LIMITS: DecodeLimits = DecodeLimits {
    reply_bytes: 64 * 1024,
    decoding_quota: 1_000_000,
    skipping_quota: 10_000,
    type_entries: 256,
    header_bytes: 8 * 1024,
};

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
    let encoded = encode_args((&observed,)).expect("encode borrowed cohort observation");
    let (decoded,) = decode_args::<(CohortObservation<String>,)>(&encoded, LIMITS)
        .expect("decode owned cohort observation");
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
        schedule_requests: CounterObservation {
            total: u128::from(u64::MAX) + 1,
            total_overflowed: false,
        },
        work_started: CounterObservation {
            total: u128::MAX,
            total_overflowed: false,
        },
        work_completed: CounterObservation {
            total: u128::MAX,
            total_overflowed: true,
        },
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
    let encoded = encode_args((&observed,)).expect("encode wide probe snapshot");
    assert_eq!(
        decode_args::<(ProbeSnapshot,)>(&encoded, LIMITS).unwrap().0,
        observed
    );
}
