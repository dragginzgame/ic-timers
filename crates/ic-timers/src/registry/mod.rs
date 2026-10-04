//! Bounded canonical registry and provider-neutral policy transition engine.
//!
//! The registry emits provider-neutral effects. The runtime binds them to
//! linear provider handles while the registry remains the sole logical state
//! authority.

use crate::{
    control::{OrdinaryPending, TimerControl, TimerRegistration, WakeupArm},
    platform::{MemoryPages, TimerHandle},
    schedule::{
        DirectiveError, ResolvedSchedule, ScheduleError, TimerCadence, TimerDirective,
        TimerSchedule,
    },
    snapshot::{
        DeclarationLifetime, InactiveReason, MemoryPageExtent, MemoryPageSample,
        OrdinaryRuntimeStateSnapshot, TimerCompletion, TimerCompletionOutcome, TimerControlFailure,
        TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerInventorySnapshot,
        TimerObservabilitySnapshot, TimerPolicy, TimerRegistrationId, TimerRunResult,
        TimerRuntimeStateSnapshot, TimerSchedulingMode, TimerSnapshot, WatchdogAttemptSnapshot,
        WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult, WatchdogRuntimeStateSnapshot,
    },
};
use std::{cell::RefCell, collections::BTreeMap, future::Future, pin::Pin, rc::Rc};
use thiserror::Error;

/// Maximum declarations owned by one canonical registry.
pub const MAX_TIMER_REGISTRATIONS: usize = 64;

/// Opaque logical ownership claim returned by pure registration.
#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationClaim {
    identity: TimerIdentity,
    claim_generation: u64,
}

/// Internal callback role carried by a generation-checked dispatch token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackRole {
    OrdinaryWork,
    WatchdogScheduler,
    WatchdogWork,
}

/// Identity and generations an internal callback must present.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackToken {
    identity: TimerIdentity,
    claim_generation: u64,
    callback_generation: u64,
    role: CallbackRole,
}

impl CallbackToken {
    const fn new(
        identity: TimerIdentity,
        claim_generation: u64,
        callback_generation: u64,
        role: CallbackRole,
    ) -> Self {
        Self {
            identity,
            claim_generation,
            callback_generation,
            role,
        }
    }

    pub(crate) const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    #[cfg(test)]
    pub(crate) const fn callback_generation(&self) -> u64 {
        self.callback_generation
    }

    pub(crate) const fn role(&self) -> CallbackRole {
        self.role
    }

    pub(crate) fn belongs_to_same_claim(&self, other: &Self) -> bool {
        self.identity == other.identity && self.claim_generation == other.claim_generation
    }
}

impl RegistrationClaim {
    pub(crate) const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    pub(crate) const fn claim_generation(&self) -> u64 {
        self.claim_generation
    }

    pub(crate) fn from_callback(token: &CallbackToken) -> Self {
        Self {
            identity: token.identity.clone(),
            claim_generation: token.claim_generation,
        }
    }
}

/// Provider-neutral effect emitted by one pure transition.
/// Absolute deadlines remain in control state; arms carry resolved provider delays.
#[derive(Debug, Eq, PartialEq)]
pub enum RegistryEffect {
    None,
    ArmWakeup {
        token: CallbackToken,
        delay_ns: u64,
        arm: WakeupArm,
    },
    ClearCallbacks {
        identity: TimerIdentity,
        handles: CallbacksToClear,
    },
    DispatchWatchdog {
        successor: CallbackToken,
        successor_delay_ns: u64,
        work: CallbackToken,
    },
}

/// Non-empty subset of provider callbacks cleared by one effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbacksToClear {
    Wakeup,
    Work,
    WakeupAndWork,
}

impl CallbacksToClear {
    pub(crate) const fn includes_wakeup(self) -> bool {
        matches!(self, Self::Wakeup | Self::WakeupAndWork)
    }

    pub(crate) const fn includes_work(self) -> bool {
        matches!(self, Self::Work | Self::WakeupAndWork)
    }

    const fn wakeup_and_maybe_work(include_work: bool) -> Self {
        if include_work {
            Self::WakeupAndWork
        } else {
            Self::Wakeup
        }
    }
}

impl RegistryEffect {
    pub(crate) fn has_valid_shape(&self) -> bool {
        match self {
            Self::None | Self::ClearCallbacks { .. } => true,
            Self::ArmWakeup { token, .. } => matches!(
                token.role,
                CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler
            ),
            Self::DispatchWatchdog {
                successor, work, ..
            } => {
                successor.role == CallbackRole::WatchdogScheduler
                    && work.role == CallbackRole::WatchdogWork
                    && successor.belongs_to_same_claim(work)
            }
        }
    }
}

/// One pure transition and any terminal checked-control failure it produced.
#[derive(Debug, Eq, PartialEq)]
pub struct RegistryTransition {
    effect: RegistryEffect,
    failure: Option<TimerControlFailure>,
}

impl RegistryTransition {
    const fn normal(effect: RegistryEffect) -> Self {
        Self {
            effect,
            failure: None,
        }
    }

    const fn terminal(effect: RegistryEffect, failure: TimerControlFailure) -> Self {
        Self {
            effect,
            failure: Some(failure),
        }
    }

    pub(crate) const fn effect(&self) -> &RegistryEffect {
        &self.effect
    }

    pub(crate) const fn failure(&self) -> Option<TimerControlFailure> {
        self.failure
    }

    pub(crate) fn into_effect(self) -> RegistryEffect {
        self.effect
    }
}

/// Whether an internal callback generation won arbitration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallbackAcceptance {
    Accepted,
    Stale,
}

/// Failure to claim one canonical timer identity.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegisterError {
    /// The identity already has one canonical claimant.
    #[error("timer identity is already registered: {0:?}")]
    IdentityAlreadyRegistered(TimerIdentity),
    /// The fixed registry capacity has been reached.
    #[error("timer registry capacity of {max} declarations has been reached")]
    CapacityExceeded {
        /// Fixed registry capacity.
        max: usize,
    },
    /// Logical registration claim generations are exhausted.
    #[error("timer registration claim generation exhausted")]
    ClaimGenerationExhausted,
}

/// Invalid request against a logical registration claim.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegistryError {
    #[error("timer registration is no longer present")]
    UnknownRegistration,
    #[error("timer registration claim is stale")]
    StaleRegistration,
    #[error("timer policy invariant failed for {actual}")]
    PolicyMismatch { actual: &'static str },
    #[error("timer callback token is stale")]
    StaleCallback,
    #[error("timer registration already owns the provider handle for this callback role")]
    ProviderHandleAlreadyOwned,
    #[error(transparent)]
    Schedule(#[from] ScheduleError),
}

type OrdinaryFuture = Pin<Box<dyn Future<Output = TimerRunResult>>>;
pub type OrdinaryCallback = Rc<RefCell<Box<dyn FnMut(CallbackToken) -> OrdinaryFuture>>>;
pub type WatchdogCallback = Rc<RefCell<Box<dyn FnMut(CallbackToken) -> WatchdogRunResult>>>;

struct OwnedProviderHandle {
    callback_generation: u64,
    handle: TimerHandle,
}

/// One temporarily detached exact provider capability.
#[must_use = "restore or clear the detached provider handle"]
pub struct ProviderHandle {
    token: CallbackToken,
    handle: TimerHandle,
}

impl ProviderHandle {
    pub(crate) fn into_parts(self) -> (CallbackToken, TimerHandle) {
        (self.token, self.handle)
    }
}

/// The at-most-two provider capabilities owned by one timer entry.
#[must_use = "restore or clear every detached provider handle"]
#[derive(Default)]
pub struct ProviderHandles {
    wakeup: Option<ProviderHandle>,
    work: Option<ProviderHandle>,
}

impl ProviderHandles {
    pub(crate) const fn from_parts(
        wakeup: Option<ProviderHandle>,
        work: Option<ProviderHandle>,
    ) -> Self {
        Self { wakeup, work }
    }

    pub(crate) const fn take_wakeup(&mut self) -> Option<ProviderHandle> {
        self.wakeup.take()
    }

    pub(crate) const fn take_work(&mut self) -> Option<ProviderHandle> {
        self.work.take()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OrdinaryRequest {
    EnsureOnce,
    EnsureRecurring,
    Reconcile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogPending {
    Cancel,
    Ensure,
    EnsureImmediately,
    Reconcile(ResolvedSchedule),
    Unregister,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogScheduleRequest {
    Cadence,
    Immediate,
    Reconcile(ResolvedSchedule),
}

// One payload owns policy, callback type and control together. An ordinary
// entry without cadence is Once; one with cadence is AfterCompletion.
enum EntryKind {
    Ordinary {
        cadence: Option<TimerCadence>,
        callback: OrdinaryCallback,
        control: TimerControl,
    },
    Watchdog {
        cadence: TimerCadence,
        callback: WatchdogCallback,
        control: WatchdogControl,
    },
}

impl EntryKind {
    const fn wakeup_role(&self) -> CallbackRole {
        match self {
            Self::Ordinary { .. } => CallbackRole::OrdinaryWork,
            Self::Watchdog { .. } => CallbackRole::WatchdogScheduler,
        }
    }

    fn ordinary(cadence: Option<TimerCadence>, callback: OrdinaryCallback) -> Self {
        Self::Ordinary {
            cadence,
            callback,
            control: TimerControl::default(),
        }
    }

    fn watchdog(cadence: TimerCadence, callback: WatchdogCallback) -> Self {
        Self::Watchdog {
            cadence,
            callback,
            control: WatchdogControl::default(),
        }
    }

    const fn policy(&self) -> TimerPolicy {
        match self {
            Self::Ordinary { cadence: None, .. } => TimerPolicy::Once,
            Self::Ordinary {
                cadence: Some(cadence),
                ..
            } => TimerPolicy::AfterCompletion { cadence: *cadence },
            Self::Watchdog { cadence, .. } => TimerPolicy::Watchdog { cadence: *cadence },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogState {
    Inactive {
        reason: InactiveReason,
    },
    Scheduled {
        scheduler_generation: u64,
        deadline_ns: u64,
    },
    AwaitingWork {
        successor_generation: u64,
        successor_deadline_ns: u64,
        attempt_generation: u64,
        attempt_status: WatchdogAttemptStatus,
        pending: Option<WatchdogPending>,
    },
}

#[derive(Debug)]
struct WatchdogControl {
    scheduler_generation: u64,
    attempt_generation: u64,
    state: WatchdogState,
}

impl Default for WatchdogControl {
    fn default() -> Self {
        Self {
            scheduler_generation: 0,
            attempt_generation: 0,
            state: WatchdogState::Inactive {
                reason: InactiveReason::NeverScheduled,
            },
        }
    }
}

impl WatchdogControl {
    fn arm_scheduler(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        deadline_ns: u64,
        arm: WakeupArm,
    ) -> RegistryTransition {
        let Some(generation) = self.scheduler_generation.checked_add(1) else {
            let cleanup = if arm.replaces_existing() {
                clear_callbacks(claim.identity.clone(), CallbacksToClear::Wakeup)
            } else {
                RegistryEffect::None
            };
            return self.terminate(cleanup, TimerControlFailure::GenerationExhausted);
        };
        self.scheduler_generation = generation;
        self.state = WatchdogState::Scheduled {
            scheduler_generation: generation,
            deadline_ns,
        };
        RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: token_for(claim, generation, CallbackRole::WatchdogScheduler),
            delay_ns: deadline_ns.saturating_sub(now_ns),
            arm,
        })
    }

    const fn next_dispatch_generations(&self) -> Option<(u64, u64)> {
        let Some(scheduler_generation) = self.scheduler_generation.checked_add(1) else {
            return None;
        };
        let Some(attempt_generation) = self.attempt_generation.checked_add(1) else {
            return None;
        };
        Some((scheduler_generation, attempt_generation))
    }

    const fn terminate(
        &mut self,
        effect: RegistryEffect,
        failure: TimerControlFailure,
    ) -> RegistryTransition {
        self.state = WatchdogState::Inactive {
            reason: InactiveReason::ControlFailure(failure),
        };
        RegistryTransition::terminal(effect, failure)
    }
}

struct Entry {
    claim_generation: u64,
    lifetime: DeclarationLifetime,
    kind: EntryKind,
    scheduling_mode: TimerSchedulingMode,
    latest_directive: Option<TimerDirectiveSnapshot>,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    confirmed_wakeup_generation: Option<u64>,
    wakeup: Option<OwnedProviderHandle>,
    work: Option<OwnedProviderHandle>,
    observability: TimerObservabilitySnapshot,
}

impl Entry {
    const fn new(
        claim_generation: u64,
        kind: EntryKind,
        lifetime: DeclarationLifetime,
        epoch: TimerEpoch,
    ) -> Self {
        let scheduling_mode = match kind.policy() {
            TimerPolicy::Once => TimerSchedulingMode::Once,
            TimerPolicy::AfterCompletion { .. } => TimerSchedulingMode::AfterCompletion,
            TimerPolicy::Watchdog { .. } => TimerSchedulingMode::Watchdog,
        };

        Self {
            claim_generation,
            lifetime,
            kind,
            scheduling_mode,
            latest_directive: None,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            confirmed_wakeup_generation: None,
            wakeup: None,
            work: None,
            observability: TimerObservabilitySnapshot::new(epoch),
        }
    }

    const fn snapshot(&self, identity: TimerIdentity) -> TimerSnapshot {
        let state = match &self.kind {
            EntryKind::Ordinary { control, .. } => match control.registration {
                TimerRegistration::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                } => TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled {
                    generation,
                    deadline_ns,
                }),
                TimerRegistration::Running { generation, .. } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Running {
                        generation,
                    })
                }
            },
            EntryKind::Watchdog { control, .. } => match control.state {
                WatchdogState::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                WatchdogState::Scheduled {
                    scheduler_generation,
                    deadline_ns,
                } => TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                    scheduler_generation,
                    deadline_ns,
                }),
                WatchdogState::AwaitingWork {
                    successor_generation,
                    successor_deadline_ns,
                    attempt_generation,
                    attempt_status,
                    ..
                } => TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork {
                        successor_generation,
                        successor_deadline_ns,
                        attempt: WatchdogAttemptSnapshot::new(attempt_generation, attempt_status),
                    },
                ),
            },
        };

        TimerSnapshot::new(
            identity,
            TimerRegistrationId::new(self.observability.epoch(), self.claim_generation),
            self.kind.policy(),
            self.lifetime,
            state,
            self.scheduling_mode,
            self.latest_directive,
            self.latest_requested_delay_ns,
            self.latest_armed_delay_ns,
            &self.observability,
        )
    }

    fn take_wakeup_handle(&mut self, identity: &TimerIdentity) -> Option<ProviderHandle> {
        let role = self.kind.wakeup_role();
        self.wakeup
            .take()
            .map(|owned| detach_provider_handle(identity, self.claim_generation, role, owned))
    }

    fn take_work_handle(&mut self, identity: &TimerIdentity) -> Option<ProviderHandle> {
        self.work.take().map(|owned| {
            detach_provider_handle(
                identity,
                self.claim_generation,
                CallbackRole::WatchdogWork,
                owned,
            )
        })
    }

    fn take_provider_handles(&mut self, identity: &TimerIdentity) -> ProviderHandles {
        ProviderHandles {
            wakeup: self.take_wakeup_handle(identity),
            work: self.take_work_handle(identity),
        }
    }

    fn provider_slot_mut(
        &mut self,
        role: CallbackRole,
    ) -> Option<&mut Option<OwnedProviderHandle>> {
        match role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler
                if role == self.kind.wakeup_role() =>
            {
                Some(&mut self.wakeup)
            }
            CallbackRole::WatchdogWork if matches!(self.kind, EntryKind::Watchdog { .. }) => {
                Some(&mut self.work)
            }
            _ => None,
        }
    }

    const fn owns_token_claim(&self, token: &CallbackToken) -> bool {
        self.claim_generation == token.claim_generation
    }

    // Registry lookups select this entry by token identity first.
    const fn owns_running_work(&self, token: &CallbackToken) -> bool {
        if !self.owns_token_claim(token) {
            return false;
        }
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration,
                TimerRegistration::Running { generation, .. }
                    if generation == token.callback_generation
            ),
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork) => matches!(
                control.state,
                WatchdogState::AwaitingWork {
                    attempt_generation,
                    attempt_status: WatchdogAttemptStatus::Running,
                    ..
                } if attempt_generation == token.callback_generation
            ),
            (
                EntryKind::Ordinary { .. },
                CallbackRole::WatchdogScheduler | CallbackRole::WatchdogWork,
            )
            | (
                EntryKind::Watchdog { .. },
                CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler,
            ) => false,
        }
    }
}

/// Pure, fixed-capacity canonical registry.
pub struct TimerRegistry {
    epoch: TimerEpoch,
    next_claim_generation: u64,
    entries: BTreeMap<TimerIdentity, Entry>,
}

impl TimerRegistry {
    pub(crate) const fn new(epoch: TimerEpoch) -> Self {
        Self {
            epoch,
            next_claim_generation: 0,
            entries: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) const fn epoch(&self) -> TimerEpoch {
        self.epoch
    }

    #[cfg(test)]
    pub(crate) fn register_once(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_once_with_callback(identity, lifetime, tests::ordinary_callback_fixture())
    }

    #[cfg(test)]
    pub(crate) fn register_after_completion(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_after_completion_with_callback(
            identity,
            cadence,
            lifetime,
            tests::ordinary_callback_fixture(),
        )
    }

    #[cfg(test)]
    pub(crate) fn register_watchdog(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register_watchdog_with_callback(
            identity,
            cadence,
            lifetime,
            tests::watchdog_callback_fixture(),
        )
    }

    pub(crate) fn register_once_with_callback(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
        callback: OrdinaryCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(identity, EntryKind::ordinary(None, callback), lifetime)
    }

    pub(crate) fn register_after_completion_with_callback(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
        callback: OrdinaryCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            EntryKind::ordinary(Some(cadence), callback),
            lifetime,
        )
    }

    pub(crate) fn register_watchdog_with_callback(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
        callback: WatchdogCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(identity, EntryKind::watchdog(cadence, callback), lifetime)
    }

    fn register(
        &mut self,
        identity: TimerIdentity,
        kind: EntryKind,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        if self.entries.contains_key(&identity) {
            return Err(RegisterError::IdentityAlreadyRegistered(identity));
        }
        if self.entries.len() == MAX_TIMER_REGISTRATIONS {
            return Err(RegisterError::CapacityExceeded {
                max: MAX_TIMER_REGISTRATIONS,
            });
        }
        let claim_generation = self
            .next_claim_generation
            .checked_add(1)
            .ok_or(RegisterError::ClaimGenerationExhausted)?;

        self.next_claim_generation = claim_generation;
        self.entries.insert(
            identity.clone(),
            Entry::new(claim_generation, kind, lifetime, self.epoch),
        );
        Ok(RegistrationClaim {
            identity,
            claim_generation,
        })
    }

    pub(crate) fn ensure_once(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: TimerSchedule,
    ) -> Result<RegistryTransition, RegistryError> {
        self.request_ordinary(
            claim,
            now_ns,
            schedule.resolve(now_ns)?,
            OrdinaryRequest::EnsureOnce,
        )
    }

    /// Reconcile an ordinary declaration to one exact desired schedule.
    ///
    /// Unlike `ensure`, reconciliation may move an existing deadline later.
    /// `None` cancels live work; declaration lifetime determines whether the
    /// callback authority remains registered.
    pub(crate) fn reconcile_ordinary(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: Option<TimerSchedule>,
    ) -> Result<RegistryTransition, RegistryError> {
        self.validate_ordinary_claim(claim)?;
        let Some(schedule) = schedule else {
            return self.cancel(claim);
        };
        self.request_ordinary(
            claim,
            now_ns,
            schedule.resolve(now_ns)?,
            OrdinaryRequest::Reconcile,
        )
    }

    pub(crate) fn validate_ordinary_claim(
        &self,
        claim: &RegistrationClaim,
    ) -> Result<(), RegistryError> {
        let entry = self.entry(claim)?;
        if matches!(entry.kind, EntryKind::Ordinary { .. }) {
            Ok(())
        } else {
            Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            })
        }
    }

    pub(crate) fn ensure_recurring(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        match &entry.kind {
            EntryKind::Ordinary { cadence: None, .. } => {
                Err(RegistryError::PolicyMismatch { actual: "once" })
            }
            EntryKind::Ordinary {
                cadence: Some(cadence),
                control,
                ..
            } => {
                let cadence = *cadence;
                // Existing authority satisfies recurrence without calculating
                // an unused successor, even when now + cadence would overflow.
                let deadline_ns = match control.registration {
                    TimerRegistration::Scheduled { deadline_ns, .. } => deadline_ns,
                    TimerRegistration::Inactive { .. } | TimerRegistration::Running { .. } => {
                        cadence.deadline_after(now_ns)?
                    }
                };
                self.request_ordinary(
                    claim,
                    now_ns,
                    ResolvedSchedule {
                        deadline_ns,
                        requested_delay_ns: Some(cadence.as_nanos()),
                        mode: TimerSchedulingMode::AfterCompletion,
                    },
                    OrdinaryRequest::EnsureRecurring,
                )
            }
            EntryKind::Watchdog { .. } => {
                self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Cadence)
            }
        }
    }

    pub(crate) fn ensure_watchdog_immediately(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
    ) -> Result<RegistryTransition, RegistryError> {
        self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Immediate)
    }

    pub(crate) fn reconcile_watchdog_schedule(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        schedule: Option<TimerSchedule>,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        if !matches!(entry.kind, EntryKind::Watchdog { .. }) {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        }
        let Some(schedule) = schedule else {
            return self.cancel(claim);
        };
        let requested = schedule.resolve(now_ns)?;
        self.ensure_watchdog(claim, now_ns, WatchdogScheduleRequest::Reconcile(requested))
    }

    fn request_ordinary(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        requested: ResolvedSchedule,
        request: OrdinaryRequest,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        let actual = entry.kind.policy().label();
        let EntryKind::Ordinary {
            cadence, control, ..
        } = &mut entry.kind
        else {
            return Err(RegistryError::PolicyMismatch { actual });
        };
        let policy_matches = match request {
            OrdinaryRequest::EnsureOnce => cadence.is_none(),
            OrdinaryRequest::EnsureRecurring => cadence.is_some(),
            OrdinaryRequest::Reconcile => true,
        };
        if !policy_matches {
            return Err(RegistryError::PolicyMismatch { actual });
        }
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested.requested_delay_ns;

        let arm = match request {
            OrdinaryRequest::EnsureOnce | OrdinaryRequest::EnsureRecurring => {
                control.schedule(requested.deadline_ns)
            }
            OrdinaryRequest::Reconcile => control.reconcile(requested.deadline_ns),
        };

        let arm = match arm {
            Ok(arm) => arm,
            Err(error) => {
                let transition = terminal_ordinary(entry, claim.identity.clone(), error);
                return Ok(self.remove_transient_on_failure(claim.identity(), transition));
            }
        };

        if let TimerRegistration::Running { pending, .. } = &mut control.registration {
            *pending = Some(select_pending_ordinary(*pending, request, requested));
        }
        if matches!(request, OrdinaryRequest::Reconcile) {
            entry.scheduling_mode = requested.mode;
        }

        let Some(arm) = arm else {
            entry.observability.counters_mut().record_coalesced();
            return Ok(RegistryTransition::normal(RegistryEffect::None));
        };
        entry.scheduling_mode = requested.mode;
        Ok(RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: token_for(claim, control.generation(), CallbackRole::OrdinaryWork),
            delay_ns: requested.deadline_ns.saturating_sub(now_ns),
            arm,
        }))
    }

    fn ensure_watchdog(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        request: WatchdogScheduleRequest,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        let actual = entry.kind.policy().label();
        let EntryKind::Watchdog {
            cadence, control, ..
        } = &mut entry.kind
        else {
            return Err(RegistryError::PolicyMismatch { actual });
        };
        let cadence = *cadence;
        let requested_delay_ns = match request {
            WatchdogScheduleRequest::Cadence => Some(cadence.as_nanos()),
            WatchdogScheduleRequest::Immediate => Some(0),
            WatchdogScheduleRequest::Reconcile(requested) => requested.requested_delay_ns,
        };
        let transition = match &mut control.state {
            WatchdogState::Inactive { .. } => {
                let deadline_ns = match request {
                    WatchdogScheduleRequest::Cadence => cadence.deadline_after(now_ns)?,
                    WatchdogScheduleRequest::Immediate => now_ns,
                    WatchdogScheduleRequest::Reconcile(requested) => requested.deadline_ns,
                };
                let transition =
                    control.arm_scheduler(claim, now_ns, deadline_ns, WakeupArm::Initial);
                if transition.failure().is_none() {
                    entry.scheduling_mode = match request {
                        WatchdogScheduleRequest::Cadence => TimerSchedulingMode::Watchdog,
                        WatchdogScheduleRequest::Immediate => TimerSchedulingMode::Continuation,
                        WatchdogScheduleRequest::Reconcile(requested) => requested.mode,
                    };
                }
                transition
            }
            WatchdogState::Scheduled { deadline_ns, .. }
                if match request {
                    WatchdogScheduleRequest::Immediate => *deadline_ns > now_ns,
                    WatchdogScheduleRequest::Reconcile(requested) => {
                        *deadline_ns != requested.deadline_ns
                    }
                    WatchdogScheduleRequest::Cadence => false,
                } =>
            {
                let (deadline_ns, mode) = match request {
                    WatchdogScheduleRequest::Reconcile(requested) => {
                        (requested.deadline_ns, requested.mode)
                    }
                    WatchdogScheduleRequest::Immediate | WatchdogScheduleRequest::Cadence => {
                        (now_ns, TimerSchedulingMode::Continuation)
                    }
                };
                let transition =
                    control.arm_scheduler(claim, now_ns, deadline_ns, WakeupArm::Replacement);
                if transition.failure().is_none() {
                    entry.scheduling_mode = mode;
                }
                transition
            }
            WatchdogState::Scheduled { .. } => {
                entry.observability.counters_mut().record_coalesced();
                RegistryTransition::normal(RegistryEffect::None)
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                pending,
                ..
            } => {
                if let WatchdogScheduleRequest::Reconcile(requested) = request {
                    *pending = Some(WatchdogPending::Reconcile(requested));
                }
                entry.observability.counters_mut().record_coalesced();
                RegistryTransition::normal(RegistryEffect::None)
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                pending,
                ..
            } => {
                *pending = Some(select_pending_watchdog(*pending, request, now_ns));
                entry.observability.counters_mut().record_coalesced();
                RegistryTransition::normal(RegistryEffect::None)
            }
        };
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested_delay_ns;
        Ok(self.remove_transient_on_failure(claim.identity(), transition))
    }

    pub(crate) fn cancel(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity.clone();
        let (transition, remove) = {
            let entry = self.entry_mut(claim)?;
            match &mut entry.kind {
                EntryKind::Ordinary { control, .. } => {
                    let before = control.registration;
                    if let Err(error) = control.cancel() {
                        let transition = terminal_ordinary(entry, identity.clone(), error);
                        return Ok(self.remove_transient_on_failure(&identity, transition));
                    }
                    let remove = !matches!(before, TimerRegistration::Running { .. })
                        && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                    let transition = if let TimerRegistration::Running { pending, .. } =
                        &mut control.registration
                    {
                        if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                            *pending = Some(OrdinaryPending::Cancel);
                        }
                        RegistryTransition::normal(RegistryEffect::None)
                    } else {
                        let clear_wakeup = matches!(before, TimerRegistration::Scheduled { .. });
                        if clear_wakeup {
                            entry.observability.counters_mut().record_cancellation();
                        }
                        RegistryTransition::normal(clear_wakeup_if(identity.clone(), clear_wakeup))
                    };
                    (transition, remove)
                }
                EntryKind::Watchdog { control, .. } => {
                    let cancels_immediately = matches!(
                        control.state,
                        WatchdogState::Scheduled { .. }
                            | WatchdogState::AwaitingWork {
                                attempt_status: WatchdogAttemptStatus::Dispatched,
                                ..
                            }
                    );
                    let transition = cancel_watchdog(control, &identity);
                    if cancels_immediately && transition.failure().is_none() {
                        entry.observability.counters_mut().record_cancellation();
                    }
                    let stopped = matches!(control.state, WatchdogState::Inactive { .. });
                    (
                        transition,
                        stopped && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
                    )
                }
            }
        };

        Ok(remove_after(
            &mut self.entries,
            &identity,
            transition,
            remove,
        ))
    }

    /// Remove the declaration owned by one exact logical claim.
    ///
    /// Removal requested by running work is finalized by that work's normal
    /// completion. A trapping work message rolls the request back with the
    /// rest of its heap mutations.
    pub(crate) fn unregister(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity.clone();
        let entry = self.entry_mut(claim)?;
        match &mut entry.kind {
            EntryKind::Ordinary {
                control:
                    TimerControl {
                        registration: TimerRegistration::Running { pending, .. },
                        ..
                    },
                ..
            } => {
                *pending = Some(OrdinaryPending::Unregister);
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            EntryKind::Watchdog {
                control:
                    WatchdogControl {
                        state:
                            WatchdogState::AwaitingWork {
                                attempt_status: WatchdogAttemptStatus::Running,
                                pending,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                *pending = Some(WatchdogPending::Unregister);
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            EntryKind::Ordinary { .. } | EntryKind::Watchdog { .. } => {
                let transition = self.cancel(claim)?;
                self.entries.remove(&identity);
                Ok(transition)
            }
        }
    }

    pub(crate) fn begin_ordinary(&mut self, token: &CallbackToken) -> CallbackAcceptance {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return CallbackAcceptance::Stale;
        };
        if token.role != CallbackRole::OrdinaryWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_wakeup();
            return CallbackAcceptance::Stale;
        }
        let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
            entry.observability.counters_mut().record_stale_wakeup();
            return CallbackAcceptance::Stale;
        };
        if control.begin(token.callback_generation) {
            entry.observability.counters_mut().record_work_started();
            CallbackAcceptance::Accepted
        } else {
            entry.observability.counters_mut().record_stale_wakeup();
            CallbackAcceptance::Stale
        }
    }

    pub(crate) fn complete_ordinary(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: TimerRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity.clone();
        let (transition, remove) = {
            let entry = self.running_work_entry_mut(token)?;
            let EntryKind::Ordinary {
                cadence, control, ..
            } = &mut entry.kind
            else {
                return Err(RegistryError::StaleCallback);
            };
            let cadence = *cadence;
            let completion = result.completion();
            let pending_command = control.pending();
            let remove_on_stop = matches!(pending_command, Some(OrdinaryPending::Unregister))
                || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
            let terminal_pending = matches!(
                pending_command,
                Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister)
            );
            // Arbitrate authoritative commands before resolving a discarded
            // callback proposal. Explicit consumer invariant failure still wins.
            let effective_directive = match pending_command {
                Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => TimerDirective::Stop,
                Some(OrdinaryPending::Reconcile(requested)) => {
                    TimerDirective::ScheduleAt(requested.deadline_ns)
                }
                Some(OrdinaryPending::Schedule(_)) | None => result.directive(),
            };
            if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                let transition = stop_ordinary_completion(entry, completion, now_ns, None);
                (transition, remove_on_stop)
            } else {
                let resolved = match effective_directive.resolve(now_ns, cadence) {
                    Ok(value) => value,
                    Err(error) => {
                        let transition = stop_ordinary_completion(
                            entry,
                            completion,
                            now_ns,
                            Some(map_directive_failure(error)),
                        );
                        return Ok(remove_after(
                            &mut self.entries,
                            &identity,
                            transition,
                            remove_on_stop,
                        ));
                    }
                };
                let directive_snapshot = TimerDirectiveSnapshot::try_from(effective_directive)
                    .map_err(RegistryError::Schedule)?;
                let selected_schedule = select_completion_schedule(pending_command, resolved);
                if let Some(failure) = selected_schedule
                    .and_then(|selected| control.arm_deadline(selected.deadline_ns).err())
                {
                    let transition =
                        stop_ordinary_completion(entry, completion, now_ns, Some(failure));
                    return Ok(remove_after(
                        &mut self.entries,
                        &identity,
                        transition,
                        remove_on_stop,
                    ));
                }
                entry.latest_directive = Some(directive_snapshot);
                entry.observability.record_completion(completion, now_ns);

                let remove = selected_schedule.is_none() && remove_on_stop;
                let transition = if let Some(selected) = selected_schedule {
                    entry.scheduling_mode = selected.mode;
                    entry.latest_requested_delay_ns = selected.requested_delay_ns;
                    RegistryTransition::normal(RegistryEffect::ArmWakeup {
                        token: CallbackToken::new(
                            identity.clone(),
                            entry.claim_generation,
                            control.generation(),
                            CallbackRole::OrdinaryWork,
                        ),
                        delay_ns: selected.deadline_ns.saturating_sub(now_ns),
                        arm: WakeupArm::Initial,
                    })
                } else {
                    let reason = if terminal_pending {
                        entry.observability.counters_mut().record_cancellation();
                        InactiveReason::Cancelled
                    } else {
                        InactiveReason::Stopped
                    };
                    control.terminate(reason);
                    RegistryTransition::normal(RegistryEffect::None)
                };
                (transition, remove)
            }
        };

        Ok(remove_after(
            &mut self.entries,
            &identity,
            transition,
            remove,
        ))
    }

    pub(crate) fn begin_watchdog_scheduler(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
    ) -> RegistryTransition {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return RegistryTransition::normal(RegistryEffect::None);
        };
        if token.role != CallbackRole::WatchdogScheduler || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        }
        let EntryKind::Watchdog {
            cadence, control, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        };
        let cadence = *cadence;

        let accepted = match control.state {
            WatchdogState::Scheduled {
                scheduler_generation,
                ..
            } => scheduler_generation == token.callback_generation,
            WatchdogState::AwaitingWork {
                successor_generation,
                attempt_status: WatchdogAttemptStatus::Dispatched,
                ..
            } => successor_generation == token.callback_generation,
            WatchdogState::Inactive { .. }
            | WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                ..
            } => false,
        };
        if !accepted {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        }

        entry
            .observability
            .counters_mut()
            .record_scheduler_started();
        if matches!(control.state, WatchdogState::AwaitingWork { .. }) {
            entry.observability.record_unacknowledged(now_ns);
        }
        let Some((successor_generation, attempt_generation)) = control.next_dispatch_generations()
        else {
            let transition = control.terminate(
                clear_callbacks(token.identity.clone(), CallbacksToClear::Work),
                TimerControlFailure::GenerationExhausted,
            );
            return self.remove_transient_on_failure(token.identity(), transition);
        };
        let Ok(successor_deadline_ns) = cadence.deadline_after(now_ns) else {
            let transition = control.terminate(
                clear_callbacks(token.identity.clone(), CallbacksToClear::Work),
                TimerControlFailure::DeadlineOverflow,
            );
            return self.remove_transient_on_failure(token.identity(), transition);
        };

        control.scheduler_generation = successor_generation;
        control.attempt_generation = attempt_generation;
        control.state = WatchdogState::AwaitingWork {
            successor_generation,
            successor_deadline_ns,
            attempt_generation,
            attempt_status: WatchdogAttemptStatus::Dispatched,
            pending: None,
        };
        entry.scheduling_mode = TimerSchedulingMode::Watchdog;
        RegistryTransition::normal(RegistryEffect::DispatchWatchdog {
            successor: CallbackToken::new(
                token.identity.clone(),
                entry.claim_generation,
                successor_generation,
                CallbackRole::WatchdogScheduler,
            ),
            successor_delay_ns: cadence.as_nanos(),
            work: CallbackToken::new(
                token.identity.clone(),
                entry.claim_generation,
                attempt_generation,
                CallbackRole::WatchdogWork,
            ),
        })
    }

    pub(crate) fn begin_watchdog_work(&mut self, token: &CallbackToken) -> CallbackAcceptance {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return CallbackAcceptance::Stale;
        };
        if token.role != CallbackRole::WatchdogWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_work();
            return CallbackAcceptance::Stale;
        }
        let EntryKind::Watchdog { control, .. } = &mut entry.kind else {
            entry.observability.counters_mut().record_stale_work();
            return CallbackAcceptance::Stale;
        };
        match &mut control.state {
            WatchdogState::AwaitingWork {
                attempt_generation,
                attempt_status,
                ..
            } if *attempt_generation == token.callback_generation
                && *attempt_status == WatchdogAttemptStatus::Dispatched =>
            {
                *attempt_status = WatchdogAttemptStatus::Running;
                entry.observability.counters_mut().record_work_started();
                CallbackAcceptance::Accepted
            }
            WatchdogState::Inactive { .. }
            | WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork { .. } => {
                entry.observability.counters_mut().record_stale_work();
                CallbackAcceptance::Stale
            }
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "One atomic successor and request-arbitration transition."
    )]
    pub(crate) fn complete_watchdog_work(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: WatchdogRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity.clone();
        let (transition, remove) = {
            let entry = self.running_work_entry_mut(token)?;
            let EntryKind::Watchdog {
                cadence, control, ..
            } = &mut entry.kind
            else {
                return Err(RegistryError::StaleCallback);
            };
            let cadence = *cadence;
            let WatchdogState::AwaitingWork {
                successor_generation,
                successor_deadline_ns,
                pending,
                ..
            } = control.state
            else {
                return Err(RegistryError::StaleCallback);
            };

            let completion = result.completion();
            entry.observability.record_completion(completion, now_ns);
            let decision = if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                WatchdogDecision::Stop
            } else {
                match pending {
                    Some(WatchdogPending::Cancel | WatchdogPending::Unregister) => {
                        WatchdogDecision::Stop
                    }
                    Some(WatchdogPending::Ensure) => WatchdogDecision::Continue,
                    Some(WatchdogPending::EnsureImmediately) => {
                        WatchdogDecision::ContinueImmediately
                    }
                    Some(WatchdogPending::Reconcile(requested)) => {
                        WatchdogDecision::ScheduleAt(requested.deadline_ns)
                    }
                    None => result.decision(),
                }
            };
            let cancelled = matches!(pending, Some(WatchdogPending::Cancel));
            let unregister = matches!(pending, Some(WatchdogPending::Unregister));
            let pending_schedule = match pending {
                Some(WatchdogPending::Reconcile(requested)) => Some(requested),
                _ => None,
            };

            match decision {
                WatchdogDecision::Continue => {
                    control.state = WatchdogState::Scheduled {
                        scheduler_generation: successor_generation,
                        deadline_ns: successor_deadline_ns,
                    };
                    entry.scheduling_mode = TimerSchedulingMode::Watchdog;
                    entry.latest_requested_delay_ns = Some(cadence.as_nanos());
                    (RegistryTransition::normal(RegistryEffect::None), false)
                }
                WatchdogDecision::ContinueImmediately | WatchdogDecision::ScheduleAt(_) => {
                    let (deadline_ns, retain_successor) =
                        if let WatchdogDecision::ScheduleAt(deadline_ns) = decision {
                            entry.scheduling_mode = pending_schedule
                                .map_or(TimerSchedulingMode::Deadline, |requested| requested.mode);
                            entry.latest_requested_delay_ns =
                                pending_schedule.and_then(|requested| requested.requested_delay_ns);
                            (deadline_ns, successor_deadline_ns == deadline_ns)
                        } else {
                            entry.scheduling_mode = TimerSchedulingMode::Continuation;
                            entry.latest_requested_delay_ns = Some(0);
                            (now_ns, successor_deadline_ns <= now_ns)
                        };
                    if retain_successor {
                        control.state = WatchdogState::Scheduled {
                            scheduler_generation: successor_generation,
                            deadline_ns: successor_deadline_ns,
                        };
                        (RegistryTransition::normal(RegistryEffect::None), false)
                    } else {
                        let claim = RegistrationClaim::from_callback(token);
                        let transition = control.arm_scheduler(
                            &claim,
                            now_ns,
                            deadline_ns,
                            WakeupArm::Replacement,
                        );
                        let remove = transition.failure().is_some()
                            && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                        (transition, remove)
                    }
                }
                WatchdogDecision::Stop => {
                    let reason = if completion.outcome() == TimerCompletionOutcome::InvariantFailure
                    {
                        InactiveReason::InvariantFailure
                    } else if cancelled {
                        entry.observability.counters_mut().record_cancellation();
                        InactiveReason::Cancelled
                    } else {
                        InactiveReason::Stopped
                    };
                    control.state = WatchdogState::Inactive { reason };
                    (
                        RegistryTransition::normal(clear_callbacks(
                            identity.clone(),
                            CallbacksToClear::Wakeup,
                        )),
                        unregister
                            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
                    )
                }
            }
        };

        Ok(remove_after(
            &mut self.entries,
            &identity,
            transition,
            remove,
        ))
    }

    /// Remove a terminal transient whose provider handles the runtime detached
    /// before invoking the policy transition.
    fn remove_transient_on_failure(
        &mut self,
        identity: &TimerIdentity,
        transition: RegistryTransition,
    ) -> RegistryTransition {
        let remove = transition.failure().is_some()
            && self.entries.get(identity).is_some_and(|entry| {
                matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped)
            });
        remove_after(&mut self.entries, identity, transition, remove)
    }

    pub(crate) fn snapshot(&self, identity: &TimerIdentity) -> Option<TimerSnapshot> {
        self.entries
            .get(identity)
            .map(|entry| entry.snapshot(identity.clone()))
    }

    pub(crate) fn inventory(&self) -> TimerInventorySnapshot {
        TimerInventorySnapshot::new(
            self.epoch,
            self.entries
                .iter()
                .map(|(identity, entry)| entry.snapshot(identity.clone()))
                .collect(),
        )
    }

    pub(crate) fn consecutive_expected_failures(&self, identity: &TimerIdentity) -> Option<u64> {
        self.entries.get(identity).map(|entry| {
            entry
                .observability
                .outcomes()
                .consecutive_expected_failures()
        })
    }

    pub(crate) fn has_armed_wakeup(
        &self,
        claim: &RegistrationClaim,
    ) -> Result<bool, RegistryError> {
        Ok(self.entry(claim)?.wakeup.is_some())
    }

    pub(crate) fn declaration_matches(
        &self,
        claim: &RegistrationClaim,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
    ) -> Result<bool, RegistryError> {
        let entry = self.entry(claim)?;
        Ok(entry.kind.policy() == policy && entry.lifetime == lifetime)
    }

    pub(crate) fn record_callback_measurements(
        &mut self,
        token: &CallbackToken,
        instructions: u64,
        memory_start: MemoryPages,
        memory_end: MemoryPages,
    ) -> Result<(), RegistryError> {
        // A normal remove-on-stop completion can delete its entry before the
        // post-run measurement is committed, leaving nothing to observe.
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return Ok(());
        };
        // Identity reuse must not let a late callback write into a newer
        // registration's observations.
        if !entry.owns_token_claim(token) {
            return Ok(());
        }

        let memory = memory_sample(memory_start, memory_end);
        match (&entry.kind, token.role) {
            (EntryKind::Watchdog { .. }, CallbackRole::WatchdogScheduler) => entry
                .observability
                .record_scheduler_measurements(instructions, memory),
            (EntryKind::Ordinary { .. }, CallbackRole::OrdinaryWork)
            | (EntryKind::Watchdog { .. }, CallbackRole::WatchdogWork) => entry
                .observability
                .record_work_measurements(instructions, memory),
            _ => {
                return Err(RegistryError::PolicyMismatch {
                    actual: entry.kind.policy().label(),
                });
            }
        }
        Ok(())
    }

    pub(crate) fn fail_registration(
        &mut self,
        claim: &RegistrationClaim,
        failure: TimerControlFailure,
    ) -> Result<ProviderHandles, RegistryError> {
        let identity = claim.identity.clone();
        let (handles, remove) = {
            let entry = self.entry_mut(claim)?;
            match &mut entry.kind {
                EntryKind::Ordinary { control, .. } => {
                    control.terminate(InactiveReason::ControlFailure(failure));
                }
                EntryKind::Watchdog { control, .. } => {
                    control.state = WatchdogState::Inactive {
                        reason: InactiveReason::ControlFailure(failure),
                    };
                }
            }
            let handles = entry.take_provider_handles(&identity);
            (
                handles,
                matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
            )
        };
        if remove {
            self.entries.remove(&identity);
        }
        Ok(handles)
    }

    pub(crate) fn ordinary_callback(
        &self,
        token: &CallbackToken,
    ) -> Result<OrdinaryCallback, RegistryError> {
        let entry = self.running_work_entry(token)?;
        match &entry.kind {
            EntryKind::Ordinary { callback, .. } => Ok(Rc::clone(callback)),
            EntryKind::Watchdog { .. } => Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            }),
        }
    }

    pub(crate) fn watchdog_callback(
        &self,
        token: &CallbackToken,
    ) -> Result<WatchdogCallback, RegistryError> {
        let entry = self.running_work_entry(token)?;
        match &entry.kind {
            EntryKind::Watchdog { callback, .. } => Ok(Rc::clone(callback)),
            EntryKind::Ordinary { .. } => Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            }),
        }
    }

    pub(crate) fn validate_running_context(
        &self,
        token: &CallbackToken,
    ) -> Result<(), RegistryError> {
        self.running_work_entry(token).map(|_| ())
    }

    pub(crate) fn install_provider_handle(
        &mut self,
        token: &CallbackToken,
        handle: TimerHandle,
    ) -> Result<(), (RegistryError, TimerHandle)> {
        let entry = match self.entry_by_token_mut(token) {
            Ok(entry) => entry,
            Err(error) => return Err((error, handle)),
        };
        let valid = match (&entry.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration,
                TimerRegistration::Scheduled { generation, .. }
                    if generation == token.callback_generation
            ),
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler) => {
                matches!(
                    control.state,
                    WatchdogState::Scheduled {
                        scheduler_generation,
                        ..
                    } if scheduler_generation == token.callback_generation
                ) || matches!(
                    control.state,
                    WatchdogState::AwaitingWork {
                        successor_generation,
                        ..
                    } if successor_generation == token.callback_generation
                )
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork) => matches!(
                control.state,
                WatchdogState::AwaitingWork {
                    attempt_generation,
                    attempt_status: WatchdogAttemptStatus::Dispatched,
                    ..
                } if attempt_generation == token.callback_generation
            ),
            (
                EntryKind::Ordinary { .. },
                CallbackRole::WatchdogScheduler | CallbackRole::WatchdogWork,
            )
            | (EntryKind::Watchdog { .. }, CallbackRole::OrdinaryWork) => false,
        };
        if !valid {
            return Err((RegistryError::StaleCallback, handle));
        }
        let Some(slot) = entry.provider_slot_mut(token.role) else {
            return Err((RegistryError::StaleCallback, handle));
        };
        if slot.is_some() {
            return Err((RegistryError::ProviderHandleAlreadyOwned, handle));
        }
        *slot = Some(OwnedProviderHandle {
            callback_generation: token.callback_generation,
            handle,
        });
        Ok(())
    }

    pub(crate) fn take_wakeup_handle(
        &mut self,
        identity: &TimerIdentity,
    ) -> Option<ProviderHandle> {
        let entry = self.entries.get_mut(identity)?;
        entry.take_wakeup_handle(identity)
    }

    pub(crate) fn take_work_handle(&mut self, identity: &TimerIdentity) -> Option<ProviderHandle> {
        let entry = self.entries.get_mut(identity)?;
        entry.take_work_handle(identity)
    }

    pub(crate) fn take_provider_handles_for_claim(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<ProviderHandles, RegistryError> {
        let identity = claim.identity.clone();
        let entry = self.entry_mut(claim)?;
        Ok(entry.take_provider_handles(&identity))
    }

    pub(crate) fn consume_provider_handle(&mut self, token: &CallbackToken) {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return;
        };
        if !entry.owns_token_claim(token) {
            return;
        }
        let Some(slot) = entry.provider_slot_mut(token.role) else {
            return;
        };
        let matches_token = slot
            .as_ref()
            .is_some_and(|owned| owned.callback_generation == token.callback_generation);
        if matches_token {
            *slot = None;
        }
    }

    /// Confirm that the platform successfully applied one emitted arm effect.
    ///
    /// The runtime calls this synchronously after binding the returned provider
    /// handles. Pure tests use it to distinguish requested effects from actual
    /// provider operations.
    pub(crate) fn confirm_effect_applied(
        &mut self,
        effect: &RegistryEffect,
    ) -> Result<(), RegistryError> {
        if !effect.has_valid_shape() {
            return Err(RegistryError::StaleCallback);
        }
        let (entry, callback_generation, delay_ns) = match effect {
            RegistryEffect::None | RegistryEffect::ClearCallbacks { .. } => return Ok(()),
            RegistryEffect::ArmWakeup {
                token, delay_ns, ..
            } => {
                let entry = self.entry_by_token_mut(token)?;
                let valid_generation = match (&entry.kind, token.role) {
                    (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => {
                        matches!(
                            control.registration,
                            TimerRegistration::Scheduled { generation, .. }
                                if generation == token.callback_generation
                        )
                    }
                    (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler) => {
                        matches!(
                            control.state,
                            WatchdogState::Scheduled {
                                scheduler_generation,
                                ..
                            } if scheduler_generation == token.callback_generation
                        )
                    }
                    (EntryKind::Ordinary { .. } | EntryKind::Watchdog { .. }, _) => false,
                };
                if !valid_generation {
                    return Err(RegistryError::StaleCallback);
                }
                (entry, token.callback_generation, *delay_ns)
            }
            RegistryEffect::DispatchWatchdog {
                successor,
                successor_delay_ns,
                work,
                ..
            } => {
                let successor_callback_generation = successor.callback_generation;
                let entry = self.entry_by_token_mut(successor)?;
                let EntryKind::Watchdog { control, .. } = &entry.kind else {
                    return Err(RegistryError::StaleCallback);
                };
                if !matches!(
                    control.state,
                    WatchdogState::AwaitingWork {
                        successor_generation,
                        attempt_generation,
                        ..
                    } if successor_generation == successor_callback_generation
                        && attempt_generation == work.callback_generation
                ) {
                    return Err(RegistryError::StaleCallback);
                }
                (entry, successor_callback_generation, *successor_delay_ns)
            }
        };
        // Validate state and both dispatch tokens before deduplicating. Each
        // dispatch advances the non-wrapping wakeup generation and binds it to
        // one work attempt, so one marker deduplicates both committed counters.
        if entry.confirmed_wakeup_generation == Some(callback_generation) {
            return Ok(());
        }
        entry.confirmed_wakeup_generation = Some(callback_generation);
        entry.latest_armed_delay_ns = Some(delay_ns);
        let counters = entry.observability.counters_mut();
        counters.record_wakeup_armed();
        if matches!(effect, RegistryEffect::DispatchWatchdog { .. }) {
            counters.record_work_dispatched();
        }
        Ok(())
    }

    fn entry(&self, claim: &RegistrationClaim) -> Result<&Entry, RegistryError> {
        let entry = self
            .entries
            .get(claim.identity())
            .ok_or(RegistryError::UnknownRegistration)?;
        if entry.claim_generation != claim.claim_generation() {
            return Err(RegistryError::StaleRegistration);
        }
        Ok(entry)
    }

    fn entry_mut(&mut self, claim: &RegistrationClaim) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(claim.identity())
            .ok_or(RegistryError::UnknownRegistration)?;
        if entry.claim_generation != claim.claim_generation() {
            return Err(RegistryError::StaleRegistration);
        }
        Ok(entry)
    }

    // Select identity and exact claim; callers validate the role and generation
    // against their operation's policy state.
    fn entry_by_token_mut(&mut self, token: &CallbackToken) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if !entry.owns_token_claim(token) {
            return Err(RegistryError::StaleCallback);
        }
        Ok(entry)
    }

    fn running_work_entry(&self, token: &CallbackToken) -> Result<&Entry, RegistryError> {
        let entry = self
            .entries
            .get(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.owns_running_work(token) {
            Ok(entry)
        } else {
            Err(RegistryError::StaleCallback)
        }
    }

    fn running_work_entry_mut(
        &mut self,
        token: &CallbackToken,
    ) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.owns_running_work(token) {
            Ok(entry)
        } else {
            Err(RegistryError::StaleCallback)
        }
    }
}

fn token_for(
    claim: &RegistrationClaim,
    callback_generation: u64,
    role: CallbackRole,
) -> CallbackToken {
    CallbackToken::new(
        claim.identity.clone(),
        claim.claim_generation,
        callback_generation,
        role,
    )
}

fn detach_provider_handle(
    identity: &TimerIdentity,
    claim_generation: u64,
    role: CallbackRole,
    owned: OwnedProviderHandle,
) -> ProviderHandle {
    ProviderHandle {
        token: CallbackToken::new(
            identity.clone(),
            claim_generation,
            owned.callback_generation,
            role,
        ),
        handle: owned.handle,
    }
}

const fn memory_sample(start: MemoryPages, end: MemoryPages) -> MemoryPageSample {
    MemoryPageSample::new(
        MemoryPageExtent::new(start.wasm(), start.stable()),
        MemoryPageExtent::new(end.wasm(), end.stable()),
    )
}

const fn clear_callbacks(identity: TimerIdentity, handles: CallbacksToClear) -> RegistryEffect {
    RegistryEffect::ClearCallbacks { identity, handles }
}

fn clear_wakeup_if(identity: TimerIdentity, clear_wakeup: bool) -> RegistryEffect {
    if clear_wakeup {
        clear_callbacks(identity, CallbacksToClear::Wakeup)
    } else {
        RegistryEffect::None
    }
}

fn terminal_ordinary(
    entry: &mut Entry,
    identity: TimerIdentity,
    failure: TimerControlFailure,
) -> RegistryTransition {
    let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
        return RegistryTransition::terminal(RegistryEffect::None, failure);
    };
    let clear_wakeup = control.terminate(InactiveReason::ControlFailure(failure));
    RegistryTransition::terminal(clear_wakeup_if(identity, clear_wakeup), failure)
}

// Callers have validated the exact running generation. Checked completion
// failures leave that state unchanged before this shared stop finalization.
fn stop_ordinary_completion(
    entry: &mut Entry,
    completion: TimerCompletion,
    now_ns: u64,
    failure: Option<TimerControlFailure>,
) -> RegistryTransition {
    let (reason, completion, transition) = failure.map_or_else(
        || {
            (
                InactiveReason::InvariantFailure,
                completion,
                RegistryTransition::normal(RegistryEffect::None),
            )
        },
        |failure| {
            (
                InactiveReason::ControlFailure(failure),
                TimerCompletion::invariant_failure(completion.work_count()),
                RegistryTransition::terminal(RegistryEffect::None, failure),
            )
        },
    );
    let EntryKind::Ordinary { control, .. } = &mut entry.kind else {
        return transition;
    };
    control.terminate(reason);
    entry.latest_directive = Some(TimerDirectiveSnapshot::Stop);
    entry.observability.record_completion(completion, now_ns);
    transition
}

const fn map_directive_failure(error: DirectiveError) -> TimerControlFailure {
    match error {
        DirectiveError::Schedule(ScheduleError::DeadlineOverflow) => {
            TimerControlFailure::DeadlineOverflow
        }
        DirectiveError::Schedule(ScheduleError::DelayOutOfRange) => {
            TimerControlFailure::DelayOutOfRange
        }
        DirectiveError::Schedule(ScheduleError::ZeroCadence) | DirectiveError::MissingCadence => {
            TimerControlFailure::DirectiveNotAllowed
        }
    }
}

const fn select_completion_schedule(
    pending: Option<OrdinaryPending>,
    callback: Option<ResolvedSchedule>,
) -> Option<ResolvedSchedule> {
    match pending {
        Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => None,
        Some(OrdinaryPending::Reconcile(pending)) => Some(pending),
        Some(OrdinaryPending::Schedule(pending)) => match callback {
            Some(callback) if callback.deadline_ns < pending.deadline_ns => Some(callback),
            Some(_) | None => Some(pending),
        },
        None => callback,
    }
}

const fn select_pending_ordinary(
    current: Option<OrdinaryPending>,
    request: OrdinaryRequest,
    requested: ResolvedSchedule,
) -> OrdinaryPending {
    if matches!(current, Some(OrdinaryPending::Unregister)) {
        return OrdinaryPending::Unregister;
    }
    match request {
        OrdinaryRequest::Reconcile => OrdinaryPending::Reconcile(requested),
        OrdinaryRequest::EnsureOnce | OrdinaryRequest::EnsureRecurring => match current {
            Some(OrdinaryPending::Reconcile(current) | OrdinaryPending::Schedule(current))
                if current.deadline_ns <= requested.deadline_ns =>
            {
                OrdinaryPending::Schedule(current)
            }
            Some(
                OrdinaryPending::Cancel
                | OrdinaryPending::Reconcile(_)
                | OrdinaryPending::Schedule(_),
            )
            | None => OrdinaryPending::Schedule(requested),
            Some(OrdinaryPending::Unregister) => OrdinaryPending::Unregister,
        },
    }
}

const fn select_pending_watchdog(
    pending: Option<WatchdogPending>,
    request: WatchdogScheduleRequest,
    now_ns: u64,
) -> WatchdogPending {
    match (pending, request) {
        (Some(WatchdogPending::Unregister), _) => WatchdogPending::Unregister,
        (_, WatchdogScheduleRequest::Reconcile(requested)) => WatchdogPending::Reconcile(requested),
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Cadence) => {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::Reconcile(requested)), WatchdogScheduleRequest::Immediate)
            if requested.deadline_ns <= now_ns =>
        {
            WatchdogPending::Reconcile(requested)
        }
        (Some(WatchdogPending::EnsureImmediately), _) | (_, WatchdogScheduleRequest::Immediate) => {
            WatchdogPending::EnsureImmediately
        }
        (
            Some(WatchdogPending::Cancel | WatchdogPending::Ensure) | None,
            WatchdogScheduleRequest::Cadence,
        ) => WatchdogPending::Ensure,
    }
}

fn cancel_watchdog(control: &mut WatchdogControl, identity: &TimerIdentity) -> RegistryTransition {
    match &mut control.state {
        WatchdogState::Inactive { .. } => RegistryTransition::normal(RegistryEffect::None),
        WatchdogState::AwaitingWork {
            attempt_status: WatchdogAttemptStatus::Running,
            pending,
            ..
        } => {
            if !matches!(pending, Some(WatchdogPending::Unregister)) {
                *pending = Some(WatchdogPending::Cancel);
            }
            RegistryTransition::normal(RegistryEffect::None)
        }
        WatchdogState::Scheduled { .. }
        | WatchdogState::AwaitingWork {
            attempt_status: WatchdogAttemptStatus::Dispatched,
            ..
        } => {
            let clear_work = matches!(control.state, WatchdogState::AwaitingWork { .. });
            let Some((scheduler_generation, attempt_generation)) =
                control.next_dispatch_generations()
            else {
                return control.terminate(
                    clear_callbacks(
                        identity.clone(),
                        CallbacksToClear::wakeup_and_maybe_work(clear_work),
                    ),
                    TimerControlFailure::GenerationExhausted,
                );
            };
            control.scheduler_generation = scheduler_generation;
            control.attempt_generation = attempt_generation;
            control.state = WatchdogState::Inactive {
                reason: InactiveReason::Cancelled,
            };
            RegistryTransition::normal(clear_callbacks(
                identity.clone(),
                CallbacksToClear::wakeup_and_maybe_work(clear_work),
            ))
        }
    }
}

fn remove_after(
    entries: &mut BTreeMap<TimerIdentity, Entry>,
    identity: &TimerIdentity,
    transition: RegistryTransition,
    remove: bool,
) -> RegistryTransition {
    if remove {
        entries.remove(identity);
    }
    transition
}

#[cfg(test)]
mod tests;
