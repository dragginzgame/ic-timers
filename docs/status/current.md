# Current status

Last updated: 2026-08-15

## Purpose

This is the compact handoff for a new session. `ic-timers` is a higher-level
wrapper around `ic-cdk-timers`: the CDK crate remains the low-level provider,
while this crate owns bounded identity, policy, arbitration, observation, and
lifecycle reconstruction.

Historical implementation and release detail belongs in `CHANGELOG.md`,
`docs/changelog/`, and `docs/audits/`, not in this handoff.

## Release state

- Workspace package version: `0.6.0`.
- Open release line: `0.6.1`; package remains `0.6.0`.
- Direct provider dependency: exact `ic-cdk-timers` 1.0.0.
- Minimum supported Rust version: 1.88.0.
- Development toolchain: Rust 1.97.1.
- The maintainer owns commits, tags, pushes, version bumps, and publication.

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

- Normal CI passes with 84 native tests, warning-denied Clippy and rustdoc,
  Wasm compilation, offline package verification, formatting, provider
  boundary checks, release wiring, and release-truth checks.
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

## Downstream state

- IcyDB has completed a hard-cut adoption of exact `ic-timers` 0.5.0. Its
  maintained Rust 1.88, warning-denied lint, dependency, lifecycle, Candid,
  and real-canister recovery evidence passes; the historical tagged 0.3.4 and
  post-tag 0.3.8 subjects remain distinct in its adoption record.
- Canic has completed a hard-cut adoption of exact `ic-timers` 0.5.0. Its
  schema-3 runtime projection exports shared timer, instruction, and bounded
  memory-page observations from one resolved package without a second
  scheduler; its earlier exact-0.3.8/schema-2 subject remains historical.
- This repository has not modified either downstream repository and does not
  claim combined framework qualification.
- A combined Canic/IcyDB/application Wasm must resolve exactly one `ic-timers`
  package and show both owners in one inventory, synchronous lifecycle
  reconstruction, IcyDB Watchdog recovery, and continued Canic timer progress.
  The open blocker is Canic's lifecycle-composition seam, not `ic-timers`
  scheduler correctness.
- IcyDB owns any allocator-derived sub-page live-byte bound and its maximum
  64-index fanout probe; page extent alone cannot supply either result.

## Next action

Coordinate Canic and IcyDB onto the 0.6 inventory hard cut atomically, then
qualify one combined Wasm through Canic's lifecycle-composition seam without
adding a second registry or compatibility path. Downstream 0.6 performance
baselines must label the corrected interval boundary and retain external
message-exhaustion evidence for hard instruction-limit claims.
