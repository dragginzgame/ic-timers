//! Typed decisions for work completed by a timer callback.

use std::time::Duration;
use thiserror::Error;

/// Scheduling decision returned after one bounded invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerDirective {
    /// Do not schedule another invocation.
    Stop,
    /// Continue as soon as the runtime can execute another message.
    ContinueImmediately,
    /// Retry after the provided delay.
    RetryAfter(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    ScheduleAt(u64),
    /// Recur after the provided delay measured from completion.
    RecurAfter(Duration),
}

/// Failure to represent a requested deadline.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ScheduleError {
    /// The duration cannot be represented as nanoseconds in a `u64`.
    #[error("timer delay exceeds the supported nanosecond range")]
    DelayOutOfRange,
    /// Adding the delay to the current time overflows a `u64`.
    #[error("timer deadline exceeds the supported timestamp range")]
    DeadlineOverflow,
}

impl TimerDirective {
    /// Resolve this directive to its next absolute nanosecond deadline.
    ///
    /// `None` means the timer should stop. Scheduling mode and retry semantics
    /// remain available from the directive itself.
    pub fn deadline_ns(self, now_ns: u64) -> Result<Option<u64>, ScheduleError> {
        match self {
            Self::Stop => Ok(None),
            Self::ContinueImmediately => Ok(Some(now_ns)),
            Self::ScheduleAt(deadline_ns) => Ok(Some(deadline_ns)),
            Self::RetryAfter(delay) | Self::RecurAfter(delay) => {
                deadline_after(now_ns, delay).map(Some)
            }
        }
    }
}

fn deadline_after(now_ns: u64, delay: Duration) -> Result<u64, ScheduleError> {
    let delay_ns = u64::try_from(delay.as_nanos()).map_err(|_| ScheduleError::DelayOutOfRange)?;
    now_ns
        .checked_add(delay_ns)
        .ok_or(ScheduleError::DeadlineOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_all_directive_deadlines() {
        assert_eq!(TimerDirective::Stop.deadline_ns(10), Ok(None));
        assert_eq!(
            TimerDirective::ContinueImmediately.deadline_ns(10),
            Ok(Some(10))
        );
        assert_eq!(TimerDirective::ScheduleAt(50).deadline_ns(10), Ok(Some(50)));
        assert_eq!(
            TimerDirective::RetryAfter(Duration::from_nanos(5)).deadline_ns(10),
            Ok(Some(15))
        );
        assert_eq!(
            TimerDirective::RecurAfter(Duration::from_nanos(7)).deadline_ns(10),
            Ok(Some(17))
        );
    }

    #[test]
    fn rejects_overflowing_deadline() {
        assert_eq!(
            TimerDirective::RetryAfter(Duration::from_nanos(1)).deadline_ns(u64::MAX),
            Err(ScheduleError::DeadlineOverflow)
        );
    }
}
