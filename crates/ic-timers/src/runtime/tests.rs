use super::*;
use crate::{
    InactiveReason, TimerLastOutcome, TimerPolicy, TimerRegistrationStatus,
    TimerRuntimeStateSnapshot, WatchdogDecision, WatchdogRuntimeStateSnapshot,
    platform::{advance_instructions, discard_next_due, run_next_due, set_time, timer_count},
};
use std::{
    cell::{Cell, RefCell},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
};

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "runtime", name).expect("fixture identity should be valid")
}

fn setup() -> TimerEpoch {
    reset_for_test(10, 7);
    initialize_runtime().expect("runtime initialization should succeed")
}

fn assert_retained_provider_binding_failure(
    timer: &TimerIdentity,
    schedule_requests: u64,
    wakeups_armed: u64,
) {
    let failed = timer_snapshot(timer)
        .expect("snapshot lookup should succeed")
        .expect("retained registration should remain declared");
    assert_eq!(
        failed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::ProviderBindingFailed),
        }
    );
    assert_eq!(failed.generation(), None);
    assert_eq!(
        failed.observability().counters().schedule_requests(),
        schedule_requests
    );
    assert_eq!(
        failed.observability().counters().wakeups_armed(),
        wakeups_armed
    );
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
fn fresh_inactive_reconciliation_reserves_complete_retained_inventory() {
    setup();
    let once_identity = identity("built-in-once");
    let after_identity = identity("built-in-after");
    let watchdog_identity = identity("built-in-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let mut once = None;
    let mut after = None;
    let mut watchdog = None;

    reconcile_once(&mut once, &once_identity, None, |_context| async {
        TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop)
    })
    .expect("fresh inactive once declaration should be retained");
    reconcile_after_completion(
        &mut after,
        &after_identity,
        cadence,
        TimerReconcileState::Inactive,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("fresh inactive after-completion declaration should be retained");
    reconcile_watchdog(
        &mut watchdog,
        &watchdog_identity,
        cadence,
        TimerReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("fresh inactive watchdog declaration should be retained");

    assert!(once.is_some());
    assert!(after.is_some());
    assert!(watchdog.is_some());
    assert_eq!(timer_count(), 0);
    let snapshots = timer_snapshots().expect("inventory should be available");
    assert_eq!(snapshots.len(), 3);
    assert_eq!(
        snapshots
            .iter()
            .map(TimerSnapshot::policy)
            .collect::<Vec<_>>(),
        vec![
            TimerPolicy::AfterCompletion { cadence },
            TimerPolicy::Once,
            TimerPolicy::Watchdog { cadence },
        ]
    );
    for snapshot in snapshots {
        assert_eq!(snapshot.lifetime(), DeclarationLifetime::Retained);
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::NeverScheduled,
            }
        );
        assert_eq!(
            snapshot.registration_status(),
            TimerRegistrationStatus::Unregistered
        );
        assert_eq!(snapshot.generation(), None);
    }
}

#[test]
fn registration_claims_report_exact_provider_wakeup_ownership() {
    setup();
    let once = register_once(
        identity("liveness-once"),
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        identity("liveness-after"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        identity("liveness-watchdog"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");

    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.ensure_scheduled(TimerSchedule::At(100))
        .expect("once wake-up should arm");
    after
        .ensure_scheduled()
        .expect("after-completion wake-up should arm");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");
    assert!(once.has_armed_wakeup().expect("claim should be readable"));
    assert!(after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");
    assert!(!once.has_armed_wakeup().expect("claim should be readable"));
    assert!(!after.has_armed_wakeup().expect("claim should be readable"));
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("claim should be readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_claim_observes_the_prearmed_successor_not_queued_work() {
    setup();
    let watchdog = register_watchdog(
        identity("liveness-watchdog-successor"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("watchdog registration should succeed");
    watchdog
        .ensure_scheduled()
        .expect("watchdog wake-up should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler callback should execute");
    assert_eq!(timer_count(), 2, "successor and work should both be queued");
    assert!(
        watchdog
            .has_armed_wakeup()
            .expect("successor ownership should be readable")
    );

    watchdog
        .cancel()
        .expect("cancellation should clear successor and work");
    assert!(
        !watchdog
            .has_armed_wakeup()
            .expect("retained claim should remain readable")
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn removed_transient_claim_cannot_report_wakeup_liveness() {
    setup();
    let timer_identity = identity("liveness-transient");
    let timer = register_once(
        timer_identity.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("registration should succeed");
    timer
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("wake-up should arm");
    assert!(timer.has_armed_wakeup().expect("claim should be readable"));

    set_time(15);
    assert!(run_next_due());
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));

    let replacement = register_once(
        timer_identity,
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should be readable")
    );
    assert!(matches!(
        timer.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
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
fn retained_ordinary_context_expires_after_its_work_attempt() {
    setup();
    let timer = identity("ordinary-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            let directive = if call == 1 {
                *callback_context.borrow_mut() = Some(context);
                TimerDirective::ContinueImmediately
            } else {
                TimerDirective::Stop
            };
            async move { TimerRunResult::new(TimerCompletion::no_work(), directive) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("initial ensure should succeed");

    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "completion should arm the next generation"
    );

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first callback should retain its context");
    assert_eq!(
        expired.identity(),
        &timer,
        "identity remains inert metadata"
    );
    assert!(matches!(
        expired.cancel(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_once(TimerSchedule::At(30)),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or replace the next generation"
    );

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
fn retained_watchdog_context_expires_without_clearing_successor() {
    setup();
    let timer = identity("watchdog-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |context| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                *callback_context.borrow_mut() = Some(context);
            }
            let decision = if call == 1 {
                WatchdogDecision::Continue
            } else {
                WatchdogDecision::Stop
            };
            WatchdogRunResult::new(TimerCompletion::no_work(), decision)
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
    assert_eq!(timer_count(), 1, "committed successor should remain armed");

    let expired = retained_context
        .borrow_mut()
        .take()
        .expect("first work callback should retain its context");
    assert_eq!(
        expired.identity(),
        &timer,
        "identity remains inert metadata"
    );
    assert!(matches!(
        expired.cancel(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_recurring(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(
        timer_count(),
        1,
        "expired context must not clear or duplicate the successor"
    );

    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
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
fn unexpected_watchdog_completion_failure_traps_and_leaves_successor_armed() {
    setup();
    let timer = identity("watchdog-completion-fault");
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

    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    inject_watchdog_completion_fault();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(trapped.is_err(), "an internal completion failure must trap");
    assert_eq!(
        timer_count(),
        1,
        "the cadence successor was committed by the earlier scheduler message"
    );
    let counters = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared")
        .observability()
        .counters();
    assert_eq!(counters.work_started(), 1);
    assert_eq!(counters.work_completed(), 0);
}

#[test]
fn provider_cleanup_borrow_failure_is_returned_instead_of_discarded() {
    setup();
    let timer = identity("provider-cleanup-fault");
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

    let failure = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        clear_entry_provider_handles(&timer)
    });
    assert!(matches!(failure, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 1, "failed cleanup must not lose the handle");
    registration
        .cancel()
        .expect("cleanup remains possible after the borrow is released");
    assert_eq!(timer_count(), 0);
}

#[test]
fn public_provider_install_failure_retires_false_scheduled_state() {
    setup();
    let timer = identity("provider-install-fault");
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
        .expect("initial provider arm should succeed");
    assert_eq!(timer_count(), 1);
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(
        timer_count(),
        0,
        "failed replacement must clear both old and new provider arms"
    );
    assert_retained_provider_binding_failure(&timer, 2, 1);

    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("retained registration should recover on a later request");
    assert_eq!(timer_count(), 1);
    set_time(15);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn initial_once_provider_install_failure_retires_false_scheduled_state() {
    setup();
    let timer = identity("provider-initial-once-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("registration should succeed");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 1, 0);
}

#[test]
fn after_completion_provider_install_failure_retires_false_scheduled_state() {
    setup();
    let timer = identity("provider-after-completion-fault");
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("registration should succeed");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 1, 0);
}

#[test]
fn watchdog_partial_provider_binding_clears_its_committed_successor() {
    setup();
    let timer = identity("provider-watchdog-partial-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    inject_provider_install_fault_after(1);
    set_time(15);
    assert!(run_next_due(), "scheduler callback should execute");
    assert_eq!(timer_count(), 0, "partial binding must clear its successor");
    assert_retained_provider_binding_failure(&timer, 1, 1);
}

#[test]
fn provider_confirmation_failure_clears_the_installed_handle() {
    setup();
    let timer = identity("provider-confirmation-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("registration should succeed");

    inject_provider_confirmation_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 1, 0);
}

#[test]
fn remove_on_stop_provider_failure_removes_the_expired_claim() {
    setup();
    let timer = identity("provider-remove-on-stop-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) },
    )
    .expect("registration should succeed");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(15)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none(),
        "remove-on-stop failure must not retain a false declaration"
    );
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::RegistrationExpired)
    ));
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

    readiness.set(StartupReadiness::RetryableFailure);
    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(pages.get(), 1);
    assert_eq!(timer_count(), 1);
    let retrying = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retryable failure must retain the watchdog");
    assert!(matches!(
        retrying.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { .. })
    ));
    let retrying_counters = retrying.observability().counters();
    assert_eq!(retrying_counters.work_completed(), 2);
    assert_eq!(retrying_counters.retryable_failure(), 1);

    readiness.set(StartupReadiness::Ready);
    set_time(25);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(pages.get(), 1);
    assert_eq!(timer_count(), 0);
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    let performance = stopped.observability().performance();
    assert_eq!(performance.scheduler_instructions().samples(), 3);
    assert_eq!(performance.scheduler_instructions().latest(), Some(10));
    assert_eq!(performance.work_instructions().samples(), 3);
    assert!(
        performance
            .work_instructions()
            .latest()
            .is_some_and(|value| value >= 13)
    );

    readiness.set(StartupReadiness::RetryableFailure);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("returned-error commit guard should synchronously restore a wake-up");
    assert_eq!(timer_count(), 1);

    set_time(30);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    readiness.set(StartupReadiness::TerminalFailure);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    let terminal = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_eq!(
        terminal.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        }
    );
    let terminal_counters = terminal.observability().counters();
    assert_eq!(terminal_counters.work_completed(), 4);
    assert_eq!(terminal_counters.invariant_failure(), 1);
    assert!(terminal_counters.completion_partition_is_valid());

    readiness.set(StartupReadiness::Recovering);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("retained terminal declaration can be reconstructed from durable demand");
    assert_eq!(timer_count(), 1);
    set_time(35);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("terminal durable authority should cancel successor and queued work");
    assert_eq!(timer_count(), 0);

    let mismatch = reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(6).expect("fixture cadence should be valid"),
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
        TimerReconcileState::Scheduled,
        |_context| async {
            TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop)
        },
    )
    .expect("repeated reconstruction should coalesce");
    registration
        .as_ref()
        .expect("reconstruction should retain its claim")
        .reconcile_schedule(Some(TimerSchedule::At(25)))
        .expect("ordinary reconciliation should replace a later deadline exactly");
    assert_eq!(timer_count(), 1);
    set_time(25);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
}

#[test]
fn once_reconciliation_owns_one_exact_deadline_and_retains_its_callback() {
    setup();
    let timer = identity("once-reconstruct");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;

    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(20)),
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            async { TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop) }
        },
    )
    .expect("fresh reconstruction should register and arm");
    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(40)),
        |_context| async {
            TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop)
        },
    )
    .expect("authoritative reconciliation should move the deadline later");
    assert_eq!(timer_count(), 1);
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(40)
    );

    reconcile_once(&mut registration, &timer, None, |_context| async {
        TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop)
    })
    .expect("inactive reconciliation should clear the exact handle");
    assert_eq!(timer_count(), 0);

    registration
        .as_ref()
        .expect("retained claim should remain available")
        .reconcile_schedule(Some(TimerSchedule::At(50)))
        .expect("retained registration should re-arm");
    set_time(50);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "reconciliation must retain the first callback"
    );
}
