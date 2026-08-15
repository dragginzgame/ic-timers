//! Provider-neutral identity and coherent canonical snapshot values.
//!
//! Public snapshots are inert observations with private top-level fields. The
//! registry is their only constructor and mutation authority.

mod identity;
mod metrics;
mod model;

pub use identity::{
    MAX_TIMER_IDENTITY_COMPONENT_BYTES, TimerIdentity, TimerIdentityError, TimerIdentityField,
};
pub use metrics::{
    MeasurementSummary, MemoryPageExtent, MemoryPageSample, MemoryPageSummary, TimerCounters,
    TimerObservabilitySnapshot, TimerPerformance,
};
pub use model::{
    DeclarationLifetime, InactiveReason, OrdinaryRuntimeStateSnapshot, TimerCompletion,
    TimerCompletionOutcome, TimerControlFailure, TimerDirectiveSnapshot, TimerEpoch,
    TimerLastOutcome, TimerOutcomeSnapshot, TimerPolicy, TimerProcessCondition,
    TimerRegistrationStatus, TimerRunResult, TimerRuntimeStateSnapshot, TimerSchedulingMode,
    WatchdogAttemptSnapshot, WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult,
    WatchdogRuntimeStateSnapshot,
};

/// Canonical provider-neutral operator snapshot for one logical timer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimerSnapshot {
    identity: TimerIdentity,
    policy: TimerPolicy,
    lifetime: DeclarationLifetime,
    state: TimerRuntimeStateSnapshot,
    scheduling_mode: TimerSchedulingMode,
    latest_directive: Option<TimerDirectiveSnapshot>,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    observability: TimerObservabilitySnapshot,
}

impl TimerSnapshot {
    #[allow(clippy::too_many_arguments)] // Registry-only constructor keeps one coherent boundary.
    pub(crate) const fn new(
        identity: TimerIdentity,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
        state: TimerRuntimeStateSnapshot,
        scheduling_mode: TimerSchedulingMode,
        latest_directive: Option<TimerDirectiveSnapshot>,
        latest_requested_delay_ns: Option<u64>,
        latest_armed_delay_ns: Option<u64>,
        observability: &TimerObservabilitySnapshot,
    ) -> Self {
        Self {
            identity,
            policy,
            lifetime,
            state,
            scheduling_mode,
            latest_directive,
            latest_requested_delay_ns,
            latest_armed_delay_ns,
            observability: *observability,
        }
    }

    /// Return the stable structured identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    /// Return the configured scheduling policy.
    #[must_use]
    pub const fn policy(&self) -> TimerPolicy {
        self.policy
    }

    /// Return whether the declaration remains after terminal stop.
    #[must_use]
    pub const fn lifetime(&self) -> DeclarationLifetime {
        self.lifetime
    }

    /// Return the closed policy-specific runtime state.
    #[must_use]
    pub const fn state(&self) -> TimerRuntimeStateSnapshot {
        self.state
    }

    /// Return the effective scheduling mode.
    ///
    /// A new declaration starts with its configured policy mode. Later
    /// requests and completed directives update this value, including after
    /// the declaration becomes inactive.
    #[must_use]
    pub const fn scheduling_mode(&self) -> TimerSchedulingMode {
        self.scheduling_mode
    }

    /// Return the latest completed ordinary directive.
    #[must_use]
    pub const fn latest_directive(&self) -> Option<TimerDirectiveSnapshot> {
        self.latest_directive
    }

    /// Return the latest requested relative delay.
    #[must_use]
    pub const fn latest_requested_delay_ns(&self) -> Option<u64> {
        self.latest_requested_delay_ns
    }

    /// Return the latest relative delay whose provider arm committed.
    #[must_use]
    pub const fn latest_armed_delay_ns(&self) -> Option<u64> {
        self.latest_armed_delay_ns
    }

    /// Return the next authoritative absolute deadline.
    #[must_use]
    pub const fn next_deadline_ns(&self) -> Option<u64> {
        self.state.next_deadline_ns()
    }

    /// Return a portable registration projection.
    #[must_use]
    pub fn registration_status(&self) -> TimerRegistrationStatus {
        self.state.into()
    }

    /// Return an operator-facing condition derived from coherent state.
    #[must_use]
    pub const fn process_condition(&self) -> TimerProcessCondition {
        match self.state {
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::Cancelled,
            } => TimerProcessCondition::Disabled,
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::InvariantFailure | InactiveReason::ControlFailure(_),
            } => TimerProcessCondition::Failed,
            TimerRuntimeStateSnapshot::Inactive {
                reason: InactiveReason::Stopped,
            } if matches!(
                self.observability.outcomes().last_outcome(),
                Some(TimerLastOutcome::Completed(
                    TimerCompletionOutcome::RetryableFailure
                ))
            ) =>
            {
                TimerProcessCondition::Failed
            }
            TimerRuntimeStateSnapshot::Inactive { .. } => TimerProcessCondition::Idle,
            TimerRuntimeStateSnapshot::Ordinary(_) | TimerRuntimeStateSnapshot::Watchdog(_)
                if matches!(self.scheduling_mode, TimerSchedulingMode::Retry) =>
            {
                TimerProcessCondition::Retrying
            }
            TimerRuntimeStateSnapshot::Ordinary(_) | TimerRuntimeStateSnapshot::Watchdog(_) => {
                TimerProcessCondition::Active
            }
        }
    }

    /// Return the latest authoritative callback generation.
    #[must_use]
    pub const fn generation(&self) -> Option<u64> {
        match self.state {
            TimerRuntimeStateSnapshot::Inactive { .. } => None,
            TimerRuntimeStateSnapshot::Ordinary(
                OrdinaryRuntimeStateSnapshot::Scheduled { generation, .. }
                | OrdinaryRuntimeStateSnapshot::Running { generation },
            ) => Some(generation),
            TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                scheduler_generation,
                ..
            }) => Some(scheduler_generation),
            TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::AwaitingWork {
                successor_generation,
                ..
            }) => Some(successor_generation),
        }
    }

    /// Return epoch-scoped outcomes, counters, and measurements.
    #[must_use]
    pub const fn observability(&self) -> TimerObservabilitySnapshot {
        self.observability
    }
}

#[cfg(test)]
mod tests;
