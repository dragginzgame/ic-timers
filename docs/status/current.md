![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Current released source is **0.17.5**
`d0fbdfb3288205f9f804c1706ed9ffa036dd2ea5`. Its committed graph selects four local
0.17.5 packages, Metrics **0.5.5**, Testkit **0.33.1**, all four Host packages at
**0.12.8** and PocketIC **16.1.0**. Its
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38071494892)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38071495004)
is queued at observation. Job states alone do not supply raw-log native acceptance.
Do not transfer historical evidence or redispatch jobs.

Earlier **0.17.0** has complete Linux/MSRV/tag and both macOS evidence;
its [release record](../releasing.md#0170-release-acceptance) owns source-bound
closure of [#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37). Public measurement
identity remains with the selected Metrics package or
`ic_timers::MeasurementSummary`, recorded by the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph).

## Pending 0.17.6

All three canonical exports select committed Shared **0.3.8**
`67285b28a98b7c4211ad32de726709d4e87edea4`, with **56/30/11** files. The
[adoption owner](../shared-tooling.md#shared-tooling-038-mandatory-assertions)
records clean detached source, canonical exports and preserved inputs at
`/tmp/ic-timers-shared038.yce385dv/`. Uncommitted sibling work is excluded.
Binaryen **132** pins remain; no optional archive qualification command,
optimizer/Node gate, scheduler or server route is enabled. Released 0.17.5's
latest-only CI concurrency stays unchanged.

[#41](https://github.com/dragginzgame/ic-timers/issues/41) repairs mandatory
Bash 3.2 assertions in two production release guards and eight local fixtures.
The macOS workflow also explicitly refuses incorrect host/Bash identity.
Explicit exits stop mismatched versions and invalid lock outputs before later
operations; contradictory fixture assertions cannot continue to completion.
Actual-source contradiction and lock-admission cases are written, not run.
The [release owner](../releasing.md#mandatory-bash-32-assertions) records scope,
negative cases and current/native acceptance requirements. Intentional boolean
predicates are preserved.

[#38](https://github.com/dragginzgame/ic-timers/issues/38),
[#39](https://github.com/dragginzgame/ic-timers/issues/39) and
[#40](https://github.com/dragginzgame/ic-timers/issues/40) remain open pending
source-bound current/native logs. Their implemented completion/collector/hook
boundaries remain intact. Retained hook cache diagnosis stays at
[its owner](../shared-tooling.md#shared-tooling-036-hook-and-lock-selection);
staged-entry preservation is checked independently of Git's TREE cache bytes.

An external lock update selects Metrics **0.5.6**, with no library source change
from 0.5.5. It is preserved; Cargo versions/catalog and release effects remain
user-owned. Testkit 0.33.1's cached published CLI lock selects Host **0.12.7**;
workspace Host **0.12.8** alone does not update that executable. No direct Host
import or override is needed.

This compatible batch has one undated **0.17.6** changelog; Cargo remains 0.17.5.
It is repository-only, with no timer/public API or product Wasm/instruction change.
Snapshot integrity, shell/embedded/generated/workflow syntax, diff whitespace
and locked offline metadata checks pass; no fixture or workflow body executes.
Catalog/pins/index and the separately captured external lock are preserved.
Normally bundle it into code-bearing work; an explicit maintainer release retains
the full gate. Tests/builds/lint, setup, Cargo mutation, staging/commits and release
execution remain user-owned.
