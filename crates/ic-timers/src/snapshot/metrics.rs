//! Saturating epoch-local counters and bounded callback measurements.

use super::{TimerCompletion, TimerCompletionOutcome, TimerEpoch, TimerOutcomeSnapshot};

/// Epoch-local timer event counters.
///
/// Fields are private so mutation preserves saturation and the completion
/// partition. The registry is the sole writer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerCounters {
    schedule_requests: u64,
    wakeups_armed: u64,
    work_dispatched: u64,
    scheduler_started: u64,
    work_started: u64,
    work_completed: u64,
    succeeded: u64,
    no_work: u64,
    retryable_failure: u64,
    invariant_failure: u64,
    cancelled: u64,
    stale_wakeups: u64,
    stale_work: u64,
    coalesced: u64,
    unacknowledged: u64,
}

impl TimerCounters {
    const EMPTY: Self = Self {
        schedule_requests: 0,
        wakeups_armed: 0,
        work_dispatched: 0,
        scheduler_started: 0,
        work_started: 0,
        work_completed: 0,
        succeeded: 0,
        no_work: 0,
        retryable_failure: 0,
        invariant_failure: 0,
        cancelled: 0,
        stale_wakeups: 0,
        stale_work: 0,
        coalesced: 0,
        unacknowledged: 0,
    };

    pub(crate) const fn record_schedule_request(&mut self) {
        self.schedule_requests = self.schedule_requests.saturating_add(1);
    }

    pub(crate) const fn record_wakeup_armed(&mut self) {
        self.wakeups_armed = self.wakeups_armed.saturating_add(1);
    }

    pub(crate) const fn record_work_dispatched(&mut self) {
        self.work_dispatched = self.work_dispatched.saturating_add(1);
    }

    pub(crate) const fn record_scheduler_started(&mut self) {
        self.scheduler_started = self.scheduler_started.saturating_add(1);
    }

    pub(crate) const fn record_work_started(&mut self) {
        self.work_started = self.work_started.saturating_add(1);
    }

    pub(crate) const fn record_completion(&mut self, outcome: TimerCompletionOutcome) {
        self.work_completed = self.work_completed.saturating_add(1);
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

    pub(crate) const fn record_cancellation(&mut self) {
        self.cancelled = self.cancelled.saturating_add(1);
    }

    pub(crate) const fn record_stale_wakeup(&mut self) {
        self.stale_wakeups = self.stale_wakeups.saturating_add(1);
    }

    pub(crate) const fn record_stale_work(&mut self) {
        self.stale_work = self.stale_work.saturating_add(1);
    }

    pub(crate) const fn record_coalesced(&mut self) {
        self.coalesced = self.coalesced.saturating_add(1);
    }

    pub(crate) const fn record_unacknowledged(&mut self) {
        self.unacknowledged = self.unacknowledged.saturating_add(1);
    }

    /// Return validated activation and reconciliation requests.
    #[must_use]
    pub const fn schedule_requests(self) -> u64 {
        self.schedule_requests
    }

    /// Return ordinary or scheduler provider one-shots armed.
    #[must_use]
    pub const fn wakeups_armed(self) -> u64 {
        self.wakeups_armed
    }

    /// Return immediate watchdog work one-shots dispatched.
    #[must_use]
    pub const fn work_dispatched(self) -> u64 {
        self.work_dispatched
    }

    /// Return accepted watchdog scheduler callbacks.
    #[must_use]
    pub const fn scheduler_started(self) -> u64 {
        self.scheduler_started
    }

    /// Return accepted consumer-work callbacks.
    #[must_use]
    pub const fn work_started(self) -> u64 {
        self.work_started
    }

    /// Return consumer work whose completion accounting committed.
    #[must_use]
    pub const fn work_completed(self) -> u64 {
        self.work_completed
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

    /// Return logical cancellations that changed authoritative state.
    #[must_use]
    pub const fn cancelled(self) -> u64 {
        self.cancelled
    }

    /// Return rejected ordinary or scheduler callback generations.
    #[must_use]
    pub const fn stale_wakeups(self) -> u64 {
        self.stale_wakeups
    }

    /// Return rejected watchdog work generations.
    #[must_use]
    pub const fn stale_work(self) -> u64 {
        self.stale_work
    }

    /// Return scheduling demand satisfied without another logical arm.
    #[must_use]
    pub const fn coalesced(self) -> u64 {
        self.coalesced
    }

    /// Return committed watchdog dispatches retired without completion.
    #[must_use]
    pub const fn unacknowledged(self) -> u64 {
        self.unacknowledged
    }

    /// Check the owner-local completion partition invariant.
    #[must_use]
    #[cfg(test)]
    pub(crate) const fn completion_partition_is_valid(self) -> bool {
        self.work_completed
            == self
                .succeeded
                .saturating_add(self.no_work)
                .saturating_add(self.retryable_failure)
                .saturating_add(self.invariant_failure)
    }
}

/// Saturating aggregate for one instruction measurement role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MeasurementSummary {
    samples: u64,
    total: u64,
    latest: Option<u64>,
    maximum: Option<u64>,
}

impl MeasurementSummary {
    const EMPTY: Self = Self {
        samples: 0,
        total: 0,
        latest: None,
        maximum: None,
    };

    pub(crate) const fn record(&mut self, value: u64) {
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

/// Wasm and stable memory extents in 64 KiB pages at one instant.
///
/// Within one runtime epoch these are monotonic page extents, not allocator
/// liveness or exact live-byte measurements. Consumers that need live bytes
/// must supply an owner-derived bound for allocations within the final page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageExtent {
    wasm: u64,
    stable: u64,
}

impl MemoryPageExtent {
    const EMPTY: Self = Self { wasm: 0, stable: 0 };

    pub(crate) const fn new(wasm: u64, stable: u64) -> Self {
        Self { wasm, stable }
    }

    /// Return the Wasm linear-memory extent in 64 KiB pages.
    #[must_use]
    pub const fn wasm_pages(self) -> u64 {
        self.wasm
    }

    /// Return the stable-memory extent in 64 KiB pages.
    #[must_use]
    pub const fn stable_pages(self) -> u64 {
        self.stable
    }
}

/// Page extents observed at the start and end of one normally completed
/// callback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageSample {
    start: MemoryPageExtent,
    end: MemoryPageExtent,
}

impl MemoryPageSample {
    const EMPTY: Self = Self {
        start: MemoryPageExtent::EMPTY,
        end: MemoryPageExtent::EMPTY,
    };

    pub(crate) const fn new(start: MemoryPageExtent, end: MemoryPageExtent) -> Self {
        Self { start, end }
    }

    /// Return page extents sampled at callback start.
    #[must_use]
    pub const fn start(self) -> MemoryPageExtent {
        self.start
    }

    /// Return page extents sampled after normal callback completion.
    #[must_use]
    pub const fn end(self) -> MemoryPageExtent {
        self.end
    }

    /// Return non-negative Wasm-memory page growth observed between samples.
    ///
    /// For async ordinary work, the interval may include interleaved canister
    /// activity while the callback future is awaiting.
    #[must_use]
    pub const fn wasm_growth_pages(self) -> u64 {
        self.end.wasm.saturating_sub(self.start.wasm)
    }

    /// Return non-negative stable-memory page growth observed between samples.
    ///
    /// For async ordinary work, the interval may include interleaved canister
    /// activity while the callback future is awaiting.
    #[must_use]
    pub const fn stable_growth_pages(self) -> u64 {
        self.end.stable.saturating_sub(self.start.stable)
    }
}

/// Bounded page-extent observations for one callback role.
///
/// Absolute page extents are retained only for the latest normal completion;
/// they are never totaled. Maximums describe observed start-to-end page
/// growth, which is not exclusive allocation attribution for async work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPageSummary {
    samples: u64,
    latest: MemoryPageSample,
    maximum_wasm_growth: u64,
    maximum_stable_growth: u64,
}

impl MemoryPageSummary {
    const EMPTY: Self = Self {
        samples: 0,
        latest: MemoryPageSample::EMPTY,
        maximum_wasm_growth: 0,
        maximum_stable_growth: 0,
    };

    const fn record(&mut self, sample: MemoryPageSample) {
        self.samples = self.samples.saturating_add(1);
        self.latest = sample;
        self.maximum_wasm_growth = max_u64(self.maximum_wasm_growth, sample.wasm_growth_pages());
        self.maximum_stable_growth =
            max_u64(self.maximum_stable_growth, sample.stable_growth_pages());
    }

    /// Return the number of normally completed callback samples.
    #[must_use]
    pub const fn samples(self) -> u64 {
        self.samples
    }

    /// Return start/end page extents for the latest normal completion.
    #[must_use]
    pub const fn latest(self) -> Option<MemoryPageSample> {
        if self.samples == 0 {
            None
        } else {
            Some(self.latest)
        }
    }

    /// Return the largest observed Wasm-memory page growth in one sample.
    #[must_use]
    pub const fn maximum_wasm_growth_pages(self) -> Option<u64> {
        if self.samples == 0 {
            None
        } else {
            Some(self.maximum_wasm_growth)
        }
    }

    /// Return the largest observed stable-memory page growth in one sample.
    #[must_use]
    pub const fn maximum_stable_growth_pages(self) -> Option<u64> {
        if self.samples == 0 {
            None
        } else {
            Some(self.maximum_stable_growth)
        }
    }
}

const fn max_u64(left: u64, right: u64) -> u64 {
    if left > right { left } else { right }
}

/// Completed instruction and memory-page measurements split by callback role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerPerformance {
    scheduler_instructions: MeasurementSummary,
    work_instructions: MeasurementSummary,
    scheduler_memory_pages: MemoryPageSummary,
    work_memory_pages: MemoryPageSummary,
}

impl TimerPerformance {
    const EMPTY: Self = Self {
        scheduler_instructions: MeasurementSummary::EMPTY,
        work_instructions: MeasurementSummary::EMPTY,
        scheduler_memory_pages: MemoryPageSummary::EMPTY,
        work_memory_pages: MemoryPageSummary::EMPTY,
    };

    pub(crate) const fn record_scheduler(&mut self, instructions: u64, memory: MemoryPageSample) {
        self.scheduler_instructions.record(instructions);
        self.scheduler_memory_pages.record(memory);
    }

    pub(crate) const fn record_work(&mut self, instructions: u64, memory: MemoryPageSample) {
        self.work_instructions.record(instructions);
        self.work_memory_pages.record(memory);
    }

    /// Return normally completed scheduler instruction measurements.
    #[must_use]
    pub const fn scheduler_instructions(self) -> MeasurementSummary {
        self.scheduler_instructions
    }

    /// Return normally completed consumer-work instruction measurements.
    #[must_use]
    pub const fn work_instructions(self) -> MeasurementSummary {
        self.work_instructions
    }

    /// Return normally completed scheduler memory-page observations.
    #[must_use]
    pub const fn scheduler_memory_pages(self) -> MemoryPageSummary {
        self.scheduler_memory_pages
    }

    /// Return normally completed consumer-work memory-page observations.
    #[must_use]
    pub const fn work_memory_pages(self) -> MemoryPageSummary {
        self.work_memory_pages
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
    pub(crate) const fn new(epoch: TimerEpoch) -> Self {
        Self {
            epoch,
            outcomes: TimerOutcomeSnapshot::EMPTY,
            counters: TimerCounters::EMPTY,
            performance: TimerPerformance::EMPTY,
        }
    }

    pub(crate) const fn record_completion(
        &mut self,
        completion: TimerCompletion,
        completed_at_ns: u64,
    ) {
        self.outcomes.record_completion(completion, completed_at_ns);
        self.counters.record_completion(completion.outcome());
    }

    pub(crate) const fn record_unacknowledged(&mut self, observed_at_ns: u64) {
        self.outcomes.record_unacknowledged(observed_at_ns);
        self.counters.record_unacknowledged();
    }

    pub(crate) const fn counters_mut(&mut self) -> &mut TimerCounters {
        &mut self.counters
    }

    pub(crate) const fn record_scheduler_measurements(
        &mut self,
        instructions: u64,
        memory: MemoryPageSample,
    ) {
        self.performance.record_scheduler(instructions, memory);
    }

    pub(crate) const fn record_work_measurements(
        &mut self,
        instructions: u64,
        memory: MemoryPageSample,
    ) {
        self.performance.record_work(instructions, memory);
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

    /// Return completed instruction and memory-page measurements.
    #[must_use]
    pub const fn performance(self) -> TimerPerformance {
        self.performance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_counters_saturate() {
        let mut counters = TimerCounters {
            schedule_requests: u64::MAX,
            wakeups_armed: u64::MAX,
            work_dispatched: u64::MAX,
            scheduler_started: u64::MAX,
            work_started: u64::MAX,
            work_completed: u64::MAX,
            succeeded: u64::MAX,
            no_work: u64::MAX,
            retryable_failure: u64::MAX,
            invariant_failure: u64::MAX,
            cancelled: u64::MAX,
            stale_wakeups: u64::MAX,
            stale_work: u64::MAX,
            coalesced: u64::MAX,
            unacknowledged: u64::MAX,
        };

        counters.record_schedule_request();
        counters.record_wakeup_armed();
        counters.record_work_dispatched();
        counters.record_scheduler_started();
        counters.record_work_started();
        counters.record_completion(TimerCompletionOutcome::Success);
        counters.record_cancellation();
        counters.record_stale_wakeup();
        counters.record_stale_work();
        counters.record_coalesced();
        counters.record_unacknowledged();

        assert_eq!(counters.schedule_requests(), u64::MAX);
        assert_eq!(counters.wakeups_armed(), u64::MAX);
        assert_eq!(counters.work_dispatched(), u64::MAX);
        assert_eq!(counters.work_completed(), u64::MAX);
        assert_eq!(counters.cancelled(), u64::MAX);
        assert_eq!(counters.stale_wakeups(), u64::MAX);
        assert_eq!(counters.stale_work(), u64::MAX);
        assert_eq!(counters.coalesced(), u64::MAX);
        assert_eq!(counters.unacknowledged(), u64::MAX);
        assert!(counters.completion_partition_is_valid());
    }

    #[test]
    fn callback_measurements_are_role_specific_bounded_and_saturating() {
        let mut performance = TimerPerformance::EMPTY;
        performance.record_scheduler(
            20,
            MemoryPageSample::new(MemoryPageExtent::new(1, 2), MemoryPageExtent::new(2, 4)),
        );
        performance.record_work(
            30,
            MemoryPageSample::new(MemoryPageExtent::new(2, 4), MemoryPageExtent::new(5, 5)),
        );
        performance.record_work(
            10,
            MemoryPageSample::new(MemoryPageExtent::new(5, 5), MemoryPageExtent::new(6, 9)),
        );

        assert_eq!(performance.scheduler_instructions().total(), 20);
        assert_eq!(performance.work_instructions().samples(), 2);
        assert_eq!(performance.work_instructions().total(), 40);
        assert_eq!(performance.work_instructions().latest(), Some(10));
        assert_eq!(performance.work_instructions().maximum(), Some(30));

        let scheduler_memory = performance.scheduler_memory_pages();
        assert_eq!(
            scheduler_memory.samples(),
            performance.scheduler_instructions().samples()
        );
        assert_eq!(scheduler_memory.samples(), 1);
        let scheduler_latest = scheduler_memory
            .latest()
            .expect("scheduler sample should exist");
        assert_eq!(scheduler_latest.start().wasm_pages(), 1);
        assert_eq!(scheduler_latest.start().stable_pages(), 2);
        assert_eq!(scheduler_latest.end().wasm_pages(), 2);
        assert_eq!(scheduler_latest.end().stable_pages(), 4);
        assert_eq!(scheduler_latest.wasm_growth_pages(), 1);
        assert_eq!(scheduler_latest.stable_growth_pages(), 2);

        let work_memory = performance.work_memory_pages();
        assert_eq!(
            work_memory.samples(),
            performance.work_instructions().samples()
        );
        assert_eq!(work_memory.samples(), 2);
        let work_latest = work_memory.latest().expect("work sample should exist");
        assert_eq!(work_latest.wasm_growth_pages(), 1);
        assert_eq!(work_latest.stable_growth_pages(), 4);
        assert_eq!(work_memory.maximum_wasm_growth_pages(), Some(3));
        assert_eq!(work_memory.maximum_stable_growth_pages(), Some(4));
    }

    #[test]
    fn memory_sample_count_saturates_while_latest_and_maximum_continue() {
        let mut summary = MemoryPageSummary {
            samples: u64::MAX,
            latest: MemoryPageSample::EMPTY,
            maximum_wasm_growth: 1,
            maximum_stable_growth: 1,
        };
        let sample =
            MemoryPageSample::new(MemoryPageExtent::new(10, 20), MemoryPageExtent::new(13, 25));

        summary.record(sample);

        assert_eq!(summary.samples(), u64::MAX);
        assert_eq!(summary.latest(), Some(sample));
        assert_eq!(summary.maximum_wasm_growth_pages(), Some(3));
        assert_eq!(summary.maximum_stable_growth_pages(), Some(5));
    }
}
