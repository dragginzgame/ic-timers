![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Current released source is **0.17.6**
`3174935874796c3ffae30b0dfcf9c0c8f4557322`. Its committed graph selects four local
0.17.6 packages, Metrics **0.5.6**, Testkit **0.33.1**, all four Host packages at
**0.12.9** and PocketIC **16.1.0**. That release's Shared exports select
**0.3.8** `67285b28a98b7c4211ad32de726709d4e87edea4`, with **56/30/11** files.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38073888597)
and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38073888232)
complete successfully at that exact SHA.

The [0.17.6 acceptance owner](../releasing.md#0176-release-acceptance) records
source-bound Linux/MSRV and both complete native macOS gates. Downloaded raw
logs, run/job metadata, hashes and matched line references remain at
`/tmp/ic-timers-0176-acceptance.EkYVSQ/`. Linux and both macOS logs execute the
fixture completion/contradiction, collector exit/retention/archive, actual-hook
failure isolation and release-admission cases, plus 142 library tests, 14
maintained recovery subjects and four policy cohorts. Both macOS gates admit
Apple's Bash 3.2 and end with `VALIDATION PASSED`.

This closes the consumer qualification obligations in
[#38](https://github.com/dragginzgame/ic-timers/issues/38),
[#39](https://github.com/dragginzgame/ic-timers/issues/39),
[#40](https://github.com/dragginzgame/ic-timers/issues/40) and
[#41](https://github.com/dragginzgame/ic-timers/issues/41). Older cancelled runs
are excluded. This ordinary CI run skips intentional early/late live-workflow
failure qualification; no new artifact round trip is claimed.

Public measurement identity remains with the selected Metrics package or
`ic_timers::MeasurementSummary`, recorded by the
[identity owner](../design/callback-delivery-ownership.md#ic-metrics-05-released-graph).
Testkit's installed CLI has its own packaged locked graph; workspace Host
selection does not override it. Retain the existing Testkit owner and no direct
Host dependency. Consumer-owned Binaryen **132** pins remain unchanged.

## Pending 0.17.7

The three canonical exports now select reviewed Shared **0.3.11**
`db143ae9ae140eb66826c413360723d23c078f66`, retaining **56/30/11** files.
The producer corrected its version annotation in
[Shared #114](https://github.com/dragginzgame/shared-tooling/issues/114); the
canonical exporter now reports 0.3.11 with no local relabelling.
The [adoption record](../shared-tooling.md#shared-tooling-0311-portability-setup-and-diagnostics)
records clean detached source, preserved inputs and export logs at
`/tmp/ic-timers-shared0311.IIBZGj5v/`. Existing dirty 0.17.6 acceptance records
are preserved. Optional commands, Binaryen **132** and all Cargo/lock selections
remain unchanged, including the incoming Host 0.12.10 lock selection.
Shared #111/#112 repair physical fixture paths and relative child temporary
directories in already-selected release checks. Producer lint passes; Linux and
both macOS jobs are still running at observation. The cancelled 0.3.10 run does
not qualify its complete host matrix. Shared 0.3.11 commits the diagnostic
fixture spacing correction prompted by Timers' feedback; our consumer already
has spaced cases, and that producer integration fixture remains unselected.
Selected payload bytes are unchanged by this last provenance/version refresh.
The derive-macro formatter defect
has no affected maintained Timers macro; no markers or pin change are needed.

[Timers #42](https://github.com/dragginzgame/ic-timers/issues/42) tracks retained
release-summary diagnostics, applying Shared #110's reproduced gap. The existing
release-gate fixture covers the actual adapter, both search backends, separated
diagnostic families, long ordinary tails, literal-prefix and Rust-name controls,
raw/combined logs and GitHub summaries. The selected shared Rust installer and
fixture adopt Shared #101's consistent consumer/toolchain configuration. See
[the release owner](../releasing.md#shared-0311-portability-diagnostic-and-setup-adoption).
Snapshot integrity, shell/embedded/recipe syntax, links, diff whitespace and
locked offline metadata checks pass, with catalog/lock/pins/index preserved.
Cases are written, not run; #42 remains open for committed consumer current/native
qualification. Producer CI success and earlier 0.17.6 evidence cannot replace it.

Host **0.12.10** changes setup tooling, with no library Rust source change from
0.12.9. Timers adopts the shared fix directly, retaining Testkit **0.33.1**,
the incoming Host **0.12.10**, Metrics **0.5.6** and PocketIC **16.1.0** root lock
selections. Exact-source Host CI passes Linux/MSRV and both native macOS jobs;
consumer qualification of the changed graph remains separate.
No direct Host dependency or new CLI/server route is needed.

One undated **0.17.7** changelog covers this compatible repository tooling batch;
Cargo remains 0.17.6. There is no timer/API change or measured product Wasm or
instruction delta. Normally bundle repository work into code-bearing changes;
a maintainer-selected release retains the complete gate. Tests/builds/lint,
setup, Cargo mutation, staging/commits and release execution remain user-owned.
