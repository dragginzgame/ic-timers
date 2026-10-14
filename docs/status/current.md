![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-14

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered 0.22.2 and prepared 0.22.3

Current HEAD remains `26f1cdfc399fd4a321b73f9484a2234cae8a43de` (0.22.2).
The compatible repository-only [0.22.3 batch](../releasing.md#prepared-0223-shared-owner-cleanup)
now adopts committed Shared 0.12.3
`ac68c74439e13f19229a35f424f968c64325cc44` through all three canonical exports
(**75/40/17** payloads). The [adoption owner](../shared-tooling.md#prepared-shared-tooling-0123)
records reviewed policy, preserved roots and exact qualification scope. Dirty
Shared 0.12.4 work is excluded; local validation/commit/release exceptions remain.

The accepted reductions reuse canonical rendering, fixture cleanup, coordinated
graph/member projection, exact version ordering and generic evidence mechanics.
Local rollback, source/candidate checks, selected-tree sorting, identity/scenario
assertions, qualified catalogs, Testkit and native/product gates remain explicit.
All 15 previously approved shell fixtures and the Python downloaded-evidence
suite pass at the earlier recorded inputs. Three current snapshot checks and the
locked/offline root graph pass; the repeated snapshot fixture passes after refresh.
No native macOS/PocketIC or complete release-gate result is inferred.

This inspection began with the existing batch already dirty and Host
artifacts/fs/process 0.17.11 in the incoming lockfile. Their official archives and
producer commit are verified; Testkit 0.38.0 remains the direct dependency owner.
No additional Host API or direct dependency is needed. Host 0.17.10 CI passed;
exact 0.17.11 and Shared 0.12.3 CI are still running at inspection. The old frozen
0.12.2 inventory-fixture failure retains its historical scope; the fix now exists
in 0.12.3 and is not a newly passing full-suite receipt. Issues #51/#52 retain
latest-delivered hosted acceptance.

Evidence stays at `/tmp/ic-timers-upstream0123.3juyp1lg/`; earlier cleanup and
qualification are at `/tmp/ic-timers-three-owners.ylz2aykc/` and
`/tmp/ic-timers-shared0120.UujtaKfH/`. Incoming Cargo inputs, tool selections,
real index and refs are preserved. No version bump or real release execution ran.
The [2026-10-13 audit](../audits/code-hygiene-2026-10-13-01.md) keeps its original
source/method scope; no new runtime or Wasm claim follows from this inspection.

## Prepared tag regression separation

PR #56 owns issue #55. Its small tag-job change now preserves explicit admission
of all three actual snapshots while removing the duplicate release-check roster.
The existing tag fixture now exercises the actual workflow with disposable refs
and substitute tool/Make calls;
it checks valid tags, stale event names, unreachable/lightweight tags, snapshot
corruption, failed version reads and the permitted dispatches. It remains in the
normal full gate. The larger overlapping draft #58 is closed; its branch/evidence
are retained. Tool setup, main/native validation and failure retention stay intact.
The actual-workflow and existing tag/publication fixture pass, as does focused
ShellCheck. No normal checkout, tag or release is changed.

## Prior 0.22.0 delivery evidence

The maintainer reports 0.22.0 live at
`ffe175b2ae1b38719ec649286468b96a08a2f111`. The completed standard receipt binds
owner release validation to `9ad2c39b1f6bf89a12098612111aebb66e23aa72` and staged
tree `5f00837b17b2a0bda924d33eb8ce9c4a49f09c61`. Source-to-release differences
are only Cargo/local-lock versions and changelog finalization. The
[delivery owner](../releasing.md#0220-delivery-and-prepared-0221-tooling) records
the exact graph and separates receipt completion from raw/hosted acceptance.

Delivered graph: Metrics 0.9.0, Testkit 0.37.0, Host artifacts/fs/process 0.17.2,
PocketIC 16.1.0 and Shared 0.10.0 across 70/37/11 files. Host's three registry
archive checksums and VCS receipts match the selected graph; runtime source is
unchanged from 0.17.1. No new Host selection is needed. Dirty Host 0.17.3 is excluded.

Metrics overflow assertions and public summary identity now use 0.9. The private
callback measurement path records directly from its captured start, preserving
sampling order and stale-claim rejection. The earlier frozen four-cohort source
experiment yielded byte-identical Wasm for that cleanup; it does not measure
this dependency graph. Full LTO remains a larger-application qualification topic
with IC Build; the local profile still selects thin LTO. The
[callback owner](../design/callback-delivery-ownership.md) retains preparation
receipts and their original compiler/graph scopes.

## 0.22.1 preparation evidence

The [prepared contributor cleanup](../releasing.md#prepared-0221-contributor-cleanup)
removes the separate exact-version delivery and manual Git phase commands.
Shared standard release/resume commands now own delivery; publication admission
and metadata rollback retain focused fixtures. Seven probe wire records live
once in the unpublished `ic-timers-test-protocol` package, shared by the three
probe packages. Empty/populated observations reuse one application-metadata
construction. The root workspace has five members; only local lock rows and
links change, with all existing package versions and registry selections retained.
Timer library source, public API and dependency graph are unchanged by this
contributor cleanup. Its new Candid interop test and changed fixture/build
qualification remain distinct from earlier harness evidence.
Approved focused checks pass: Rust 1.88 native probe/test-target compilation,
the protocol test and runtime/four-cohort Wasm compile-only checks; Rust 1.99
probe/protocol Clippy; version-preparation, tag/publication, fixture-completion
and release-gate fixtures. The gate fixture's stale two-test expectation is
updated to require the new protocol test and propagate its failure. Before-images
and logs remain at `/tmp/ic-timers-cleanup0221.zH22uPmx/`; the original failed
fixture remains at `/tmp/tmp.slBA01ZRdq`. No PocketIC execution, optimized Wasm
artifact build, setup or real release ran; real index/refs and external versions
are preserved. Complete owner/native macOS qualification remains outstanding.

The unpublished PocketIC harness also delegates combined server/instance startup
and lifetime ordering to selected Testkit 0.37.0. Every fixture retains the
upstream owner and borrows its instance. One 60-second startup budget replaces
two 30-second phase budgets; admitted binary selection and canister assertions
remain unchanged. The [harness owner](../releasing.md#prepared-0221-harness-ownership)
records the qualification scope and the decision to retain the simple Make probe
builds: IC Build's package/target-name admission blocker is reported in
[#16](https://github.com/dragginzgame/ic-build/issues/16), and a new adapter and
dependency have no established reduction or measured saving. The literal
workspace-version renderer proposal is on
[Shared #159](https://github.com/dragginzgame/shared-tooling/issues/159#issuecomment-6147567013).
No sibling source, snapshot, IC Build dependency, cache or build policy changes
are introduced by this cleanup.
Explicitly approved Rust 1.88 probe compilation and Clippy pass locked/offline
with `RUSTUP_AUTO_INSTALL=0`. Before-images and logs remain at
`/tmp/ic-timers-harness0221.y2PU7FAH/`; no tests, PocketIC, Wasm builds, setup or
release commands ran. Cargo inputs and real index/refs are preserved.

The undated 0.22.1 batch also includes a small
[relative-deadline cleanup](../design/callback-delivery-ownership.md#prepared-0221-relative-deadline-cleanup):
callback retries reuse initial scheduling's checked duration/deadline path,
retaining Retry mode and terminal error mapping. The existing boundary fixture
covers zero and maximal encoded delays as well as its prior failures. Public
APIs and timer semantics are unchanged. Native/test-target compilation passes on
Rust 1.99 and 1.88; Wasm library compilation passes on 1.88, all locked/offline
with `RUSTUP_AUTO_INSTALL=0`. Before-images and logs stay at
`/tmp/ic-timers-relative-cleanup0221.csCl1Y1d/`. Tests, Clippy and PocketIC have
not run for this cleanup, and no Wasm/instruction saving is measured.

All three canonical snapshots select clean committed
`626fc2bd3125195520fb870b92abfec11b01454d`, preserving roots and 70/37/11 files.
Complete previews precede apply. Literal Cargo cache keys ignore unrelated pins,
comments and record order, retaining compiler/build identity and conservative
fallback for ambiguous catalogs. Catalog/exception review is explicit. Common
Cargo-tool CI cache transport is not activated, and no job-time saving is claimed.

The shared host/Cargo and IC catalogs are unchanged; Make and local failure
collection retain `ci/ic-tools-qualified.tsv` for qualified Binaryen 132.
Installer/build-policy identities are unchanged, so no reinstall is needed solely
for this refresh. Dirty Shared 0.10.3 is excluded. The
[adoption evidence](../shared-tooling.md#prepared-shared-tooling-0102) stays at
`/tmp/ic-timers-shared0102.XdtpHHvq/`, with clean source, saved inputs/index/refs,
previews/applies and the 0.22.0 delivery receipt. That snapshot refresh ran no
tests, builds, lint, setup, PocketIC or release execution.
Snapshot integrity, changed Bash syntax, local documentation links, diff checks
and full locked offline metadata pass; Cargo inputs and real index/refs are preserved.
The undated 0.22.1 notes now include the private code-bearing cleanup above.

## Issues and hosted acceptance

[Metrics #51](https://github.com/dragginzgame/ic-timers/issues/51) and
[catalog #52](https://github.com/dragginzgame/ic-timers/issues/52) have delivered
source adoption in 0.22.0. Their remaining hosted native acceptance is open.
Both issues now carry delivered-source evidence and the outstanding gate.
[Tag run 38399245736](https://github.com/dragginzgame/ic-timers/actions/runs/38399245736)
passes tag truth; full native jobs are skipped on the tag. The same-SHA
[main run 38399245629](https://github.com/dragginzgame/ic-timers/actions/runs/38399245629)
passes Linux checks/MSRV and Apple Silicon; Intel is queued at final
readback. Do not close host qualification from tag success or the
older 0.21.4 hosted results. No workflow dispatch or rerun is pending.

Siblings remain read-only. Tests, builds, lint, PocketIC and setup remain
user-owned outside explicit scoped approval; prepared locked/offline library
compile-only checks retain the AGENTS.md exception after Rust changes. Versions,
staging, commits, tags, pushes, publication and cleanup remain user-owned.
Preserve incoming edits, retained artifacts and failure evidence.
