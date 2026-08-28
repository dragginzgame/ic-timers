use super::*;
use crate::{TimerLastOutcome, TimerProcessCondition, TimerRegistrationStatus};
use std::time::Duration;
fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("test", "registry", name).expect("fixture identity should be valid")
}

fn registry() -> TimerRegistry {
    TimerRegistry::new(TimerEpoch::new(7, 10))
}

fn cadence(nanoseconds: u64) -> TimerCadence {
    TimerCadence::from_nanos(nanoseconds).expect("fixture cadence should be valid")
}

fn arm(transition: RegistryTransition) -> (CallbackToken, u64, WakeupArm) {
    match transition.into_effect() {
        RegistryEffect::ArmWakeup {
            token,
            deadline_ns,
            arm,
            ..
        } => (token, deadline_ns, arm),
        effect => panic!("expected arm effect, got {effect:?}"),
    }
}

fn dispatch(transition: RegistryTransition) -> (CallbackToken, u64, CallbackToken) {
    match transition.into_effect() {
        RegistryEffect::DispatchWatchdog {
            successor,
            successor_deadline_ns,
            work,
            ..
        } => (successor, successor_deadline_ns, work),
        effect => panic!("expected watchdog dispatch, got {effect:?}"),
    }
}

fn confirm(registry: &mut TimerRegistry, transition: &RegistryTransition) {
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
}

#[test]
fn duplicate_registration_and_capacity_fail_without_partial_state() {
    let mut registry = registry();
    let first = identity("timer-00");
    registry
        .register_once(first.clone(), DeclarationLifetime::Retained)
        .expect("first claim should succeed");

    assert_eq!(
        registry.register_watchdog(first.clone(), cadence(1), DeclarationLifetime::Retained,),
        Err(RegisterError::IdentityAlreadyRegistered(first))
    );
    assert_eq!(registry.len(), 1);

    for index in 1..MAX_TIMER_REGISTRATIONS {
        registry
            .register_once(
                identity(&format!("timer-{index:02}")),
                DeclarationLifetime::Retained,
            )
            .expect("registration within capacity should succeed");
    }
    assert_eq!(registry.len(), MAX_TIMER_REGISTRATIONS);

    assert_eq!(
        registry.register_once(identity("overflow"), DeclarationLifetime::Retained),
        Err(RegisterError::CapacityExceeded {
            max: MAX_TIMER_REGISTRATIONS,
        })
    );
    assert_eq!(registry.len(), MAX_TIMER_REGISTRATIONS);

    let names = registry
        .inventory()
        .into_timers()
        .into_iter()
        .map(|snapshot| snapshot.identity().name().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(names.first().map(String::as_str), Some("timer-00"));
    assert_eq!(names.last().map(String::as_str), Some("timer-63"));
}

#[test]
fn removal_invalidates_old_claim_before_identity_reuse() {
    let mut registry = registry();
    let timer = identity("reused");
    let old = registry
        .register_once(timer.clone(), DeclarationLifetime::RemoveWhenStopped)
        .expect("first claim should succeed");
    let old_generation = old.claim_generation();
    let _ = arm(registry
        .ensure_once(&old, 10, TimerSchedule::At(20))
        .expect("ensure should succeed"));
    registry.cancel(&old).expect("cancel should succeed");
    assert!(registry.is_empty());
    assert_eq!(
        registry.ensure_once(&old, 10, TimerSchedule::At(20)),
        Err(RegistryError::UnknownRegistration)
    );

    let new = registry
        .register_once(timer, DeclarationLifetime::Retained)
        .expect("identity can be claimed again after removal");
    assert!(new.claim_generation() > old_generation);
    assert_eq!(
        registry.ensure_once(&old, 10, TimerSchedule::At(20)),
        Err(RegistryError::StaleRegistration)
    );
}

#[test]
fn fresh_remove_when_stopped_cancellation_releases_every_policy() {
    let mut registry = registry();

    let once = registry
        .register_once(
            identity("fresh-transient-once"),
            DeclarationLifetime::RemoveWhenStopped,
        )
        .expect("once claim should succeed");
    registry.cancel(&once).expect("once cancel should succeed");
    assert!(registry.is_empty());

    let after = registry
        .register_after_completion(
            identity("fresh-transient-after"),
            cadence(1),
            DeclarationLifetime::RemoveWhenStopped,
        )
        .expect("after-completion claim should succeed");
    registry
        .cancel(&after)
        .expect("after-completion cancel should succeed");
    assert!(registry.is_empty());

    let watchdog = registry
        .register_watchdog(
            identity("fresh-transient-watchdog"),
            cadence(1),
            DeclarationLifetime::RemoveWhenStopped,
        )
        .expect("watchdog claim should succeed");
    registry
        .cancel(&watchdog)
        .expect("watchdog cancel should succeed");
    assert!(registry.is_empty());
}

#[test]
fn measurement_routing_rejects_a_policy_role_mismatch() {
    let mut registry = registry();
    let timer = identity("measurement-role");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("once claim should succeed");
    let invalid = CallbackToken::new(
        timer.clone(),
        claim.claim_generation(),
        1,
        CallbackRole::WatchdogScheduler,
    );
    let pages = crate::platform::memory_pages();

    assert_eq!(
        registry.ensure_watchdog_immediately(&claim, 10),
        Err(RegistryError::PolicyMismatch { actual: "once" })
    );

    assert_eq!(
        registry.record_callback_measurements(&invalid, 10, pages, pages),
        Err(RegistryError::PolicyMismatch { actual: "once" })
    );
    let performance = registry
        .snapshot(&timer)
        .expect("retained declaration should remain")
        .observability()
        .performance();
    assert_eq!(performance.scheduler_instructions().samples(), 0);
    assert_eq!(performance.work_instructions().samples(), 0);
    assert_eq!(performance.scheduler_memory_pages().samples(), 0);
    assert_eq!(performance.work_memory_pages().samples(), 0);
}

#[test]
fn explicit_unregistration_consumes_scheduled_and_running_claims() {
    let mut registry = registry();
    let scheduled_id = identity("unregister-scheduled");
    let scheduled = registry
        .register_once(scheduled_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let scheduled_transition = registry
        .ensure_once(&scheduled, 0, TimerSchedule::At(10))
        .expect("ensure should succeed");
    confirm(&mut registry, &scheduled_transition);
    let (queued, _, _) = arm(scheduled_transition);
    let transition = registry
        .unregister(&scheduled)
        .expect("scheduled unregistration should succeed");
    assert_eq!(
        transition.effect(),
        &RegistryEffect::ClearCallbacks {
            identity: scheduled_id.clone(),
            handles: CallbacksToClear::Wakeup,
        }
    );
    assert!(registry.snapshot(&scheduled_id).is_none());
    assert_eq!(registry.begin_ordinary(&queued), CallbackAcceptance::Stale);

    let running_id = identity("unregister-running");
    let running = registry
        .register_once(running_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let running_transition = registry
        .ensure_once(&running, 0, TimerSchedule::At(10))
        .expect("ensure should succeed");
    confirm(&mut registry, &running_transition);
    let (active, _, _) = arm(running_transition);
    assert_eq!(
        registry.begin_ordinary(&active),
        CallbackAcceptance::Accepted
    );
    assert_eq!(
        registry
            .unregister(&running)
            .expect("running unregistration should defer")
            .effect(),
        &RegistryEffect::None
    );
    registry
        .complete_ordinary(
            &active,
            11,
            TimerRunResult::new(
                TimerCompletion::success(1),
                TimerDirective::ContinueImmediately,
            ),
        )
        .expect("normal completion should finalize removal");
    assert!(registry.snapshot(&running_id).is_none());
}

#[test]
fn running_watchdog_unregistration_clears_its_committed_successor() {
    let mut registry = registry();
    let timer = identity("unregister-watchdog");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (scheduler, _, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 10);
    confirm(&mut registry, &dispatched);
    let (_successor, _, work) = dispatch(dispatched);
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    assert_eq!(
        registry
            .unregister(&claim)
            .expect("running unregistration should defer")
            .effect(),
        &RegistryEffect::None
    );
    let completed = registry
        .complete_watchdog_work(
            &work,
            11,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
        )
        .expect("normal completion should finalize removal");
    assert_eq!(
        completed.effect(),
        &RegistryEffect::ClearCallbacks {
            identity: timer.clone(),
            handles: CallbacksToClear::Wakeup,
        }
    );
    assert!(registry.snapshot(&timer).is_none());
}

#[test]
fn watchdog_dispatch_confirmation_rejects_cross_claim_work() {
    let mut registry = registry();
    let first_id = identity("dispatch-claim-first");
    let second_id = identity("dispatch-claim-second");
    let first = registry
        .register_watchdog(first_id.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("first claim should succeed");
    let second = registry
        .register_watchdog(second_id, cadence(5), DeclarationLifetime::Retained)
        .expect("second claim should succeed");

    let (first_scheduler, _, _) = arm(registry
        .ensure_recurring(&first, 0)
        .expect("first ensure should succeed"));
    let (second_scheduler, _, _) = arm(registry
        .ensure_recurring(&second, 0)
        .expect("second ensure should succeed"));
    let (first_successor, first_deadline, _) =
        dispatch(registry.begin_watchdog_scheduler(&first_scheduler, 5));
    let (_, _, second_work) = dispatch(registry.begin_watchdog_scheduler(&second_scheduler, 5));
    let malformed_arm = RegistryEffect::ArmWakeup {
        token: second_work.clone(),
        deadline_ns: first_deadline,
        delay_ns: 0,
        arm: WakeupArm::Initial,
    };
    assert_eq!(
        registry.confirm_effect_applied(&malformed_arm),
        Err(RegistryError::StaleCallback)
    );
    let malformed_replacement = RegistryEffect::ArmWakeup {
        token: first_successor.clone(),
        deadline_ns: first_deadline,
        delay_ns: 5,
        arm: WakeupArm::Replacement,
    };
    assert_eq!(
        registry.confirm_effect_applied(&malformed_replacement),
        Err(RegistryError::StaleCallback)
    );
    let malformed = RegistryEffect::DispatchWatchdog {
        successor: first_successor,
        successor_deadline_ns: first_deadline,
        successor_delay_ns: 5,
        work: second_work,
    };

    assert_eq!(
        registry.confirm_effect_applied(&malformed),
        Err(RegistryError::StaleCallback)
    );
    let counters = registry
        .snapshot(&first_id)
        .expect("first snapshot should remain")
        .observability()
        .counters();
    assert_eq!(counters.wakeups_armed(), 0);
    assert_eq!(counters.work_dispatched(), 0);
}

#[test]
fn once_coalesces_and_rotates_generations_while_nested_schedule_wins() {
    let mut registry = registry();
    let timer = identity("once");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let first_transition = registry
        .ensure_once(&claim, 10, TimerSchedule::At(100))
        .expect("initial ensure should succeed");
    confirm(&mut registry, &first_transition);
    confirm(&mut registry, &first_transition);
    assert_eq!(
        registry
            .snapshot(&timer)
            .expect("snapshot should exist")
            .observability()
            .counters()
            .wakeups_armed(),
        1
    );
    let (first, deadline, arm_kind) = arm(first_transition);
    assert_eq!(deadline, 100);
    assert_eq!(arm_kind, WakeupArm::Initial);
    assert_eq!(first.callback_generation(), 1);

    let duplicate = registry
        .ensure_once(&claim, 10, TimerSchedule::At(200))
        .expect("duplicate ensure should succeed");
    assert_eq!(duplicate.effect(), &RegistryEffect::None);
    assert_eq!(
        registry.begin_ordinary(&first),
        CallbackAcceptance::Accepted
    );

    let nested = registry
        .ensure_once(&claim, 20, TimerSchedule::At(80))
        .expect("nested ensure should succeed");
    assert_eq!(nested.effect(), &RegistryEffect::None);
    let second_transition = registry
        .complete_ordinary(
            &first,
            30,
            TimerRunResult::new(TimerCompletion::success(1), TimerDirective::Stop),
        )
        .expect("completion should succeed");
    confirm(&mut registry, &second_transition);
    let (second, deadline, _) = arm(second_transition);
    assert_eq!(deadline, 80);
    assert_eq!(second.callback_generation(), 2);
    assert_eq!(registry.begin_ordinary(&first), CallbackAcceptance::Stale);
    assert_eq!(
        registry.begin_ordinary(&second),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_ordinary(
            &second,
            40,
            TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop),
        )
        .expect("terminal completion should succeed");

    let snapshot = registry.snapshot(&timer).expect("retained snapshot exists");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        }
    );
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests(), 3);
    assert_eq!(counters.wakeups_armed(), 2);
    assert_eq!(counters.coalesced(), 2);
    assert_eq!(counters.stale_wakeups(), 1);
    assert_eq!(counters.work_started(), 2);
    assert_eq!(counters.work_completed(), 2);
    assert!(counters.completion_partition_is_valid());
}

#[test]
fn nested_cancel_and_ensure_use_latest_request_order() {
    let mut registry = registry();
    let timer = identity("nested");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let (first, _, _) = arm(registry
        .ensure_once(&claim, 0, TimerSchedule::At(10))
        .expect("ensure should succeed"));
    assert_eq!(
        registry.begin_ordinary(&first),
        CallbackAcceptance::Accepted
    );
    registry
        .ensure_once(&claim, 10, TimerSchedule::At(30))
        .expect("nested ensure should succeed");
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    let cancelled = registry
        .complete_ordinary(
            &first,
            11,
            TimerRunResult::new(
                TimerCompletion::success(1),
                TimerDirective::ContinueImmediately,
            ),
        )
        .expect("completion should succeed");
    assert_eq!(cancelled.effect(), &RegistryEffect::None);
    assert_eq!(
        registry.snapshot(&timer).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled,
        })
    );

    let (second, _, _) = arm(registry
        .ensure_once(&claim, 20, TimerSchedule::At(40))
        .expect("retained declaration should re-enable"));
    assert_eq!(
        registry.begin_ordinary(&second),
        CallbackAcceptance::Accepted
    );
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    registry
        .ensure_once(&claim, 21, TimerSchedule::At(50))
        .expect("later nested ensure should succeed");
    let (_, deadline, _) = arm(registry
        .complete_ordinary(
            &second,
            22,
            TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop),
        )
        .expect("later ensure should override cancellation"));
    assert_eq!(deadline, 50);
}

#[test]
fn ordinary_reconciliation_is_authoritative_and_registry_pending_is_ordered() {
    let mut registry = registry();
    let timer = identity("reconcile-ordinary");
    let claim = registry
        .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");

    let (replaced, deadline, arm_kind) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(100)))
        .expect("initial reconciliation should arm"));
    assert_eq!(deadline, 100);
    assert_eq!(arm_kind, WakeupArm::Initial);
    let (current, deadline, arm_kind) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(200)))
        .expect("authoritative reconciliation may move later"));
    assert_eq!(deadline, 200);
    assert_eq!(arm_kind, WakeupArm::Replacement);
    assert_eq!(
        registry.begin_ordinary(&replaced),
        CallbackAcceptance::Stale
    );
    assert_eq!(
        registry.begin_ordinary(&current),
        CallbackAcceptance::Accepted
    );

    registry
        .ensure_recurring(&claim, 200)
        .expect("nested ensure should establish an earlier pending schedule");
    registry
        .reconcile_ordinary(&claim, 200, Some(TimerSchedule::At(300)))
        .expect("later authoritative request should supersede the ensure");
    let (successor, deadline, _) = arm(registry
        .complete_ordinary(
            &current,
            201,
            TimerRunResult::new(TimerCompletion::success(1), TimerDirective::ScheduleAt(225)),
        )
        .expect("authoritative request should replace the callback directive"));
    assert_eq!(deadline, 300);
    assert_eq!(
        registry.begin_ordinary(&successor),
        CallbackAcceptance::Accepted
    );

    registry
        .reconcile_ordinary(&claim, 300, Some(TimerSchedule::At(400)))
        .expect("nested reconciliation should succeed");
    registry
        .ensure_recurring(&claim, 301)
        .expect("a later ensure should supersede reconciliation by request order");
    let (_, deadline, _) = arm(registry
        .complete_ordinary(
            &successor,
            302,
            TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::ScheduleAt(375)),
        )
        .expect("completion should use the ordered pending request"));
    assert_eq!(deadline, 306);

    registry
        .reconcile_ordinary(&claim, 400, None)
        .expect("inactive reconciliation should cancel the retained declaration");
    assert_eq!(
        registry.snapshot(&timer).map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled,
        })
    );
}

#[test]
fn after_completion_owns_cadence_and_failure_state() {
    let mut registry = registry();
    let timer = identity("after-completion");
    let claim = registry
        .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (first, deadline, _) = arm(registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed"));
    assert_eq!(deadline, 15);
    assert_eq!(
        registry.begin_ordinary(&first),
        CallbackAcceptance::Accepted
    );
    let (second, deadline, _) = arm(registry
        .complete_ordinary(
            &first,
            20,
            TimerRunResult::new(
                TimerCompletion::retryable_failure(2),
                TimerDirective::RecurAfterCompletion,
            ),
        )
        .expect("recurrence should succeed"));
    assert_eq!(deadline, 25);
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(1));
    assert_eq!(
        registry.begin_ordinary(&second),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_ordinary(
            &second,
            30,
            TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop),
        )
        .expect("stop should succeed");
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(0));
}

#[test]
fn ordinary_directive_matrix_preserves_mode_and_checked_deadline() {
    let mut registry = registry();
    let timer = identity("directive-matrix");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_once(&claim, 0, TimerSchedule::At(1))
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (first, _, _) = arm(initial);
    assert_eq!(
        registry.begin_ordinary(&first),
        CallbackAcceptance::Accepted
    );

    let immediate = registry
        .complete_ordinary(
            &first,
            10,
            TimerRunResult::new(
                TimerCompletion::success(1),
                TimerDirective::ContinueImmediately,
            ),
        )
        .expect("immediate continuation should succeed");
    confirm(&mut registry, &immediate);
    let (second, deadline, _) = arm(immediate);
    assert_eq!(deadline, 10);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.scheduling_mode()),
        Some(TimerSchedulingMode::Continuation)
    );
    assert_eq!(
        registry.begin_ordinary(&second),
        CallbackAcceptance::Accepted
    );

    let retry = registry
        .complete_ordinary(
            &second,
            20,
            TimerRunResult::new(
                TimerCompletion::retryable_failure(0),
                TimerDirective::RetryAfter(Duration::from_nanos(5)),
            ),
        )
        .expect("retry should succeed");
    confirm(&mut registry, &retry);
    let (third, deadline, _) = arm(retry);
    assert_eq!(deadline, 25);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.process_condition()),
        Some(TimerProcessCondition::Retrying)
    );
    assert_eq!(
        registry.begin_ordinary(&third),
        CallbackAcceptance::Accepted
    );

    let absolute = registry
        .complete_ordinary(
            &third,
            30,
            TimerRunResult::new(TimerCompletion::success(1), TimerDirective::ScheduleAt(40)),
        )
        .expect("absolute scheduling should succeed");
    confirm(&mut registry, &absolute);
    let (fourth, deadline, _) = arm(absolute);
    assert_eq!(deadline, 40);
    assert_eq!(
        registry
            .snapshot(&timer)
            .map(|value| value.scheduling_mode()),
        Some(TimerSchedulingMode::Deadline)
    );
    assert_eq!(
        registry.begin_ordinary(&fourth),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_ordinary(
            &fourth,
            41,
            TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop),
        )
        .expect("stop should succeed");
    assert_eq!(
        registry.snapshot(&timer).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Stopped,
        })
    );
}

#[test]
fn recurring_duplicate_ensure_is_idempotent_without_deadline_recalculation() {
    let mut registry = registry();
    let timer = identity("recurring-idempotent");
    let claim = registry
        .register_after_completion(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 0)
        .expect("initial ensure should succeed");
    confirm(&mut registry, &initial);
    let duplicate = registry
        .ensure_recurring(&claim, u64::MAX)
        .expect("existing schedule should satisfy duplicate demand");
    assert_eq!(duplicate.effect(), &RegistryEffect::None);
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(5));
    assert_eq!(snapshot.observability().counters().schedule_requests(), 2);
    assert_eq!(snapshot.observability().counters().wakeups_armed(), 1);
    assert_eq!(snapshot.observability().counters().coalesced(), 1);
}

#[test]
fn illegal_once_recurrence_and_invariant_result_stop_truthfully() {
    let mut registry = registry();
    let illegal_id = identity("illegal");
    let illegal = registry
        .register_once(illegal_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (token, _, _) = arm(registry
        .ensure_once(&illegal, 0, TimerSchedule::At(1))
        .expect("ensure should succeed"));
    assert_eq!(
        registry.begin_ordinary(&token),
        CallbackAcceptance::Accepted
    );
    let transition = registry
        .complete_ordinary(
            &token,
            2,
            TimerRunResult::new(
                TimerCompletion::success(1),
                TimerDirective::RecurAfterCompletion,
            ),
        )
        .expect("illegal directive becomes a terminal transition");
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::DirectiveNotAllowed)
    );
    let snapshot = registry
        .snapshot(&illegal_id)
        .expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::DirectiveNotAllowed,),
        }
    );
    assert_eq!(snapshot.observability().counters().invariant_failure(), 1);

    let invariant_id = identity("invariant");
    let invariant = registry
        .register_once(invariant_id.clone(), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (token, _, _) = arm(registry
        .ensure_once(&invariant, 0, TimerSchedule::At(1))
        .expect("ensure should succeed"));
    assert_eq!(
        registry.begin_ordinary(&token),
        CallbackAcceptance::Accepted
    );
    let transition = registry
        .complete_ordinary(
            &token,
            2,
            TimerRunResult::new(
                TimerCompletion::invariant_failure(3),
                TimerDirective::ContinueImmediately,
            ),
        )
        .expect("invariant completion should stop normally");
    assert_eq!(transition.failure(), None);
    assert_eq!(
        registry.snapshot(&invariant_id).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        })
    );
}

#[test]
fn watchdog_dispatches_successor_first_and_retires_unacknowledged_attempt() {
    let mut registry = registry();
    let timer = identity("watchdog");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial_transition = registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed");
    confirm(&mut registry, &initial_transition);
    let (scheduler, deadline, _) = arm(initial_transition);
    assert_eq!(deadline, 15);

    let first_dispatch = registry.begin_watchdog_scheduler(&scheduler, 20);
    confirm(&mut registry, &first_dispatch);
    let (successor, deadline, work) = dispatch(first_dispatch);
    assert_eq!(deadline, 25);
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
            successor_generation: successor.callback_generation(),
            successor_deadline_ns: 25,
            attempt: WatchdogAttemptSnapshot::new(
                work.callback_generation(),
                WatchdogAttemptStatus::Dispatched,
            ),
        },)
    );
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    assert_eq!(
        registry
            .snapshot(&timer)
            .and_then(|snapshot| match snapshot.state() {
                TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork { attempt, .. },
                ) => Some(attempt.status()),
                _ => None,
            }),
        Some(WatchdogAttemptStatus::Running)
    );
    registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue),
        )
        .expect("watchdog completion should succeed");

    let second_dispatch = registry.begin_watchdog_scheduler(&successor, 30);
    confirm(&mut registry, &second_dispatch);
    let (next_successor, deadline, delayed_work) = dispatch(second_dispatch);
    assert_eq!(deadline, 35);
    let third_dispatch = registry.begin_watchdog_scheduler(&next_successor, 40);
    confirm(&mut registry, &third_dispatch);
    let (_third_successor, deadline, _third_work) = dispatch(third_dispatch);
    assert_eq!(deadline, 45);
    assert_eq!(
        registry.begin_watchdog_work(&delayed_work),
        CallbackAcceptance::Stale
    );

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    let counters = snapshot.observability().counters();
    assert_eq!(counters.scheduler_started(), 3);
    assert_eq!(counters.work_dispatched(), 3);
    assert_eq!(counters.unacknowledged(), 1);
    assert_eq!(counters.stale_work(), 1);
    assert_eq!(
        snapshot.observability().outcomes().last_outcome(),
        Some(TimerLastOutcome::Unacknowledged)
    );
}

#[test]
fn watchdog_immediate_initial_and_replacement_requests_coalesce_without_duplicates() {
    let mut registry = registry();
    let immediate_timer = identity("watchdog-immediate-initial");
    let immediate_claim = registry
        .register_watchdog(
            immediate_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("claim should succeed");
    let initial = registry
        .ensure_watchdog_immediately(&immediate_claim, 10)
        .expect("immediate initial ensure should succeed");
    assert!(matches!(
        initial.effect(),
        RegistryEffect::ArmWakeup {
            deadline_ns: 10,
            delay_ns: 0,
            arm: WakeupArm::Initial,
            ..
        }
    ));
    confirm(&mut registry, &initial);
    let duplicate = registry
        .ensure_watchdog_immediately(&immediate_claim, 10)
        .expect("equivalent immediate ensure should coalesce");
    assert_eq!(duplicate.effect(), &RegistryEffect::None);
    let snapshot = registry
        .snapshot(&immediate_timer)
        .expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(10));
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    assert_eq!(snapshot.observability().counters().schedule_requests(), 2);
    assert_eq!(snapshot.observability().counters().wakeups_armed(), 1);
    assert_eq!(snapshot.observability().counters().coalesced(), 1);

    let replacement_timer = identity("watchdog-immediate-replacement");
    let replacement_claim = registry
        .register_watchdog(
            replacement_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("replacement claim should succeed");
    let cadence_arm = registry
        .ensure_recurring(&replacement_claim, 10)
        .expect("cadence ensure should succeed");
    confirm(&mut registry, &cadence_arm);
    let (stale_scheduler, _, _) = arm(cadence_arm);
    let replacement = registry
        .ensure_watchdog_immediately(&replacement_claim, 12)
        .expect("later cadence deadline should move to now");
    assert!(matches!(
        replacement.effect(),
        RegistryEffect::ArmWakeup {
            deadline_ns: 12,
            delay_ns: 0,
            arm: WakeupArm::Replacement,
            ..
        }
    ));
    confirm(&mut registry, &replacement);
    let (immediate_scheduler, _, _) = arm(replacement);
    assert_eq!(
        registry
            .begin_watchdog_scheduler(&stale_scheduler, 15)
            .effect(),
        &RegistryEffect::None
    );
    let duplicate = registry
        .ensure_watchdog_immediately(&replacement_claim, 12)
        .expect("repeated replacement should coalesce");
    assert_eq!(duplicate.effect(), &RegistryEffect::None);
    let overdue_duplicate = registry
        .ensure_watchdog_immediately(&replacement_claim, 13)
        .expect("an already earlier deadline should satisfy immediate demand");
    assert_eq!(overdue_duplicate.effect(), &RegistryEffect::None);
    let snapshot = registry
        .snapshot(&replacement_timer)
        .expect("replacement snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(12));
    assert_eq!(
        snapshot.generation(),
        Some(immediate_scheduler.callback_generation())
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests(), 4);
    assert_eq!(counters.wakeups_armed(), 2);
    assert_eq!(counters.coalesced(), 2);
    assert_eq!(counters.stale_wakeups(), 1);
}

#[test]
fn watchdog_running_immediate_request_replaces_exact_successor_and_beats_cadence() {
    let mut registry = registry();
    let timer = identity("watchdog-running-immediate");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 10)
        .expect("initial ensure should succeed");
    confirm(&mut registry, &initial);
    let (scheduler, _, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 20);
    confirm(&mut registry, &dispatched);
    let (cadence_successor, cadence_deadline, work) = dispatch(dispatched);
    assert_eq!(cadence_deadline, 25);

    let dispatched_request = registry
        .ensure_watchdog_immediately(&claim, 20)
        .expect("dispatched work should satisfy immediate demand");
    assert_eq!(dispatched_request.effect(), &RegistryEffect::None);
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    assert_eq!(
        registry
            .ensure_watchdog_immediately(&claim, 21)
            .expect("running immediate request should pend")
            .effect(),
        &RegistryEffect::None
    );
    assert_eq!(
        registry
            .ensure_recurring(&claim, 21)
            .expect("cadence ensure must not delay pending immediate work")
            .effect(),
        &RegistryEffect::None
    );
    let completed = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Stop),
        )
        .expect("pending immediate request should override callback stop");
    assert!(matches!(
        completed.effect(),
        RegistryEffect::ArmWakeup {
            deadline_ns: 21,
            delay_ns: 0,
            arm: WakeupArm::Replacement,
            ..
        }
    ));
    confirm(&mut registry, &completed);
    let (immediate_successor, _, _) = arm(completed);
    assert_eq!(
        registry
            .begin_watchdog_scheduler(&cadence_successor, 25)
            .effect(),
        &RegistryEffect::None,
        "the replaced cadence generation must remain stale"
    );
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(21));
    assert_eq!(
        snapshot.generation(),
        Some(immediate_successor.callback_generation())
    );
    assert_eq!(
        snapshot.scheduling_mode(),
        TimerSchedulingMode::Continuation
    );
    assert_eq!(snapshot.latest_requested_delay_ns(), Some(0));
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
    let counters = snapshot.observability().counters();
    assert_eq!(counters.schedule_requests(), 4);
    assert_eq!(counters.wakeups_armed(), 3);
    assert_eq!(counters.work_dispatched(), 1);
    assert_eq!(counters.coalesced(), 3);
    assert_eq!(counters.stale_wakeups(), 1);
}

#[test]
#[allow(clippy::too_many_lines)] // One precedence matrix over three independent declarations.
fn watchdog_completion_arbitrates_immediate_cancellation_and_unregistration() {
    let mut registry = registry();
    let continue_timer = identity("watchdog-decision-immediate");
    let continue_claim = registry
        .register_watchdog(continue_timer, cadence(5), DeclarationLifetime::Retained)
        .expect("continue claim should succeed");
    let (scheduler, _, _) = arm(registry
        .ensure_recurring(&continue_claim, 10)
        .expect("continue ensure should succeed"));
    let (cadence_successor, _, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    let immediate = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("immediate decision should succeed");
    assert!(matches!(
        immediate.effect(),
        RegistryEffect::ArmWakeup {
            deadline_ns: 21,
            delay_ns: 0,
            arm: WakeupArm::Replacement,
            ..
        }
    ));
    assert_ne!(
        arm(immediate).0.callback_generation(),
        cadence_successor.callback_generation()
    );

    let cancel_timer = identity("watchdog-immediate-then-cancel");
    let cancel_claim = registry
        .register_watchdog(
            cancel_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("cancel claim should succeed");
    let (scheduler, _, _) = arm(registry
        .ensure_recurring(&cancel_claim, 10)
        .expect("cancel ensure should succeed"));
    let (_successor, _, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    registry
        .ensure_watchdog_immediately(&cancel_claim, 21)
        .expect("immediate request should pend");
    registry
        .cancel(&cancel_claim)
        .expect("later cancellation should pend");
    let cancelled = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("cancellation should override continuation");
    assert!(matches!(
        cancelled.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert_eq!(
        registry
            .snapshot(&cancel_timer)
            .map(|snapshot| snapshot.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::Cancelled,
        })
    );

    let unregister_timer = identity("watchdog-immediate-then-unregister");
    let unregister_claim = registry
        .register_watchdog(
            unregister_timer.clone(),
            cadence(5),
            DeclarationLifetime::Retained,
        )
        .expect("unregister claim should succeed");
    let (scheduler, _, _) = arm(registry
        .ensure_recurring(&unregister_claim, 10)
        .expect("unregister ensure should succeed"));
    let (_successor, _, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 20));
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    registry
        .ensure_watchdog_immediately(&unregister_claim, 21)
        .expect("immediate request should pend");
    registry
        .unregister(&unregister_claim)
        .expect("unregistration should pend");
    registry
        .ensure_watchdog_immediately(&unregister_claim, 21)
        .expect("unregistration should remain sticky");
    let unregistered = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(
                TimerCompletion::success(1),
                WatchdogDecision::ContinueImmediately,
            ),
        )
        .expect("unregistration should override every continuation");
    assert!(matches!(
        unregistered.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert!(registry.snapshot(&unregister_timer).is_none());
}

#[test]
fn watchdog_terminal_cancellation_makes_queued_callbacks_stale() {
    let mut registry = registry();
    let timer = identity("watchdog-cancel");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    assert_eq!(
        registry.reconcile_ordinary(&claim, 0, None),
        Err(RegistryError::PolicyMismatch { actual: "watchdog" })
    );
    let (scheduler, _, _) = arm(registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed"));
    let (successor, _, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
    let cancelled = registry.cancel(&claim).expect("cancel should succeed");
    assert_eq!(
        cancelled.effect(),
        &RegistryEffect::ClearCallbacks {
            identity: timer.clone(),
            handles: CallbacksToClear::WakeupAndWork,
        }
    );
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Stale
    );
    assert_eq!(
        registry.begin_watchdog_scheduler(&successor, 20).effect(),
        &RegistryEffect::None
    );
    assert_eq!(
        registry
            .cancel(&claim)
            .expect("repeated cancellation should be idempotent")
            .effect(),
        &RegistryEffect::None
    );

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.registration_status(),
        TimerRegistrationStatus::Unregistered
    );
    assert_eq!(snapshot.observability().counters().cancelled(), 1);
    assert_eq!(snapshot.observability().counters().stale_work(), 1);
    assert_eq!(snapshot.observability().counters().stale_wakeups(), 1);
}

#[test]
fn watchdog_result_matrix_tracks_retry_stop_and_invariant_failure() {
    let mut registry = registry();
    let timer = identity("watchdog-results");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let initial = registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed");
    confirm(&mut registry, &initial);
    let (scheduler, _, _) = arm(initial);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 10);
    confirm(&mut registry, &dispatched);
    let (successor, _, work) = dispatch(dispatched);
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_watchdog_work(
            &work,
            11,
            WatchdogRunResult::new(
                TimerCompletion::retryable_failure(1),
                WatchdogDecision::Continue,
            ),
        )
        .expect("retryable continuation should succeed");
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(1));
    assert_eq!(
        registry
            .snapshot(&timer)
            .and_then(|snapshot| snapshot.next_deadline_ns()),
        Some(15),
        "ordinary Continue must retain the cadence successor unchanged"
    );

    let dispatched = registry.begin_watchdog_scheduler(&successor, 20);
    confirm(&mut registry, &dispatched);
    let (_next, _, work) = dispatch(dispatched);
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    let stopped = registry
        .complete_watchdog_work(
            &work,
            21,
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .expect("terminal stop should succeed");
    assert!(matches!(
        stopped.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Wakeup,
            ..
        }
    ));
    assert_eq!(registry.consecutive_expected_failures(&timer), Some(0));

    let restarted = registry
        .ensure_recurring(&claim, 30)
        .expect("retained watchdog should restart");
    confirm(&mut registry, &restarted);
    let (scheduler, _, _) = arm(restarted);
    let dispatched = registry.begin_watchdog_scheduler(&scheduler, 40);
    confirm(&mut registry, &dispatched);
    let (_successor, _, work) = dispatch(dispatched);
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_watchdog_work(
            &work,
            41,
            WatchdogRunResult::new(
                TimerCompletion::invariant_failure(2),
                WatchdogDecision::Continue,
            ),
        )
        .expect("invariant failure should stop");
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::InvariantFailure,
        }
    );
    assert_eq!(snapshot.observability().counters().retryable_failure(), 1);
    assert_eq!(snapshot.observability().counters().no_work(), 1);
    assert_eq!(snapshot.observability().counters().invariant_failure(), 1);
}

#[test]
fn watchdog_nested_cancel_then_ensure_retains_committed_successor() {
    let mut registry = registry();
    let timer = identity("watchdog-nested");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    let (scheduler, _, _) = arm(registry
        .ensure_recurring(&claim, 0)
        .expect("ensure should succeed"));
    let (_successor, _, work) = dispatch(registry.begin_watchdog_scheduler(&scheduler, 10));
    assert_eq!(
        registry.begin_watchdog_work(&work),
        CallbackAcceptance::Accepted
    );
    registry
        .cancel(&claim)
        .expect("nested cancel should succeed");
    registry
        .ensure_recurring(&claim, 11)
        .expect("later ensure should succeed");
    registry
        .complete_watchdog_work(
            &work,
            12,
            WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        )
        .expect("later ensure should override stop");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert!(matches!(
        snapshot.state(),
        TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { .. })
    ));
    assert_eq!(snapshot.observability().counters().cancelled(), 0);
}

#[test]
fn watchdog_checked_generation_exhaustion_is_terminal_and_atomic() {
    let mut registry = registry();
    let timer = identity("watchdog-overflow");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(1), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    {
        let entry = registry
            .entries
            .get_mut(&timer)
            .expect("fixture entry should exist");
        let EntryControl::Watchdog(control) = &mut entry.control else {
            panic!("fixture should be watchdog control");
        };
        control.scheduler_generation = u64::MAX;
    }
    let transition = registry
        .ensure_recurring(&claim, 0)
        .expect("overflow is a terminal transition");
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::GenerationExhausted)
    );
    assert_eq!(
        registry.snapshot(&timer).map(|value| value.state()),
        Some(TimerRuntimeStateSnapshot::Inactive {
            reason: InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted,),
        })
    );

    let attempt_timer = identity("watchdog-attempt-overflow");
    let attempt_claim = registry
        .register_watchdog(
            attempt_timer.clone(),
            cadence(1),
            DeclarationLifetime::Retained,
        )
        .expect("attempt-overflow claim should succeed");
    let initial = registry
        .ensure_recurring(&attempt_claim, 0)
        .expect("initial attempt-overflow ensure should succeed");
    let (scheduler, _, _) = arm(initial);
    {
        let entry = registry
            .entries
            .get_mut(&attempt_timer)
            .expect("attempt-overflow entry should exist");
        let EntryControl::Watchdog(control) = &mut entry.control else {
            panic!("fixture should be watchdog control");
        };
        control.attempt_generation = u64::MAX;
    }

    let transition = registry.begin_watchdog_scheduler(&scheduler, 1);
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::GenerationExhausted)
    );
    assert!(matches!(
        transition.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Work,
            ..
        }
    ));
    let entry = registry
        .entries
        .get(&attempt_timer)
        .expect("retained attempt-overflow entry should remain");
    let EntryControl::Watchdog(control) = &entry.control else {
        panic!("fixture should remain watchdog control");
    };
    assert_eq!(
        control.scheduler_generation, 1,
        "paired generation allocation must be atomic"
    );
    assert_eq!(control.state, WatchdogState::Inactive);
    assert_eq!(control.pending, None);
}

#[test]
fn watchdog_checked_deadline_overflow_is_terminal() {
    let mut registry = registry();
    let deadline_timer = identity("watchdog-deadline-overflow");
    let deadline_claim = registry
        .register_watchdog(
            deadline_timer.clone(),
            cadence(1),
            DeclarationLifetime::Retained,
        )
        .expect("deadline-overflow claim should succeed");
    let initial = registry
        .ensure_recurring(&deadline_claim, 0)
        .expect("initial deadline-overflow ensure should succeed");
    let (scheduler, _, _) = arm(initial);
    let transition = registry.begin_watchdog_scheduler(&scheduler, u64::MAX);
    assert_eq!(
        transition.failure(),
        Some(TimerControlFailure::DeadlineOverflow)
    );
    assert!(matches!(
        transition.effect(),
        RegistryEffect::ClearCallbacks {
            handles: CallbacksToClear::Work,
            ..
        }
    ));
    let entry = registry
        .entries
        .get(&deadline_timer)
        .expect("retained deadline-overflow entry should remain");
    let EntryControl::Watchdog(control) = &entry.control else {
        panic!("fixture should remain watchdog control");
    };
    assert_eq!(control.state, WatchdogState::Inactive);
    assert_eq!(control.pending, None);
}

#[test]
fn initial_snapshots_are_policy_specific_and_coherent() {
    let mut registry = registry();
    let once_id = identity("a-once");
    let after_id = identity("b-after");
    let watchdog_id = identity("c-watchdog");
    registry
        .register_once(once_id.clone(), DeclarationLifetime::Retained)
        .expect("once registration should succeed");
    registry
        .register_after_completion(after_id.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("after-completion registration should succeed");
    registry
        .register_watchdog(
            watchdog_id.clone(),
            cadence(7),
            DeclarationLifetime::Retained,
        )
        .expect("watchdog registration should succeed");

    let inventory = registry.inventory();
    assert_eq!(inventory.epoch(), TimerEpoch::new(7, 10));
    assert_eq!(inventory.len(), 3);
    assert_eq!(inventory.timers()[0].identity(), &once_id);
    assert_eq!(inventory.timers()[1].identity(), &after_id);
    assert_eq!(inventory.timers()[2].identity(), &watchdog_id);
    for snapshot in inventory.timers() {
        assert_eq!(
            snapshot.state(),
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::NeverScheduled,
            }
        );
        assert_eq!(snapshot.next_deadline_ns(), None);
        assert_eq!(snapshot.generation(), None);
        assert_eq!(snapshot.observability().epoch(), TimerEpoch::new(7, 10));
    }
}
