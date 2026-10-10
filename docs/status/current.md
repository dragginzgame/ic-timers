![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This is the compact session handoff. Historical implementation and acceptance
belong with their [release owner](../releasing.md), [adoption owner](../shared-tooling.md)
and [safety boundary](../../SAFETY.md). The previous handoff is preserved verbatim
in [the historical archive](archive-2026-10-10.md).

## Released 0.16.6

The maintainer reports **0.16.6** live. Its release commit is
`0b929539686a5c428a6a3af96c2a88139cc5553d`; all four local members are 0.16.6.
The single root graph selects Testkit **0.28.1**, Host **0.10.2**, Metrics
**0.3.7** and PocketIC **16.1.0**. All three released snapshots select Shared
**0.2.13** `5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e`, **55/30/11** files.

Matching main MSRV and tag-truth jobs pass. Linux is running; both native macOS
jobs are queued at inspection. The [0.16.6 acceptance owner](../releasing.md#0166-release-acceptance)
records exact jobs and remaining scope. Do not reuse the completed
[0.16.5 three-host evidence](../releasing.md#0165-linux-and-tag-acceptance) for
0.16.6's new lock/path/preflight cases or graph, or dispatch duplicate jobs.

Only [#35](https://github.com/dragginzgame/ic-timers/issues/35) remains open
locally, for the released explicit staged/working Cargo.lock preservation
assertions on all three hosts. Original Make/format adoption acceptance is
complete. #34 and #30 remain closed with their separate completed qualification.

## Pending 0.16.7

The undated changelog batch adopts committed Shared **0.2.14**
`fd11692f31e7dfd44dcc2ca56634eaeab3569825`, retaining **55/30/11** selections.
The [adoption owner](../shared-tooling.md#shared-tooling-0214-ci-inspection)
records the clean canonical exports and CI inspection fix. Empty failed-step
logs now produce an explicit evidence gap for failed/unfinished runs; failed
fetches retain partial logs and their original status. This adds no timer API,
runtime change or local copy of producer tests. The same batch records incoming
Testkit 0.29 / Host 0.11 dependency selections separately below.

Cargo versions remain 0.16.6. Incoming maintainer catalog/lock edits and unchanged
consumer pins/index are preserved. Snapshot integrity, source/mode/companion,
syntax, documentation and locked offline metadata checks are preparation only.
Contributor tests/builds/lint/setup and all Cargo/release effects remain
user-owned. The normal user-operated gate qualifies the pending adoption.

## Upstream ownership and next action

Timers uses all four Host crates through the unpublished native Testkit harness;
the timer library has no Host/Testkit dependency. Testkit's separately installed
CLI uses its own packaged lock, independently of the root graph.
[The earlier Host ownership review](../releasing.md#host-011-ownership-review)
traced the migration to its Testkit owner. No direct Timers dependency is needed.

Testkit **0.29.0** is now published and on remote main at
`e15cc2acfd9324f6877f854415005f91d169a031`. Maintainer edits select it and all four
Host **0.11.0** packages. Its packaged CLI lock also selects Host 0.11.0, while
PocketIC remains 16.1.0. The existing harness's public startup calls and CLI
setup/check contract need no adapter. The
[preparation owner](../releasing.md#testkit-029-and-host-011-preparation) records
published source equality, the locked graph and remaining execution qualification.
[Testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47) owns upstream
migration acceptance. Metrics remains 0.3.7; no metric adapter or timer feature is
justified by this pass.

Next: finish source-bound 0.16.6 all-host acceptance and close #35 when its
remaining proof passes; qualify the newly selected Testkit/Host graph through the
normal user-operated gate.
Pending 0.16.7 is repository-only maintenance and normally bundles with the next
code-bearing release; an explicit maintainer release still retains the full gate.
