![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

The maintainer reports **0.17.2** live at
`1cc87a467c4552b95d8a9728334e4e29eff0bc4c`. Its committed graph selects four local
0.17.2 packages, Metrics **0.5.1**, Testkit **0.32.0**, all four Host packages at
**0.12.2** and PocketIC **16.1.0**. All three snapshots select Shared **0.3.3**
`d63f0cfaba8ab2961d6012064adbf051c1898bc1`, with **56/30/11** files. The
[adoption owner](../shared-tooling.md#shared-tooling-033-validation-completion)
records jobserver, fixture and production runner completion guards plus the
advisory README task; no task scheduler is enabled.

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38053169331)
passes Linux checks **114216248770** and MSRV **114216248652**; ARM
**114216248792** and Intel **114216248815** are queued at observation. The
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38053169356) succeeds.
These job-state observations are not complete native acceptance or downloaded
proof of every current fixture.

Earlier **0.17.0** now has passing Linux/MSRV/tag and both macOS jobs. Downloaded
Intel/ARM logs show actual complete tool setup/check, ordered Testkit failure
propagation, 142 library cases, all 14 recovery subjects, four policy cohorts and
final validation success at its exact source. The
[release record](../releasing.md#0170-release-acceptance) owns this evidence for
[#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37), now closed with that
source-bound evidence.
Do not transfer that acceptance to the new graph or redispatch jobs.
Public measurement values still require the selected Metrics package identity
or `ic_timers::MeasurementSummary`; the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
records published source checks and downstream convergence.

## Pending 0.17.3

[#38](https://github.com/dragginzgame/ic-timers/issues/38) owns the remaining local
fixture completion repair. Fourteen CI/release fixture EXIT boundaries now
require successful status and explicit completion before cleanup, preserve
nonzero status and retain incomplete/failed inputs. Hook and release-gate
admission precede helper work. The
[boundary check](../../scripts/ci/test-fixture-completion.sh) covers all sixteen
local fixtures, including itself and Testkit, through copies truncated before
the fixture bodies. Six exit cases check status, successful cleanup and retained
evidence using the invoking Bash. The check is first in `release-check`, replacing
Testkit's separate exit probes. Cases are written, not run; current/native Bash
3.2 acceptance remains required. The
[preparation owner](../releasing.md#local-fixture-completion-follow-up) records
scope and evidence at `/tmp/ic-timers-fixture-completion.9z8952mm/`.

[#39](https://github.com/dragginzgame/ic-timers/issues/39) owns the related production
collector repair. Collection now requires explicit completion before successful
cleanup, preserving failure status and incomplete scratch evidence. The existing
collector fixture adds six actual-boundary cases truncated before Git/tool/archive
work. Script and generated-prefix syntax pass; cases are written, not run.
Separate preserved inputs/evidence are at
`/tmp/ic-timers-collector-completion.vb52tc9h/`. Current/native Bash qualification
for both #38 and #39 remains with the user-operated gate.

Release preflight now admits the workspace-version reader's status before its
value comparison. The existing index fixture adds failed empty/matching reads
and successful mismatch, checking exact status and no fetch/setup, metadata or
index mutation. Adapter, fixture and embedded-stub syntax pass; cases are written,
not run. Inputs are at `/tmp/ic-timers-preflight-reader.1hfppy0_/`. The analogous
shared-hook observation gap is tracked by
[Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106); its
immutable hook stays unchanged here, pending canonical producer repair.

The incoming maintainer-owned lock selects Testkit **0.32.1** under the existing
`0.32` catalog requirement. External updates during inspection advance all
four Host packages to **0.12.3** and Metrics to **0.5.2**; preserve that newer lock
rather than reverting
to the initial 0.12.2 copy. Preserve the Cargo catalog, pin catalogs,
index and existing installations/failure evidence. Qualification must use the
existing selected CLI/server path; there is no direct Host dependency or sibling
route. Locked offline metadata resolves this graph, without qualifying execution.
Shared's latest committed source is **0.3.4**; the
[review owner](../shared-tooling.md#shared-tooling-034-review-and-retained-optimizer-pins)
records Binaryen 133 and producer qualification still queued at observation.
Retain reviewed 0.3.3 snapshots and consumer-owned Binaryen 132 pins. Current
probe targets do not invoke `wasm-opt`, and existing unoptimized probe evidence
does not qualify a new optimizer. No new tool setup or Node/optimization gate
is added.

This is repository-only work with an undated 0.17.3 draft and no timer/API or
measured Wasm/instruction change. Integrity, syntax, documentation, diff and
cheap locked metadata inspection are allowed. Tests/builds/lint/setup and
Cargo/release effects remain user-owned.
