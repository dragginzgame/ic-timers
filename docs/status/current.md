![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

The maintainer reports **0.17.3** live at
`4cc3c64e6b773d73edb66f6bdf2d91cce5e68790`. Its committed graph selects four local
0.17.3 packages, Metrics **0.5.3**, Testkit **0.32.1**, all four Host packages at
**0.12.3** and PocketIC **16.1.0**. All three snapshots select Shared **0.3.3**
`d63f0cfaba8ab2961d6012064adbf051c1898bc1`, with **56/30/11** files. The
[adoption owner](../shared-tooling.md#shared-tooling-033-validation-completion)
records canonical runner completion and nesting-depth admission.

The [main run](https://github.com/dragginzgame/ic-timers/actions/runs/38056525080)
has Linux **114226065101** running and MSRV **114226065026** passing; ARM
**114226065141** and Intel **114226065188** are queued at observation. The
[tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38056525399)
has tag-truth **114226066046** running. Job states alone are not raw-log proof of
completed current/native qualification. Do not transfer earlier release evidence
or redispatch jobs.

Earlier **0.17.0** has downloaded complete Linux/MSRV/tag and both macOS evidence;
its [release record](../releasing.md#0170-release-acceptance) owns the source-bound
closure of [#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37). Public measurement
identity remains with the selected Metrics package or
`ic_timers::MeasurementSummary`, recorded by the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph).

## Open issues

[#38](https://github.com/dragginzgame/ic-timers/issues/38) and
[#39](https://github.com/dragginzgame/ic-timers/issues/39) are implemented in
0.17.3 and remain open for source-bound current/native Bash qualification.
The [preparation owner](../releasing.md#local-fixture-completion-follow-up)
records all sixteen local fixture completion boundaries, six exit cases each,
collector completion/retention cases and the preflight reader status repair.
Preparation source/syntax evidence remains at
`/tmp/ic-timers-fixture-completion.9z8952mm/`,
`/tmp/ic-timers-collector-completion.vb52tc9h/` and
`/tmp/ic-timers-preflight-reader.1hfppy0_/`. None of these inspection checks is
fixture execution evidence.

New [#40](https://github.com/dragginzgame/ic-timers/issues/40) tracks canonical
adoption of [Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106).
Latest committed Shared is **0.3.5**
`a744d7f1990b9e1451ef45cd6d495de00a141cd3`; the sibling's 0.3.6 hook repair is still
uncommitted. Retain the immutable hook and all three reviewed 0.3.3 exports until
producer delivery. The [review owner](../shared-tooling.md#shared-tooling-035-review-and-pending-hook-repair)
records why 0.3.5 changes no selected executable payload. Consumer-owned Binaryen
**132** pins remain; unoptimized probe evidence does not qualify Binaryen 133.
No Node helper, optimization gate, task scheduler or direct Host dependency is added.

## Pending 0.17.4

The consumer-owned release-gate fixture now clears inherited retained/failure
log directory selections. Its deliberate synthetic failures stay within its own
evidence roots; real nested production checks retain their inheritance contract.
The [release owner](../releasing.md#0173-delivery-and-fixture-log-isolation)
records the concrete gap and preserved inputs at
`/tmp/ic-timers-fixture-logs.a34mj0d4/`.

This compatible repository-only fix selects an undated **0.17.4** changelog,
with no timer/API or measured Wasm/instruction change. Normally bundle it into
later code-bearing work; an explicit maintainer release remains sufficient
publication authority and keeps the complete gate. Integrity, syntax, local
links, diff and cheap locked metadata inspection are permitted. Tests/builds/lint,
setup, Cargo mutation, staging/commits and all release execution remain user-owned.
