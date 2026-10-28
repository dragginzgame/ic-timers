![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-28

## Source and delivery

Base and verified remote `main`: **0.31.13**,
`c9c22eaae725c62de5b1918aeda45702f9f213b9`. Registry publication of 0.31.13 is
verified (not yanked). The checkout was clean before this follow-up. The pending
**0.31.14** draft describes repository-only boundary enforcement/documentation;
no Timers runtime, public API, Cargo version or root lock changed. Bundle this
into the next code-bearing release unless the maintainer explicitly selects a
repository-only patch. No staging, commits, pushes or release execution ran.

Shared **0.27.3** remains at reviewed revision
`7ee5c83744aa822ecd070703dbf0a7d95076d5f6` (134 exported payloads). Build/Host/Testkit
**0.69.4**, Platform **0.7.1**, Metrics **0.16.3**, Wire **0.7.18** and PocketIC
**16.1.0** remain selected. PocketIC still constrains Thiserror to **2.0.18**.
The prepared tool selections and adoption proof remain at
`/tmp/ic-timers-tooling-03113.y1v9ur/`; ordinary checks remain offline.

## Issue disposition

- [#78](https://github.com/dragginzgame/ic-timers/issues/78) is **closed** after
  delivery and review of native ownership states plus installed competing-setter
  rollback/firing/cancellation evidence. Equal writes, overdue foreign clears and
  interference followed by no rearm remain undetectable; keep one global owner.
- [#79](https://github.com/dragginzgame/ic-timers/issues/79): the published pure
  predicate composes with the host's existing inspector. Timers exports no second
  hook, and endpoint caller authentication remains independent. Installed Timers
  evidence covers ingress refusal, foreign-call authentication and legitimate
  self-dispatch. Direct refused ingress had no observed target cycle decrease;
  foreign refusal cost 5,485,070 cycles in that fixture. Canic adoption is now
  prepared and installed-tested in the isolated checkout below; it is not on
  Canic main. Keep the issue open for the remaining delivery boundary.
- [#77](https://github.com/dragginzgame/ic-timers/issues/77): 0.31.13 delivers the
  shared pre-poll delivery guard and exact owner reconciliation repair, including
  unchanged deadlines and initial/successor scheduling. Native tests cover stale
  generation isolation and live/lost queued work without effect replay.
  **Still unqualified:** installed stop-between-submit-and-scheduler-poll and
  scheduler-body trap acceptance. All prior stop experiments ran the scheduler
  first, including a same-round controller stop. The maintainer has no original
  reproducer. Do not repeat unchanged experiments or count ordinary stop/resume
  as this proof. Temporary encoders, RPCs, Candid selections and exceptions were
  removed; the retained qualified probe bytes were restored exactly.
- [#80](https://github.com/dragginzgame/ic-timers/issues/80): pending docs/checks
  specify Platform primitives versus Timers' private dispatch/flight ownership,
  keeping application calls/Relay above the runtime. A real offline Cargo fixture
  refuses renamed target-normal and host-build Relay edges; the existing codec,
  feature, executor-source and coordinator boundaries remain. No application-call
  wrapper, callback change or extra runtime was introduced. Delivery remains pending.

## Authorized Canic work

The maintainer selected **Prepare focused Canic changes**. Scope is that exact
repository; other siblings remain read-only. Shared's isolation requirement keeps
`/home/adam/projects/canic` untouched, including its substantial pending Wire/Host
batch. Prepared checkout: `/tmp/ic-timers-backlog-03114.GlZdEf/canic`, based on
verified Canic main **0.112.0**, `5c236ab130c0d63892ef281b45bdc3fa353e012f`.
No staging/commit/push/version mutation occurred there. The original catalog
independently advanced to Timers 0.31.13/Tooling 0.69.4 during preparation;
a read-only apply check confirms `canic-code.patch` fits the three changed source
files. Reconcile existing metadata/changelog work rather than overwriting it
with the isolated base's full patch.

Its existing single inspector now rejects Timers' private method before argument
copy/acceptance. The pool's duplicate cancellation custody is shared locally,
preserving maintenance-first ordering, empty success and typed busy/provider
failures ([Canic #695](https://github.com/dragginzgame/canic/issues/695)). The root
minimum is published Timers **0.31.13**; all three locks move only Timers and its
required Platform/macros **0.7.1**. The undated **0.112.1** notes and runtime ingress
documentation are updated. This evidence qualifies that isolated base plus patch,
not the original dirty Canic batch.

Canic [#688](https://github.com/dragginzgame/canic/issues/688) and
[#689](https://github.com/dragginzgame/canic/issues/689) are closed after verifying
their existing metric/output consolidations on remote main. Core's metric source
matches its retained focused proof; all five host output paths preserve kind and
record-before-measure ordering. Fresh query/update/binary command-mode tests pass.
These closures do not certify unrelated failed Canic CI jobs.

The larger call-stack retirement stays with
[Canic #700](https://github.com/dragginzgame/canic/issues/700) and
[Platform #75](https://github.com/dragginzgame/ic-platform/issues/75).
A Rust 1.91 witness against published Platform 0.7.1 confirms that a prepared Call
cannot own a runtime-selected method name: the owning factory fails E0515 while
the borrowed control passes. Minimal example and acceptance checks are posted on
the existing issues. Platform already supplies `get_cost`; no duplicate request
wrapper or invented cost API was added.

## Verification and limits

Current evidence: `/tmp/ic-timers-backlog-03114.GlZdEf/`. Timers' actual dependency
fixture passes (including renamed Relay normal/build edges and existing negative
cases), as do direct dependency checks, provider checks, Bash syntax, scoped
ShellCheck with source following, changelog admission and whitespace checks. The
root lock matches HEAD byte-for-byte. Initial fixture diagnostics are retained:
a nested workspace needed exclusion; target-build cfgs must select the host.

Canic's native cancellation regression and strict pool library/test Clippy pass.
The two installed ingress tests pass with Rust 1.99.1, Testkit 0.69.0 and admitted
PocketIC 16.1.0: private ingress refusal leaves the observed target balance
unchanged, ordinary calls succeed, and default/explicit/named limits hold. Strict
Core library and ingress-target Clippy pass. Rust 1.91 Wasm checks for both changed libraries, direct boundaries, all three
provider identity checks, native changelog governance and three host command-mode
tests also pass. Full and code-only Canic patches plus a PR description are in
the evidence directory. No broad release gate or native macOS/live IC
qualification is claimed for these prepared changes.

Published Timers main CI
[40671619214](https://github.com/dragginzgame/ic-timers/actions/runs/40671619214)
has passed Linux checks, MSRV and macOS arm64; Intel macOS was still running at
last observation. The tag-only job passed separately. Neither run tests the new
repository-only boundary changes. Canic main CI
[40659441082](https://github.com/dragginzgame/canic/actions/runs/40659441082) passes
preflight/security/checks/MSRV/release-build but fails both macOS Host builds and
ordinary/PocketIC jobs; these precede this isolated patch. Do not claim Canic's
complete batch is qualified.

Earlier runtime evidence: `/tmp/ic-timers-issues-03113.HKcNGJ/`, including the
optimized runtime-probe hash
`380e416c802e945de291051cd2d5ee1371c029b79bba36b19d0ca5e57e3ad41c`, native
Watchdog/ordinary/ownership/ingress tests and nine focused PocketIC subjects.
Earlier freeze proof remains at `/tmp/ic-timers-freeze-03112.voNdez/` and dependency
proof at `/tmp/ic-timers-upstream-03112.xc99Cb/`. Each retains its original graph,
artifact and host scope. Preserve full user-operated release and host gates.
