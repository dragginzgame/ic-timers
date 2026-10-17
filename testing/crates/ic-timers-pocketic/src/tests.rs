use super::harness::REPLY_LIMITS;
use super::harness::{INIT_CYCLES, fresh_pocket_ic, install_probe, update_unit, wait_for_probe};
use candid::{Encode, Principal};
use ic_testkit::pic::{
    CandidCallExt, PocketIc, PocketIcInstance, advance_and_tick, query_candid_args,
    update_candid_args,
};
use ic_timers_test_protocol::{
    ChurnObservation, OrdinaryObservation, ProbeMemorySummary, ProbeSnapshot,
};
use std::{env, fs, path::PathBuf, time::Duration};

const TIMER_EXECUTOR_METHOD: &str = "<ic-timers internal> dispatch";

#[test]
fn self_call_queue_pressure_defers_timer_submission_without_losing_work() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    update_unit(pic, canister_id, "pressure_and_start");
    advance_and_tick(pic, Duration::ZERO, 8);
    let (failed, replies): (u64, u64) = query_candid_args(
        pic,
        canister_id,
        Principal::anonymous(),
        "pressure_observation",
        (),
        REPLY_LIMITS,
    )
    .expect("pressure counters");
    println!("ic_timers_queue_pressure failed={failed} replies={replies}");
    assert!(failed > 0);
    assert!(replies >= 500);
    assert!(pic.cycle_balance(canister_id) > INIT_CYCLES / 2);
    assert_eq!(snapshot(pic, canister_id).scheduler_started, 0);
    update_unit(pic, canister_id, "release_pressure");
    advance_and_tick(pic, Duration::ZERO, 16);
    assert_eq!(snapshot(pic, canister_id).scheduler_started, 0);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.scheduler_started, 1);
    assert_eq!(completed.completed_work, 1);
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);
}

#[test]
fn deferred_submission_cancellation_and_replacement_do_not_revive_old_work() {
    for replace in [false, true] {
        let owned = fresh_pocket_ic();
        let pic = &owned.instance;
        let canister_id = install_probe(pic, probe_wasm());
        update_unit(pic, canister_id, "pressure_and_start");
        advance_and_tick(pic, Duration::ZERO, 8);
        let original = snapshot(pic, canister_id);
        assert_eq!(original.scheduler_started, 0);
        assert!(pic.cycle_balance(canister_id) > INIT_CYCLES / 2);
        update_unit(
            pic,
            canister_id,
            if replace {
                "replace_registration"
            } else {
                "stop"
            },
        );
        let changed = snapshot(pic, canister_id);
        if replace {
            assert_ne!(changed.registration_id, original.registration_id);
        } else {
            assert_eq!(changed.next_deadline_ns, None);
        }
        update_unit(pic, canister_id, "release_pressure");
        advance_and_tick(pic, Duration::ZERO, 16);
        // A fresh canister's first backend identity belongs to the old scheduler.
        // Even an authenticated late delivery must not revive it after removal.
        let stale_delivery: bool = pic
            .update_candid(
                canister_id,
                Principal::anonymous(),
                "fixture_dispatch_payload",
                (1_u64.to_le_bytes().to_vec(),),
                REPLY_LIMITS,
            )
            .expect("stale self-call result");
        assert!(stale_delivery);
        assert_eq!(snapshot(pic, canister_id).completed_work, 0);
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 16);
        let recovered = snapshot(pic, canister_id);
        assert_eq!(recovered.completed_work, u64::from(replace));
        assert_eq!(recovered.scheduler_started, u64::from(replace));
        if !replace {
            pic.update_candid::<(), _>(
                canister_id,
                Principal::anonymous(),
                "schedule_at",
                (0_u64,),
                REPLY_LIMITS,
            )
            .expect("reschedule retained registration");
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(snapshot(pic, canister_id).completed_work, 1);
        }
        advance_and_tick(pic, Duration::ZERO, 8);
        assert_eq!(snapshot(pic, canister_id).completed_work, 1);
    }
}

#[test]
fn immediate_initial_watchdog_runs_without_advancing_cadence_time() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());

    assert!(update_bool(pic, canister_id, "start_immediately"));
    let initial = snapshot(pic, canister_id);
    assert_eq!(initial.completed_work, 0);
    // Initial After(0) is an explicit request, not a callback continuation.
    assert!(!initial.immediate_scheduling);
    assert_eq!(initial.latest_requested_delay_ns, Some(0));
    assert_eq!(initial.latest_armed_delay_ns, Some(0));
    assert_eq!(initial.schedule_requests, 1);
    assert_eq!(initial.wakeups_armed, 1);

    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.scheduler_started, 1);
    assert_eq!(completed.work_dispatched, 1);
    assert_eq!(completed.work_completed, 1);
    assert!(completed.next_deadline_ns.is_some());
}

#[test]
fn successful_immediate_continuation_runs_again_without_waiting_for_cadence() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    update_unit(pic, canister_id, "continue_immediately_on_next_work");

    let cycles_before_first = pic.cycle_balance(canister_id);
    pic.advance_time(Duration::from_secs(1));
    let immediate = wait_for_probe(
        pic,
        16,
        |pic| snapshot(pic, canister_id),
        |current| current.completed_work == 1,
    );
    assert!(immediate.immediate_scheduling);
    assert_eq!(immediate.latest_requested_delay_ns, Some(0));
    assert_eq!(immediate.latest_armed_delay_ns, Some(0));
    assert_eq!(immediate.schedule_requests, 1);
    assert_eq!(immediate.wakeups_armed, 3);
    assert_eq!(immediate.work_dispatched, 1);
    let first_dispatch_cycles = cycles_before_first.saturating_sub(pic.cycle_balance(canister_id));

    let cycles_before_second = pic.cycle_balance(canister_id);
    advance_and_tick(pic, Duration::ZERO, 16);
    let continued = snapshot(pic, canister_id);
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
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());

    assert!(update_bool(pic, canister_id, "start"));
    assert!(!update_bool(pic, canister_id, "start"));
    assert!(update_bool(pic, canister_id, "start_secondary"));
    assert!(!update_bool(pic, canister_id, "start_secondary"));

    let before_external = snapshot(pic, canister_id);
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
    assert_eq!(snapshot(pic, canister_id), before_external);

    let trap_at = update_u64(pic, canister_id, "arm_trap");
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 12);
    let trapped = snapshot(pic, canister_id);
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
    advance_and_tick(pic, Duration::ZERO, 12);
    let recovered = snapshot(pic, canister_id);
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
    let upgraded = snapshot(pic, canister_id);
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
    advance_and_tick(pic, Duration::ZERO, 12);
    let after_upgrade = snapshot(pic, canister_id);
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
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));

    let exhaust_at = update_u64(pic, canister_id, "arm_exhaustion");
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 30);
    let exhausted = snapshot(pic, canister_id);
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
    advance_and_tick(pic, Duration::ZERO, 16);
    let recovered = snapshot(pic, canister_id);
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
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));

    update_unit(pic, canister_id, "stop");
    let inactive = snapshot(pic, canister_id);
    assert_eq!(inactive.next_deadline_ns, None);
    assert!(!update_bool(pic, canister_id, "ensure_from_returned_error"));
    let ensured = snapshot(pic, canister_id);
    assert!(ensured.next_deadline_ns.is_some());
    let arms_after_ensure = ensured.wakeups_armed;
    assert!(!update_bool(pic, canister_id, "ensure_from_returned_error"));
    assert_eq!(snapshot(pic, canister_id).wakeups_armed, arms_after_ensure);

    pic.stop_canister(canister_id, None)
        .expect("canister stop should succeed");
    pic.advance_time(Duration::from_secs(300));
    advance_and_tick(pic, Duration::ZERO, 4);
    pic.start_canister(canister_id, None)
        .expect("canister resume should succeed");
    advance_and_tick(pic, Duration::ZERO, 16);
    let resumed = snapshot(pic, canister_id);
    assert_eq!(resumed.completed_work, 1);
    assert_eq!(resumed.scheduler_started, 1);
    assert_eq!(resumed.work_dispatched, 1);
    assert_eq!(resumed.work_started, 1);
    assert_eq!(resumed.work_completed, 1);
    assert!(resumed.next_deadline_ns.is_some());

    update_unit(pic, canister_id, "stop_on_next_work");
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let terminal = snapshot(pic, canister_id);
    assert_eq!(terminal.completed_work, 2);
    assert_eq!(terminal.next_deadline_ns, None);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(snapshot(pic, canister_id).completed_work, 2);
}

#[test]
fn insufficient_cycles_defer_due_work_until_top_up() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    let (_burned, call_cost): (u128, u128) = update_candid_args(
        pic,
        canister_id,
        Principal::anonymous(),
        "burn_to_below_timer_call_cost",
        (),
        REPLY_LIMITS,
    )
    .expect("probe multi-result update succeeds");
    assert!(call_cost > 0);

    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 8);
    let deferred = snapshot(pic, canister_id);
    assert_eq!(deferred.completed_work, 0);
    assert_eq!(deferred.scheduler_started, 0);

    pic.add_cycles(canister_id, INIT_CYCLES);
    advance_and_tick(pic, Duration::ZERO, 4);
    assert_eq!(snapshot(pic, canister_id).scheduler_started, 0);
    pic.advance_time(Duration::from_secs(1));
    advance_and_tick(pic, Duration::ZERO, 16);
    let progressed = snapshot(pic, canister_id);
    assert_eq!(progressed.completed_work, 1);
    assert_eq!(progressed.scheduler_started, 1);
}

#[test]
fn full_inventory_is_bounded_deterministic_and_measured() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    update_unit(pic, canister_id, "stop");
    assert_eq!(update_u64(pic, canister_id, "fill_inventory"), 64);
    assert_eq!(update_u64(pic, canister_id, "fill_inventory"), 64);

    let (len, instructions, ordered): (u64, u64, bool) = query_candid_args(
        pic,
        canister_id,
        Principal::anonymous(),
        "inventory_measurement",
        (),
        REPLY_LIMITS,
    )
    .expect("probe inventory query succeeds");
    assert_eq!(len, 64);
    assert!(instructions > 0);
    assert!(ordered);
    println!("ic_timers_inventory entries={len} instructions={instructions}");
}

#[test]
fn cancellation_between_scheduler_and_work_makes_late_provider_delivery_harmless() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    pic.advance_time(Duration::from_secs(1));

    let dispatched = wait_for_probe(
        pic,
        12,
        |pic| snapshot(pic, canister_id),
        |current| current.work_dispatched == 1 && current.work_started == 0,
    );
    assert_eq!(dispatched.completed_work, 0);
    assert!(dispatched.next_deadline_ns.is_some());

    update_unit(pic, canister_id, "stop");
    advance_and_tick(pic, Duration::ZERO, 12);
    let cancelled = snapshot(pic, canister_id);
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

fn update_bool(pic: &PocketIc, canister_id: Principal, method: &str) -> bool {
    pic.update_candid(
        canister_id,
        Principal::anonymous(),
        method,
        (),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds")
}

fn update_u64(pic: &PocketIc, canister_id: Principal, method: &str) -> u64 {
    pic.update_candid(
        canister_id,
        Principal::anonymous(),
        method,
        (),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds")
}

fn snapshot(pic: &PocketIc, canister_id: Principal) -> ProbeSnapshot {
    pic.query_candid(
        canister_id,
        Principal::anonymous(),
        "snapshot",
        (),
        REPLY_LIMITS,
    )
    .expect("probe query succeeds")
}

#[test]
fn exact_deadline_sleeps_and_registration_replacement_marks_counter_regrowth() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    let initial = snapshot(pic, canister_id);
    let first_deadline = initial.next_deadline_ns.expect("scheduled");
    let later = first_deadline + 10_000_000_000;
    update_deadline(pic, canister_id, "schedule_at", later);
    update_deadline(pic, canister_id, "schedule_at", later);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 16);
    let sleeping = snapshot(pic, canister_id);
    assert_eq!(sleeping.completed_work, 0);
    assert_eq!(sleeping.scheduler_started, 0);
    assert_eq!(sleeping.next_deadline_ns, Some(later));
    assert_eq!(sleeping.wakeups_armed, 2);
    assert_eq!(sleeping.registration_id, initial.registration_id);

    let continuation = later + 10_000_000_000;
    // Last request wins; superseded stop/immediate requests cannot leak into
    // this callback or a later one.
    update_unit(pic, canister_id, "stop_on_next_work");
    update_unit(pic, canister_id, "continue_immediately_on_next_work");
    update_deadline(pic, canister_id, "schedule_at_on_next_work", continuation);
    update_deadline(pic, canister_id, "schedule_at", first_deadline);
    advance_and_tick(pic, Duration::ZERO, 16);
    let completed = snapshot(pic, canister_id);
    assert_eq!(completed.completed_work, 1);
    assert_eq!(completed.next_deadline_ns, Some(continuation));
    assert_eq!(completed.registration_id, initial.registration_id);
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 16);
    assert_eq!(snapshot(pic, canister_id).completed_work, 1);

    update_unit(pic, canister_id, "stop");
    assert_eq!(
        snapshot(pic, canister_id).registration_id,
        completed.registration_id
    );
    update_unit(pic, canister_id, "replace_registration");
    let replaced = snapshot(pic, canister_id);
    assert_eq!(replaced.work_completed, 0);
    assert_ne!(replaced.registration_id, completed.registration_id);
    let old_id = completed.registration_id.expect("old id");
    let new_id = replaced.registration_id.expect("new id");
    assert_eq!((old_id.0, old_id.1), (new_id.0, new_id.1));
    assert!(new_id.2 > old_id.2);
    for _ in 0..2 {
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 16);
    }
    let regrown = snapshot(pic, canister_id);
    assert!(regrown.work_completed > completed.work_completed);
    assert!(regrown.scheduler_instruction_total > completed.scheduler_instruction_total);
    assert_eq!(regrown.registration_id, replaced.registration_id);
}

fn update_deadline(pic: &PocketIc, canister_id: Principal, method: &str, deadline: u64) {
    pic.update_candid::<(), _>(
        canister_id,
        Principal::anonymous(),
        method,
        (deadline,),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds");
}

fn ordinary_observation(pic: &PocketIc, canister_id: Principal) -> OrdinaryObservation {
    pic.query_candid(
        canister_id,
        Principal::anonymous(),
        "ordinary_observation",
        (),
        REPLY_LIMITS,
    )
    .expect("probe query succeeds")
}

#[test]
fn ordinary_await_allows_ingress_and_other_timers_before_completion() {
    for after_completion in [false, true] {
        for command in ["cancel", "reconcile", "unregister", "recur"] {
            if command == "recur" && !after_completion {
                continue;
            }
            let owned = fresh_pocket_ic();
            let pic = &owned.instance;
            let canister_id = install_probe(pic, probe_wasm());
            let transient = command == "cancel";
            pic.update_candid::<(), _>(
                canister_id,
                Principal::anonymous(),
                "start_ordinary",
                (after_completion, transient),
                REPLY_LIMITS,
            )
            .expect("probe update succeeds");
            let suspended = wait_for_probe(
                pic,
                32,
                |pic| ordinary_observation(pic, canister_id),
                |current| current.running && current.waiting && current.gate_replies > 0,
            );
            assert_eq!(suspended.gate_error, None);
            assert!(update_bool(pic, canister_id, "start"));
            pic.advance_time(Duration::from_secs(2));
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(
                snapshot(pic, canister_id).completed_work,
                1,
                "other timer must progress while ordinary work awaits"
            );
            let waiting = ordinary_observation(pic, canister_id);
            assert!(waiting.running);
            assert!(waiting.waiting);
            assert_eq!(waiting.gate_error, None);
            assert_eq!(waiting.completed, 0);
            assert_eq!(waiting.work_completed, 0);
            let exact_deadline =
                snapshot(pic, canister_id).next_deadline_ns.unwrap() + 60_000_000_000;
            match command {
                "cancel" => update_unit(pic, canister_id, "cancel_ordinary"),
                "reconcile" => {
                    update_deadline(pic, canister_id, "reconcile_ordinary_at", exact_deadline)
                }
                "unregister" => update_unit(pic, canister_id, "unregister_ordinary"),
                "recur" => {}
                _ => unreachable!("fixed command matrix"),
            }
            assert!(
                ordinary_observation(pic, canister_id).running,
                "control does not interrupt consumer work"
            );
            update_unit(pic, canister_id, "release_ordinary_work");
            advance_and_tick(pic, Duration::ZERO, 16);
            let completed = ordinary_observation(pic, canister_id);
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

fn trapping_ordinary_probe(
    after_completion: bool,
    transient: bool,
    before_await: bool,
) -> (PocketIcInstance<PocketIc>, Principal) {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    pic.update_candid::<(), _>(
        canister_id,
        Principal::anonymous(),
        "trap_ordinary_work",
        (before_await,),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds");
    pic.update_candid::<(), _>(
        canister_id,
        Principal::anonymous(),
        "start_ordinary",
        (after_completion, transient),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds");
    (owned, canister_id)
}

fn assert_ordinary_abandonment(observed: &OrdinaryObservation, removed: bool) {
    assert!(
        !observed.running,
        "discarded futures must release execution authority: {observed:?}"
    );
    assert!(!observed.armed);
    assert_eq!(observed.declared, !removed);
    assert_eq!(observed.abandoned, !removed);
    assert_eq!(observed.next_deadline_ns, None);
    assert_eq!(observed.completed, 0);
    assert_eq!(observed.work_completed, 0);
    assert_eq!(observed.instruction_samples, 0);
    assert_eq!(observed.unacknowledged, u64::from(!removed));
    assert_eq!(observed.captures_dropped, u64::from(removed));
    assert!(observed.capture_registry_accessible);
    assert_eq!(observed.inventory_len, u64::from(!removed));
}

#[test]
fn dispatch_response_trap_cleanup_preserves_pending_work_for_retry() {
    for after_completion in [false, true] {
        let owned = fresh_pocket_ic();
        let pic = &owned.instance;
        let canister_id = install_probe(pic, probe_wasm());
        pic.update_candid::<(), _>(
            canister_id,
            Principal::anonymous(),
            "trap_ordinary_capture_drop",
            (true,),
            REPLY_LIMITS,
        )
        .expect("enable destructor trap");
        pic.update_candid::<(), _>(
            canister_id,
            Principal::anonymous(),
            "trap_ordinary_work",
            (true,),
            REPLY_LIMITS,
        )
        .expect("enable pre-await trap");
        pic.update_candid::<(), _>(
            canister_id,
            Principal::anonymous(),
            "start_ordinary",
            (after_completion, true),
            REPLY_LIMITS,
        )
        .expect("start transient work");
        advance_and_tick(pic, Duration::ZERO, 32);
        let pending = ordinary_observation(pic, canister_id);
        assert!(pending.declared && pending.armed);
        assert!(!pending.running);
        assert_eq!(pending.captures_dropped, 0);
        assert_eq!(pending.completed, 0);

        pic.update_candid::<(), _>(
            canister_id,
            Principal::anonymous(),
            "trap_ordinary_capture_drop",
            (false,),
            REPLY_LIMITS,
        )
        .expect("disable destructor trap");
        advance_and_tick(pic, Duration::ZERO, 8);
        assert!(ordinary_observation(pic, canister_id).declared);
        pic.advance_time(Duration::from_secs(1));
        advance_and_tick(pic, Duration::ZERO, 32);
        assert_ordinary_abandonment(&ordinary_observation(pic, canister_id), true);

        // Confirm the backend still dispatches independent work after cleanup.
        assert!(update_bool(pic, canister_id, "start"));
        pic.advance_time(Duration::from_secs(2));
        advance_and_tick(pic, Duration::ZERO, 16);
        assert_eq!(snapshot(pic, canister_id).completed_work, 1);
    }
}

#[test]
fn ordinary_trap_before_await_retires_the_rolled_back_scheduled_delivery() {
    for after_completion in [false, true] {
        for transient in [false, true] {
            let (owned, canister_id) = trapping_ordinary_probe(after_completion, transient, true);
            let pic = &owned.instance;
            advance_and_tick(pic, Duration::ZERO, 32);
            let observed = ordinary_observation(pic, canister_id);
            assert_ordinary_abandonment(&observed, transient);
            assert_eq!(observed.gate_replies, 0);
            assert!(update_bool(pic, canister_id, "start"));
            pic.advance_time(Duration::from_secs(2));
            advance_and_tick(pic, Duration::ZERO, 16);
            assert_eq!(snapshot(pic, canister_id).completed_work, 1);
            if !transient {
                update_unit(pic, canister_id, "unregister_ordinary");
                assert_eq!(ordinary_observation(pic, canister_id).captures_dropped, 1);
            }
        }
    }
}

#[test]
fn ordinary_continuation_trap_retires_running_work_and_releases_expired_authority() {
    for after_completion in [false, true] {
        for transient in [false, true] {
            for command in ["none", "cancel", "reconcile", "unregister"] {
                let (owned, canister_id) =
                    trapping_ordinary_probe(after_completion, transient, false);
                let pic = &owned.instance;
                wait_for_probe(
                    pic,
                    32,
                    |pic| ordinary_observation(pic, canister_id),
                    |current| current.running && current.waiting && current.gate_replies > 0,
                );
                match command {
                    "none" => {}
                    "cancel" => update_unit(pic, canister_id, "cancel_ordinary"),
                    "reconcile" => {
                        update_deadline(pic, canister_id, "reconcile_ordinary_at", u64::MAX)
                    }
                    "unregister" => update_unit(pic, canister_id, "unregister_ordinary"),
                    _ => unreachable!("fixed command matrix"),
                }
                update_unit(pic, canister_id, "release_ordinary_work");
                advance_and_tick(pic, Duration::ZERO, 32);
                let observed = ordinary_observation(pic, canister_id);
                let removed = transient || command == "unregister";
                assert_ordinary_abandonment(&observed, removed);
                assert_eq!(observed.gate_error, None);
                assert!(update_bool(
                    pic,
                    canister_id,
                    "expired_ordinary_context_rejected"
                ));
                update_unit(pic, canister_id, "clear_ordinary_trap");
                if removed {
                    if command != "unregister" {
                        update_unit(pic, canister_id, "discard_expired_ordinary_claim");
                    }
                    pic.update_candid::<(), _>(
                        canister_id,
                        Principal::anonymous(),
                        "start_ordinary",
                        (after_completion, false),
                        REPLY_LIMITS,
                    )
                    .expect("probe update succeeds");
                } else {
                    // The original retained capability can rearm explicitly.
                    update_deadline(pic, canister_id, "reconcile_ordinary_at", 0);
                }
                advance_and_tick(pic, Duration::ZERO, 16);
                let completed = ordinary_observation(pic, canister_id);
                assert_eq!(completed.completed, 1);
                assert_eq!(completed.work_completed, 1);
                assert_eq!(completed.instruction_samples, 1);
                assert_eq!(completed.unacknowledged, u64::from(!removed));
                update_unit(pic, canister_id, "unregister_ordinary");
                assert_eq!(
                    ordinary_observation(pic, canister_id).captures_dropped,
                    if removed { 2 } else { 1 }
                );
                assert!(update_bool(pic, canister_id, "start"));
                pic.advance_time(Duration::from_secs(2));
                advance_and_tick(pic, Duration::ZERO, 16);
                assert_eq!(snapshot(pic, canister_id).completed_work, 1);
            }
        }
    }
}

fn churn_observation(pic: &PocketIc, canister_id: Principal) -> ChurnObservation {
    pic.query_candid(
        canister_id,
        Principal::anonymous(),
        "churn_observation",
        (),
        REPLY_LIMITS,
    )
    .expect("probe query succeeds")
}

#[test]
fn provider_replacement_churn_preserves_cancellation_and_reports_memory() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    let before = churn_observation(pic, canister_id);
    pic.update_candid::<(), _>(
        canister_id,
        Principal::anonymous(),
        "replace_distant_deadlines",
        (32_768_u32,),
        REPLY_LIMITS,
    )
    .expect("probe update succeeds");
    let replaced = churn_observation(pic, canister_id);
    assert_eq!(replaced.inventory_len, 1);
    assert!(!replaced.armed);
    assert_eq!(replaced.completed, 0);
    // Churn may reuse existing allocation capacity. Report page extents without
    // requiring memory growth for cancellation and continued work to be correct.
    update_unit(pic, canister_id, "start_immediate_churn");
    let immediate = wait_for_probe(
        pic,
        768,
        |pic| churn_observation(pic, canister_id),
        |current| current.completed == 64,
    );
    assert_eq!(immediate.completed, 64);
    assert_eq!(immediate.inventory_len, 1);
    assert!(!immediate.armed);
    assert!(immediate.wasm_pages >= replaced.wasm_pages);
    pic.advance_time(Duration::from_secs(3_601));
    advance_and_tick(pic, Duration::ZERO, 16);
    let drained = churn_observation(pic, canister_id);
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

#[test]
fn armed_callback_removal_releases_captures_with_registry_access() {
    for policy in 0_u8..3 {
        let owned = fresh_pocket_ic();
        let pic = &owned.instance;
        let canister_id = install_probe(pic, probe_wasm());
        pic.update_candid::<(), _>(
            canister_id,
            Principal::anonymous(),
            "remove_armed_callback",
            (policy,),
            REPLY_LIMITS,
        )
        .expect("probe update succeeds");
        let removed = ordinary_observation(pic, canister_id);
        assert!(!removed.declared);
        assert!(!removed.armed);
        assert_eq!(removed.captures_dropped, 1);
        assert!(removed.capture_registry_accessible);
        pic.advance_time(Duration::from_secs(2));
        advance_and_tick(pic, Duration::ZERO, 8);
        let after_deadline = ordinary_observation(pic, canister_id);
        assert_eq!(after_deadline.completed, 0);
        assert_eq!(after_deadline.captures_dropped, 1);
        assert_eq!(after_deadline.inventory_len, 0);
    }
}

#[test]
fn rejected_initial_schedules_preserve_ic_inventory_and_release_captures() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    for policy in 0_u8..3 {
        for transient in [false, true] {
            let result: (bool, u64, bool) = update_candid_args(
                pic,
                canister_id,
                Principal::anonymous(),
                "reject_initial_schedule",
                (policy, transient),
                REPLY_LIMITS,
            )
            .expect("initial schedule rejection observation");
            assert_eq!(result, (true, 1, true));
        }
    }
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    let observation = ordinary_observation(pic, canister_id);
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 6);
}

#[test]
fn rejected_initial_reconciliation_preserves_ic_inventory_and_releases_captures() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    // A positive time makes now + u64::MAX overflow; never move IC time backwards.
    pic.advance_time(Duration::from_secs(1));
    let canister_id = install_probe(pic, probe_wasm());
    for policy in 0_u8..3 {
        let result: (bool, bool, u64, bool) = update_candid_args(
            pic,
            canister_id,
            Principal::anonymous(),
            "reject_initial_reconciliation",
            (policy,),
            REPLY_LIMITS,
        )
        .expect("initial reconciliation rejection observation");
        assert_eq!(result, (true, true, 1, true));
    }
    pic.advance_time(Duration::from_secs(2));
    advance_and_tick(pic, Duration::ZERO, 8);
    let observation = ordinary_observation(pic, canister_id);
    assert_eq!(observation.inventory_len, 0);
    assert_eq!(observation.completed, 0);
    assert_eq!(observation.captures_dropped, 3);
}

#[test]
fn backend_rejects_malformed_self_calls_and_ignores_unknown_deliveries() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    assert!(update_bool(pic, canister_id, "start"));
    let before = snapshot(pic, canister_id);
    for payload in [vec![], vec![0_u8; 7], vec![0_u8; 9]] {
        let accepted: bool = pic
            .update_candid(
                canister_id,
                Principal::anonymous(),
                "fixture_dispatch_payload",
                (payload,),
                REPLY_LIMITS,
            )
            .unwrap();
        assert!(!accepted);
    }
    for _ in 0..2 {
        let accepted: bool = pic
            .update_candid(
                canister_id,
                Principal::anonymous(),
                "fixture_dispatch_payload",
                (0_u64.to_le_bytes().to_vec(),),
                REPLY_LIMITS,
            )
            .unwrap();
        assert!(accepted);
    }
    assert_eq!(snapshot(pic, canister_id), before);
}

#[test]
fn accepted_ordinary_work_survives_dispatch_timeout_without_replay() {
    let owned = fresh_pocket_ic();
    let pic = &owned.instance;
    let canister_id = install_probe(pic, probe_wasm());
    let (): () = pic
        .update_candid(
            canister_id,
            Principal::anonymous(),
            "start_ordinary",
            (false, false),
            REPLY_LIMITS,
        )
        .unwrap();
    let waiting = wait_for_probe(
        pic,
        20,
        |pic| ordinary_observation(pic, canister_id),
        |state| state.waiting,
    );
    assert!(waiting.running);
    // The first pending task identity is 1 in this fresh canister. Acceptance
    // consumed it before the ordinary callback awaited its application gate.
    let accepted: bool = pic
        .update_candid(
            canister_id,
            Principal::anonymous(),
            "fixture_dispatch_payload",
            (1_u64.to_le_bytes().to_vec(),),
            REPLY_LIMITS,
        )
        .unwrap();
    assert!(accepted);
    pic.advance_time(Duration::from_secs(301));
    advance_and_tick(pic, Duration::ZERO, 20);
    let timed_out = ordinary_observation(pic, canister_id);
    assert!(timed_out.running);
    assert_eq!(timed_out.completed, 0);
    update_unit(pic, canister_id, "release_ordinary_work");
    let completed = wait_for_probe(
        pic,
        30,
        |pic| ordinary_observation(pic, canister_id),
        |state| state.completed == 1,
    );
    assert!(!completed.running);
    assert_eq!(completed.gate_error, None);
    advance_and_tick(pic, Duration::ZERO, 8);
    assert_eq!(ordinary_observation(pic, canister_id).completed, 1);
}
