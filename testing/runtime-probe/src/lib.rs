use candid::CandidType;
use ic_timers::{
    DeclarationLifetime, MAX_TIMER_REGISTRATIONS, MemoryPageSummary, TimerCadence, TimerCompletion,
    TimerIdentity, TimerLastOutcome, TimerSchedulingMode, WatchdogDecision, WatchdogRegistration,
    WatchdogRunResult, initialize_runtime, register_watchdog, timer_inventory, timer_snapshot,
};
use serde::Deserialize;
use std::cell::{Cell, RefCell};

const CADENCE_NS: u64 = 1_000_000_000;

thread_local! {
    static REGISTRATION: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    static SECONDARY_REGISTRATION: RefCell<Option<WatchdogRegistration>> = const { RefCell::new(None) };
    static EXTRA_REGISTRATIONS: RefCell<Vec<WatchdogRegistration>> = const { RefCell::new(Vec::new()) };
    static COMPLETED_WORK: Cell<u64> = const { Cell::new(0) };
    static SECONDARY_COMPLETED_WORK: Cell<u64> = const { Cell::new(0) };
    static TRAP_WINDOW: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
    static EXHAUST_WINDOW: Cell<Option<(u64, u64)>> = const { Cell::new(None) };
    static STOP_ON_NEXT_WORK: Cell<bool> = const { Cell::new(false) };
    static CONTINUE_IMMEDIATELY_ON_NEXT_WORK: Cell<bool> = const { Cell::new(false) };
    static DESIRED_SCHEDULED: Cell<bool> = const { Cell::new(false) };
    static SECONDARY_DESIRED_SCHEDULED: Cell<bool> = const { Cell::new(false) };
    static POST_UPGRADE_RECONSTRUCTED: Cell<bool> = const { Cell::new(false) };
}

#[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
struct ProbeMemorySummary {
    samples: u64,
    latest_wasm_start_pages: u64,
    latest_wasm_end_pages: u64,
    latest_stable_start_pages: u64,
    latest_stable_end_pages: u64,
    maximum_wasm_growth_pages: u64,
    maximum_stable_growth_pages: u64,
}

#[derive(CandidType, Debug, Deserialize, Eq, PartialEq)]
struct ProbeSnapshot {
    registered: bool,
    completed_work: u64,
    next_deadline_ns: Option<u64>,
    immediate_scheduling: bool,
    latest_requested_delay_ns: Option<u64>,
    latest_armed_delay_ns: Option<u64>,
    schedule_requests: u64,
    scheduler_started: u64,
    wakeups_armed: u64,
    work_dispatched: u64,
    work_started: u64,
    work_completed: u64,
    coalesced: u64,
    unacknowledged: u64,
    last_unacknowledged: bool,
    scheduler_instruction_samples: u64,
    scheduler_instruction_total: u64,
    work_instruction_samples: u64,
    work_instruction_total: u64,
    scheduler_memory: Option<ProbeMemorySummary>,
    work_memory: Option<ProbeMemorySummary>,
    post_upgrade_reconstructed: bool,
    secondary_registered: bool,
    secondary_completed_work: u64,
}

#[ic_cdk::init]
fn init() {
    if initialize_runtime().is_err() {
        ic_cdk::trap("ic-timers runtime initialization failed");
    }
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    let state = (
        DESIRED_SCHEDULED.with(Cell::get),
        SECONDARY_DESIRED_SCHEDULED.with(Cell::get),
        COMPLETED_WORK.with(Cell::get),
        SECONDARY_COMPLETED_WORK.with(Cell::get),
    );
    if ic_cdk::storage::stable_save(state).is_err() {
        ic_cdk::trap("runtime probe stable save failed");
    }
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (desired_scheduled, secondary_desired, completed_work, secondary_completed): (
        bool,
        bool,
        u64,
        u64,
    ) = match ic_cdk::storage::stable_restore() {
        Ok(state) => state,
        Err(_) => ic_cdk::trap("runtime probe stable restore failed"),
    };
    DESIRED_SCHEDULED.with(|desired| desired.set(desired_scheduled));
    SECONDARY_DESIRED_SCHEDULED.with(|desired| desired.set(secondary_desired));
    COMPLETED_WORK.with(|completed| completed.set(completed_work));
    SECONDARY_COMPLETED_WORK.with(|completed| completed.set(secondary_completed));
    if initialize_runtime().is_err() {
        ic_cdk::trap("post-upgrade runtime initialization failed");
    }
    if desired_scheduled && !install_watchdog() {
        ic_cdk::trap("post-upgrade watchdog reconstruction failed");
    }
    if secondary_desired && !install_secondary_watchdog() {
        ic_cdk::trap("post-upgrade secondary watchdog reconstruction failed");
    }
    let reconstructed = timer_snapshot(&probe_identity())
        .ok()
        .flatten()
        .and_then(|snapshot| snapshot.next_deadline_ns())
        .is_some();
    POST_UPGRADE_RECONSTRUCTED.with(|observed| observed.set(reconstructed));
}

#[ic_cdk::update]
fn start() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_watchdog()
}

#[ic_cdk::update]
fn start_immediately() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_watchdog_with_initial_schedule(true)
}

fn install_watchdog() -> bool {
    install_watchdog_with_initial_schedule(false)
}

fn install_watchdog_with_initial_schedule(immediate: bool) -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let identity = probe_identity();
        let cadence = match TimerCadence::from_nanos(CADENCE_NS) {
            Ok(cadence) => cadence,
            Err(_) => ic_cdk::trap("probe cadence is invalid"),
        };
        let registration = match register_watchdog(
            identity,
            cadence,
            DeclarationLifetime::Retained,
            |_context| {
                let now_ns = ic_cdk::api::time();
                let should_trap = TRAP_WINDOW
                    .with(Cell::get)
                    .is_some_and(|(start, end)| now_ns >= start && now_ns < end);
                if should_trap {
                    ic_cdk::trap("intentional watchdog work trap");
                }
                let should_exhaust = EXHAUST_WINDOW
                    .with(Cell::get)
                    .is_some_and(|(start, end)| now_ns >= start && now_ns < end);
                if should_exhaust {
                    exhaust_message_instructions();
                }
                COMPLETED_WORK.with(|completed| completed.set(completed.get().saturating_add(1)));
                let decision = if STOP_ON_NEXT_WORK.with(|stop| stop.replace(false)) {
                    DESIRED_SCHEDULED.with(|desired| desired.set(false));
                    WatchdogDecision::Stop
                } else if CONTINUE_IMMEDIATELY_ON_NEXT_WORK
                    .with(|requested| requested.replace(false))
                {
                    WatchdogDecision::ContinueImmediately
                } else {
                    WatchdogDecision::Continue
                };
                WatchdogRunResult::new(TimerCompletion::success(1), decision)
            },
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("watchdog registration failed"),
        };
        let scheduled = if immediate {
            registration.ensure_scheduled_immediately()
        } else {
            registration.ensure_scheduled()
        };
        if scheduled.is_err() {
            ic_cdk::trap("initial watchdog scheduling failed");
        }
        *slot = Some(registration);
        true
    })
}

#[ic_cdk::update]
fn continue_immediately_on_next_work() {
    CONTINUE_IMMEDIATELY_ON_NEXT_WORK.with(|requested| requested.set(true));
}

#[ic_cdk::update]
fn start_secondary() -> bool {
    SECONDARY_DESIRED_SCHEDULED.with(|desired| desired.set(true));
    install_secondary_watchdog()
}

fn install_secondary_watchdog() -> bool {
    SECONDARY_REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let cadence = match TimerCadence::from_nanos(CADENCE_NS) {
            Ok(cadence) => cadence,
            Err(_) => ic_cdk::trap("secondary cadence is invalid"),
        };
        let registration = match register_watchdog(
            secondary_identity(),
            cadence,
            DeclarationLifetime::Retained,
            |_context| {
                SECONDARY_COMPLETED_WORK.with(|completed| {
                    completed.set(completed.get().saturating_add(1));
                });
                WatchdogRunResult::new(TimerCompletion::success(1), WatchdogDecision::Continue)
            },
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("secondary watchdog registration failed"),
        };
        if registration.ensure_scheduled().is_err() {
            ic_cdk::trap("secondary watchdog scheduling failed");
        }
        *slot = Some(registration);
        true
    })
}

#[ic_cdk::update]
fn arm_trap() -> u64 {
    let trap_at_ns = match ic_cdk::api::time().checked_add(CADENCE_NS) {
        Some(value) => value,
        None => ic_cdk::trap("trap deadline overflow"),
    };
    let trap_until_ns = match trap_at_ns.checked_add(CADENCE_NS) {
        Some(value) => value,
        None => ic_cdk::trap("trap window overflow"),
    };
    TRAP_WINDOW.with(|window| window.set(Some((trap_at_ns, trap_until_ns))));
    trap_at_ns
}

#[ic_cdk::update]
fn arm_exhaustion() -> u64 {
    let exhaust_at_ns = match ic_cdk::api::time().checked_add(CADENCE_NS) {
        Some(value) => value,
        None => ic_cdk::trap("exhaustion deadline overflow"),
    };
    let exhaust_until_ns = match exhaust_at_ns.checked_add(CADENCE_NS) {
        Some(value) => value,
        None => ic_cdk::trap("exhaustion window overflow"),
    };
    EXHAUST_WINDOW.with(|window| window.set(Some((exhaust_at_ns, exhaust_until_ns))));
    exhaust_at_ns
}

#[ic_cdk::update]
fn stop_on_next_work() {
    STOP_ON_NEXT_WORK.with(|stop| stop.set(true));
}

#[ic_cdk::update]
fn stop() {
    DESIRED_SCHEDULED.with(|desired| desired.set(false));
    REGISTRATION.with_borrow(|slot| {
        if let Some(registration) = slot.as_ref()
            && registration.cancel().is_err()
        {
            ic_cdk::trap("watchdog cancellation failed");
        }
    });
}

#[ic_cdk::update]
fn ensure_from_returned_error() -> bool {
    DESIRED_SCHEDULED.with(|desired| desired.set(true));
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            ic_cdk::trap("watchdog registration is absent");
        };
        if registration.ensure_scheduled().is_err() {
            ic_cdk::trap("commit-window ensure failed");
        }
    });
    false
}

#[ic_cdk::update]
fn burn_to_below_timer_call_cost() -> (u128, u128) {
    let method_len = "<ic-cdk internal> timer_executor".len() as u64;
    let call_cost = ic_cdk::api::cost_call(method_len, 8);
    let liquid = ic_cdk::api::canister_liquid_cycle_balance();
    let target = call_cost.saturating_sub(1);
    let burned = ic_cdk::api::cycles_burn(liquid.saturating_sub(target));
    (burned, call_cost)
}

#[ic_cdk::update]
fn fill_inventory() -> u64 {
    loop {
        let current = match timer_inventory() {
            Ok(inventory) => inventory.len(),
            Err(_) => ic_cdk::trap("inventory lookup failed"),
        };
        if current >= MAX_TIMER_REGISTRATIONS {
            return current as u64;
        }
        let index = EXTRA_REGISTRATIONS.with_borrow(Vec::len);
        let name = format!("extra-{index:02}");
        let identity = match TimerIdentity::try_new("ic-timers", "runtime-probe", name) {
            Ok(identity) => identity,
            Err(_) => ic_cdk::trap("extra probe identity is invalid"),
        };
        let cadence = match TimerCadence::from_nanos(CADENCE_NS) {
            Ok(cadence) => cadence,
            Err(_) => ic_cdk::trap("extra probe cadence is invalid"),
        };
        let registration = match register_watchdog(
            identity,
            cadence,
            DeclarationLifetime::Retained,
            |_context| WatchdogRunResult::new(TimerCompletion::no_work(), WatchdogDecision::Stop),
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("extra watchdog registration failed"),
        };
        EXTRA_REGISTRATIONS.with_borrow_mut(|registrations| registrations.push(registration));
    }
}

#[ic_cdk::query]
fn inventory_measurement() -> (u64, u64, bool) {
    let before = ic_cdk::api::performance_counter(1);
    let inventory = match timer_inventory() {
        Ok(inventory) => inventory,
        Err(_) => ic_cdk::trap("inventory snapshot failed"),
    };
    let instructions = ic_cdk::api::performance_counter(1).saturating_sub(before);
    let ordered = inventory
        .timers()
        .windows(2)
        .all(|pair| pair[0].identity() < pair[1].identity());
    (inventory.len() as u64, instructions, ordered)
}

#[inline(never)]
fn exhaust_message_instructions() -> ! {
    loop {
        std::hint::black_box(ic_cdk::api::performance_counter(0));
    }
}

#[ic_cdk::query]
fn snapshot() -> ProbeSnapshot {
    let registered = REGISTRATION.with_borrow(Option::is_some);
    let completed_work = COMPLETED_WORK.with(Cell::get);
    let snapshot = match timer_snapshot(&probe_identity()) {
        Ok(snapshot) => snapshot,
        Err(_) => ic_cdk::trap("snapshot lookup failed"),
    };
    let Some(snapshot) = snapshot else {
        return ProbeSnapshot {
            registered,
            completed_work,
            next_deadline_ns: None,
            immediate_scheduling: false,
            latest_requested_delay_ns: None,
            latest_armed_delay_ns: None,
            schedule_requests: 0,
            scheduler_started: 0,
            wakeups_armed: 0,
            work_dispatched: 0,
            work_started: 0,
            work_completed: 0,
            coalesced: 0,
            unacknowledged: 0,
            last_unacknowledged: false,
            scheduler_instruction_samples: 0,
            scheduler_instruction_total: 0,
            work_instruction_samples: 0,
            work_instruction_total: 0,
            scheduler_memory: None,
            work_memory: None,
            post_upgrade_reconstructed: POST_UPGRADE_RECONSTRUCTED.with(Cell::get),
            secondary_registered: SECONDARY_REGISTRATION.with_borrow(Option::is_some),
            secondary_completed_work: SECONDARY_COMPLETED_WORK.with(Cell::get),
        };
    };
    let counters = snapshot.observability().counters();
    let performance = snapshot.observability().performance();
    let scheduler_memory = performance.scheduler_memory_pages();
    let work_memory = performance.work_memory_pages();
    ProbeSnapshot {
        registered,
        completed_work,
        next_deadline_ns: snapshot.next_deadline_ns(),
        immediate_scheduling: snapshot.scheduling_mode() == TimerSchedulingMode::Continuation,
        latest_requested_delay_ns: snapshot.latest_requested_delay_ns(),
        latest_armed_delay_ns: snapshot.latest_armed_delay_ns(),
        schedule_requests: counters.schedule_requests(),
        scheduler_started: counters.scheduler_started(),
        wakeups_armed: counters.wakeups_armed(),
        work_dispatched: counters.work_dispatched(),
        work_started: counters.work_started(),
        work_completed: counters.work_completed(),
        coalesced: counters.coalesced(),
        unacknowledged: counters.unacknowledged(),
        last_unacknowledged: snapshot.observability().outcomes().last_outcome()
            == Some(TimerLastOutcome::Unacknowledged),
        scheduler_instruction_samples: performance.scheduler_instructions().samples(),
        scheduler_instruction_total: performance.scheduler_instructions().total(),
        work_instruction_samples: performance.work_instructions().samples(),
        work_instruction_total: performance.work_instructions().total(),
        scheduler_memory: probe_memory_summary(scheduler_memory),
        work_memory: probe_memory_summary(work_memory),
        post_upgrade_reconstructed: POST_UPGRADE_RECONSTRUCTED.with(Cell::get),
        secondary_registered: SECONDARY_REGISTRATION.with_borrow(Option::is_some),
        secondary_completed_work: SECONDARY_COMPLETED_WORK.with(Cell::get),
    }
}

fn probe_memory_summary(summary: MemoryPageSummary) -> Option<ProbeMemorySummary> {
    let latest = summary.latest()?;
    Some(ProbeMemorySummary {
        samples: summary.samples(),
        latest_wasm_start_pages: latest.start().wasm_pages(),
        latest_wasm_end_pages: latest.end().wasm_pages(),
        latest_stable_start_pages: latest.start().stable_pages(),
        latest_stable_end_pages: latest.end().stable_pages(),
        maximum_wasm_growth_pages: summary.maximum_wasm_growth_pages()?,
        maximum_stable_growth_pages: summary.maximum_stable_growth_pages()?,
    })
}

fn probe_identity() -> TimerIdentity {
    match TimerIdentity::try_new("ic-timers", "runtime-probe", "watchdog") {
        Ok(identity) => identity,
        Err(_) => ic_cdk::trap("probe identity is invalid"),
    }
}

fn secondary_identity() -> TimerIdentity {
    match TimerIdentity::try_new("ic-timers", "runtime-probe", "secondary") {
        Ok(identity) => identity,
        Err(_) => ic_cdk::trap("secondary identity is invalid"),
    }
}
