![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
Earlier history remains at its evidence owners and
[the historical archive](archive-2026-10-10.md).

## Released source and qualification

Released **0.17.11** is `b89023f4e3696d65c9b15e4749cc19fa747e57ec`.
Its graph selects Metrics **0.6.4**, Testkit **0.33.7**, all four Host **0.13.4**,
PocketIC **16.1.0** and Shared **0.6.1**
`731909525038c58d6f80b498e80f8630b11c685d` through **56/33/11** exports.
[Full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38143223594)
has Linux/MSRV running and Intel/ARM queued. The separate
[tag job](https://github.com/dragginzgame/ic-timers/actions/runs/38143223595)
is running; tag truth does not establish full native acceptance.
Keep [#44](https://github.com/dragginzgame/ic-timers/issues/44) open for actual
exact-source acceptance and its controlled early/late artifact boundary.

Released 0.17.10 passes Linux/MSRV/tag checks but both native jobs fail at the
shared Cargo fixture's padded-count boundary. The downloaded Intel failure ZIP
matches exact source/host identity and retains three builds; ARM's downloaded log
matches the boundary. [Its owner](../releasing.md#01710-release-qualification)
retains source, artifact digest and original failures. Ordinary artifact retention
is not controlled qualification. Earlier 0.17.8 has full native acceptance;
0.17.9 main was cancelled. Neither graph qualifies current changes. Native reset
is not IC upgrade/rollback proof; retain product PocketIC gates.

## Current prepared work

Pending **0.17.12** adopts committed Shared **0.6.2**
`7f1ca16b0dfa98021d2671017ecb7de767409b0e` through the same canonical exports.
[The adoption owner](../shared-tooling.md#shared-tooling-062-release-diagnostics-adoption)
records detached source, previews, apply logs and preserved inputs at
`/tmp/ic-timers-shared062.7tp4y5tp/`. Source-conflict diagnostics and guarded
maintainer recovery address [Shared #126](https://github.com/dragginzgame/shared-tooling/issues/126).
[The local preparation boundary](../releasing.md#changed-source-at-preparation-intent)
explains why temporary backups and an unchanged version cannot prove no effects.
No real plan retirement or release recovery is performed.

The native-only correction under [#45](https://github.com/dragginzgame/ic-timers/issues/45)
drops cancelled/discarded futures after releasing the task-map borrow, matching
production provider removal. One regression covers queued/suspended cancellation
and queued discard: a capture destructor inspects the queue and schedules a
replacement. It is prepared, not executed. Production behavior and APIs are unchanged.

Incoming maintainer lock changes select all four Host **0.13.5**, with Metrics
0.6.4 / Testkit 0.33.7 / PocketIC 16.1.0 unchanged. Host's product Rust source is
unchanged from 0.13.4. Preserve exact incoming lock/catalog, consumer pins and
raw index. Cheap locked offline metadata resolves one ID per selected package.
The installed Testkit CLI uses its packaged lock; Host stays outside the timer
library's normal dependency closure.

All snapshot integrity checks, changed shell/stub syntax, local links/diff and
cheap metadata pass. Upstream 0.6.2 lint passes; portable/native jobs are pending.
Shared 0.6.1 Linux/lint passes but native jobs were cancelled by the newer run.
Shared setup/build policy is unchanged since 0.6.1; no forced reinstall is added.
Keep `.tools/rust/common/bin` and the same `SHARED_TOOLING_STORE` for setup,
checks and evidence. Testkit retains server/PocketIC admission, the collector
retains safe local evidence on selector failure, and consumer tool pins stay intact.

Only inspection, syntax/diff and cheap metadata checks may run. Tests/builds/lint,
setup/deployment and staging/commit/release remain user-owned. No measured Wasm,
instruction or IC recovery improvement is claimed from this native-only repair.
