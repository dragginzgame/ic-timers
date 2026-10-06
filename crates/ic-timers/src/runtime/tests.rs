use super::*;
use crate::{
    AfterCompletionDecision, InactiveReason, MemoryPageSample, OnceDecision, TimerLastOutcome,
    TimerPolicy, TimerProcessCondition, TimerRegistrationStatus, TimerRuntimeStateSnapshot,
    TimerSchedulingMode, WatchdogDecision, WatchdogRuntimeStateSnapshot,
    control::WakeupArm,
    platform::{
        advance_instructions, discard_next_due, grow_memory_pages, run_next_due, set_time,
        timer_count,
    },
};
use std::{
    cell::{Cell, RefCell},
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    task::{Poll, Waker},
};

#[derive(Clone, Default)]
struct SuspendedWork {
    ready: Rc<Cell<bool>>,
    waker: Rc<RefCell<Option<Waker>>>,
    polls: Rc<Cell<u64>>,
}

impl SuspendedWork {
    #[expect(
        clippy::future_not_send,
        reason = "The native executor models single-threaded IC callbacks with canister-local state."
    )]
    async fn wait(&self) {
        std::future::poll_fn(|context| {
            self.polls.set(self.polls.get() + 1);
            if self.ready.get() {
                Poll::Ready(())
            } else {
                *self.waker.borrow_mut() = Some(context.waker().clone());
                Poll::Pending
            }
        })
        .await;
    }

    fn resume(&self) {
        self.ready.set(true);
        self.waker
            .borrow_mut()
            .take()
            .expect("work suspended")
            .wake();
    }
}

fn suspended_ordinary_registration(
    after_completion: bool,
    timer: TimerIdentity,
    lifetime: DeclarationLifetime,
    gate: SuspendedWork,
    context: Rc<RefCell<Option<CallbackToken>>>,
    captured: Rc<()>,
) -> RegistrationClaim {
    if after_completion {
        register_after_completion(
            timer,
            TimerCadence::from_nanos(5).unwrap(),
            lifetime,
            move |work| {
                *context.borrow_mut() = Some(work.token);
                let gate = gate.clone();
                let captured = Rc::clone(&captured);
                async move {
                    gate.wait().await;
                    std::hint::black_box(captured);
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap()
        .claim
    } else {
        register_once(timer, lifetime, move |work| {
            *context.borrow_mut() = Some(work.token);
            let gate = gate.clone();
            let captured = Rc::clone(&captured);
            async move {
                gate.wait().await;
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
            }
        })
        .unwrap()
        .claim
    }
}

#[test]
fn dropped_ordinary_deliveries_retire_confirmed_scheduled_and_running_work() {
    for after_completion in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for started in [false, true] {
                let _fixture = setup();
                let timer = identity("dropped-ordinary");
                let gate = SuspendedWork::default();
                let context = Rc::new(RefCell::new(None));
                let captured = Rc::new(());
                let weak = Rc::downgrade(&captured);
                let claim = suspended_ordinary_registration(
                    after_completion,
                    timer.clone(),
                    lifetime,
                    gate.clone(),
                    Rc::clone(&context),
                    captured,
                );
                reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).unwrap();
                set_time(10);
                if started {
                    assert!(run_next_due());
                    assert_eq!(gate.polls.get(), 1);
                    gate.resume();
                }
                set_time(20);
                // Drop only; the native fake neither traps nor rolls messages back.
                assert!(discard_next_due());
                assert_eq!(timer_count(), 0);
                if let Some(token) = context.borrow().as_ref() {
                    assert!(matches!(
                        cancel_claim(token.claim(), Some(token)),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                if lifetime == DeclarationLifetime::Retained {
                    let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Abandoned,
                        }
                    );
                    assert_eq!(snapshot.process_condition(), TimerProcessCondition::Failed);
                    assert_eq!(snapshot.next_deadline_ns(), None);
                    assert!(!has_armed_wakeup_claim(&claim).unwrap());
                    let observations = snapshot.observability();
                    assert_eq!(observations.counters().unacknowledged(), 1);
                    assert_eq!(observations.counters().work_started(), u64::from(started));
                    assert_eq!(observations.counters().work_completed(), 0);
                    assert_eq!(observations.performance().work_instructions().samples(), 0);
                    assert_eq!(
                        observations.outcomes().last_outcome(),
                        Some(TimerLastOutcome::Unacknowledged)
                    );
                    assert_eq!(
                        observations.outcomes().last_unacknowledged_at_ns(),
                        Some(20)
                    );
                    assert!(weak.upgrade().is_some());
                    if !started {
                        gate.ready.set(true);
                    }
                    reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(30))).unwrap();
                    set_time(30);
                    assert!(run_next_due());
                    let completed = timer_snapshot(&timer).unwrap().unwrap();
                    assert_eq!(completed.observability().counters().work_completed(), 1);
                    assert_eq!(completed.observability().counters().unacknowledged(), 1);
                    unregister_claim(&claim).unwrap();
                } else {
                    assert!(timer_snapshot(&timer).unwrap().is_none());
                    assert!(matches!(
                        has_armed_wakeup_claim(&claim),
                        Err(TimerError::RegistrationExpired)
                    ));
                }
                assert!(
                    weak.upgrade().is_none(),
                    "removed callbacks must release captures"
                );
                assert_eq!(timer_inventory().unwrap().timers(), []);
            }
        }
    }
}

#[test]
fn abandoned_running_work_discards_scheduling_commands_and_finishes_unregistration() {
    for after_completion in [false, true] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for command in ["cancel", "reconcile", "ensure", "unregister"] {
                let _fixture = setup();
                let timer = identity("abandoned-command");
                let gate = SuspendedWork::default();
                let context = Rc::new(RefCell::new(None));
                let claim = suspended_ordinary_registration(
                    after_completion,
                    timer.clone(),
                    lifetime,
                    gate.clone(),
                    Rc::clone(&context),
                    Rc::new(()),
                );
                reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).unwrap();
                set_time(10);
                assert!(run_next_due());
                match command {
                    "cancel" => cancel_claim(&claim, None).unwrap(),
                    "reconcile" => {
                        reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(40)))
                            .unwrap();
                    }
                    "ensure" if after_completion => ensure_recurring_claim(&claim, None).unwrap(),
                    "ensure" => ensure_once_claim(&claim, None, TimerSchedule::At(40)).unwrap(),
                    "unregister" => unregister_claim(&claim).unwrap(),
                    _ => unreachable!("fixed command matrix"),
                }
                gate.resume();
                set_time(20);
                assert!(discard_next_due());
                if command == "unregister" || lifetime == DeclarationLifetime::RemoveWhenStopped {
                    assert!(timer_snapshot(&timer).unwrap().is_none());
                } else {
                    let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Abandoned
                        }
                    );
                    assert_eq!(snapshot.next_deadline_ns(), None);
                    assert_eq!(snapshot.observability().counters().work_completed(), 0);
                    unregister_claim(&claim).unwrap();
                }
                let token = context.borrow();
                let token = token.as_ref().unwrap();
                assert!(matches!(
                    cancel_claim(token.claim(), Some(token)),
                    Err(TimerError::RegistrationExpired)
                ));
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn stale_delivery_drop_cannot_retire_rearmed_or_reused_identity() {
    let _fixture = setup();
    let timer = identity("stale-delivery-drop");
    let registration = register_once(timer.clone(), DeclarationLifetime::Retained, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(20)))
        .unwrap();
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .unwrap()
        .unwrap();
    let (token, handle) = detached.into_parts();
    bind_provider_handle(&token, handle).unwrap();
    let old_delivery = dispatch_wakeup(token.clone());
    registration
        .reconcile_schedule(Some(TimerSchedule::At(30)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(old_delivery);
    assert_eq!(timer_inventory().unwrap(), before);
    registration.unregister().unwrap();
    let replacement = register_once(timer, DeclarationLifetime::Retained, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    replacement
        .reconcile_schedule(Some(TimerSchedule::At(40)))
        .unwrap();
    let before = timer_inventory().unwrap();
    drop(dispatch_wakeup(token));
    assert_eq!(timer_inventory().unwrap(), before);
    assert!(replacement.has_armed_wakeup().unwrap());
    replacement.unregister().unwrap();
}

#[test]
fn abandoned_transient_releases_capacity_and_drops_captures_outside_registry_borrow() {
    struct CaptureDropCheck {
        timer: TimerIdentity,
        dropped: Rc<Cell<bool>>,
    }
    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert!(timer_snapshot(&self.timer).unwrap().is_none());
            self.dropped.set(true);
        }
    }

    let _fixture = setup();
    let timer = identity("abandoned-capacity");
    let dropped = Rc::new(Cell::new(false));
    let captured = Rc::new(CaptureDropCheck {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    });
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        move |_| {
            let captured = Rc::clone(&captured);
            async move {
                std::hint::black_box(captured);
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            }
        },
    )
    .unwrap();
    registration
        .ensure_scheduled(TimerSchedule::After(Duration::ZERO))
        .unwrap();
    let mut remaining = Vec::new();
    for index in 1..crate::MAX_TIMER_REGISTRATIONS {
        remaining.push(
            register_once(
                identity(&format!("capacity-{index}")),
                DeclarationLifetime::Retained,
                |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
            )
            .unwrap(),
        );
    }
    assert_eq!(
        timer_inventory().unwrap().timers().len(),
        crate::MAX_TIMER_REGISTRATIONS
    );
    assert!(discard_next_due());
    assert!(dropped.get());
    let replacement = register_once(timer, DeclarationLifetime::Retained, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    assert!(matches!(
        registration.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    replacement.unregister().unwrap();
    for registration in remaining {
        registration.unregister().unwrap();
    }
}

#[test]
fn unconfirmed_ordinary_delivery_drop_preserves_binding_failure_accounting() {
    for after_completion in [false, true] {
        for confirmation_failure in [false, true] {
            let _fixture = setup();
            let timer = identity("unconfirmed-delivery-drop");
            let claim = suspended_ordinary_registration(
                after_completion,
                timer.clone(),
                DeclarationLifetime::Retained,
                SuspendedWork::default(),
                Rc::new(RefCell::new(None)),
                Rc::new(()),
            );
            if confirmation_failure {
                inject_provider_confirmation_fault();
            } else {
                inject_provider_install_fault();
            }
            assert!(reconcile_ordinary_claim(&claim, None, Some(TimerSchedule::At(10))).is_err());
            let snapshot = timer_snapshot(&timer).unwrap().unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::ControlFailure(
                        TimerControlFailure::ProviderBindingFailed
                    ),
                }
            );
            assert_eq!(snapshot.observability().counters().unacknowledged(), 0);
            assert_eq!(snapshot.observability().outcomes().last_outcome(), None);
            assert_eq!(timer_count(), 0);
            unregister_claim(&claim).unwrap();
        }
    }
}

#[test]
fn suspended_once_work_allows_other_timers_and_arbitrates_external_commands() {
    for command in ["cancel", "reconcile", "ensure", "unregister"] {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            let _fixture = setup();
            let timer = identity("suspended-once");
            let gate = SuspendedWork::default();
            let callback_gate = gate.clone();
            let context_slot = Rc::new(RefCell::new(None));
            let callback_slot = Rc::clone(&context_slot);
            let registration = register_once(timer.clone(), lifetime, move |context| {
                let gate = callback_gate.clone();
                *callback_slot.borrow_mut() = Some(context);
                async move {
                    gate.wait().await;
                    OnceRunResult::new(TimerCompletion::success(1), OnceDecision::ScheduleAt(80))
                }
            })
            .unwrap();
            registration
                .ensure_scheduled(TimerSchedule::At(15))
                .unwrap();
            let unrelated_calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&unrelated_calls);
            let unrelated = register_watchdog(
                identity("unrelated-watchdog"),
                TimerCadence::from_nanos(5).unwrap(),
                DeclarationLifetime::Retained,
                move |_| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
                },
            )
            .unwrap();
            unrelated.ensure_scheduled().unwrap();
            set_time(15);
            assert!(run_next_due()); // Ordinary work suspends without an armed handle.
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(timer_count(), 1);
            assert!(run_next_due()); // Unrelated scheduler and work must still run.
            assert!(run_next_due());
            assert_eq!(unrelated_calls.get(), 1);
            assert!(!run_next_due());
            assert_eq!(
                gate.polls.get(),
                1,
                "sleeping work must not be polled again"
            );
            match command {
                "cancel" => registration.cancel().unwrap(),
                "reconcile" => {
                    registration.cancel().unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(50)))
                        .unwrap();
                }
                "ensure" => registration
                    .ensure_scheduled(TimerSchedule::At(40))
                    .unwrap(),
                "unregister" => registration.unregister().unwrap(),
                _ => unreachable!("fixed command matrix"),
            }
            set_time(30);
            gate.resume();
            assert!(run_next_due());
            let observed = timer_snapshot(&timer).unwrap();
            match command {
                "reconcile" | "ensure" => {
                    let snapshot = observed.unwrap();
                    assert_eq!(
                        snapshot.next_deadline_ns(),
                        Some(if command == "ensure" { 40 } else { 50 })
                    );
                    assert_eq!(snapshot.observability().counters().work_completed(), 1);
                    assert_eq!(timer_count(), 1);
                }
                "cancel" if lifetime == DeclarationLifetime::Retained => {
                    assert_eq!(
                        observed.unwrap().state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::Cancelled
                        }
                    );
                    assert_eq!(timer_count(), 0);
                }
                _ => {
                    assert!(observed.is_none());
                    assert_eq!(timer_count(), 0);
                }
            }
            assert!(matches!(
                context_slot.borrow().as_ref().unwrap().cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep both policy paths and their arbitration observations in one decision matrix."
)]
fn policy_specific_results_schedule_through_live_runtime() {
    for recurring in [false, true] {
        for case in 0..8 {
            if !recurring && case == 4 {
                continue;
            }
            for reconcile in [false, true] {
                let _fixture = setup();
                let timer = identity("typed-results");
                let completion = if case == 7 {
                    TimerCompletion::invariant_failure(2)
                } else {
                    TimerCompletion::retryable_failure(2)
                };
                if recurring {
                    let decision = match case {
                        0 => AfterCompletionDecision::Stop,
                        1 | 7 => AfterCompletionDecision::ContinueImmediately,
                        2 => AfterCompletionDecision::RetryAfter(Duration::from_nanos(7)),
                        3 => AfterCompletionDecision::ScheduleAt(50),
                        4 => AfterCompletionDecision::RecurAfterCompletion,
                        5 => AfterCompletionDecision::RetryAfter(Duration::MAX),
                        6 => AfterCompletionDecision::RetryAfter(Duration::from_nanos(u64::MAX)),
                        _ => unreachable!("fixed decision cases"),
                    };
                    let registration = register_after_completion(
                        timer.clone(),
                        TimerCadence::from_nanos(5).unwrap(),
                        DeclarationLifetime::Retained,
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            AfterCompletionRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .reconcile_schedule(Some(TimerSchedule::At(20)))
                        .unwrap();
                } else {
                    let decision = match case {
                        0 => OnceDecision::Stop,
                        1 | 7 => OnceDecision::ContinueImmediately,
                        2 => OnceDecision::RetryAfter(Duration::from_nanos(7)),
                        3 => OnceDecision::ScheduleAt(50),
                        5 => OnceDecision::RetryAfter(Duration::MAX),
                        6 => OnceDecision::RetryAfter(Duration::from_nanos(u64::MAX)),
                        _ => unreachable!("Once has no configured recurrence"),
                    };
                    let registration = register_once(
                        timer.clone(),
                        DeclarationLifetime::Retained,
                        move |context| async move {
                            if reconcile {
                                context
                                    .reconcile_schedule(Some(TimerSchedule::At(100)))
                                    .unwrap();
                            }
                            OnceRunResult::new(completion, decision)
                        },
                    )
                    .unwrap();
                    registration
                        .ensure_scheduled(TimerSchedule::At(20))
                        .unwrap();
                }
                set_time(20);
                assert!(run_next_due());
                let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                let deadline = if reconcile && case != 7 {
                    Some(100)
                } else {
                    match case {
                        1 => Some(20),
                        2 => Some(27),
                        3 => Some(50),
                        4 => Some(25),
                        _ => None,
                    }
                };
                assert_eq!(snapshot.next_deadline_ns(), deadline);
                assert_eq!(timer_count(), usize::from(deadline.is_some()));
                if case == 7 {
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::InvariantFailure,
                        }
                    );
                } else if !reconcile && matches!(case, 5 | 6) {
                    let failure = if case == 5 {
                        TimerControlFailure::DelayOutOfRange
                    } else {
                        TimerControlFailure::DeadlineOverflow
                    };
                    assert_eq!(
                        snapshot.state(),
                        TimerRuntimeStateSnapshot::Inactive {
                            reason: InactiveReason::ControlFailure(failure),
                        }
                    );
                } else if reconcile {
                    assert_eq!(
                        snapshot.latest_directive(),
                        Some(crate::TimerDirectiveSnapshot::ScheduleAt { deadline_ns: 100 })
                    );
                }
                assert_eq!(
                    snapshot.observability().outcomes().last_outcome(),
                    Some(TimerLastOutcome::Completed(
                        if !reconcile && matches!(case, 5 | 6) {
                            crate::TimerCompletionOutcome::InvariantFailure
                        } else {
                            completion.outcome()
                        }
                    ))
                );
            }
        }
    }
}

#[test]
fn suspended_after_completion_uses_completion_time_and_exact_reconciliation() {
    for reconcile in [false, true] {
        let _fixture = setup();
        let timer = identity("suspended-after-completion");
        let gate = SuspendedWork::default();
        let callback_gate = gate.clone();
        let registration = register_after_completion(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            DeclarationLifetime::Retained,
            move |_| {
                let gate = callback_gate.clone();
                async move {
                    gate.wait().await;
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::RecurAfterCompletion,
                    )
                }
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        assert!(!registration.has_armed_wakeup().unwrap());
        assert!(!run_next_due());
        assert_eq!(timer_count(), 0);
        if reconcile {
            registration
                .reconcile_schedule(Some(TimerSchedule::At(50)))
                .unwrap();
        }
        set_time(30);
        gate.resume();
        assert!(run_next_due());
        assert_eq!(
            timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
            Some(if reconcile { 50 } else { 35 })
        );
        assert_eq!(timer_count(), 1);
        registration.unregister().unwrap();
        assert_eq!(timer_count(), 0);
    }
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "runtime", name).expect("fixture identity should be valid")
}

#[must_use = "retain the fixture until the test scope ends"]
struct RuntimeFixture {
    epoch: TimerEpoch,
}

impl Drop for RuntimeFixture {
    fn drop(&mut self) {
        // The production delivery guard must run while runtime TLS is available.
        // Native thread teardown is not an IC lifecycle or rollback event.
        platform::clear_tasks();
    }
}

fn setup() -> RuntimeFixture {
    reset_for_test(10, 7);
    RuntimeFixture {
        epoch: initialize_runtime().expect("runtime initialization should succeed"),
    }
}

#[test]
fn fixture_cleanup_drops_queued_and_suspended_work_before_tls_teardown() {
    struct CaptureDropCheck(Rc<Cell<bool>>);

    impl Drop for CaptureDropCheck {
        fn drop(&mut self) {
            assert_eq!(timer_count(), 0, "cleanup must release the task-map borrow");
            assert!(
                timer_inventory().is_ok(),
                "runtime TLS must remain available"
            );
            self.0.set(true);
        }
    }

    for started in [false, true] {
        let dropped = Rc::new(Cell::new(false));
        {
            let _fixture = setup();
            let gate = SuspendedWork::default();
            let captured = Rc::new(CaptureDropCheck(Rc::clone(&dropped)));
            let registration = register_once(
                identity("fixture-cleanup"),
                DeclarationLifetime::RemoveWhenStopped,
                move |_| {
                    let gate = gate.clone();
                    let captured = Rc::clone(&captured);
                    async move {
                        gate.wait().await;
                        std::hint::black_box(captured);
                        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                    }
                },
            )
            .unwrap();
            registration
                .ensure_scheduled(TimerSchedule::At(10))
                .unwrap();
            if started {
                assert!(run_next_due());
            }
            assert!(!dropped.get());
        }
        assert!(
            dropped.get(),
            "fixture cleanup must release callback captures"
        );
        assert_eq!(timer_count(), 0);
        assert_eq!(timer_inventory().unwrap().timers(), []);
        assert!(!run_next_due());
    }
}

#[test]
fn rejected_watchdog_cadence_preserves_the_complete_snapshot() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("rejected-watchdog-cadence");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(u64::MAX).expect("positive cadence is valid"),
            lifetime,
            |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .expect("declaration should register without arming");
        let before = timer_snapshot(&timer).expect("snapshot should be readable");

        assert!(matches!(
            registration.ensure_scheduled(),
            Err(TimerError::Schedule(ScheduleError::DeadlineOverflow))
        ));
        assert_eq!(
            timer_snapshot(&timer).expect("snapshot should be readable"),
            before
        );
        assert_eq!(timer_count(), 0);
        assert!(
            !registration
                .has_armed_wakeup()
                .expect("claim remains valid")
        );

        registration
            .ensure_scheduled_immediately()
            .expect("a valid request remains possible after rejection");
        assert_eq!(timer_count(), 1);
        assert_eq!(
            timer_snapshot(&timer)
                .expect("snapshot should be readable")
                .expect("armed declaration exists")
                .observability()
                .counters()
                .schedule_requests(),
            1
        );
    }
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
    assert!(matches!(timer_inventory(), Err(TimerError::NotInitialized)));

    let first = initialize_runtime().expect("first initialization should succeed");
    set_time(20);
    let second = initialize_runtime().expect("repeated initialization should succeed");
    assert_eq!(first, second);
    assert_eq!(first.canister_version(), 7);
    assert_eq!(first.started_at_ns(), 10);
    let busy = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        initialize_runtime()
    });
    assert!(matches!(busy, Err(TimerError::RuntimeBusy)));
    let inventory = timer_inventory().expect("initialized inventory should be available");
    assert_eq!(inventory.epoch(), first);
    assert!(inventory.is_empty());
    assert_eq!(inventory.timers(), []);
    assert_eq!(inventory.into_timers(), []);
}

#[test]
fn fresh_inactive_reconciliation_reserves_complete_retained_inventory() {
    let _fixture = setup();
    let once_identity = identity("built-in-once");
    let after_identity = identity("built-in-after");
    let watchdog_identity = identity("built-in-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let mut once = None;
    let mut after = None;
    let mut watchdog = None;

    reconcile_once(&mut once, &once_identity, None, |_context| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .expect("fresh inactive once declaration should be retained");
    reconcile_after_completion(
        &mut after,
        &after_identity,
        cadence,
        TimerReconcileState::Inactive,
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
    )
    .expect("fresh inactive after-completion declaration should be retained");
    reconcile_watchdog(
        &mut watchdog,
        &watchdog_identity,
        cadence,
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("fresh inactive watchdog declaration should be retained");

    assert!(once.is_some());
    assert!(after.is_some());
    assert!(watchdog.is_some());
    assert_eq!(timer_count(), 0);
    let inventory = timer_inventory().expect("inventory should be available");
    assert_eq!(inventory.epoch(), TimerEpoch::new(7, 10));
    assert_eq!(inventory.len(), 3);
    let snapshots = inventory.timers();
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
fn fresh_transient_cancellation_expires_every_registration_policy() {
    let _fixture = setup();
    let once_identity = identity("fresh-transient-once");
    let after_identity = identity("fresh-transient-after");
    let watchdog_identity = identity("fresh-transient-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");

    let once = register_once(
        once_identity.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        after_identity.clone(),
        cadence,
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
    )
    .expect("after-completion registration should succeed");
    let watchdog = register_watchdog(
        watchdog_identity.clone(),
        cadence,
        DeclarationLifetime::RemoveWhenStopped,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");

    once.cancel().expect("once cancellation should succeed");
    after
        .cancel()
        .expect("after-completion cancellation should succeed");
    watchdog
        .cancel()
        .expect("watchdog cancellation should succeed");

    assert_eq!(timer_count(), 0);
    for identity in [&once_identity, &after_identity, &watchdog_identity] {
        assert!(
            timer_snapshot(identity)
                .expect("snapshot lookup should succeed")
                .is_none(),
            "transient cancellation should remove {identity:?}"
        );
    }
    assert!(matches!(
        once.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        after.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        watchdog.has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
}

#[test]
fn registration_claims_report_exact_provider_wakeup_ownership() {
    let _fixture = setup();
    let once = register_once(
        identity("liveness-once"),
        DeclarationLifetime::Retained,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("once registration should succeed");
    let after = register_after_completion(
        identity("liveness-after"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
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
    let _fixture = setup();
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
fn immediate_watchdog_reconciliation_arms_one_zero_delay_scheduler() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-reconcile");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        WatchdogReconcileState::ScheduledImmediately,
        move |_context| {
            callback_calls.set(callback_calls.get().saturating_add(1));
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .expect("fresh immediate reconciliation should succeed");

    assert_eq!(timer_count(), 1);
    let scheduled = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(scheduled.next_deadline_ns(), Some(10));
    assert_eq!(
        scheduled.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(scheduled.latest_requested_delay_ns(), Some(0));
    assert_eq!(scheduled.latest_armed_delay_ns(), Some(0));
    let counters = scheduled.observability().counters();
    assert_eq!(counters.schedule_requests(), 1);
    assert_eq!(counters.wakeups_armed(), 1);

    assert!(run_next_due(), "zero-delay scheduler should run later");
    assert_eq!(calls.get(), 0, "the scheduler must not invoke work inline");
    assert_eq!(
        timer_count(),
        2,
        "cadence successor and work should be armed"
    );
    assert!(run_next_due(), "separate work callback should run later");
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_moves_cadence_earlier_and_repeats_idempotently() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-ensure");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");
    assert_eq!(timer_count(), 1);
    registration
        .ensure_scheduled_immediately()
        .expect("cadence scheduler should move to now");
    registration
        .ensure_scheduled_immediately()
        .expect("repeated immediate ensure should coalesce");

    assert_eq!(timer_count(), 1, "replacement must own exactly one handle");
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(10));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests(), 3);
    assert_eq!(counters.wakeups_armed(), 2);
    assert_eq!(counters.coalesced(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_ensure_coalesces_an_overdue_cadence_wakeup() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-overdue-cadence");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    set_time(20);
    registration
        .ensure_scheduled_immediately()
        .expect("overdue cadence scheduler should satisfy immediate demand");

    assert_eq!(timer_count(), 1, "coalescing must not add a provider timer");
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(snapshot.scheduling_mode(), TimerSchedulingMode::Watchdog);
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(5));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests(), 2);
    assert_eq!(counters.wakeups_armed(), 1);
    assert_eq!(counters.coalesced(), 1);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn removed_transient_claim_cannot_report_wakeup_liveness() {
    let _fixture = setup();
    let timer_identity = identity("liveness-transient");
    let timer = register_once(
        timer_identity.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
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
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
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
    let _fixture = setup();
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
            grow_memory_pages(2, 3);
            let visible = timer_snapshot(&callback_identity)
                .expect("consumer work must not observe a registry borrow")
                .is_some();
            async move {
                assert!(visible);
                OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
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
    let memory = stopped.observability().performance().work_memory_pages();
    assert_eq!(memory.samples(), 1);
    let latest = memory.latest().expect("normal work should have one sample");
    assert_eq!(latest.start().wasm_pages(), 1);
    assert_eq!(latest.start().stable_pages(), 0);
    assert_eq!(latest.end().wasm_pages(), 3);
    assert_eq!(latest.end().stable_pages(), 3);
    assert_eq!(memory.maximum_wasm_growth_pages(), Some(2));
    assert_eq!(memory.maximum_stable_growth_pages(), Some(3));
}

#[test]
fn ordinary_callback_borrow_failure_stops_without_invoking_or_measuring_work() {
    for (recurring, lifetime) in [
        (false, DeclarationLifetime::Retained),
        (false, DeclarationLifetime::RemoveWhenStopped),
        (true, DeclarationLifetime::Retained),
        (true, DeclarationLifetime::RemoveWhenStopped),
    ] {
        let _fixture = setup();
        let timer = identity("ordinary-borrow-failure");
        let calls = Rc::new(Cell::new(0));
        let callback_calls = Rc::clone(&calls);
        let callback = if recurring {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async {
                        AfterCompletionRunResult::new(
                            TimerCompletion::success(1),
                            AfterCompletionDecision::RecurAfterCompletion,
                        )
                    }
                },
                AfterCompletionContext::new,
                OrdinaryRunResult::from,
            )
        } else {
            erase_ordinary_callback(
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
                },
                OnceContext::new,
                OrdinaryRunResult::from,
            )
        };
        let claim = with_registry_mut(|registry| {
            let registered = if recurring {
                registry.register_after_completion_with_callback(
                    timer.clone(),
                    TimerCadence::from_nanos(5).unwrap(),
                    lifetime,
                    Rc::clone(&callback),
                )
            } else {
                registry.register_once_with_callback(timer.clone(), lifetime, Rc::clone(&callback))
            };
            registered.map_err(TimerError::from)
        })
        .unwrap();
        if recurring {
            ensure_recurring_claim(&claim, None).unwrap();
        } else {
            ensure_once_claim(&claim, None, TimerSchedule::At(15)).unwrap();
        }
        assert_eq!(timer_count(), 1);

        // Keep the callback unavailable while dispatch accepts and completes work.
        let _borrow = callback.borrow_mut();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(calls.get(), 0);
        assert_eq!(timer_count(), 0);
        assert!(!run_next_due());

        let snapshot = timer_snapshot(&timer).unwrap();
        if lifetime == DeclarationLifetime::Retained {
            let snapshot = snapshot.unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
            );
            assert_eq!(snapshot.process_condition(), TimerProcessCondition::Failed);
            let counters = snapshot.observability().counters();
            assert_eq!(counters.work_started(), 1);
            assert_eq!(counters.work_completed(), 1);
            assert_eq!(counters.invariant_failure(), 1);
            let performance = snapshot.observability().performance();
            assert_eq!(performance.work_instructions().samples(), 0);
            assert_eq!(performance.work_memory_pages().samples(), 0);
            assert!(!has_armed_wakeup_claim(&claim).unwrap());
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                has_armed_wakeup_claim(&claim),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn after_completion_recurrence_follows_returned_completion_classification() {
    for completion in [
        TimerCompletion::success(3),
        TimerCompletion::no_work(),
        TimerCompletion::retryable_failure(2),
        TimerCompletion::invariant_failure(4),
    ] {
        let _fixture = setup();
        let timer = identity("classified-recurrence");
        let registration = register_after_completion(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            DeclarationLifetime::Retained,
            move |_| async move {
                AfterCompletionRunResult::new(
                    completion,
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        let snapshot = timer_snapshot(&timer).unwrap().unwrap();
        let invariant = completion.outcome() == crate::TimerCompletionOutcome::InvariantFailure;
        assert_eq!(
            snapshot.next_deadline_ns(),
            if invariant { None } else { Some(20) }
        );
        assert_eq!(registration.has_armed_wakeup().unwrap(), !invariant);
        assert_eq!(timer_count(), usize::from(!invariant));
        let outcomes = snapshot.observability().outcomes();
        assert_eq!(
            outcomes.last_outcome(),
            Some(TimerLastOutcome::Completed(completion.outcome()))
        );
        assert_eq!(outcomes.last_work_count(), Some(completion.work_count()));
        assert_eq!(
            outcomes.consecutive_expected_failures(),
            u64::from(completion.outcome() == crate::TimerCompletionOutcome::RetryableFailure)
        );
        assert_eq!(snapshot.observability().counters().work_completed(), 1);
        assert_eq!(
            snapshot.latest_directive(),
            Some(if invariant {
                crate::TimerDirectiveSnapshot::Stop
            } else {
                crate::TimerDirectiveSnapshot::RecurAfterCompletion
            })
        );
    }
}

#[test]
fn after_completion_rearms_from_actual_completion_time() {
    let _fixture = setup();
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
                    AfterCompletionDecision::RecurAfterCompletion
                } else {
                    AfterCompletionDecision::Stop
                };
                AfterCompletionRunResult::new(TimerCompletion::success(1), directive)
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
fn after_completion_context_can_restore_recurrence_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("after-context");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |context: AfterCompletionContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled()
                        .expect("later nested ensure should succeed");
                }
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
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
    assert_eq!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(20)
    );

    set_time(20);
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn once_context_can_restore_scheduling_after_nested_cancel() {
    let _fixture = setup();
    let timer = identity("nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            async move {
                if call == 1 {
                    context
                        .cancel()
                        .expect("nested cancellation should succeed");
                    context
                        .ensure_scheduled(TimerSchedule::At(30))
                        .expect("later nested ensure should succeed");
                }
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
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
fn retained_once_context_expires_after_its_work_attempt() {
    let _fixture = setup();
    let timer = identity("ordinary-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |context: OnceContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            let directive = if call == 1 {
                *callback_context.borrow_mut() = Some(context);
                OnceDecision::ContinueImmediately
            } else {
                OnceDecision::Stop
            };
            async move { OnceRunResult::new(TimerCompletion::no_work(), directive) }
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
        expired.ensure_scheduled(TimerSchedule::At(30)),
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
fn stale_reused_identity_callback_cannot_change_handles_or_measurements() {
    let _fixture = setup();
    let timer_identity = identity("stale-consume-reuse");
    let old = register_once(
        timer_identity.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("old registration should succeed");
    old.ensure_scheduled(TimerSchedule::At(15))
        .expect("old wake-up should arm");
    let old_handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(&timer_identity)
            .ok_or(TimerError::OwnershipInvariant)
    })
    .expect("old provider handle should detach");
    let (old_token, old_provider_handle) = old_handle.into_parts();
    platform::clear_timer(old_provider_handle);
    old.cancel()
        .expect("old transient registration should be removed");

    let stale_measurement = CallbackMeasurement {
        instructions: 1_000,
        memory_start: MemoryPageExtent::new(1, 0),
        memory_end: MemoryPageExtent::new(11, 20),
    };
    record_callback_measurements(&old_token, stale_measurement);
    assert!(timer_snapshot(old_token.identity()).unwrap().is_none());

    let replacement = register_once(
        timer_identity,
        DeclarationLifetime::Retained,
        |_context| async {
            advance_instructions(7);
            grow_memory_pages(2, 3);
            OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
        },
    )
    .expect("replacement registration should succeed");
    replacement
        .ensure_scheduled(TimerSchedule::At(25))
        .expect("replacement wake-up should arm");
    assert_eq!(timer_count(), 1);

    let before = timer_snapshot(replacement.identity()).unwrap().unwrap();
    let before_performance = before.observability().performance();
    let stale_token = old_token.clone();
    let _stale_delivery = platform::set_timer(Duration::ZERO, async move {
        dispatch_wakeup(stale_token).await;
    });
    assert!(run_next_due(), "stale callback delivery should be harmless");
    // Exercise the accounting boundary directly; stale dispatch would return
    // before taking a measurement and cannot reach this ownership check.
    record_callback_measurements(&old_token, stale_measurement);
    let after = timer_snapshot(replacement.identity()).unwrap().unwrap();
    assert_eq!(after.observability().performance(), before_performance);
    assert_eq!(after.observability().counters().stale_wakeups(), 1);
    assert_eq!(after.observability().counters().work_started(), 0);
    assert!(
        replacement
            .has_armed_wakeup()
            .expect("replacement claim should retain its handle")
    );
    assert_eq!(timer_count(), 1);

    set_time(25);
    assert!(run_next_due());
    let completed = timer_snapshot(replacement.identity()).unwrap().unwrap();
    let performance = completed.observability().performance();
    assert_eq!(performance.work_instructions().samples(), 1);
    assert_eq!(performance.work_instructions().total(), 7);
    assert_eq!(performance.work_memory_pages().samples(), 1);
    assert_eq!(
        performance.work_memory_pages().latest(),
        Some(MemoryPageSample::new(
            MemoryPageExtent::new(1, 0),
            MemoryPageExtent::new(3, 3),
        ))
    );
    assert_eq!(
        performance.scheduler_instructions(),
        before_performance.scheduler_instructions()
    );
    assert_eq!(
        performance.scheduler_memory_pages(),
        before_performance.scheduler_memory_pages()
    );
    record_callback_measurements(&old_token, stale_measurement);
    let after_stale = timer_snapshot(replacement.identity()).unwrap().unwrap();
    assert_eq!(after_stale.observability().performance(), performance);

    replacement
        .cancel()
        .expect("replacement cleanup should succeed");
    assert_eq!(timer_count(), 0);
}

#[test]
fn replacement_and_cancellation_clear_actual_owned_handles() {
    let _fixture = setup();
    let timer = identity("replace-cancel");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
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
    let cancelled_generation = timer_snapshot(&timer)
        .unwrap()
        .unwrap()
        .generation()
        .unwrap();

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

    registration
        .ensure_scheduled(TimerSchedule::At(150))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(150);
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn duplicate_registration_and_reconstruction_preserve_live_work_and_release_capacity() {
    let _fixture = setup();
    let timer = identity("duplicate");
    let first_calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&first_calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(20))
        .expect("ensure should succeed");
    let before = timer_inventory().unwrap();
    let duplicate = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
        },
    );
    assert!(matches!(
        duplicate,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(_)
        ))
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    let mut empty_slot = None;
    let reconstruction = reconcile_once(&mut empty_slot, &timer, None, |_| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    });
    assert!(matches!(
        reconstruction,
        Err(TimerError::Register(
            RegisterError::IdentityAlreadyRegistered(ref occupied)
        )) if occupied == &timer
    ));
    assert!(empty_slot.is_none());
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(registration.has_armed_wakeup().unwrap());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(first_calls.get(), 1);
    assert!(
        timer_snapshot(&timer)
            .expect("snapshot lookup should succeed")
            .is_none()
    );
    assert!(
        timer_inventory()
            .expect("inventory should succeed")
            .is_empty()
    );
}

#[test]
fn watchdog_scheduler_prearms_successor_before_synchronous_work() {
    let _fixture = setup();
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
fn watchdog_immediate_decision_replaces_successor_without_synchronous_recursion() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-decision");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |_context| {
            let call = callback_calls.get().saturating_add(1);
            callback_calls.set(call);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if call == 1 {
                    WatchdogDecision::ContinueImmediately
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due(), "scheduler should pre-arm cadence safety");
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "first work callback should run");
    assert_eq!(calls.get(), 1);
    assert_eq!(
        timer_count(),
        1,
        "cadence successor must be replaced, not added to"
    );
    let immediate = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(immediate.next_deadline_ns(), Some(15));
    assert_eq!(
        immediate.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(immediate.latest_requested_delay_ns(), Some(0));
    assert_eq!(immediate.latest_armed_delay_ns(), Some(0));
    let counters = immediate.observability().counters();
    assert_eq!(counters.wakeups_armed(), 3);
    assert_eq!(counters.work_dispatched(), 1);

    assert!(
        run_next_due(),
        "replacement scheduler should be a later message"
    );
    assert_eq!(
        calls.get(),
        1,
        "scheduler still must not invoke consumer work"
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due(), "second separate work callback should run");
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_running_immediate_context_targets_that_attempts_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-context-immediate");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |context| {
            context
                .ensure_scheduled_immediately()
                .expect("running immediate request should succeed");
            context
                .ensure_scheduled()
                .expect("later cadence ensure must not delay immediate demand");
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(timer_count(), 1);
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("watchdog snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(15));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.observability().counters().schedule_requests(), 3);
    assert_eq!(snapshot.observability().counters().coalesced(), 2);

    registration
        .cancel()
        .expect("fixture cleanup should succeed");
}

#[test]
fn watchdog_cancellation_clears_successor_and_queued_work() {
    let _fixture = setup();
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
    let cancelled_generation = timer_snapshot(&timer)
        .unwrap()
        .unwrap()
        .generation()
        .unwrap();

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

    registration.ensure_scheduled().unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().generation(),
        Some(cancelled_generation + 1)
    );
    assert_eq!(timer_count(), 1);
    set_time(20);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 1);
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_invariant_failure_overrides_nested_cancellation_and_clears_handles() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("watchdog-cancel-invariant");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            lifetime,
            |context: WatchdogContext| {
                context.cancel().unwrap();
                WatchdogRunResult::new(
                    TimerCompletion::invariant_failure(1),
                    WatchdogDecision::ContinueImmediately,
                )
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due(), "scheduler should pre-arm its successor");
        assert_eq!(timer_count(), 2);
        assert!(
            run_next_due(),
            "work should complete with invariant failure"
        );
        assert_eq!(
            timer_count(),
            0,
            "terminal failure must clear the successor"
        );

        let snapshot = timer_snapshot(&timer).unwrap();
        if lifetime == DeclarationLifetime::Retained {
            let snapshot = snapshot.unwrap();
            assert_eq!(
                snapshot.state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::InvariantFailure,
                }
            );
            assert_eq!(snapshot.process_condition(), TimerProcessCondition::Failed);
            let counters = snapshot.observability().counters();
            assert_eq!(counters.work_completed(), 1);
            assert_eq!(counters.invariant_failure(), 1);
            assert_eq!(counters.cancelled(), 0);
        } else {
            assert!(snapshot.is_none());
            assert!(matches!(
                registration.cancel(),
                Err(TimerError::RegistrationExpired)
            ));
        }
    }
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    let _fixture = setup();
    let timer = identity("watchdog-nested");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |context: WatchdogContext| {
            let call = callback_calls.get() + 1;
            callback_calls.set(call);
            if call == 1 {
                context.cancel().expect("nested cancel should succeed");
                context
                    .ensure_scheduled()
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
    let _fixture = setup();
    let timer = identity("watchdog-context-expiry");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let retained_context = Rc::new(RefCell::new(None));
    let callback_context = Rc::clone(&retained_context);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        move |context: WatchdogContext| {
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
        expired.ensure_scheduled(),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        expired.ensure_scheduled_immediately(),
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
    let _fixture = setup();
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
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .expect("dispatched reconciliation should queue for this attempt");
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

    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(
        completed.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        },
        "the retired attempt's reconciliation must not override the successor's Stop"
    );
    assert_eq!(completed.observability().counters().unacknowledged(), 1);
    assert_eq!(completed.observability().counters().work_completed(), 1);
    assert_eq!(timer_count(), 0);
}

#[test]
fn immediate_watchdog_completion_fault_traps_and_leaves_cadence_successor_armed() {
    let _fixture = setup();
    let timer = identity("watchdog-completion-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| {
            grow_memory_pages(2, 3);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
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
    let snapshot = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    let counters = snapshot.observability().counters();
    assert_eq!(counters.work_started(), 1);
    assert_eq!(counters.work_completed(), 0);
    let performance = snapshot.observability().performance();
    assert_eq!(performance.scheduler_memory_pages().samples(), 1);
    assert_eq!(performance.work_memory_pages().samples(), 0);
    assert_eq!(performance.work_memory_pages().latest(), None);
}

#[test]
fn immediate_watchdog_provider_replacement_failure_traps_for_message_rollback() {
    let _fixture = setup();
    let timer = identity("watchdog-immediate-provider-fault");
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            )
        },
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("initial scheduler should arm");
    set_time(15);
    assert!(run_next_due(), "scheduler should commit cadence safety");

    inject_provider_install_fault();
    let trapped = catch_unwind(AssertUnwindSafe(run_next_due));
    assert!(
        trapped.is_err(),
        "replacement binding failure must trap the work message"
    );
    // The native provider mock is deliberately not transactional. On the IC,
    // the trap rolls the clear/install and registry transition back together,
    // restoring the cadence successor committed by the scheduler message.
}

#[test]
fn public_immediate_watchdog_replacement_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("watchdog-public-immediate-provider-fault");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    )
    .expect("watchdog registration should succeed");
    registration
        .ensure_scheduled()
        .expect("cadence scheduler should arm");

    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled_immediately(),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0);
    assert_retained_provider_binding_failure(&timer, 2, 1);
}

#[test]
fn provider_cleanup_borrow_failure_is_returned_instead_of_discarded() {
    let _fixture = setup();
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
    clear_entry_provider_handles(&timer).expect("empty entry cleanup should be harmless");
    registration.unregister().unwrap();
    clear_entry_provider_handles(&timer).expect("missing entry cleanup should be harmless");
    assert_eq!(timer_count(), 0);
}

#[test]
fn rejected_provider_binding_clears_handles_for_unavailable_or_expired_authority() {
    let _fixture = setup();
    let timer = identity("provider-binding-authority");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("provider wake-up should arm");
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .expect("registry should be available")
        .expect("provider handle should detach");
    let (token, handle) = detached.into_parts();

    let rejected = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow();
        bind_provider_handle(&token, handle)
    });
    assert!(matches!(rejected, Err(TimerError::RuntimeBusy)));
    assert_eq!(timer_count(), 0);

    registration
        .unregister()
        .expect("declaration should be removable after the borrow is released");
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 0);

    RUNTIME.with(|runtime| *runtime.borrow_mut() = None);
    let handle = platform::set_timer(Duration::ZERO, async {});
    assert!(matches!(
        bind_provider_handle(&token, handle),
        Err(TimerError::NotInitialized)
    ));
    assert_eq!(timer_count(), 0);
}

#[test]
fn provider_restoration_drains_all_detached_handles_after_first_failure() {
    let _fixture = setup();
    let timer = identity("provider-restore-drain");
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
    assert!(run_next_due(), "scheduler callback should execute");
    assert_eq!(timer_count(), 2, "successor and work should be armed");
    let handles = with_registry_mut(|registry| {
        registry
            .take_provider_handles_for_claim(&registration.claim)
            .map_err(TimerError::from)
    })
    .expect("both handles should detach");

    inject_provider_install_fault();
    assert!(matches!(
        restore_provider_handles(handles),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(
        timer_count(),
        1,
        "the failed handle should clear while the remaining handle is restored"
    );

    let mut restored = with_registry_mut(|registry| Ok(registry.take_provider_handles(&timer)))
        .expect("restored handle should detach for fixture cleanup");
    let restored_wakeup = restored.take_wakeup();
    let restored_work = restored.take_work();
    assert!(restored_wakeup.is_none());
    clear_provider_handle(restored_work.expect("work handle must have been restored"));
    assert_eq!(timer_count(), 0);
}

#[test]
fn detached_provider_selection_does_not_reborrow_the_registry() {
    let _fixture = setup();
    let timer = identity("detached-selection");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
    )
    .expect("registration should succeed");
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .expect("provider wake-up should arm");
    let detached = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
        .expect("registry should be available")
        .expect("provider handle should detach");

    let selected = RUNTIME.with(|runtime| {
        let _borrow = runtime.borrow_mut();
        take_detached_or_owned_handle(Some(detached), |_| None)
    });
    let selected = selected
        .expect("a detached handle should avoid another registry borrow")
        .expect("the detached handle should be retained");

    clear_provider_handle(selected);
    assert_eq!(timer_count(), 0);
}

#[test]
fn terminal_scheduler_failure_clears_queued_work_before_transient_removal() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        let _fixture = setup();
        let timer = identity("terminal-scheduler-cleanup");
        let work = Rc::new(Cell::new(0));
        let observed_work = Rc::clone(&work);
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            lifetime,
            move |_| {
                observed_work.set(observed_work.get() + 1);
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
            },
        )
        .unwrap();
        registration.ensure_scheduled().unwrap();
        set_time(15);
        assert!(run_next_due());
        assert_eq!(timer_count(), 2);

        // Deliver the successor before its deferred work. Consume the fake
        // provider callback exactly as a scheduler entry would, retaining work.
        let wakeup = with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
            .unwrap()
            .unwrap();
        let (scheduler, handle) = wakeup.into_parts();
        platform::clear_timer(handle);
        assert_eq!(timer_count(), 1);
        set_time(u64::MAX);
        dispatch_watchdog_scheduler(&scheduler);

        assert_eq!(timer_count(), 0);
        assert_eq!(work.get(), 0);
        assert!(!run_next_due());
        if lifetime == DeclarationLifetime::Retained {
            assert!(!registration.has_armed_wakeup().unwrap());
            assert_eq!(
                timer_snapshot(&timer).unwrap().unwrap().state(),
                TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::ControlFailure(TimerControlFailure::DeadlineOverflow),
                }
            );
        } else {
            assert!(timer_snapshot(&timer).unwrap().is_none());
            assert!(matches!(
                registration.has_armed_wakeup(),
                Err(TimerError::RegistrationExpired)
            ));
            assert!(
                register_once(timer, DeclarationLifetime::Retained, |_| async {
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                })
                .is_ok()
            );
        }
    }
}

#[test]
fn rejected_and_coalesced_requests_preserve_bound_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("request-handle-preservation");
    let registration = register_once(timer.clone(), DeclarationLifetime::Retained, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    registration
        .ensure_scheduled(TimerSchedule::At(15))
        .unwrap();
    let before = timer_snapshot(&timer).unwrap().unwrap();

    // If either request attempts a handle reinstall it consumes this fault.
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::After(Duration::MAX)),
        Err(TimerError::Schedule(ScheduleError::DelayOutOfRange))
    ));
    assert_eq!(timer_snapshot(&timer).unwrap().unwrap(), before);
    registration
        .ensure_scheduled(TimerSchedule::At(20))
        .unwrap();
    assert!(registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 1);
    let after = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(after.generation(), before.generation());
    assert_eq!(after.next_deadline_ns(), before.next_deadline_ns());
    assert_eq!(after.observability().counters().coalesced(), 1);

    // A real replacement must still encounter the unconsumed installation fault.
    assert!(matches!(
        registration.reconcile_schedule(Some(TimerSchedule::At(25))),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(!registration.has_armed_wakeup().unwrap());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}

#[test]
fn public_provider_install_failure_retires_false_scheduled_state() {
    let _fixture = setup();
    let timer = identity("provider-install-fault");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        move |_context| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
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
    let _fixture = setup();
    let timer = identity("provider-initial-once-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
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
    let _fixture = setup();
    let timer = identity("provider-after-completion-fault");
    let registration = register_after_completion(
        timer.clone(),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| async {
            AfterCompletionRunResult::new(TimerCompletion::no_work(), AfterCompletionDecision::Stop)
        },
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
fn watchdog_failed_dispatch_clears_handles_without_confirming_work() {
    for lifetime in [
        DeclarationLifetime::Retained,
        DeclarationLifetime::RemoveWhenStopped,
    ] {
        for failure_stage in ["successor-binding", "work-binding", "confirmation"] {
            let _fixture = setup();
            let timer = identity("provider-watchdog-dispatch-fault");
            let calls = Rc::new(Cell::new(0));
            let callback_calls = Rc::clone(&calls);
            let registration = register_watchdog(
                timer.clone(),
                TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
                lifetime,
                move |_context| {
                    callback_calls.set(callback_calls.get() + 1);
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
                },
            )
            .expect("registration should succeed");
            registration
                .ensure_scheduled()
                .expect("initial scheduler should arm");

            match failure_stage {
                "successor-binding" => inject_provider_install_fault(),
                "work-binding" => inject_provider_install_fault_after(1),
                "confirmation" => inject_provider_confirmation_fault(),
                _ => unreachable!("closed failure stages"),
            }
            set_time(15);
            assert!(run_next_due(), "scheduler callback should execute");
            assert_eq!(timer_count(), 0, "failed dispatch must clear every handle");
            assert!(
                !run_next_due(),
                "failed dispatch must not leave queued work"
            );
            assert_eq!(
                calls.get(),
                0,
                "failed dispatch must not invoke consumer work"
            );

            if lifetime == DeclarationLifetime::Retained {
                assert_retained_provider_binding_failure(&timer, 1, 1);
                assert!(!registration.has_armed_wakeup().unwrap());
                let snapshot = timer_snapshot(&timer).unwrap().unwrap();
                let counters = snapshot.observability().counters();
                assert_eq!(counters.scheduler_started(), 1);
                assert_eq!(counters.work_dispatched(), 0);
                assert_eq!(counters.work_started(), 0);
                assert_eq!(counters.work_completed(), 0);
            } else {
                assert!(timer_snapshot(&timer).unwrap().is_none());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                let replacement = register_once(timer, DeclarationLifetime::Retained, |_| async {
                    OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                })
                .expect("failed transient dispatch should release the identity");
                replacement.ensure_scheduled(TimerSchedule::At(25)).unwrap();
                assert!(replacement.has_armed_wakeup().unwrap());
                assert!(matches!(
                    registration.has_armed_wakeup(),
                    Err(TimerError::RegistrationExpired)
                ));
                assert_eq!(timer_count(), 1);
                replacement.unregister().unwrap();
                assert_eq!(timer_count(), 0);
            }
        }
    }
}

#[test]
fn provider_confirmation_failure_clears_the_installed_handle() {
    let _fixture = setup();
    let timer = identity("provider-confirmation-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::Retained,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
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
fn watchdog_dispatch_rejects_cross_claim_tokens_before_provider_arms() {
    let _fixture = setup();
    let first = register_watchdog(
        identity("cross-claim-first"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("first watchdog registration should succeed");
    let second = register_watchdog(
        identity("cross-claim-second"),
        TimerCadence::from_nanos(5).expect("fixture cadence should be valid"),
        DeclarationLifetime::Retained,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("second watchdog registration should succeed");
    first
        .ensure_scheduled()
        .expect("first scheduler should arm");
    second
        .ensure_scheduled()
        .expect("second scheduler should arm");

    let first_handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(first.identity())
            .ok_or(TimerError::OwnershipInvariant)
    })
    .expect("first scheduler handle should detach");
    let second_handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(second.identity())
            .ok_or(TimerError::OwnershipInvariant)
    })
    .expect("second scheduler handle should detach");
    let (first_scheduler, first_provider_handle) = first_handle.into_parts();
    let (second_scheduler, second_provider_handle) = second_handle.into_parts();
    platform::clear_timer(first_provider_handle);
    platform::clear_timer(second_provider_handle);
    assert_eq!(timer_count(), 0);

    let first_dispatch =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&first_scheduler, 15)))
            .expect("first scheduler should dispatch");
    let second_dispatch =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&second_scheduler, 15)))
            .expect("second scheduler should dispatch");
    let RegistryEffect::DispatchWatchdog {
        successor: first_successor,
        successor_delay_ns: first_delay,
        ..
    } = first_dispatch.into_effect()
    else {
        panic!("first scheduler should emit a watchdog dispatch");
    };
    let RegistryEffect::DispatchWatchdog {
        work: second_work, ..
    } = second_dispatch.into_effect()
    else {
        panic!("second scheduler should emit a watchdog dispatch");
    };
    let malformed_arm = RegistryEffect::ArmWakeup {
        token: second_work.clone(),
        delay_ns: 0,
        arm: WakeupArm::Initial,
    };
    assert!(matches!(
        apply_effect(&malformed_arm, ProviderHandles::default()),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0, "validation must precede provider arms");
    let malformed_replacement = RegistryEffect::ArmWakeup {
        token: first_successor.clone(),
        delay_ns: first_delay,
        arm: WakeupArm::Replacement,
    };
    assert!(matches!(
        apply_effect(&malformed_replacement, ProviderHandles::default()),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 0, "validation must precede provider arms");
    let malformed = RegistryEffect::DispatchWatchdog {
        successor: first_successor,
        successor_delay_ns: first_delay,
        work: second_work,
    };

    assert!(matches!(
        apply_effect(&malformed, ProviderHandles::default()),
        Err(TimerError::OwnershipInvariant)
    ));
    assert_eq!(timer_count(), 0, "validation must precede provider arms");
}

#[test]
fn watchdog_dispatch_rejects_mixed_generations_before_provider_arms() {
    let _fixture = setup();
    let registration = register_watchdog(
        identity("mixed-dispatch-generations"),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        |_context| panic!("malformed effects must not execute consumer work"),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let handle = with_registry_mut(|registry| {
        registry
            .take_wakeup_handle(registration.identity())
            .ok_or(TimerError::OwnershipInvariant)
    })
    .unwrap();
    let (scheduler, handle) = handle.into_parts();
    platform::clear_timer(handle);
    let dispatch =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&scheduler, 15)))
            .unwrap();
    let RegistryEffect::DispatchWatchdog {
        successor: old_successor,
        work: old_work,
        ..
    } = dispatch.into_effect()
    else {
        panic!("fixture must emit a dispatch");
    };
    let recovery =
        with_registry_mut(|registry| Ok(registry.begin_watchdog_scheduler(&old_successor, 20)))
            .unwrap();
    let RegistryEffect::DispatchWatchdog {
        successor,
        successor_delay_ns,
        work,
    } = recovery.into_effect()
    else {
        panic!("fixture must emit a recovery dispatch");
    };
    assert_eq!(
        old_successor.callback_generation(),
        old_work.callback_generation()
    );
    assert_eq!(successor.callback_generation(), work.callback_generation());
    assert_ne!(
        old_successor.callback_generation(),
        successor.callback_generation()
    );
    assert_eq!(timer_count(), 0);
    let before = timer_inventory().unwrap();
    for malformed in [
        RegistryEffect::DispatchWatchdog {
            successor,
            successor_delay_ns,
            work: old_work,
        },
        RegistryEffect::DispatchWatchdog {
            successor: old_successor,
            successor_delay_ns,
            work,
        },
    ] {
        assert!(matches!(
            apply_effect(&malformed, ProviderHandles::default()),
            Err(TimerError::OwnershipInvariant)
        ));
        assert_eq!(timer_count(), 0, "validation must precede provider arms");
        assert_eq!(timer_inventory().unwrap(), before);
    }
}

#[test]
fn remove_on_stop_provider_failure_removes_the_expired_claim() {
    let _fixture = setup();
    let timer = identity("provider-remove-on-stop-fault");
    let registration = register_once(
        timer.clone(),
        DeclarationLifetime::RemoveWhenStopped,
        |_context| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
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
    let _fixture = setup();
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
    let _fixture = setup();
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
            WatchdogDecision::ContinueImmediately,
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
fn icydb_shaped_readiness_selects_work_outcomes_and_watchdog_decisions() {
    for (readiness, work_count, completion, decision) in [
        (
            StartupReadiness::Recovering,
            1,
            TimerCompletion::success(1),
            WatchdogDecision::ContinueImmediately,
        ),
        (
            StartupReadiness::RetryableFailure,
            0,
            TimerCompletion::retryable_failure(0),
            WatchdogDecision::Continue,
        ),
        (
            StartupReadiness::Ready,
            0,
            TimerCompletion::no_work(),
            WatchdogDecision::Stop,
        ),
        (
            StartupReadiness::Ready,
            1,
            TimerCompletion::success(1),
            WatchdogDecision::Stop,
        ),
        (
            StartupReadiness::TerminalFailure,
            0,
            TimerCompletion::invariant_failure(0),
            WatchdogDecision::Stop,
        ),
    ] {
        assert_eq!(
            icydb_watchdog_result(readiness, work_count),
            WatchdogRunResult::new(completion, decision),
            "{readiness:?} with {work_count} work"
        );
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered IcyDB-shaped reconstruction and commit-guard sequence."
)]
fn icydb_shaped_reconstruction_and_commit_guard_ensure_are_synchronous_and_idempotent() {
    let _fixture = setup();
    let timer = identity("icydb-startup-watchdog");
    let cadence = TimerCadence::from_nanos(5).expect("fixture cadence should be valid");
    let readiness = Rc::new(Cell::new(StartupReadiness::Recovering));
    let pages = Rc::new(Cell::new(0_u64));
    let mut registration = None;

    let callback_readiness = Rc::clone(&readiness);
    let callback_pages = Rc::clone(&pages);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        move |_context| {
            advance_instructions(13);
            let readiness = callback_readiness.get();
            let completed_work_count = u64::from(readiness == StartupReadiness::Recovering);
            if completed_work_count != 0 {
                callback_pages.set(callback_pages.get().saturating_add(completed_work_count));
            }
            icydb_watchdog_result(readiness, completed_work_count)
        },
    )
    .expect("fresh reconstruction should register and schedule");
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
        |_context| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    )
    .expect("repeated reconstruction should reuse the original callback claim");
    assert_eq!(timer_count(), 1);

    run_icydb_watchdog_round(15);
    assert_eq!((pages.get(), timer_count()), (1, 1));

    readiness.set(StartupReadiness::RetryableFailure);
    run_icydb_watchdog_round(20);
    assert_eq!((pages.get(), timer_count()), (1, 1));
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
    run_icydb_watchdog_round(25);
    assert_eq!((pages.get(), timer_count()), (1, 0));
    let stopped = timer_snapshot(&timer)
        .expect("snapshot lookup should succeed")
        .expect("retained watchdog should remain declared");
    assert_icydb_lifecycle_measurements(&stopped);

    readiness.set(StartupReadiness::RetryableFailure);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
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
    assert_eq!(terminal_counters.succeeded(), 1);
    assert_eq!(terminal_counters.no_work(), 1);
    assert_eq!(terminal_counters.retryable_failure(), 1);

    readiness.set(StartupReadiness::Recovering);
    reconcile_watchdog(
        &mut registration,
        &timer,
        cadence,
        WatchdogReconcileState::ScheduledImmediately,
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
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .expect("terminal durable authority should cancel successor and queued work");
    assert_eq!(timer_count(), 0);

    let mismatch = reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(6).expect("fixture cadence should be valid"),
        WatchdogReconcileState::Inactive,
        |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
    );
    assert!(matches!(mismatch, Err(TimerError::ReconciliationConflict)));
}

fn run_icydb_watchdog_round(now_ns: u64) {
    set_time(now_ns);
    assert!(run_next_due(), "scheduler message must run at {now_ns}");
    assert!(run_next_due(), "work message must run at {now_ns}");
}

fn assert_icydb_lifecycle_measurements(snapshot: &TimerSnapshot) {
    let performance = snapshot.observability().performance();
    for (measurement, samples) in [
        (
            "scheduler instructions",
            performance.scheduler_instructions().samples(),
        ),
        (
            "work instructions",
            performance.work_instructions().samples(),
        ),
        (
            "scheduler memory",
            performance.scheduler_memory_pages().samples(),
        ),
        ("work memory", performance.work_memory_pages().samples()),
    ] {
        assert_eq!(
            samples, 3,
            "{measurement} after three completed work rounds"
        );
    }
    assert_eq!(performance.scheduler_instructions().latest(), Some(10));
    assert!(
        performance
            .work_instructions()
            .latest()
            .is_some_and(|value| value >= 13)
    );
}

#[test]
fn once_reconciliation_rejects_identity_mismatch_without_disturbing_live_work() {
    let _fixture = setup();
    let timer = identity("reconcile-once-identity");
    let other = identity("reconcile-once-other");
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_once(timer.clone(), DeclarationLifetime::Retained, move |_| {
            callback_calls.set(callback_calls.get() + 1);
            async { OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop) }
        })
        .unwrap(),
    );
    registration
        .as_ref()
        .unwrap()
        .ensure_scheduled(TimerSchedule::At(20))
        .unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_once(&mut registration, &other, None, |_| async {
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
    });
    assert!(matches!(rejected, Err(TimerError::ReconciliationConflict)));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    set_time(20);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "the original callback must remain installed"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn after_completion_reconciliation_rejects_transient_lifetime_without_cancelling_work() {
    let _fixture = setup();
    let timer = identity("reconcile-after-lifetime");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_after_completion(
            timer.clone(),
            cadence,
            DeclarationLifetime::RemoveWhenStopped,
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap(),
    );
    registration.as_ref().unwrap().ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Inactive,
        |_| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    );
    assert!(matches!(rejected, Err(TimerError::ReconciliationConflict)));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    set_time(15);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "the original callback must remain installed"
    );
    assert_eq!(timer_count(), 0);
    assert!(timer_snapshot(&timer).unwrap().is_none());
}

#[test]
fn after_completion_reconciliation_rejects_cadence_mismatch_without_disturbing_live_work() {
    let _fixture = setup();
    let timer = identity("reconcile-after-cadence");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let other_cadence = TimerCadence::from_nanos(6).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let callback_calls = Rc::clone(&calls);
    let mut registration = Some(
        register_after_completion(
            timer.clone(),
            cadence,
            DeclarationLifetime::Retained,
            move |_| {
                callback_calls.set(callback_calls.get() + 1);
                async {
                    AfterCompletionRunResult::new(
                        TimerCompletion::success(1),
                        AfterCompletionDecision::Stop,
                    )
                }
            },
        )
        .unwrap(),
    );
    registration.as_ref().unwrap().ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_after_completion(
        &mut registration,
        &timer,
        other_cadence,
        TimerReconcileState::Inactive,
        |_| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
        },
    );
    assert!(matches!(rejected, Err(TimerError::ReconciliationConflict)));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    let original = registration.as_ref().unwrap();
    assert_eq!(original.identity(), &timer);
    assert!(original.has_armed_wakeup().unwrap());

    set_time(14);
    assert!(!run_next_due());
    assert_eq!(calls.get(), 0);
    set_time(15);
    assert!(run_next_due());
    assert_eq!(
        calls.get(),
        1,
        "the original callback must run at its original deadline"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_reconciliation_rejects_reused_identity_claim_without_clearing_replacement() {
    let _fixture = setup();
    let timer = identity("reconcile-watchdog-expired");
    let cadence = TimerCadence::from_nanos(5).unwrap();
    let calls = Rc::new(Cell::new(0_u64));
    let expired_calls = Rc::clone(&calls);
    let mut expired = Some(
        register_watchdog(
            timer.clone(),
            cadence,
            DeclarationLifetime::RemoveWhenStopped,
            move |_| {
                expired_calls.set(expired_calls.get() + 1);
                WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
            },
        )
        .unwrap(),
    );
    expired.as_ref().unwrap().ensure_scheduled().unwrap();
    expired.as_ref().unwrap().cancel().unwrap();
    assert!(timer_snapshot(&timer).unwrap().is_none());
    let replacement_count = Rc::new(Cell::new(0_u64));
    let replacement_calls = Rc::clone(&replacement_count);
    let replacement = register_watchdog(
        timer.clone(),
        cadence,
        DeclarationLifetime::Retained,
        move |_| {
            replacement_calls.set(replacement_calls.get() + 1);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    replacement.ensure_scheduled().unwrap();
    let before = timer_inventory().unwrap();

    let rejected = reconcile_watchdog(
        &mut expired,
        &timer,
        cadence,
        WatchdogReconcileState::Inactive,
        |_| {
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(0),
                WatchdogDecision::Stop,
            )
        },
    );
    assert!(matches!(rejected, Err(TimerError::RegistrationExpired)));
    assert!(matches!(
        expired.as_ref().unwrap().has_armed_wakeup(),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_inventory().unwrap(), before);
    assert_eq!(timer_count(), 1);
    assert!(replacement.has_armed_wakeup().unwrap());

    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 0, "expired work must not execute");
    assert_eq!(
        replacement_count.get(),
        1,
        "replacement work must execute once"
    );
    assert_eq!(timer_count(), 0);
}

#[test]
fn after_completion_reconstruction_reuses_its_exact_claim() {
    let _fixture = setup();
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
            async {
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::Stop,
                )
            }
        },
    )
    .expect("fresh reconstruction should succeed");
    reconcile_after_completion(
        &mut registration,
        &timer,
        cadence,
        TimerReconcileState::Scheduled,
        |_context| async {
            AfterCompletionRunResult::new(
                TimerCompletion::invariant_failure(0),
                AfterCompletionDecision::Stop,
            )
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
    let _fixture = setup();
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
            async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
        },
    )
    .expect("fresh reconstruction should register and arm");
    reconcile_once(
        &mut registration,
        &timer,
        Some(TimerSchedule::At(40)),
        |_context| async {
            OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
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
        OnceRunResult::new(TimerCompletion::invariant_failure(0), OnceDecision::Stop)
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

#[test]
fn registration_identity_survives_control_but_changes_on_replacement_and_restart() {
    let fixture = setup();
    let epoch = fixture.epoch;
    let timer = identity("continuity");
    let create = || {
        register_once(timer.clone(), DeclarationLifetime::Retained, |_| async {
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        })
        .unwrap()
    };
    let snapshot = || timer_snapshot(&timer).unwrap().unwrap();
    let first = create();
    let original = snapshot().registration_id();
    assert_eq!(original.epoch(), epoch);
    assert_eq!(original.sequence(), 1);
    for _ in 0..2 {
        first.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    first.cancel().unwrap();
    let before = snapshot();
    assert_eq!(before.registration_id(), original);
    first.unregister().unwrap();
    let replacement = create();
    assert_ne!(snapshot().registration_id(), original);
    assert_eq!(snapshot().registration_id().epoch(), epoch);
    for _ in 0..3 {
        replacement.ensure_scheduled(TimerSchedule::At(10)).unwrap();
        assert!(run_next_due());
    }
    let regrown = snapshot();
    assert!(
        regrown.observability().counters().work_completed()
            > before.observability().counters().work_completed()
    );
    assert_ne!(regrown.registration_id(), before.registration_id());
    reset_for_test(20, 8);
    initialize_runtime().unwrap();
    let _reconstructed = create();
    assert_eq!(snapshot().registration_id().sequence(), original.sequence());
    assert_ne!(snapshot().registration_id().epoch(), original.epoch());
    assert_ne!(snapshot().registration_id(), original);
}

#[test]
fn watchdog_exact_deadline_reconstruction_moves_both_directions_and_sleeps() {
    let _fixture = setup();
    let timer = identity("deadline");
    let mut registration = None;
    reconcile_watchdog(
        &mut registration,
        &timer,
        TimerCadence::from_nanos(5).unwrap(),
        WatchdogReconcileState::ScheduledAt(20),
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    let registration = registration.unwrap();
    let initial = timer_snapshot(&timer).unwrap().unwrap();
    for deadline in [40, 15, 15] {
        registration
            .reconcile_schedule(Some(TimerSchedule::At(deadline)))
            .unwrap();
        let current = timer_snapshot(&timer).unwrap().unwrap();
        assert_eq!(current.next_deadline_ns(), Some(deadline));
        assert_eq!(current.registration_id(), initial.registration_id());
        assert_eq!(timer_count(), 1);
    }
    let scheduled = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(scheduled.observability().counters().wakeups_armed(), 3);
    assert_eq!(scheduled.observability().counters().coalesced(), 1);
    assert!(!run_next_due());
    registration.reconcile_schedule(None).unwrap();
    assert_eq!(timer_count(), 0);
    registration
        .reconcile_schedule(Some(TimerSchedule::At(0)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer)
            .unwrap()
            .unwrap()
            .latest_armed_delay_ns(),
        Some(0)
    );
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(timer_count(), 0);
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().registration_id(),
        initial.registration_id()
    );
}

#[test]
fn watchdog_deadline_decision_replaces_only_the_prearmed_successor() {
    let _fixture = setup();
    let timer = identity("deadline-result");
    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        move |_| {
            callback_calls.set(callback_calls.get() + 1);
            assert_eq!(timer_count(), 1);
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                if callback_calls.get() == 1 {
                    WatchdogDecision::ScheduleAt(100)
                } else {
                    WatchdogDecision::Stop
                },
            )
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let completed = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(completed.next_deadline_ns(), Some(100));
    assert_eq!(completed.scheduling_mode(), TimerSchedulingMode::Deadline);
    assert_eq!(completed.latest_requested_delay_ns(), None);
    assert_eq!(completed.latest_armed_delay_ns(), Some(90));
    assert_eq!(timer_count(), 1);
    set_time(99);
    assert!(!run_next_due());
    set_time(100);
    assert!(run_next_due());
    assert!(run_next_due());
    assert_eq!(calls.get(), 2);
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_dispatched_reconciliation_preserves_recovery_until_completion() {
    let _fixture = setup();
    let timer = identity("dispatched-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(15)
    );
    assert_eq!(timer_count(), 2);
    assert!(run_next_due());
    assert_eq!(
        timer_snapshot(&timer).unwrap().unwrap().next_deadline_ns(),
        Some(100)
    );
    assert_eq!(timer_count(), 1);
    registration.cancel().unwrap();
    assert_eq!(timer_count(), 0);
}

#[test]
fn watchdog_deadline_context_arbitrates_and_expires() {
    let _fixture = setup();
    let timer = identity("deadline-context");
    let saved = Rc::new(RefCell::new(None));
    let callback_saved = Rc::clone(&saved);
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        move |context| {
            context.cancel().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            context
                .reconcile_schedule(Some(TimerSchedule::After(Duration::from_nanos(30))))
                .unwrap();
            context.ensure_scheduled().unwrap();
            *callback_saved.borrow_mut() = Some(context);
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    assert!(run_next_due());
    let current = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(current.next_deadline_ns(), Some(40));
    assert_eq!(current.latest_requested_delay_ns(), Some(30));
    assert!(matches!(
        saved
            .borrow()
            .as_ref()
            .unwrap()
            .reconcile_schedule(Some(TimerSchedule::At(60))),
        Err(TimerError::RegistrationExpired)
    ));
    assert!(matches!(
        saved.borrow().as_ref().unwrap().reconcile_schedule(None),
        Err(TimerError::RegistrationExpired)
    ));
    assert_eq!(timer_count(), 1);
}

#[test]
fn invalid_watchdog_deadline_request_preserves_the_live_schedule() {
    let _fixture = setup();
    let timer = identity("invalid-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        |_| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    let before = timer_snapshot(&timer).unwrap();
    for delay in [
        Duration::from_secs(u64::MAX),
        Duration::from_nanos(u64::MAX),
    ] {
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::After(delay))),
            Err(TimerError::Schedule(_))
        ));
        assert_eq!(timer_snapshot(&timer).unwrap(), before);
        assert_eq!(timer_count(), 1);
    }
}

#[test]
fn transient_watchdog_deadline_cancellation_clears_both_owned_handles() {
    for dispatched in [false, true] {
        let _fixture = setup();
        let timer = identity("transient-deadline");
        let registration = register_watchdog(
            timer.clone(),
            TimerCadence::from_nanos(5).unwrap(),
            DeclarationLifetime::RemoveWhenStopped,
            |_| panic!("cancelled work must not execute"),
        )
        .unwrap();
        registration
            .reconcile_schedule(Some(TimerSchedule::At(10)))
            .unwrap();
        if dispatched {
            assert!(run_next_due());
        }
        assert_eq!(timer_count(), if dispatched { 2 } else { 1 });
        registration.reconcile_schedule(None).unwrap();
        assert!(timer_snapshot(&timer).unwrap().is_none());
        assert_eq!(timer_count(), 0);
        assert!(matches!(
            registration.reconcile_schedule(Some(TimerSchedule::At(20))),
            Err(TimerError::RegistrationExpired)
        ));
    }
}

#[test]
fn recovery_retires_an_interrupted_attempts_exact_deadline_proposal() {
    let _fixture = setup();
    let timer = identity("retired-deadline");
    let registration = register_watchdog(
        timer.clone(),
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        |_| WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
    )
    .unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert!(run_next_due());
    let before = timer_snapshot(&timer).unwrap().unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(100)))
        .unwrap();
    assert!(discard_next_due());
    set_time(15);
    assert!(run_next_due());
    assert!(run_next_due());
    let recovered = timer_snapshot(&timer).unwrap().unwrap();
    assert_eq!(recovered.next_deadline_ns(), Some(20));
    assert_eq!(recovered.observability().counters().unacknowledged(), 1);
    assert_eq!(recovered.registration_id(), before.registration_id());
    assert_eq!(timer_count(), 1);
}

#[test]
fn coalesced_watchdog_requests_preserve_paired_handles_without_reinstallation() {
    let _fixture = setup();
    let timer = identity("paired-request-preservation");
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::Retained,
        |context| {
            inject_provider_install_fault();
            context.ensure_scheduled().unwrap();
            context.ensure_scheduled_immediately().unwrap();
            // The pending immediate request would replace the successor; Stop
            // via exact cancellation wins and needs no handle installation.
            context.cancel().unwrap();
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration.ensure_scheduled().unwrap();
    set_time(15);
    assert!(run_next_due());
    assert_eq!(timer_count(), 2);
    inject_provider_install_fault();
    registration.ensure_scheduled().unwrap();
    registration.ensure_scheduled_immediately().unwrap();
    assert_eq!(timer_count(), 2);
    // Read/reset the injection so actual work can complete without an unrelated
    // injected failure; a no-op demand must not have consumed it.
    assert!(take_provider_install_fault());
    assert!(run_next_due());
    assert!(take_provider_install_fault());
    assert_eq!(timer_count(), 0);
    registration.unregister().unwrap();
}

struct RemovalCapture {
    timer: TimerIdentity,
    dropped: Rc<Cell<bool>>,
}

impl Drop for RemovalCapture {
    fn drop(&mut self) {
        assert!(timer_snapshot(&self.timer).unwrap().is_none());
        assert_eq!(
            timer_count(),
            0,
            "provider cleanup must precede capture Drop"
        );
        // Reuse the removed identity and capacity from consumer destruction.
        // This also proves nested mutation can run without a registry borrow.
        let replacement = register_once(
            self.timer.clone(),
            DeclarationLifetime::Retained,
            |_| async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) },
        )
        .unwrap();
        replacement.ensure_scheduled(TimerSchedule::At(20)).unwrap();
        replacement.unregister().unwrap();
        self.dropped.set(true);
    }
}

#[test]
fn cancellation_and_unregistration_release_captures_after_provider_cleanup() {
    for policy in 0..3 {
        for armed in [false, true] {
            for unregister in [false, true] {
                let _fixture = setup();
                let timer = identity("normal-removal-capture");
                let dropped = Rc::new(Cell::new(false));
                let captured = RemovalCapture {
                    timer: timer.clone(),
                    dropped: Rc::clone(&dropped),
                };
                let lifetime = if unregister {
                    DeclarationLifetime::Retained
                } else {
                    DeclarationLifetime::RemoveWhenStopped
                };
                let claim = match policy {
                    0 => {
                        register_once(timer, lifetime, move |_| {
                            std::hint::black_box(&captured);
                            async {
                                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
                            }
                        })
                        .unwrap()
                        .claim
                    }
                    1 => {
                        register_after_completion(
                            timer,
                            TimerCadence::from_nanos(5).unwrap(),
                            lifetime,
                            move |_| {
                                std::hint::black_box(&captured);
                                async {
                                    AfterCompletionRunResult::new(
                                        TimerCompletion::no_work(),
                                        AfterCompletionDecision::Stop,
                                    )
                                }
                            },
                        )
                        .unwrap()
                        .claim
                    }
                    _ => {
                        register_watchdog(
                            timer,
                            TimerCadence::from_nanos(5).unwrap(),
                            lifetime,
                            move |_| {
                                std::hint::black_box(&captured);
                                WatchdogRunResult::new(
                                    TimerCompletion::no_work(),
                                    WatchdogDecision::Stop,
                                )
                            },
                        )
                        .unwrap()
                        .claim
                    }
                };
                if armed {
                    if policy == 0 {
                        ensure_once_claim(&claim, None, TimerSchedule::At(20)).unwrap();
                    } else {
                        ensure_recurring_claim(&claim, None).unwrap();
                    }
                }
                if unregister {
                    unregister_claim(&claim).unwrap();
                } else {
                    cancel_claim(&claim, None).unwrap();
                }
                assert!(dropped.get());
            }
        }
    }
}

#[test]
fn rejected_registration_releases_its_captures_outside_registry_borrow() {
    struct RejectedCapture(Rc<Cell<bool>>);
    impl Drop for RejectedCapture {
        fn drop(&mut self) {
            assert_eq!(timer_inventory().unwrap().len(), 1);
            self.0.set(true);
        }
    }
    let _fixture = setup();
    let timer = identity("duplicate-capture");
    let original = register_once(timer.clone(), DeclarationLifetime::Retained, |_| async {
        OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
    })
    .unwrap();
    for policy in 0..3 {
        let dropped = Rc::new(Cell::new(false));
        let captured = RejectedCapture(Rc::clone(&dropped));
        let rejected = match policy {
            0 => register_once(timer.clone(), DeclarationLifetime::Retained, move |_| {
                std::hint::black_box(&captured);
                async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
            })
            .map(|_| ()),
            1 => register_after_completion(
                timer.clone(),
                TimerCadence::from_nanos(5).unwrap(),
                DeclarationLifetime::Retained,
                move |_| {
                    std::hint::black_box(&captured);
                    async {
                        AfterCompletionRunResult::new(
                            TimerCompletion::no_work(),
                            AfterCompletionDecision::Stop,
                        )
                    }
                },
            )
            .map(|_| ()),
            _ => register_watchdog(
                timer.clone(),
                TimerCadence::from_nanos(5).unwrap(),
                DeclarationLifetime::Retained,
                move |_| {
                    std::hint::black_box(&captured);
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
                },
            )
            .map(|_| ()),
        };
        assert!(matches!(
            rejected,
            Err(TimerError::Register(
                RegisterError::IdentityAlreadyRegistered(_)
            ))
        ));
        assert!(dropped.get());
    }
    original.unregister().unwrap();
}

#[test]
fn terminal_provider_and_scheduler_failures_release_captures_outside_registry_borrow() {
    let _fixture = setup();
    let timer = identity("binding-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let registration = register_once(timer, DeclarationLifetime::RemoveWhenStopped, move |_| {
        std::hint::black_box(&captured);
        async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
    })
    .unwrap();
    inject_provider_install_fault();
    assert!(matches!(
        registration.ensure_scheduled(TimerSchedule::At(20)),
        Err(TimerError::OwnershipInvariant)
    ));
    assert!(dropped.get());

    let timer = identity("scheduler-failure-capture");
    let dropped = Rc::new(Cell::new(false));
    let captured = RemovalCapture {
        timer: timer.clone(),
        dropped: Rc::clone(&dropped),
    };
    let registration = register_watchdog(
        timer,
        TimerCadence::from_nanos(5).unwrap(),
        DeclarationLifetime::RemoveWhenStopped,
        move |_| {
            std::hint::black_box(&captured);
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
        },
    )
    .unwrap();
    registration
        .reconcile_schedule(Some(TimerSchedule::At(u64::MAX)))
        .unwrap();
    set_time(u64::MAX);
    assert!(run_next_due());
    assert!(dropped.get());
    assert_eq!(timer_inventory().unwrap().timers(), []);
}
