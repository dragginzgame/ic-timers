![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Released source

The maintainer reports **0.18.0 live**. HEAD and annotated `v0.18.0` resolve to
`52bc4bad399122be040cf155d9b4563b5509e009`; root metadata and finalized notes
agree. [Exact-source full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38159662258)
passes Linux (114529738250), MSRV (114529738169), ARM (114529738256) and Intel
(114529738316). The [tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38159662226)
passes separately. It qualifies the delivered construction cut, not the dirty
next graph or downstream adoption.

The release selects Metrics 0.6.6, Testkit 0.34.0, Host 0.14.1, PocketIC 16.1.0
and Shared 0.6.3. Canic received [release availability](https://github.com/dragginzgame/canic/issues/33#issuecomment-6111737141)
and [concrete adoption instructions](https://github.com/dragginzgame/canic/issues/33#issuecomment-6111101911).
Its separate custody rollback remains necessary; sibling files/pins are unchanged.

## Prepared 0.19.0

The incoming maintainer catalog/lock selects Metrics **0.7.0**, Testkit **0.34.1**,
Host **0.14.3**, Candid **0.10.38** and PocketIC **16.1.0**. Their bytes are
preserved. Package versions remain 0.18.0; the one undated pending section is
**0.19.0**, because exposed Metrics types now have `u128` totals and integer means.
Samples/counts/latest/maximum remain `u64`; no narrowing wrapper or compatibility
reader is kept. The [owning contract](../design/callback-delivery-ownership.md#prepared-ic-metrics-07-adoption)
records report/codec propagation, volatile-state scope and required qualification.
Runtime/size probes, PocketIC decoders/calibration, native role/order and Canic
projection fixtures are updated. Wide Candid round-trip coverage uses synthetic
report values separately from actual small instruction deltas. No stable-map
adapter or platform reads are added.

Committed Shared **0.6.5 `42f2091768b5f71c98a684d3dab563f408ca2dd9`** is canonically
adopted through **59/35/11** exports. The [adoption record](../shared-tooling.md)
owns preserved inputs and clean-source preview/apply logs. `make/consumer.mk`
replaces equivalent includes/hook/snapshot wiring; all actual fixture exports
carry it. Direct release policy and ordered Testkit provisioning stay local.
The CI roster runs compiler/Clippy before lengthy release fixtures; both actual
consumer fixtures now agree on its ordering/failure assertions. The release-gate
fixture's stale old order was corrected before execution. Its staged formatter receipt fix and the logger checksum companion are propagated
through the current exports. Optional input checkpoints are not configured in
the local caller. Uncommitted 0.6.6 derive-formatter pins are not adopted; consumer
pins and installed-tool policy stay unchanged. Clean refresh evidence is retained
at `/tmp/ic-timers-shared065.9v5uxcxv/`.

The maintainer explicitly approved offline, locked compile-only development
checks on 2026-10-12. AGENTS.md records that standing exception. The existing
library/all-targets `check` passes, compiling test targets without executing them.
Native library/test-target and Wasm compile-only checks both pass with already
prepared Rust 1.88.0 (`6b00bc388 2025-06-23`) and its Wasm target.
Formatting, all snapshot integrity checks, changed shell syntax, diff and cheap
locked metadata pass. Incoming Cargo/catalog and consumer tool pins compare
byte-identical to their respective preserved incoming inputs. The later incoming
Host 0.14.3 lock change has its own checksum/source review; old Host 0.14.2
observations are not relabeled. Tests, Clippy, API doctests/probe builds, PocketIC,
behavioral fixtures and matched Wasm/instruction measurements remain user-owned
and were not run for this dirty batch. No performance saving is claimed.

The incoming Host **0.14.3** family is published at
`543b137458feba0f6dfd3e458b680fb2be3d9e6d`. All four cached official archives
match selected lock checksums and that source; library Rust source is unchanged
from 0.14.2. It remains transitive through Testkit, with no direct Host requirement
or second identity. Review inputs/archives are at `/tmp/ic-timers-host0143.j57w4_7_/`.

## Remaining qualification and authority

[#47](https://github.com/dragginzgame/ic-timers/issues/47) owns the Metrics cut;
[#48](https://github.com/dragginzgame/ic-timers/issues/48) owns compile-check
prevention and earlier CI admission. [#49](https://github.com/dragginzgame/ic-timers/issues/49)
now has the library minimum-Wasm check prepared in the existing MSRV gate,
explicit hosted target setup, and ordered command/failure coverage. Its local
offline compile proof passes; delivery and hosted acceptance remain pending. Prepared changes and successful compile-only
checks do not establish delivery or full acceptance; retain open qualification.

[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains open.
Approved frozen `v0.17.13` [early](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
producer/upload observations are preserved; all frozen early download verifiers
fail the obsolete candidate path. The corrected released verifier accepts the
same downloaded payloads locally, without relabeling the failed hosted source.
Approved [late 38154578484](https://github.com/dragginzgame/ic-timers/actions/runs/38154578484)
is complete: all original producers fail at the late qualifier and upload;
all three download verifiers pass (Intel 114535962580, ARM 114535962592,
Linux 114535962669). Source remains `e6ac645aac1b185bfaa3b146c2b8e1dd38b3299f`.
These observations do not satisfy both stages at one corrected frozen source.

The earlier asynchronous request to dispatch corrected-source early then late
at `v0.17.14` still has no answer. Compile-only approval does not authorize those
dispatches or silently select a newer ref. Do not repeat original dispatches.
[The existing evidence owner](../releasing.md#01714-delivery-and-frozen-failure-qualification-follow-up)
retains archives/digests and source-bound history under
`/tmp/ic-timers-01713-qualification.n07_3uur/`.

Staging, commits, package/lock release mutation, tags, pushes and publication stay
user-owned. Preserve the unrelated untracked `.vscode/` and incoming Cargo edits.
Sibling checkouts stay read-only. Native substitutes remain separate from IC
trap/rollback evidence. Tests/lint/setup/full gates require explicit authorization;
only the recorded offline compile-only exception changes that boundary.
