![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and acceptance

The maintainer reports **0.17.0** live at
`5e0d0865248f6ebfc1f98c896581f21e2ce67831`; its annotated tag matches that commit.
All four local packages select 0.17.0. The released graph is Metrics **0.5.0**,
Testkit **0.31.0**, Host **0.11.0** and PocketIC **16.1.0**, with Shared **0.3.0**
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` snapshots (55/30/11).

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38048325468)
passes Linux and MSRV;
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38048325379) passes.
Both native macOS jobs remain queued at observation. Downloaded completed-job
REST logs prove the actual released Linux fixtures, library tests and maintained
recovery/cohorts. The [release owner](../releasing.md#0170-release-acceptance)
records exact jobs, graph and proof limits. Earlier host results do not qualify
this graph. Do not redispatch CI or treat queued jobs as acceptance.

[#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37) remain open for complete
native qualification; both adoptions are delivered. Current guidance supersedes
0.17.0's earlier Metrics 0.4/Testkit 0.30 preparation. Consumers exchanging public
measurement summaries must use the same selected Metrics package identity or
`ic_timers::MeasurementSummary`. The
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
records seven matching published Metrics and 40 matching Testkit Rust sources,
unchanged arithmetic and no required timer adapter. Testkit's packaged CLI also
selects Host 0.11; Host/Testkit remain outside the timer library closure. Read-only
clean IcyDB source/lock inspection now finds one Timers 0.17/Metrics 0.5 graph;
that is downstream convergence evidence, not new compilation. Delivered-source
feedback is posted on both adoption issues; their macOS proof remains pending.

The README freshness gate and automatic rewrite/staging are removed in 0.17.0.
Only Cargo.toml, Cargo.lock and CHANGELOG.md are release outputs. README edits
retain ordinary source/staging admission; stale examples never select release
identity. [Shared #100](https://github.com/dragginzgame/shared-tooling/issues/100)
requests periodic read-only advisory review; no task or scheduler is enabled.

## Pending 0.17.1

All three canonical snapshots now select committed Shared **0.3.1**
`fa452afaa5012866eb1c20820dfa8038c106e7ec`, preserving the 55/30/11 selections.
The [adoption owner](../shared-tooling.md#shared-tooling-031-setup-preflight)
records read-only IC/Rust preflight before downloads and exact host-tool
failure diagnostics. The existing local Testkit fixture admits both preflight
calls, their order and stop-on-failure prefixes under parallel Make. New cases
are written, not run. Source/export and preserved input evidence remains under
`/tmp/ic-timers-shared031.aFkkRB/`. Producer Linux regression and lint/security pass,
with both native macOS jobs queued at observation;
released 0.17.0 proof does not qualify this new snapshot. Consumer Cargo, root
lock, tool pins, real index and existing installations are unchanged.

## Next action and authority

Finish source-bound native acceptance for the released adoption issues as jobs
complete, then close their owning issues with actual proof. The new snapshot
requires the normal full user-operated gate. No timer runtime feature, dependency
update or measured Wasm/instruction saving is added. This batch is repository-only
and normally bundles into the next code-bearing release; the undated 0.17.1 draft
records it without mutating Cargo. Tests/builds/lint/setup and Cargo/release effects
remain user-owned. Preserve installations, pins, index and failed evidence.
