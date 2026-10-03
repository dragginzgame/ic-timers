# Current status

Last updated: 2026-10-03

## Purpose

This is the compact handoff for a new session. `ic-timers` is a higher-level
wrapper around `ic-cdk-timers`: the CDK crate remains the low-level provider,
while this crate owns bounded identity, policy, arbitration, observation, and
lifecycle reconstruction.

Historical implementation and release detail belongs in `CHANGELOG.md`,
`docs/changelog/`, and `docs/audits/`, not in this handoff.

## Release state

- Workspace package version: `0.8.3`.
- Latest release line: `0.8.3`.
- Snapshot registration continuity and exact-deadline Watchdog scheduling are
  implemented, with validation recorded in the
  [0.8.0 release note](../changelog/0.8.0.md). The version bump and local
  `v0.8.0` release tag are complete. Publication is corroborated by the cached
  registry package, its VCS commit and Toko Miner's registry lockfile; see the
  [dated downstream record](../adoption/toko-miner.md).
- The 0.8.1 tooling and lint-expectation changes have a completed version bump,
  dated changelog and local `v0.8.1` release tag. Registry publication was not
  checked in the 2026-10-03 repository review.
- Direct provider dependency: exact `ic-cdk-timers` 1.0.0.
- Released in 0.7.1: exact `ic0` 1.2.0 in both lockfiles; existing
  platform bindings are unchanged upstream. See the
  [0.7.1 release note](../changelog/0.7.1.md).
- The nested test canisters use exact `ic-cdk` 0.20.3 (updated in 0.7.1);
  the published timer crate does not depend on `ic-cdk` itself.
- Minimum supported Rust version: 1.88.0.
- Development and hosted CI toolchain: Rust 1.99.0.
- Hosted PR/main checks lint all supported nested probe configurations on
  both Rust 1.99.0 and the Rust 1.88.0 MSRV.
- Commits, tags, pushes and publication are user-owned. Automated contributors
  prepare only the next changelog section and release-line note. Version bumps,
  staging and all `make release-*` commands are also user-owned. Test, build
  and lint gates run only when explicitly requested.

## Canonical runtime

- One volatile canister-local runtime owns a fixed-capacity registry of 64
  deterministic structured identities.
- The crate supports asynchronous `Once`, asynchronous `AfterCompletion`, and
  synchronous pre-armed `Watchdog` policies.
- The registry owns claim generations, callback generations, policy state,
  pending-command arbitration, callbacks, observations, and every provider
  handle. It emits effects but makes no provider calls.
- The private `platform` module is the only direct `ic-cdk-timers` and IC
  system-fact boundary. Repository checks forbid provider re-export or a
  second direct provider path.
- The runtime binds registry effects, executes callbacks without retaining a
  registry borrow, and owns the two-message Watchdog protocol. The scheduler
  commits the cadence successor before separately queued consumer work runs.
- Synchronous, idempotent lifecycle helpers reconstruct retained declarations
  from consumer-owned durable authority. The library persists no timer policy,
  handle, generation, snapshot, or consumer recovery state.
- Every registration exposes claim-scoped `has_armed_wakeup()` observation.
  It reports canonical provider-handle ownership, not durable demand or a
  delivery guarantee.
- Snapshots are inert provider-neutral values. They contain closed
  policy-specific state, outcomes, counters, instruction aggregates, bounded
  memory-page observations, and an explicit runtime epoch.

## 0.6 hard cut

- `timer_inventory()` returns one `TimerInventorySnapshot` with the runtime
  epoch and complete deterministic timer vector. An initialized empty registry
  now exposes its counter-reset boundary atomically.
- The superseded bare-vector `timer_snapshots()` function is removed. No alias,
  deprecated forwarder, or second inventory shape is retained.
- `timer_snapshot()` remains the focused lookup for one known identity.
- Public measurement documentation defines scheduler/work values as the
  accepted `ic-timers` execution interval, not application-only work or the
  complete IC message. Provider entry/exit work, page reads, and the bounded
  summary write remain outside the instruction delta. A terminal
  `RemoveWhenStopped` declaration may disappear before its final measurement
  is retained because no timer remains to expose it.

## 0.7.0 immediate Watchdog continuation

- `WatchdogDecision::ContinueImmediately` replaces the exact cadence successor
  committed before work with one scheduler deadline at current IC time. It
  remains a later replicated scheduler message and never recurses into work.
- `WatchdogRegistration` and `WatchdogContext` expose
  `ensure_scheduled_immediately()`. Inactive immediate demand arms one
  zero-delay scheduler; a later cadence deadline is replaced; equivalent,
  earlier, repeated, and dispatched demand coalesces; a running request applies
  to that exact attempt's successor.
- `reconcile_watchdog` hard-cuts its desired state to the policy-specific
  `WatchdogReconcileState`, whose `ScheduledImmediately` variant covers the
  first actionable wake-up. No compatibility alias or dual reconciliation path
  remains. This public pre-1.0 hard cut was released in `0.7.0`.
- Continue stays cadence-based. Invariant failure, stop, cancellation, and
  unregistration remain terminal; unregistration is sticky, later cancellation
  wins, later ensure can re-enable cancellation as before, and immediate demand
  cannot be downgraded by cadence ensure.
- The runtime state variants, snapshot fields, persistence count, provider
  boundary, callback roles, registry/handle bounds, and two-message protocol
  are unchanged. Immediate state projects through existing continuation mode,
  zero delay, deadline, request, arm, and coalescing observations.

## 0.5 hard cut

- The shared public `TimerContext` is removed. `OnceContext`,
  `AfterCompletionContext`, and `WatchdogContext` expose only policy-valid
  nested control operations while sharing one private exact-token mechanism.
- Public `TimerError::WrongPolicy` is removed; an invalid cross-policy context
  operation is no longer expressible. Defensive internal mismatches fail as
  ownership invariants.
- Unused request-sequence counters and
  `TimerControlFailure::RequestSequenceExhausted` are removed. Callback
  generations and the registry's sole pending command remain the actual stale
  delivery and request-order authorities.
- Normally completed scheduler and work callbacks sample Wasm and stable
  memory extents in 64 KiB pages at start and end. Each role retains only a
  saturating sample count, the latest extent pair, and maximum observed growth.
- Absolute memory extents are never totaled. Trapped or instruction-exhausted
  work commits no sample. Page extent is a runtime-epoch-local high-water
  observation, not exact live bytes or sub-page allocator liveness.
- Async ordinary measurements can include canister activity interleaved while
  the callback future awaits; they are not exclusive allocation attribution.
- Paired instruction/page measurements follow the callback token's canonical
  role through one registry path. Contradictory policy/role input fails as an
  ownership invariant rather than silently losing accounting.
- No compatibility alias, deprecated forwarder, legacy feature, alternate
  registry, provider fallback, or second pending-command machine is retained.

## Current evidence

- The 2026-10-03 follow-up now includes transient terminal-failure cleanup,
  shared control detachment and Watchdog arming, and stronger release/provider
  validation. `make ci` passes with 108 native tests, as do MSRV checks,
  nested-probe linting and strict production Wasm Clippy on both toolchains.
  The nine audited PocketIC Watchdog subjects and four policy cohorts pass
  after the runtime changes. Track the native boundary and compiler fixtures in the
  [0.8.2 note](../changelog/0.8.2.md). No performance improvement is claimed.
- The recorded 0.8.1 Rust 1.99.0 update passes `make ci`, including all 105 native
  tests, warning-denied Clippy/rustdoc, Wasm checking and offline package
  verification. Rust 1.88.0 workspace checks and the changed inventory test
  pass. All supported nested-probe lint configurations pass on both 1.99.0
  and 1.88.0; the runtime probe also passes Wasm checking on 1.99.0. Workflow
  syntax passes `actionlint`. PocketIC suites were not rerun for this update.
- IC-TIMERS-001/002 fixes pass the same CI, MSRV and nested-probe gates, plus
  strict production Wasm Clippy on both Rust 1.99.0 and 1.88.0. The nine active
  lint expectations are fulfilled; the obsolete scheduler suppression is
  removed. Supplemental root MSRV test lint complexity remains advisory as
  recorded in the [0.8.1 note](../changelog/0.8.1.md).
- Recorded 0.8.0 validation passes 105 native tests and nine audited
  PocketIC Watchdog subjects, plus warning-denied lint/docs, MSRV, Wasm, nested-probe lint,
  provider-boundary and package checks. Registration continuity, exact-deadline
  sleeping/replacement, reset/regrowth and interrupted deadline-proposal
  recovery are covered. Later owner-supplied downstream adoption evidence is
  scoped separately below; no cost saving is claimed.
- The remaining entries below retain earlier release evidence and performance
  observations; the policy size/performance cohorts were not rerun for 0.8.0.
- Package validation passes with 94 native tests, warning-denied Clippy and
  rustdoc, Wasm compilation, offline package verification, formatting,
  provider-boundary checks, Rust 1.88 workspace checking, and every supported
  nested-probe lint configuration.
- Pinned PocketIC 15 passes eight Watchdog subjects. New zero-delay initial and
  successful immediate-continuation subjects execute without cadence-time
  advancement; the existing explicit trap and actual 40-billion-instruction
  exhaustion recovery, lifecycle, isolation, capacity, and cancellation
  subjects remain green.
- One same-callback PocketIC observation reports an immediate-replacement work
  interval of 27,811 instructions versus 19,451 for cadence retention, with
  scheduler/work cycle deltas of 30,742,889 and 30,732,414. These are accepted
  runtime intervals and pair-level cycle deltas, not complete-message totals.
- The 0.7.0 optimized size cohorts report 262,791 bytes for Watchdog versus
  261,914 for after-completion: +877 bytes (0.335%). Registry capacity and the
  at-most-two Watchdog handle bound do not change.
- Rust 1.88 passes the complete workspace and every supported nested probe
  configuration.
- The root dependency graph contains no duplicate packages.
- Focused PocketIC 15.0.0 tests execute the real Wasm memory-size operations.
  An explicit trap and actual 40-billion-instruction exhaustion each leave a
  coherent scheduler sample, no work sample, a committed successor, and later
  normal work with one coherent sample.
- A focused optimized Watchdog cohort reports 200 call-context instructions
  for both an empty bracket and the four start/end Wasm/stable page reads: an
  observed sampling delta of zero, kept outside callback aggregates. This is a
  PocketIC regression subject, not a future metering guarantee.
- A bounded minimal-versus-representative Watchdog calibration records the
  available scheduler/work instruction intervals and cycle deltas. PocketIC
  15 does not expose a per-message update instruction total through its public
  test API, so the complete-message total and unaccounted difference remain
  explicitly unavailable rather than being inferred from cycles.
- The 0.6 inventory subject reruns the complete PocketIC watchdog matrix and
  all policy cohorts successfully. The inventory hard cut does not change
  provider binding or the two-message protocol.
- Release truth, the root and testing lockfiles, and each dedicated release
  note are mechanically checked for agreement.
- Explicit maintainer-owned bumps may release a repository-only patch after a
  clear advisory. Empty subjects remain rejected, and the complete CI, MSRV,
  nested-probe, and PocketIC release gates still run.

## Downstream state

- Read-only inspection on 2026-10-02 finds Toko Miner selecting registry
  `ic-timers` 0.8.0, Canic 0.110.49 and IcyDB 0.262.2. Its lockfile contains
  one timer package; the downstream owner reports `make timer-check` passing
  for Game Shard and Translation. This is dependency evidence, not proof of
  current deployment or recovery.
- The retained 2026-09-20 receipt for Canic 0.110.33 and IcyDB 0.261.0 records
  one 0.8.0 timer package in every role, shared gameplay timer inventory,
  idle/wake behavior and scoped managed same-release state/timer recovery.
  The old lifecycle-composition blocker is resolved for that recorded subject.
  Concurrent gameplay changes were excluded; no current staging deployment,
  general combined qualification or cost comparison follows from the receipt.
- The [dated downstream record](../adoption/toko-miner.md) preserves source
  identities, publication evidence and scope. Earlier exact-0.5.0 adoption
  records for [IcyDB](../adoption/icydb.md) and [Canic](../adoption/canic.md)
  remain historical. This repository did not rerun or modify downstream suites.
- IcyDB owns any allocator-derived sub-page live-byte bound and its maximum
  64-index fanout probe; page extent alone cannot supply either result.

## Next action

The maintainer selected 0.8.3 for release workflow fixes and clarified that
`make release-patch` must always bump. Its undated changelog and
[release-line note](../changelog/0.8.3.md) are prepared; Cargo and both lockfiles
remain at 0.8.2. The user committed 0.8.2 at `9f4e1d5`, without a release tag.
Impact classification now uses the most recent reachable release tag, so the
absent `v0.8.2` does not block advancing to 0.8.3. The user stages and commits
these changes, then runs `make release-patch` for the complete gate, bump,
release commit, annotated tag and push. No tests or release commands were run
for these follow-up tooling edits; deployment validation is user-owned.

Remaining application work is Toko Miner's auxiliary checkpoint deadline
registration and diagnostic registration-sequence projection. The published
exact Watchdog scheduling and `TimerSnapshot::registration_id()` APIs already
support those changes; application owners must qualify them against their
actual artifacts. Current deployment, complete transfer evidence and live cost
comparisons remain downstream work. No sibling repository was edited.
