![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-22

## Source and accepted work

Released base: **0.30.1**, `6c3aa21`; package versions remain unchanged. The
undated **0.30.2** draft covers compatible upstream adoption and repository
qualification cleanup. Public timer types, protocols and lifecycle contracts are
unchanged. There is one root catalog/lock and five maintained packages.

Incoming lock selections are preserved byte-for-byte: Build/Host/Testkit
**0.63.3**, Platform/macros **0.6.7**, Metrics **0.15.4**, Wire/derive **0.5.19**
and Tokio **1.54.1**. Official registry indexes confirm the selected IC releases
are latest non-yanked publications at inspection. All ten IC package archives
match registry/lock checksums and the compiled-cache files. No unpublished sibling
code is selected. Metrics arithmetic, executor/IC0 implementations and Platform's
macro sources match the prior selections. New optional storage/Candid features
remain unused; no memory/storage migration is included.

## Shared ownership cleanup

Canonical preview/apply adopts reviewed Shared **0.26.6**,
`4cb9bc907154225a3f7e4c33fd8d75fbeaa125c8`, with **134** payloads. Four generic
command/formatter/Make-interface/LOC test roots and their unused jobserver probe
retire. The new CI/release audit method resolves its catalog link without
activating an audit. Surviving payloads remain supplier-owned and verified.

This implements the fix prepared in [PR #73](https://github.com/dragginzgame/ic-timers/pull/73)
for [#74](https://github.com/dragginzgame/ic-timers/issues/74) locally. Neither is
merged/closed here; default-branch delivery remains required. Timers retains
actual setup/admission, hook/index, repository orchestration, snapshot corruption,
release/recovery, evidence and product qualification. Shared's generic producer
matrices are no longer routine Timers release prerequisites.

Shared's compact collector now requires an explicit IC executable list. Timers
passes an empty list, preserving host-only compaction and full legacy IC evidence.
It selects no IC executable bundle; Testkit still owns PocketIC setup/admission.
Build's editor and Testkit's server CLI retain explicit `cli` selections in the
local adapters, without enabling CLI features in workspace library dependencies.
Both 0.63.3 executables are explicitly prepared and pass offline admission.

## Retained product boundaries

Timers owns scheduling, claims and observations; Platform owns IC bindings,
Wire codecs, Testkit simulator lifecycle, and Build/Host artifact/process machinery.
Applications retain durable intent and lifecycle policy. The library selects only
Platform executor/IC0, with one registry executor **2.0.0** and IC0 **1.2.0**.
Its independent Wasm normal/build graph remains **14** package identities;
the native build coordinator remains **53**. These are graph counts, not
performance claims. Library/probe graphs exclude Candid/full CDK and host tools;
the library also excludes Principal/Serde/crypto. Candid remains native-only
through PocketIC. Protocol records select Wire/Serde only; simulator/codec
integration stays dev-only in the coordinator.

## Current qualification

Focused Linux checks pass: library Clippy, Rust 1.91 probe/protocol/driver lint
and all four size feature selections, grouped formatting, native/Wasm MSRV builds,
declaration/feature/graph policies and negative boundary fixtures. Three protocol
round-trip/admission tests, six Metrics projection tests and 7,776 generated native
command traces pass. These native tests do not establish IC recovery guarantees.

Live PocketIC qualification passes 27 product subjects plus three host fixtures,
all four policy cohorts and two calibrations. The generated cohort receipt matches
the preserved root lock. No cross-context performance improvement is claimed.
Tool setup/offline checks, snapshot corruption, fixture retention, failure evidence,
repository/release ordering, hook/index preservation, metadata/version/changelog
fixtures, checksum checks, ShellCheck and whitespace checks pass.

Incoming before-images, preview/apply logs, published archives, source comparisons
and qualification logs: `/tmp/ic-timers-upstream-qloog2ew/`. Native macOS,
installed-IC qualification and the complete maintainer-operated release gate remain
separate. No staging, commits, pushes, package version changes or releases were
performed. Other repositories remain read-only.
