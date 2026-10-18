use ic_timers::{
    TimerCadence, TimerIdentity, TimerSchedule, TimerStart, WatchdogContext, WatchdogRegistration,
    WatchdogRunResult, register_watchdog,
};
use std::time::Duration;

pub(super) fn install(
    slot: &mut Option<WatchdogRegistration>,
    identity: TimerIdentity,
    initial_delay_ns: u64,
    callback: impl FnMut(WatchdogContext) -> WatchdogRunResult + 'static,
) -> bool {
    if slot.is_some() {
        return false;
    }
    let cadence = TimerCadence::from_nanos(super::CADENCE_NS)
        .unwrap_or_else(|_| ic_cdk::trap("probe cadence is invalid"));
    let registration = register_watchdog(
        identity,
        cadence,
        TimerStart::Retained {
            initial: Some(TimerSchedule::After(Duration::from_nanos(initial_delay_ns))),
        },
        callback,
    )
    .unwrap_or_else(|_| ic_cdk::trap("watchdog registration failed"));
    *slot = Some(registration);
    true
}
