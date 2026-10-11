![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Current released source is **0.17.7**
`1d21b9f55a8d9121355d216471495f92b0db7865`. Its committed graph selects four local
0.17.7 packages, Metrics **0.6.0**, Testkit **0.33.2**, all four Host packages at
**0.12.10** and PocketIC **16.1.0**. All three Shared exports select **0.3.11**
`db143ae9ae140eb66826c413360723d23c078f66`, with **56/30/11** files.

[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38123424158)
passes for that exact SHA. In
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38123424231), MSRV
and Linux checks pass and both macOS jobs are queued at observation.
Tag-only success does not qualify the complete gate.
[Issue #42](https://github.com/dragginzgame/ic-timers/issues/42) remains open until
source-bound Linux/current and both native macOS gates qualify its diagnostic
cases. Existing 0.17.6 acceptance closes #38–#41 only and remains at
[its release owner](../releasing.md#0176-release-acceptance), with downloaded
source-bound raw evidence at `/tmp/ic-timers-0176-acceptance.EkYVSQ/`.

Public measurement identity and the already-released Metrics 0.6 boundary are
recorded by the [identity owner](../design/callback-delivery-ownership.md#ic-metrics-06-released-graph).
The public Metrics identity/mean change landed in a patch in error; future such
hard cuts require a minor release. Do not restore shims or rewrite history.
Testkit owns its installed CLI's packaged lock and server admission; workspace
Host selection does not override it. No direct Host dependency is needed.
Consumer-owned Binaryen **132** pins remain unchanged. Shared **0.3.12** changes
unselected formatting integration helpers; no local caller adoption is justified.
Host **0.13.0** removes a lock helper Timers does not call; retain the existing
Testkit owner rather than adding a new Host route.

## Pending 0.17.8

[Issue #43](https://github.com/dragginzgame/ic-timers/issues/43) owns focused
qualification of the already-selected Metrics 0.6 behavior. New tests cover empty
and completed-zero instruction summaries, an exactly reached numeric cap, actual
sticky total overflow, retained latest/maximum values, copied snapshots and both
callback roles. The existing native registration-identity sequence now checks
completed measurements, preservation through cancellation, empty replacement/reset
summaries and a numerically admitted interval across different identities.
That admission demonstrates why callers must establish continuity separately.

README and public performance guidance distinguish tracked instruction overflow
from conservatively saturated event/page counters. Ordered summaries stay in place;
no API, production recording path, dependency selection or persistence changes.
Tests are written, not run. Native reset is not simulated IC recovery.
[The release owner](../releasing.md#metrics-06-consumer-qualification) records
preparation checks and the remaining full user-operated qualification.

One undated **0.17.8** changelog covers this test/documentation batch; Cargo remains
0.17.7. Normally bundle repository-only work into code-bearing changes; a
maintainer-selected release retains the complete gate. Tests/builds/lint, setup,
Cargo mutation, staging/commits and release execution remain user-owned.
