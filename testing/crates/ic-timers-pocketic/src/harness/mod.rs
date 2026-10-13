//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use ic_testkit::pic::{PocketIc, PocketIcBuilder, PocketIcInstance, PocketIcStartupConfig};
use std::{env, path::PathBuf, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(60);

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
