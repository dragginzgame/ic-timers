use super::harness::REPLY_LIMITS;
use super::harness::{fresh_pocket_ic, install_probe, update_unit};
use ic_platform::Principal;
use ic_testkit::pic::{PocketIc, advance_and_tick, query_candid_args, update_candid_args};
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
        assert_eq!(scheduled.schedule_requests.total, 0);
    } else {
        assert!(scheduled.registered);
        assert_eq!(scheduled.schedule_requests.total, 2);
        assert_eq!(scheduled.wakeups_armed.total, 1);
        assert_eq!(scheduled.coalesced.total, 1);
        assert!(scheduled.next_deadline_ns.is_some());
    }

    let cycles_before_dispatch = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let dispatch_cycles = cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id));
    let completed = observe(pic, canister_id);
    match cohort {
        "baseline" => {
            assert_eq!(completed.callbacks, 0);
            assert_eq!(completed.work_started.total, 0);
        }
        "once" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
            assert_eq!(completed.next_deadline_ns, None);
            assert_eq!(completed.work_instruction_samples, 1);
        }
        "after-completion" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
            assert!(completed.next_deadline_ns.is_some());
            assert_eq!(completed.work_instruction_samples, 1);
        }
        "watchdog" => {
            assert_eq!(completed.callbacks, 1);
            assert_eq!(completed.scheduler_started.total, 1);
            assert_eq!(completed.work_dispatched.total, 1);
            assert_eq!(completed.work_started.total, 1);
            assert_eq!(completed.work_completed.total, 1);
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
    advance_and_tick(pic, Duration::ZERO, 16);
    let cancelled = observe(pic, canister_id);
    assert_eq!(cancelled.callbacks, completed.callbacks);
    assert_eq!(cancelled.next_deadline_ns, None);

    let memory_sampling = if cohort == "watchdog" {
        let measurement: MemorySamplingMeasurement = update_candid_args(
            pic,
            canister_id,
            Principal::anonymous(),
            "memory_sampling_overhead",
            (),
            REPLY_LIMITS,
        )
        .map(|(value,)| value)
        .expect("probe update succeeds");
        assert!(measurement.sampled_bracket_instructions >= measurement.empty_bracket_instructions);
        Some(measurement)
    } else {
        None
    };
    let (memory_sampling_empty, memory_sampling_sampled, memory_sampling_overhead) =
        memory_sampling.map_or((0, 0, 0), |measurement| {
            (
                measurement.empty_bracket_instructions,
                measurement.sampled_bracket_instructions,
                measurement
                    .sampled_bracket_instructions
                    .saturating_sub(measurement.empty_bracket_instructions),
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
    advance_and_tick(pic, Duration::ZERO, 16);
    let dispatch_cycles = cycles_before_dispatch.saturating_sub(pic.cycle_balance(canister_id));
    let completed = observe(pic, canister_id);
    assert_eq!(completed.scheduler_instruction_samples, 1);
    assert_eq!(completed.work_instruction_samples, 1);
    assert_eq!(completed.work_completed.total, 1);

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
    let (_, variable) = crate::build::COHORTS
        .iter()
        .find(|(label, _)| *label == cohort)
        .expect("known policy cohort");
    let path = env::var_os(variable)
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("{variable} must name the resolved cohort Wasm artifact"));
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn update_measurement(
    pic: &PocketIc,
    canister_id: Principal,
    method: &str,
) -> OperationMeasurement {
    update_candid_args(
        pic,
        canister_id,
        Principal::anonymous(),
        method,
        (),
        REPLY_LIMITS,
    )
    .map(|(value,)| value)
    .expect("probe update succeeds")
}

fn observe(pic: &PocketIc, canister_id: Principal) -> CohortObservation {
    let observed: CohortObservation = query_candid_args(
        pic,
        canister_id,
        Principal::anonymous(),
        "observe",
        (),
        REPLY_LIMITS,
    )
    .map(|(value,)| value)
    .expect("probe query succeeds");
    assert!(!observed.schedule_requests.total_overflowed);
    assert!(!observed.scheduler_started.total_overflowed);
    assert!(!observed.wakeups_armed.total_overflowed);
    assert!(!observed.work_dispatched.total_overflowed);
    assert!(!observed.work_started.total_overflowed);
    assert!(!observed.work_completed.total_overflowed);
    assert!(!observed.coalesced.total_overflowed);
    observed
}
