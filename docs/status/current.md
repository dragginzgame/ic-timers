![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.8**, `69910ed`. The undated **0.31.9** draft records the
accepted IC Tooling, Shared Tooling and Platform adoption. Platform's removed
Candid compiler APIs have no Timers consumers; the published timer API, runtime
semantics and protocols are unchanged, so the next patch remains appropriate.
Cargo package versions remain 0.31.8. Staging, commits, pushes and release
execution remain maintainer-owned.

The batch began with a pending root lock update to Build/Host/Testkit 0.68.12
and Platform 0.6.21. That work is preserved; the root catalog now selects the
published Platform 0.7.0 and Tooling 0.68.12 minimums. The lock additionally
selects Platform/macros 0.7.0. Official registry records and matching published
archive checksums establish availability. Wire 0.7.18 and Metrics 0.16.3 stay
unchanged; no unpublished sibling source is used.

## Shared adoption and ownership

Shared Tooling **0.26.22**, revision
`5fc904a935d82ad31f0fba14434b71ccb8c10345`, is verified against upstream main
and adopted from a clean clone through canonical impact/apply. All 131 payloads
verify. The existing roots are preserved; five governance documents change,
without executable, tool-version, payload-roster or gate changes. The local
[adoption guide](../shared-tooling.md) distinguishes updated guidance from
executable enforcement. No optional audit or schedule was activated.

The new rules clarify owned-type Candid restrictions, mechanical error derives,
complete source inspection, strict machine-output admission, subprocess bounds
and safe diagnostics. Existing Timers declaration/graph/provider checks retain
codec and deployment enforcement; adopting broader prose does not add a general
scanner or qualify native-host behavior.

The deployed timer library still selects only Platform's executor/ic0 features,
with one executor 2.0.0 and ic0 1.2.0 identity. Candid, Wire and host tooling
remain excluded from its resolved normal/build graph. Probes retain their Wire
protocol and existing runtime feature selection. The native simulator test graph
contains both Platform 0.7.0 and 0.6.21: published Testkit 0.68.12 still requires
`^0.6.19`. This is a host-test facade duplication, not a parallel timer provider;
no patch, source override or consumer compatibility layer was introduced. Testkit
must publish a 0.7-compatible requirement to eliminate its remaining facade.

The preceding 0.31.8 adoption already delegates retained build ownership and
freshness to `WasmBuildEvidence`, including its published `record()` accessor
([Tooling #255](https://github.com/dragginzgame/ic-tooling/issues/255)). Exact
artifact selection, recipe/report context and final publication stay Timers-owned.

## Verification and limits

Before-images, official registry records, impact/apply output and focused logs:
`/tmp/ic-timers-adoption-0319.yLi7Qi/`. Rust 1.91 strict Clippy passes for library,
coordinator/test targets, runtime probe and protocol. All 15 selected native
tests pass: build-driver (3), report admission (3), private platform (9).
Rust 1.91 Wasm compilation passes for library, runtime probe and the size probe's
default baseline. Snapshot integrity, formatting, changelog, pin/inheritance,
external-IC declarations, native/Wasm graph and private-provider checks pass.

Selected editor and Testkit CLI 0.68.12 setup and offline admission pass; Testkit
admits the existing PocketIC 16.1.0 server. The first server setup timed out on
an active sibling-owned installation slot. After that installer exited, the
second setup reused its completed build. Both attempts remain retained; no
foreign lock or previous tool/evidence generation was removed.

These are local Linux native tests and Wasm cross-compilation, not a fresh full
release gate, installed-IC recovery, PocketIC measurement or native macOS run.
The full user-operated release gate and required host matrix remain intact.
Previous dependency/evidence-owner adoption logs remain at
`/tmp/ic-timers-deps-0318.9bj5hB/`; preceding native-fixture evidence remains at
`/tmp/ic-timers-native-cases-0318.iRKVot/`, each with its original input scope.
