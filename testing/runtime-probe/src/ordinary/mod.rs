//! Real await/ingress interleaving evidence; no timer recovery claim.

use candid::CandidType;
use ic_timers::{
    AfterCompletionRegistration, DeclarationLifetime, OnceRegistration, TimerCadence,
    TimerCompletion, TimerDirective, TimerIdentity, TimerRegistrationStatus, TimerRunResult,
    TimerSchedule, register_after_completion, register_once, timer_snapshot,
};
use std::{
    cell::{Cell, RefCell},
    task::{Poll, Waker},
};

enum Registration {
    Once(OnceRegistration),
    AfterCompletion(AfterCompletionRegistration),
}

thread_local! {
    static REGISTRATION: RefCell<Option<Registration>> = const { RefCell::new(None) };
    static GATE_OPEN: Cell<bool> = const { Cell::new(false) };
    static GATE_WAKER: RefCell<Option<Waker>> = const { RefCell::new(None) };
    static COMPLETIONS: Cell<u64> = const { Cell::new(0) };
    static COMPLETED_AT_NS: Cell<Option<u64>> = const { Cell::new(None) };
}

#[derive(CandidType)]
struct Observation {
    declared: bool,
    running: bool,
    waiting: bool,
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
                    |_| work(true),
                )
                .expect("register after-completion"),
            )
        } else {
            Registration::Once(
                register_once(identity(), lifetime, |_| work(false)).expect("register Once"),
            )
        };
        reconcile(&registration, Some(TimerSchedule::At(ic_cdk::api::time())));
        *slot = Some(registration);
    });
}

async fn work(recur: bool) -> TimerRunResult {
    if ic_cdk::call::Call::unbounded_wait(ic_cdk::api::canister_self(), "ordinary_gate")
        .await
        .is_err()
    {
        return TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop);
    }
    COMPLETIONS.with(|count| count.set(count.get() + 1));
    COMPLETED_AT_NS.with(|time| time.set(Some(ic_cdk::api::time())));
    TimerRunResult::new(
        TimerCompletion::success(1),
        if recur {
            TimerDirective::RecurAfterCompletion
        } else {
            TimerDirective::Stop
        },
    )
}

#[ic_cdk::update]
async fn ordinary_gate() {
    assert_eq!(
        ic_cdk::api::msg_caller(),
        ic_cdk::api::canister_self(),
        "self-call gate"
    );
    std::future::poll_fn(|context| {
        if GATE_OPEN.with(Cell::get) {
            Poll::Ready(())
        } else {
            GATE_WAKER.with_borrow_mut(|slot| *slot = Some(context.waker().clone()));
            Poll::Pending
        }
    })
    .await;
}

#[ic_cdk::update]
fn release_ordinary_work() {
    GATE_OPEN.with(|open| open.set(true));
    if let Some(waker) = GATE_WAKER.with_borrow_mut(Option::take) {
        waker.wake();
    }
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
        waiting: GATE_WAKER.with_borrow(Option::is_some),
        completed: COMPLETIONS.with(Cell::get),
        completed_at_ns: COMPLETED_AT_NS.with(Cell::get),
        next_deadline_ns: snapshot.as_ref().and_then(|value| value.next_deadline_ns()),
        work_completed: snapshot
            .as_ref()
            .map_or(0, |value| value.observability().counters().work_completed()),
    }
}
