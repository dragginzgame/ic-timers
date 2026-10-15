use super::harness::{drive_rounds, fresh_pocket_ic, install_probe, update_unit};
use candid::Principal;
use ic_testkit::pic::{CandidCallExt, PocketIc};
use ic_timers_test_protocol::{CohortObservation, MemorySamplingMeasurement, OperationMeasurement};
use std::{env, fs, path::PathBuf, time::Duration};

#[test]
fn comparable_policy_cohorts_report_size_and_instruction_subjects() {
    for cohort in ["baseline", "once", "after-completion", "watchdog"] {
        run_cohort(cohort);
    }
    let minimal_work = run_watchdog_calibration("minimal", false);
    let representative_work = run_watchdog_calibration("representative", true);
    assert!(representative_work > minimal_work);
}

fn run_cohort(cohort: &str) {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let wasm = cohort_wasm(cohort);
    let wasm_bytes = wasm.len();
    let canister_id = install_probe(pic, wasm);

    let initial = observe(pic, canister_id);
    assert_eq!(initial.cohort, cohort);
    assert!(!initial.registered);
    assert_eq!(initial.callbacks, 0);
    assert_eq!(initial.inventory_len, 0);

    let start = update_measurement(pic, canister_id, "start");
    let duplicate = update_measurement(pic, canister_id, "ensure_again");
    assert_eq!(start.changed, cohort != "baseline");
    assert!(!duplicate.changed);

    let scheduled = observe(pic, canister_id);
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
    drive_rounds(pic, 16);
    let dispatch_cycles = cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id));
    let completed = observe(pic, canister_id);
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

    let cancellation = update_measurement(pic, canister_id, "cancel");
    assert_eq!(
        cancellation.changed,
        matches!(cohort, "after-completion" | "watchdog")
    );
    assert_eq!(observe(pic, canister_id).next_deadline_ns, None);
    assert!(!update_measurement(pic, canister_id, "cancel").changed);
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(pic, 16);
    let cancelled = observe(pic, canister_id);
    assert_eq!(cancelled.callbacks, completed.callbacks);
    assert_eq!(cancelled.next_deadline_ns, None);

    let memory_sampling = if cohort == "watchdog" {
        let measurement = memory_sampling_measurement(pic, canister_id);
        assert!(measurement.sampled_bracket_instructions >= measurement.empty_bracket_instructions);
        assert_eq!(
            measurement.overhead_instructions,
            measurement
                .sampled_bracket_instructions
                .saturating_sub(measurement.empty_bracket_instructions)
        );
        Some(measurement)
    } else {
        None
    };
    let (memory_sampling_empty, memory_sampling_sampled, memory_sampling_overhead) =
        memory_sampling.map_or((0, 0, 0), |measurement| {
            (
                measurement.empty_bracket_instructions,
                measurement.sampled_bracket_instructions,
                measurement.overhead_instructions,
            )
        });

    println!(
        "ic_timers_cohort cohort={} start={} duplicate={} cancel={} snapshot={} inventory={} scheduler={} work={} memory_sampling_empty={} memory_sampling_sampled={} memory_sampling_overhead={} dispatch_cycles={} wasm_bytes={}",
        cohort,
        start.instructions,
        duplicate.instructions,
        cancellation.instructions,
        completed.snapshot_instructions,
        completed.inventory_instructions,
        completed.scheduler_instruction_total,
        completed.work_instruction_total,
        memory_sampling_empty,
        memory_sampling_sampled,
        memory_sampling_overhead,
        dispatch_cycles,
        wasm_bytes,
    );
}

fn run_watchdog_calibration(profile: &str, representative_work: bool) -> u128 {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, cohort_wasm("watchdog"));
    if representative_work {
        update_unit(pic, canister_id, "use_representative_watchdog_work");
    }
    let start = update_measurement(pic, canister_id, "start");
    assert!(start.changed);

    let cycles_before_dispatch = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(pic, 16);
    let dispatch_cycles = cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id));
    let completed = observe(pic, canister_id);
    assert_eq!(completed.scheduler_instruction_samples, 1);
    assert_eq!(completed.work_instruction_samples, 1);
    assert_eq!(completed.work_completed, 1);

    println!(
        "ic_timers_watchdog_calibration profile={} total_message_instructions=unavailable scheduler={} work={} unaccounted_instructions=unavailable dispatch_cycles={}",
        profile,
        completed.scheduler_instruction_total,
        completed.work_instruction_total,
        dispatch_cycles,
    );
    completed.work_instruction_total
}

fn cohort_wasm(cohort: &str) -> Vec<u8> {
    let root = env::var_os("IC_TIMERS_COHORT_ROOT")
        .map(PathBuf::from)
        .expect("IC_TIMERS_COHORT_ROOT must name the cohort target root");
    let path = root
        .join(format!("cohort-{cohort}"))
        .join("wasm32-unknown-unknown/timer-probe/ic_timers_size_probe.wasm");
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn update_measurement(
    pic: &PocketIc,
    canister_id: Principal,
    method: &str,
) -> OperationMeasurement {
    pic.update_candid(canister_id, method, ())
        .expect("probe update succeeds")
}

fn memory_sampling_measurement(
    pic: &PocketIc,
    canister_id: Principal,
) -> MemorySamplingMeasurement {
    pic.update_candid(canister_id, "memory_sampling_overhead", ())
        .expect("probe update succeeds")
}

fn observe(pic: &PocketIc, canister_id: Principal) -> CohortObservation {
    pic.query_candid(canister_id, "observe", ())
        .expect("probe query succeeds")
}
