//! One-shot scheduling state; the FFI module alone registers IC callbacks.
#![forbid(unsafe_code)]

use super::queue::{AdmissionError, Queue};
use ic_cdk_executor::MethodHandle;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    time::Duration,
};

// Two roles per registration, plus room for the two arms in one transition.
// Accepted futures leave this store; outstanding replies have their own bound.
const PENDING_LIMIT: usize = 2 * crate::registry::MAX_TIMER_REGISTRATIONS + 2;
pub(super) const DISPATCH_LIMIT: usize = 128;
pub(super) const RETRY_NS: u64 = 1_000_000_000;
pub(super) const METHOD: &str = "<ic-timers internal> dispatch";
type Task = Pin<Box<dyn Future<Output = ()>>>;

/// Linear authority over a pending task, never an executor task handle.
#[derive(Debug, Eq, PartialEq)]
#[must_use = "bind or clear the platform timer handle"]
pub(crate) struct TimerHandle(u64);

pub(super) struct Flight {
    pub(super) task: u64,
    pub(super) method: MethodHandle,
}

thread_local! {
    static QUEUE: RefCell<Queue<Task, PENDING_LIMIT>> = const { RefCell::new(Queue::new()) };
    static FLIGHTS: RefCell<super::slots::Slots<Flight, DISPATCH_LIMIT>> = const { RefCell::new(super::slots::Slots::new()) };
    static BLOCKED_UNTIL: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn try_set_timer(
    delay: Duration,
    task: impl Future<Output = ()> + 'static,
) -> Result<TimerHandle, AdmissionError> {
    let deadline = super::queue::deadline(ic0::time(), delay)?;
    let result = QUEUE.with_borrow_mut(|queue| queue.insert(deadline, Box::pin(task) as Task));
    let id = result.map_err(|(error, task)| {
        drop(task);
        error
    })?;
    rearm();
    Ok(TimerHandle(id))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "Cancellation consumes linear authority."
)]
pub(crate) fn clear_timer(handle: TimerHandle) {
    let task = remove(handle.0);
    rearm();
    drop(task);
}

pub(super) fn remove(id: u64) -> Option<Task> {
    QUEUE.with_borrow_mut(|queue| queue.remove(id))
}

pub(super) fn rearm() {
    let next = QUEUE.with_borrow(Queue::next);
    let timestamp = next.map_or(0, |(deadline, _)| deadline.max(BLOCKED_UNTIL.get()).max(1));
    ic0::global_timer_set(timestamp);
}

pub(super) fn due(now: u64) -> Option<u64> {
    if now < BLOCKED_UNTIL.get() {
        return None;
    }
    QUEUE
        .with_borrow(Queue::next)
        .filter(|(deadline, _)| *deadline <= now)
        .map(|(_, id)| id)
}

pub(super) fn defer(now: u64) {
    BLOCKED_UNTIL.set(now.saturating_add(RETRY_NS));
}

pub(super) fn reserve(id: u64) -> Option<usize> {
    let flight = Flight {
        task: id,
        method: ic_cdk_executor::extend_current_method_context(),
    };
    // A rejected context is destroyed after releasing the table borrow.
    FLIGHTS
        .with_borrow_mut(|flights| flights.insert(flight))
        .ok()
}

pub(super) fn take_flight(slot: usize) -> Flight {
    FLIGHTS
        .with_borrow_mut(|flights| flights.take(slot))
        .unwrap_or_else(|| ic0::trap(b"ic-timers: invalid transport callback"))
}

pub(super) fn submitted(id: u64) {
    QUEUE.with_borrow_mut(|queue| queue.submit(id));
}

pub(super) fn retry(id: u64) {
    let deadline = ic0::time().saturating_add(RETRY_NS);
    QUEUE.with_borrow_mut(|queue| queue.retry(id, deadline));
}
