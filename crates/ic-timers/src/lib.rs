//! Observable and recovery-aware timer primitives for Internet Computer
//! canisters.
//!
//! `ic-timers` is a higher-level wrapper around `ic-cdk-timers`, which remains
//! the provider that arms and clears platform timers. This crate adds
//! bounded coordination, scheduling, and observation above it.
//!
//! The 0.3 runtime executes `Once`, after-completion, and pre-armed
//! synchronous watchdog callbacks through one volatile canister-local registry.
//! Consumers retain durable application authority and synchronously reconstruct
//! desired registrations during their existing lifecycle hooks.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]

mod control;
mod platform;
pub(crate) mod registry;
mod runtime;
pub mod schedule;
pub mod snapshot;

use control::{TimerControl, TimerControlAction, TimerControlError, TimerRegistration};
pub use registry::{MAX_TIMER_REGISTRATIONS, RegisterError};
pub use runtime::{
    AfterCompletionRegistration, OnceRegistration, TimerContext, TimerError, TimerFuture,
    TimerReconcileState, WatchdogRegistration, consecutive_expected_failures, initialize_runtime,
    reconcile_after_completion, reconcile_once, reconcile_watchdog, register_after_completion,
    register_once, register_watchdog, timer_snapshot, timer_snapshots,
};
pub use schedule::{ScheduleError, TimerCadence, TimerDirective, TimerSchedule};
pub use snapshot::{
    DeclarationLifetime, InactiveReason, MAX_TIMER_LABEL_BYTES, MeasurementSummary,
    OrdinaryRuntimeStateSnapshot, TimerCompletion, TimerCompletionOutcome, TimerControlFailure,
    TimerCounters, TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerIdentityError,
    TimerIdentityField, TimerLabel, TimerLabelError, TimerLastOutcome, TimerObservabilitySnapshot,
    TimerOutcomeSnapshot, TimerPerformance, TimerPolicy, TimerProcessCondition,
    TimerRegistrationStatus, TimerRunResult, TimerRuntimeStateSnapshot, TimerSchedulingMode,
    TimerSnapshot, WatchdogAttemptSnapshot, WatchdogAttemptStatus, WatchdogDecision,
    WatchdogRunResult, WatchdogRuntimeStateSnapshot,
};
