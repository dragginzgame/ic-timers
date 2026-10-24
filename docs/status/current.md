![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and pending work

Released base: **0.30.13**, `801b640`. Own package versions remain 0.30.13;
the undated **0.31.0** section covers the current dependency batch. Staging,
commits, pushes and release execution remain maintainer-owned.

Adopt published Metrics **0.16.0**, source revision
`c0f033a8f41c280fd0fc77b2ec934d2f2b8176ed`. The complete library Rust source
matches 0.15.8, but Timers publicly re-exports `MeasurementCounter` and
`MeasurementSummary`. Their Metrics 0.16 Rust identity requires a Timers minor
release; direct Metrics dependencies exchanging these values must align to 0.16.
The root catalog now selects 0.16. No aliases, conversion shim or compatibility
lane is added. Metrics' extracted stable adapter belongs to Persistence and is
not used by Timers; no storage migration or reset is needed.

Incoming lockfile edits already selected Build/Host/Testkit **0.68.1**. Official
archives verify source `c9c5ef7c44e3d0c3b23c8531a5d43c1800656c0a`; all five used
crates' complete Rust source matches 0.68.0. Their published dependency minimums
now align with selected Platform **0.6.15** and Wire **0.7.2**. These and their
macro/derive crates remain current; no unpublished sibling code is used.
Only Metrics changes relative to this task's incoming lockfile.

All direct registry dependencies were checked against official metadata.
Clap, Serde and JSON are current. Thiserror remains **2.0.18**, required exactly
by selected PocketIC 16.1.0, despite published 2.0.21. Shared Tooling is a reviewed
snapshot, not a Cargo dependency; this task does not refresh that separate owner.

## Ownership and maintained contracts

The published timer library retains three direct dependencies: Metrics for
arithmetic, Platform for executor/IC0 and Thiserror for typed errors. Executor
**2.0.0** and IC0 **1.2.0** identities are unchanged. No member directly declares
Candid or CDK-family crates. Library and probe deployment graphs remain
Candid-free. Native Testkit/PocketIC transport retains external Candid through
Platform. Platform's optional host Wire metadata is not enabled.

The protocol package selects Wire/Serde only. Testkit owns native typed transport
and errors; current method IDs, record layouts, resource budgets and trap
translation remain. Wire decoding requires only Deserialize and may normalize
collections/custom visitors; success is not proof of original byte identity.
The private platform boundary remains the sole one-shot timer/executor owner;
FFI retains callback registration. Native substitutes do not prove installed-IC
rollback, and this dependency update changes no timer lifecycle policy.

## Tooling and qualification

Both lock-selected Build/Testkit 0.68.1 CLIs are refreshed through canonical
setup; Testkit retains server selection/admission. Schema-2 cohort receipts still
require four Build recipe identities, alongside independent source/artifact,
lock/compiler/server and baseline-context checks. Baselines are not rewritten.

Shared Tooling remains reviewed commit
`4c0f198e56d4c6d27b12a5c2d515e0e01070151e`, with 137 payloads. Its committed VERSION
says 0.26.10; committed 0.26.11 notes contain setup phase/duration diagnostics.
The supplier's dirty VERSION was excluded. CI tool caching remains disabled.

## Verification and limits

Current before-images, official archives and focused logs:
`/tmp/ic-timers-deps-refresh/`. Rust 1.91 passes library/probe Clippy, 165 timer
tests, eight doctests, three protocol roundtrips, library/runtime/protocol and all
four size-probe Wasm checks. An isolated consumer compiles direct equality of
Timers' re-exported counters/summaries and snapshot getter results with Metrics
0.16. Declaration/deployed graph and provider guards, Testkit adapter checks,
both refreshed CLI offline checks and the release-editor fixture pass. Linux
PocketIC passes all 33 watchdog/recovery selections and the policy-cohort test,
including four policy artifacts and both watchdog calibrations. The fresh
schema-2 receipt at `testing/target/cohorts.json` matches the current lock and
retains all four recipe identities. No reviewed baseline was rewritten.

Earlier evidence remains at `/tmp/ic-timers-platform-adoption/` and
`/tmp/ic-timers-tooling-wire-adoption/`; their receipts describe their own lock
contexts. Older evidence at `/tmp/ic-timers-upstream-refresh/` and
`/tmp/ic-timers-cleanup-0311/` remains available. Retained failures, including
`/tmp/timer-qualification-971360-0/`, must not be removed. Issue acceptance and
hosted matrix evidence stay with their owning GitHub issues.

Native macOS, live deployments and the complete maintainer-operated release gate
are not qualified by Linux checks or cross-compilation.
