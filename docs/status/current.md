![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-28

## Source and pending work

Released base: **0.31.11**, `eca0235`. The undated **0.31.12** draft retains
pending Shared Tooling/dependency adoption and the accepted freeze/top-up
regression. Cargo package versions remain 0.31.11. Unrelated work is preserved;
no staging, commits, pushes or release execution were performed.

The root minimums now match selected published Tooling **0.69.3** and Platform
**0.7.1**. All 12 direct registry dependencies were checked against crates.io;
Wire **0.7.18**, Metrics **0.16.3**, Clap **4.6.7**, Serde **1.0.229** and
Serde JSON **1.0.154** are already current. The sole constrained update is
Thiserror: PocketIC 16.1.0 requires exactly **2.0.18**, so Cargo rejected the
targeted **2.0.21** update. Keep that existing selection; no local shim or
supplier override was introduced. The failed resolution left Cargo.lock
unchanged, including the previous externally selected IC patch updates.

## Freeze regression and boundary

`tests::frozen_canister_resumes_overdue_timer_after_top_up_only` arms a Watchdog,
then raises its real freezing reserve above the observed positive balance. The
reserve uses the instance's reported storage charge, avoiding an assumed probe
size or PocketIC creation balance. Five simulated frozen minutes span the
original deadline. A valid update must reject with typed `SysTransient` /
`CanisterOutOfCycles`, while the positive balance remains below the reserve.

Only an external top-up and replica ticks restore progress. A read-only snapshot
before further rounds shows the overdue registration and zero scheduler/work
progress were retained. The test requires one overdue completion, no additional
work under unchanged time, and the next normal cadence completion. Module hash
and raised freezing threshold remain unchanged; the upgrade-reconstruction flag
stays clear. No ensure/reconcile call, raw timer repair or upgrade is used.

The subject is in the driver's required watchdog roster, so the normal recovery
and release gate must discover and execute it. The existing near-threshold
self-call-cost test is retained as a separate case. This qualifies an armed
Watchdog before work begins, not every timer policy, freeze during an await, or
recovery after cycle exhaustion uninstalls the canister. README, architecture,
SAFETY and the release guide state that scope. No runtime implementation, probe
protocol, supported API or feature selection was changed by this test work.

## Pending tooling retained

Shared Tooling **0.27.2**, revision
`7efce1e36275ff9e4bbbdd9ccab9a8d94520fd98`, is the reviewed 134-payload
snapshot, applied through its canonical impact/exporter workflow. Its source
has pending 0.27.3 notes; committed VERSION still says 0.27.2. The refresh adds
supplier-finding ownership and native bootstrap guidance, plus newline-safe
fixture roots. Timers' hook fixture stages its copied companion closure without
a duplicate helper roster. Optional stack refresh, Node setup and scanner
selections remain inactive; executable catalogs and product gates are unchanged.
The current library graph check
passes with one qualified executor 2.0.0/ic0 1.2.0 path through Platform;
Candid, Wire and host tooling remain excluded from deployed normal/build edges.
Native simulator tests retain required external Candid through Platform.

## Dependency adoption verification

Evidence: `/tmp/ic-timers-upstream-03112.xc99Cb/`, including the incoming dirty
patch/before-images, registry inventory, failed targeted update, reviewed source
checkout and impact report. Root locked MSRV metadata, direct dependency policy,
transitive deployment exclusions/feature restrictions and singleton executor/ic0
identities pass. No Candid or host dependency was added to deployed normal/build
edges. Native simulator tests still require external Candid through Platform.

Explicit editor/Testkit setup and subsequent offline admission pass for 0.69.3,
closing the prior editor-preparation gap. PocketIC remains **16.1.0**. Rust 1.91
strict library/test Clippy, coordinator/runtime-probe/protocol Clippy, optimized
runtime-probe Wasm and all four size-probe Wasm Clippy feature selections pass.
Snapshot integrity, formatting, actual hook integration, substituted release
command routing, pin policy, changelog and diff checks pass. Probe wire protocol,
features and timer APIs are unchanged. No new issue or external write was needed.

Cargo.lock matches this task's incoming lock. The rebuilt runtime-probe digest
also exactly matches the freeze proof below, so its two focused PocketIC results
are reused rather than presented as fresh executions after this manifest refresh.

## Freeze evidence and limits

Before-images, fixture diagnostics, selected lock/metadata, exact Wasm bytes/hash,
server logs and focused results are retained at
`/tmp/ic-timers-freeze-03112.voNdez/`. The `updated-*` logs qualify the final
Tooling 0.69.3 / Platform 0.7.1 graph; older logs retain their narrower input scope.
The lock changed during the first verification pass, so its earlier runtime
results were not reused as qualification of the new graph.

Final Linux evidence passes: Rust 1.91 strict coordinator/test Clippy, optimized
`timer-probe` Wasm build, the new real PocketIC freeze test, the adjacent low-cycle
PocketIC test and exact required-subject discovery regression. Testkit CLI 0.69.3
was explicitly prepared and admits PocketIC **16.1.0** offline. The final lock
and loaded-Wasm digest stay unchanged across the focused tests. Formatting,
provider/deployment boundary, changelog and diff checks pass.

Initial fixture failures and their corrections are explained in
`fixture-development-notes.txt`; the transient type-inference compiler log was
overwritten, with its cause/excerpt retained in the notes/tool transcript.
No runtime defect was found or runtime repair introduced.

No fresh full recovery/cohort/release gate, native macOS or installed-IC run was
performed. This is source-bound Linux PocketIC evidence. Normal release
preparation must retain the complete product/host gates.
Prior tooling and Shared evidence remain at `/tmp/ic-timers-deps-03112.5RKZ5o/`
and `/tmp/ic-timers-shared-03112.nWc5TV/` for their original inputs only.
