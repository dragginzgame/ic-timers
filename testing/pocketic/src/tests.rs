use candid::{CandidType, Decode, Encode, Principal};
use pocket_ic::PocketIc;
use serde::Deserialize;
use std::{env, fs, path::PathBuf, time::Duration};

const TIMER_EXECUTOR_METHOD: &str = "<ic-cdk internal> timer_executor";
const INIT_CYCLES: u128 = 2_000_000_000_000;

#[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
struct ProbeMemorySummary {
    samples: u64,
    latest_wasm_start_pages: u64,
    latest_wasm_end_pages: u64,
    latest_stable_start_pages: u64,
    latest_stable_end_pages: u64,
    maximum_wasm_growth_pages: u64,
    maximum_stable_growth_pages: u64,
}

#[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
struct ProbeSnapshot {
    registration_id: Option<(u64, u64, u64)>,
    registered: bool,
    completed_work: u64,
    next_deadline_ns: Option<u64>,
    immediate_scheduling: bool,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    schedule_requests: u64,
    scheduler_started: u64,
    wakeups_armed: u64,
    work_dispatched: u64,
    work_started: u64,
    work_completed: u64,
    coalesced: u64,
    unacknowledged: u64,
    last_unacknowledged: bool,
    scheduler_instruction_samples: u64,
    scheduler_instruction_total: u64,
    work_instruction_samples: u64,
    work_instruction_total: u64,
    scheduler_memory: Option<ProbeMemorySummary>,
    work_memory: Option<ProbeMemorySummary>,
    post_upgrade_reconstructed: bool,
    secondary_registered: bool,
    secondary_completed_work: u64,
}

#[test]
fn immediate_initial_watchdog_runs_without_advancing_cadence_time() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );

    assert!(update_bool(&pic, canister_id, "start_immediately"));
    let initial = snapshot(&pic, canister_id);
    assert_eq!(initial.completed_work, 0);
    assert!(initial.immediate_scheduling);
    assert_eq!(initial.latest_requested_delay_ns, Some(0));
    assert_eq!(initial.latest_armed_delay_ns, Some(0));
    assert_eq!(initial.schedule_requests, 1);
    assert_eq!(initial.wakeups_armed, 1);

    drive_rounds(&pic, 16);
    let completed = snapshot(&pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.scheduler_started, 1);
    assert_eq!(completed.work_dispatched, 1);
    assert_eq!(completed.work_completed, 1);
    assert!(completed.next_deadline_ns.is_some());
}

#[test]
fn successful_immediate_continuation_runs_again_without_waiting_for_cadence() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));
    update_unit(&pic, canister_id, "continue_immediately_on_next_work");

    let cycles_before_first = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    let mut immediate = None;
    for _ in 0..16 {
        pic.tick();
        let current = snapshot(&pic, canister_id);
        if current.completed_work == 1 {
            immediate = Some(current);
            break;
        }
    }
    let immediate = immediate.expect("first work completion should be observable");
    assert!(immediate.immediate_scheduling);
    assert_eq!(immediate.latest_requested_delay_ns, Some(0));
    assert_eq!(immediate.latest_armed_delay_ns, Some(0));
    assert_eq!(immediate.schedule_requests, 1);
    assert_eq!(immediate.wakeups_armed, 3);
    assert_eq!(immediate.work_dispatched, 1);
    let first_dispatch_cycles = cycles_before_first.saturating_sub(pic.cycle_balance(canister_id));

    let cycles_before_second = pic.cycle_balance(canister_id);
    drive_rounds(&pic, 16);
    let continued = snapshot(&pic, canister_id);
    assert_eq!(continued.completed_work, 2);
    assert_eq!(continued.scheduler_started, 2);
    assert_eq!(continued.work_dispatched, 2);
    assert_eq!(continued.work_completed, 2);
    assert_eq!(continued.wakeups_armed, 4);
    assert_eq!(continued.schedule_requests, 1);
    assert_eq!(continued.coalesced, 0);
    let second_dispatch_cycles =
        cycles_before_second.saturating_sub(pic.cycle_balance(canister_id));
    println!(
        "ic_timers_immediate first_scheduler={} first_work={} first_cycles={} second_scheduler={} second_work={} second_cycles={}",
        immediate.scheduler_instruction_total,
        immediate.work_instruction_total,
        first_dispatch_cycles,
        continued
            .scheduler_instruction_total
            .saturating_sub(immediate.scheduler_instruction_total),
        continued
            .work_instruction_total
            .saturating_sub(immediate.work_instruction_total),
        second_dispatch_cycles,
    );
}

#[test]
#[allow(clippy::too_many_lines)] // One real-canister trap, isolation, upgrade, and measurement flow.
fn trapped_work_keeps_committed_successor_and_executor_is_private() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );

    assert!(update_bool(&pic, canister_id, "start"));
    assert!(!update_bool(&pic, canister_id, "start"));
    assert!(update_bool(&pic, canister_id, "start_secondary"));
    assert!(!update_bool(&pic, canister_id, "start_secondary"));

    let before_external = snapshot(&pic, canister_id);
    let external = pic.update_call(
        canister_id,
        Principal::anonymous(),
        TIMER_EXECUTOR_METHOD,
        0_u64.to_be_bytes().to_vec(),
    );
    assert!(
        external.is_err(),
        "provider executor must reject external ingress"
    );
    assert_eq!(snapshot(&pic, canister_id), before_external);

    let trap_at = update_u64(&pic, canister_id, "arm_trap");
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 12);
    let trapped = snapshot(&pic, canister_id);
    assert_eq!(trapped.registration_id, before_external.registration_id);
    assert_eq!(trapped.completed_work, 0);
    assert_eq!(trapped.secondary_completed_work, 1);
    assert_eq!(trapped.scheduler_started, 1);
    assert_eq!(trapped.work_dispatched, 1);
    assert_eq!(trapped.work_completed, 0);
    assert_eq!(trapped.scheduler_instruction_samples, 1);
    assert!(trapped.scheduler_instruction_total > 0);
    assert_eq!(trapped.work_instruction_samples, 0);
    let trapped_scheduler_memory = trapped
        .scheduler_memory
        .as_ref()
        .expect("normally completed scheduler must retain a memory sample");
    assert_eq!(trapped_scheduler_memory.samples, 1);
    assert_memory_summary_is_coherent(trapped_scheduler_memory);
    assert_eq!(trapped.work_memory, None);
    assert_eq!(trapped.next_deadline_ns, Some(trap_at + 1_000_000_000));

    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 12);
    let recovered = snapshot(&pic, canister_id);
    assert_eq!(recovered.registration_id, trapped.registration_id);
    assert_eq!(recovered.completed_work, 1);
    assert_eq!(recovered.secondary_completed_work, 2);
    assert_eq!(recovered.scheduler_started, 2);
    assert_eq!(recovered.work_dispatched, 2);
    assert_eq!(recovered.work_completed, 1);
    assert_eq!(recovered.unacknowledged, 1);
    assert!(!recovered.last_unacknowledged);
    assert_eq!(recovered.scheduler_instruction_samples, 2);
    assert!(recovered.scheduler_instruction_total > trapped.scheduler_instruction_total);
    assert_eq!(recovered.work_instruction_samples, 1);
    assert!(recovered.work_instruction_total > 0);
    let recovered_scheduler_memory = recovered
        .scheduler_memory
        .as_ref()
        .expect("second normal scheduler completion must retain a memory sample");
    assert_eq!(recovered_scheduler_memory.samples, 2);
    assert_memory_summary_is_coherent(recovered_scheduler_memory);
    let recovered_work_memory = recovered
        .work_memory
        .as_ref()
        .expect("normally completed work must retain a memory sample");
    assert_eq!(recovered_work_memory.samples, 1);
    assert_memory_summary_is_coherent(recovered_work_memory);
    assert!(
        recovered
            .next_deadline_ns
            .is_some_and(|deadline| deadline > trap_at)
    );

    pic.upgrade_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode upgrade"),
        None,
    )
    .expect("upgrade should succeed");
    let upgraded = snapshot(&pic, canister_id);
    assert_ne!(upgraded.registration_id, recovered.registration_id);
    assert!(upgraded.registered);
    assert!(upgraded.secondary_registered);
    assert!(upgraded.post_upgrade_reconstructed);
    assert_eq!(upgraded.completed_work, recovered.completed_work);
    assert_eq!(
        upgraded.secondary_completed_work,
        recovered.secondary_completed_work
    );
    assert_eq!(upgraded.scheduler_started, 0);
    assert_eq!(upgraded.wakeups_armed, 1);
    assert!(upgraded.next_deadline_ns.is_some());

    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 12);
    let after_upgrade = snapshot(&pic, canister_id);
    assert_eq!(after_upgrade.completed_work, recovered.completed_work + 1);
    assert_eq!(
        after_upgrade.secondary_completed_work,
        recovered.secondary_completed_work + 1
    );
    println!(
        "ic_timers_watchdog scheduler_samples={} scheduler_instructions={} work_samples={} work_instructions={} unacknowledged={}",
        recovered.scheduler_instruction_samples,
        recovered.scheduler_instruction_total,
        recovered.work_instruction_samples,
        recovered.work_instruction_total,
        recovered.unacknowledged,
    );
}

fn assert_memory_summary_is_coherent(summary: &ProbeMemorySummary) {
    assert!(summary.latest_wasm_end_pages >= summary.latest_wasm_start_pages);
    assert!(summary.latest_stable_end_pages >= summary.latest_stable_start_pages);
    assert!(
        summary.maximum_wasm_growth_pages
            >= summary
                .latest_wasm_end_pages
                .saturating_sub(summary.latest_wasm_start_pages)
    );
    assert!(
        summary.maximum_stable_growth_pages
            >= summary
                .latest_stable_end_pages
                .saturating_sub(summary.latest_stable_start_pages)
    );
}

#[test]
fn instruction_exhaustion_leaves_successor_for_later_progress() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));

    let exhaust_at = update_u64(&pic, canister_id, "arm_exhaustion");
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 30);
    let exhausted = snapshot(&pic, canister_id);
    assert_eq!(exhausted.completed_work, 0);
    assert_eq!(exhausted.scheduler_started, 1);
    assert_eq!(exhausted.work_dispatched, 1);
    assert_eq!(exhausted.work_completed, 0);
    assert_eq!(exhausted.work_instruction_samples, 0);
    let exhausted_scheduler_memory = exhausted
        .scheduler_memory
        .as_ref()
        .expect("normally completed scheduler must retain a memory sample");
    assert_eq!(exhausted_scheduler_memory.samples, 1);
    assert_memory_summary_is_coherent(exhausted_scheduler_memory);
    assert_eq!(exhausted.work_memory, None);
    assert_eq!(exhausted.next_deadline_ns, Some(exhaust_at + 1_000_000_000));

    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 16);
    let recovered = snapshot(&pic, canister_id);
    assert_eq!(recovered.completed_work, 1);
    assert_eq!(recovered.scheduler_started, 2);
    assert_eq!(recovered.work_completed, 1);
    assert_eq!(recovered.unacknowledged, 1);
    assert_eq!(recovered.work_instruction_samples, 1);
    let recovered_scheduler_memory = recovered
        .scheduler_memory
        .as_ref()
        .expect("second normal scheduler completion must retain a memory sample");
    assert_eq!(recovered_scheduler_memory.samples, 2);
    assert_memory_summary_is_coherent(recovered_scheduler_memory);
    let recovered_work_memory = recovered
        .work_memory
        .as_ref()
        .expect("normally completed recovery work must retain a memory sample");
    assert_eq!(recovered_work_memory.samples, 1);
    assert_memory_summary_is_coherent(recovered_work_memory);
}

#[test]
fn commit_window_ensure_stop_resume_and_large_overdue_time_remain_bounded() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));

    update_unit(&pic, canister_id, "stop");
    let inactive = snapshot(&pic, canister_id);
    assert_eq!(inactive.next_deadline_ns, None);
    assert!(!update_bool(
        &pic,
        canister_id,
        "ensure_from_returned_error"
    ));
    let ensured = snapshot(&pic, canister_id);
    assert!(ensured.next_deadline_ns.is_some());
    let arms_after_ensure = ensured.wakeups_armed;
    assert!(!update_bool(
        &pic,
        canister_id,
        "ensure_from_returned_error"
    ));
    assert_eq!(snapshot(&pic, canister_id).wakeups_armed, arms_after_ensure);

    pic.stop_canister(canister_id, None)
        .expect("canister stop should succeed");
    pic.advance_time(Duration::from_secs(300));
    drive_rounds(&pic, 4);
    pic.start_canister(canister_id, None)
        .expect("canister resume should succeed");
    drive_rounds(&pic, 16);
    let resumed = snapshot(&pic, canister_id);
    assert_eq!(resumed.completed_work, 1);
    assert_eq!(resumed.scheduler_started, 1);
    assert_eq!(resumed.work_dispatched, 1);
    assert_eq!(resumed.work_started, 1);
    assert_eq!(resumed.work_completed, 1);
    assert!(resumed.next_deadline_ns.is_some());

    update_unit(&pic, canister_id, "stop_on_next_work");
    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 16);
    let terminal = snapshot(&pic, canister_id);
    assert_eq!(terminal.completed_work, 2);
    assert_eq!(terminal.next_deadline_ns, None);
    pic.advance_time(Duration::from_secs(2));
    drive_rounds(&pic, 8);
    assert_eq!(snapshot(&pic, canister_id).completed_work, 2);
}

#[test]
fn insufficient_cycles_defer_due_work_until_top_up() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));
    let (_burned, call_cost) = update_u128_pair(&pic, canister_id, "burn_to_below_timer_call_cost");
    assert!(call_cost > 0);

    pic.advance_time(Duration::from_secs(1));
    drive_rounds(&pic, 8);
    let deferred = snapshot(&pic, canister_id);
    assert_eq!(deferred.completed_work, 0);
    assert_eq!(deferred.scheduler_started, 0);

    pic.add_cycles(canister_id, INIT_CYCLES);
    drive_rounds(&pic, 16);
    let progressed = snapshot(&pic, canister_id);
    assert_eq!(progressed.completed_work, 1);
    assert_eq!(progressed.scheduler_started, 1);
}

#[test]
fn full_inventory_is_bounded_deterministic_and_measured() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));
    update_unit(&pic, canister_id, "stop");
    assert_eq!(update_u64(&pic, canister_id, "fill_inventory"), 64);
    assert_eq!(update_u64(&pic, canister_id, "fill_inventory"), 64);

    let (len, instructions, ordered) = query_inventory_measurement(&pic, canister_id);
    assert_eq!(len, 64);
    assert!(instructions > 0);
    assert!(ordered);
    println!("ic_timers_inventory entries={len} instructions={instructions}");
}

#[test]
fn cancellation_between_scheduler_and_work_makes_late_provider_delivery_harmless() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));
    pic.advance_time(Duration::from_secs(1));

    let mut dispatched = None;
    for _ in 0..12 {
        pic.tick();
        let current = snapshot(&pic, canister_id);
        if current.work_dispatched == 1 && current.work_started == 0 {
            dispatched = Some(current);
            break;
        }
    }
    let dispatched = dispatched.expect("scheduler/work message boundary must be observable");
    assert_eq!(dispatched.completed_work, 0);
    assert!(dispatched.next_deadline_ns.is_some());

    update_unit(&pic, canister_id, "stop");
    drive_rounds(&pic, 12);
    let cancelled = snapshot(&pic, canister_id);
    assert_eq!(cancelled.completed_work, 0);
    assert_eq!(cancelled.work_started, 0);
    assert_eq!(cancelled.work_completed, 0);
    assert_eq!(cancelled.next_deadline_ns, None);
}

fn probe_wasm() -> Vec<u8> {
    let path = env::var_os("IC_TIMERS_PROBE_WASM")
        .map(PathBuf::from)
        .expect("IC_TIMERS_PROBE_WASM must name the built runtime probe");
    fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn drive_rounds(pic: &PocketIc, count: usize) {
    for _ in 0..count {
        pic.tick();
    }
}

fn update_bool(pic: &PocketIc, canister_id: Principal, method: &str) -> bool {
    let bytes = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!().expect("encode call"),
        )
        .unwrap_or_else(|error| panic!("update {method}: {error:?}"));
    Decode!(&bytes, bool).unwrap_or_else(|error| panic!("decode {method}: {error}"))
}

fn update_u64(pic: &PocketIc, canister_id: Principal, method: &str) -> u64 {
    let bytes = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!().expect("encode call"),
        )
        .unwrap_or_else(|error| panic!("update {method}: {error:?}"));
    Decode!(&bytes, u64).unwrap_or_else(|error| panic!("decode {method}: {error}"))
}

fn update_u128_pair(pic: &PocketIc, canister_id: Principal, method: &str) -> (u128, u128) {
    let bytes = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!().expect("encode call"),
        )
        .unwrap_or_else(|error| panic!("update {method}: {error:?}"));
    Decode!(&bytes, u128, u128).unwrap_or_else(|error| panic!("decode {method}: {error}"))
}

fn update_unit(pic: &PocketIc, canister_id: Principal, method: &str) {
    let bytes = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!().expect("encode call"),
        )
        .unwrap_or_else(|error| panic!("update {method}: {error:?}"));
    Decode!(&bytes, ()).unwrap_or_else(|error| panic!("decode {method}: {error}"));
}

fn query_inventory_measurement(pic: &PocketIc, canister_id: Principal) -> (u64, u64, bool) {
    let bytes = pic
        .query_call(
            canister_id,
            Principal::anonymous(),
            "inventory_measurement",
            Encode!().expect("encode inventory query"),
        )
        .unwrap_or_else(|error| panic!("query inventory measurement: {error:?}"));
    Decode!(&bytes, u64, u64, bool)
        .unwrap_or_else(|error| panic!("decode inventory measurement: {error}"))
}

fn snapshot(pic: &PocketIc, canister_id: Principal) -> ProbeSnapshot {
    let bytes = pic
        .query_call(
            canister_id,
            Principal::anonymous(),
            "snapshot",
            Encode!().expect("encode query"),
        )
        .unwrap_or_else(|error| panic!("query snapshot: {error:?}"));
    Decode!(&bytes, ProbeSnapshot).expect("decode snapshot")
}

#[test]
fn exact_deadline_sleeps_and_registration_replacement_marks_counter_regrowth() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(
        canister_id,
        probe_wasm(),
        Encode!().expect("encode init"),
        None,
    );
    assert!(update_bool(&pic, canister_id, "start"));
    let initial = snapshot(&pic, canister_id);
    let first_deadline = initial.next_deadline_ns.expect("scheduled");
    let later = first_deadline + 10_000_000_000;
    update_deadline(&pic, canister_id, "schedule_at", later);
    update_deadline(&pic, canister_id, "schedule_at", later);
    pic.advance_time(Duration::from_secs(2));
    drive_rounds(&pic, 16);
    let sleeping = snapshot(&pic, canister_id);
    assert_eq!(sleeping.completed_work, 0);
    assert_eq!(sleeping.scheduler_started, 0);
    assert_eq!(sleeping.next_deadline_ns, Some(later));
    assert_eq!(sleeping.wakeups_armed, 2);
    assert_eq!(sleeping.registration_id, initial.registration_id);

    let continuation = later + 10_000_000_000;
    update_deadline(&pic, canister_id, "schedule_at_on_next_work", continuation);
    update_deadline(&pic, canister_id, "schedule_at", first_deadline);
    drive_rounds(&pic, 16);
    let completed = snapshot(&pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.next_deadline_ns, Some(continuation));
    assert_eq!(completed.registration_id, initial.registration_id);
    pic.advance_time(Duration::from_secs(2));
    drive_rounds(&pic, 16);
    assert_eq!(snapshot(&pic, canister_id).completed_work, 1);

    update_unit(&pic, canister_id, "stop");
    assert_eq!(
        snapshot(&pic, canister_id).registration_id,
        completed.registration_id
    );
    update_unit(&pic, canister_id, "replace_registration");
    let replaced = snapshot(&pic, canister_id);
    assert_eq!(replaced.work_completed, 0);
    assert_ne!(replaced.registration_id, completed.registration_id);
    let old_id = completed.registration_id.expect("old id");
    let new_id = replaced.registration_id.expect("new id");
    assert_eq!((old_id.0, old_id.1), (new_id.0, new_id.1));
    assert!(new_id.2 > old_id.2);
    for _ in 0..2 {
        pic.advance_time(Duration::from_secs(1));
        drive_rounds(&pic, 16);
    }
    let regrown = snapshot(&pic, canister_id);
    assert!(regrown.work_completed > completed.work_completed);
    assert!(regrown.scheduler_instruction_total > completed.scheduler_instruction_total);
    assert_eq!(regrown.registration_id, replaced.registration_id);
}

fn update_deadline(pic: &PocketIc, canister_id: Principal, method: &str, deadline: u64) {
    let result = pic
        .update_call(
            canister_id,
            Principal::anonymous(),
            method,
            Encode!(&deadline).expect("encode deadline"),
        )
        .expect("deadline update");
    Decode!(&result, ()).expect("decode unit");
}

#[derive(CandidType, Debug, Deserialize)]
struct OrdinaryObservation {
    declared: bool,
    running: bool,
    waiting: bool,
    gate_replies: u64,
    gate_error: Option<String>,
    completed: u64,
    completed_at_ns: Option<u64>,
    next_deadline_ns: Option<u64>,
    work_completed: u64,
}

fn ordinary_observation(pic: &PocketIc, canister_id: Principal) -> OrdinaryObservation {
    let bytes = pic
        .query_call(
            canister_id,
            Principal::anonymous(),
            "ordinary_observation",
            Encode!().unwrap(),
        )
        .unwrap();
    Decode!(&bytes, OrdinaryObservation).unwrap()
}

#[test]
fn ordinary_await_allows_ingress_and_other_timers_before_completion() {
    for after_completion in [false, true] {
        for command in ["cancel", "reconcile", "unregister", "recur"] {
            if command == "recur" && !after_completion {
                continue;
            }
            let pic = PocketIc::new();
            let canister_id = pic.create_canister();
            pic.add_cycles(canister_id, INIT_CYCLES);
            pic.install_canister(canister_id, probe_wasm(), Encode!().unwrap(), None);
            let transient = command == "cancel";
            pic.update_call(
                canister_id,
                Principal::anonymous(),
                "start_ordinary",
                Encode!(&after_completion, &transient).unwrap(),
            )
            .unwrap();
            let mut suspended = ordinary_observation(&pic, canister_id);
            for _ in 0..32 {
                pic.tick();
                suspended = ordinary_observation(&pic, canister_id);
                if suspended.running && suspended.waiting && suspended.gate_replies > 0 {
                    break;
                }
            }
            assert!(
                suspended.running && suspended.waiting && suspended.gate_replies > 0,
                "self-call must commit real suspension: after_completion={after_completion} command={command} observed={suspended:?}"
            );
            assert_eq!(suspended.gate_error, None);
            assert!(update_bool(&pic, canister_id, "start"));
            pic.advance_time(Duration::from_secs(2));
            drive_rounds(&pic, 16);
            assert_eq!(
                snapshot(&pic, canister_id).completed_work,
                1,
                "other timer must progress while ordinary work awaits"
            );
            let waiting = ordinary_observation(&pic, canister_id);
            assert!(waiting.running);
            assert!(waiting.waiting);
            assert_eq!(waiting.gate_error, None);
            assert_eq!(waiting.completed, 0);
            assert_eq!(waiting.work_completed, 0);
            let exact_deadline =
                snapshot(&pic, canister_id).next_deadline_ns.unwrap() + 60_000_000_000;
            match command {
                "cancel" => update_unit(&pic, canister_id, "cancel_ordinary"),
                "reconcile" => {
                    update_deadline(&pic, canister_id, "reconcile_ordinary_at", exact_deadline)
                }
                "unregister" => update_unit(&pic, canister_id, "unregister_ordinary"),
                "recur" => {}
                _ => unreachable!("fixed command matrix"),
            }
            assert!(
                ordinary_observation(&pic, canister_id).running,
                "control does not interrupt consumer work"
            );
            update_unit(&pic, canister_id, "release_ordinary_work");
            drive_rounds(&pic, 16);
            let completed = ordinary_observation(&pic, canister_id);
            assert_eq!(completed.completed, 1);
            assert!(!completed.running);
            assert!(!completed.waiting);
            assert_eq!(completed.gate_error, None);
            assert!(completed.gate_replies > suspended.gate_replies);
            if matches!(command, "cancel" | "unregister") {
                assert!(!completed.declared);
                assert_eq!(completed.next_deadline_ns, None);
            } else {
                assert!(completed.declared);
                assert_eq!(completed.work_completed, 1);
                let expected = if command == "reconcile" {
                    exact_deadline
                } else {
                    completed.completed_at_ns.unwrap() + 1_000_000_000
                };
                assert_eq!(completed.next_deadline_ns, Some(expected));
            }
        }
    }
}

#[derive(CandidType, Debug, Deserialize)]
struct ChurnObservation {
    wasm_pages: u64,
    inventory_len: u64,
    armed: bool,
    completed: u64,
}

fn churn_observation(pic: &PocketIc, canister_id: Principal) -> ChurnObservation {
    let bytes = pic
        .query_call(
            canister_id,
            Principal::anonymous(),
            "churn_observation",
            Encode!().unwrap(),
        )
        .unwrap();
    Decode!(&bytes, ChurnObservation).unwrap()
}

#[test]
fn provider_replacement_churn_records_memory_beyond_live_handle_bounds() {
    let pic = PocketIc::new();
    let canister_id = pic.create_canister();
    pic.add_cycles(canister_id, INIT_CYCLES);
    pic.install_canister(canister_id, probe_wasm(), Encode!().unwrap(), None);
    let before = churn_observation(&pic, canister_id);
    pic.update_call(
        canister_id,
        Principal::anonymous(),
        "replace_distant_deadlines",
        Encode!(&32_768_u32).unwrap(),
    )
    .unwrap();
    let replaced = churn_observation(&pic, canister_id);
    assert_eq!(replaced.inventory_len, 1);
    assert!(!replaced.armed);
    assert_eq!(replaced.completed, 0);
    assert!(
        replaced.wasm_pages > before.wasm_pages,
        "many cancelled future deadlines can grow memory despite zero owned handles"
    );
    update_unit(&pic, canister_id, "start_immediate_churn");
    for _ in 0..768 {
        pic.tick();
        if churn_observation(&pic, canister_id).completed == 64 {
            break;
        }
    }
    let immediate = churn_observation(&pic, canister_id);
    assert_eq!(immediate.completed, 64);
    assert_eq!(immediate.inventory_len, 1);
    assert!(!immediate.armed);
    assert!(immediate.wasm_pages >= replaced.wasm_pages);
    pic.advance_time(Duration::from_secs(3_601));
    drive_rounds(&pic, 16);
    let drained = churn_observation(&pic, canister_id);
    assert_eq!(
        drained.completed, 64,
        "cancelled deadlines cannot run consumer work"
    );
    assert!(!drained.armed);
    println!(
        "ic_timers_churn initial_pages={} replaced_pages={} immediate_pages={} drained_pages={} registrations={} owned_wakeups=0 replacements=32768 immediate_steps=64",
        before.wasm_pages,
        replaced.wasm_pages,
        immediate.wasm_pages,
        drained.wasm_pages,
        drained.inventory_len
    );
}
