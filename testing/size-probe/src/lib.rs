use candid::CandidType;
use ic_timers::{TimerIdentity, initialize_runtime, timer_inventory, timer_snapshot};
use std::cell::Cell;

#[cfg(not(feature = "baseline"))]
use ic_timers::DeclarationLifetime;
#[cfg(not(feature = "baseline"))]
use std::cell::RefCell;

#[cfg(feature = "after-completion")]
use ic_timers::{
    AfterCompletionRegistration, TimerCadence, TimerCompletion, TimerDirective, TimerRunResult,
    register_after_completion,
};
#[cfg(feature = "once")]
use ic_timers::{
    OnceRegistration, TimerCompletion, TimerDirective, TimerRunResult, TimerSchedule, register_once,
};
#[cfg(feature = "watchdog")]
use ic_timers::{
    TimerCadence, TimerCompletion, WatchdogDecision, WatchdogRegistration, WatchdogRunResult,
    register_watchdog,
};

#[cfg(not(any(
    feature = "baseline",
    feature = "once",
    feature = "after-completion",
    feature = "watchdog"
)))]
compile_error!("select exactly one size-probe cohort feature");
#[cfg(any(
    all(feature = "baseline", feature = "once"),
    all(feature = "baseline", feature = "after-completion"),
    all(feature = "baseline", feature = "watchdog"),
    all(feature = "once", feature = "after-completion"),
    all(feature = "once", feature = "watchdog"),
    all(feature = "after-completion", feature = "watchdog")
))]
compile_error!("size-probe cohort features are mutually exclusive");

#[cfg(not(feature = "baseline"))]
const CADENCE_NS: u64 = 1_000_000_000;

#[cfg(feature = "once")]
type CohortRegistration = OnceRegistration;
#[cfg(feature = "after-completion")]
type CohortRegistration = AfterCompletionRegistration;
#[cfg(feature = "watchdog")]
type CohortRegistration = WatchdogRegistration;

#[cfg(not(feature = "baseline"))]
thread_local! {
    static REGISTRATION: RefCell<Option<CohortRegistration>> = const { RefCell::new(None) };
}

thread_local! {
    static CALLBACKS: Cell<u64> = const { Cell::new(0) };
}

#[cfg(feature = "watchdog")]
thread_local! {
    static REPRESENTATIVE_WATCHDOG_WORK: Cell<bool> = const { Cell::new(false) };
}

#[derive(CandidType, Debug)]
struct OperationMeasurement {
    changed: bool,
    instructions: u64,
}

#[derive(CandidType, Debug)]
struct MemorySamplingMeasurement {
    empty_bracket_instructions: u64,
    sampled_bracket_instructions: u64,
    overhead_instructions: u64,
}

#[derive(CandidType, Debug)]
struct CohortObservation {
    cohort: &'static str,
    registered: bool,
    callbacks: u64,
    next_deadline_ns: Option<u64>,
    schedule_requests: u64,
    wakeups_armed: u64,
    work_dispatched: u64,
    scheduler_started: u64,
    work_started: u64,
    work_completed: u64,
    coalesced: u64,
    scheduler_instruction_samples: u64,
    scheduler_instruction_total: u64,
    work_instruction_samples: u64,
    work_instruction_total: u64,
    snapshot_instructions: u64,
    inventory_len: u64,
    inventory_instructions: u64,
}

#[ic_cdk::init]
fn init() {
    if initialize_runtime().is_err() {
        ic_cdk::trap("size-probe runtime initialization failed");
    }
}

#[ic_cdk::update]
fn start() -> OperationMeasurement {
    measure(start_inner)
}

#[ic_cdk::update]
fn ensure_again() -> OperationMeasurement {
    measure(ensure_again_inner)
}

#[ic_cdk::update]
fn cancel() -> OperationMeasurement {
    measure(cancel_inner)
}

/// Isolate the raw instruction cost of the start/end page reads used by one
/// normally completed callback. This probe-only endpoint does not alter the
/// runtime's callback instruction aggregate.
#[ic_cdk::update]
fn memory_sampling_overhead() -> MemorySamplingMeasurement {
    let empty_bracket_instructions = measure_interval(|| {
        std::hint::black_box((0_u64, 0_u64));
        std::hint::black_box((0_u64, 0_u64));
    });
    let sampled_bracket_instructions = measure_interval(|| {
        std::hint::black_box(memory_page_extents());
        std::hint::black_box(memory_page_extents());
    });
    MemorySamplingMeasurement {
        empty_bracket_instructions,
        sampled_bracket_instructions,
        overhead_instructions: sampled_bracket_instructions
            .saturating_sub(empty_bracket_instructions),
    }
}

/// Select a bounded non-trivial callback body for the probe's next Watchdog
/// registration. This endpoint exists only in the Watchdog test canister.
#[cfg(feature = "watchdog")]
#[ic_cdk::update]
fn use_representative_watchdog_work() {
    REPRESENTATIVE_WATCHDOG_WORK.with(|enabled| enabled.set(true));
}

#[ic_cdk::query]
fn observe() -> CohortObservation {
    let snapshot_before = ic_cdk::api::performance_counter(1);
    let snapshot = match timer_snapshot(&cohort_identity()) {
        Ok(snapshot) => snapshot,
        Err(_) => ic_cdk::trap("size-probe snapshot failed"),
    };
    let snapshot_instructions = ic_cdk::api::performance_counter(1).saturating_sub(snapshot_before);
    let inventory_before = ic_cdk::api::performance_counter(1);
    let inventory = match timer_inventory() {
        Ok(inventory) => inventory,
        Err(_) => ic_cdk::trap("size-probe inventory failed"),
    };
    let inventory_instructions =
        ic_cdk::api::performance_counter(1).saturating_sub(inventory_before);
    let registered = registration_present();
    let callbacks = CALLBACKS.with(Cell::get);
    let Some(snapshot) = snapshot else {
        return CohortObservation {
            cohort: cohort_name(),
            registered,
            callbacks,
            next_deadline_ns: None,
            schedule_requests: 0,
            wakeups_armed: 0,
            work_dispatched: 0,
            scheduler_started: 0,
            work_started: 0,
            work_completed: 0,
            coalesced: 0,
            scheduler_instruction_samples: 0,
            scheduler_instruction_total: 0,
            work_instruction_samples: 0,
            work_instruction_total: 0,
            snapshot_instructions,
            inventory_len: inventory.len() as u64,
            inventory_instructions,
        };
    };
    let counters = snapshot.observability().counters();
    let performance = snapshot.observability().performance();
    CohortObservation {
        cohort: cohort_name(),
        registered,
        callbacks,
        next_deadline_ns: snapshot.next_deadline_ns(),
        schedule_requests: counters.schedule_requests(),
        wakeups_armed: counters.wakeups_armed(),
        work_dispatched: counters.work_dispatched(),
        scheduler_started: counters.scheduler_started(),
        work_started: counters.work_started(),
        work_completed: counters.work_completed(),
        coalesced: counters.coalesced(),
        scheduler_instruction_samples: performance.scheduler_instructions().samples(),
        scheduler_instruction_total: performance.scheduler_instructions().total(),
        work_instruction_samples: performance.work_instructions().samples(),
        work_instruction_total: performance.work_instructions().total(),
        snapshot_instructions,
        inventory_len: inventory.len() as u64,
        inventory_instructions,
    }
}

fn measure(operation: impl FnOnce() -> bool) -> OperationMeasurement {
    let before = ic_cdk::api::performance_counter(1);
    let changed = operation();
    OperationMeasurement {
        changed,
        instructions: ic_cdk::api::performance_counter(1).saturating_sub(before),
    }
}

fn measure_interval(operation: impl FnOnce()) -> u64 {
    let before = ic_cdk::api::performance_counter(1);
    operation();
    ic_cdk::api::performance_counter(1).saturating_sub(before)
}

#[inline(always)]
fn memory_page_extents() -> (u64, u64) {
    #[cfg(target_arch = "wasm32")]
    let wasm = core::arch::wasm32::memory_size::<0>() as u64;
    #[cfg(not(target_arch = "wasm32"))]
    let wasm = 0;

    (wasm, ic_cdk::api::stable_size())
}

#[cfg(feature = "baseline")]
fn start_inner() -> bool {
    false
}

#[cfg(feature = "once")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = match register_once(
            cohort_identity(),
            DeclarationLifetime::Retained,
            |_context| async {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                TimerRunResult::new(TimerCompletion::no_work(), TimerDirective::Stop)
            },
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("size-probe Once registration failed"),
        };
        if registration
            .ensure_scheduled(TimerSchedule::After(std::time::Duration::from_nanos(
                CADENCE_NS,
            )))
            .is_err()
        {
            ic_cdk::trap("size-probe Once scheduling failed");
        }
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "after-completion")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = match register_after_completion(
            cohort_identity(),
            cadence(),
            DeclarationLifetime::Retained,
            |_context| async {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                TimerRunResult::new(
                    TimerCompletion::no_work(),
                    TimerDirective::RecurAfterCompletion,
                )
            },
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("size-probe after-completion registration failed"),
        };
        if registration.ensure_scheduled().is_err() {
            ic_cdk::trap("size-probe after-completion scheduling failed");
        }
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "watchdog")]
fn start_inner() -> bool {
    REGISTRATION.with_borrow_mut(|slot| {
        if slot.is_some() {
            return false;
        }
        let registration = match register_watchdog(
            cohort_identity(),
            cadence(),
            DeclarationLifetime::Retained,
            |_context| {
                CALLBACKS.with(|calls| calls.set(calls.get().saturating_add(1)));
                WatchdogRunResult::new(watchdog_completion(), WatchdogDecision::Continue)
            },
        ) {
            Ok(registration) => registration,
            Err(_) => ic_cdk::trap("size-probe Watchdog registration failed"),
        };
        if registration.ensure_scheduled().is_err() {
            ic_cdk::trap("size-probe Watchdog scheduling failed");
        }
        *slot = Some(registration);
        true
    })
}

#[cfg(feature = "watchdog")]
fn watchdog_completion() -> TimerCompletion {
    if !REPRESENTATIVE_WATCHDOG_WORK.with(Cell::get) {
        return TimerCompletion::no_work();
    }

    // Keep the calibration body dependency-free, deterministic, bounded, and
    // visibly non-trivial after optimization. It models synchronous work, not
    // IcyDB database logic or a production instruction budget.
    let mut state = std::hint::black_box(0x9e37_79b9_7f4a_7c15_u64);
    for step in 0..100_000_u64 {
        state = state.rotate_left(7) ^ step;
        state = state.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    }
    std::hint::black_box(state);
    TimerCompletion::success(1)
}

#[cfg(feature = "baseline")]
fn ensure_again_inner() -> bool {
    false
}

#[cfg(feature = "once")]
fn ensure_again_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        if registration
            .ensure_scheduled(TimerSchedule::After(std::time::Duration::from_nanos(
                CADENCE_NS,
            )))
            .is_err()
        {
            ic_cdk::trap("size-probe duplicate Once ensure failed");
        }
        false
    })
}

#[cfg(any(feature = "after-completion", feature = "watchdog"))]
fn ensure_again_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        if registration.ensure_scheduled().is_err() {
            ic_cdk::trap("size-probe duplicate recurring ensure failed");
        }
        false
    })
}

#[cfg(feature = "baseline")]
fn cancel_inner() -> bool {
    false
}

#[cfg(not(feature = "baseline"))]
fn cancel_inner() -> bool {
    REGISTRATION.with_borrow(|slot| {
        let Some(registration) = slot.as_ref() else {
            return false;
        };
        let was_scheduled = timer_snapshot(&cohort_identity())
            .ok()
            .flatten()
            .and_then(|snapshot| snapshot.next_deadline_ns())
            .is_some();
        if registration.cancel().is_err() {
            ic_cdk::trap("size-probe cancellation failed");
        }
        was_scheduled
    })
}

#[cfg(feature = "baseline")]
const fn registration_present() -> bool {
    false
}

#[cfg(not(feature = "baseline"))]
fn registration_present() -> bool {
    REGISTRATION.with_borrow(Option::is_some)
}

#[cfg(any(feature = "after-completion", feature = "watchdog"))]
fn cadence() -> TimerCadence {
    match TimerCadence::from_nanos(CADENCE_NS) {
        Ok(cadence) => cadence,
        Err(_) => ic_cdk::trap("size-probe cadence is invalid"),
    }
}

fn cohort_identity() -> TimerIdentity {
    match TimerIdentity::try_new("ic-timers", "size-probe", cohort_name()) {
        Ok(identity) => identity,
        Err(_) => ic_cdk::trap("size-probe identity is invalid"),
    }
}

#[cfg(feature = "baseline")]
const fn cohort_name() -> &'static str {
    "baseline"
}

#[cfg(feature = "once")]
const fn cohort_name() -> &'static str {
    "once"
}

#[cfg(feature = "after-completion")]
const fn cohort_name() -> &'static str {
    "after-completion"
}

#[cfg(feature = "watchdog")]
const fn cohort_name() -> &'static str {
    "watchdog"
}
