//! Canister-local owner and live timer execution.

use crate::{
    DeclarationLifetime, RegisterError, ScheduleError, TimerCadence, TimerCompletion,
    TimerControlFailure, TimerDirective, TimerEpoch, TimerIdentity, TimerRunResult, TimerSchedule,
    TimerSnapshot, WatchdogRunResult,
    platform::{self, TimerHandle},
    registry::{
        CallbackAcceptance, CallbackRole, CallbackToken, OrdinaryCallback, ProviderHandle,
        ProviderHandles, RegistrationClaim, RegistryEffect, RegistryError, RegistryTransition,
        TimerRegistry, WatchdogCallback,
    },
};
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc, time::Duration};
use thiserror::Error;

/// Erased future returned by an ordinary timer callback.
pub type TimerFuture = Pin<Box<dyn Future<Output = TimerRunResult>>>;

thread_local! {
    static RUNTIME: RefCell<Option<TimerRegistry>> = const { RefCell::new(None) };
}

/// Failure from the canister-local timer runtime API.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum TimerError {
    /// The lifecycle owner has not initialized the volatile runtime.
    #[error("timer runtime is not initialized")]
    NotInitialized,
    /// A nested internal borrow indicates unsupported re-entrancy.
    #[error("timer runtime is already borrowed")]
    RuntimeBusy,
    /// Registration failed before any provider callback was armed.
    #[error(transparent)]
    Register(#[from] RegisterError),
    /// Cadence or deadline validation failed.
    #[error(transparent)]
    Schedule(#[from] ScheduleError),
    /// The logical registration was removed or superseded.
    #[error("timer registration is no longer authoritative")]
    RegistrationExpired,
    /// An operation was attempted through the wrong policy-specific API.
    #[error("timer operation does not match its registered policy")]
    WrongPolicy,
    /// Pure checked control reached a terminal failure after effects were applied.
    #[error("timer control failed: {0:?}")]
    ControlFailure(TimerControlFailure),
    /// Canonical callback or provider-handle ownership was internally inconsistent.
    #[error("timer runtime ownership invariant failed")]
    OwnershipInvariant,
    /// A retained lifecycle claim no longer matches its canonical declaration.
    #[error("timer lifecycle reconciliation conflicts with the canonical declaration")]
    ReconciliationConflict,
}

impl From<RegistryError> for TimerError {
    fn from(value: RegistryError) -> Self {
        match value {
            RegistryError::UnknownRegistration
            | RegistryError::StaleRegistration
            | RegistryError::StaleCallback => Self::RegistrationExpired,
            RegistryError::WrongPolicy { .. } => Self::WrongPolicy,
            RegistryError::Schedule(error) => Self::Schedule(error),
            RegistryError::MissingCallback | RegistryError::ProviderHandleAlreadyOwned => {
                Self::OwnershipInvariant
            }
        }
    }
}

/// Initialize the volatile canister-local runtime once for this Wasm instance.
///
/// Repeated calls are idempotent and return the original epoch. This function
/// exports no lifecycle hook; the canister's existing lifecycle owner calls it.
pub fn initialize_runtime() -> Result<TimerEpoch, TimerError> {
    let epoch = TimerEpoch::new(platform::canister_version(), platform::time_ns());
    RUNTIME.with(|runtime| {
        let mut runtime = runtime
            .try_borrow_mut()
            .map_err(|_| TimerError::RuntimeBusy)?;
        if let Some(registry) = runtime.as_ref() {
            return Ok(registry.epoch());
        }
        *runtime = Some(TimerRegistry::new(epoch));
        Ok(epoch)
    })
}

/// Delegated control capability scoped to one exact consumer-work attempt.
///
/// Identity remains inspectable after work returns, but mutation methods then
/// return [`TimerError::RegistrationExpired`]. Retain the policy-specific
/// registration capability for longer-lived ownership.
pub struct TimerContext {
    token: CallbackToken,
}

impl TimerContext {
    const fn new(token: CallbackToken) -> Self {
        Self { token }
    }

    fn claim(&self) -> RegistrationClaim {
        RegistrationClaim::delegated(self.token.identity().clone(), self.token.claim_generation())
    }

    /// Return the logical timer identity executing this work.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.token.identity()
    }

    /// Schedule a `Once` declaration while this exact work attempt is active.
    ///
    /// A context retained after its callback completes is expired and returns
    /// [`TimerError::RegistrationExpired`].
    pub fn ensure_once(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(&self.claim(), Some(&self.token), schedule)
    }

    /// Reconcile the executing ordinary declaration to one exact schedule.
    ///
    /// `None` cancels live work while retaining callback authority. Watchdog
    /// declarations reject this operation. A retained context cannot mutate
    /// the registration after its exact work attempt ends.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(&self.claim(), Some(&self.token), schedule)
    }

    /// Ensure recurrence while this exact work attempt is active.
    pub fn ensure_recurring(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(&self.claim(), Some(&self.token))
    }

    /// Request cancellation while this exact work attempt is active.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(&self.claim(), Some(&self.token))
    }
}

/// Opaque non-clone claim for one registered `Once` callback.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct OnceRegistration {
    claim: RegistrationClaim,
}

impl OnceRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns a future provider wake-up.
    ///
    /// This is a volatile observation, not durable scheduling authority or a
    /// delivery guarantee. Call [`Self::ensure_scheduled`] unconditionally when
    /// a wake-up is required rather than using this value as a scheduling guard.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Synchronously ensure one callback is scheduled.
    pub fn ensure_scheduled(&self, schedule: TimerSchedule) -> Result<(), TimerError> {
        ensure_once_claim(&self.claim, None, schedule)
    }

    /// Reconcile to one exact desired schedule, replacing a later or earlier
    /// live deadline as necessary. `None` retains the declaration but cancels
    /// its live callback.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(&self.claim, None, schedule)
    }

    /// Cancel the current schedule while retaining callback authority when configured.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(&self.claim, None)
    }

    /// Consume the claim and unregister its callback authority.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(self.claim)
    }
}

/// Opaque non-clone claim for one after-completion callback.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct AfterCompletionRegistration {
    claim: RegistrationClaim,
}

/// Opaque non-clone claim for one pre-armed watchdog callback.
#[must_use = "retain the registration claim so the timer remains controllable"]
pub struct WatchdogRegistration {
    claim: RegistrationClaim,
}

/// Desired volatile scheduling state during synchronous lifecycle reconciliation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerReconcileState {
    /// Retain an existing declaration but clear its live callbacks.
    Inactive,
    /// Ensure one authoritative wake-up exists.
    Scheduled,
}

impl WatchdogRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns a future scheduler wake-up.
    ///
    /// The separately queued work callback is not itself a wake-up. During
    /// watchdog work this returns `true` only because the scheduler has already
    /// committed and installed the cadence successor. This is a volatile
    /// observation, not a delivery guarantee.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Synchronously ensure one watchdog scheduler wake-up is authoritative.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(&self.claim, None)
    }

    /// Cancel the scheduler and any dispatched work callback.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(&self.claim, None)
    }

    /// Consume the claim and unregister its callback authority.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(self.claim)
    }
}

impl AfterCompletionRegistration {
    /// Return the claimed logical identity.
    #[must_use]
    pub const fn identity(&self) -> &TimerIdentity {
        self.claim.identity()
    }

    /// Return whether this exact claim currently owns a future provider wake-up.
    ///
    /// A callback currently running without an installed successor returns
    /// `false`. This is a volatile observation, not durable scheduling authority
    /// or a delivery guarantee.
    pub fn has_armed_wakeup(&self) -> Result<bool, TimerError> {
        has_armed_wakeup_claim(&self.claim)
    }

    /// Synchronously ensure one callback is scheduled at the configured cadence.
    pub fn ensure_scheduled(&self) -> Result<(), TimerError> {
        ensure_recurring_claim(&self.claim, None)
    }

    /// Reconcile to one exact desired schedule without changing the configured
    /// after-completion cadence. `None` retains the declaration but cancels its
    /// live callback.
    pub fn reconcile_schedule(&self, schedule: Option<TimerSchedule>) -> Result<(), TimerError> {
        reconcile_ordinary_claim(&self.claim, None, schedule)
    }

    /// Cancel the current schedule while retaining callback authority when configured.
    pub fn cancel(&self) -> Result<(), TimerError> {
        cancel_claim(&self.claim, None)
    }

    /// Consume the claim and unregister its callback authority.
    pub fn unregister(self) -> Result<(), TimerError> {
        unregister_claim(self.claim)
    }
}

/// Register one asynchronous `Once` callback without scheduling it.
pub fn register_once<F, Fut>(
    identity: TimerIdentity,
    lifetime: DeclarationLifetime,
    mut callback: F,
) -> Result<OnceRegistration, TimerError>
where
    F: FnMut(TimerContext) -> Fut + 'static,
    Fut: Future<Output = TimerRunResult> + 'static,
{
    let callback: OrdinaryCallback = Rc::new(RefCell::new(Box::new(move |context| {
        Box::pin(callback(context))
    })));
    let claim = with_registry_mut(|registry| {
        registry
            .register_once_with_callback(identity, lifetime, callback)
            .map_err(TimerError::from)
    })?;
    Ok(OnceRegistration { claim })
}

/// Register one asynchronous fixed-cadence after-completion callback.
pub fn register_after_completion<F, Fut>(
    identity: TimerIdentity,
    cadence: TimerCadence,
    lifetime: DeclarationLifetime,
    mut callback: F,
) -> Result<AfterCompletionRegistration, TimerError>
where
    F: FnMut(TimerContext) -> Fut + 'static,
    Fut: Future<Output = TimerRunResult> + 'static,
{
    let callback: OrdinaryCallback = Rc::new(RefCell::new(Box::new(move |context| {
        Box::pin(callback(context))
    })));
    let claim = with_registry_mut(|registry| {
        registry
            .register_after_completion_with_callback(identity, cadence, lifetime, callback)
            .map_err(TimerError::from)
    })?;
    Ok(AfterCompletionRegistration { claim })
}

/// Register one synchronous pre-armed watchdog callback without scheduling it.
///
/// The callback cannot be async: it runs only in the work message after a
/// separate scheduler message has armed the next cadence successor. Its
/// `WatchdogDecision` either retains or clears that committed successor.
pub fn register_watchdog<F>(
    identity: TimerIdentity,
    cadence: TimerCadence,
    lifetime: DeclarationLifetime,
    callback: F,
) -> Result<WatchdogRegistration, TimerError>
where
    F: FnMut(TimerContext) -> WatchdogRunResult + 'static,
{
    let callback: WatchdogCallback = Rc::new(RefCell::new(Box::new(callback)));
    let claim = with_registry_mut(|registry| {
        registry
            .register_watchdog_with_callback(identity, cadence, lifetime, callback)
            .map_err(TimerError::from)
    })?;
    Ok(WatchdogRegistration { claim })
}

/// Reconstruct or reconcile one `Once` declaration synchronously.
///
/// `Some(schedule)` is authoritative and may move an existing deadline in
/// either direction. `None` retains an inactive declaration in the canonical
/// inventory, including on a fresh heap. Lifecycle reconciliation always owns
/// a [`DeclarationLifetime::Retained`] declaration; transient
/// `RemoveWhenStopped` callbacks use [`register_once`] directly.
pub fn reconcile_once<F, Fut>(
    registration: &mut Option<OnceRegistration>,
    identity: &TimerIdentity,
    desired: Option<TimerSchedule>,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(TimerContext) -> Fut + 'static,
    Fut: Future<Output = TimerRunResult> + 'static,
{
    if registration.is_none() {
        *registration = Some(register_once(
            identity.clone(),
            DeclarationLifetime::Retained,
            callback,
        )?);
    }
    verify_declaration(
        registration.as_ref().map(OnceRegistration::identity),
        identity,
        crate::TimerPolicy::Once,
    )?;
    registration
        .as_ref()
        .ok_or(TimerError::ReconciliationConflict)?
        .reconcile_schedule(desired)
}

/// Reconstruct or reconcile one after-completion declaration synchronously.
///
/// The consumer owns `registration` in volatile state. A fresh Wasm heap has
/// `None`, so this function installs callback authority before reconciling it
/// active or inactive. A repeated call reuses the exact claim and does not
/// replace its callback. The installed declaration is always retained;
/// transient `RemoveWhenStopped` recurrence uses
/// [`register_after_completion`] directly.
pub fn reconcile_after_completion<F, Fut>(
    registration: &mut Option<AfterCompletionRegistration>,
    identity: &TimerIdentity,
    cadence: TimerCadence,
    desired: TimerReconcileState,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(TimerContext) -> Fut + 'static,
    Fut: Future<Output = TimerRunResult> + 'static,
{
    if registration.is_none() {
        *registration = Some(register_after_completion(
            identity.clone(),
            cadence,
            DeclarationLifetime::Retained,
            callback,
        )?);
    }
    verify_declaration(
        registration
            .as_ref()
            .map(AfterCompletionRegistration::identity),
        identity,
        crate::TimerPolicy::AfterCompletion { cadence },
    )?;
    let registration = registration
        .as_ref()
        .ok_or(TimerError::ReconciliationConflict)?;
    match desired {
        TimerReconcileState::Inactive => registration.cancel(),
        TimerReconcileState::Scheduled => registration.ensure_scheduled(),
    }
}

/// Reconstruct or reconcile one watchdog declaration synchronously.
///
/// Durable readiness remains consumer-owned. Fresh inactive authority still
/// installs an observable retained declaration. This helper owns no lifecycle
/// export and persists no policy, generation, provider handle, or callback.
/// Transient `RemoveWhenStopped` watchdogs use [`register_watchdog`] directly.
pub fn reconcile_watchdog<F>(
    registration: &mut Option<WatchdogRegistration>,
    identity: &TimerIdentity,
    cadence: TimerCadence,
    desired: TimerReconcileState,
    callback: F,
) -> Result<(), TimerError>
where
    F: FnMut(TimerContext) -> WatchdogRunResult + 'static,
{
    if registration.is_none() {
        *registration = Some(register_watchdog(
            identity.clone(),
            cadence,
            DeclarationLifetime::Retained,
            callback,
        )?);
    }
    verify_declaration(
        registration.as_ref().map(WatchdogRegistration::identity),
        identity,
        crate::TimerPolicy::Watchdog { cadence },
    )?;
    let registration = registration
        .as_ref()
        .ok_or(TimerError::ReconciliationConflict)?;
    match desired {
        TimerReconcileState::Inactive => registration.cancel(),
        TimerReconcileState::Scheduled => registration.ensure_scheduled(),
    }
}

fn verify_declaration(
    claimed_identity: Option<&TimerIdentity>,
    identity: &TimerIdentity,
    policy: crate::TimerPolicy,
) -> Result<(), TimerError> {
    if claimed_identity != Some(identity) {
        return Err(TimerError::ReconciliationConflict);
    }
    let snapshot = timer_snapshot(identity)?.ok_or(TimerError::RegistrationExpired)?;
    if snapshot.policy() != policy || snapshot.lifetime() != DeclarationLifetime::Retained {
        return Err(TimerError::ReconciliationConflict);
    }
    Ok(())
}

/// Return one coherent inert snapshot by identity.
pub fn timer_snapshot(identity: &TimerIdentity) -> Result<Option<TimerSnapshot>, TimerError> {
    with_registry(|registry| Ok(registry.snapshot(identity)))
}

/// Return the complete bounded inventory in deterministic identity order.
pub fn timer_snapshots() -> Result<Vec<TimerSnapshot>, TimerError> {
    with_registry(|registry| Ok(registry.snapshots()))
}

/// Return functional expected-failure state by identity without a full inventory.
pub fn consecutive_expected_failures(identity: &TimerIdentity) -> Result<Option<u64>, TimerError> {
    with_registry(|registry| Ok(registry.consecutive_expected_failures(identity)))
}

fn has_armed_wakeup_claim(claim: &RegistrationClaim) -> Result<bool, TimerError> {
    with_registry(|registry| registry.has_armed_wakeup(claim).map_err(TimerError::from))
}

fn ensure_once_claim(
    claim: &RegistrationClaim,
    context: Option<&CallbackToken>,
    schedule: TimerSchedule,
) -> Result<(), TimerError> {
    let transition = with_registry_mut(|registry| {
        validate_context(registry, context)?;
        registry
            .ensure_once(claim, platform::time_ns(), schedule)
            .map_err(TimerError::from)
    })?;
    finish_claim_transition(claim, transition, ProviderHandles::default())
}

fn reconcile_ordinary_claim(
    claim: &RegistrationClaim,
    context: Option<&CallbackToken>,
    schedule: Option<TimerSchedule>,
) -> Result<(), TimerError> {
    if schedule.is_none() {
        let (handles, transition) = with_registry_mut(|registry| {
            validate_context(registry, context)?;
            registry
                .validate_ordinary_claim(claim)
                .map_err(TimerError::from)?;
            let handles = registry
                .take_provider_handles_for_claim(claim)
                .map_err(TimerError::from)?;
            let transition = registry
                .reconcile_ordinary(claim, platform::time_ns(), None)
                .map_err(TimerError::from)?;
            Ok((handles, transition))
        })?;
        return finish_claim_transition(claim, transition, handles);
    }
    let transition = with_registry_mut(|registry| {
        validate_context(registry, context)?;
        registry
            .reconcile_ordinary(claim, platform::time_ns(), schedule)
            .map_err(TimerError::from)
    })?;
    finish_claim_transition(claim, transition, ProviderHandles::default())
}

fn ensure_recurring_claim(
    claim: &RegistrationClaim,
    context: Option<&CallbackToken>,
) -> Result<(), TimerError> {
    let transition = with_registry_mut(|registry| {
        validate_context(registry, context)?;
        registry
            .ensure_recurring(claim, platform::time_ns())
            .map_err(TimerError::from)
    })?;
    finish_claim_transition(claim, transition, ProviderHandles::default())
}

fn cancel_claim(
    claim: &RegistrationClaim,
    context: Option<&CallbackToken>,
) -> Result<(), TimerError> {
    let (handles, transition) = with_registry_mut(|registry| {
        validate_context(registry, context)?;
        let handles = registry
            .take_provider_handles_for_claim(claim)
            .map_err(TimerError::from)?;
        let transition = registry.cancel(claim).map_err(TimerError::from)?;
        Ok((handles, transition))
    })?;
    finish_claim_transition(claim, transition, handles)
}

fn validate_context(
    registry: &TimerRegistry,
    context: Option<&CallbackToken>,
) -> Result<(), TimerError> {
    context.map_or(Ok(()), |token| {
        registry
            .validate_running_context(token)
            .map_err(TimerError::from)
    })
}

fn unregister_claim(claim: RegistrationClaim) -> Result<(), TimerError> {
    let cleanup_claim =
        RegistrationClaim::delegated(claim.identity().clone(), claim.claim_generation());
    let (handles, transition) = with_registry_mut(|registry| {
        let handles = registry
            .take_provider_handles_for_claim(&claim)
            .map_err(TimerError::from)?;
        let transition = registry.unregister(claim).map_err(TimerError::from)?;
        Ok((handles, transition))
    })?;
    finish_claim_transition(&cleanup_claim, transition, handles)
}

fn finish_claim_transition(
    claim: &RegistrationClaim,
    transition: RegistryTransition,
    handles: ProviderHandles,
) -> Result<(), TimerError> {
    match finish_transition(transition, handles) {
        result @ (Ok(()) | Err(TimerError::ControlFailure(_))) => result,
        Err(error) => match fail_claim_provider_binding(claim) {
            Ok(()) | Err(TimerError::RegistrationExpired) => Err(error),
            Err(cleanup_error) => Err(cleanup_error),
        },
    }
}

fn finish_transition(
    transition: RegistryTransition,
    handles: ProviderHandles,
) -> Result<(), TimerError> {
    let failure = transition.failure();
    let effect = transition.into_effect();
    apply_effect(&effect, handles)?;
    failure.map_or(Ok(()), |failure| Err(TimerError::ControlFailure(failure)))
}

fn apply_effect(effect: &RegistryEffect, mut handles: ProviderHandles) -> Result<(), TimerError> {
    match effect {
        RegistryEffect::None => restore_provider_handles(handles),
        RegistryEffect::ArmWakeup {
            token,
            delay_ns,
            replace,
            ..
        } => {
            let detached_wakeup = handles.take_wakeup();
            if *replace {
                let replaced = match detached_wakeup {
                    Some(handle) => Some(handle),
                    None => with_registry_mut(|registry| {
                        Ok(registry.take_wakeup_handle(token.identity()))
                    })?,
                };
                if let Some(replaced) = replaced {
                    clear_provider_handle(replaced);
                }
            } else if let Some(detached_wakeup) = detached_wakeup {
                restore_provider_handle(detached_wakeup)?;
            }
            if let Some(work) = handles.take_work() {
                restore_provider_handle(work)?;
            }
            arm_wakeup(token, *delay_ns, effect)
        }
        RegistryEffect::ClearCallbacks {
            identity,
            clear_wakeup,
            clear_work,
        } => {
            let detached_wakeup = handles.take_wakeup();
            if *clear_wakeup {
                let wakeup = match detached_wakeup {
                    Some(handle) => Some(handle),
                    None => {
                        with_registry_mut(|registry| Ok(registry.take_wakeup_handle(identity)))?
                    }
                };
                if let Some(wakeup) = wakeup {
                    clear_provider_handle(wakeup);
                }
            } else if let Some(wakeup) = detached_wakeup {
                restore_provider_handle(wakeup)?;
            }
            let detached_work = handles.take_work();
            if *clear_work {
                let work = match detached_work {
                    Some(handle) => Some(handle),
                    None => with_registry_mut(|registry| Ok(registry.take_work_handle(identity)))?,
                };
                if let Some(work) = work {
                    clear_provider_handle(work);
                }
            } else if let Some(work) = detached_work {
                restore_provider_handle(work)?;
            }
            restore_provider_handles(handles)
        }
        RegistryEffect::DispatchWatchdog {
            successor,
            successor_delay_ns,
            work,
            ..
        } => {
            if let Some(wakeup) = handles.take_wakeup() {
                clear_provider_handle(wakeup);
            }
            let replaced_work = handles.take_work().or(with_registry_mut(|registry| {
                Ok(registry.take_work_handle(successor.identity()))
            })?);
            if let Some(replaced_work) = replaced_work {
                clear_provider_handle(replaced_work);
            }
            dispatch_watchdog_effect(successor, *successor_delay_ns, work, effect)
        }
    }
}

fn arm_wakeup(
    token: &CallbackToken,
    delay_ns: u64,
    effect: &RegistryEffect,
) -> Result<(), TimerError> {
    let task_token = token.clone();
    let handle = platform::set_timer(Duration::from_nanos(delay_ns), async move {
        dispatch_wakeup(task_token).await;
    });
    if let Err((error, handle)) = install_provider_handle(token, handle) {
        platform::clear_timer(handle);
        return Err(error);
    }
    if let Err(error) = confirm_effect(effect) {
        let handle = with_registry_mut(|registry| {
            registry
                .take_wakeup_handle(token.identity())
                .ok_or(TimerError::OwnershipInvariant)
        })?;
        clear_provider_handle(handle);
        return Err(error);
    }
    Ok(())
}

fn dispatch_watchdog_effect(
    successor: &CallbackToken,
    successor_delay_ns: u64,
    work: &CallbackToken,
    effect: &RegistryEffect,
) -> Result<(), TimerError> {
    let successor_token = successor.clone();
    let successor_handle =
        platform::set_timer(Duration::from_nanos(successor_delay_ns), async move {
            dispatch_watchdog_scheduler(&successor_token);
        });
    if let Err((error, handle)) = install_provider_handle(successor, successor_handle) {
        platform::clear_timer(handle);
        return Err(error);
    }

    let work_token = work.clone();
    let work_handle = platform::set_timer(Duration::ZERO, async move {
        dispatch_watchdog_work(&work_token);
    });
    if let Err((error, handle)) = install_provider_handle(work, work_handle) {
        platform::clear_timer(handle);
        clear_entry_provider_handles(successor.identity())?;
        return Err(error);
    }
    if let Err(error) = confirm_effect(effect) {
        clear_entry_provider_handles(successor.identity())?;
        return Err(error);
    }
    Ok(())
}

fn install_provider_handle(
    token: &CallbackToken,
    handle: TimerHandle,
) -> Result<(), (TimerError, TimerHandle)> {
    #[cfg(test)]
    if take_provider_install_fault() {
        return Err((TimerError::OwnershipInvariant, handle));
    }
    RUNTIME.with(|runtime| {
        let Ok(mut runtime) = runtime.try_borrow_mut() else {
            return Err((TimerError::RuntimeBusy, handle));
        };
        let Some(registry) = runtime.as_mut() else {
            return Err((TimerError::NotInitialized, handle));
        };
        match registry.install_provider_handle(token, handle) {
            Ok(()) => Ok(()),
            Err((error, handle)) => Err((TimerError::from(error), handle)),
        }
    })
}

fn confirm_effect(effect: &RegistryEffect) -> Result<(), TimerError> {
    #[cfg(test)]
    if take_provider_confirmation_fault() {
        return Err(TimerError::OwnershipInvariant);
    }
    with_registry_mut(|registry| {
        registry
            .confirm_effect_applied(effect)
            .map_err(TimerError::from)
    })
}

fn restore_provider_handles(mut handles: ProviderHandles) -> Result<(), TimerError> {
    if let Some(wakeup) = handles.take_wakeup() {
        restore_provider_handle(wakeup)?;
    }
    if let Some(work) = handles.take_work() {
        restore_provider_handle(work)?;
    }
    Ok(())
}

fn restore_provider_handle(handle: ProviderHandle) -> Result<(), TimerError> {
    let (token, handle) = handle.into_parts();
    match install_provider_handle(&token, handle) {
        Ok(()) => Ok(()),
        Err((error, handle)) => {
            platform::clear_timer(handle);
            Err(error)
        }
    }
}

fn clear_provider_handle(handle: ProviderHandle) {
    let (_, handle) = handle.into_parts();
    platform::clear_timer(handle);
}

fn clear_entry_provider_handles(identity: &TimerIdentity) -> Result<(), TimerError> {
    let mut handles = with_registry_mut(|registry| {
        Ok(ProviderHandles::from_parts(
            registry.take_wakeup_handle(identity),
            registry.take_work_handle(identity),
        ))
    })?;
    if let Some(wakeup) = handles.take_wakeup() {
        clear_provider_handle(wakeup);
    }
    if let Some(work) = handles.take_work() {
        clear_provider_handle(work);
    }
    Ok(())
}

#[allow(clippy::future_not_send)] // IC callbacks and canister-local state are single-threaded.
async fn dispatch_wakeup(token: CallbackToken) {
    match token.role() {
        CallbackRole::OrdinaryWork => dispatch_ordinary(token).await,
        CallbackRole::WatchdogScheduler => dispatch_watchdog_scheduler(&token),
        CallbackRole::WatchdogWork => {}
    }
}

#[allow(clippy::future_not_send)] // IC callbacks and canister-local state are single-threaded.
async fn dispatch_ordinary(token: CallbackToken) {
    let instructions_before = platform::instruction_counter();
    let accepted = with_registry_mut(|registry| {
        registry.consume_provider_handle(&token);
        Ok(registry.begin_ordinary(&token))
    });
    match accepted {
        Ok(CallbackAcceptance::Accepted) => {}
        Ok(CallbackAcceptance::Stale) => return,
        Err(error) => trap_callback_failure("ordinary callback acceptance", &error),
    }

    let callback = match with_registry(|registry| {
        registry.ordinary_callback(&token).map_err(TimerError::from)
    }) {
        Ok(callback) => callback,
        Err(TimerError::OwnershipInvariant) => {
            fail_ordinary_dispatch(&token);
            return;
        }
        Err(error) => trap_callback_failure("ordinary callback lookup", &error),
    };
    let context = TimerContext::new(token.clone());
    let future = {
        let Ok(mut callback) = callback.try_borrow_mut() else {
            fail_ordinary_dispatch(&token);
            return;
        };
        callback(context)
    };
    let result = future.await;
    let transition = with_registry_mut(|registry| {
        registry
            .complete_ordinary(&token, platform::time_ns(), result)
            .map_err(TimerError::from)
    });
    let transition = transition
        .unwrap_or_else(|error| trap_callback_failure("ordinary callback completion", &error));
    finish_callback_transition(&token, transition, ProviderHandles::default());
    record_work_instructions(&token, instructions_before);
}

fn fail_ordinary_dispatch(token: &CallbackToken) {
    let transition = with_registry_mut(|registry| {
        registry
            .complete_ordinary(
                token,
                platform::time_ns(),
                TimerRunResult::new(TimerCompletion::invariant_failure(0), TimerDirective::Stop),
            )
            .map_err(TimerError::from)
    });
    let transition = transition.unwrap_or_else(|error| {
        trap_callback_failure("ordinary invariant-failure completion", &error)
    });
    finish_callback_transition(token, transition, ProviderHandles::default());
}

fn dispatch_watchdog_scheduler(token: &CallbackToken) {
    let instructions_before = platform::instruction_counter();
    let transition = with_registry_mut(|registry| {
        registry.consume_provider_handle(token);
        Ok(registry.begin_watchdog_scheduler(token, platform::time_ns()))
    });
    let transition = transition
        .unwrap_or_else(|error| trap_callback_failure("watchdog scheduler transition", &error));
    let accepted = !matches!(transition.effect(), RegistryEffect::None);
    finish_callback_transition(token, transition, ProviderHandles::default());
    if accepted {
        let instructions = platform::instruction_counter().saturating_sub(instructions_before);
        with_registry_mut(|registry| {
            registry.record_scheduler_instructions(token, instructions);
            Ok(())
        })
        .unwrap_or_else(|error| {
            trap_callback_failure("watchdog scheduler instruction accounting", &error)
        });
    }
}

fn dispatch_watchdog_work(token: &CallbackToken) {
    let instructions_before = platform::instruction_counter();
    let accepted = with_registry_mut(|registry| {
        registry.consume_provider_handle(token);
        Ok(registry.begin_watchdog_work(token))
    });
    match accepted {
        Ok(CallbackAcceptance::Accepted) => {}
        Ok(CallbackAcceptance::Stale) => return,
        Err(error) => trap_callback_failure("watchdog work acceptance", &error),
    }

    let callback =
        match with_registry(|registry| registry.watchdog_callback(token).map_err(TimerError::from))
        {
            Ok(callback) => callback,
            Err(error) => trap_callback_failure("watchdog callback lookup", &error),
        };
    let context = TimerContext::new(token.clone());
    let result = {
        let Ok(mut callback) = callback.try_borrow_mut() else {
            trap_callback_failure(
                "watchdog callback ownership",
                &TimerError::OwnershipInvariant,
            );
        };
        callback(context)
    };
    finish_watchdog_dispatch(token, result);
    record_work_instructions(token, instructions_before);
}

fn finish_watchdog_dispatch(token: &CallbackToken, result: WatchdogRunResult) {
    let claim = RegistrationClaim::delegated(token.identity().clone(), token.claim_generation());
    let completed = with_registry_mut(|registry| {
        let handles = registry
            .take_provider_handles_for_claim(&claim)
            .map_err(TimerError::from)?;
        #[cfg(test)]
        {
            if take_watchdog_completion_fault() {
                return Err(TimerError::OwnershipInvariant);
            }
        }
        let transition = registry
            .complete_watchdog_work(token, platform::time_ns(), result)
            .map_err(TimerError::from)?;
        Ok((transition, handles))
    });
    let (transition, handles) =
        completed.unwrap_or_else(|error| trap_callback_failure("watchdog work completion", &error));
    finish_callback_transition(token, transition, handles);
}

fn finish_callback_transition(
    token: &CallbackToken,
    transition: RegistryTransition,
    handles: ProviderHandles,
) {
    match finish_transition(transition, handles) {
        Ok(()) | Err(TimerError::ControlFailure(_)) => {}
        Err(
            error @ (TimerError::NotInitialized
            | TimerError::RuntimeBusy
            | TimerError::Register(_)
            | TimerError::Schedule(_)
            | TimerError::RegistrationExpired
            | TimerError::WrongPolicy
            | TimerError::OwnershipInvariant
            | TimerError::ReconciliationConflict),
        ) => {
            if token.role() == CallbackRole::WatchdogWork {
                trap_callback_failure("watchdog provider-handle completion", &error);
            }
            fail_provider_binding(token).unwrap_or_else(|binding_error| {
                trap_callback_failure("provider-binding failure cleanup", &binding_error)
            });
        }
    }
}

fn fail_provider_binding(token: &CallbackToken) -> Result<(), TimerError> {
    let claim = RegistrationClaim::delegated(token.identity().clone(), token.claim_generation());
    fail_claim_provider_binding(&claim)
}

fn fail_claim_provider_binding(claim: &RegistrationClaim) -> Result<(), TimerError> {
    let failed = with_registry_mut(|registry| {
        registry
            .fail_registration(claim, TimerControlFailure::ProviderBindingFailed)
            .map_err(TimerError::from)
    });
    let mut handles = failed?;
    if let Some(wakeup) = handles.take_wakeup() {
        clear_provider_handle(wakeup);
    }
    if let Some(work) = handles.take_work() {
        clear_provider_handle(work);
    }
    Ok(())
}

fn record_work_instructions(token: &CallbackToken, instructions_before: u64) {
    let instructions = platform::instruction_counter().saturating_sub(instructions_before);
    with_registry_mut(|registry| {
        registry.record_work_instructions(token, instructions);
        Ok(())
    })
    .unwrap_or_else(|error| trap_callback_failure("work instruction accounting", &error));
}

fn trap_callback_failure(context: &str, error: &TimerError) -> ! {
    platform::trap(&format!("ic-timers {context} failed: {error}"))
}

fn with_registry<T>(
    operation: impl FnOnce(&TimerRegistry) -> Result<T, TimerError>,
) -> Result<T, TimerError> {
    RUNTIME.with(|runtime| {
        let runtime = runtime.try_borrow().map_err(|_| TimerError::RuntimeBusy)?;
        let registry = runtime.as_ref().ok_or(TimerError::NotInitialized)?;
        operation(registry)
    })
}

fn with_registry_mut<T>(
    operation: impl FnOnce(&mut TimerRegistry) -> Result<T, TimerError>,
) -> Result<T, TimerError> {
    RUNTIME.with(|runtime| {
        let mut runtime = runtime
            .try_borrow_mut()
            .map_err(|_| TimerError::RuntimeBusy)?;
        let registry = runtime.as_mut().ok_or(TimerError::NotInitialized)?;
        operation(registry)
    })
}

#[cfg(test)]
fn reset_for_test(now_ns: u64, canister_version: u64) {
    platform::reset(now_ns, canister_version);
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.set(false));
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| fault.set(None));
    PROVIDER_CONFIRMATION_FAULT.with(|fault| fault.set(false));
    RUNTIME.with(|runtime| {
        *runtime.borrow_mut() = None;
    });
}

#[cfg(test)]
thread_local! {
    static WATCHDOG_COMPLETION_FAULT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static PROVIDER_INSTALL_FAULT_AFTER: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) };
    static PROVIDER_CONFIRMATION_FAULT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
fn inject_watchdog_completion_fault() {
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.set(true));
}

#[cfg(test)]
fn take_watchdog_completion_fault() -> bool {
    WATCHDOG_COMPLETION_FAULT.with(|fault| fault.replace(false))
}

#[cfg(test)]
fn inject_provider_install_fault() {
    inject_provider_install_fault_after(0);
}

#[cfg(test)]
fn inject_provider_install_fault_after(successful_installs: u64) {
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| fault.set(Some(successful_installs)));
}

#[cfg(test)]
fn take_provider_install_fault() -> bool {
    PROVIDER_INSTALL_FAULT_AFTER.with(|fault| match fault.get() {
        Some(0) => {
            fault.set(None);
            true
        }
        Some(remaining) => {
            fault.set(Some(remaining - 1));
            false
        }
        None => false,
    })
}

#[cfg(test)]
fn inject_provider_confirmation_fault() {
    PROVIDER_CONFIRMATION_FAULT.with(|fault| fault.set(true));
}

#[cfg(test)]
fn take_provider_confirmation_fault() -> bool {
    PROVIDER_CONFIRMATION_FAULT.with(|fault| fault.replace(false))
}

#[cfg(test)]
mod tests;
