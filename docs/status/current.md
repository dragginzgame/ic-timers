![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This compact handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
Earlier accumulated history remains at its evidence owners and in
[the historical archive](archive-2026-10-10.md).

## Released source and qualification

Released **0.17.10** is `c4ce6480a2cbc656b35bde832d70c21befcf56db`.
Its committed graph selects Metrics **0.6.2**, Testkit **0.33.5**, all four Host
packages at **0.13.1**, PocketIC **16.1.0** and Shared **0.6.0**
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e` through **56/33/11** exports.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38137975652),
Linux checks and MSRV pass. Downloaded Linux execution includes the canonical
Cargo fixture, local collector cases, 144 library tests, 14 PocketIC recovery
subjects and four policy cohorts. Exact jobs/logs/digests are at the
[0.17.10 qualification owner](../releasing.md#01710-release-qualification).

Intel macOS fails in the shared Rust fixture after first common setup. Its
downloaded artifact matches the uploaded ZIP digest and exact source/host/run
identity, with exactly three retained common builds at the reported padded-count
boundary. ARM job **114465681030** now also fails at the same shared Cargo fixture
boundary; its downloaded log is retained with the new adoption evidence. This ordinary-failure artifact
inspection does not replace controlled early/late qualification. Keep
[#44](https://github.com/dragginzgame/ic-timers/issues/44#issuecomment-6109107659)
open for complete native acceptance. Evidence is retained at
`/tmp/ic-timers-01710-qualification.y69x5ito/`; no execution was rerun.

Earlier 0.17.8 has complete Linux/MSRV/Intel/ARM acceptance, closing #42/#43;
[its owner](../releasing.md#0178-release-evidence) retains source-bound logs.
0.17.9 main was cancelled. Neither earlier graph nor tag-only acceptance qualifies
new tooling/lock bytes. Native reset is not IC upgrade/rollback proof; retain
product PocketIC gates. The [Metrics identity owner](../design/callback-delivery-ownership.md#ic-metrics-06-released-graph)
records the 0.17.7 public-boundary mistake; future public hard cuts require a minor.

## Current prepared work

Pending **0.17.11** adopts committed Shared **0.6.1**
`731909525038c58d6f80b498e80f8630b11c685d` through **56/33/11** canonical exports.
[The adoption owner](../shared-tooling.md#shared-tooling-061-failure-and-native-fixture-corrections)
records previews, preserved inputs and required setup at
`/tmp/ic-timers-shared061.ynyutnkh/`. Explicit Cargo failure propagation and
acquired-lock cleanup address [Shared #120](https://github.com/dragginzgame/shared-tooling/issues/120);
numeric padded-count coverage addresses [Shared #124](https://github.com/dragginzgame/shared-tooling/issues/124).
All three snapshot integrity checks, changed Bash/stub syntax, links/diff and
locked offline metadata pass. Upstream lint/security passes; portable Linux is
running and native jobs queued. Keep #44 open for exact-source consumer acceptance.

Shared owns host/IC/Cargo storage and the standard tool-build environment. Run
`make install-tools`, then `make tools-check`; the changed build-policy hash selects
new generations. Common PATH is `.tools/rust/common/bin`. Use the same
`SHARED_TOOLING_STORE` for setup/checks/evidence; retain old installations and
failed attempts without fallback. Testkit executes the installer-returned,
root-lock-selected CLI and owns server/PocketIC admission. The local collector
keeps safe evidence and explicit incomplete status when the selector fails.
Consumer Binaryen 132 and other tool pins remain unchanged.

Incoming maintainer Cargo.lock already selects **Metrics 0.6.4 / Testkit 0.33.6 /
all four Host 0.13.4**, PocketIC 16.1.0. Preserve these incoming lock bytes and
catalog/index. Cheap metadata resolves one ID per package. Host's new private
sidecar API needs no timer caller or direct dependency; the installed Testkit
CLI still uses its own packaged lock. [The graph owner](../releasing.md#shared-061-correction-and-host-0134-selection)
records applicability and source identities. Released-source CI does not qualify
this graph or corrected tooling. No measured Wasm/instruction delta is claimed.

Only source/evidence inspection, syntax/diff and cheap metadata checks may run.
Tests/builds/lint/setup/deployment and all staging/commit/release effects remain
user-owned. The current changes are contributor tooling and dependency selection;
no timer-runtime feature or public hard cut is justified by the open issues.
