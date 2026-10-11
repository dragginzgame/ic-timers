![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Released source

Released **0.17.13** is `e6ac645aac1b185bfaa3b146c2b8e1dd38b3299f`.
It selects Metrics **0.6.4**, Testkit **0.33.8**, all four Host **0.13.5**,
PocketIC **16.1.0** and Shared **0.6.3**
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d` through **56/33/11** exports.
The completed maintainer release journal binds preparation/validation source
`560c9a55f0679380fa6933222acfac564da9ebec`; the release is its sole child and
native test bytes are identical. Local `v0.17.13` resolves to this release.
[The evidence owner](../releasing.md#01713-delivery-and-remaining-issue-acceptance)
records the scope and read-only comparisons at
`/tmp/ic-timers-01713-issues.3zinjt3e/`.

[Full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38149582052)
passes Linux/MSRV with both native jobs queued.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38149582074)
passes separately. Neither pending jobs nor delivery alone establish
complete hosted acceptance. Previous 0.17.12 Linux/MSRV/ARM passes; its Intel
job was cancelled by the newer push. Earlier evidence retains its own source.

## Remaining issues

[#46](https://github.com/dragginzgame/ic-timers/issues/46)'s native fixture cleanup
and formatting correction are delivered and its regression actually executes
and passes in downloaded Linux job 114500137818 among 145 library tests. The
source-bound log/digest is recorded with the evidence owner. This specific repair
is closed using that execution evidence; #44 retains native/adoption acceptance.
Native fixture retirement proves no IC persistence, upgrade or rollback behavior.

[#44](https://github.com/dragginzgame/ic-timers/issues/44)'s canonical storage
adoption and local collector changes are delivered. Complete native and controlled
early/late artifact acceptance remain. The maintainer can use the existing manual
CI dispatch at frozen `v0.17.13`, early then late sequentially, retaining original
failed producers and all six successful download-verification observations.
The authorized controlled run exposed the verifier correction below; no further
runtime change is justified by this finding.

The maintainer now explicitly authorizes the two artifact qualification stages
at `v0.17.13`, sequentially after tag CI finishes. Tag CI passes and the
[early run](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
was dispatched at exact release source. Linux reaches controlled exit 22 and
uploads its artifact; both native producers are queued. Late dispatch waits for
early completion. Source/ref/authority and run observations are retained at
`/tmp/ic-timers-01713-qualification.n07_3uur/`. Close no acceptance issue on
dispatch or queue status alone.
The late stage remains explicitly authorized at this same frozen ref once early
completes; do not request that permission again or silently change its source.

## Prepared 0.17.14

The early artifact really retains rejected bytes under its physical Shared host
generation, but the frozen download verifier still checks the obsolete physical
checkout slot. Pending **0.17.14** replaces that match with the current store
shape, adds malformed/obsolete/duplicate/linked candidate rejection cases and
passes actual driver/collector output to the existing Ubuntu-only verifier suite.
Native macOS and Make release gates gain no Python prerequisite.
[The owner](../releasing.md#early-verifier-shared-store-path-correction) records
the original artifact digest and old rejection/fixed acceptance outputs. The
corrected verifier admits the exact downloaded Linux bytes. Syntax/diff checks
pass; new unit/producer fixtures are not locally executed. No canonical snapshot,
runtime API, Cargo version/lock, consumer pin or production Wasm change occurs.
The local correction does not qualify the frozen hosted early run; #44 remains
open for delivery of the correction and exact-source controlled qualification.

Published Host **0.14.0** requires Testkit's
[owner migration](https://github.com/dragginzgame/ic-testkit/issues/61) and published
minor boundary. Testkit 0.33.8 still admits Host 0.13. Keep its returned CLI/server
path; add no direct Host dependency, parallel graph or mutable sibling selection.
Shared setup/build policy and consumer Binaryen 132/tool pins are unchanged.
Keep `.tools/rust/common/bin` and one `SHARED_TOOLING_STORE` across setup/checks/
evidence; existing generations and failure logs remain retained.

Only inspection, syntax/diff and cheap metadata checks may run. Tests/builds/lint,
setup/deployment and staging/commit/release remain user-owned. The explicit
exception above authorizes only the two qualification stages and their evidence
inspection. It changes no Cargo versions, lockfiles, pins, index or release refs.
