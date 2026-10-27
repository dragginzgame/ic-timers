![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.7**, `f159476`; current source is maintainer commit
`86bca11` containing the named native-fixture cleanup. The undated **0.31.8**
section also records the accepted dependency/evidence-owner adoption. Published
APIs and timer semantics are unchanged, so the next patch remains appropriate.
Cargo package versions remain 0.31.7. Commits, staging, pushes and release
execution remain maintainer-owned.

The dependency batch began with an already updated lock selecting Platform
0.6.20 and Build/Host/Testkit 0.68.11, plus the pending handoff edit. Both were
preserved. Compatible `cargo update` selected no additional packages; the lock
is byte-identical to that incoming selection. Root dependency minimums now name
those releases and the existing Wire 0.7.18 / Metrics 0.16.3 selections.
Official sparse-index records verify these are the latest published releases;
Wire's sibling 0.7.19 notes do not establish publication. Published Build and
Platform archive checksums match the registry. No sibling source is substituted.
Shared Tooling remains the reviewed 0.26.21 snapshot.

## Completed cleanup and ownership

The preceding committed test cleanup replaces numeric policies, trace commands,
result cases and Watchdog scenarios with named cases. Original exhaustive words,
prefix checks, independent expected outcomes and capture/lifecycle cases remain.
Its focused evidence is `/tmp/ic-timers-native-cases-0318.iRKVot/`.

Build 0.68.11 supplies `WasmBuildEvidence::record()`. The qualification driver now
retains that owner through measurement and calls `verify_current()` before
accepting results, removing the parallel spec/input/build tuple and manual
fingerprint comparisons. Timers still owns recipes, exact artifact cardinality,
report context, loaded-Wasm admission and durable publication. Report schema,
probe protocols, runtime behavior and deployed feature selections are unchanged.
Persistent input-change refusal belongs to Build; callers still hold source
custody, since checks cannot detect edits reverted between observations.

## Issues and upstream boundary

[Timers #75](https://github.com/dragginzgame/ic-timers/issues/75) was closed after
[released-main run 40474305674](https://github.com/dragginzgame/ic-timers/actions/runs/40474305674)
passed Linux, MSRV and native macOS Intel/ARM64. The preceding backlog check
found no remaining open Timers issues or PRs; this dependency batch did not
repeat or change that inventory.

[IC Tooling #255](https://github.com/dragginzgame/ic-tooling/issues/255) is no
longer a publication blocker for this consumer: its narrow accessor is verified
in the downloaded 0.68.11 package and adopted locally. Upstream issue closure and
its separate native-host acceptance remain supplier-owned.

## Verification and limits

Dependency before-images, official registry records and focused logs are in
`/tmp/ic-timers-deps-0318.9bj5hB/`. Rust 1.91 strict Clippy passes for library,
coordinator/test targets, runtime probe and protocol. All 15 selected native
tests pass: three build-driver, three report-admission and nine private-platform
subjects. Rust 1.91 Wasm compilation passes for library, runtime probe and the
size probe's default baseline selection. Formatting, changelog, pin/inheritance,
external-IC declaration, native/Wasm graph and private-provider checks pass.

The selected release editor and Testkit CLI were explicitly prepared and pass
offline admission; Testkit admits its existing PocketIC 16.1.0 server. Initial
checks correctly refused unprepared selections. Testkit's first setup reached
the bounded 60-second wait on an active sibling-owned installer; once that owner
exited, a second setup reused its completed build. No foreign lock was removed.
Both setup attempts and final admissions are retained with the focused logs.

The deployed library graph retains only executor 2.0.0 and ic0 1.2.0 through
Platform, with no Candid, Wire or host tooling. Candid remains transitive in the
native simulator test graph through Platform/PocketIC. This is graph admission
and cross-compilation evidence, not a new installed-IC recovery qualification.
No fresh full release gate, PocketIC measurement or native macOS run was performed.
