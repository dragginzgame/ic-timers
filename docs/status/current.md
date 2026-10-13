![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-13

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.21.2 is committed/tagged/pushed** at
`1065512b6d6d231aa8cad25e3a668b1a65c99b51`. The local annotated tag resolves to
that commit; GitHub push runs match it. The completed standard direct receipt
binds validation to source `d3d9ecad898e42bf0c60c3ccce1f876e1362bbae` and tree
`1a15c83ee63313da0a64ffc130ff4c8c8ca9f105`. Source-to-release changes are only
workspace versions and changelog finalization. The
[delivery owner](../releasing.md#0212-delivery-and-prepared-0213-tooling)
records owner-operated acceptance without fabricating successful raw logs or
agent replay, separately from earlier failures and later tooling changes.

Delivered selections: Metrics **0.8.1**, Testkit **0.35.1**, Host artifacts/fs/process
**0.15.2**, PocketIC **16.1.0**, Shared **0.9.10** across **68/36/11** files.
The canonical Rust-tool fixture now owns compiler isolation and commit-free
history reuse; the temporary Make adapter is removed. Original failed fixture
and 0.21.1 adapter evidence remain at their existing owners.

The 0.21 lifecycle semantics retain atomic fresh registration publication,
cleanup before retry and existing callback ownership on later failures.
The [lifecycle](../design/0.20-typed-registration.md#prepared-021-lifecycle-creation)
and [Metrics](../design/callback-delivery-ownership.md#prepared-ic-metrics-08-adoption)
owners preserve the earlier evidence. The recent bounded cleanup inspection
found no justified additional runtime deletion; do not manufacture a hard cut.

Exact-source hosted [run 38363085866](https://github.com/dragginzgame/ic-timers/actions/runs/38363085866)
passes; [run 38363085915](https://github.com/dragginzgame/ic-timers/actions/runs/38363085915)
is in progress. No complete new hosted/macOS acceptance is claimed.

## Prepared 0.21.3 maintenance

All three canonical snapshots select reviewed Shared **0.9.13**
`35cad65677ff989b5fb24a88d947fda37d2366e8`, retaining roots and **68/36/11**
payloads. Complete clean-source previews precede apply, without additions or
retirements. Dirty upstream 0.9.14 log-recovery work is excluded.

Store/checkout locks preserve filesystem diagnostics/status without clearing
another owner's lock. Cargo acquisition preserves its original source failure
before build/activation. CI inspection rejects failed/empty branch discovery;
explicit branch/commit/run selection remains supported. Locked source-preparation
guidance retains local explicit offline and user-owned validation exceptions.
No automatic fetch or additional runtime/CI cache path is introduced.

**Run `make install-tools` before validation:** the changed Cargo build-helper
identity requires explicit new selections. Earlier generations remain retained;
checks do not install or fall back. Setup remains user-owned.

[Adoption evidence](../shared-tooling.md#prepared-shared-tooling-0913) stays at
`/tmp/ic-timers-shared0913.8v63usJT/`: clean source, saved inputs/index, all
previews/applies, integrity logs and delivered receipt. Integrity, changed Bash
syntax, documentation links and diff checks pass. No tests, lint, setup or
release execution. Cargo package versions remain 0.21.2. Preserve incoming
Metrics **0.8.2** / deranged **0.5.9** lock changes; this new graph is not qualified
by the delivered receipt. Contributor-only notes use undated **0.21.3**, normally
bundled into the next code-bearing release. Product Rust source is unchanged.
Upstream [run 38368099898](https://github.com/dragginzgame/shared-tooling/actions/runs/38368099898)
is queued at inspection.

## Issues and authority

The latest bounded issue inspection found no open Timers issues.
[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
remains closed from delivered Metrics adoption at 0.21.0.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains closed from
frozen sequential v0.19.1 acceptance; original failures, artifact identities and
restored/missing evidence stay with the release owner. Shared fleet
[#98](https://github.com/dragginzgame/shared-tooling/issues/98) retains other
consumer obligations. No qualification dispatch is pending.

Siblings remain read-only. Tests, builds, lint, PocketIC and setup remain
user-owned outside explicit scoped approvals; prepared locked/offline library
compile-only checks retain their AGENTS.md exception. Cargo versions, staging,
commits, tags, pushes, publication and cleanup remain user-owned. Preserve
incoming edits and retained evidence.
