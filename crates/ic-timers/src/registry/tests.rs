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

fn arm(transition: RegistryTransition) -> (CallbackToken, u64, bool) {
    match transition.into_effect() {
        RegistryEffect::ArmWakeup {
            token,
            deadline_ns,
            replace,
            ..
        } => (token, deadline_ns, replace),
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
        .snapshots()
        .into_iter()
        .map(|snapshot| snapshot.identity().name().as_str().to_owned())
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
        .unregister(scheduled)
        .expect("scheduled unregistration should succeed");
    assert_eq!(
        transition.effect(),
        &RegistryEffect::ClearCallbacks {
            identity: scheduled_id.clone(),
            clear_wakeup: true,
            clear_work: false,
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
            .unregister(running)
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
            .unregister(claim)
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
            clear_wakeup: true,
            clear_work: false,
        }
    );
    assert!(registry.snapshot(&timer).is_none());
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
    let (first, deadline, replace) = arm(first_transition);
    assert_eq!(deadline, 100);
    assert!(!replace);
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

    let (replaced, deadline, replace) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(100)))
        .expect("initial reconciliation should arm"));
    assert_eq!(deadline, 100);
    assert!(!replace);
    let (current, deadline, replace) = arm(registry
        .reconcile_ordinary(&claim, 0, Some(TimerSchedule::At(200)))
        .expect("authoritative reconciliation may move later"));
    assert_eq!(deadline, 200);
    assert!(replace);
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
fn watchdog_terminal_cancellation_makes_queued_callbacks_stale() {
    let mut registry = registry();
    let timer = identity("watchdog-cancel");
    let claim = registry
        .register_watchdog(timer.clone(), cadence(5), DeclarationLifetime::Retained)
        .expect("claim should succeed");
    assert_eq!(
        registry.reconcile_ordinary(&claim, 0, None),
        Err(RegistryError::WrongPolicy { actual: "watchdog" })
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
            clear_wakeup: true,
            clear_work: true,
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
            clear_wakeup: true,
            clear_work: false,
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
fn watchdog_checked_generation_and_request_exhaustion_are_terminal() {
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

    let snapshots = registry.snapshots();
    assert_eq!(snapshots.len(), 3);
    assert_eq!(snapshots[0].identity(), &once_id);
    assert_eq!(snapshots[1].identity(), &after_id);
    assert_eq!(snapshots[2].identity(), &watchdog_id);
    for snapshot in snapshots {
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
