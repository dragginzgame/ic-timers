//! Provider-neutral timer identity and canonical snapshot values.
//!
//! This module defines the 0.2 observability contract without owning registry
//! storage, callback execution, platform effects, or consumer serialization.

mod identity;
mod metrics;
mod model;

pub use identity::{
    MAX_TIMER_LABEL_BYTES, TimerIdentity, TimerIdentityError, TimerIdentityField, TimerLabel,
    TimerLabelError,
};
pub use metrics::{
    MeasurementSummary, TimerCounters, TimerMeasurement, TimerObservabilitySnapshot,
    TimerPerformance,
};
pub use model::{
    PreArmedSuccessor, TimerCompletion, TimerCompletionOutcome, TimerDirectiveSnapshot, TimerEpoch,
    TimerLastOutcome, TimerOutcomeSnapshot, TimerPolicy, TimerProcessCondition,
    TimerRegistrationStatus, TimerSchedulingMode, TimerSchedulingSnapshot, TimerStateSnapshot,
};

/// Canonical provider-neutral operator snapshot for one logical timer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimerSnapshot {
    /// Stable structured identity used for lookup and deterministic inventory ordering.
    pub identity: TimerIdentity,
    /// Configured policy and authoritative scheduling details.
    pub scheduling: TimerSchedulingSnapshot,
    /// Current logical and operator-facing state.
    pub state: TimerStateSnapshot,
    /// Epoch-scoped outcomes, counters, measurements, and functional failure streak.
    pub observability: TimerObservabilitySnapshot,
}

impl TimerSnapshot {
    /// Construct an unregistered timer snapshot for one observation epoch.
    #[must_use]
    pub fn new(identity: TimerIdentity, policy: TimerPolicy, epoch: TimerEpoch) -> Self {
        Self {
            identity,
            scheduling: TimerSchedulingSnapshot::new(policy),
            state: TimerStateSnapshot {
                enabled: true,
                registration: TimerRegistrationStatus::Unregistered,
                condition: TimerProcessCondition::Idle,
                generation: 0,
                in_flight: false,
            },
            observability: TimerObservabilitySnapshot::new(epoch),
        }
    }

    /// Return recovery-sensitive expected-failure state for this timer.
    ///
    /// A registry can expose this after one ordered lookup by `identity`;
    /// callers do not need to scan counters or rebuild an adapter snapshot.
    #[must_use]
    pub const fn consecutive_expected_failures(&self) -> u64 {
        self.observability.consecutive_expected_failures()
    }
}

#[cfg(test)]
mod tests;
