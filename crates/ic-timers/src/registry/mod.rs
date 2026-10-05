//! Bounded canonical registry and provider-neutral policy transition engine.
//!
//! The registry emits provider-neutral effects. The runtime binds them to
//! linear provider handles while the registry remains the sole logical state
//! authority.

use crate::{
    callback::OrdinaryRunResult,
    control::{OrdinaryPending, TimerControl, TimerRegistration, WakeupArm},
    platform::TimerHandle,
    schedule::{OrdinaryDirective, ResolvedSchedule, ScheduleError, TimerCadence, TimerSchedule},
    snapshot::{
        DeclarationLifetime, InactiveReason, MemoryPageExtent, MemoryPageSample,
        OrdinaryRuntimeStateSnapshot, TimerCompletion, TimerCompletionOutcome, TimerControlFailure,
        TimerDirectiveSnapshot, TimerEpoch, TimerIdentity, TimerInventorySnapshot,
        TimerObservabilitySnapshot, TimerPolicy, TimerRegistrationId, TimerRuntimeStateSnapshot,
        TimerSchedulingMode, TimerSnapshot, WatchdogAttemptStatus, WatchdogDecision,
        WatchdogRunResult, WatchdogRuntimeStateSnapshot,
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
#[derive(Debug, Eq, PartialEq)]
pub struct CallbackToken {
    claim: RegistrationClaim,
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
            claim: RegistrationClaim {
                identity,
                claim_generation,
            },
            callback_generation,
            role,
        }
    }

    pub(crate) const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    pub(crate) const fn claim(&self) -> &RegistrationClaim {
        &self.claim
    }

    #[cfg(test)]
    pub(crate) const fn callback_generation(&self) -> u64 {
        self.callback_generation
    }

    pub(crate) const fn role(&self) -> CallbackRole {
        self.role
    }

    pub(crate) fn belongs_to_same_claim(&self, other: &Self) -> bool {
        self.claim == other.claim
    }
}

impl Clone for CallbackToken {
    fn clone(&self) -> Self {
        // Queued deliveries need owned tokens. Retained registration capabilities
        // remain non-clone, and running contexts borrow this token's exact claim.
        Self::new(
            self.identity().clone(),
            self.claim.claim_generation,
            self.callback_generation,
            self.role,
        )
    }
}

impl RegistrationClaim {
    pub(crate) const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    pub(crate) const fn claim_generation(&self) -> u64 {
        self.claim_generation
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
                    && successor.callback_generation == work.callback_generation
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

type OrdinaryFuture = Pin<Box<dyn Future<Output = OrdinaryRunResult>>>;
pub(crate) type OrdinaryCallback = Rc<RefCell<Box<dyn FnMut(CallbackToken) -> OrdinaryFuture>>>;
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
        deadline_ns: u64,
    },
    AwaitingWork {
        successor_deadline_ns: u64,
        attempt_status: WatchdogAttemptStatus,
        pending: Option<WatchdogPending>,
    },
}

#[derive(Debug)]
struct WatchdogControl {
    // One allocation history survives inactive state. A dispatch stamps its
    // successor and work with one generation, distinguished by callback role.
    // Requests while awaiting work cannot replace that successor until completion.
    generation: u64,
    state: WatchdogState,
}

impl Default for WatchdogControl {
    fn default() -> Self {
        Self {
            generation: 0,
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
        let Some(generation) = self.generation.checked_add(1) else {
            let cleanup = clear_wakeup_if(claim.identity(), arm.replaces_existing());
            return self.terminate(cleanup, TimerControlFailure::GenerationExhausted);
        };
        self.generation = generation;
        self.state = WatchdogState::Scheduled { deadline_ns };
        RegistryTransition::normal(RegistryEffect::ArmWakeup {
            token: token_for(claim, generation, CallbackRole::WatchdogScheduler),
            delay_ns: deadline_ns.saturating_sub(now_ns),
            arm,
        })
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
                TimerRegistration::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled {
                        generation: control.generation(),
                        deadline_ns,
                    })
                }
                TimerRegistration::Running { .. } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Running {
                        generation: control.generation(),
                    })
                }
            },
            EntryKind::Watchdog { control, .. } => match control.state {
                WatchdogState::Inactive { reason } => {
                    TimerRuntimeStateSnapshot::Inactive { reason }
                }
                WatchdogState::Scheduled { deadline_ns } => {
                    TimerRuntimeStateSnapshot::Watchdog(WatchdogRuntimeStateSnapshot::Scheduled {
                        scheduler_generation: control.generation,
                        deadline_ns,
                    })
                }
                WatchdogState::AwaitingWork {
                    successor_deadline_ns,
                    attempt_status,
                    ..
                } => TimerRuntimeStateSnapshot::Watchdog(
                    WatchdogRuntimeStateSnapshot::AwaitingWork {
                        successor_generation: control.generation,
                        successor_deadline_ns,
                        attempt_status,
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

    /// Decide terminal removal from the entry already selected and authorized
    /// by the transition owner. Runtime detaches handles before that transition.
    const fn remove_on_failure(&self, transition: &RegistryTransition) -> bool {
        transition.failure().is_some()
            && matches!(self.lifetime, DeclarationLifetime::RemoveWhenStopped)
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

    // The caller selected this entry by identity. Fired handles are consumed
    // independently of callback acceptance, but only for the exact claim,
    // policy role and owned slot generation.
    fn consume_provider_handle(&mut self, token: &CallbackToken) {
        if !self.owns_token_claim(token) {
            return;
        }
        let slot = match token.role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler
                if token.role == self.kind.wakeup_role() =>
            {
                &mut self.wakeup
            }
            CallbackRole::WatchdogWork if matches!(self.kind, EntryKind::Watchdog { .. }) => {
                &mut self.work
            }
            _ => return,
        };
        if slot
            .as_ref()
            .is_some_and(|owned| owned.callback_generation == token.callback_generation)
        {
            *slot = None;
        }
    }

    const fn owns_token_claim(&self, token: &CallbackToken) -> bool {
        self.claim_generation == token.claim().claim_generation()
    }

    // Registry lookups select this entry by token identity first.
    const fn owns_running_work(&self, token: &CallbackToken) -> bool {
        if !self.owns_token_claim(token) {
            return false;
        }
        match (&self.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration,
                TimerRegistration::Running { .. }
                    if control.generation() == token.callback_generation
            ),
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork) => matches!(
                control.state,
                WatchdogState::AwaitingWork {
                    attempt_status: WatchdogAttemptStatus::Running,
                    ..
                } if control.generation == token.callback_generation
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
        let entry = self.entry(claim)?;
        if !matches!(entry.kind, EntryKind::Ordinary { .. }) {
            return Err(RegistryError::PolicyMismatch {
                actual: entry.kind.policy().label(),
            });
        }
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
                    TimerRegistration::Scheduled { deadline_ns } => deadline_ns,
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
                let clear_wakeup = control.terminate(InactiveReason::ControlFailure(error));
                let transition = RegistryTransition::terminal(
                    clear_wakeup_if(claim.identity(), clear_wakeup),
                    error,
                );
                let remove = entry.remove_on_failure(&transition);
                return Ok(remove_after(
                    &mut self.entries,
                    claim.identity(),
                    transition,
                    remove,
                ));
            }
        };

        if let TimerRegistration::Running { pending } = &mut control.registration {
            *pending = Some(select_pending_ordinary(*pending, request, requested));
        }
        if arm.is_some() || matches!(request, OrdinaryRequest::Reconcile) {
            entry.scheduling_mode = requested.mode;
        }

        let Some(arm) = arm else {
            entry.observability.counters_mut().record_coalesced();
            return Ok(RegistryTransition::normal(RegistryEffect::None));
        };
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
        let (mode, requested_delay_ns) = match request {
            WatchdogScheduleRequest::Cadence => {
                (TimerSchedulingMode::Watchdog, Some(cadence.as_nanos()))
            }
            WatchdogScheduleRequest::Immediate => (TimerSchedulingMode::Continuation, Some(0)),
            WatchdogScheduleRequest::Reconcile(requested) => {
                (requested.mode, requested.requested_delay_ns)
            }
        };
        let transition = match &mut control.state {
            WatchdogState::Inactive { .. } => {
                let deadline_ns = match request {
                    WatchdogScheduleRequest::Cadence => cadence.deadline_after(now_ns)?,
                    WatchdogScheduleRequest::Immediate => now_ns,
                    WatchdogScheduleRequest::Reconcile(requested) => requested.deadline_ns,
                };
                control.arm_scheduler(claim, now_ns, deadline_ns, WakeupArm::Initial)
            }
            WatchdogState::Scheduled { deadline_ns }
                if match request {
                    WatchdogScheduleRequest::Immediate => *deadline_ns > now_ns,
                    WatchdogScheduleRequest::Reconcile(requested) => {
                        *deadline_ns != requested.deadline_ns
                    }
                    WatchdogScheduleRequest::Cadence => false,
                } =>
            {
                let deadline_ns = match request {
                    WatchdogScheduleRequest::Reconcile(requested) => requested.deadline_ns,
                    WatchdogScheduleRequest::Immediate | WatchdogScheduleRequest::Cadence => now_ns,
                };
                control.arm_scheduler(claim, now_ns, deadline_ns, WakeupArm::Replacement)
            }
            WatchdogState::Scheduled { .. } => RegistryTransition::normal(RegistryEffect::None),
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                pending,
                ..
            } => {
                if let WatchdogScheduleRequest::Reconcile(requested) = request {
                    *pending = Some(WatchdogPending::Reconcile(requested));
                }
                RegistryTransition::normal(RegistryEffect::None)
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                pending,
                ..
            } => {
                *pending = Some(select_pending_watchdog(*pending, request, now_ns));
                RegistryTransition::normal(RegistryEffect::None)
            }
        };
        if matches!(transition.effect(), RegistryEffect::ArmWakeup { .. }) {
            entry.scheduling_mode = mode;
        }
        if matches!(
            (transition.effect(), transition.failure()),
            (RegistryEffect::None, None)
        ) {
            entry.observability.counters_mut().record_coalesced();
        }
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested_delay_ns;
        let remove = entry.remove_on_failure(&transition);
        Ok(remove_after(
            &mut self.entries,
            claim.identity(),
            transition,
            remove,
        ))
    }

    pub(crate) fn cancel(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity();
        let (effect, remove) = {
            let entry = self.entry_mut(claim)?;
            let (effect, stopped) = match &mut entry.kind {
                EntryKind::Ordinary { control, .. } => {
                    let effect = match &mut control.registration {
                        TimerRegistration::Inactive { .. } => RegistryEffect::None,
                        TimerRegistration::Scheduled { .. } => {
                            control.terminate(InactiveReason::Cancelled);
                            entry.observability.counters_mut().record_cancellation();
                            RegistryEffect::ClearCallbacks {
                                identity: identity.clone(),
                                handles: CallbacksToClear::Wakeup,
                            }
                        }
                        TimerRegistration::Running { pending } => {
                            if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                                *pending = Some(OrdinaryPending::Cancel);
                            }
                            RegistryEffect::None
                        }
                    };
                    (
                        effect,
                        matches!(control.registration, TimerRegistration::Inactive { .. }),
                    )
                }
                EntryKind::Watchdog { control, .. } => {
                    let handles = match &mut control.state {
                        WatchdogState::Inactive { .. } => None,
                        WatchdogState::Scheduled { .. } => Some(CallbacksToClear::Wakeup),
                        WatchdogState::AwaitingWork {
                            attempt_status: WatchdogAttemptStatus::Dispatched,
                            ..
                        } => Some(CallbacksToClear::WakeupAndWork),
                        WatchdogState::AwaitingWork {
                            attempt_status: WatchdogAttemptStatus::Running,
                            pending,
                            ..
                        } => {
                            if !matches!(*pending, Some(WatchdogPending::Unregister)) {
                                *pending = Some(WatchdogPending::Cancel);
                            }
                            None
                        }
                    };
                    let effect = handles.map_or(RegistryEffect::None, |handles| {
                        control.state = WatchdogState::Inactive {
                            reason: InactiveReason::Cancelled,
                        };
                        entry.observability.counters_mut().record_cancellation();
                        RegistryEffect::ClearCallbacks {
                            identity: identity.clone(),
                            handles,
                        }
                    });
                    let stopped = matches!(control.state, WatchdogState::Inactive { .. });
                    (effect, stopped)
                }
            };
            (
                effect,
                stopped && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
            )
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
            RegistryTransition::normal(effect),
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
        let identity = claim.identity();
        let entry = self.entry_mut(claim)?;
        match &mut entry.kind {
            EntryKind::Ordinary {
                control:
                    TimerControl {
                        registration: TimerRegistration::Running { pending },
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
                self.entries.remove(identity);
                Ok(transition)
            }
        }
    }

    /// Consume this delivery's handle, then accept ordinary work and return its
    /// callback.
    pub(crate) fn begin_ordinary(&mut self, token: &CallbackToken) -> Option<OrdinaryCallback> {
        let entry = self.entries.get_mut(token.identity())?;
        entry.consume_provider_handle(token);
        if token.role != CallbackRole::OrdinaryWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_wakeup();
            return None;
        }
        let EntryKind::Ordinary {
            control, callback, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_wakeup();
            return None;
        };
        if control.begin(token.callback_generation) {
            entry.observability.counters_mut().record_work_started();
            Some(Rc::clone(callback))
        } else {
            entry.observability.counters_mut().record_stale_wakeup();
            None
        }
    }

    pub(crate) fn complete_ordinary(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: OrdinaryRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity();
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
                Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => {
                    OrdinaryDirective::Stop
                }
                Some(OrdinaryPending::Reconcile(requested)) => {
                    OrdinaryDirective::ScheduleAt(requested.deadline_ns)
                }
                Some(OrdinaryPending::Schedule(_)) | None => result.directive(),
            };
            if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                let transition = stop_ordinary_completion(entry, completion, now_ns, None);
                (transition, remove_on_stop)
            } else {
                let prepared = match effective_directive.resolve(now_ns, cadence) {
                    Ok(resolved) => {
                        let directive_snapshot =
                            TimerDirectiveSnapshot::try_from(effective_directive)
                                .map_err(RegistryError::Schedule)?;
                        let selected = select_completion_schedule(pending_command, resolved);
                        selected
                            .map_or(Ok(()), |selected| {
                                control.arm_deadline(selected.deadline_ns)
                            })
                            .map(|()| (selected, directive_snapshot))
                    }
                    Err(failure) => Err(failure),
                };
                match prepared {
                    Err(failure) => (
                        stop_ordinary_completion(entry, completion, now_ns, Some(failure)),
                        remove_on_stop,
                    ),
                    Ok((selected_schedule, directive_snapshot)) => {
                        entry.latest_directive = Some(directive_snapshot);
                        entry.observability.record_completion(completion, now_ns);

                        let remove = selected_schedule.is_none() && remove_on_stop;
                        let transition = if let Some(selected) = selected_schedule {
                            entry.scheduling_mode = selected.mode;
                            entry.latest_requested_delay_ns = selected.requested_delay_ns;
                            RegistryTransition::normal(RegistryEffect::ArmWakeup {
                                token: token_for(
                                    token.claim(),
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
                }
            }
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
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
            WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                ..
            } => control.generation == token.callback_generation,
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
        let next_dispatch = control
            .generation
            .checked_add(1)
            .ok_or(TimerControlFailure::GenerationExhausted)
            .and_then(|generation| {
                cadence
                    .deadline_after(now_ns)
                    .map(|deadline_ns| (generation, deadline_ns))
                    .map_err(|_| TimerControlFailure::DeadlineOverflow)
            });
        let (generation, successor_deadline_ns) = match next_dispatch {
            Ok(next) => next,
            Err(failure) => {
                let transition = control.terminate(
                    RegistryEffect::ClearCallbacks {
                        identity: token.identity().clone(),
                        handles: CallbacksToClear::Work,
                    },
                    failure,
                );
                let remove = entry.remove_on_failure(&transition);
                return remove_after(&mut self.entries, token.identity(), transition, remove);
            }
        };

        control.generation = generation;
        control.state = WatchdogState::AwaitingWork {
            successor_deadline_ns,
            attempt_status: WatchdogAttemptStatus::Dispatched,
            pending: None,
        };
        entry.scheduling_mode = TimerSchedulingMode::Watchdog;
        RegistryTransition::normal(RegistryEffect::DispatchWatchdog {
            successor: token_for(token.claim(), generation, CallbackRole::WatchdogScheduler),
            successor_delay_ns: cadence.as_nanos(),
            work: token_for(token.claim(), generation, CallbackRole::WatchdogWork),
        })
    }

    /// Consume this delivery's work handle, then accept Watchdog work and return
    /// its callback.
    pub(crate) fn begin_watchdog_work(
        &mut self,
        token: &CallbackToken,
    ) -> Option<WatchdogCallback> {
        let entry = self.entries.get_mut(token.identity())?;
        entry.consume_provider_handle(token);
        if token.role != CallbackRole::WatchdogWork || !entry.owns_token_claim(token) {
            entry.observability.counters_mut().record_stale_work();
            return None;
        }
        let EntryKind::Watchdog {
            control, callback, ..
        } = &mut entry.kind
        else {
            entry.observability.counters_mut().record_stale_work();
            return None;
        };
        match &mut control.state {
            WatchdogState::AwaitingWork { attempt_status, .. }
                if control.generation == token.callback_generation
                    && *attempt_status == WatchdogAttemptStatus::Dispatched =>
            {
                *attempt_status = WatchdogAttemptStatus::Running;
                entry.observability.counters_mut().record_work_started();
                Some(Rc::clone(callback))
            }
            WatchdogState::Inactive { .. }
            | WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork { .. } => {
                entry.observability.counters_mut().record_stale_work();
                None
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
        let identity = token.identity();
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
            let pending_schedule = match pending {
                Some(WatchdogPending::Reconcile(requested)) => Some(requested),
                _ => None,
            };

            let transition = match decision {
                WatchdogDecision::Continue => {
                    control.state = WatchdogState::Scheduled {
                        deadline_ns: successor_deadline_ns,
                    };
                    entry.scheduling_mode = TimerSchedulingMode::Watchdog;
                    entry.latest_requested_delay_ns = Some(cadence.as_nanos());
                    RegistryTransition::normal(RegistryEffect::None)
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
                            deadline_ns: successor_deadline_ns,
                        };
                        RegistryTransition::normal(RegistryEffect::None)
                    } else {
                        control.arm_scheduler(
                            token.claim(),
                            now_ns,
                            deadline_ns,
                            WakeupArm::Replacement,
                        )
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
                    RegistryTransition::normal(RegistryEffect::ClearCallbacks {
                        identity: identity.clone(),
                        handles: CallbacksToClear::Wakeup,
                    })
                }
            };
            let remove = matches!(control.state, WatchdogState::Inactive { .. })
                && (matches!(pending, Some(WatchdogPending::Unregister))
                    || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped));
            (transition, remove)
        };

        Ok(remove_after(
            &mut self.entries,
            identity,
            transition,
            remove,
        ))
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
        memory_start: MemoryPageExtent,
        memory_end: MemoryPageExtent,
    ) -> Result<(), RegistryError> {
        // A normal remove-on-stop completion can delete its entry before the
        // post-run measurement is committed, leaving nothing to observe.
        // Identity reuse must not let a late callback write into a newer
        // registration's observations.
        let Ok(entry) = self.entry_mut(token.claim()) else {
            return Ok(());
        };

        let memory = MemoryPageSample::new(memory_start, memory_end);
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
        let identity = claim.identity();
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
            let handles = entry.take_provider_handles(identity);
            (
                handles,
                matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
            )
        };
        if remove {
            self.entries.remove(identity);
        }
        Ok(handles)
    }

    pub(crate) fn validate_running_context(
        &self,
        token: &CallbackToken,
    ) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.owns_running_work(token) {
            Ok(())
        } else {
            Err(RegistryError::StaleCallback)
        }
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
        let slot = match (&entry.kind, token.role) {
            (EntryKind::Ordinary { control, .. }, CallbackRole::OrdinaryWork)
                if matches!(
                    control.registration,
                    TimerRegistration::Scheduled { .. }
                        if control.generation() == token.callback_generation
                ) =>
            {
                &mut entry.wakeup
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler)
                if matches!(
                    control.state,
                    WatchdogState::Scheduled { .. } | WatchdogState::AwaitingWork { .. }
                        if control.generation == token.callback_generation
                ) =>
            {
                &mut entry.wakeup
            }
            (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogWork)
                if matches!(
                    control.state,
                    WatchdogState::AwaitingWork {
                        attempt_status: WatchdogAttemptStatus::Dispatched,
                        ..
                    } if control.generation == token.callback_generation
                ) =>
            {
                &mut entry.work
            }
            _ => return Err((RegistryError::StaleCallback, handle)),
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

    pub(crate) fn take_provider_handles(&mut self, identity: &TimerIdentity) -> ProviderHandles {
        self.entries
            .get_mut(identity)
            .map_or_else(ProviderHandles::default, |entry| {
                entry.take_provider_handles(identity)
            })
    }

    pub(crate) fn take_provider_handles_for_claim(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<ProviderHandles, RegistryError> {
        let identity = claim.identity();
        let entry = self.entry_mut(claim)?;
        Ok(entry.take_provider_handles(identity))
    }

    /// Consume the delivered scheduler handle before detaching the remaining
    /// capabilities. A scheduler transition can remove a transient declaration.
    pub(crate) fn take_watchdog_scheduler_handles(
        &mut self,
        token: &CallbackToken,
    ) -> Result<ProviderHandles, RegistryError> {
        let entry = match self.entry_mut(token.claim()) {
            Ok(entry) => entry,
            // Removed or superseded claims are normal stale callback delivery.
            Err(RegistryError::UnknownRegistration | RegistryError::StaleRegistration) => {
                return Ok(ProviderHandles::default());
            }
            Err(error) => return Err(error),
        };
        entry.consume_provider_handle(token);
        Ok(entry.take_provider_handles(token.identity()))
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
                            TimerRegistration::Scheduled { .. }
                                if control.generation() == token.callback_generation
                        )
                    }
                    (EntryKind::Watchdog { control, .. }, CallbackRole::WatchdogScheduler) => {
                        matches!(
                            control.state,
                            WatchdogState::Scheduled { .. }
                                if control.generation == token.callback_generation
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
                ..
            } => {
                let successor_callback_generation = successor.callback_generation;
                let entry = self.entry_by_token_mut(successor)?;
                let EntryKind::Watchdog { control, .. } = &entry.kind else {
                    return Err(RegistryError::StaleCallback);
                };
                if !matches!(
                    control.state,
                    WatchdogState::AwaitingWork { .. }
                        if control.generation == successor_callback_generation
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
        // Missing and superseded claims are both stale callback delivery.
        self.entry_mut(token.claim())
            .map_err(|_| RegistryError::StaleCallback)
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

fn clear_wakeup_if(identity: &TimerIdentity, clear_wakeup: bool) -> RegistryEffect {
    if clear_wakeup {
        RegistryEffect::ClearCallbacks {
            identity: identity.clone(),
            handles: CallbacksToClear::Wakeup,
        }
    } else {
        RegistryEffect::None
    }
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
    match (current, request) {
        (Some(OrdinaryPending::Unregister), _) => OrdinaryPending::Unregister,
        (_, OrdinaryRequest::Reconcile) => OrdinaryPending::Reconcile(requested),
        (
            Some(OrdinaryPending::Reconcile(current) | OrdinaryPending::Schedule(current)),
            OrdinaryRequest::EnsureOnce | OrdinaryRequest::EnsureRecurring,
        ) if current.deadline_ns <= requested.deadline_ns => OrdinaryPending::Schedule(current),
        (_, OrdinaryRequest::EnsureOnce | OrdinaryRequest::EnsureRecurring) => {
            OrdinaryPending::Schedule(requested)
        }
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
