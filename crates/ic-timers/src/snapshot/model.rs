//! Portable scheduling, state, outcome, and epoch values.

use crate::{ScheduleError, TimerDirective, TimerRegistration};
use std::time::Duration;

/// Configured recurrence policy for one logical timer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerPolicy {
    /// Run at most once unless an explicit later request schedules it again.
    Once,
    /// Arm the next run after the current callback completes.
    AfterCompletion {
        /// Configured delay following completion, in nanoseconds.
        cadence_ns: u64,
    },
    /// Pre-arm a successor before invoking fallible work.
    Watchdog {
        /// Configured watchdog cadence, in nanoseconds.
        cadence_ns: u64,
    },
}

impl TimerPolicy {
    /// Return the stable configured-policy label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::AfterCompletion { .. } => "after_completion",
            Self::Watchdog { .. } => "watchdog",
        }
    }

    /// Return a configured cadence when the policy recurs.
    #[must_use]
    pub const fn cadence_ns(self) -> Option<u64> {
        match self {
            Self::Once => None,
            Self::AfterCompletion { cadence_ns } | Self::Watchdog { cadence_ns } => {
                Some(cadence_ns)
            }
        }
    }

    /// Return the initial effective scheduling mode.
    #[must_use]
    pub const fn initial_mode(self) -> TimerSchedulingMode {
        match self {
            Self::Once => TimerSchedulingMode::Once,
            Self::AfterCompletion { .. } => TimerSchedulingMode::AfterCompletion,
            Self::Watchdog { .. } => TimerSchedulingMode::Watchdog,
        }
    }
}

/// Effective reason for the currently authoritative schedule.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerSchedulingMode {
    /// Initial or explicitly requested one-shot work.
    Once,
    /// Recurrence delayed from the previous completion.
    AfterCompletion,
    /// An explicit absolute deadline.
    Deadline,
    /// A delayed retry following an expected failure.
    Retry,
    /// Immediate continuation of bounded work.
    Continuation,
    /// A successor committed before fallible work begins.
    Watchdog,
}

impl TimerSchedulingMode {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::AfterCompletion => "after_completion",
            Self::Deadline => "deadline",
            Self::Retry => "retry",
            Self::Continuation => "continuation",
            Self::Watchdog => "watchdog",
        }
    }
}

/// Portable representation of the latest post-run scheduling directive.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerDirectiveSnapshot {
    /// Stop after the completed invocation.
    Stop,
    /// Continue in the next available timer message.
    ContinueImmediately,
    /// Retry after a relative delay.
    RetryAfter {
        /// Requested delay in nanoseconds.
        delay_ns: u64,
    },
    /// Schedule at one absolute IC timestamp.
    ScheduleAt {
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// Recur after a delay measured from completion.
    RecurAfter {
        /// Requested delay in nanoseconds.
        delay_ns: u64,
    },
}

impl TimerDirectiveSnapshot {
    /// Return the scheduling mode produced by this directive, if any.
    #[must_use]
    pub const fn scheduling_mode(self) -> Option<TimerSchedulingMode> {
        match self {
            Self::Stop => None,
            Self::ContinueImmediately => Some(TimerSchedulingMode::Continuation),
            Self::RetryAfter { .. } => Some(TimerSchedulingMode::Retry),
            Self::ScheduleAt { .. } => Some(TimerSchedulingMode::Deadline),
            Self::RecurAfter { .. } => Some(TimerSchedulingMode::AfterCompletion),
        }
    }
}

impl TryFrom<TimerDirective> for TimerDirectiveSnapshot {
    type Error = ScheduleError;

    fn try_from(value: TimerDirective) -> Result<Self, Self::Error> {
        Ok(match value {
            TimerDirective::Stop => Self::Stop,
            TimerDirective::ContinueImmediately => Self::ContinueImmediately,
            TimerDirective::RetryAfter(delay) => Self::RetryAfter {
                delay_ns: duration_ns(delay)?,
            },
            TimerDirective::ScheduleAt(deadline_ns) => Self::ScheduleAt { deadline_ns },
            TimerDirective::RecurAfter(delay) => Self::RecurAfter {
                delay_ns: duration_ns(delay)?,
            },
        })
    }
}

impl From<TimerDirectiveSnapshot> for TimerDirective {
    fn from(value: TimerDirectiveSnapshot) -> Self {
        match value {
            TimerDirectiveSnapshot::Stop => Self::Stop,
            TimerDirectiveSnapshot::ContinueImmediately => Self::ContinueImmediately,
            TimerDirectiveSnapshot::RetryAfter { delay_ns } => {
                Self::RetryAfter(Duration::from_nanos(delay_ns))
            }
            TimerDirectiveSnapshot::ScheduleAt { deadline_ns } => Self::ScheduleAt(deadline_ns),
            TimerDirectiveSnapshot::RecurAfter { delay_ns } => {
                Self::RecurAfter(Duration::from_nanos(delay_ns))
            }
        }
    }
}

fn duration_ns(duration: Duration) -> Result<u64, ScheduleError> {
    u64::try_from(duration.as_nanos()).map_err(|_| ScheduleError::DelayOutOfRange)
}

/// One watchdog successor made authoritative before the current work.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PreArmedSuccessor {
    /// Generation the successor callback must present.
    pub generation: u64,
    /// Absolute successor deadline in nanoseconds.
    pub deadline_ns: u64,
}

/// Scheduling portion of the canonical timer snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerSchedulingSnapshot {
    /// Timer's configured behavior across successful runs.
    pub configured_policy: TimerPolicy,
    /// Reason the currently authoritative deadline was selected.
    pub current_mode: TimerSchedulingMode,
    /// Most recent completed callback directive.
    pub latest_directive: Option<TimerDirectiveSnapshot>,
    /// Most recent relative delay requested from the wrapper.
    pub latest_requested_delay_ns: Option<u64>,
    /// Most recent relative delay actually armed with the provider.
    pub latest_armed_delay_ns: Option<u64>,
    /// Next authoritative absolute deadline.
    pub next_deadline_ns: Option<u64>,
    /// Watchdog successor committed before current fallible work.
    pub pre_armed_successor: Option<PreArmedSuccessor>,
}

impl TimerSchedulingSnapshot {
    /// Construct an unscheduled snapshot for a configured policy.
    #[must_use]
    pub const fn new(configured_policy: TimerPolicy) -> Self {
        Self {
            configured_policy,
            current_mode: configured_policy.initial_mode(),
            latest_directive: None,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            next_deadline_ns: None,
            pre_armed_successor: None,
        }
    }
}

/// Portable projection of the control registration.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerRegistrationStatus {
    /// No provider callback is registered or running.
    Unregistered,
    /// One provider callback is scheduled.
    Scheduled,
    /// One callback owns logical execution.
    Running,
}

impl TimerRegistrationStatus {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Unregistered => "unregistered",
            Self::Scheduled => "scheduled",
            Self::Running => "running",
        }
    }
}

impl From<TimerRegistration> for TimerRegistrationStatus {
    fn from(value: TimerRegistration) -> Self {
        match value {
            TimerRegistration::Unregistered => Self::Unregistered,
            TimerRegistration::Scheduled { .. } => Self::Scheduled,
            TimerRegistration::Running { .. } => Self::Running,
        }
    }
}

/// Operator-facing condition of one timer process.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerProcessCondition {
    /// Configuration prevents the timer from running.
    Disabled,
    /// Enabled but without pending work.
    Idle,
    /// Scheduled or running normally.
    Active,
    /// Waiting for an expected retry.
    Retrying,
    /// Stopped by an invariant or terminal failure.
    Failed,
    /// Expected logical work has no provider registration.
    MissingRegistration,
}

impl TimerProcessCondition {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::Idle => "idle",
            Self::Active => "active",
            Self::Retrying => "retrying",
            Self::Failed => "failed",
            Self::MissingRegistration => "missing_registration",
        }
    }
}

/// State portion of the canonical timer snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerStateSnapshot {
    /// Whether configuration permits future execution.
    pub enabled: bool,
    /// Current logical control registration.
    pub registration: TimerRegistrationStatus,
    /// Operator-facing process condition.
    pub condition: TimerProcessCondition,
    /// Latest allocated callback generation.
    pub generation: u64,
    /// Whether one callback currently owns logical execution.
    pub in_flight: bool,
}

impl Default for TimerStateSnapshot {
    fn default() -> Self {
        Self {
            enabled: true,
            registration: TimerRegistrationStatus::Unregistered,
            condition: TimerProcessCondition::Idle,
            generation: 0,
            in_flight: false,
        }
    }
}

/// Outcome of one callback that returned to the runtime.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerCompletionOutcome {
    /// Callback completed useful work successfully.
    Success,
    /// Callback completed normally but found no work.
    NoWork,
    /// Expected failure permits retry policy.
    RetryableFailure,
    /// Unexpected invariant or terminal failure.
    InvariantFailure,
}

impl TimerCompletionOutcome {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::NoWork => "no_work",
            Self::RetryableFailure => "retryable_failure",
            Self::InvariantFailure => "invariant_failure",
        }
    }
}

/// Latest observed terminal event for a timer invocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerLastOutcome {
    /// One callback returned with a classified completion.
    Completed(TimerCompletionOutcome),
    /// A started generation was later established not to have completed.
    Interrupted,
}

/// Classified result of one returned callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerCompletion {
    /// Completion classification.
    pub outcome: TimerCompletionOutcome,
    /// Bounded units of application work completed by this invocation.
    pub work_count: u64,
}

impl TimerCompletion {
    /// Construct successful completed work.
    #[must_use]
    pub const fn success(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::Success,
            work_count,
        }
    }

    /// Construct a valid no-work completion.
    #[must_use]
    pub const fn no_work() -> Self {
        Self {
            outcome: TimerCompletionOutcome::NoWork,
            work_count: 0,
        }
    }

    /// Construct an expected failure, retaining any completed partial work.
    #[must_use]
    pub const fn retryable_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::RetryableFailure,
            work_count,
        }
    }

    /// Construct an invariant failure, retaining any completed partial work.
    #[must_use]
    pub const fn invariant_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::InvariantFailure,
            work_count,
        }
    }
}

/// Latest outcome and functional failure state for one timer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerOutcomeSnapshot {
    last_outcome: Option<TimerLastOutcome>,
    last_work_count: Option<u64>,
    last_success_at_ns: Option<u64>,
    last_failure_at_ns: Option<u64>,
    last_interrupted_at_ns: Option<u64>,
    consecutive_expected_failures: u64,
}

impl TimerOutcomeSnapshot {
    /// Record one returned callback using saturating failure-streak arithmetic.
    pub const fn record_completion(&mut self, completion: TimerCompletion, completed_at_ns: u64) {
        self.last_outcome = Some(TimerLastOutcome::Completed(completion.outcome));
        self.last_work_count = Some(completion.work_count);
        match completion.outcome {
            TimerCompletionOutcome::Success | TimerCompletionOutcome::NoWork => {
                self.last_success_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures = 0;
            }
            TimerCompletionOutcome::RetryableFailure => {
                self.last_failure_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures =
                    self.consecutive_expected_failures.saturating_add(1);
            }
            TimerCompletionOutcome::InvariantFailure => {
                self.last_failure_at_ns = Some(completed_at_ns);
                self.consecutive_expected_failures = 0;
            }
        }
    }

    /// Record an interruption observed by recovery or reconstruction.
    ///
    /// An interruption is not a completed callback and does not change the
    /// expected-failure streak.
    pub const fn record_interruption(&mut self, observed_at_ns: u64) {
        self.last_outcome = Some(TimerLastOutcome::Interrupted);
        self.last_work_count = None;
        self.last_interrupted_at_ns = Some(observed_at_ns);
    }

    /// Return the latest terminal event.
    #[must_use]
    pub const fn last_outcome(self) -> Option<TimerLastOutcome> {
        self.last_outcome
    }

    /// Return work reported by the latest completion, or `None` after interruption.
    #[must_use]
    pub const fn last_work_count(self) -> Option<u64> {
        self.last_work_count
    }

    /// Return the latest successful or valid no-work completion time.
    #[must_use]
    pub const fn last_success_at_ns(self) -> Option<u64> {
        self.last_success_at_ns
    }

    /// Return the latest expected or invariant failure completion time.
    #[must_use]
    pub const fn last_failure_at_ns(self) -> Option<u64> {
        self.last_failure_at_ns
    }

    /// Return the latest time an incomplete generation was established.
    #[must_use]
    pub const fn last_interrupted_at_ns(self) -> Option<u64> {
        self.last_interrupted_at_ns
    }

    /// Return consecutive retryable failures since the latest reset outcome.
    #[must_use]
    pub const fn consecutive_expected_failures(self) -> u64 {
        self.consecutive_expected_failures
    }
}

/// Identity and start time of one runtime-local observation epoch.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerEpoch {
    /// Monotonic runtime epoch chosen by the lifecycle owner.
    pub id: u64,
    /// IC timestamp at which this observation epoch began.
    pub started_at_ns: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_failure_streak_saturates() {
        let mut outcomes = TimerOutcomeSnapshot {
            consecutive_expected_failures: u64::MAX,
            ..TimerOutcomeSnapshot::default()
        };

        outcomes.record_completion(TimerCompletion::retryable_failure(0), 10);

        assert_eq!(outcomes.consecutive_expected_failures(), u64::MAX);
    }
}
