![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Current released source is **0.17.4**
`d7ae76c8b23ea4b0a868eb86330531ef8bd70116`. Its committed graph selects four local
0.17.4 packages, Metrics **0.5.3**, Testkit **0.32.2**, all four Host packages at
**0.12.4** and PocketIC **16.1.0**. Its
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38060308653)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38060308649)
is queued at observation. Job states alone are not raw-log proof of completed
current/native qualification. Do not transfer earlier evidence or redispatch jobs.

Earlier **0.17.0** has downloaded complete Linux/MSRV/tag and both macOS evidence;
its [release record](../releasing.md#0170-release-acceptance) owns the source-bound
closure of [#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37). Public measurement
identity remains with the selected Metrics package or
`ic_timers::MeasurementSummary`, recorded by the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph).

## Pending 0.17.5

All three canonical exports now select committed Shared **0.3.6**
`0604bfd730ec7ec288cd2cfdad217a0d42bf256b`, with **56/30/11** files. The
[adoption owner](../shared-tooling.md#shared-tooling-036-hook-and-lock-selection)
records clean detached source, canonical export logs, preserved inputs and
failure-isolation cases at `/tmp/ic-timers-shared036.u0iglx_p/`.
[Producer CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38061078001)
is queued at observation; that is not consumer/native qualification.

The shared hook preserves failed initial/post-snapshot/post-format tree reads
before copying or staging, including matching output with failed status. The
consumer fixture adds empty/matching failure cases at all three boundaries.
The maintainer's gate reached the first hook case and exposed an overly strict
raw-index assertion: only Git's `TREE` cache changed, with all 263 entries intact.
The fixture now checks staged paths/objects/modes/stages, plus its existing file,
diff, failure-status and stopped-command assertions. Failed fixture
`/tmp/timer-hook-test.lcNSLP/` remains; diagnosis and inputs are at
`/tmp/ic-timers-hook-index-cache.nbhv05iq/`. No contributor rerun occurs.
The Testkit adapter now delegates its absolute root lock to the shared Cargo
installer, removing the duplicate parser and parser-specific local cases.
Shared fixtures own strict package/source/version admission and selection changes;
consumer cases cover argument/lock preservation, failed admission before server
dispatch, offline policy and ordered Make/jobserver failure propagation.
Cases are written, not run. Binaryen **132** pins remain; no Node/optimizer gate,
optional adoption checker, task scheduler or new server route is enabled.

Incoming maintainer-owned Cargo changes select Testkit **0.33.0**, Host **0.12.5**
and Metrics **0.5.4**. All local packages remain 0.17.4. These inputs are preserved
and resolve with locked offline metadata; that does not qualify CLI/server or
product execution. The [release owner](../releasing.md#0174-delivery-and-shared-036-preparation)
separates this graph from released 0.17.4.

The later [Host review](../releasing.md#host-0125-review) confirms its NUL-path
fix and byte-exact published source, without adding a direct dependency.
Testkit 0.33.0's separately installed CLI still builds with its shipped Host
0.12.4 lock; the workspace graph alone cannot update that executable's internals.
Wait for a published, qualified Testkit update through the existing CLI owner.

[#40](https://github.com/dragginzgame/ic-timers/issues/40) is implemented through
canonical adoption and remains open for committed consumer current/native
qualification. [#38](https://github.com/dragginzgame/ic-timers/issues/38) and
[#39](https://github.com/dragginzgame/ic-timers/issues/39) were implemented in
0.17.3 and also await source-bound native evidence. Their preparation remains at
[its owner](../releasing.md#local-fixture-completion-follow-up); do not relabel
inspection or historical acceptance as execution of these new boundaries.

The compatible tooling batch selects an undated **0.17.5** changelog. This is
repository-only work with no timer/public API or measured Wasm/instruction change;
normally bundle it into later code-bearing work. An explicit maintainer release
retains the complete gate. Snapshot integrity, syntax (41 scripts/27 embedded
Bash bodies), locked metadata and preserved catalog/lock/pins/index checks pass.
Tests/builds/lint, setup, Cargo mutation, staging/commits and all release execution
remain user-owned.
