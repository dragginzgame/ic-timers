//! Real await/ingress interleaving and ordinary abandonment qualification.

use candid::CandidType;
use ic_timers::{
    AfterCompletionContext, AfterCompletionDecision, AfterCompletionRegistration,
    AfterCompletionRunResult, DeclarationLifetime, InactiveReason, OnceContext, OnceDecision,
    OnceRegistration, OnceRunResult, TimerCadence, TimerCompletion, TimerError, TimerIdentity,
    TimerRegistrationStatus, TimerRuntimeStateSnapshot, TimerSchedule, WatchdogDecision,
    WatchdogRunResult, register_after_completion, register_once, register_watchdog,
    timer_inventory, timer_snapshot,
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

enum Registration {
    Once(OnceRegistration),
    AfterCompletion(AfterCompletionRegistration),
}

enum WorkContext {
    Once(OnceContext),
    AfterCompletion(AfterCompletionContext),
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TrapPoint {
    None,
    BeforeAwait,
    AfterAwait,
}

struct CapturedState;

impl Drop for CapturedState {
    fn drop(&mut self) {
        CAPTURE_REGISTRY_ACCESSIBLE.with(|accessible| {
            accessible.set(accessible.get() && timer_inventory().is_ok());
        });
        CAPTURES_DROPPED.with(|count| count.set(count.get().saturating_add(1)));
    }
}

thread_local! {
    static REGISTRATION: RefCell<Option<Registration>> = const { RefCell::new(None) };
    static GATE_OPEN: Cell<bool> = const { Cell::new(false) };
    static GATE_WAITING: Cell<bool> = const { Cell::new(false) };
    static GATE_REPLIES: Cell<u64> = const { Cell::new(0) };
    static GATE_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static COMPLETIONS: Cell<u64> = const { Cell::new(0) };
    static COMPLETED_AT_NS: Cell<Option<u64>> = const { Cell::new(None) };
    static TRAP_POINT: Cell<TrapPoint> = const { Cell::new(TrapPoint::None) };
    static WORK_CONTEXT: RefCell<Option<WorkContext>> = const { RefCell::new(None) };
    static CAPTURES_DROPPED: Cell<u64> = const { Cell::new(0) };
    static CAPTURE_REGISTRY_ACCESSIBLE: Cell<bool> = const { Cell::new(true) };
}

#[derive(CandidType)]
struct Observation {
    declared: bool,
    running: bool,
    waiting: bool,
    gate_replies: u64,
    gate_error: Option<String>,
    completed: u64,
    completed_at_ns: Option<u64>,
    next_deadline_ns: Option<u64>,
    work_completed: u64,
    abandoned: bool,
    armed: bool,
    unacknowledged: u64,
    instruction_samples: u64,
    captures_dropped: u64,
    capture_registry_accessible: bool,
    inventory_len: u64,
}

fn identity() -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "await-probe", "ordinary").expect("fixed identity")
}

#[ic_cdk::update]
fn start_ordinary(after_completion: bool, transient: bool) {
    let lifetime = if transient {
        DeclarationLifetime::RemoveWhenStopped
    } else {
        DeclarationLifetime::Retained
    };
    REGISTRATION.with_borrow_mut(|slot| {
        assert!(slot.is_none(), "one probe registration");
        let captured = Rc::new(CapturedState);
        let registration = if after_completion {
            Registration::AfterCompletion(
                register_after_completion(
                    identity(),
                    TimerCadence::from_nanos(super::CADENCE_NS).expect("fixed cadence"),
                    lifetime,
                    move |context| {
                        WORK_CONTEXT.with_borrow_mut(|slot| {
                            *slot = Some(WorkContext::AfterCompletion(context))
                        });
                        let captured = Rc::clone(&captured);
                        async move {
                            let completion = work().await;
                            std::hint::black_box(captured);
                            AfterCompletionRunResult::new(
                                completion,
                                AfterCompletionDecision::RecurAfterCompletion,
                            )
                        }
                    },
                )
                .expect("register after-completion"),
            )
        } else {
            Registration::Once(
                register_once(identity(), lifetime, move |context| {
                    WORK_CONTEXT.with_borrow_mut(|slot| *slot = Some(WorkContext::Once(context)));
                    let captured = Rc::clone(&captured);
                    async move {
                        let completion = work().await;
                        std::hint::black_box(captured);
                        OnceRunResult::new(completion, OnceDecision::Stop)
                    }
                })
                .expect("register Once"),
            )
        };
        reconcile(&registration, Some(TimerSchedule::At(ic_cdk::api::time())));
        *slot = Some(registration);
    });
}

async fn work() -> TimerCompletion {
    if TRAP_POINT.with(Cell::get) == TrapPoint::BeforeAwait {
        ic_cdk::trap("ordinary probe trap before await");
    }
    GATE_WAITING.with(|waiting| waiting.set(true));
    loop {
        // Every closed-gate reply is followed by another real call await. A bare
        // Pending future cannot keep the IC call context alive, and a protected
        // CDK task cannot resume in the separate ingress that opens the gate.
        let result =
            ic_cdk::call::Call::unbounded_wait(ic_cdk::api::canister_self(), "ordinary_gate")
                .await
                .map_err(|error| error.to_string())
                .and_then(|response| response.candid::<bool>().map_err(|error| error.to_string()));
        let open = match result {
            Ok(open) => open,
            Err(error) => {
                GATE_WAITING.with(|waiting| waiting.set(false));
                GATE_ERROR.with_borrow_mut(|slot| *slot = Some(error));
                return TimerCompletion::invariant_failure(0);
            }
        };
        GATE_REPLIES.with(|count| count.set(count.get().saturating_add(1)));
        if open {
            break;
        }
    }
    if TRAP_POINT.with(Cell::get) == TrapPoint::AfterAwait {
        ic_cdk::trap("ordinary probe trap after await");
    }
    GATE_WAITING.with(|waiting| waiting.set(false));
    COMPLETIONS.with(|count| count.set(count.get() + 1));
    COMPLETED_AT_NS.with(|time| time.set(Some(ic_cdk::api::time())));
    TimerCompletion::success(1)
}

#[ic_cdk::update]
fn ordinary_gate() -> bool {
    assert_eq!(
        ic_cdk::api::msg_caller(),
        ic_cdk::api::canister_self(),
        "self-call gate"
    );
    GATE_OPEN.with(Cell::get)
}

#[ic_cdk::update]
fn release_ordinary_work() {
    GATE_OPEN.with(|open| open.set(true));
}

#[ic_cdk::update]
fn trap_ordinary_work(before_await: bool) {
    TRAP_POINT.with(|point| {
        point.set(if before_await {
            TrapPoint::BeforeAwait
        } else {
            TrapPoint::AfterAwait
        })
    });
}

#[ic_cdk::update]
fn clear_ordinary_trap() {
    TRAP_POINT.with(|point| point.set(TrapPoint::None));
}

#[ic_cdk::update]
fn expired_ordinary_context_rejected() -> bool {
    WORK_CONTEXT.with_borrow(|slot| {
        let result = match slot.as_ref().expect("committed work context") {
            WorkContext::Once(context) => context.cancel(),
            WorkContext::AfterCompletion(context) => context.cancel(),
        };
        matches!(result, Err(TimerError::RegistrationExpired))
    })
}

#[ic_cdk::update]
fn discard_expired_ordinary_claim() {
    REGISTRATION.with_borrow_mut(|slot| {
        let result = match slot.as_ref().expect("expired registration") {
            Registration::Once(timer) => timer.has_armed_wakeup(),
            Registration::AfterCompletion(timer) => timer.has_armed_wakeup(),
        };
        assert!(matches!(result, Err(TimerError::RegistrationExpired)));
        *slot = None;
    });
}

fn reconcile(registration: &Registration, desired: Option<TimerSchedule>) {
    let result = match registration {
        Registration::Once(timer) => timer.reconcile_schedule(desired),
        Registration::AfterCompletion(timer) => timer.reconcile_schedule(desired),
    };
    result.expect("ordinary reconciliation");
}

#[ic_cdk::update]
fn reconcile_ordinary_at(deadline_ns: u64) {
    REGISTRATION.with_borrow(|slot| {
        reconcile(
            slot.as_ref().expect("registered"),
            Some(TimerSchedule::At(deadline_ns)),
        )
    });
}

#[ic_cdk::update]
fn cancel_ordinary() {
    REGISTRATION.with_borrow(|slot| reconcile(slot.as_ref().expect("registered"), None));
}

#[ic_cdk::update]
fn unregister_ordinary() {
    let registration = REGISTRATION
        .with_borrow_mut(Option::take)
        .expect("registered");
    match registration {
        Registration::Once(timer) => timer.unregister(),
        Registration::AfterCompletion(timer) => timer.unregister(),
    }
    .expect("ordinary unregister");
}

#[ic_cdk::query]
fn ordinary_observation() -> Observation {
    let snapshot = timer_snapshot(&identity()).expect("ordinary snapshot");
    let armed = REGISTRATION.with_borrow(|slot| {
        let result = match slot.as_ref() {
            Some(Registration::Once(timer)) => timer.has_armed_wakeup(),
            Some(Registration::AfterCompletion(timer)) => timer.has_armed_wakeup(),
            None => return false,
        };
        match result {
            Ok(armed) => armed,
            Err(TimerError::RegistrationExpired) => false,
            Err(error) => panic!("ordinary ownership observation failed: {error}"),
        }
    });
    Observation {
        declared: snapshot.is_some(),
        running: snapshot
            .as_ref()
            .is_some_and(|value| value.registration_status() == TimerRegistrationStatus::Running),
        waiting: GATE_WAITING.with(Cell::get),
        gate_replies: GATE_REPLIES.with(Cell::get),
        gate_error: GATE_ERROR.with_borrow(Clone::clone),
        completed: COMPLETIONS.with(Cell::get),
        completed_at_ns: COMPLETED_AT_NS.with(Cell::get),
        next_deadline_ns: snapshot.as_ref().and_then(|value| value.next_deadline_ns()),
        work_completed: snapshot
            .as_ref()
            .map_or(0, |value| value.observability().counters().work_completed()),
        abandoned: snapshot.as_ref().is_some_and(|value| {
            value.state()
                == TimerRuntimeStateSnapshot::Inactive {
                    reason: InactiveReason::Abandoned,
                }
        }),
        armed,
        unacknowledged: snapshot
            .as_ref()
            .map_or(0, |value| value.observability().counters().unacknowledged()),
        instruction_samples: snapshot.as_ref().map_or(0, |value| {
            value
                .observability()
                .performance()
                .work_instructions()
                .samples()
        }),
        captures_dropped: CAPTURES_DROPPED.with(Cell::get),
        capture_registry_accessible: CAPTURE_REGISTRY_ACCESSIBLE.with(Cell::get),
        inventory_len: timer_inventory()
            .expect("ordinary inventory")
            .timers()
            .len() as u64,
    }
}

/// Remove armed authority in one message before any consumer work can start.
#[ic_cdk::update]
fn remove_armed_callback(policy: u8) {
    let deadline = ic_cdk::api::time()
        .checked_add(super::CADENCE_NS)
        .expect("fixture deadline");
    let captured = CapturedState;
    match policy {
        0 => {
            let timer = register_once(identity(), DeclarationLifetime::Retained, move |_| {
                std::hint::black_box(&captured);
                async { OnceRunResult::new(TimerCompletion::no_work(), OnceDecision::Stop) }
            })
            .expect("register Once");
            timer
                .ensure_scheduled(TimerSchedule::At(deadline))
                .expect("arm Once");
            timer.unregister().expect("unregister Once");
        }
        1 => {
            let timer = register_after_completion(
                identity(),
                TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
                DeclarationLifetime::RemoveWhenStopped,
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
            .expect("register after-completion");
            timer.ensure_scheduled().expect("arm after-completion");
            timer.cancel().expect("cancel transient");
        }
        2 => {
            let timer = register_watchdog(
                identity(),
                TimerCadence::from_nanos(super::CADENCE_NS).expect("cadence"),
                DeclarationLifetime::Retained,
                move |_| {
                    std::hint::black_box(&captured);
                    WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop)
                },
            )
            .expect("register Watchdog");
            timer.ensure_scheduled().expect("arm Watchdog");
            timer.unregister().expect("unregister Watchdog");
        }
        _ => ic_cdk::trap("unknown removal policy"),
    }
}
