![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety](../../SAFETY.md).
Earlier history stays at its evidence owners and
[the archive](archive-2026-10-10.md).

## Released source

Released **0.17.12** is `549d1f5e9e69ec2d58d37c72760ffd93b88b175f`.
The graph selects Metrics **0.6.4**, Testkit **0.33.7**, all four Host **0.13.5**,
PocketIC **16.1.0** and Shared **0.6.2**
`7f1ca16b0dfa98021d2671017ecb7de767409b0e` through **56/33/11** exports.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38144421659)
passes; [full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38144421665)
has Linux/MSRV/ARM passing and Intel running. The
[qualification owner](../releasing.md#01712-source-and-native-cleanup-follow-up)
records source/graph identities and downloaded Linux log digest. The removal
regression from #45 actually executes and passes among 145 library tests, with
14 PocketIC recovery subjects and four policy cohorts also passing. Keep #44 open
for complete native/controlled artifact evidence.
0.17.11 Linux/MSRV passes but native jobs were cancelled. 0.17.10 Linux passes
and both native jobs fail at the padded-count boundary. Earlier 0.17.8 has full
native acceptance; those sources do not qualify new changes.

## Prepared work

Pending **0.17.13** adopts Shared **0.6.3**
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d` through **56/33/11** canonical exports.
[The adoption owner](../shared-tooling.md#shared-tooling-063-committed-preparation-restart)
records review and preservation at `/tmp/ic-timers-shared063.dfz0hesa/`.
Normal direct targets can archive stale prepare intent from a clean descendant
commit, recompute the requested increment and repeat full validation. Exact
resume and staged/later recovery keep their boundaries. The local helper mutates
only release metadata; no per-version receipts are replaced. No real release
state is changed here. Shared 0.6.3 Linux/lint passes; both native jobs are queued.

Pending **0.17.13** fixes native fixture cleanup under
[#46](https://github.com/dragginzgame/ic-timers/issues/46). Clearing queued futures
alone left retained/inactive callback factories in RUNTIME until TLS teardown.
Cleanup now detaches remaining registry ownership, exposes an empty initialized
native registry and drops captures outside the borrow while TLS remains live.
The existing regression covers both lifetimes and inactive/queued/suspended work,
checking capture release, queue cleanup and live runtime access. It is prepared,
not executed. This test-only retirement is not IC lifecycle or rollback evidence.
Production behavior and APIs are unchanged; no measured Wasm/instruction claim.

Incoming maintainer lock changes select **Testkit 0.33.8**, with Host 0.13.5,
Metrics 0.6.4 and PocketIC 16.1.0 unchanged. The selected Testkit package's shipped
CLI lock now also selects Host 0.13.5; product Rust source is unchanged from 0.33.7.
Keep Testkit's returned CLI path and server admission; add no direct Host import.
Published Host **0.14.0** has incompatible process completion/observer APIs.
Testkit still requires Host 0.13; its
[owner migration](https://github.com/dragginzgame/ic-testkit/issues/61) and minor
release are prerequisites to adopting Host 0.14 here. No parallel Host graph is
introduced.
Cheap locked offline metadata resolves one ID per package. Preserve incoming
catalog/lock, consumer tool pins and index byte-for-byte. Preparation evidence is
at `/tmp/ic-timers-01713.mgzz1bha/`; links/diff and metadata pass.

Shared setup/build policy and common paths are unchanged. Keep
`.tools/rust/common/bin` and one `SHARED_TOOLING_STORE` for setup/checks/evidence.
Testkit retains PocketIC ownership; the collector retains safe local evidence on
selector failure. Consumer Binaryen 132 and other tool pins remain intact.

Only inspection, syntax/diff and cheap metadata checks may run. Tests/builds/lint,
setup/deployment and staging/commit/release remain user-owned. Retain the complete
PocketIC and native gates; neither source review nor native reset proves IC recovery.
