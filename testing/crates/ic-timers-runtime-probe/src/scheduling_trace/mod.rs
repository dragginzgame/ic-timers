//! Observable deadline order, callback-created timers and self-cancellation.

use ic_timers::{
    AfterCompletionDecision, AfterCompletionRegistration, AfterCompletionRunResult, OnceDecision,
    OnceRegistration, OnceRunResult, TimerCadence, TimerCompletion, TimerIdentity, TimerSchedule,
    TimerStart, register_after_completion, register_once,
};
use std::{cell::RefCell, time::Duration};

thread_local! {
    static EVENTS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static ONCE: RefCell<Vec<OnceRegistration>> = const { RefCell::new(Vec::new()) };
    static PERIODIC: RefCell<Option<AfterCompletionRegistration>> = const { RefCell::new(None) };
}

fn identity(name: &str) -> TimerIdentity {
    TimerIdentity::try_new("ic-timers", "scheduling-trace", name).expect("fixed trace identity")
}

fn install_once(name: &str, seconds: u64, event: u8) {
    let registration = register_once(
        identity(name),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(seconds)),
        },
        move |_| async move {
            EVENTS.with_borrow_mut(|events| events.push(event));
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("trace registration");
    ONCE.with_borrow_mut(|slots| slots.push(registration));
}

#[ic_platform::cdk::update]
fn start_scheduling_trace() {
    assert!(ONCE.with_borrow(Vec::is_empty), "fresh trace");
    let parent = register_once(
        identity("parent"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(1)),
        },
        |_| async {
            EVENTS.with_borrow_mut(|events| events.push(1));
            install_once("child", 2, 3);
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("parent registration");
    ONCE.with_borrow_mut(|slots| slots.push(parent));
    install_once("middle", 2, 2);
    install_once("late", 4, 4);
    let periodic = register_after_completion(
        identity("self-cancelling-periodic"),
        TimerCadence::new(Duration::from_secs(1)).expect("trace cadence"),
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::from_secs(7))),
        },
        |context| async move {
            EVENTS.with_borrow_mut(|events| events.push(5));
            context.cancel().expect("periodic self-cancellation");
            AfterCompletionRunResult::new(
                TimerCompletion::success(1),
                AfterCompletionDecision::RecurAfterCompletion,
            )
        },
    )
    .expect("periodic trace registration");
    PERIODIC.with_borrow_mut(|slot| *slot = Some(periodic));
    let once = register_once(
        identity("self-cancelling-once"),
        TimerStart::RemoveWhenStopped {
            initial: TimerSchedule::After(Duration::from_secs(8)),
        },
        |context| async move {
            EVENTS.with_borrow_mut(|events| events.push(6));
            context.cancel().expect("one-shot self-cancellation");
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
        },
    )
    .expect("self-cancelling one-shot registration");
    ONCE.with_borrow_mut(|slots| slots.push(once));
}

#[ic_platform::cdk::query]
fn scheduling_trace() -> Vec<u8> {
    EVENTS.with_borrow(Clone::clone)
}
