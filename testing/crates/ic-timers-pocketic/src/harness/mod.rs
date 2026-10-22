//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use ic_platform::pocket_ic::{PocketIc, PocketIcBuilder};
use ic_platform::principal::ExternalPrincipal;
use ic_testkit::pic::{
    CanisterInstallExt, InstallSpec, PocketIcInstance, PocketIcStartupConfig, tick_until,
};
use ic_timers_test_protocol::rpc::{Method, REQUEST_LIMITS, RESPONSE_LIMITS};
use serde::{Serialize, de::DeserializeOwned};
use std::{convert::Infallible, env, fmt::Debug, path::PathBuf, task::Poll, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);
pub const INIT_CYCLES: u128 = 2_000_000_000_000;

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
pub fn install_probe(pic: &PocketIc, wasm: Vec<u8>) -> ExternalPrincipal {
    pic.try_create_and_install(InstallSpec {
        wasm,
        init_bytes: Vec::new(),
        cycles: INIT_CYCLES,
        install_sender: None,
        label: Some("ic-timers probe".to_owned()),
    })
    .expect("install funded timer probe")
}

/// Testkit retains the server and drops the instance before stopping it.
/// Startup has its own deadline. A five-minute hard TTL also covers each server
/// while the driver bounds the worker, including synchronous instance deletion.
pub fn fresh_pocket_ic() -> PocketIcInstance<PocketIc> {
    let binary = env::var_os("POCKET_IC_BIN")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .expect("run the Testkit pocketic-check gate and supply its admitted POCKET_IC_BIN");
    PocketIcStartupConfig::Spawn {
        server_binary: binary,
        timeout: STARTUP_TIMEOUT,
        server_hard_ttl: Some(Duration::from_secs(5 * 60)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(PocketIcBuilder::new().with_application_subnet())
    .expect("construct a fresh owned PocketIC instance through ic-testkit")
}

/// Invoke an anonymous update with no arguments or return values.
pub fn update_unit(pic: &PocketIc, canister_id: ExternalPrincipal, method: Method) {
    update_wire::<(), _>(pic, canister_id, ExternalPrincipal::anonymous(), method, ())
        .expect("probe update succeeds");
}

/// Send one method-bound Wire request through the maintained raw PocketIC API.
pub fn update_wire<T: Serialize + DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    caller: ExternalPrincipal,
    method: Method,
    args: A,
) -> Result<T, Box<dyn std::error::Error>> {
    let id = method as u64;
    let request = ic_wire::native::encode_request(id, &args, REQUEST_LIMITS)?;
    let response = pic
        .update_call(canister_id, caller, method.name(), request)
        .map_err(|error| format!("{}: {error:?}", method.name()))?;
    Ok(ic_wire::native::decode_response(
        id,
        &response,
        RESPONSE_LIMITS,
    )?)
}

/// Decode a bounded Wire query reply without changing its caller or timing.
pub fn query_wire<T: Serialize + DeserializeOwned, A: Serialize>(
    pic: &PocketIc,
    canister_id: ExternalPrincipal,
    caller: ExternalPrincipal,
    method: Method,
    args: A,
) -> Result<T, Box<dyn std::error::Error>> {
    let id = method as u64;
    let request = ic_wire::native::encode_request(id, &args, REQUEST_LIMITS)?;
    let response = pic
        .query_call(canister_id, caller, method.name(), request)
        .map_err(|error| format!("{}: {error:?}", method.name()))?;
    Ok(ic_wire::native::decode_response(
        id,
        &response,
        RESPONSE_LIMITS,
    )?)
}
