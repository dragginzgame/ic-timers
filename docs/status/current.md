![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.21.0 is committed/tagged/pushed** at
`a6b518722438d2f7e2dccd7ccfef07debc35b5b3`; the maintainer reports it live.
The completed standard direct-release record binds validation to source
`854e85f0a5d3129be3aa545352cfae4b2073ce12` and the delivered tree. The only
source-to-release changes are version metadata and changelog finalization.
The [delivery owner](../releasing.md#021-delivery-and-issue-50-acceptance)
records maintainer-operated native/MSRV/probe and PocketIC gate acceptance.
No successful raw log or agent replay is fabricated. The old backward-clock
fixture attempt remains failed; forward advancement is in the validated source.
Its separately requested replay is no longer needed.

Delivered graph: Metrics **0.8.0**, Testkit **0.34.7**, four Host **0.14.10**
and PocketIC **16.1.0**, with Shared **0.9.2** across **67/35/11** files.
Metrics types now share 0.8 identity and expose checked fractional means.
First lifecycle reconciliation publishes a fresh slot only after its requested
state succeeds; failure releases the exact creation before retry. Later errors
retain existing authority/callbacks and unused-factory destruction ordering.
The [lifecycle](../design/0.20-typed-registration.md#prepared-021-lifecycle-creation)
and [Metrics](../design/callback-delivery-ownership.md#prepared-ic-metrics-08-adoption)
owners preserve earlier focused evidence separately from delivered acceptance.

Exact-source [tag run 38283128868](https://github.com/dragginzgame/ic-timers/actions/runs/38283128868)
passes; full [main run 38283128647](https://github.com/dragginzgame/ic-timers/actions/runs/38283128647)
is queued at inspection. No complete new hosted/macOS acceptance is claimed.
The local release receipt does not qualify later contributor changes.

## Prepared 0.21.1 maintenance

All three canonical snapshots select reviewed Shared **0.9.7**
`4d10a06d50e44d302f00d5b7b706cac5f9b0127c`, retaining roots and **67/35/11**
payloads without additions or retirements. Complete clean-source previews
precede apply. Selected changes cover bounded transient downloads, complete Git
identities, strict included PR-helper identities and typed Cargo offline config.
Direct delivery remains the only adopted release route. The optional same-job
artifact-discovery helper is not activated in the cross-job matrix verifier.

Incoming maintainer Cargo changes select Testkit **0.35.0** and Host
artifacts/fs/process **0.15.1**; Metrics/PocketIC stay **0.8.0/16.1.0**.
Probe imports use the maintained PocketIC surface and need no artifact/Host-tools
replacement. No direct IC Build/Host dependency is added. The locked graph drops
from **299 to 288 packages**, a dependency count rather than measured savings.
All four maintained members still inherit the root dependency catalog.
Published Rust API/runtime and probe source are unchanged.

[Adoption evidence](../shared-tooling.md#prepared-shared-tooling-097) and
[graph scope](../releasing.md#prepared-0211-contributor-graph) retain source,
manifests, previews/applies, metadata, delivered receipt and inputs at
`/tmp/ic-timers-shared097.Oh8uAbQu/`. Integrity, changed Bash syntax, document
links, locked metadata and diff checks pass. Incoming Cargo files/pins and real
index entries are preserved. No product tests, lint, setup or full gate runs
for this new graph. Metrics' pending 0.8.1 restoration fix is not published or
selected; Timers produces summaries through recording and has no restoration
path needing a local replacement.

**Run `make install-tools` before validation:** the changed Cargo build-helper
identity requires explicit new tool selections, and Testkit's selected CLI/server
also remains owned by its existing setup route. Earlier generations are retained;
offline admission performs no repair or fallback. Setup stays user-owned.
Undated 0.21.1 notes are repository-only maintenance, normally bundled into the
next code-bearing release unless the maintainer explicitly chooses a tooling
patch. Cargo package versions remain 0.21.0.

## Issues and authority

[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
is closed from the delivered Metrics adoption and standard gate receipt.
The post-closure inspection finds no open Timers issues.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains closed from
its frozen sequential v0.19.1 early/late acceptance; original controlled failures,
artifact identities and restored/missing local evidence stay in the release owner.
Shared fleet [#98](https://github.com/dragginzgame/shared-tooling/issues/98)
still has other consumers' obligations. No qualification dispatch is pending.

Do not invent another product hard cut. Continue bounded issue/CI inspection or
qualify this contributor batch when requested. Siblings stay read-only.
Tests, builds, lint, PocketIC and setup remain user-owned outside explicit scoped
approvals; prepared locked/offline library compile-only checks retain their
AGENTS.md exception. Cargo version mutation, staging, commits, tags, pushes,
publication and cleanup remain user-owned. Preserve incoming edits and evidence.
