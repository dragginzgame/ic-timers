![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Pushed source and qualification

Latest pushed release source **0.17.1** is
`2a8710834c9283ccc46daa4e4c93b1cc4629c6bc`, with four local 0.17.1 packages,
Metrics **0.5.0**, Testkit **0.31.0**, Host **0.11.0** and PocketIC **16.1.0**.
Its three snapshots select Shared **0.3.1** `fa452afaa5012866eb1c20820dfa8038c106e7ec`
(55/30/11). [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38051205368)
passes Linux/MSRV; both native macOS jobs are queued. The
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38051205322) is
running at initial observation. Job-state inspection is not complete native
acceptance or downloaded proof of each fixture.

Earlier **0.17.0** has Linux/MSRV/tag acceptance and now a passing ARM job, with
Intel running at observation. Its downloaded Linux logs and exact graph remain
with the [release owner](../releasing.md#0170-release-acceptance).
[#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37) stay open for complete
source-bound native proof. Do not transfer earlier acceptance to a new graph or
redispatch jobs. Public summaries must use the same selected Metrics identity or
`ic_timers::MeasurementSummary`; the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
records published source checks and downstream convergence.

The incoming root catalog/lock now select Metrics **0.5.1**, Testkit **0.32.0**
and all four Host packages at **0.12.2**, with PocketIC **16.1.0** and four local
0.17.1 packages. Preserve those maintainer-owned bytes. Full locked offline
metadata now resolves; no fetch or dependency/version repair is performed.
The [preparation record](../releasing.md#0172-preparation-inputs) distinguishes
resolution from the user-operated setup/server/native/PocketIC qualification.

## Pending 0.17.2

All three canonical snapshots select committed Shared **0.3.3**
`d63f0cfaba8ab2961d6012064adbf051c1898bc1`. Selections remain **56/30/11**, including
the shared advisory README task. The
[adoption owner](../shared-tooling.md#shared-tooling-033-validation-completion)
records malformed nesting-depth admission and explicit runner completion before
success, retaining available evidence on premature exits. Earlier 0.3.2 jobserver,
standalone Make admission and fixture guards remain included. Local Cargo/Testkit
recipes retain descriptor handoff; the Testkit adapter fixture checks descriptor
accessibility and early-exit retention. New cases are written, not run. Current
source/export/input records are at `/tmp/ic-timers-shared033.0vrxoaq1/`; the earlier
0.3.2 records remain at `/tmp/ic-timers-shared032.rXciNI/`. The README task adds no
release gate, prose rewrite or enabled schedule. Incoming Cargo, lock, pins, real
index and existing tool/server installations remain unchanged by this inspection.

Producer Shared 0.3.3 CI is queued at observation. Remaining consumer-owned fixture
completion guards are tracked by [#38](https://github.com/dragginzgame/ic-timers/issues/38).
[Shared #104](https://github.com/dragginzgame/shared-tooling/issues/104) separately
owns producer qualification for the runner repair now included in this snapshot.
No earlier source's acceptance qualifies the new runner revision.

## Host and next action

Host's latest remote release is **0.12.2**; Rust source is unchanged from 0.11.0.
Its relevant Make/fixture improvements are adopted through Shared. The incoming
Testkit 0.32 graph selects Host 0.12.2 transitively and now resolves offline.
The [Host/Testkit owner](../releasing.md#host-0122-and-testkit-032-review) records
the prior source review; the current preparation record supersedes its wait for
selection. There is no direct Host dependency or sibling path. Follow the existing
root catalog/lock and owner CLI qualification workflow for the selected graph.

This is repository-only work with an undated 0.17.2 draft; no timer feature or
measured Wasm/instruction saving is added. Integrity, syntax, documentation and
locked offline metadata checks are allowed. Tests/builds/lint/setup and
Cargo/release effects remain user-owned.
Preserve incoming selections, installations, index and failure evidence.
