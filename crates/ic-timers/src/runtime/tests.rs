use super::*;
use crate::{
    InactiveReason, TimerLastOutcome, TimerRuntimeStateSnapshot, WatchdogDecision,
    WatchdogRuntimeStateSnapshot,
    platform::{advance_instructions, discard_next_due, run_next_due, set_time, timer_count},
};
use std::{cell::Cell, rc::Rc};

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "runtime", name).expect("fixture identity should be valid")
}

fn setup() -> TimerEpoch {
    reset_for_test(10, 7);
    initialize_runtime().expect("runtime initialization should succeed")
}

#[test]
fn initialization_is_required_and_idempotent() {
    reset_for_test(10, 7);
    assert!(matches!(timer_snapshots(), Err(TimerError::NotInitialized)));

    let first = initialize_runtime().expect("first initialization should succeed");
    set_time(20);
    let second = initialize_runtime().expect("repeated initialization should succeed");
    assert_eq!(first, second);
    assert_eq!(first.canister_version(), 7);
    assert_eq!(first.started_at_ns(), 10);
}

#[test]
fn once_owns_one_provider_handle_and_executes_without_registry_borrow() {
    setup();
    let timer = identity("once");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let callback_identity = timer.clone();
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            advance_instructions(7);
            let visible = timer_snapshot(&callback_identity)
                .expect("consumer work must not observe a registry borrow")
                .is_some();
            async move {
                assert!(visible);
                TimerRunResult::new(TimerCompletion::success(1), TimerDirective::Stop)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled(TimerSchedule::After(Duration::from_nanos(5)))
        .expect("ensure should arm provider timer");
    assert_eq!(timer_count(), 1);
    let scheduled = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("snapshot should exist");
    assert_eq!(scheduled.observability().counters().wakeups_armed(), 1);
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(5));

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(calls.get(), 1);
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(
        stopped.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
    assert_eq!(stopped.observability().counters().work_started(), 1);
    assert_eq!(stopped.observability().counters().work_completed(), 1);
    assert_eq!(
        stopped
            .observability()
            .performance()
            .work_instructions()
            .latest(),
        Some(7)
    );
}

#[test]
fn after_completion_rearms_from_actual_completion_time() {
    setup();
    let timer = identity("after-completion");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                let directive = if call == 1 {
                    TimerDirective::RecurAfterCompletion
                } else {
                    TimerDirective::Stop
                };
                TimerRunResult::new(TimerCompletion::success(1), directive)
            }
        },
    )
    .expect("registration should succeed");

    registration
        .ensure_scheduled()
        .expect("initial ensure should succeed");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(snapshot.observability().counters().wakeups_armed(), 2);
    assert_eq!(snapshot.observability().counters().work_completed(), 2);
}

#[test]
fn nested_cancel_then_ensure_reenables_after_callback_completion() {
    setup();
    let timer = identity("nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_once(TimerSchedule::At(30))
                        .expect("later nested ensure should succeed");
                }
                TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop)
            }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(30)
    );
    set_time(30);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn replacement_and_cancellation_clear_actual_owned_handles() {
    setup();
    let timer = identity("replace-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(100))
        .expect("initial ensure should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(50))
        .expect("earlier deadline should replace the handle");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(50)
    );

    registration.cancel().expect("cancel should clear handle");
    assert_eq!(timer_count(), 0);
    registration
        .cancel()
        .expect("repeated cancellation should be idempotent");
    set_time(100);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained snapshot should exist");
    assert_eq!(snapshot.observability().counters().cancelled(), 1);
}

#[test]
fn duplicate_registration_does_not_replace_callback_and_remove_on_stop_releases_capacity() {
    setup();
    let timer = identity("duplicate");
    let first_calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&first_calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) }
        },
    )
    .expect("registration should succeed");
    let duplicate = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async {
            TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop)
        },
    );
    assert!(matches!(
        duplicate,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(_)
        ))
    ));

    registration
        .ensure_scheduled(TimerSchedule::At(10))
        .expect("ensure should succeed");
    assert!(run_next_due());
    assert_eq!(first_calls.get(), 1);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
    assert!(
        timer_snapshots()
            .expect("inventory should succeed")
            .is_empty()
    );
}

#[test]
fn watchdog_scheduler_prearms_successor_before_synchronous_work() {
    setup();
    let timer = identity("watchdog-prearm");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |_context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            assert_eq!(timer_count(), 1, "successor must exist during work");
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::success(1), decision)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    assert_eq!(timer_count(), 1);

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "scheduler must not invoke consumer work");
    assert_eq!(timer_count(), 2, "successor and work must both be owned");
    let dispatched = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert!(matches!(
        dispatched.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork { .. })
    ));
    assert_eq!(dispatched.observability().counters().scheduler_started(), 1);
    assert_eq!(dispatched.observability().counters().wakeups_armed(), 2);
    assert_eq!(dispatched.observability().counters().work_dispatched(), 1);

    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_cancellation_clears_successor_and_queued_work() {
    setup();
    let timer = identity("watchdog-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);

    registration
        .cancel()
        .expect("cancellation should clear both handles");
    assert_eq!(timer_count(), 0);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_eq!(snapshot.observability().counters().cancelled(), 1);
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    setup();
    let timer = identity("watchdog-nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                context.cancel().expect("nested cancel should succeed");
                context
                    .ensure_recurring()
                    .expect("later nested ensure should succeed");
            }
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    assert!(matches!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Watchdog(
            WatchdogRuntimeStateSnapshot::Scheduled { .. }
        ))
    ));
}

#[test]
fn watchdog_successor_retires_an_unacknowledged_dispatched_attempt() {
    setup();
    let timer = identity("watchdog-unacknowledged");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert!(
        discard_next_due(),
        "simulate work without committed completion"
    );
    assert_eq!(timer_count(), 1);

    set_time(20);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.observability().counters().unacknowledged(), 1);
    assert_eq!(snapshot.observability().counters().work_started(), 0);
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );
}

#[test]
fn overdue_watchdog_coalesces_to_one_dispatch_and_schedules_from_now() {
    setup();
    let timer = identity("watchdog-overdue");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(1_000);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(1_005)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    assert!(!run_next_due(), "no historical cadence point may replay");
}

#[test]
fn remove_on_stop_watchdog_clears_successor_before_dropping_entry() {
    setup();
    let timer = identity("watchdog-remove");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StartupReadiness {
    Ready,
    Recovering,
    RetryableFailure,
    TerminalFailure,
}

fn icydb_watchdog_result(
    readiness: StartupReadiness,
    completed_work_count: u64,
) -> WatchdogRunResult {
    match readiness {
        StartupReadiness::Recovering => WatchdogRunResult::new(
            TimerCompletion::success(completed_work_count),
            WatchdogDecision::Continue,
        ),
        StartupReadiness::RetryableFailure => WatchdogRunResult::new(
            TimerCompletion::retryable_failure(completed_work_count),
            WatchdogDecision::Continue,
        ),
        StartupReadiness::Ready if completed_work_count == 0 => {
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        }
        StartupReadiness::Ready => WatchdogRunResult::new(
            TimerCompletion::success(completed_work_count),
            WatchdogDecision::Stop,
        ),
        StartupReadiness::TerminalFailure => WatchdogRunResult::new(
            TimerCompletion::invariant_failure(completed_work_count),
            WatchdogDecision::Stop,
        ),
    }
}

#[test]
#[allow(clippy::too_many_lines)] // One end-to-end IcyDB-shaped lifecycle fixture.
fn icydb_shaped_reconstruction_and_commit_guard_ensure_are_synchronous_and_idempotent() {
    setup();
    let timer = identity("icydb-startup-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let readiness = Rc::new(Cell::new(StartupReadiness::Recovering));
    let pages = Rc::new(Cell::new(0_u64));
    let mut registration = None;

    assert_eq!(
        icydb_watchdog_result(StartupReadiness::Recovering, 1),
        WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue)
    );
    assert_eq!(
        icydb_watchdog_result(StartupReadiness::RetryableFailure, 0),
        WatchdogRunResult::new(
            TimerCompletion::retryable_failure(0),
            WatchdogDecision::Continue,
        )
    );
    assert_eq!(
        icydb_watchdog_result(StartupReadiness::Ready, 0),
        WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
    );
    assert_eq!(
        icydb_watchdog_result(StartupReadiness::Ready, 1),
        WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
    );
    assert_eq!(
        icydb_watchdog_result(StartupReadiness::TerminalFailure, 0),
        WatchdogRunResult::new(
            TimerCompletion::invariant_failure(0),
            WatchdogDecision::Stop,
        )
    );

    let callback_readiness = Rc::clone(&readiness);
    let callback_pages = Rc::clone(&pages);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Scheduled,
        move |_context| {
            advance_instructions(13);
            match callback_readiness.get() {
                StartupReadiness::Recovering => {
                    callback_pages.set(callback_pages.get().saturating_add(1));
                }
                StartupReadiness::Ready
                | StartupReadiness::RetryableFailure
                | StartupReadiness::TerminalFailure => {}
            }
            let completed_work_count =
                u64::from(callback_readiness.get() == StartupReadiness::Recovering);
            icydb_watchdog_result(callback_readiness.get(), completed_work_count)
        },
    )
    .expect("fresh reconstruction should register and schedule");
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Scheduled,
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should reuse the original callback claim");
    assert_eq!(timer_count(), 1);

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(pages.get(), 1);
    assert_eq!(timer_count(), 1);

    readiness.set(StartupReadiness::Ready);
    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(pages.get(), 1);
    assert_eq!(timer_count(), 0);
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    let performance = stopped.observability().performance();
    assert_eq!(performance.scheduler_instructions().samples(), 2);
    assert_eq!(performance.scheduler_instructions().latest(), Some(10));
    assert_eq!(performance.work_instructions().samples(), 2);
    assert!(
        performance
            .work_instructions()
            .latest()
            .is_some_and(|value| value >= 13)
    );

    readiness.set(StartupReadiness::Recovering);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Scheduled,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("returned-error commit guard should synchronously restore a wake-up");
    assert_eq!(timer_count(), 1);

    set_time(25);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    readiness.set(StartupReadiness::TerminalFailure);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("terminal durable authority should cancel successor and queued work");
    assert_eq!(timer_count(), 0);

    let mismatch = reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(6).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        TimerReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    );
    assert!(matches!(mismatch, Err(TimerError::ReconciliationConflict)));
}

#[test]
fn after_completion_reconstruction_reuses_its_exact_claim() {
    setup();
    let timer = identity("after-reconstruct");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Scheduled,
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) }
        },
    )
    .expect("fresh reconstruction should succeed");
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        DeclarationLifetime::Retained,
        TimerReconcileState::Scheduled,
        |_context| async {
            TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop)
        },
    )
    .expect("repeated reconstruction should coalesce");
    assert_eq!(timer_count(), 1);
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
}
