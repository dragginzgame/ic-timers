//! Bounded command interleavings against the real runtime and native provider.
//! This checks authority/handle ownership, not IC message rollback or recovery.

use super::*;

fn declaration(
    policy: u8,
    timer: &TimerIdentity,
    lifetime: DeclarationLifetime,
) -> RegistrationClaim {
    let start = scheduled_start(lifetime, TimerSchedule::After(Duration::from_nanos(5)));
    let cadence = TimerCadence::from_nanos(5).unwrap();
    match policy {
        0 => {
            register_once(timer.clone(), start, |_| async {
                OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop)
            })
            .unwrap()
            .claim
        }
        1 => {
            register_after_completion(timer.clone(), cadence, start, |_| async {
                AfterCompletionRunResult::new(
                    TimerCompletion::no_work(),
                    AfterCompletionDecision::RecurAfterCompletion,
                )
            })
            .unwrap()
            .claim
        }
        2 => {
            register_watchdog(timer.clone(), cadence, start, |_| {
                WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Continue)
            })
            .unwrap()
            .claim
        }
        _ => unreachable!(),
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "Keep the bounded trace driver and its invariant checks together."
)]
fn generated_command_sequences_preserve_authority_and_handle_ownership() {
    // Exhaust every four-command word over six operations, for all three
    // policies and both lifetimes: 7,776 traces. Include each command prefix.
    for policy in 0..3 {
        for lifetime in [
            DeclarationLifetime::Retained,
            DeclarationLifetime::RemoveWhenStopped,
        ] {
            for word in 0..6_u32.pow(4) {
                let _fixture = setup();
                let timer = identity("sequence");
                let sentinel = register_once(
                    identity("untouched"),
                    TimerStart::Retained {
                        initial: Some(TimerSchedule::At(u64::MAX)),
                    },
                    |_| async { panic!("unrelated timer must not run") },
                )
                .unwrap();
                let sentinel_before = timer_snapshot(sentinel.identity()).unwrap();
                let mut claim = declaration(policy, &timer, lifetime);
                let mut retired = Vec::new();
                let mut commands = word;
                for step in 0..4 {
                    let command = commands % 6;
                    commands /= 6;
                    let trace = (policy, lifetime, word, step, command);
                    let exists = timer_snapshot(&timer).unwrap().is_some();
                    // Capture a delivery only when this command invalidates it.
                    let stale = if exists && matches!(command, 2 | 3) {
                        let detached =
                            with_registry_mut(|registry| Ok(registry.take_wakeup_handle(&timer)))
                                .unwrap();
                        detached.map(|handle| {
                            let (token, handle) = handle.into_parts();
                            bind_provider_handle(&token, handle, None).unwrap();
                            token
                        })
                    } else {
                        None
                    };
                    let result = match command {
                        0 if policy == 0 => ensure_once_claim(
                            &claim,
                            None,
                            TimerSchedule::After(Duration::from_nanos(5)),
                        ),
                        0 => ensure_recurring_claim(&claim, None),
                        1 => {
                            let schedule = Some(TimerSchedule::After(Duration::from_nanos(7)));
                            if policy == 2 {
                                reconcile_watchdog_claim(&claim, None, schedule)
                            } else {
                                reconcile_ordinary_claim(&claim, None, schedule)
                            }
                        }
                        2 => cancel_claim(&claim, None),
                        3 => {
                            if exists {
                                unregister_claim(&claim).unwrap();
                            }
                            retired.push(claim);
                            claim = declaration(policy, &timer, lifetime);
                            Ok(())
                        }
                        4 => {
                            set_time(platform::time_ns() + 10);
                            // Leave watchdog work pending in some prefixes;
                            // later commands must honor its lifecycle ownership.
                            let _ran = run_next_due();
                            Ok(())
                        }
                        5 => {
                            let _discarded = discard_next_due();
                            Ok(())
                        }
                        _ => unreachable!(),
                    };
                    assert!(
                        result.is_ok() || matches!(result, Err(TimerError::RegistrationExpired)),
                        "{trace:?}: {result:?}"
                    );
                    for old in &retired {
                        assert!(
                            matches!(
                                cancel_claim(old, None),
                                Err(TimerError::RegistrationExpired)
                            ),
                            "{trace:?}"
                        );
                    }
                    if let Some(token) = stale {
                        // Scheduler cancellation while work is pending can be
                        // deferred. Deliver only after its claim really retires,
                        // or ordinary cancellation has invalidated the generation.
                        if policy != 2
                            || command == 3
                            || timer_snapshot(&timer)
                                .unwrap()
                                .is_none_or(|s| s.next_deadline_ns().is_none())
                        {
                            let before = timer_snapshot(&timer).unwrap();
                            let handles = timer_count();
                            if policy == 2 {
                                dispatch_watchdog_scheduler(&token);
                            } else {
                                let _delivery = platform::set_timer(
                                    Duration::ZERO,
                                    dispatch_wakeup(WakeupToken::Ordinary(token)),
                                );
                                assert!(run_next_due());
                            }
                            let after = timer_snapshot(&timer).unwrap();
                            assert_eq!(
                                after.as_ref().map(|s| (
                                    s.registration_id(),
                                    s.state(),
                                    s.next_deadline_ns(),
                                    s.observability().performance()
                                )),
                                before.as_ref().map(|s| (
                                    s.registration_id(),
                                    s.state(),
                                    s.next_deadline_ns(),
                                    s.observability().performance()
                                )),
                                "stale authority: {trace:?}"
                            );
                            assert_eq!(timer_count(), handles, "stale handles: {trace:?}");
                        }
                    }
                    assert_eq!(
                        timer_snapshot(sentinel.identity()).unwrap(),
                        sentinel_before,
                        "{trace:?}"
                    );
                    assert!(timer_inventory().unwrap().len() <= 2, "{trace:?}");
                    assert!(
                        (1..=3).contains(&timer_count()),
                        "orphan handles: {trace:?}"
                    );
                }
                if timer_snapshot(&timer).unwrap().is_some() {
                    unregister_claim(&claim).unwrap();
                }
                // A pending watchdog invocation can defer removal. Drain it at
                // the same time; no future cadence should survive unregister.
                for _ in 0..3 {
                    if !run_next_due() {
                        break;
                    }
                }
                assert_eq!(
                    timer_count(),
                    1,
                    "cleanup: policy={policy} lifetime={lifetime:?} word={word}"
                );
                sentinel.unregister().unwrap();
                assert_eq!(timer_count(), 0);
            }
        }
    }
}
