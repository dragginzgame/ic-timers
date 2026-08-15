//! Observable and recovery-aware timer primitives for Internet Computer
//! canisters.
//!
//! `ic-timers` is a higher-level wrapper around `ic-cdk-timers`, which remains
//! the provider that arms and clears platform timers. This crate adds
//! bounded coordination, scheduling, and observation above it.
//!
//! The runtime executes `Once`, after-completion, and pre-armed synchronous
//! watchdog callbacks through one volatile canister-local registry.
//! Consumers retain durable application authority and synchronously reconstruct
//! retained registrations during their existing lifecycle hooks. Transient
//! remove-on-stop callbacks use direct registration. Registration capabilities
//! can observe exact armed provider-wakeup ownership without making snapshots
//! or observations into scheduling authority.
//! [`timer_inventory`] returns the epoch together with the complete ordered
//! timer set, including for an initialized empty registry.

#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
mod control;
mod platform;
mod registry;
mod runtime;
mod schedule;
mod snapshot;

pub use registry::{MAX_TIMER_REGISTRATIONS, RegisterError};
pub use runtime::{
    AfterCompletionContext, AfterCompletionRegistration, OnceContext, OnceRegistration, TimerError,
    TimerReconcileState, WatchdogContext, WatchdogRegistration, consecutive_expected_failures,
    initialize_runtime, reconcile_after_completion, reconcile_once, reconcile_watchdog,
    register_after_completion, register_once, register_watchdog, timer_inventory, timer_snapshot,
};
pub use schedule::{ScheduleError, TimerCadence, TimerDirective, TimerSchedule};
pub use snapshot::{
    DeclarationLifetime, InactiveReason, MAX_TIMER_IDENTITY_COMPONENT_BYTES, MeasurementSummary,
    MemoryPageExtent, MemoryPageSample, MemoryPageSummary, OrdinaryRuntimeStateSnapshot,
    TimerCompletion, TimerCompletionOutcome, TimerControlFailure, TimerCounters,
    TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerIdentityError, TimerIdentityField,
    TimerInventorySnapshot, TimerLastOutcome, TimerObservabilitySnapshot, TimerOutcomeSnapshot,
    TimerPerformance, TimerPolicy, TimerProcessCondition, TimerRegistrationStatus, TimerRunResult,
    TimerRuntimeStateSnapshot, TimerSchedulingMode, TimerSnapshot, WatchdogAttemptSnapshot,
    WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult, WatchdogRuntimeStateSnapshot,
};
