//! Apply registry effects while restoring or clearing every linear provider handle.
//! Captures are released only after the registry borrow and provider cleanup end.

#[cfg(test)]
use super::take_provider_install_fault;
use super::{
    CallbackToken, ProviderHandle, ProviderHandles, RUNTIME, RegistryEffect, RegistryTransition,
    TimerError, TimerHandle, TimerIdentity, dispatch_wakeup, dispatch_watchdog_scheduler,
    dispatch_watchdog_work, platform, with_registry_mut,
};
use std::time::Duration;

pub(super) fn finish_transition(
    transition: RegistryTransition,
    handles: ProviderHandles,
) -> Result<(), TimerError> {
    let failure = transition.failure();
    let applied = apply_effect(transition.effect(), handles);
    // Callback capture destructors may inspect or control other timers. Drop
    // them after both the registry borrow and provider cleanup have ended.
    drop(transition);
    applied?;
    failure.map_or(Ok(()), |failure| Err(TimerError::ControlFailure(failure)))
}

pub(super) fn apply_effect(
    effect: &RegistryEffect,
    mut handles: ProviderHandles,
) -> Result<(), TimerError> {
    match effect {
        RegistryEffect::None => restore_provider_handles(handles),
        RegistryEffect::ArmWakeup {
            token,
            delay_ns,
            arm,
        } => {
            if arm.replaces_existing()
                && let Some(replaced) = handles.take_wakeup()
            {
                clear_provider_handle(replaced);
            }
            restore_provider_handles(handles)?;
            let handle = platform::try_set_timer(
                Duration::from_nanos(*delay_ns),
                dispatch_wakeup(token.clone()),
            )?;
            bind_provider_handle(token.token(), handle, Some(*delay_ns))
        }
        RegistryEffect::ClearCallbacks { handles: selected } => {
            if selected.includes_wakeup()
                && let Some(wakeup) = handles.take_wakeup()
            {
                clear_provider_handle(wakeup);
            }
            if selected.includes_work()
                && let Some(work) = handles.take_work()
            {
                clear_provider_handle(work);
            }
            restore_provider_handles(handles)
        }
        RegistryEffect::DispatchWatchdog {
            dispatch,
            successor_delay_ns,
        } => {
            let successor = dispatch.successor();
            let work = dispatch.work();
            if let Some(wakeup) = handles.take_wakeup() {
                clear_provider_handle(wakeup);
            }
            if let Some(replaced_work) = handles.take_work() {
                clear_provider_handle(replaced_work);
            }
            let successor_token = successor.clone();
            let successor_handle =
                platform::try_set_timer(Duration::from_nanos(*successor_delay_ns), async move {
                    dispatch_watchdog_scheduler(&successor_token);
                })?;
            bind_provider_handle(successor, successor_handle, None)?;

            let work_token = work.clone();
            let work_handle = platform::try_set_timer(Duration::ZERO, async move {
                dispatch_watchdog_work(&work_token);
            })?;
            if let Err(error) = bind_provider_handle(&work, work_handle, Some(*successor_delay_ns))
            {
                clear_entry_provider_handles(successor.identity())?;
                return Err(error);
            }
            Ok(())
        }
    }
}

/// Consume a provider handle by installing it or clearing it on rejection.
pub(super) fn bind_provider_handle(
    token: &CallbackToken,
    handle: TimerHandle,
    armed_delay_ns: Option<u64>,
) -> Result<(), TimerError> {
    RUNTIME
        .with(|runtime| {
            #[cfg(test)]
            if take_provider_install_fault() {
                return Err((TimerError::OwnershipInvariant, handle));
            }
            let Ok(mut runtime) = runtime.try_borrow_mut() else {
                return Err((TimerError::RuntimeBusy, handle));
            };
            let Some(registry) = runtime.as_mut() else {
                return Err((TimerError::NotInitialized, handle));
            };
            match registry.install_provider_handle(token, handle, armed_delay_ns) {
                Ok(()) => Ok(()),
                Err((error, handle)) => Err((TimerError::from(error), handle)),
            }
        })
        .map_err(|(error, handle)| {
            // Release the registry borrow before calling the provider.
            platform::clear_timer(handle);
            error
        })
}

pub(super) fn restore_provider_handles(mut handles: ProviderHandles) -> Result<(), TimerError> {
    // Every detached linear capability must be restored or cleared even when
    // restoring an earlier handle fails.
    let wakeup_failure = handles
        .take_wakeup()
        .and_then(|handle| restore_provider_handle(handle).err());
    let work_failure = handles
        .take_work()
        .and_then(|handle| restore_provider_handle(handle).err());
    wakeup_failure.or(work_failure).map_or(Ok(()), Err)
}

fn restore_provider_handle(handle: ProviderHandle) -> Result<(), TimerError> {
    let (token, handle) = handle.into_parts();
    bind_provider_handle(&token, handle, None)
}

pub(super) fn clear_provider_handle(handle: ProviderHandle) {
    let (_, handle) = handle.into_parts();
    platform::clear_timer(handle);
}

pub(super) fn clear_provider_handles(mut handles: ProviderHandles) {
    if let Some(wakeup) = handles.take_wakeup() {
        clear_provider_handle(wakeup);
    }
    if let Some(work) = handles.take_work() {
        clear_provider_handle(work);
    }
}

pub(super) fn clear_entry_provider_handles(identity: &TimerIdentity) -> Result<(), TimerError> {
    let handles = with_registry_mut(|registry| Ok(registry.take_provider_handles(identity)))?;
    clear_provider_handles(handles);
    Ok(())
}
