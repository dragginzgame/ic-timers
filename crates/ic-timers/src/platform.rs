//! Direct boundary to the Internet Computer timer provider.
//!
//! Recurrence, identity, arbitration, inventory, and measurement belong above
//! this module.

use ic_cdk_timers::{
    TimerId as CdkTimerId, clear_timer as cdk_clear_timer, set_timer as cdk_set_timer,
};
use std::{future::Future, time::Duration};

/// Opaque handle for one armed platform timer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerHandle(CdkTimerId);

/// Arm one asynchronous one-shot callback.
///
/// Higher-level policies should use one-shots even for recurring work so they
/// can control exactly when a successor becomes authoritative.
pub fn set_timer(delay: Duration, task: impl Future<Output = ()> + 'static) -> TimerHandle {
    TimerHandle(cdk_set_timer(delay, task))
}

/// Clear one still-armed platform callback.
pub fn clear_timer(handle: TimerHandle) {
    cdk_clear_timer(handle.0);
}
