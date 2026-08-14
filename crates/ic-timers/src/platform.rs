//! Private boundary to IC system facts and `ic-cdk-timers`.
//!
//! Recurrence, identity, arbitration, inventory, and consumer work belong to
//! the canonical runtime above this module.

use std::{future::Future, time::Duration};

#[cfg(not(test))]
use ic_cdk_timers::{
    TimerId as CdkTimerId, clear_timer as cdk_clear_timer, set_timer as cdk_set_timer,
};

/// Linear handle for one armed platform timer.
#[cfg(not(test))]
#[derive(Debug, Eq, PartialEq)]
pub struct TimerHandle(CdkTimerId);

/// Arm one asynchronous one-shot callback.
#[cfg(not(test))]
pub fn set_timer(delay: Duration, task: impl Future<Output = ()> + 'static) -> TimerHandle {
    TimerHandle(cdk_set_timer(delay, task))
}

/// Clear one still-armed platform callback.
#[cfg(not(test))]
#[allow(clippy::needless_pass_by_value)] // Clearing consumes the linear handle.
pub fn clear_timer(handle: TimerHandle) {
    cdk_clear_timer(handle.0);
}

/// Return current IC message time in nanoseconds.
#[cfg(not(test))]
pub fn time_ns() -> u64 {
    ic0::time()
}

/// Return the current canister version.
#[cfg(not(test))]
pub fn canister_version() -> u64 {
    ic0::canister_version()
}

/// Return call-context instruction consumption on the IC.
#[cfg(not(test))]
pub fn instruction_counter() -> u64 {
    ic0::performance_counter(1)
}

/// Abort the current message when an internal callback invariant is violated.
#[cfg(not(test))]
pub fn trap(message: &str) -> ! {
    ic0::trap(message.as_bytes())
}

#[cfg(test)]
pub use fake::{
    TimerHandle, advance_instructions, canister_version, clear_timer, discard_next_due,
    instruction_counter, reset, run_next_due, set_time, set_timer, time_ns, timer_count, trap,
};

#[cfg(test)]
mod fake {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        collections::BTreeMap,
        pin::Pin,
        task::{Context, Poll, Waker},
    };

    type Task = Pin<Box<dyn Future<Output = ()>>>;

    struct ScheduledTask {
        deadline_ns: u64,
        task: Task,
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct TimerHandle(u64);

    thread_local! {
        static NOW_NS: Cell<u64> = const { Cell::new(0) };
        static CANISTER_VERSION: Cell<u64> = const { Cell::new(0) };
        static NEXT_HANDLE: Cell<u64> = const { Cell::new(0) };
        static INSTRUCTIONS: Cell<u64> = const { Cell::new(0) };
        static TASKS: RefCell<BTreeMap<u64, ScheduledTask>> = const {
            RefCell::new(BTreeMap::new())
        };
    }

    pub fn set_timer(delay: Duration, task: impl Future<Output = ()> + 'static) -> TimerHandle {
        advance_instructions(5);
        let delay_ns = u64::try_from(delay.as_nanos()).unwrap_or(u64::MAX);
        let deadline_ns = time_ns().saturating_add(delay_ns);
        let handle = NEXT_HANDLE.with(|next| {
            let handle = next.get().saturating_add(1);
            next.set(handle);
            handle
        });
        TASKS.with(|tasks| {
            tasks.borrow_mut().insert(
                handle,
                ScheduledTask {
                    deadline_ns,
                    task: Box::pin(task),
                },
            );
        });
        TimerHandle(handle)
    }

    #[allow(clippy::needless_pass_by_value)] // Clearing consumes the linear handle.
    pub fn clear_timer(handle: TimerHandle) {
        advance_instructions(2);
        TASKS.with(|tasks| {
            tasks.borrow_mut().remove(&handle.0);
        });
    }

    pub fn time_ns() -> u64 {
        NOW_NS.with(Cell::get)
    }

    pub fn canister_version() -> u64 {
        CANISTER_VERSION.with(Cell::get)
    }

    pub fn instruction_counter() -> u64 {
        INSTRUCTIONS.with(Cell::get)
    }

    pub fn trap(message: &str) -> ! {
        panic!("{message}")
    }

    pub fn advance_instructions(amount: u64) {
        INSTRUCTIONS.with(|instructions| {
            instructions.set(instructions.get().saturating_add(amount));
        });
    }

    pub fn reset(now_ns: u64, canister_version: u64) {
        NOW_NS.with(|now| now.set(now_ns));
        CANISTER_VERSION.with(|version| version.set(canister_version));
        NEXT_HANDLE.with(|next| next.set(0));
        INSTRUCTIONS.with(|instructions| instructions.set(0));
        TASKS.with(|tasks| tasks.borrow_mut().clear());
    }

    pub fn set_time(now_ns: u64) {
        NOW_NS.with(|now| now.set(now_ns));
    }

    pub fn timer_count() -> usize {
        TASKS.with(|tasks| tasks.borrow().len())
    }

    pub fn run_next_due() -> bool {
        let now_ns = time_ns();
        let next_handle = next_due_handle(now_ns);
        let Some(next_handle) = next_handle else {
            return false;
        };
        let Some(mut scheduled) = TASKS.with(|tasks| tasks.borrow_mut().remove(&next_handle))
        else {
            return false;
        };

        let mut context = Context::from_waker(Waker::noop());
        if matches!(scheduled.task.as_mut().poll(&mut context), Poll::Pending) {
            TASKS.with(|tasks| {
                tasks.borrow_mut().insert(next_handle, scheduled);
            });
        }
        true
    }

    pub fn discard_next_due() -> bool {
        let Some(next_handle) = next_due_handle(time_ns()) else {
            return false;
        };
        TASKS.with(|tasks| tasks.borrow_mut().remove(&next_handle).is_some())
    }

    fn next_due_handle(now_ns: u64) -> Option<u64> {
        TASKS.with(|tasks| {
            tasks
                .borrow()
                .iter()
                .filter(|(_, scheduled)| scheduled.deadline_ns <= now_ns)
                .min_by_key(|(handle, scheduled)| (scheduled.deadline_ns, **handle))
                .map(|(handle, _)| *handle)
        })
    }
}
