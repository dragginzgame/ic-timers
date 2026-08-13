use candid::{CandidType, Decode, Encode, Principal};
use pocket_ic::PocketIc;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, time::Duration};

const INIT_CYCLES: u128 = 2_000_000_000_000;

#[derive(CandidType, Debug, Deserialize)]
struct OperationMeasurement {
    changed: bool,
    instructions: u64,
}

#[derive(CandidType, Debug, Deserialize)]
struct CohortObservation {
    cohort: String,
    registered: bool,
    callbacks: u64,
    next_deadline_ns: Option<u64>,
    schedule_requests: u64,
    wakeups_armed: u64,
    work_dispatched: u64,
    scheduler_started: u64,
    work_started: u64,
    work_completed: u64,
    coalesced: u64,
    scheduler_instruction_samples: u64,
    scheduler_instruction_total: u64,
    work_instruction_samples: u64,
    work_instruction_total: u64,
    snapshot_instructions: u64,
    inventory_len: u64,
    inventory_instructions: u64,
}

#[test]
fn comparable_policy_cohorts_report_size_and_instruction_subjects() {
    for cohort in ["baseline", "once", "after-completion", "watchdog"] {
        run_cohort(cohort);
    }
}

fn run_cohort(cohort: &str) {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        cohort_wasm(cohort),
        Encode!().expect("encode cohort init"),
        None,
    );

    let initial = observe(&pic, canister_id);
    assert_eq!(initial.cohort, cohort);
    assert!(!initial.registered);
    assert_eq!(initial.callbacks, 0);
    assert_eq!(initial.inventory_len, 0);

    let start = update_measurement(&pic, canister_id, "start");
    let duplicate = update_measurement(&pic, canister_id, "ensure_again");
    assert_eq!(start.changed, cohort != "baseline");
    assert!(!duplicate.changed);

    let scheduled = observe(&pic, canister_id);
    if cohort == "baseline" {
        assert!(!scheduled.registered);
        assert_eq!(scheduled.schedule_requests, 0);
    } else {
        assert!(scheduled.registered);
        assert_eq!(scheduled.schedule_requests, 2);
        assert_eq!(scheduled.wakeups_armed, 1);
        assert_eq!(scheduled.coalesced, 1);
        assert!(scheduled.next_deadline_ns.is_some());
    }

    let cycles_before_dispatch = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 16);
    let dispatch_cycles = cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id));
    let completed = observe(&pic, canister_id);
    match cohort {
        "baseline" => {
            assert_eq!(completed.callbacks, 0);
            assert_eq!(completed.work_started, 0);
        }
        "once" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started, 1);
            assert_eq!(completed.work_completed, 1);
            assert_eq!(completed.next_deadline_ns, None);
            assert_eq!(completed.work_instruction_samples, 1);
        }
        "after-completion" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started, 1);
            assert_eq!(completed.work_completed, 1);
            assert!(completed.next_deadline_ns.is_some());
            assert_eq!(completed.work_instruction_samples, 1);
        }
        "watchdog" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.scheduler_started, 1);
            assert_eq!(completed.work_dispatched, 1);
            assert_eq!(completed.work_started, 1);
            assert_eq!(completed.work_completed, 1);
            assert!(completed.next_deadline_ns.is_some());
            assert_eq!(completed.scheduler_instruction_samples, 1);
            assert_eq!(completed.work_instruction_samples, 1);
        }
        _ => unreachable!("fixed cohort list"),
    }

    let cancellation = update_measurement(&pic, canister_id, "cancel");
    assert_eq!(
        cancellation.changed,
        matches!(cohort, "after-completion" | "watchdog")
    );
    assert_eq!(observe(&pic, canister_id).next_deadline_ns, None);

    println!(
        "ic_timers_cohort cohort={} start={} duplicate={} cancel={} snapshot={} inventory={} scheduler={} work={} dispatch_cycles={}",
        cohort,
        start.instructions,
        duplicate.instructions,
        cancellation.instructions,
        completed.snapshot_instructions,
        completed.inventory_instructions,
        completed.scheduler_instruction_total,
        completed.work_instruction_total,
        dispatch_cycles,
    );
}

fn cohort_wasm(cohort: &str) -> Vec<u8> {
    let root = env::var_os("IC_TIMERS_COHORT_ROOT")
        .map(PathBuf::from)
        .expect("IC_TIMERS_COHORT_ROOT must name the cohort target root");
    let path = root
        .join(format!("cohort-{cohort}"))
        .join("wasm32-unknown-unknown/release/ic_timers_size_probe.wasm");
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn drive_rounds(pic: &PocketIc, count: usize) {
    for _ in 0..count {
        pic.tick();
    }
}

fn update_measurement(
    pic: &PocketIc,
    canister_id: Principal,
    method: &str,
) -> OperationMeasurement {
    let bytes = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!().expect("encode cohort update"),
        )
        .unwrap_or_else(|error| panic!("update {method}: {error:?}"));
    Decode!(&bytes, OperationMeasurement).unwrap_or_else(|error| panic!("decode {method}: {error}"))
}

fn observe(pic: &PocketIc, canister_id: Principal) -> CohortObservation {
    let bytes = pic
        .query_call(
            canister_id,
            Principal::anonymous(),
            "observe",
            Encode!().expect("encode cohort query"),
        )
        .unwrap_or_else(|error| panic!("query cohort observation: {error:?}"));
    Decode!(&bytes, CohortObservation).expect("decode cohort observation")
}
