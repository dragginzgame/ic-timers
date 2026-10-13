![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-13

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

The maintainer reports **0.21.4 live** at
`44935d6188a0da82314e00a56f5f68a9d0c59638`. The completed standard receipt binds
owner validation to `903a548263ea2bfdd1eca395c63703dac31fee2c` and tree
`cc2bec1f20ba29516dd7d270005b5dd521007799`. Source-to-release differences are
only workspace Cargo/lock versions and changelog finalization. The
[delivery owner](../releasing.md#0214-delivery-and-prepared-0215-tooling)
records complete owner-operated native/IC/cohort acceptance without fabricating
successful raw logs or agent replay. Earlier focused proofs retain their original
source/graph and are not reused as execution of later dependency selections.

Delivered graph: Metrics **0.8.3**, Testkit **0.36.0**, Host artifacts/fs/process
**0.16.0**, PocketIC **16.1.0**, Shared **0.9.13** across **68/36/11** files.
Callbacks now live directly in their shared owner without a nested Box; returned
ordinary futures stay boxed/pinned. Lifecycle slots are selected in one branch,
with cleanup, publication and unused-factory drop ordering preserved. Public
APIs and timer semantics are unchanged. No comparable footprint/instruction
saving is measured. Retain protected ordinary/watchdog, detached-handle and
confirmed-delivery boundaries rather than forcing another consolidation.

[Initial cleanup evidence](../design/callback-delivery-ownership.md#prepared-0214-callback-ownership-simplification)
remains at `/tmp/ic-timers-complexity0214.CsNLubA0/`: prepared inputs and compiler/
test logs. All 154 native subjects and library Clippy passed under scoped approval,
with compile-only native/test targets on Rust 1.99/1.88 and Wasm on 1.88.
That approval is complete and does not authorize new independent fixtures.

Hosted [run 38375894239](https://github.com/dragginzgame/ic-timers/actions/runs/38375894239)
is in progress; [run 38375894348](https://github.com/dragginzgame/ic-timers/actions/runs/38375894348)
is queued. No complete new hosted/macOS acceptance is claimed.

## Prepared 0.22 cleanup and upstream adoption

The private [callback measurement simplification](../design/callback-delivery-ownership.md#prepared-022-callback-measurement-simplification)
records directly from the captured start, preserving instruction/memory sampling
order and the registry's stale-claim rejection. Existing identity-reuse fixtures
now use that production recording path, retaining normal accounting assertions.
The private cleanup preserves public APIs and timer semantics; the profile selects thin
LTO. Earlier frozen experiments showed byte-identical Wasm for this source change.
Earlier native/test-target compile-only checks pass on Rust 1.99 and 1.88;
Wasm library compilation passes on 1.88, with locked/offline inputs and selected
Metrics 0.8.4. Logs and before-images stay at
`/tmp/ic-timers-measurement0215.dq7zQXfN/`. Production source matches the earlier
measured private prototype. Tests, Clippy and PocketIC have not been rerun;
complete owner validation remains pending.

Incoming Cargo inputs now select registry Metrics **0.9.0**, Testkit **0.37.0**,
and Host artifacts/fs/process **0.17.1**, with PocketIC **16.1.0** unchanged.
Their prepared official archives match lock checksums and producer VCS receipts.
The [Metrics adoption](../design/callback-delivery-ownership.md#prepared-metrics-09-adoption)
updates overflow assertions and public summary identity; consumers must coordinate
Metrics 0.9. This public cut supersedes the 0.21.5 draft with one undated
**0.22.0** section. Package versions and the incoming root lock remain untouched.

All three canonical snapshots select reviewed Shared **0.10.0**
`ed9c26f015691859f67b50b512a94709dcdc0c6c`, preserving roots and deriving
**70/37/11** payloads. Complete clean-source previews precede apply. The shared
catalogs are canonical; `ci/ic-tools-qualified.tsv` keeps the prior byte-exact
Binaryen 132 selection, explicitly chosen by Make and local failure collection.
No optimizer/provider/PocketIC policy is silently replaced. Maintenance/LOC
path rejection, release-lock diagnostics and earlier CI recovery are included.

[Adoption evidence](../shared-tooling.md#prepared-shared-tooling-0100) stays at
`/tmp/ic-timers-upstream022.Kjn981dG/`: clean source, previews/applies, saved
inputs/index/refs, integrity logs, selected registry receipts and full locked
offline metadata. Remote API requests were unavailable/refused; no new hosted
acceptance is claimed. Native/test-target compile-only checks for Metrics 0.9
pass on Rust 1.99 and 1.88; Wasm library compilation passes on 1.88. All use
locked/offline inputs and `RUSTUP_AUTO_INSTALL=0` with the prepared tools.
Run owner-operated `make install-tools` and `make tools-check` before formatting
and complete native/IC/release qualification. No tests, lint, PocketIC, executable
setup or release execution has been performed for this adoption.

## Issues and authority

The 2026-10-13 isolated Wasm review found full LTO saves 18.67–18.85% across
four released 0.21.4 linked probes; two private source simplifications produced
byte-identical artifacts. The
[owning evidence](../design/callback-delivery-ownership.md#whole-canister-wasm-experiments)
records frozen inputs and limits. Reusable recipe guidance is prepared in
IC Build; larger-app final transforms and instruction qualification remain
necessary before central adoption. Local profiles remain unchanged; the private
measurement simplification is now applied as described above.

The current issue inspection finds no open Timers issues. No additional runtime
cleanup justifies weakening the protected state paths.
[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
remains closed from delivered Metrics adoption at 0.21.0.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) retains its frozen
v0.19.1 acceptance and original/restored evidence. Shared fleet
[#98](https://github.com/dragginzgame/shared-tooling/issues/98) retains other
consumer obligations. No qualification dispatch is pending.

Siblings remain read-only. Tests, builds, lint, PocketIC and setup remain
user-owned outside explicit scoped approvals; prepared locked/offline library
compile-only checks retain their AGENTS.md exception. Cargo versions, staging,
commits, tags, pushes, publication and cleanup remain user-owned. Preserve
incoming edits and retained evidence.
