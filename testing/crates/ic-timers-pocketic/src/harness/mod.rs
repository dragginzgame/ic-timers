//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use candid::{Encode, Principal};
use ic_testkit::pic::{
    CandidCallExt, CanisterInstallExt, InstallSpec, PocketIc, PocketIcBuilder, PocketIcInstance,
    PocketIcStartupConfig, tick_until,
};
use ic_testkit_candid::DecodeLimits;
use std::{convert::Infallible, env, fmt::Debug, path::PathBuf, task::Poll, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);
pub const INIT_CYCLES: u128 = 2_000_000_000_000;

// Probe replies are bounded snapshots/counters, not application payloads.
// These ceilings bound decoding only; tick budgets and transport stay separate.
pub const REPLY_LIMITS: DecodeLimits = DecodeLimits {
    reply_bytes: 64 * 1024,
    decoding_quota: 1_000_000,
    skipping_quota: 10_000,
    type_entries: 256,
    header_bytes: 8 * 1024,
};

/// Preserve tick-before-observation boundaries while Testkit owns bounded progress.
pub fn wait_for_probe<T: Debug>(
    pic: &PocketIc,
    rounds: u32,
    mut observe: impl FnMut(&PocketIc) -> T,
    ready: impl Fn(&T) -> bool,
) -> T {
    let remaining = rounds.checked_sub(1).expect("positive probe round budget");
    let mut last = None;
    pic.tick();
    tick_until(pic, remaining, Duration::ZERO, |pic| {
        let observation = observe(pic);
        if ready(&observation) {
            Poll::Ready(Ok::<_, Infallible>(observation))
        } else {
            last = Some(observation);
            Poll::Pending
        }
    })
    .unwrap_or_else(|error| panic!("{error}; last probe observation: {last:?}"))
}

/// Install one probe with the fixtures' explicit funding and empty init payload.
pub fn install_probe(pic: &PocketIc, wasm: Vec<u8>) -> Principal {
    pic.try_create_and_install(
        InstallSpec::new(wasm, Encode!().expect("encode probe init"), INIT_CYCLES)
            .label("ic-timers probe"),
    )
    .expect("install funded timer probe")
}

/// Testkit retains the server and drops the instance before stopping it.
/// One deadline covers both startup phases; synchronous instance deletion on
/// Drop remains unbounded.
pub fn fresh_pocket_ic() -> PocketIcInstance<PocketIc> {
    let binary = env::var_os("POCKET_IC_BIN")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .expect("run the Testkit pocketic-check gate and supply its admitted POCKET_IC_BIN");
    PocketIcStartupConfig::spawn(binary, STARTUP_TIMEOUT)
        .build_instance(PocketIcBuilder::new().with_application_subnet())
        .expect("construct a fresh owned PocketIC instance through ic-testkit")
}

/// Invoke an anonymous update with no arguments or return values.
pub fn update_unit(pic: &PocketIc, canister_id: Principal, method: &str) {
    pic.update_candid::<(), _>(canister_id, method, (), REPLY_LIMITS)
        .expect("probe update succeeds");
}
