![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Released source

The maintainer reports **0.19.0 live**. HEAD and annotated `v0.19.0` resolve to
`81b82589ca08aeb966edaf77a09f3f611bcce079`; package metadata and finalized notes
agree. [Tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/38221325648)
passes. [Full main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38221325703)
passes Linux 114718018090, MSRV 114718018369 and ARM 114718018350;
Intel 114718018302 is running at inspection. Older native proofs keep their scope.

The release delivers Metrics 0.7 wide totals/means, Shared 0.6.5, earlier compiler/
Clippy admission, approved compile-only development permission and the minimum-
compiler Wasm check. The released graph uses Testkit 0.34.1, Host 0.14.3 and
PocketIC 16.1.0. Samples/counts/latest/maximum and timer policies remain unchanged.
[The Metrics owner](../design/callback-delivery-ownership.md#prepared-ic-metrics-07-adoption)
records report/codec propagation and volatile-state scope. No stable-map adapter,
compatibility reader or new platform reader is added.

## Prepared 0.19.1

Committed Shared **0.8.0 `045a6f22a9de497bfb3af90cb2571957384806f2`** is canonically
adopted through **59/35/11** exports. Committed VERSION and canonical annotations
agree. Recorded entrypoints and derived payloads remain intact; no files are
added or retired. Future refreshes automatically retire only verified unchanged
owned files outside the reviewed root closure; the old `--retire-file` option
is removed. The [adoption owner](../shared-tooling.md) records all three complete
clean-source previews/apply logs and preserved inputs under
`/tmp/ic-timers-shared080.IMvYFaPt/`. The preceding entrypoint conversion remains
at `/tmp/ic-timers-shared070.n53MkYKh/`.
The earlier pin adoption remains at `/tmp/ic-timers-shared066.jqtm0bp5/`.
The complete common bundle now selects
cargo-sort-derives 0.14.1 instead of 0.13.0, preserving `$crate` paths. Timers has
no affected derive macro/range marker or additional derive-format stage to alter.
The tool requires Rust 1.88, already the product floor. Explicit maintainer setup
is `make install-tools`, then `make tools-check`; no setup or tool validation ran.

The requested root-lock Host update selects one **0.14.4** family through Testkit,
without adding direct Host dependencies. All four official archives match candidate
checksums and `f93cd8854d500616a73eda713d215a20ff0617c7`; their Rust payloads match
0.14.3 exactly. The offline Cargo candidate was prepared/reviewed in an isolated
copy before applying. Every other package version stays fixed; Cargo regenerates
six Windows dependency edges, recorded with the complete reviewed result at
`/tmp/ic-timers-host0144.5jgzykdz/`. No user version bump or Git effect occurs.

The subsequent Host **0.14.5 `bb8f1e71f3d4599618d0f4084d27fa637408f1fb`** matches
upstream main. Its only changes from 0.14.4 are package/release metadata;
`crates/` is byte-identical. Keep the reviewed 0.14.4 candidate rather than
refreshing the graph without a code change. Testkit remains the sole Host caller.

The one undated pending changelog is **0.19.1**; package versions remain 0.19.0.
This is contributor/tooling and test dependency maintenance, with an unchanged
timer API, and no claimed Wasm/instruction saving. README references the released
0.19 API and registry pin without adding a prose/version release blocker.
The approved locked/offline library/test-target compile check passes (2.72s).
Formatting, all snapshot integrity checks, diff and cheap locked metadata pass.
The subsequent entrypoint refresh changes only the canonical verifier and
guidance; integrity/syntax/link/diff checks pass without repeating Rust builds.
The final 0.8.0 refresh changes selected guidance and manifest provenance only;
snapshot integrity/link/diff checks pass. Incoming Cargo/tool pins/index are
preserved; no code or validation-permission change requires another build.
Tests, Clippy, simulator, full/native gates and the new tool bundle's preparation/
admission remain maintainer-owned. No sibling source, staging, commits, tags,
push, publication, workflow dispatch or cleanup ran here.

## Issue acceptance and authority

[#49](https://github.com/dragginzgame/ic-timers/issues/49) is closed from actual delivered-
source hosted proof: its successful MSRV job selects Rust 1.88.0 and executes the
locked library Wasm check, then doctests/probe lint. The raw log includes the exact
checkout SHA; [the evidence owner](../releasing.md#0190-delivery-and-0191-maintenance)
records its digest and retained path. This closes that floor-qualification gap,
without closing unrelated native/source qualification.
[#47](https://github.com/dragginzgame/ic-timers/issues/47) and
[#48](https://github.com/dragginzgame/ic-timers/issues/48) have delivered changes;
retain their remaining Intel acceptance rather than closing on an unfinished job.
The completed ARM log proves the wide-total regression, release-gate fixtures,
Wasm/MSRV checks and recovery/cohort tests at the exact released source; its
retained path/digest are in the evidence owner.

[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains open. Approved
frozen `v0.17.13` [early](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
original producers fail at the intended boundary and upload successfully; its old download verifiers
reject the obsolete candidate path. The corrected released verifier admits those
exact archives locally; that does not change the old hosted outcome. Approved
[late 38154578484](https://github.com/dragginzgame/ic-timers/actions/runs/38154578484)
is complete: original producers fail/upload, and all three download verifiers
pass (Intel 114535962580, ARM 114535962592, Linux 114535962669). Both stages at one
corrected frozen source remain outstanding. The previous corrected-source dispatch
question is unanswered; compile-only permission does not select a new ref or
repeat dispatches. Existing archives/digests remain at
`/tmp/ic-timers-01713-qualification.n07_3uur/`.
The prepared follow-up selects frozen v0.19.0 for both stages, early then late
after main CI completes. A new explicit dispatch approval is pending; no new
qualification run has started.

AGENTS.md retains the approved offline compile-only exception, including already
prepared native/Wasm compilers. Tests, doctests, Clippy, PocketIC, setup and full
validation require explicit authorization. Release execution and real Git effects
stay user-owned. Siblings stay read-only; native substitutes remain separate from
IC trap/rollback evidence. Canic's existing owner thread retains construction and
coherent-family adoption feedback; no downstream edits are inferred from delivery.
