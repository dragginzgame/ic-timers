//! Fresh, caller-owned testkit servers and IC instances from the verified binary.

use ic_testkit::pic::{
    PocketIc, PocketIcBuilder, PocketIcBuilderExt, PocketIcManagedServer, PocketIcStartupConfig,
};
use std::{env, path::PathBuf, time::Duration};

const STARTUP_TIMEOUT: Duration = Duration::from_secs(30);

/// Retain both bindings as `let (_server, pic) = fresh_pocket_ic()` so the
/// instance is dropped before its server. Each startup phase has a deadline;
/// upstream instance deletion on Drop remains unbounded.
#[must_use]
pub fn fresh_pocket_ic() -> (PocketIcManagedServer, PocketIc) {
    let binary = env::var_os("POCKET_IC_BIN")
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .expect("run the pinned pocketic-check gate and supply POCKET_IC_BIN");
    let server = PocketIcStartupConfig::spawn(binary, STARTUP_TIMEOUT)
        .start_managed_server()
        .expect("start the verified PocketIC server through ic-testkit");
    let pic = PocketIcBuilder::new()
        .with_application_subnet()
        .try_build(PocketIcStartupConfig::connect(
            server.url(),
            STARTUP_TIMEOUT,
        ))
        .expect("construct a fresh PocketIC instance through ic-testkit");
    (server, pic)
}
