//! Saturating epoch-local counters and performance aggregates.

use super::{TimerCompletion, TimerCompletionOutcome, TimerEpoch, TimerOutcomeSnapshot};

/// Epoch-local timer event counters.
///
/// Fields are private so all mutation preserves saturation and the completion
/// partition. Consumer adapters read values through the accessors.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerCounters {
    requested: u64,
    armed: u64,
    started: u64,
    completed: u64,
    succeeded: u64,
    no_work: u64,
    retryable_failure: u64,
    invariant_failure: u64,
    cancelled: u64,
    stale: u64,
    coalesced: u64,
    interrupted: u64,
}

impl TimerCounters {
    /// Record a validated schedule or reconciliation request.
    pub const fn record_request(&mut self) {
        self.requested = self.requested.saturating_add(1);
    }

    /// Record an actual one-shot provider arm.
    pub const fn record_arm(&mut self) {
        self.armed = self.armed.saturating_add(1);
    }

    /// Record a non-stale callback entering logical execution.
    pub const fn record_start(&mut self) {
        self.started = self.started.saturating_add(1);
    }

    /// Record one returned callback and its single completion class.
    pub const fn record_completion(&mut self, outcome: TimerCompletionOutcome) {
        self.completed = self.completed.saturating_add(1);
        match outcome {
            TimerCompletionOutcome::Success => {
                self.succeeded = self.succeeded.saturating_add(1);
            }
            TimerCompletionOutcome::NoWork => {
                self.no_work = self.no_work.saturating_add(1);
            }
            TimerCompletionOutcome::RetryableFailure => {
                self.retryable_failure = self.retryable_failure.saturating_add(1);
            }
            TimerCompletionOutcome::InvariantFailure => {
                self.invariant_failure = self.invariant_failure.saturating_add(1);
            }
        }
    }

    /// Record a logical cancellation that wins arbitration.
    pub const fn record_cancellation(&mut self) {
        self.cancelled = self.cancelled.saturating_add(1);
    }

    /// Record a provider callback or completion rejected as stale.
    pub const fn record_stale(&mut self) {
        self.stale = self.stale.saturating_add(1);
    }

    /// Record scheduling demand merged into existing work.
    pub const fn record_coalesced(&mut self) {
        self.coalesced = self.coalesced.saturating_add(1);
    }

    /// Record a started generation established not to have completed.
    ///
    /// The interruption belongs to the epoch in which it is observed. It can
    /// therefore describe a generation started in an earlier epoch and has no
    /// arithmetic invariant with this epoch's `started` count.
    pub const fn record_interruption(&mut self) {
        self.interrupted = self.interrupted.saturating_add(1);
    }

    /// Return validated scheduling requests.
    #[must_use]
    pub const fn requested(self) -> u64 {
        self.requested
    }

    /// Return actual provider arm operations.
    #[must_use]
    pub const fn armed(self) -> u64 {
        self.armed
    }

    /// Return callbacks that entered logical execution.
    #[must_use]
    pub const fn started(self) -> u64 {
        self.started
    }

    /// Return callbacks that completed accounting.
    #[must_use]
    pub const fn completed(self) -> u64 {
        self.completed
    }

    /// Return successful-work completions.
    #[must_use]
    pub const fn succeeded(self) -> u64 {
        self.succeeded
    }

    /// Return valid no-work completions.
    #[must_use]
    pub const fn no_work(self) -> u64 {
        self.no_work
    }

    /// Return retryable expected-failure completions.
    #[must_use]
    pub const fn retryable_failure(self) -> u64 {
        self.retryable_failure
    }

    /// Return invariant or terminal-failure completions.
    #[must_use]
    pub const fn invariant_failure(self) -> u64 {
        self.invariant_failure
    }

    /// Return logical cancellations that won arbitration.
    #[must_use]
    pub const fn cancelled(self) -> u64 {
        self.cancelled
    }

    /// Return callbacks or completions rejected as stale.
    #[must_use]
    pub const fn stale(self) -> u64 {
        self.stale
    }

    /// Return scheduling demands merged into existing work.
    #[must_use]
    pub const fn coalesced(self) -> u64 {
        self.coalesced
    }

    /// Return incomplete generations observed in this epoch.
    #[must_use]
    pub const fn interrupted(self) -> u64 {
        self.interrupted
    }

    /// Check that every completion belongs to exactly one outcome class.
    #[must_use]
    pub const fn completion_partition_is_valid(self) -> bool {
        self.completed
            == self
                .succeeded
                .saturating_add(self.no_work)
                .saturating_add(self.retryable_failure)
                .saturating_add(self.invariant_failure)
    }
}

/// One callback's completed performance measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerMeasurement {
    /// Instructions consumed by the callback.
    pub instructions: u64,
    /// Wall-clock callback duration in nanoseconds.
    pub elapsed_ns: u64,
}

/// Saturating aggregate for one non-negative measurement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MeasurementSummary {
    samples: u64,
    total: u64,
    latest: Option<u64>,
    maximum: Option<u64>,
}

impl MeasurementSummary {
    /// Record one completed sample.
    pub const fn record(&mut self, value: u64) {
        self.samples = self.samples.saturating_add(1);
        self.total = self.total.saturating_add(value);
        self.latest = Some(value);
        self.maximum = Some(match self.maximum {
            Some(current) if current > value => current,
            Some(_) | None => value,
        });
    }

    /// Return the number of completed samples.
    #[must_use]
    pub const fn samples(self) -> u64 {
        self.samples
    }

    /// Return the saturating sum of all samples.
    #[must_use]
    pub const fn total(self) -> u64 {
        self.total
    }

    /// Return the latest sample, if one exists.
    #[must_use]
    pub const fn latest(self) -> Option<u64> {
        self.latest
    }

    /// Return the largest sample, if one exists.
    #[must_use]
    pub const fn maximum(self) -> Option<u64> {
        self.maximum
    }
}

/// Completed callback performance aggregates for one runtime epoch.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerPerformance {
    instructions: MeasurementSummary,
    elapsed_ns: MeasurementSummary,
}

impl TimerPerformance {
    /// Record one callback with valid end measurements.
    pub const fn record(&mut self, measurement: TimerMeasurement) {
        self.instructions.record(measurement.instructions);
        self.elapsed_ns.record(measurement.elapsed_ns);
    }

    /// Return the instruction aggregate.
    #[must_use]
    pub const fn instructions(self) -> MeasurementSummary {
        self.instructions
    }

    /// Return the elapsed-nanosecond aggregate.
    #[must_use]
    pub const fn elapsed_ns(self) -> MeasurementSummary {
        self.elapsed_ns
    }
}

/// Epoch-scoped outcomes, counters, and performance for one timer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerObservabilitySnapshot {
    epoch: TimerEpoch,
    outcomes: TimerOutcomeSnapshot,
    counters: TimerCounters,
    performance: TimerPerformance,
}

impl TimerObservabilitySnapshot {
    /// Begin empty observation state for one runtime epoch.
    #[must_use]
    pub fn new(epoch: TimerEpoch) -> Self {
        Self {
            epoch,
            outcomes: TimerOutcomeSnapshot::default(),
            counters: TimerCounters::default(),
            performance: TimerPerformance::default(),
        }
    }

    /// Replace all epoch-scoped observations with a new empty epoch.
    ///
    /// This resets outcomes, timestamps, the expected-failure streak,
    /// counters, and performance aggregates. Persistent scheduling state is
    /// owned outside this observation value.
    pub fn begin_epoch(&mut self, epoch: TimerEpoch) {
        *self = Self::new(epoch);
    }

    /// Record one returned callback atomically across outcome and counters.
    ///
    /// Performance changes only when a valid end measurement is supplied.
    pub const fn record_completion(
        &mut self,
        completion: TimerCompletion,
        completed_at_ns: u64,
        measurement: Option<TimerMeasurement>,
    ) {
        self.outcomes.record_completion(completion, completed_at_ns);
        self.counters.record_completion(completion.outcome);
        if let Some(measurement) = measurement {
            self.performance.record(measurement);
        }
    }

    /// Record an interruption in the epoch where it becomes observable.
    pub const fn record_interruption(&mut self, observed_at_ns: u64) {
        self.outcomes.record_interruption(observed_at_ns);
        self.counters.record_interruption();
    }

    /// Record a validated schedule or reconciliation request.
    pub const fn record_request(&mut self) {
        self.counters.record_request();
    }

    /// Record an actual one-shot provider arm.
    pub const fn record_arm(&mut self) {
        self.counters.record_arm();
    }

    /// Record a non-stale callback entering logical execution.
    pub const fn record_start(&mut self) {
        self.counters.record_start();
    }

    /// Record a logical cancellation that wins arbitration.
    pub const fn record_cancellation(&mut self) {
        self.counters.record_cancellation();
    }

    /// Record a provider callback or completion rejected as stale.
    pub const fn record_stale(&mut self) {
        self.counters.record_stale();
    }

    /// Record scheduling demand merged into existing work.
    pub const fn record_coalesced(&mut self) {
        self.counters.record_coalesced();
    }

    /// Return the observation epoch.
    #[must_use]
    pub const fn epoch(self) -> TimerEpoch {
        self.epoch
    }

    /// Return latest outcomes and functional failure state.
    #[must_use]
    pub const fn outcomes(self) -> TimerOutcomeSnapshot {
        self.outcomes
    }

    /// Return epoch-local event counters.
    #[must_use]
    pub const fn counters(self) -> TimerCounters {
        self.counters
    }

    /// Return completed callback performance aggregates.
    #[must_use]
    pub const fn performance(self) -> TimerPerformance {
        self.performance
    }

    /// Return functional expected-failure state without rebuilding a snapshot.
    #[must_use]
    pub const fn consecutive_expected_failures(self) -> u64 {
        self.outcomes.consecutive_expected_failures()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_counters_saturate() {
        let mut counters = TimerCounters {
            requested: u64::MAX,
            armed: u64::MAX,
            started: u64::MAX,
            completed: u64::MAX,
            succeeded: u64::MAX,
            no_work: u64::MAX,
            retryable_failure: u64::MAX,
            invariant_failure: u64::MAX,
            cancelled: u64::MAX,
            stale: u64::MAX,
            coalesced: u64::MAX,
            interrupted: u64::MAX,
        };

        counters.record_request();
        counters.record_arm();
        counters.record_start();
        counters.record_completion(TimerCompletionOutcome::Success);
        counters.record_cancellation();
        counters.record_stale();
        counters.record_coalesced();
        counters.record_interruption();

        assert_eq!(counters.requested(), u64::MAX);
        assert_eq!(counters.armed(), u64::MAX);
        assert_eq!(counters.started(), u64::MAX);
        assert_eq!(counters.completed(), u64::MAX);
        assert_eq!(counters.succeeded(), u64::MAX);
        assert_eq!(counters.cancelled(), u64::MAX);
        assert_eq!(counters.stale(), u64::MAX);
        assert_eq!(counters.coalesced(), u64::MAX);
        assert_eq!(counters.interrupted(), u64::MAX);
        assert!(counters.completion_partition_is_valid());
    }

    #[test]
    fn measurement_count_and_total_saturate_while_latest_and_maximum_advance() {
        let mut summary = MeasurementSummary {
            samples: u64::MAX,
            total: u64::MAX,
            latest: Some(10),
            maximum: Some(20),
        };

        summary.record(30);

        assert_eq!(summary.samples(), u64::MAX);
        assert_eq!(summary.total(), u64::MAX);
        assert_eq!(summary.latest(), Some(30));
        assert_eq!(summary.maximum(), Some(30));
    }
}
