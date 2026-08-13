//! Pure bounded registry and policy-specific transition engine.
//!
//! Patch 2 deliberately stops at provider-neutral effects. Patch 3 binds
//! those effects to linear `ic-cdk-timers` handles without replacing this
//! ownership or state model.

#![cfg_attr(not(test), allow(dead_code))]

use crate::{
    DeclarationLifetime, InactiveReason, OrdinaryRuntimeStateSnapshot, ScheduleError, TimerCadence,
    TimerCompletion, TimerCompletionOutcome, TimerControl, TimerControlAction, TimerControlError,
    TimerControlFailure, TimerDirective, TimerDirectiveSnapshot, TimerEpoch, TimerIdentity,
    TimerObservabilitySnapshot, TimerPolicy, TimerRunResult, TimerRuntimeStateSnapshot,
    TimerSchedule, TimerSchedulingMode, TimerSnapshot, WatchdogAttemptSnapshot,
    WatchdogAttemptStatus, WatchdogDecision, WatchdogRunResult, WatchdogRuntimeStateSnapshot,
    platform::TimerHandle,
    runtime::{TimerContext, TimerFuture},
};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use thiserror::Error;

/// Maximum declarations owned by one canonical registry.
pub const MAX_TIMER_REGISTRATIONS: usize = 64;

/// Opaque logical ownership claim returned by pure registration.
#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationClaim {
    identity: TimerIdentity,
    claim_generation: u64,
}

impl RegistrationClaim {
    pub(crate) const fn identity(&self) -> &TimerIdentity {
        &self.identity
    }

    pub(crate) const fn claim_generation(&self) -> u64 {
        self.claim_generation
    }

    pub(crate) const fn delegated(identity: TimerIdentity, claim_generation: u64) -> Self {
        Self {
            identity,
            claim_generation,
        }
    }
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

    pub(crate) const fn callback_generation(&self) -> u64 {
        self.callback_generation
    }

    pub(crate) const fn claim_generation(&self) -> u64 {
        self.claim_generation
    }

    pub(crate) const fn role(&self) -> CallbackRole {
        self.role
    }
}

/// Provider-neutral effect emitted by one pure transition.
#[derive(Debug, Eq, PartialEq)]
pub enum RegistryEffect {
    None,
    ArmWakeup {
        token: CallbackToken,
        deadline_ns: u64,
        delay_ns: u64,
        replace: bool,
    },
    ClearCallbacks {
        identity: TimerIdentity,
        clear_wakeup: bool,
        clear_work: bool,
    },
    DispatchWatchdog {
        successor: CallbackToken,
        successor_deadline_ns: u64,
        successor_delay_ns: u64,
        work: CallbackToken,
    },
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
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegistryError {
    #[error("timer registration is no longer present")]
    UnknownRegistration,
    #[error("timer registration claim is stale")]
    StaleRegistration,
    #[error("timer operation is not legal for policy {actual}")]
    WrongPolicy { actual: &'static str },
    #[error("timer callback token is stale")]
    StaleCallback,
    #[error("timer registration has no ordinary callback")]
    MissingCallback,
    #[error("timer registration already owns the provider handle for this callback role")]
    ProviderHandleAlreadyOwned,
    #[error(transparent)]
    Schedule(#[from] ScheduleError),
}

pub type OrdinaryCallback = Rc<RefCell<Box<dyn FnMut(TimerContext) -> TimerFuture>>>;
pub type WatchdogCallback = Rc<RefCell<Box<dyn FnMut(TimerContext) -> WatchdogRunResult>>>;

enum EntryCallback {
    None,
    Ordinary(OrdinaryCallback),
    Watchdog(WatchdogCallback),
}

struct OwnedProviderHandle {
    callback_generation: u64,
    role: CallbackRole,
    handle: TimerHandle,
}

/// One temporarily detached exact provider capability.
pub struct ProviderHandle {
    token: CallbackToken,
    handle: TimerHandle,
}

impl ProviderHandle {
    pub fn into_parts(self) -> (CallbackToken, TimerHandle) {
        (self.token, self.handle)
    }
}

/// The at-most-two provider capabilities owned by one timer entry.
#[derive(Default)]
pub struct ProviderHandles {
    wakeup: Option<ProviderHandle>,
    work: Option<ProviderHandle>,
}

impl ProviderHandles {
    pub const fn from_parts(wakeup: Option<ProviderHandle>, work: Option<ProviderHandle>) -> Self {
        Self { wakeup, work }
    }

    pub const fn take_wakeup(&mut self) -> Option<ProviderHandle> {
        self.wakeup.take()
    }

    pub const fn take_work(&mut self) -> Option<ProviderHandle> {
        self.work.take()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PendingSchedule {
    deadline_ns: u64,
    requested_delay_ns: Option<u64>,
    mode: TimerSchedulingMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OrdinaryPending {
    Cancel,
    Unregister,
    Schedule(PendingSchedule),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogPending {
    Cancel,
    Ensure,
    Unregister,
}

#[derive(Debug)]
enum EntryControl {
    Ordinary {
        control: TimerControl,
        pending: Option<OrdinaryPending>,
        inactive_reason: InactiveReason,
    },
    Watchdog(WatchdogControl),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WatchdogState {
    Inactive,
    Scheduled {
        scheduler_generation: u64,
        deadline_ns: u64,
    },
    AwaitingWork {
        successor_generation: u64,
        successor_deadline_ns: u64,
        attempt_generation: u64,
        attempt_status: WatchdogAttemptStatus,
    },
}

#[derive(Debug)]
struct WatchdogControl {
    scheduler_generation: u64,
    attempt_generation: u64,
    request_sequence: u64,
    state: WatchdogState,
    pending: Option<WatchdogPending>,
    inactive_reason: InactiveReason,
}

impl Default for WatchdogControl {
    fn default() -> Self {
        Self {
            scheduler_generation: 0,
            attempt_generation: 0,
            request_sequence: 0,
            state: WatchdogState::Inactive,
            pending: None,
            inactive_reason: InactiveReason::NeverScheduled,
        }
    }
}

struct Entry {
    claim_generation: u64,
    policy: TimerPolicy,
    lifetime: DeclarationLifetime,
    control: EntryControl,
    scheduling_mode: TimerSchedulingMode,
    latest_directive: Option<TimerDirectiveSnapshot>,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    confirmed_wakeup_generation: Option<u64>,
    confirmed_work_generation: Option<u64>,
    callback: EntryCallback,
    wakeup: Option<OwnedProviderHandle>,
    work: Option<OwnedProviderHandle>,
    observability: TimerObservabilitySnapshot,
}

impl Entry {
    fn new(
        claim_generation: u64,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
        epoch: TimerEpoch,
        callback: EntryCallback,
    ) -> Self {
        let (control, scheduling_mode) = match policy {
            TimerPolicy::Once => (
                EntryControl::Ordinary {
                    control: TimerControl::default(),
                    pending: None,
                    inactive_reason: InactiveReason::NeverScheduled,
                },
                TimerSchedulingMode::Once,
            ),
            TimerPolicy::AfterCompletion { .. } => (
                EntryControl::Ordinary {
                    control: TimerControl::default(),
                    pending: None,
                    inactive_reason: InactiveReason::NeverScheduled,
                },
                TimerSchedulingMode::AfterCompletion,
            ),
            TimerPolicy::Watchdog { .. } => (
                EntryControl::Watchdog(WatchdogControl::default()),
                TimerSchedulingMode::Watchdog,
            ),
        };

        Self {
            claim_generation,
            policy,
            lifetime,
            control,
            scheduling_mode,
            latest_directive: None,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            confirmed_wakeup_generation: None,
            confirmed_work_generation: None,
            callback,
            wakeup: None,
            work: None,
            observability: TimerObservabilitySnapshot::new(epoch),
        }
    }

    const fn snapshot(&self, identity: TimerIdentity) -> TimerSnapshot {
        let state = match &self.control {
            EntryControl::Ordinary {
                control,
                inactive_reason,
                ..
            } => match control.registration() {
                crate::TimerRegistration::Unregistered => TimerRuntimeStateSnapshot::Inactive {
                    reason: *inactive_reason,
                },
                crate::TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                } => TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Scheduled {
                    generation,
                    deadline_ns,
                }),
                crate::TimerRegistration::Running { generation } => {
                    TimerRuntimeStateSnapshot::Ordinary(OrdinaryRuntimeStateSnapshot::Running {
                        generation,
                    })
                }
            },
            EntryControl::Watchdog(control) => match control.state {
                WatchdogState::Inactive => TimerRuntimeStateSnapshot::Inactive {
                    reason: control.inactive_reason,
                },
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
            self.policy,
            self.lifetime,
            state,
            self.scheduling_mode,
            self.latest_directive,
            self.latest_requested_delay_ns,
            self.latest_armed_delay_ns,
            &self.observability,
        )
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

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) const fn epoch(&self) -> TimerEpoch {
        self.epoch
    }

    pub(crate) fn register_once(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(identity, TimerPolicy::Once, lifetime, EntryCallback::None)
    }

    pub(crate) fn register_after_completion(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            TimerPolicy::AfterCompletion { cadence },
            lifetime,
            EntryCallback::None,
        )
    }

    pub(crate) fn register_watchdog(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            TimerPolicy::Watchdog { cadence },
            lifetime,
            EntryCallback::None,
        )
    }

    pub(crate) fn register_once_with_callback(
        &mut self,
        identity: TimerIdentity,
        lifetime: DeclarationLifetime,
        callback: OrdinaryCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            TimerPolicy::Once,
            lifetime,
            EntryCallback::Ordinary(callback),
        )
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
            TimerPolicy::AfterCompletion { cadence },
            lifetime,
            EntryCallback::Ordinary(callback),
        )
    }

    pub(crate) fn register_watchdog_with_callback(
        &mut self,
        identity: TimerIdentity,
        cadence: TimerCadence,
        lifetime: DeclarationLifetime,
        callback: WatchdogCallback,
    ) -> Result<RegistrationClaim, RegisterError> {
        self.register(
            identity,
            TimerPolicy::Watchdog { cadence },
            lifetime,
            EntryCallback::Watchdog(callback),
        )
    }

    fn register(
        &mut self,
        identity: TimerIdentity,
        policy: TimerPolicy,
        lifetime: DeclarationLifetime,
        callback: EntryCallback,
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
            Entry::new(claim_generation, policy, lifetime, self.epoch, callback),
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
        let resolved = schedule.resolve(now_ns)?;
        let mode = match schedule {
            TimerSchedule::After(_) => TimerSchedulingMode::Once,
            TimerSchedule::At(_) => TimerSchedulingMode::Deadline,
        };
        self.schedule_ordinary(
            claim,
            now_ns,
            PendingSchedule {
                deadline_ns: resolved.deadline_ns,
                requested_delay_ns: resolved.requested_delay_ns,
                mode,
            },
            true,
        )
    }

    pub(crate) fn ensure_recurring(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry(claim)?;
        let (cadence, mode, ordinary, already_scheduled) = match entry.policy {
            TimerPolicy::Once => {
                return Err(RegistryError::WrongPolicy { actual: "once" });
            }
            TimerPolicy::AfterCompletion { cadence } => {
                let already_scheduled = matches!(
                    entry.control,
                    EntryControl::Ordinary {
                        ref control,
                        ..
                    } if matches!(
                        control.registration(),
                        crate::TimerRegistration::Scheduled { .. }
                    )
                );
                (
                    cadence,
                    TimerSchedulingMode::AfterCompletion,
                    true,
                    already_scheduled,
                )
            }
            TimerPolicy::Watchdog { cadence } => {
                (cadence, TimerSchedulingMode::Watchdog, false, false)
            }
        };
        if ordinary {
            if already_scheduled {
                let entry = self.entry_mut(claim)?;
                entry.observability.counters_mut().record_schedule_request();
                entry.observability.counters_mut().record_coalesced();
                entry.latest_requested_delay_ns = Some(cadence.as_nanos());
                return Ok(RegistryTransition::normal(RegistryEffect::None));
            }
            let deadline_ns = cadence.deadline_after(now_ns)?;
            self.schedule_ordinary(
                claim,
                now_ns,
                PendingSchedule {
                    deadline_ns,
                    requested_delay_ns: Some(cadence.as_nanos()),
                    mode,
                },
                false,
            )
        } else {
            self.ensure_watchdog(claim, now_ns, cadence)
        }
    }

    fn schedule_ordinary(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        requested: PendingSchedule,
        require_once: bool,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        if require_once && !matches!(entry.policy, TimerPolicy::Once) {
            return Err(RegistryError::WrongPolicy {
                actual: entry.policy.label(),
            });
        }
        if !matches!(entry.control, EntryControl::Ordinary { .. }) {
            return Err(RegistryError::WrongPolicy {
                actual: entry.policy.label(),
            });
        }

        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = requested.requested_delay_ns;

        let (action, was_running) = match &mut entry.control {
            EntryControl::Ordinary { control, .. } => {
                let was_running = matches!(
                    control.registration(),
                    crate::TimerRegistration::Running { .. }
                );
                (control.schedule(requested.deadline_ns), was_running)
            }
            EntryControl::Watchdog(_) => {
                return Err(RegistryError::WrongPolicy {
                    actual: entry.policy.label(),
                });
            }
        };

        let action = match action {
            Ok(action) => action,
            Err(error) => {
                return Ok(terminal_ordinary(entry, claim.identity.clone(), error));
            }
        };

        if was_running {
            let EntryControl::Ordinary { pending, .. } = &mut entry.control else {
                return Err(RegistryError::WrongPolicy {
                    actual: entry.policy.label(),
                });
            };
            *pending = Some(match *pending {
                Some(OrdinaryPending::Unregister) => OrdinaryPending::Unregister,
                Some(OrdinaryPending::Schedule(current))
                    if current.deadline_ns <= requested.deadline_ns =>
                {
                    OrdinaryPending::Schedule(current)
                }
                Some(OrdinaryPending::Cancel | OrdinaryPending::Schedule(_)) | None => {
                    OrdinaryPending::Schedule(requested)
                }
            });
        }

        Ok(apply_ordinary_action(
            entry,
            claim.identity.clone(),
            now_ns,
            action,
            requested,
        ))
    }

    fn ensure_watchdog(
        &mut self,
        claim: &RegistrationClaim,
        now_ns: u64,
        cadence: TimerCadence,
    ) -> Result<RegistryTransition, RegistryError> {
        let entry = self.entry_mut(claim)?;
        if !matches!(entry.policy, TimerPolicy::Watchdog { .. }) {
            return Err(RegistryError::WrongPolicy {
                actual: entry.policy.label(),
            });
        }
        entry.observability.counters_mut().record_schedule_request();
        entry.latest_requested_delay_ns = Some(cadence.as_nanos());

        let EntryControl::Watchdog(control) = &mut entry.control else {
            return Err(RegistryError::WrongPolicy {
                actual: entry.policy.label(),
            });
        };
        match control.state {
            WatchdogState::Inactive => {
                let deadline_ns = cadence.deadline_after(now_ns)?;
                let Some(generation) = control.scheduler_generation.checked_add(1) else {
                    control.inactive_reason =
                        InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted);
                    return Ok(RegistryTransition::terminal(
                        RegistryEffect::None,
                        TimerControlFailure::GenerationExhausted,
                    ));
                };
                control.scheduler_generation = generation;
                control.state = WatchdogState::Scheduled {
                    scheduler_generation: generation,
                    deadline_ns,
                };
                control.pending = None;
                entry.scheduling_mode = TimerSchedulingMode::Watchdog;
                Ok(RegistryTransition::normal(RegistryEffect::ArmWakeup {
                    token: token_for(claim, generation, CallbackRole::WatchdogScheduler),
                    deadline_ns,
                    delay_ns: deadline_ns.saturating_sub(now_ns),
                    replace: false,
                }))
            }
            WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Dispatched,
                ..
            } => {
                entry.observability.counters_mut().record_coalesced();
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            WatchdogState::AwaitingWork {
                attempt_status: WatchdogAttemptStatus::Running,
                ..
            } => {
                let Some(sequence) = control.request_sequence.checked_add(1) else {
                    control.state = WatchdogState::Inactive;
                    control.inactive_reason = InactiveReason::ControlFailure(
                        TimerControlFailure::RequestSequenceExhausted,
                    );
                    return Ok(RegistryTransition::terminal(
                        clear_callbacks(claim.identity.clone(), true, false),
                        TimerControlFailure::RequestSequenceExhausted,
                    ));
                };
                control.request_sequence = sequence;
                if !matches!(control.pending, Some(WatchdogPending::Unregister)) {
                    control.pending = Some(WatchdogPending::Ensure);
                }
                entry.observability.counters_mut().record_coalesced();
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
        }
    }

    pub(crate) fn cancel(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity.clone();
        let (transition, remove) = {
            let entry = self.entry_mut(claim)?;
            match &mut entry.control {
                EntryControl::Ordinary {
                    control,
                    pending,
                    inactive_reason,
                } => {
                    let before = control.registration();
                    let action = match control.cancel() {
                        Ok(action) => action,
                        Err(error) => {
                            return Ok(terminal_ordinary(entry, identity.clone(), error));
                        }
                    };
                    let mut remove = false;
                    let transition = match (before, action) {
                        (crate::TimerRegistration::Unregistered, TimerControlAction::None) => {
                            RegistryTransition::normal(RegistryEffect::None)
                        }
                        (crate::TimerRegistration::Running { .. }, TimerControlAction::None) => {
                            if !matches!(*pending, Some(OrdinaryPending::Unregister)) {
                                *pending = Some(OrdinaryPending::Cancel);
                            }
                            RegistryTransition::normal(RegistryEffect::None)
                        }
                        (_, TimerControlAction::Clear) => {
                            *inactive_reason = InactiveReason::Cancelled;
                            entry.observability.counters_mut().record_cancellation();
                            remove =
                                matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                            RegistryTransition::normal(clear_callbacks(
                                identity.clone(),
                                true,
                                false,
                            ))
                        }
                        (crate::TimerRegistration::Scheduled { .. }, TimerControlAction::None)
                        | (
                            _,
                            TimerControlAction::Arm { .. }
                            | TimerControlAction::Replace { .. }
                            | TimerControlAction::Disarm { .. },
                        ) => {
                            let clear_wakeup = control.terminate();
                            *pending = None;
                            *inactive_reason = InactiveReason::ControlFailure(
                                TimerControlFailure::DirectiveNotAllowed,
                            );
                            RegistryTransition::terminal(
                                clear_callbacks(identity.clone(), clear_wakeup, false),
                                TimerControlFailure::DirectiveNotAllowed,
                            )
                        }
                    };
                    (transition, remove)
                }
                EntryControl::Watchdog(control) => {
                    let (transition, changed) = cancel_watchdog(control, &identity);
                    if changed && transition.failure().is_none() {
                        entry.observability.counters_mut().record_cancellation();
                    }
                    let stopped = changed || transition.failure().is_some();
                    (
                        transition,
                        stopped && matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped),
                    )
                }
            }
        };

        if remove {
            self.entries.remove(&identity);
        }
        Ok(transition)
    }

    /// Consume one logical claim and remove its declaration.
    ///
    /// Removal requested by running work is finalized by that work's normal
    /// completion. A trapping work message rolls the request back with the
    /// rest of its heap mutations.
    #[allow(clippy::needless_pass_by_value)] // Ownership consumption invalidates the claim.
    pub(crate) fn unregister(
        &mut self,
        claim: RegistrationClaim,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = claim.identity.clone();
        let running_role = {
            let entry = self.entry(&claim)?;
            match &entry.control {
                EntryControl::Ordinary { control, .. }
                    if matches!(
                        control.registration(),
                        crate::TimerRegistration::Running { .. }
                    ) =>
                {
                    Some(CallbackRole::OrdinaryWork)
                }
                EntryControl::Watchdog(control)
                    if matches!(
                        control.state,
                        WatchdogState::AwaitingWork {
                            attempt_status: WatchdogAttemptStatus::Running,
                            ..
                        }
                    ) =>
                {
                    Some(CallbackRole::WatchdogWork)
                }
                EntryControl::Ordinary { .. } | EntryControl::Watchdog(_) => None,
            }
        };

        match running_role {
            Some(CallbackRole::OrdinaryWork) => {
                let entry = self.entry_mut(&claim)?;
                let EntryControl::Ordinary {
                    control,
                    pending,
                    inactive_reason,
                } = &mut entry.control
                else {
                    return Err(RegistryError::WrongPolicy {
                        actual: entry.policy.label(),
                    });
                };
                let transition = match control.cancel() {
                    Ok(TimerControlAction::None) => {
                        *pending = Some(OrdinaryPending::Unregister);
                        RegistryTransition::normal(RegistryEffect::None)
                    }
                    Ok(_) => RegistryTransition::terminal(
                        RegistryEffect::None,
                        TimerControlFailure::DirectiveNotAllowed,
                    ),
                    Err(error) => {
                        let failure = map_control_failure(error);
                        let clear_wakeup = control.terminate();
                        *pending = None;
                        *inactive_reason = InactiveReason::ControlFailure(failure);
                        RegistryTransition::terminal(
                            clear_callbacks(identity.clone(), clear_wakeup, false),
                            failure,
                        )
                    }
                };
                if transition.failure().is_some() {
                    self.entries.remove(&identity);
                }
                Ok(transition)
            }
            Some(CallbackRole::WatchdogWork) => {
                let entry = self.entry_mut(&claim)?;
                let EntryControl::Watchdog(control) = &mut entry.control else {
                    return Err(RegistryError::WrongPolicy {
                        actual: entry.policy.label(),
                    });
                };
                let Some(sequence) = control.request_sequence.checked_add(1) else {
                    control.state = WatchdogState::Inactive;
                    control.inactive_reason = InactiveReason::ControlFailure(
                        TimerControlFailure::RequestSequenceExhausted,
                    );
                    let transition = RegistryTransition::terminal(
                        clear_callbacks(identity.clone(), true, false),
                        TimerControlFailure::RequestSequenceExhausted,
                    );
                    self.entries.remove(&identity);
                    return Ok(transition);
                };
                control.request_sequence = sequence;
                control.pending = Some(WatchdogPending::Unregister);
                Ok(RegistryTransition::normal(RegistryEffect::None))
            }
            Some(CallbackRole::WatchdogScheduler) => Err(RegistryError::StaleCallback),
            None => {
                let transition = self.cancel(&claim)?;
                self.entries.remove(&identity);
                Ok(transition)
            }
        }
    }

    pub(crate) fn begin_ordinary(&mut self, token: &CallbackToken) -> CallbackAcceptance {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return CallbackAcceptance::Stale;
        };
        if token.role != CallbackRole::OrdinaryWork
            || token.claim_generation != entry.claim_generation
        {
            entry.observability.counters_mut().record_stale_wakeup();
            return CallbackAcceptance::Stale;
        }
        let EntryControl::Ordinary { control, .. } = &mut entry.control else {
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

    #[allow(clippy::too_many_lines)] // One atomic policy transition; splitting obscures rollback state.
    pub(crate) fn complete_ordinary(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: TimerRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity.clone();
        let (transition, remove) = {
            let entry = self.entry_by_token_mut(token, CallbackRole::OrdinaryWork)?;
            let EntryControl::Ordinary {
                control,
                pending,
                inactive_reason,
            } = &mut entry.control
            else {
                return Err(RegistryError::StaleCallback);
            };
            if control.registration()
                != (crate::TimerRegistration::Running {
                    generation: token.callback_generation,
                })
            {
                return Err(RegistryError::StaleCallback);
            }

            let completion = result.completion();
            let pending_command = *pending;
            let terminal_pending = matches!(
                pending_command,
                Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister)
            );
            if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                let transition =
                    invariant_completion(entry, token.callback_generation, completion, now_ns);
                let remove = matches!(pending_command, Some(OrdinaryPending::Unregister))
                    || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                (transition, remove)
            } else if !terminal_pending
                && matches!(entry.policy, TimerPolicy::Once)
                && matches!(result.directive(), TimerDirective::RecurAfterCompletion)
            {
                let transition = terminal_completion(
                    entry,
                    token.callback_generation,
                    completion.work_count(),
                    now_ns,
                    TimerControlFailure::DirectiveNotAllowed,
                );
                let remove = matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                (transition, remove)
            } else {
                let effective_directive = if terminal_pending {
                    TimerDirective::Stop
                } else {
                    result.directive()
                };
                let cadence = match entry.policy {
                    TimerPolicy::AfterCompletion { cadence } => Some(cadence),
                    TimerPolicy::Once => None,
                    TimerPolicy::Watchdog { .. } => {
                        return Err(RegistryError::StaleCallback);
                    }
                };
                let resolved = match effective_directive.resolve(now_ns, cadence) {
                    Ok(value) => value,
                    Err(error) => {
                        let transition = terminal_completion(
                            entry,
                            token.callback_generation,
                            completion.work_count(),
                            now_ns,
                            map_schedule_failure(error),
                        );
                        let remove = matches!(pending_command, Some(OrdinaryPending::Unregister))
                            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                        return Ok(remove_after(
                            &mut self.entries,
                            &identity,
                            transition,
                            remove,
                        ));
                    }
                };
                let directive_snapshot = TimerDirectiveSnapshot::try_from(effective_directive)
                    .map_err(RegistryError::Schedule)?;
                let callback_schedule = resolved.deadline_ns.map(|deadline_ns| PendingSchedule {
                    deadline_ns,
                    requested_delay_ns: resolved.requested_delay_ns,
                    mode: directive_snapshot
                        .scheduling_mode()
                        .unwrap_or(entry.scheduling_mode),
                });
                let selected_schedule =
                    select_completion_schedule(pending_command, callback_schedule);
                let action = match control.complete(
                    token.callback_generation,
                    selected_schedule.map(|value| value.deadline_ns),
                ) {
                    Ok(action) => action,
                    Err(TimerControlError::StaleCompletion) => {
                        return Err(RegistryError::StaleCallback);
                    }
                    Err(error) => {
                        let transition = terminal_completion(
                            entry,
                            token.callback_generation,
                            completion.work_count(),
                            now_ns,
                            map_control_failure(error),
                        );
                        let remove = matches!(pending_command, Some(OrdinaryPending::Unregister))
                            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                        return Ok(remove_after(
                            &mut self.entries,
                            &identity,
                            transition,
                            remove,
                        ));
                    }
                };
                *pending = None;
                entry.latest_directive = Some(directive_snapshot);
                entry.observability.record_completion(completion, now_ns);

                let mut remove = false;
                let transition = match action {
                    TimerControlAction::Arm {
                        generation,
                        deadline_ns,
                    }
                    | TimerControlAction::Replace {
                        generation,
                        deadline_ns,
                    } => {
                        let selected = selected_schedule.unwrap_or(PendingSchedule {
                            deadline_ns,
                            requested_delay_ns: None,
                            mode: entry.scheduling_mode,
                        });
                        let replace = matches!(action, TimerControlAction::Replace { .. });
                        entry.scheduling_mode = selected.mode;
                        entry.latest_requested_delay_ns = selected.requested_delay_ns;
                        RegistryTransition::normal(RegistryEffect::ArmWakeup {
                            token: CallbackToken::new(
                                identity.clone(),
                                entry.claim_generation,
                                generation,
                                CallbackRole::OrdinaryWork,
                            ),
                            deadline_ns,
                            delay_ns: deadline_ns.saturating_sub(now_ns),
                            replace,
                        })
                    }
                    TimerControlAction::Disarm { cancelled } => {
                        *inactive_reason = if cancelled {
                            entry.observability.counters_mut().record_cancellation();
                            InactiveReason::Cancelled
                        } else {
                            InactiveReason::Stopped
                        };
                        remove = matches!(pending_command, Some(OrdinaryPending::Unregister))
                            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                        RegistryTransition::normal(RegistryEffect::None)
                    }
                    TimerControlAction::None | TimerControlAction::Clear => {
                        let clear_wakeup = control.terminate();
                        *inactive_reason = InactiveReason::ControlFailure(
                            TimerControlFailure::DirectiveNotAllowed,
                        );
                        remove = matches!(pending_command, Some(OrdinaryPending::Unregister))
                            || matches!(entry.lifetime, DeclarationLifetime::RemoveWhenStopped);
                        RegistryTransition::terminal(
                            clear_callbacks(identity.clone(), clear_wakeup, false),
                            TimerControlFailure::DirectiveNotAllowed,
                        )
                    }
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

    #[allow(clippy::too_many_lines)] // The bounded scheduler protocol is audited as one path.
    pub(crate) fn begin_watchdog_scheduler(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
    ) -> RegistryTransition {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return RegistryTransition::normal(RegistryEffect::None);
        };
        if token.role != CallbackRole::WatchdogScheduler
            || token.claim_generation != entry.claim_generation
        {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        }
        let TimerPolicy::Watchdog { cadence } = entry.policy else {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        };
        let EntryControl::Watchdog(control) = &mut entry.control else {
            entry.observability.counters_mut().record_stale_wakeup();
            return RegistryTransition::normal(RegistryEffect::None);
        };

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
            WatchdogState::Inactive
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
        let Some(successor_generation) = control.scheduler_generation.checked_add(1) else {
            control.state = WatchdogState::Inactive;
            control.inactive_reason =
                InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted);
            return RegistryTransition::terminal(
                clear_callbacks(token.identity.clone(), false, true),
                TimerControlFailure::GenerationExhausted,
            );
        };
        let Some(attempt_generation) = control.attempt_generation.checked_add(1) else {
            control.state = WatchdogState::Inactive;
            control.inactive_reason =
                InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted);
            return RegistryTransition::terminal(
                clear_callbacks(token.identity.clone(), false, true),
                TimerControlFailure::GenerationExhausted,
            );
        };
        let Ok(successor_deadline_ns) = cadence.deadline_after(now_ns) else {
            control.state = WatchdogState::Inactive;
            control.inactive_reason =
                InactiveReason::ControlFailure(TimerControlFailure::DeadlineOverflow);
            return RegistryTransition::terminal(
                clear_callbacks(token.identity.clone(), false, true),
                TimerControlFailure::DeadlineOverflow,
            );
        };

        control.scheduler_generation = successor_generation;
        control.attempt_generation = attempt_generation;
        control.state = WatchdogState::AwaitingWork {
            successor_generation,
            successor_deadline_ns,
            attempt_generation,
            attempt_status: WatchdogAttemptStatus::Dispatched,
        };
        control.pending = None;
        RegistryTransition::normal(RegistryEffect::DispatchWatchdog {
            successor: CallbackToken::new(
                token.identity.clone(),
                entry.claim_generation,
                successor_generation,
                CallbackRole::WatchdogScheduler,
            ),
            successor_deadline_ns,
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
        if token.role != CallbackRole::WatchdogWork
            || token.claim_generation != entry.claim_generation
        {
            entry.observability.counters_mut().record_stale_work();
            return CallbackAcceptance::Stale;
        }
        let EntryControl::Watchdog(control) = &mut entry.control else {
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
            WatchdogState::Inactive
            | WatchdogState::Scheduled { .. }
            | WatchdogState::AwaitingWork { .. } => {
                entry.observability.counters_mut().record_stale_work();
                CallbackAcceptance::Stale
            }
        }
    }

    pub(crate) fn complete_watchdog_work(
        &mut self,
        token: &CallbackToken,
        now_ns: u64,
        result: WatchdogRunResult,
    ) -> Result<RegistryTransition, RegistryError> {
        let identity = token.identity.clone();
        let (transition, remove) = {
            let entry = self.entry_by_token_mut(token, CallbackRole::WatchdogWork)?;
            let EntryControl::Watchdog(control) = &mut entry.control else {
                return Err(RegistryError::StaleCallback);
            };
            let (successor_generation, successor_deadline_ns) = match control.state {
                WatchdogState::AwaitingWork {
                    successor_generation,
                    successor_deadline_ns,
                    attempt_generation,
                    attempt_status: WatchdogAttemptStatus::Running,
                } if attempt_generation == token.callback_generation => {
                    (successor_generation, successor_deadline_ns)
                }
                WatchdogState::Inactive
                | WatchdogState::Scheduled { .. }
                | WatchdogState::AwaitingWork { .. } => {
                    return Err(RegistryError::StaleCallback);
                }
            };

            let completion = result.completion();
            entry.observability.record_completion(completion, now_ns);
            let decision = if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                WatchdogDecision::Stop
            } else {
                match control.pending {
                    Some(WatchdogPending::Cancel | WatchdogPending::Unregister) => {
                        WatchdogDecision::Stop
                    }
                    Some(WatchdogPending::Ensure) => WatchdogDecision::Continue,
                    None => result.decision(),
                }
            };
            let cancelled = matches!(control.pending, Some(WatchdogPending::Cancel));
            let unregister = matches!(control.pending, Some(WatchdogPending::Unregister));
            control.pending = None;

            match decision {
                WatchdogDecision::Continue => {
                    control.state = WatchdogState::Scheduled {
                        scheduler_generation: successor_generation,
                        deadline_ns: successor_deadline_ns,
                    };
                    (RegistryTransition::normal(RegistryEffect::None), false)
                }
                WatchdogDecision::Stop => {
                    control.state = WatchdogState::Inactive;
                    control.inactive_reason = if cancelled {
                        entry.observability.counters_mut().record_cancellation();
                        InactiveReason::Cancelled
                    } else if completion.outcome() == TimerCompletionOutcome::InvariantFailure {
                        InactiveReason::InvariantFailure
                    } else {
                        InactiveReason::Stopped
                    };
                    (
                        RegistryTransition::normal(clear_callbacks(identity.clone(), true, false)),
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

    pub(crate) fn snapshot(&self, identity: &TimerIdentity) -> Option<TimerSnapshot> {
        self.entries
            .get(identity)
            .map(|entry| entry.snapshot(identity.clone()))
    }

    pub(crate) fn snapshots(&self) -> Vec<TimerSnapshot> {
        self.entries
            .iter()
            .map(|(identity, entry)| entry.snapshot(identity.clone()))
            .collect()
    }

    pub(crate) fn consecutive_expected_failures(&self, identity: &TimerIdentity) -> Option<u64> {
        self.entries
            .get(identity)
            .map(|entry| entry.observability.consecutive_expected_failures())
    }

    pub(crate) fn record_scheduler_instructions(
        &mut self,
        token: &CallbackToken,
        instructions: u64,
    ) {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return;
        };
        if token.claim_generation == entry.claim_generation
            && token.role == CallbackRole::WatchdogScheduler
            && matches!(entry.control, EntryControl::Watchdog(_))
        {
            entry
                .observability
                .record_scheduler_instructions(instructions);
        }
    }

    pub(crate) fn record_work_instructions(&mut self, token: &CallbackToken, instructions: u64) {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return;
        };
        let role_matches = matches!(
            (&entry.control, token.role),
            (EntryControl::Ordinary { .. }, CallbackRole::OrdinaryWork)
                | (EntryControl::Watchdog(_), CallbackRole::WatchdogWork)
        );
        if token.claim_generation == entry.claim_generation && role_matches {
            entry.observability.record_work_instructions(instructions);
        }
    }

    pub(crate) fn fail_registration(
        &mut self,
        claim: &RegistrationClaim,
        failure: TimerControlFailure,
    ) -> Result<ProviderHandles, RegistryError> {
        let identity = claim.identity.clone();
        let (handles, remove) = {
            let entry = self.entry_mut(claim)?;
            match &mut entry.control {
                EntryControl::Ordinary {
                    control,
                    pending,
                    inactive_reason,
                } => {
                    control.terminate();
                    *pending = None;
                    *inactive_reason = InactiveReason::ControlFailure(failure);
                }
                EntryControl::Watchdog(control) => {
                    control.state = WatchdogState::Inactive;
                    control.pending = None;
                    control.inactive_reason = InactiveReason::ControlFailure(failure);
                }
            }
            let handles =
                ProviderHandles {
                    wakeup: entry.wakeup.take().map(|owned| {
                        detach_provider_handle(&identity, entry.claim_generation, owned)
                    }),
                    work: entry.work.take().map(|owned| {
                        detach_provider_handle(&identity, entry.claim_generation, owned)
                    }),
                };
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
        let entry = self
            .entries
            .get(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if token.role != CallbackRole::OrdinaryWork
            || token.claim_generation != entry.claim_generation
            || !matches!(
                entry.control,
                EntryControl::Ordinary {
                    ref control,
                    ..
                } if matches!(
                    control.registration(),
                    crate::TimerRegistration::Running { generation }
                        if generation == token.callback_generation
                )
            )
        {
            return Err(RegistryError::StaleCallback);
        }
        match &entry.callback {
            EntryCallback::Ordinary(callback) => Ok(Rc::clone(callback)),
            EntryCallback::None | EntryCallback::Watchdog(_) => Err(RegistryError::MissingCallback),
        }
    }

    pub(crate) fn watchdog_callback(
        &self,
        token: &CallbackToken,
    ) -> Result<WatchdogCallback, RegistryError> {
        let entry = self
            .entries
            .get(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if token.role != CallbackRole::WatchdogWork
            || token.claim_generation != entry.claim_generation
            || !matches!(
                entry.control,
                EntryControl::Watchdog(WatchdogControl {
                    state: WatchdogState::AwaitingWork {
                        attempt_generation,
                        attempt_status: WatchdogAttemptStatus::Running,
                        ..
                    },
                    ..
                }) if attempt_generation == token.callback_generation
            )
        {
            return Err(RegistryError::StaleCallback);
        }
        match &entry.callback {
            EntryCallback::Watchdog(callback) => Ok(Rc::clone(callback)),
            EntryCallback::None | EntryCallback::Ordinary(_) => Err(RegistryError::MissingCallback),
        }
    }

    pub(crate) fn install_provider_handle(
        &mut self,
        token: &CallbackToken,
        handle: TimerHandle,
    ) -> Result<(), (RegistryError, TimerHandle)> {
        let entry = match self.entry_by_token_mut(token, token.role) {
            Ok(entry) => entry,
            Err(error) => return Err((error, handle)),
        };
        let valid = match (&entry.control, token.role) {
            (EntryControl::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => matches!(
                control.registration(),
                crate::TimerRegistration::Scheduled { generation, .. }
                    if generation == token.callback_generation
            ),
            (EntryControl::Watchdog(control), CallbackRole::WatchdogScheduler) => {
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
            (EntryControl::Watchdog(control), CallbackRole::WatchdogWork) => matches!(
                control.state,
                WatchdogState::AwaitingWork {
                    attempt_generation,
                    attempt_status: WatchdogAttemptStatus::Dispatched,
                    ..
                } if attempt_generation == token.callback_generation
            ),
            (
                EntryControl::Ordinary { .. },
                CallbackRole::WatchdogScheduler | CallbackRole::WatchdogWork,
            )
            | (EntryControl::Watchdog(_), CallbackRole::OrdinaryWork) => false,
        };
        if !valid {
            return Err((RegistryError::StaleCallback, handle));
        }
        let slot = match token.role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler => &mut entry.wakeup,
            CallbackRole::WatchdogWork => &mut entry.work,
        };
        if slot.is_some() {
            return Err((RegistryError::ProviderHandleAlreadyOwned, handle));
        }
        *slot = Some(OwnedProviderHandle {
            callback_generation: token.callback_generation,
            role: token.role,
            handle,
        });
        Ok(())
    }

    pub(crate) fn take_wakeup_handle(
        &mut self,
        identity: &TimerIdentity,
    ) -> Option<ProviderHandle> {
        let entry = self.entries.get_mut(identity)?;
        let owned = entry.wakeup.take()?;
        Some(detach_provider_handle(
            identity,
            entry.claim_generation,
            owned,
        ))
    }

    pub(crate) fn take_work_handle(&mut self, identity: &TimerIdentity) -> Option<ProviderHandle> {
        let entry = self.entries.get_mut(identity)?;
        let owned = entry.work.take()?;
        Some(detach_provider_handle(
            identity,
            entry.claim_generation,
            owned,
        ))
    }

    pub(crate) fn take_provider_handles_for_claim(
        &mut self,
        claim: &RegistrationClaim,
    ) -> Result<ProviderHandles, RegistryError> {
        let identity = claim.identity.clone();
        let entry = self.entry_mut(claim)?;
        Ok(ProviderHandles {
            wakeup: entry
                .wakeup
                .take()
                .map(|owned| detach_provider_handle(&identity, entry.claim_generation, owned)),
            work: entry
                .work
                .take()
                .map(|owned| detach_provider_handle(&identity, entry.claim_generation, owned)),
        })
    }

    pub(crate) fn consume_provider_handle(&mut self, token: &CallbackToken) {
        let Some(entry) = self.entries.get_mut(token.identity()) else {
            return;
        };
        let slot = match token.role {
            CallbackRole::OrdinaryWork | CallbackRole::WatchdogScheduler => &mut entry.wakeup,
            CallbackRole::WatchdogWork => &mut entry.work,
        };
        let matches_token = slot.as_ref().is_some_and(|owned| {
            owned.callback_generation == token.callback_generation && owned.role == token.role
        });
        if matches_token {
            *slot = None;
        }
    }

    /// Confirm that the platform successfully applied one emitted arm effect.
    ///
    /// Patch 3 calls this synchronously after owning the returned provider
    /// handles. Pure tests use it to distinguish requested effects from actual
    /// provider operations.
    pub(crate) fn confirm_effect_applied(
        &mut self,
        effect: &RegistryEffect,
    ) -> Result<(), RegistryError> {
        match effect {
            RegistryEffect::None | RegistryEffect::ClearCallbacks { .. } => Ok(()),
            RegistryEffect::ArmWakeup {
                token, delay_ns, ..
            } => {
                let entry = self.entry_by_token_mut(token, token.role)?;
                let valid_generation = match (&entry.control, token.role) {
                    (EntryControl::Ordinary { control, .. }, CallbackRole::OrdinaryWork) => {
                        matches!(
                            control.registration(),
                            crate::TimerRegistration::Scheduled { generation, .. }
                                if generation == token.callback_generation
                        )
                    }
                    (EntryControl::Watchdog(control), CallbackRole::WatchdogScheduler) => matches!(
                        control.state,
                        WatchdogState::Scheduled {
                            scheduler_generation,
                            ..
                        } if scheduler_generation == token.callback_generation
                    ),
                    (EntryControl::Ordinary { .. } | EntryControl::Watchdog(_), _) => false,
                };
                if !valid_generation {
                    return Err(RegistryError::StaleCallback);
                }
                if entry.confirmed_wakeup_generation == Some(token.callback_generation) {
                    return Ok(());
                }
                entry.confirmed_wakeup_generation = Some(token.callback_generation);
                entry.latest_armed_delay_ns = Some(*delay_ns);
                entry.observability.counters_mut().record_wakeup_armed();
                Ok(())
            }
            RegistryEffect::DispatchWatchdog {
                successor,
                successor_delay_ns,
                work,
                ..
            } => {
                let successor_claim_generation = successor.claim_generation;
                let successor_callback_generation = successor.callback_generation;
                let successor_role = successor.role;
                let entry = self.entry_by_token_mut(successor, successor_role)?;
                if successor_role != CallbackRole::WatchdogScheduler
                    || work.role != CallbackRole::WatchdogWork
                    || work.claim_generation != successor_claim_generation
                {
                    return Err(RegistryError::StaleCallback);
                }
                let EntryControl::Watchdog(control) = &entry.control else {
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
                if entry.confirmed_wakeup_generation == Some(successor_callback_generation)
                    && entry.confirmed_work_generation == Some(work.callback_generation)
                {
                    return Ok(());
                }
                entry.confirmed_wakeup_generation = Some(successor_callback_generation);
                entry.confirmed_work_generation = Some(work.callback_generation);
                entry.latest_armed_delay_ns = Some(*successor_delay_ns);
                entry.observability.counters_mut().record_wakeup_armed();
                entry.observability.counters_mut().record_work_dispatched();
                Ok(())
            }
        }
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

    fn entry_by_token_mut(
        &mut self,
        token: &CallbackToken,
        role: CallbackRole,
    ) -> Result<&mut Entry, RegistryError> {
        let entry = self
            .entries
            .get_mut(token.identity())
            .ok_or(RegistryError::StaleCallback)?;
        if entry.claim_generation != token.claim_generation || token.role != role {
            return Err(RegistryError::StaleCallback);
        }
        Ok(entry)
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
    owned: OwnedProviderHandle,
) -> ProviderHandle {
    ProviderHandle {
        token: CallbackToken::new(
            identity.clone(),
            claim_generation,
            owned.callback_generation,
            owned.role,
        ),
        handle: owned.handle,
    }
}

const fn clear_callbacks(
    identity: TimerIdentity,
    clear_wakeup: bool,
    clear_work: bool,
) -> RegistryEffect {
    RegistryEffect::ClearCallbacks {
        identity,
        clear_wakeup,
        clear_work,
    }
}

fn apply_ordinary_action(
    entry: &mut Entry,
    identity: TimerIdentity,
    now_ns: u64,
    action: TimerControlAction,
    requested: PendingSchedule,
) -> RegistryTransition {
    match action {
        TimerControlAction::Arm {
            generation,
            deadline_ns,
        }
        | TimerControlAction::Replace {
            generation,
            deadline_ns,
        } => {
            let replace = matches!(action, TimerControlAction::Replace { .. });
            entry.scheduling_mode = requested.mode;
            RegistryTransition::normal(RegistryEffect::ArmWakeup {
                token: CallbackToken::new(
                    identity,
                    entry.claim_generation,
                    generation,
                    CallbackRole::OrdinaryWork,
                ),
                deadline_ns,
                delay_ns: deadline_ns.saturating_sub(now_ns),
                replace,
            })
        }
        TimerControlAction::None => {
            entry.observability.counters_mut().record_coalesced();
            RegistryTransition::normal(RegistryEffect::None)
        }
        TimerControlAction::Clear | TimerControlAction::Disarm { .. } => {
            terminal_ordinary(entry, identity, TimerControlError::StaleCompletion)
        }
    }
}

fn terminal_ordinary(
    entry: &mut Entry,
    identity: TimerIdentity,
    error: TimerControlError,
) -> RegistryTransition {
    let failure = map_control_failure(error);
    let EntryControl::Ordinary {
        control,
        pending,
        inactive_reason,
    } = &mut entry.control
    else {
        return RegistryTransition::terminal(RegistryEffect::None, failure);
    };
    let clear_wakeup = control.terminate();
    *pending = None;
    *inactive_reason = InactiveReason::ControlFailure(failure);
    RegistryTransition::terminal(
        RegistryEffect::ClearCallbacks {
            identity,
            clear_wakeup,
            clear_work: false,
        },
        failure,
    )
}

fn terminal_completion(
    entry: &mut Entry,
    generation: u64,
    work_count: u64,
    now_ns: u64,
    failure: TimerControlFailure,
) -> RegistryTransition {
    let EntryControl::Ordinary {
        control,
        pending,
        inactive_reason,
    } = &mut entry.control
    else {
        return RegistryTransition::terminal(RegistryEffect::None, failure);
    };
    if control.registration() == (crate::TimerRegistration::Running { generation }) {
        control.terminate();
    }
    *pending = None;
    *inactive_reason = InactiveReason::ControlFailure(failure);
    entry.latest_directive = Some(TimerDirectiveSnapshot::Stop);
    entry
        .observability
        .record_completion(TimerCompletion::invariant_failure(work_count), now_ns);
    RegistryTransition::terminal(RegistryEffect::None, failure)
}

fn invariant_completion(
    entry: &mut Entry,
    generation: u64,
    completion: TimerCompletion,
    now_ns: u64,
) -> RegistryTransition {
    let EntryControl::Ordinary {
        control,
        pending,
        inactive_reason,
    } = &mut entry.control
    else {
        return RegistryTransition::normal(RegistryEffect::None);
    };
    if control.registration() == (crate::TimerRegistration::Running { generation }) {
        control.terminate();
    }
    *pending = None;
    *inactive_reason = InactiveReason::InvariantFailure;
    entry.latest_directive = Some(TimerDirectiveSnapshot::Stop);
    entry.observability.record_completion(completion, now_ns);
    RegistryTransition::normal(RegistryEffect::None)
}

const fn map_control_failure(error: TimerControlError) -> TimerControlFailure {
    match error {
        TimerControlError::RequestSequenceExhausted => {
            TimerControlFailure::RequestSequenceExhausted
        }
        TimerControlError::GenerationExhausted => TimerControlFailure::GenerationExhausted,
        TimerControlError::StaleCompletion => TimerControlFailure::DirectiveNotAllowed,
    }
}

const fn map_schedule_failure(error: ScheduleError) -> TimerControlFailure {
    match error {
        ScheduleError::DeadlineOverflow => TimerControlFailure::DeadlineOverflow,
        ScheduleError::DelayOutOfRange => TimerControlFailure::DelayOutOfRange,
        ScheduleError::ZeroCadence | ScheduleError::MissingCadence => {
            TimerControlFailure::DirectiveNotAllowed
        }
    }
}

const fn select_completion_schedule(
    pending: Option<OrdinaryPending>,
    callback: Option<PendingSchedule>,
) -> Option<PendingSchedule> {
    match pending {
        Some(OrdinaryPending::Cancel | OrdinaryPending::Unregister) => None,
        Some(OrdinaryPending::Schedule(pending)) => match callback {
            Some(callback) if callback.deadline_ns < pending.deadline_ns => Some(callback),
            Some(_) | None => Some(pending),
        },
        None => callback,
    }
}

fn cancel_watchdog(
    control: &mut WatchdogControl,
    identity: &TimerIdentity,
) -> (RegistryTransition, bool) {
    match control.state {
        WatchdogState::Inactive => (RegistryTransition::normal(RegistryEffect::None), false),
        WatchdogState::AwaitingWork {
            attempt_status: WatchdogAttemptStatus::Running,
            ..
        } => {
            let Some(sequence) = control.request_sequence.checked_add(1) else {
                control.state = WatchdogState::Inactive;
                control.inactive_reason =
                    InactiveReason::ControlFailure(TimerControlFailure::RequestSequenceExhausted);
                return (
                    RegistryTransition::terminal(
                        clear_callbacks(identity.clone(), true, false),
                        TimerControlFailure::RequestSequenceExhausted,
                    ),
                    true,
                );
            };
            control.request_sequence = sequence;
            if !matches!(control.pending, Some(WatchdogPending::Unregister)) {
                control.pending = Some(WatchdogPending::Cancel);
            }
            (RegistryTransition::normal(RegistryEffect::None), false)
        }
        WatchdogState::Scheduled { .. }
        | WatchdogState::AwaitingWork {
            attempt_status: WatchdogAttemptStatus::Dispatched,
            ..
        } => {
            let clear_work = matches!(control.state, WatchdogState::AwaitingWork { .. });
            let Some(scheduler_generation) = control.scheduler_generation.checked_add(1) else {
                control.state = WatchdogState::Inactive;
                control.inactive_reason =
                    InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted);
                return (
                    RegistryTransition::terminal(
                        clear_callbacks(identity.clone(), true, clear_work),
                        TimerControlFailure::GenerationExhausted,
                    ),
                    true,
                );
            };
            let Some(attempt_generation) = control.attempt_generation.checked_add(1) else {
                control.state = WatchdogState::Inactive;
                control.inactive_reason =
                    InactiveReason::ControlFailure(TimerControlFailure::GenerationExhausted);
                return (
                    RegistryTransition::terminal(
                        clear_callbacks(identity.clone(), true, clear_work),
                        TimerControlFailure::GenerationExhausted,
                    ),
                    true,
                );
            };
            control.scheduler_generation = scheduler_generation;
            control.attempt_generation = attempt_generation;
            control.state = WatchdogState::Inactive;
            control.pending = None;
            control.inactive_reason = InactiveReason::Cancelled;
            (
                RegistryTransition::normal(clear_callbacks(identity.clone(), true, clear_work)),
                true,
            )
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
