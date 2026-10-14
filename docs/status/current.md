![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-14

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Draft tooling ownership PR

The maintainer requested PR delivery. Shared
[PR #180](https://github.com/dragginzgame/shared-tooling/pull/180) provides committed
canonical `5732ded141e057b27eb96400b0f7bc2820d42d79` (based on Shared 0.12.5, pending 0.13.0).
The linked Timers draft adopts that exact commit through three canonical exports
(**77/40/18** files), with complete previews before apply. It delegates fixture
probes, safe evidence orchestration and literal metadata transactions, removing
the local setter/lock updater. Product assertions and release policy remain.

This isolated PR includes the accepted repository-only 0.22.3 ownership batch.
The normal checkouts remain untouched. Cargo inputs and tool catalogs match
Timers 0.22.2; incoming Host 0.17.11 lock changes and AGENTS shortening are excluded.
The current numbered draft is undated and Cargo version remains 0.22.2. Upstream
integration and full native hosted qualification remain outstanding; no release,
PocketIC execution, build or setup is implied by the focused shell evidence.

Focused Shared retention, rollback, collector and existing host/IC action callers
pass. Timers retention, evidence, preparation and release-gate fixtures pass;
ShellCheck passes. Exact-export module-copy and snapshot qualification are recorded
with [adoption](../shared-tooling.md#shared-ownership-pr-adoption). Before-images,
previews, isolated source and logs remain at
`/tmp/ic-timers-owner-reductions.PaAdlrMw/`. The draft must be refreshed to the
integrated upstream commit before delivery if its identity changes.

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
