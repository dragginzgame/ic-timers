![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-28

## Source and delivery

Base and verified remote `main`: **0.31.12**, `5a03925f46b2eafe5881f1402ce15ec5dfa4d3bc`.
The checkout was clean before the issue work. The undated **0.31.13** draft
collects compatible fixes/additions for issues 77–79 and the tooling adoption
below. Package versions remain 0.31.12. No staging, commits, pushes,
release execution or sibling edits were performed. Keep user-owned Git/release
delivery; the fixes are not yet on the default branch.

The prior dependency/Shared/freeze batch is committed in the base. The pending
root catalog now requires published Build/Host/Testkit **0.69.4**, matching the
incoming maintainer-updated lock. That lock is preserved byte-for-byte. Platform
**0.7.1**, Metrics **0.16.3**, Wire **0.7.18** and PocketIC **16.1.0** remain selected.
The published PocketIC pin still constrains Thiserror to **2.0.18**.

## Current upstream adoption

Shared **0.27.3**, independently selected revision
`7ee5c83744aa822ecd070703dbf0a7d95076d5f6`, is exported through the canonical
impact/apply workflow with all **134** payloads and unchanged entrypoint roots.
Trusted source comparison proves every declared byte and executable mode against
that commit; ordinary checks remain offline. Optional sccache, compiler wrappers
and schedules are not activated. Unattended maintenance's report-only launch
requires separate installed-host qualification before any scheduled resumption.
Timers retains its existing stricter all-regular repository inventory.

Evidence: `/tmp/ic-timers-tooling-03113.y1v9ur/`. Published archives show identical
Rust implementation files between Tooling 0.69.3 and 0.69.4; reviewed changes align
dependency minimums with the already selected Platform/Wire graph. The changed
Shared catalog invalidated a prior common Cargo receipt; explicit setup reused
admitted builds and prepared fresh selections. Common host/Cargo admission,
editor **0.69.4**, Testkit **0.69.4** and server admission now pass offline.
Setup command: `make install-rust-tools install-release-editor install-testkit-server`.

Current-selection checks pass: locked MSRV metadata, direct and transitive
deployment boundaries, pin/source policy, trusted/local snapshot checks and the
actual consumer export/corruption fixture, formatting, changed-script syntax,
strict Rust 1.91 coordinator/test Clippy and required-subject discovery. Optimized
probe Wasm still matches the prior issue-proof artifact exactly. The installed
ingress and competing-owner subjects both pass freshly with Testkit 0.69.4.
Other earlier runtime checks below retain their original harness/dependency
scope; this adoption does not close #77's gap or qualify macOS/full release gates.
Pending product/probe source changes are preserved exactly.

## Prepared changes and acceptance boundaries

- [#79](https://github.com/dragginzgame/ic-timers/issues/79): new pure
  `is_timer_dispatch_method` predicate composes with the host's existing ingress
  inspector. Timers exports no inspection hook. The runtime probe's single
  inspector combines private-method refusal with its own payload policy; the
  authenticated endpoint remains independent. Native negative cases and an
  installed ingress/foreign-call/self-dispatch regression pass. Direct refused
  ingress leaves the fixture's observed target balance unchanged; foreign
  authentication charges **5,485,070 cycles** in the final fixture. Canic's
  generated hook still needs separate adoption before its payload inspector
  accepts ingress. Its checkout was read only, not qualified.
- [#78](https://github.com/dragginzgame/ic-timers/issues/78): the private backend
  records its last timer write and validates the next returned prior timestamp.
  Conflict traps preserve IC message rollback. Zero at/after the owned deadline
  is legitimate one-shot consumption. Native state transitions and installed
  first-arm/overwrite/premature-clear/rollback/normal-fire/cancel cases pass.
  Equal writes, overdue foreign clears and interference with no subsequent
  backend rearm remain undetectable. One provider/export owner remains required.
- [#77](https://github.com/dragginzgame/ic-timers/issues/77): initial and successor
  scheduler futures share the ordinary pre-poll delivery guard. Confirmed loss
  clears only the current scheduler handle and marks canonical control for owner
  repair. All three reconciliation modes, including unchanged exact deadlines,
  restore it. Queued work keeps its attempt and committed recovery cadence;
  no completion or uncertain-effect replay is invented. Native tests cover both
  scheduler paths, stale generations and live/lost queued work. Jobs' README
  `ScheduledAt` and Canic's `Scheduled`/`ScheduledImmediately` callers were traced.
  **Remaining gap:** installed stop-between-submit-and-delivery and scheduler-body
  trap acceptance are not qualified. Every attempted stop sequence executed the
  scheduler first, including a controller-canister stop submitted in the timer's
  round. The maintainer confirmed the original reproducer is unavailable. Do not
  describe these attempts as recovery proof; a deterministic boundary fixture is
  still needed.

All three canonical issues have factual progress comments and remain open.
No PRs were open when triaged. There are no duplicate/superseded closures.
The next concrete work is the missing #77 installed reproducer, then Canic's
host-hook adoption in its authorized owner. Delivery/required acceptance also
precedes closing any of these issues.

## Verification and retained evidence

Earlier issue evidence root: `/tmp/ic-timers-issues-03113.HKcNGJ/`. Its final logs
qualify the retained product source on Linux using Rust **1.91.0**, the original
Tooling **0.69.3** harness and PocketIC **16.1.0**.
The exact optimized runtime-probe bytes are retained as `runtime-probe.wasm`,
with `final-wasm.sha256`. The loaded copy and workspace artifact match.

Passes: locked metadata; strict library/coordinator/protocol/runtime-probe Clippy;
optimized probe Wasm; public docs with warnings denied; direct provider and
transitive deployment graph/feature exclusions; formatting, changelog and diff
checks. Native evidence includes 45 Watchdog-filtered tests, the scheduler and
ordinary abandonment regressions, three exact ordinary guard tests, four
ownership state tests and the ingress predicate test. Required-subject discovery
passes after adding the two installed regressions to the watchdog roster.

Nine named PocketIC subjects pass against the final bytes: ingress admission,
competing ownership, existing stop/resume, trapped work, instruction exhaustion,
response-trap cleanup, ordinary pre-await abandonment, genuine freeze/top-up,
and near-threshold low-cycle recovery. These are focused executions, not a full
watchdog/cohort/release gate. A normal stop/resume pass does not qualify #77's
different scheduler-abandonment interleaving.

Failed development diagnostics are retained: initially missing diagnostics
directory, inspection's actual `CanisterCalledTrap` classification, an unnecessary
integer conversion, counter Result/Option assertions, and lint corrections.
Unsuccessful stop experiments and their source patch are retained separately.
Their temporary management encoder, native Candid features, boundary exception,
probe DTO/RPC additions and failing test were removed; no such dependency or
enforcement expansion remains. No failed experiment is counted as a pass.
The controller trial also exposed distinct management status DTO identities;
its corrected fixture still missed the timing boundary. Its patch and helper
are retained separately. After removal, hashes prove restoration of every prior
qualified source/dependency input; the rebuilt Wasm again matches the exact
nine-subject PocketIC artifact. Passing evidence is reused for those inputs.

Base main CI [40658240875](https://github.com/dragginzgame/ic-timers/actions/runs/40658240875)
and tag CI [40658240699](https://github.com/dragginzgame/ic-timers/actions/runs/40658240699)
both succeeded for `5a03925`; neither tests the local fixes. No new hosted run,
full local gate, native macOS or live installed-IC qualification was performed.
Keep the full maintainer-owned release and host requirements intact.

Earlier accepted freeze evidence remains at `/tmp/ic-timers-freeze-03112.voNdez/`;
the previous dependency/Shared evidence is `/tmp/ic-timers-upstream-03112.xc99Cb/`.
Those runs retain their original source/artifact scope.
