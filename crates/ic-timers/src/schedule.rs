//! Checked cadence and scheduling decisions.

use std::time::Duration;
use thiserror::Error;

/// Validated positive recurrence cadence.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerCadence(u64);

impl TimerCadence {
    /// Validate a recurrence cadence.
    pub fn new(cadence: Duration) -> Result<Self, ScheduleError> {
        let cadence_ns = duration_ns(cadence)?;
        Self::from_nanos(cadence_ns)
    }

    /// Validate an already encoded nanosecond cadence.
    pub const fn from_nanos(cadence_ns: u64) -> Result<Self, ScheduleError> {
        if cadence_ns == 0 {
            Err(ScheduleError::ZeroCadence)
        } else {
            Ok(Self(cadence_ns))
        }
    }

    /// Return the encoded cadence in nanoseconds.
    #[must_use]
    pub const fn as_nanos(self) -> u64 {
        self.0
    }

    /// Resolve one successor relative to the supplied dispatch time.
    pub const fn deadline_after(self, now_ns: u64) -> Result<u64, ScheduleError> {
        checked_deadline_after(now_ns, self.0)
    }
}

/// Initial or explicit request for an ordinary timer deadline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerSchedule {
    /// Schedule relative to the current IC time.
    After(Duration),
    /// Schedule at an absolute IC timestamp in nanoseconds.
    At(u64),
}

impl TimerSchedule {
    /// Resolve the request to an absolute deadline and optional relative delay.
    pub(crate) fn resolve(self, now_ns: u64) -> Result<ResolvedSchedule, ScheduleError> {
        match self {
            Self::After(delay) => {
                let delay_ns = duration_ns(delay)?;
                Ok(ResolvedSchedule {
                    deadline_ns: checked_deadline_after(now_ns, delay_ns)?,
                    requested_delay_ns: Some(delay_ns),
                })
            }
            Self::At(deadline_ns) => Ok(ResolvedSchedule {
                deadline_ns,
                requested_delay_ns: None,
            }),
        }
    }
}

/// Scheduling decision returned after one bounded ordinary invocation.
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
    /// Recur using the registration's configured after-completion cadence.
    RecurAfterCompletion,
}

impl TimerDirective {
    pub(crate) fn resolve(
        self,
        now_ns: u64,
        cadence: Option<TimerCadence>,
    ) -> Result<ResolvedDirective, ScheduleError> {
        match self {
            Self::Stop => Ok(ResolvedDirective {
                deadline_ns: None,
                requested_delay_ns: None,
            }),
            Self::ContinueImmediately => Ok(ResolvedDirective {
                deadline_ns: Some(now_ns),
                requested_delay_ns: Some(0),
            }),
            Self::RetryAfter(delay) => {
                let delay_ns = duration_ns(delay)?;
                Ok(ResolvedDirective {
                    deadline_ns: Some(checked_deadline_after(now_ns, delay_ns)?),
                    requested_delay_ns: Some(delay_ns),
                })
            }
            Self::ScheduleAt(deadline_ns) => Ok(ResolvedDirective {
                deadline_ns: Some(deadline_ns),
                requested_delay_ns: None,
            }),
            Self::RecurAfterCompletion => {
                let cadence = cadence.ok_or(ScheduleError::MissingCadence)?;
                Ok(ResolvedDirective {
                    deadline_ns: Some(cadence.deadline_after(now_ns)?),
                    requested_delay_ns: Some(cadence.as_nanos()),
                })
            }
        }
    }
}

/// Failure to represent a cadence or requested deadline.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ScheduleError {
    /// A recurring timer must advance by a positive duration.
    #[error("timer cadence must be greater than zero")]
    ZeroCadence,
    /// The duration cannot be represented as nanoseconds in a `u64`.
    #[error("timer delay exceeds the supported nanosecond range")]
    DelayOutOfRange,
    /// Adding the delay to the current time overflows a `u64`.
    #[error("timer deadline exceeds the supported timestamp range")]
    DeadlineOverflow,
    /// After-completion recurrence was requested without a configured cadence.
    #[error("timer directive requires an after-completion cadence")]
    MissingCadence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedSchedule {
    pub(crate) deadline_ns: u64,
    pub(crate) requested_delay_ns: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedDirective {
    pub(crate) deadline_ns: Option<u64>,
    pub(crate) requested_delay_ns: Option<u64>,
}

const fn checked_deadline_after(now_ns: u64, delay_ns: u64) -> Result<u64, ScheduleError> {
    match now_ns.checked_add(delay_ns) {
        Some(deadline_ns) => Ok(deadline_ns),
        None => Err(ScheduleError::DeadlineOverflow),
    }
}

pub(crate) fn duration_ns(duration: Duration) -> Result<u64, ScheduleError> {
    u64::try_from(duration.as_nanos()).map_err(|_| ScheduleError::DelayOutOfRange)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cadence_is_positive_bounded_and_checked() {
        assert_eq!(
            TimerCadence::new(Duration::ZERO),
            Err(ScheduleError::ZeroCadence)
        );
        assert_eq!(
            TimerCadence::new(Duration::from_secs(u64::MAX)),
            Err(ScheduleError::DelayOutOfRange)
        );

        let cadence =
            TimerCadence::new(Duration::from_nanos(5)).expect("positive cadence should be valid");
        assert_eq!(cadence.as_nanos(), 5);
        assert_eq!(cadence.deadline_after(10), Ok(15));
        assert_eq!(
            cadence.deadline_after(u64::MAX),
            Err(ScheduleError::DeadlineOverflow)
        );
    }

    #[test]
    fn schedules_and_directives_resolve_without_truncation() {
        assert_eq!(
            TimerSchedule::After(Duration::from_nanos(5)).resolve(10),
            Ok(ResolvedSchedule {
                deadline_ns: 15,
                requested_delay_ns: Some(5),
            })
        );
        assert_eq!(
            TimerSchedule::At(7).resolve(10),
            Ok(ResolvedSchedule {
                deadline_ns: 7,
                requested_delay_ns: None,
            })
        );

        let cadence = TimerCadence::from_nanos(9).expect("fixture cadence should be valid");
        assert_eq!(
            TimerDirective::RecurAfterCompletion.resolve(10, Some(cadence)),
            Ok(ResolvedDirective {
                deadline_ns: Some(19),
                requested_delay_ns: Some(9),
            })
        );
        assert_eq!(
            TimerDirective::RecurAfterCompletion.resolve(10, None),
            Err(ScheduleError::MissingCadence)
        );
        assert_eq!(
            TimerDirective::RetryAfter(Duration::from_nanos(1)).resolve(u64::MAX, None),
            Err(ScheduleError::DeadlineOverflow)
        );
    }
}
