![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-14

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered 0.22.5 and prepared 0.22.6

Current HEAD is `6ced4341bacce7d9f08acfd9a261a6d222755b93` (released 0.22.5).
The incoming root lock selects Metrics 0.9.6 and Testkit 0.42.0; its bytes and the
root manifest remain unchanged. The undated 0.22.6 batch shares immutable identity
storage, narrows cohort cancellation to exact-claim wake-up observation, and
consolidates the trap/exhaustion probe windows and pre-failure schedule proposal.
Public timer APIs, identity value semantics and callback authority are unchanged.

The [callback owner](../design/callback-delivery-ownership.md#prepared-0226-identity-sharing-and-probe-cleanup)
records comparable raw Wasm, PocketIC instructions and separate native allocation
measurements, including the larger inactive baseline and initial allocation cost.
The 155 native tests, strict affected library/probe Clippy, Rust 1.88 native/Wasm
compilation, policy cohorts and 16 real PocketIC runtime subjects pass. The server
was already prepared and admitted by the current root-lock Testkit adapter.
Evidence stays at `/tmp/ic-timers-identity-reduction.y7d3dr0q/`; a sandbox socket
failure is retained separately from the successful local-binding run. Full owner
release/macOS qualification remains separate. No setup, Cargo metadata mutation,
staging, commits, tags, pushes or release commands ran.

## Prior delivered 0.22.4 and prepared 0.22.5

Current HEAD is `09dc8c1` (released 0.22.4). The maintainer's incoming Cargo
selection uses Testkit 0.41 and Metrics 0.9.5; this task preserves those inputs.
The undated repository-only 0.22.5 batch centralizes five readiness waits through
Testkit's bounded poller, preserving the first tick and each existing round budget.
Fixed-round drain observations remain explicit. Four cohort builds share one
fail-fast recipe, retaining exclusive features and isolated target directories.

The release-prose advisory, complete release-change collector and semantic suites belong to
[Shared PR #191](https://github.com/dragginzgame/shared-tooling/pull/191), pinned
through all three reviewed snapshots at `c576f79dcd6bb693ba704410884e81c5e3a226d3`
(82/41/18 payloads). Timers retains package path policy and qualified caller
adapters; failed partial streams never become impact classifications. Canonical
exports were generated in an isolated current-tree copy because earlier changes
are staged. Before-images were verified before copyback; index bytes and Cargo
inputs remain intact. The source includes committed Shared 0.13.1, with pending
0.13.2 helper notes. It excludes #180's release-preparation hard cut.

Testkit [#96](https://github.com/dragginzgame/ic-testkit/pull/96), at
`bc02adb2b73c5decad754cd8c30272796b4015b5`, skips time requests in zero-duration
polling. A bounded native peer proves the actual client sends only a tick for
zero and read/set/tick with the exact timestamp for nonzero. The regression fails
with the old behavior; all five polling tests, strict Clippy and Rust 1.88 test-target
compilation pass. This is native transport evidence, not live canister qualification.
The real graph still selects Testkit 0.41.0; adoption waits for publication.

Both removal-payload experiments were rejected. With Rust 1.88/thin LTO and the
same current graph, allocation-free callback/slot ownership adds 1,976–2,019 bytes
to the three active policy cohorts; native transition size grows from 168 to 232
bytes. A narrow boxed payload still adds 1,156 bytes for Once and 1,211 for Watchdog.
The allocation-free candidates passed the 154 native tests, but neither approach
justifies the extra type/code and Wasm cost for short-lived cleanup storage.
The original registry source is restored
byte-for-byte and its native suite passes. No library API or runtime change remains.
Before/after artifacts, hashes, layouts and isolated source are retained at
`/tmp/ic-timers-owner-cleanup.Qa4P1qIr/`; no IC instruction saving was measured.

Rust 1.88 probe/test-target compilation, Rust 1.99 probe Clippy, the actual cohort
recipe/failure fixture and Shared advisory/ShellCheck checks pass. Live PocketIC
qualification cannot start because the incoming Testkit 0.41 CLI has not been
prepared; `make install-testkit-server` remains explicit owner setup. No setup,
live canister execution, real release or main-branch commit ran. Snapshot integrity/corruption, prose adapter/owner semantics, version-preparation,
fixture retention/completion, changed ShellCheck and documentation links also pass.
The focused readback is retained at
`/tmp/ic-timers-three-reductions.YIlCM7hu/`. Production timer code is unchanged.

## Prior 0.22.3 and 0.22.4 preparation

Current HEAD is `730ad01` (released 0.22.3); the previous preparation and
qualification below retain their historical source scope. Shared snapshots now
select reviewed 0.12.8 `eafa07cac1fdba7d6303d061ed8af458aacf32b7` across
78/41/18 payloads, and the unchanged root catalog selects Testkit 0.40.

The compatible repository-only 0.22.4 draft reuses Testkit's existing
`InstallSpec`/`create_and_install` at all 17 fresh probe-install sites. The shared
harness adapter retains the empty init payload, two-trillion-cycle top-up and
anonymous install sender. Every fixture still owns a fresh instance/server;
recovery top-ups, upgrade operations and measurement boundaries remain explicit.
The local graph fixture now qualifies helper binding, `check-graph ic-timers`,
ambient `RELEASE_COMMIT` clearing, selected caller worktree, exact failure status
and one live full-workspace check. Generic negative graph fixtures belong to Shared.

Owner changes and the deferred Timers caller patch live at
`/tmp/ic-timers-next-three.0lpn1xkr/`. Testkit [#89](https://github.com/dragginzgame/ic-testkit/pull/89)
is open at `e175c60`, refreshed on released Testkit 0.41.0; Shared [#180](https://github.com/dragginzgame/shared-tooling/pull/180)
is refreshed at `fdacbd8` and ready for review. Dirty sibling inputs are excluded. Testkit adds two generic
multi-result Candid functions using its existing transport/error machinery;
Shared gains empty/plausible producer-failure, stale local-row and coherently
wrong-primary graph cases. Both have pending changelog notes. The Candid caller
patch removes the two local raw-decoding helpers and switches four multi-result
call sites without changing probe wire results; it must wait for owner publication.
The current working tree retains released Testkit APIs and its existing Cargo
inputs. The proposed API is qualified only through an isolated offline override;
its caller patch and prepared minimum Testkit 0.41.1 requirement wait for owner
publication. The normal checkout remains on 0.40. Shared main is adopted separately
from the dependent PR export.

The maintainer adopted ordinary focused checks under Shared's cadence, replacing
AGENTS' compile-only permission rule. Rust 1.88 probe and proposed Testkit
compilation, Rust 1.99 strict Clippy, five Candid unit tests (also requalified on
the new Testkit 0.41 anchor), relevant local/owner
shell fixtures and ShellCheck pass. All three snapshot integrity checks and
actual prepared host/IC offline checks pass. No full local suite, setup, live
PocketIC execution, version mutation or release ran. Isolated PR commits/pushes
were expressly authorized; normal sibling checkouts, real index and refs remain
untouched. No Wasm or instruction saving is claimed.

Outstanding PRs are refreshed on released main. Timers
[#56](https://github.com/dragginzgame/ic-timers/pull/56) now qualifies depth-one Git
history without `HEAD^`; its tag fixture and ShellCheck pass. Both old macOS
failures selected the obsolete Testkit/Host graph. Superseded Testkit 0.39 adoption
[#57](https://github.com/dragginzgame/ic-timers/pull/57) is closed because released
main already selects 0.40. Timers [#54](https://github.com/dragginzgame/ic-timers/pull/54)
is updated at `35b2fa2`, with canonical 79/41/18 exports from Shared #180,
current Cargo inputs, six affected shell fixtures/snapshot checks and lint passing.
The latest head is `d8543ee`; it remains draft pending owner integration and
exact-source refresh if needed.
The newer hosted #56 run exposed incomplete dependency-cache preparation before
an offline release-index fixture. The normal checks lane now runs locked `make fetch`
before `make ci`; the actual-step fixture proves order and failed-fetch short circuit.
The repaired #56 head is `f7d6d82`, and #54 carries the same CI prerequisite.
The fix is also prepared in this normal worktree. macOS release verification
already prepares its graph. Hosted checks for these newest heads remain separate
and in progress/queued; no full hosted pass is claimed.

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

Normal sibling working checkouts remain untouched. Focused checks follow Shared's
adopted cadence; broad local gates, setup and release execution retain their
explicit authority. PR delivery was authorized for the isolated contributions.
Ordinary real-worktree staging, commits, versions, tags, publication and cleanup
remain maintainer-owned. Preserve retained artifacts and failure evidence.
