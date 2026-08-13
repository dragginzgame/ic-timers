//! Observable and recovery-aware timer primitives for Internet Computer
//! canisters.
//!
//! `ic-timers` is a higher-level wrapper around `ic-cdk-timers`, which remains
//! the provider that arms and clears platform timers. This crate adds
//! provider-neutral control, scheduling, and observation values above it.
//!
//! This initial scaffold exposes the deterministic control and platform
//! boundaries extracted from Canic. It does not yet provide the complete
//! registry or watchdog runtime described in the workspace architecture note.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod control;
pub mod platform;
pub mod schedule;
pub mod snapshot;

pub use control::{TimerControl, TimerControlAction, TimerControlError, TimerRegistration};
pub use platform::{TimerHandle, clear_timer, set_timer};
pub use schedule::{ScheduleError, TimerDirective};
pub use snapshot::{
    MAX_TIMER_LABEL_BYTES, MeasurementSummary, PreArmedSuccessor, TimerCompletion,
    TimerCompletionOutcome, TimerCounters, TimerDirectiveSnapshot, TimerEpoch, TimerIdentity,
    TimerIdentityError, TimerIdentityField, TimerLabel, TimerLabelError, TimerLastOutcome,
    TimerMeasurement, TimerObservabilitySnapshot, TimerOutcomeSnapshot, TimerPerformance,
    TimerPolicy, TimerProcessCondition, TimerRegistrationStatus, TimerSchedulingMode,
    TimerSchedulingSnapshot, TimerSnapshot, TimerStateSnapshot,
};
