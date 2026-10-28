![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-28

## Source and delivery

Base and verified remote `main`: **0.31.14**,
`48c74439341ac4619f5b8a3ebe77c5e4dd4fa54f`. The checkout was clean before this
follow-up. The pending **0.31.15** draft adds maintained installed recovery
qualification and its documentation. The release-impact owner classifies it as
**repository** work: timer library code, public API, Cargo version and root lock
remain unchanged. Bundle this into the next code-bearing release unless the
maintainer explicitly selects a repository-only patch. No staging, commits,
pushes, releases or sibling mutations ran in this follow-up.

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
- [#77](https://github.com/dragginzgame/ic-timers/issues/77): the previously
  missing installed stop-between-submit-and-scheduler-poll boundary is now
  **qualified locally**, including initial and successor scheduling under all
  three owner modes. Eight instruction-heavy ordinary timers run ahead of the
  due scheduler. The test requires committed earlier work with no new scheduler
  start, then stops/restarts the canister. Another two seconds and 16 rounds
  cannot repair it. Explicit `Scheduled`, `ScheduledImmediately` or unchanged
  `ScheduledAt` reconciliation restores exactly one attempt while preserving
  registration identity, completed work and unacknowledged state. A negative
  control disabling only scheduler-loss recording fails the same maintained
  test; the actual production guard passes. This is real backend/IC execution,
  not a native provider substitute. It replaces the old inconclusive stop
  experiments; the original unavailable audit reproducer is no longer needed.
  **Limits:** scheduler-body normal return remains the existing protocol
  assumption; arbitrary scheduler-body traps or permanently live futures are
  not qualified. The new test/roster/docs are pending delivery, so #77 stays open.
- [#80](https://github.com/dragginzgame/ic-timers/issues/80) is **closed** after
  verifying the Platform/Timers/application boundary and Relay exclusion reached
  main in 0.31.14. The existing raw integration meets the infrastructure contract;
  no wrapper, callback change or extra runtime was required.

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

Current recovery evidence: `/tmp/ic-timers-scheduler-pressure.YxSpot/`. The
maintained named PocketIC subject passes all six initial/successor × owner-mode
trajectories with Rust **1.91.0**, selected Testkit **0.69.4**, admitted PocketIC
**16.1.0** and retained optimized probe Wasm
`89b3d66973489060f82cbef51e43a6863cdf49227a0e2162067b8a0e4d02add0`.
Final positive execution: `final-maintained-test.log` (7.63s test). Its causal
negative-control run is `final-negative-test.log`: disabling only
`registry.abandon_scheduler(token)` in the isolated clone fails the first
trajectory's recovery observation after 64 rounds. The production library in
this checkout was never modified. Earlier exploratory passes and diagnostics
are retained separately; none substitutes for the final maintained test.

Strict all-target MSRV Clippy passes for the changed probe/protocol/coordinator,
as do optimized MSRV probe Wasm, formatting, required-subject discovery, direct
and transitive dependency/provider boundaries, changelog admission and whitespace
checks. `qualified-inputs.json` verifies all 53 Rust/dependency inputs unchanged
across final execution. The new fixture is required by the normal Watchdog
roster; it adds no optional manual release command. Initial formatting attempted
MSRV rustfmt, which is not installed; formatting instead used the selected
Rust 1.99.1 toolchain. No component was installed and final formatting passes.
No complete local release gate, native macOS or live IC qualification ran.

Previous boundary/Canic evidence remains at
`/tmp/ic-timers-backlog-03114.GlZdEf/`, including its full/code-only Canic patches
and PR description. Those original checks retain their source/graph scope.
Canic's isolated cancellation/ingress, strict Clippy, Rust 1.91 Wasm, dependency
identity, changelog and host command tests pass on its recorded base; they do not
qualify the original sibling's concurrent dirty batch. Canic main has not
advanced, so #695 and Timers #79 remain awaiting consumer delivery.

Timers 0.31.13 main CI
[40671619214](https://github.com/dragginzgame/ic-timers/actions/runs/40671619214)
completed successfully, including MSRV and both native macOS jobs. The 0.31.14 run
[40677053990](https://github.com/dragginzgame/ic-timers/actions/runs/40677053990)
was pending at this follow-up's observation; it does not test the new local
fixture. Do not hold work open for optional hosted qualification.
Canic main CI
[40659441082](https://github.com/dragginzgame/canic/actions/runs/40659441082) has
pre-existing macOS Host and ordinary/PocketIC failures; its full release batch
was not reassessed or repaired here.

Earlier runtime evidence: `/tmp/ic-timers-issues-03113.HKcNGJ/`, including the
optimized runtime-probe hash
`380e416c802e945de291051cd2d5ee1371c029b79bba36b19d0ca5e57e3ad41c`, native
Watchdog/ordinary/ownership/ingress tests and nine focused PocketIC subjects.
Earlier freeze proof remains at `/tmp/ic-timers-freeze-03112.voNdez/` and dependency
proof at `/tmp/ic-timers-upstream-03112.xc99Cb/`. Each retains its original graph,
artifact and host scope. Preserve full user-operated release and host gates.
