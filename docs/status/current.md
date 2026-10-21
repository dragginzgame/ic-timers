![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-21

## Source and accepted work

Released base: **0.29.7**, `a39aa73`; package versions remain unchanged.
Preserve the incoming unstaged root catalog/lock changes: Metrics **0.15.0**,
Platform/macros **0.5.1**, Build/Host/Testkit **0.62.1** and Wire **0.5.4**.
One root catalog and lock still cover the five maintained packages. Shared remains
reviewed **0.25.7**, `d39e8a84bf4fd7e42776d2bb8e589ad09aacb02b`.

The undated **0.30.0** draft includes the pending Metrics dependency identity cut:
Timers publicly reexports its counter and summary types, so consumers exchanging
those values must align direct Metrics dependencies to 0.15. Its arithmetic Rust
sources are unchanged, but 0.14 and 0.15 are separate Rust type identities.
The dependency-role cleanup alone is compatible; retaining 0.14 would avoid that
minor boundary, but the maintainer's pending 0.15 selection is preserved.
No Cargo package version, index, sibling checkout or release state is mutated.

## Dependency cleanup and ownership

The published runtime retains Metrics for arithmetic, Platform for executor/IC0,
and Thiserror for typed errors. Candid, full CDK, Wire and simulator tools remain
excluded from its normal/build graph. Executor **2.0.0** and IC0 **1.2.0** retain
their qualified registry identities. No new provider, codec or fallback is added.

Platform, Testkit, Wire and the probe protocol move to **dev-dependencies** of the
native driver: only its `cfg(test)` harness/cohorts use them. The normal build
coordinator retains Build/Host process and artifact ownership, Clap argument
parsing and Serde/JSON receipts. On Linux its normal/build graph falls from
**232 to 53** distinct package identities, including the root. Those packages
remain available to tests; the workspace graph/lock and deployed runtime graph
are not reduced by this move. The numbers are graph evidence, not size/timing claims.

The existing Shared graph checker now admits the native coordinator separately.
A negative fixture rejects promoting Testkit back into its normal dependencies;
all existing library/probe/protocol feature, alias and source guards remain.
The root catalog remains the sole dependency catalog, including test dependencies.

Platform's executor/IC0-only profile still selects unused Principal, Serde and
crypto dependencies. Published **0.5.1** and its archive checksum confirm the
limitation, tracked at [#40 — Keep executor/IC0-only graphs free of Principal,
Serde and crypto dependencies](https://github.com/dragginzgame/ic-platform/issues/40).
Keep the canonical Platform dependency until its owner supplies a published lean
profile. Do not copy bindings or restore direct CDK/IC0 declarations here.

The probe entrypoint adapters retain application Wire IDs, byte/allocation limits
and traps. Testkit's typed Wire helpers require finite `Wire` contracts; they do
not directly replace these bounded native Serde payloads. A protocol rewrite only
to remove two small adapters would add migration work. Candid remains native
through PocketIC/Testkit; deleting those dependencies would remove real IC evidence.

## Focused qualification

Locked preparation fetched only already-selected missing dependencies. Rust 1.91
probe/driver/protocol Clippy and all four cohort feature selections pass. Grouped
formatting, direct role/inheritance checks, native/Wasm graph admission and all
negative boundary fixtures pass. The normal coordinator's help command and six
native measurement regressions pass. Fresh watchdog qualification passes all 27
product subjects plus three host fixtures; all four cohorts and both calibrations
pass, refreshing `testing/target/cohorts.json` for the current lock.
Initial offline inspection/tool admission found unprepared selected dependencies
and executables; explicit canonical preparation and the offline tool recheck pass.
Evidence, graph before/after and incoming before-images:
`/tmp/ic-timers-dependency-cleanup.rxc6e_k7/`.

Earlier complete watchdog/cohort evidence at `/tmp/ic-timers-deps.ftdt0lxm/` belongs
to Build 0.24/Host 0.32/Testkit 0.60, not this new graph. Historical source-bound
Platform and Shared evidence remains at `/tmp/ic-timers-platform-05.snizl9v_/`,
`/tmp/ic-timers-shared-0257.tf16fmhr/` and `/tmp/ic-timers-shared-0256.pnwelhot/`.
Native macOS, installed IC and the complete maintainer-operated release gate remain
separate. Version bumps, staging, commits, tags, pushes and publication remain
maintainer-owned. No other repository is edited; only the authorized Platform issue
is created upstream. Current architecture and safety claims retain their owners.
