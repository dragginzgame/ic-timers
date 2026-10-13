use super::CohortObservation;
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
