![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-13

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.21.1 is committed/tagged/pushed** at
`a97dcb4b5f091d98afd35038a4cbeb15fba1d7d6`. The completed standard direct-release
receipt binds validation to source `f935c706cfdb7e2554b0a2a95b06763efcc009c9`
and delivered tree `a971b09581c1433747caae7a20195cdd30bcd9b2`.
The [delivery owner](../releasing.md#0211-delivery-and-prepared-0212-tooling)
records owner-operated full-gate acceptance separately from earlier failures and
later contributor changes. No successful raw log or agent replay is fabricated.

Delivered graph: Metrics **0.8.0**, Testkit **0.35.0**, Host artifacts/fs/process
**0.15.1** and PocketIC **16.1.0**, with Shared **0.9.7** across **67/35/11**
files. The neutral-cwd Make adapter avoids the canonical Rust-tool fixture's
mock compiler/caller toolchain conflict while preserving rejection subjects,
failure status and retained evidence. The original failed attempt at `6483ffc`
remains at `/tmp/rust-tools-test.C9TDs0` and in its original release log.

The 0.21 runtime still shares Metrics 0.8 summary identity. First lifecycle
reconciliation publishes a fresh slot only after the requested state succeeds;
failed creation releases its callback and handles before retry. Later errors
retain existing authority and unused-factory destruction ordering.
The [lifecycle](../design/0.20-typed-registration.md#prepared-021-lifecycle-creation)
and [Metrics](../design/callback-delivery-ownership.md#prepared-ic-metrics-08-adoption)
owners retain earlier focused evidence separately from delivered acceptance.

Exact-source hosted [run 38288927727](https://github.com/dragginzgame/ic-timers/actions/runs/38288927727)
and [run 38288927426](https://github.com/dragginzgame/ic-timers/actions/runs/38288927426)
now pass. Those results qualify delivered 0.21.1, not later dirty tooling.

## Prepared 0.21.2 maintenance

All three canonical snapshots select reviewed Shared **0.9.10**
`7e55adecf6f0b196c050f104b37f8c69b6bf434e`, with **68/36/11** payloads.
Complete clean-source previews precede apply. Add the required reporting rule as
an explicit root; retain all other roots and the 0.9.8 cache fixture/helper.
No canonical retirement or patch. The new reporting contract separates genuine
deletions, moves/renames and scoped Rust/type/documentation counts.

The canonical fixture now enters its prepared compiler context and borrows
existing source history without creating commits. Remove the temporary Make
adapter and invoke the complete canonical fixture directly. Hook, receipt,
partial-staging and failure-retention checks remain. The 0.9.9 producer fix passes
upstream Linux and both macOS qualification; 0.9.10 CI is in progress. Consumer
execution of these new bytes remains unqualified. Shared's CI cache transport and
fleet-wide suite are not activated. Full local gates remain unchanged.

[Adoption evidence](../shared-tooling.md#prepared-shared-tooling-0910) remains at
`/tmp/ic-timers-shared0910.CyrZ3Vtp/`: clean source, before inputs/index, complete
previews/applies and integrity logs. Integrity, changed Bash syntax and diff
checks pass. Cargo inputs, pins and real index entries are preserved. No tests,
setup, Rust build/lint or release execution for this batch. Published timer source
is unchanged. Undated **0.21.2** notes are compatible repository-only work,
normally bundled into the next code-bearing release. Cargo stays at 0.21.1.
No tool reinstall is required solely for this refresh: installer/build-policy
bytes and executable pins are unchanged from 0.9.7. Earlier 0.9.8 preparation and
original failed fixture evidence remain at their existing owners.

## Issues and authority

The current issue inspection finds **no open Timers issues**.
[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
remains closed from delivered Metrics adoption at 0.21.0.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains closed from
frozen sequential v0.19.1 early/late acceptance; original controlled failures,
artifact identities and restored/missing evidence stay with the release owner.
Shared fleet [#98](https://github.com/dragginzgame/shared-tooling/issues/98)
still has other consumers' obligations. No qualification dispatch is pending.

Do not invent another product hard cut. Continue bounded issue/CI inspection
when requested. Siblings remain read-only. Tests, builds, lint, PocketIC and setup
remain user-owned outside explicit scoped approvals; prepared locked/offline
library compile-only checks retain their AGENTS.md exception. Cargo version
mutation, staging, commits, tags, pushes, publication and cleanup remain
user-owned. Preserve incoming edits and retained evidence.
