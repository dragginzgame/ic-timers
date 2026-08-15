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

- Workspace package version: `0.4.1`.
- Open release line: `0.5.0`; package remains `0.4.1`.
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

## Open 0.5 hard cut

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
- The full PocketIC recovery matrix and policy cohorts were not repeated for
  the 0.5 cleanup because provider binding and the two-message protocol are
  unchanged. The maintainer-owned release gate retains both suites.
- `make release-impact` classifies the current tree as crate-impacting. The
  0.5 changelog and release note are prepared, but no version bump or release
  action has been performed.

## Downstream state

- Tagged IcyDB 0.226.1 adopted exact `ic-timers` 0.3.4. Its validated post-tag
  integration moves to exact 0.3.8 and uses claim-scoped wake-up observation.
- A validated uncommitted Canic worktree hard-cuts its parallel timer runtime
  to exact 0.3.8 and derives timer status and metrics from the shared snapshot.
- This repository has not modified either downstream repository and does not
  claim that either has qualified the open 0.5 API.
- A combined Canic/IcyDB/application Wasm must resolve exactly one `ic-timers`
  package. The eventual 0.5 adoption must update all exact pins atomically.
- IcyDB owns any allocator-derived sub-page live-byte bound and its maximum
  64-index fanout probe; page extent alone cannot supply either result.

## Next action

Review the 0.5 hard cut with Canic and IcyDB. Incorporate owner feedback without
adding compatibility paths, then let the maintainer run the version bump. That
bump executes the complete release gate, updates both lockfiles, dates the
prepared changelog, and stages release truth automatically.
