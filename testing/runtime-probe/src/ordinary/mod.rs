//! Real await/ingress interleaving evidence; no timer recovery claim.

use candid::CandidType;
use ic_timers::{
    AfterCompletionDecision, AfterCompletionRegistration, AfterCompletionRunResult,
    DeclarationLifetime, OnceDecision, OnceRegistration, OnceRunResult, TimerCadence,
    TimerCompletion, TimerIdentity, TimerRegistrationStatus, TimerSchedule,
    register_after_completion, register_once, timer_snapshot,
};
use std::cell::{Cell, RefCell};

enum Registration {
    Once(OnceRegistration),
    AfterCompletion(AfterCompletionRegistration),
}

thread_local! {
    static REGISTRATION: RefCell<Option<Registration>> = const { RefCell::new(None) };
    static GATE_OPEN: Cell<bool> = const { Cell::new(false) };
    static GATE_WAITING: Cell<bool> = const { Cell::new(false) };
    static GATE_REPLIES: Cell<u64> = const { Cell::new(0) };
    static GATE_ERROR: RefCell<Option<String>> = const { RefCell::new(None) };
    static COMPLETIONS: Cell<u64> = const { Cell::new(0) };
    static COMPLETED_AT_NS: Cell<Option<u64>> = const { Cell::new(None) };
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
        let registration = if after_completion {
            Registration::AfterCompletion(
                register_after_completion(
                    identity(),
                    TimerCadence::from_nanos(super::CADENCE_NS).expect("fixed cadence"),
                    lifetime,
                    |_| async {
                        AfterCompletionRunResult::new(
                            work().await,
                            AfterCompletionDecision::RecurAfterCompletion,
                        )
                    },
                )
                .expect("register after-completion"),
            )
        } else {
            Registration::Once(
                register_once(identity(), lifetime, |_| async {
                    OnceRunResult::new(work().await, OnceDecision::Stop)
                })
                .expect("register Once"),
            )
        };
        reconcile(&registration, Some(TimerSchedule::At(ic_cdk::api::time())));
        *slot = Some(registration);
    });
}

async fn work() -> TimerCompletion {
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
    }
}
