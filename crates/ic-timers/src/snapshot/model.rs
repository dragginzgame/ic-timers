//! Closed policy, state, outcome, and epoch values.

use crate::{ScheduleError, TimerCadence, TimerDirective};
use std::time::Duration;

/// Configured recurrence policy for one logical timer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerPolicy {
    /// Run at most once unless an explicit directive or request schedules it.
    Once,
    /// Arm the next run after the current callback completes.
    AfterCompletion {
        /// Validated configured cadence.
        cadence: TimerCadence,
    },
    /// Commit a successor before dispatching synchronous fallible work.
    Watchdog {
        /// Validated configured cadence.
        cadence: TimerCadence,
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
    pub const fn cadence(self) -> Option<TimerCadence> {
        match self {
            Self::Once => None,
            Self::AfterCompletion { cadence } | Self::Watchdog { cadence } => Some(cadence),
        }
    }

    /// Return the configured cadence in nanoseconds, when present.
    #[must_use]
    pub const fn cadence_ns(self) -> Option<u64> {
        match self.cadence() {
            Some(cadence) => Some(cadence.as_nanos()),
            None => None,
        }
    }
}

/// Whether a stopped declaration remains in the bounded registry.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeclarationLifetime {
    /// Keep callback authority available for a later ensure request.
    Retained,
    /// Remove the declaration after terminal completion or cancellation.
    RemoveWhenStopped,
}

/// Effective reason for the currently authoritative ordinary schedule.
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
    /// A successor committed before fallible watchdog work.
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

/// Portable representation of the latest ordinary scheduling directive.
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
    /// Recur using the registration's configured cadence.
    RecurAfterCompletion,
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
            Self::RecurAfterCompletion => Some(TimerSchedulingMode::AfterCompletion),
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
            TimerDirective::RecurAfterCompletion => Self::RecurAfterCompletion,
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
            TimerDirectiveSnapshot::RecurAfterCompletion => Self::RecurAfterCompletion,
        }
    }
}

fn duration_ns(duration: Duration) -> Result<u64, ScheduleError> {
    u64::try_from(duration.as_nanos()).map_err(|_| ScheduleError::DelayOutOfRange)
}

/// Typed terminal failure in pure timer control.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerControlFailure {
    /// A callback generation counter reached its maximum.
    GenerationExhausted,
    /// A nested request sequence reached its maximum.
    RequestSequenceExhausted,
    /// Checked successor deadline arithmetic overflowed.
    DeadlineOverflow,
    /// A requested relative delay cannot be encoded as `u64` nanoseconds.
    DelayOutOfRange,
    /// A directive is not legal for the timer's configured policy.
    DirectiveNotAllowed,
    /// A checked registry effect could not establish canonical provider ownership.
    ProviderBindingFailed,
}

impl TimerControlFailure {
    /// Return a stable adapter-friendly label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::GenerationExhausted => "generation_exhausted",
            Self::RequestSequenceExhausted => "request_sequence_exhausted",
            Self::DeadlineOverflow => "deadline_overflow",
            Self::DelayOutOfRange => "delay_out_of_range",
            Self::DirectiveNotAllowed => "directive_not_allowed",
            Self::ProviderBindingFailed => "provider_binding_failed",
        }
    }
}

/// Why a retained declaration currently has no authoritative callback.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InactiveReason {
    /// The declaration has not yet been scheduled.
    NeverScheduled,
    /// Work returned a terminal stop decision.
    Stopped,
    /// Explicit cancellation won request arbitration.
    Cancelled,
    /// Consumer work reported an invariant or terminal failure.
    InvariantFailure,
    /// Checked pure control reached a terminal failure.
    ControlFailure(TimerControlFailure),
}

/// Coherent ordinary timer state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrdinaryRuntimeStateSnapshot {
    /// One callback generation is scheduled.
    Scheduled {
        /// Generation the callback must present.
        generation: u64,
        /// Authoritative absolute deadline.
        deadline_ns: u64,
    },
    /// One callback generation owns logical execution.
    Running {
        /// Generation owned by the running callback.
        generation: u64,
    },
}

/// Status of the watchdog attempt paired with a committed successor.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogAttemptStatus {
    /// The scheduler committed the work callback, which has not committed a start.
    Dispatched,
    /// The accepted work callback is currently executing synchronously.
    Running,
}

/// One watchdog work attempt paired with an authoritative successor.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatchdogAttemptSnapshot {
    generation: u64,
    status: WatchdogAttemptStatus,
}

impl WatchdogAttemptSnapshot {
    pub(crate) const fn new(generation: u64, status: WatchdogAttemptStatus) -> Self {
        Self { generation, status }
    }

    /// Return the attempt generation.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    /// Return whether work is dispatched or running.
    #[must_use]
    pub const fn status(self) -> WatchdogAttemptStatus {
        self.status
    }
}

/// Coherent watchdog timer state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogRuntimeStateSnapshot {
    /// One scheduler generation is authoritative and no work is outstanding.
    Scheduled {
        /// Generation the scheduler callback must present.
        scheduler_generation: u64,
        /// Authoritative absolute successor deadline.
        deadline_ns: u64,
    },
    /// A successor is authoritative while one work attempt is outstanding.
    AwaitingWork {
        /// Generation the successor scheduler must present.
        successor_generation: u64,
        /// Absolute successor deadline.
        successor_deadline_ns: u64,
        /// The one paired work attempt.
        attempt: WatchdogAttemptSnapshot,
    },
}

/// Closed policy-specific runtime state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerRuntimeStateSnapshot {
    /// The retained declaration has no authoritative callback.
    Inactive {
        /// Reason scheduling is inactive.
        reason: InactiveReason,
    },
    /// State legal only for `Once` and `AfterCompletion`.
    Ordinary(OrdinaryRuntimeStateSnapshot),
    /// State legal only for `Watchdog`.
    Watchdog(WatchdogRuntimeStateSnapshot),
}

impl TimerRuntimeStateSnapshot {
    /// Return the next authoritative deadline, when one exists.
    #[must_use]
    pub const fn next_deadline_ns(self) -> Option<u64> {
        match self {
            Self::Inactive { .. }
            | Self::Ordinary(OrdinaryRuntimeStateSnapshot::Running { .. }) => None,
            Self::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled { deadline_ns, .. })
            | Self::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled { deadline_ns, .. }) => {
                Some(deadline_ns)
            }
            Self::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
                successor_deadline_ns,
                ..
            }) => Some(successor_deadline_ns),
        }
    }
}

/// Portable projection of the provider-neutral registration state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerRegistrationStatus {
    /// No callback is authoritative.
    Unregistered,
    /// At least one provider callback is scheduled.
    Scheduled,
    /// Consumer work currently owns logical execution.
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

impl From<TimerRuntimeStateSnapshot> for TimerRegistrationStatus {
    fn from(value: TimerRuntimeStateSnapshot) -> Self {
        match value {
            TimerRuntimeStateSnapshot::Inactive { .. } => Self::Unregistered,
            TimerRuntimeStateSnapshot::Ordinary(state) => match state {
                OrdinaryRuntimeStateSnapshot::Scheduled { .. } => Self::Scheduled,
                OrdinaryRuntimeStateSnapshot::Running { .. } => Self::Running,
            },
            TimerRuntimeStateSnapshot::Watchdog(state) => match state {
                WatchdogRuntimeStateSnapshot::Scheduled { .. }
                | WatchdogRuntimeStateSnapshot::AwaitingWork {
                    attempt:
                        WatchdogAttemptSnapshot {
                            status: WatchdogAttemptStatus::Dispatched,
                            ..
                        },
                    ..
                } => Self::Scheduled,
                WatchdogRuntimeStateSnapshot::AwaitingWork {
                    attempt:
                        WatchdogAttemptSnapshot {
                            status: WatchdogAttemptStatus::Running,
                            ..
                        },
                    ..
                } => Self::Running,
            },
        }
    }
}

/// Operator-facing condition derived from coherent state and scheduling mode.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerProcessCondition {
    /// Explicit cancellation disabled the current declaration.
    Disabled,
    /// Declared but without pending work.
    Idle,
    /// Scheduled or running normally.
    Active,
    /// Waiting for an expected retry.
    Retrying,
    /// Stopped by an invariant or control failure.
    Failed,
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

/// Latest observed terminal event for one timer invocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerLastOutcome {
    /// One callback returned with a classified completion.
    Completed(TimerCompletionOutcome),
    /// A committed watchdog dispatch was retired without committed completion.
    Unacknowledged,
}

/// Classified result of one returned consumer invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerCompletion {
    outcome: TimerCompletionOutcome,
    work_count: u64,
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

    /// Construct an expected failure, retaining completed partial work.
    #[must_use]
    pub const fn retryable_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::RetryableFailure,
            work_count,
        }
    }

    /// Construct an invariant failure, retaining completed partial work.
    #[must_use]
    pub const fn invariant_failure(work_count: u64) -> Self {
        Self {
            outcome: TimerCompletionOutcome::InvariantFailure,
            work_count,
        }
    }

    /// Return the completion class.
    #[must_use]
    pub const fn outcome(self) -> TimerCompletionOutcome {
        self.outcome
    }

    /// Return bounded application work units reported by the consumer.
    #[must_use]
    pub const fn work_count(self) -> u64 {
        self.work_count
    }
}

/// Ordinary callback result with one legal scheduling proposal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerRunResult {
    completion: TimerCompletion,
    directive: TimerDirective,
}

impl TimerRunResult {
    /// Construct a result, forcing invariant failures to stop.
    #[must_use]
    pub const fn new(completion: TimerCompletion, directive: TimerDirective) -> Self {
        Self {
            directive: if matches!(completion.outcome, TimerCompletionOutcome::InvariantFailure) {
                TimerDirective::Stop
            } else {
                directive
            },
            completion,
        }
    }

    /// Return the completion classification and work count.
    #[must_use]
    pub const fn completion(self) -> TimerCompletion {
        self.completion
    }

    /// Return the post-run scheduling proposal.
    #[must_use]
    pub const fn directive(self) -> TimerDirective {
        self.directive
    }
}

/// Watchdog decision after one synchronous bounded work attempt.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatchdogDecision {
    /// Retain the successor committed by the scheduler message.
    Continue,
    /// Terminate and clear the committed successor.
    Stop,
}

/// Synchronous watchdog work result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WatchdogRunResult {
    completion: TimerCompletion,
    decision: WatchdogDecision,
}

impl WatchdogRunResult {
    /// Construct a result, forcing invariant failures to stop.
    #[must_use]
    pub const fn new(completion: TimerCompletion, decision: WatchdogDecision) -> Self {
        Self {
            decision: if matches!(completion.outcome, TimerCompletionOutcome::InvariantFailure) {
                WatchdogDecision::Stop
            } else {
                decision
            },
            completion,
        }
    }

    /// Return the completion classification and work count.
    #[must_use]
    pub const fn completion(self) -> TimerCompletion {
        self.completion
    }

    /// Return whether the committed successor remains authoritative.
    #[must_use]
    pub const fn decision(self) -> WatchdogDecision {
        self.decision
    }
}

/// Latest outcome and functional failure state for one timer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerOutcomeSnapshot {
    last_outcome: Option<TimerLastOutcome>,
    last_work_count: Option<u64>,
    last_success_at_ns: Option<u64>,
    last_failure_at_ns: Option<u64>,
    last_unacknowledged_at_ns: Option<u64>,
    consecutive_expected_failures: u64,
}

impl TimerOutcomeSnapshot {
    pub(crate) const fn new() -> Self {
        Self {
            last_outcome: None,
            last_work_count: None,
            last_success_at_ns: None,
            last_failure_at_ns: None,
            last_unacknowledged_at_ns: None,
            consecutive_expected_failures: 0,
        }
    }

    pub(crate) const fn record_completion(
        &mut self,
        completion: TimerCompletion,
        completed_at_ns: u64,
    ) {
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

    pub(crate) const fn record_unacknowledged(&mut self, observed_at_ns: u64) {
        self.last_outcome = Some(TimerLastOutcome::Unacknowledged);
        self.last_work_count = None;
        self.last_unacknowledged_at_ns = Some(observed_at_ns);
    }

    /// Return the latest terminal event.
    #[must_use]
    pub const fn last_outcome(self) -> Option<TimerLastOutcome> {
        self.last_outcome
    }

    /// Return work reported by the latest completion.
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

    /// Return when a dispatched watchdog attempt was most recently retired.
    #[must_use]
    pub const fn last_unacknowledged_at_ns(self) -> Option<u64> {
        self.last_unacknowledged_at_ns
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
    canister_version: u64,
    started_at_ns: u64,
}

impl TimerEpoch {
    pub(crate) const fn new(canister_version: u64, started_at_ns: u64) -> Self {
        Self {
            canister_version,
            started_at_ns,
        }
    }

    /// Return the IC canister version that owns this volatile epoch.
    #[must_use]
    pub const fn canister_version(self) -> u64 {
        self.canister_version
    }

    /// Return the IC timestamp at which the epoch began.
    #[must_use]
    pub const fn started_at_ns(self) -> u64 {
        self.started_at_ns
    }
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

    #[test]
    fn invariant_results_are_forced_to_stop() {
        let ordinary = TimerRunResult::new(
            TimerCompletion::invariant_failure(2),
            TimerDirective::ContinueImmediately,
        );
        assert_eq!(ordinary.directive(), TimerDirective::Stop);

        let watchdog = WatchdogRunResult::new(
            TimerCompletion::invariant_failure(3),
            WatchdogDecision::Continue,
        );
        assert_eq!(watchdog.decision(), WatchdogDecision::Stop);
    }
}
