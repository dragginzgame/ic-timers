//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use candid::{Encode, Principal};
use ic_testkit::pic::{
    CanisterInstallExt, InstallSpec, PocketIc, PocketIcBuilder, PocketIcInstance,
    PocketIcStartupConfig,
};
use std::{env, path::PathBuf, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);
pub const INIT_CYCLES: u128 = 2_000_000_000_000;

/// Install one probe with the fixtures' explicit funding and empty init payload.
pub fn install_probe(pic: &PocketIc, wasm: Vec<u8>) -> Principal {
    pic.create_and_install(
        InstallSpec::new(wasm, Encode!().expect("encode probe init"), INIT_CYCLES)
            .label("ic-timers probe"),
    )
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
