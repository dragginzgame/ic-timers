![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-15

## Prepared 0.24 hard cut and owner reductions

Released base is 0.23.2 at `227c51e085c328eb4ef682d554bf638f9f647843`.
Cargo versions remain unchanged; incoming Testkit 0.45 manifest/lock edits are
preserved. The unpublished harness now uses the selected Testkit fallible-only
Candid/install API with explicit test expectations. The single undated 0.24.0
draft retains the prepared package-local
MIT license fix and adds this public API cut.

`TimerObservabilitySnapshot::epoch()` and its duplicate stored epoch are removed.
Read the epoch through `TimerSnapshot::registration_id().epoch()` and keep the
registration ID alongside copied statistics. The registry owns the epoch and
supplies it to standalone and inventory snapshots. Existing continuity tests
cover cancel, replacement and restart; policy snapshots now also compare both
projection paths. No timer scheduling, callback or recovery semantics change.

All three canonical Shared bundles select reviewed contribution
`58bad48b701fbcc32d932ac60969aa347bb61c71`, integrated Shared 0.15 plus #209,
with 85/43/18 payloads. The previously isolated #54 caller changes are adopted:
Shared owns metadata mutation/rollback and safe partial tool-evidence collection.
The completion suite delegates prefix rendering while retaining its fifteen
actual fixture boundaries, path/failure matrix and selected mandatory assertions.
See the [adoption owner](../shared-tooling.md#prepared-024-owner-adoption).

The [release guide](../releasing.md) is current procedure. Its former receipts
remain in [release history](../evidence/release-history.md); the preceding full
handoff is [archived](archive-2026-10-15.md). Historical source/graph qualifications
do not admit this new batch. No Wasm or instruction saving is claimed.

## Qualification and remaining delivery

Focused results and custody are retained at
`/tmp/ic-timers-epoch-docs.z5u0v9wt/`. Native library tests, strict library Clippy,
Bash 5/genuine Bash 3.2 completion, version rollback and evidence fixtures pass.
Rust 1.88 native/Wasm library and probe compilation, probe Clippy, doctests,
public docs, snapshot integrity/corruption, affected release/Make fixtures,
Python evidence tests and moved-link checks pass. The initial probe failure from
removed Testkit convenience methods is retained alongside its successful repair. No setup, live PocketIC, full owner
release gate, hosted dispatch, version mutation or normal Git mutation is included.

The last completed all-host receipt is [0.23.1](../evidence/release-history.md#completed-0231-native-acceptance),
not this source or the incoming Testkit 0.45 graph. Issues #51/#52/#55/#59 and
superseded PR #56 are closed with that exact receipt. [License #62](https://github.com/dragginzgame/ic-timers/issues/62)
awaits delivery. [Snapshot #63](https://github.com/dragginzgame/ic-timers/issues/63)
owns the separate verified ownership handover; three bundles remain here.

PRs [#54](https://github.com/dragginzgame/ic-timers/pull/54),
[#60](https://github.com/dragginzgame/ic-timers/pull/60),
[#61](https://github.com/dragginzgame/ic-timers/pull/61) and
[Shared #209](https://github.com/dragginzgame/shared-tooling/pull/209) retain their
isolated review branches. #54's source is now adopted locally; the two Actions
pin changes remain separate. No PR merge is claimed. Previous repair receipts
remain at `/tmp/ic-timers-issue-pr-followup.k9op1zwx/` and in the archive.

Normal staging, commits, versions, tags, pushes, publication and cleanup remain
maintainer-owned. Preserve Cargo inputs, index/refs and retained evidence.
