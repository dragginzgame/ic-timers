# Changelog

All notable changes to this project are recorded here.

## [Unreleased]

## [0.3.0]

### Added

- Add the 0.3 production runtime, centered on a bounded shared registry and a
  two-message watchdog that commits its successor before fallible consumer
  work.
- Freeze the 0.3 Patch 1 runtime contract: a 64-entry canonical registry,
  policy-specific callback and state models, linear provider ownership,
  truthful dispatch and completion counters, lifecycle composition, and
  comparable Wasm and instruction measurement subjects.
- Add the pure fixed-capacity registry engine with unique identity claims,
  deterministic inventory ordering, retained and remove-on-stop lifetimes,
  checked generation and request arbitration, and policy-specific ordinary
  and watchdog transitions.
- Add focused registry tests for the complete directive/decision matrix,
  duplicate and capacity failures, idempotent ensure and cancellation,
  claim-consuming unregistration, stale generations, nested commands,
  unacknowledged watchdog dispatches, and coherent Canic-shaped projection.
- Add one canister-local volatile runtime with a synchronous, idempotent
  initialization seam, callback-owning `Once` and `AfterCompletion`
  registrations, exact-claim control, and bounded live snapshot access.
- Add a native one-shot provider harness and focused runtime tests for linear
  handle replacement/cancellation, execution without a registry borrow,
  after-completion recurrence, nested callback commands, and declaration
  lifetime.
- Add the live synchronous `Watchdog` registration and its two-message
  one-shot protocol: a bounded scheduler owns the next cadence successor and
  queues separate immediate work before returning.
- Add owner-local watchdog execution tests plus an isolated PocketIC probe
  proving that an explicitly trapped work message leaves the previously
  committed successor able to retire the attempt and make later progress; the
  probe also verifies rejection of external CDK timer-executor ingress.
- Add synchronous, idempotent reconciliation helpers for retained
  after-completion and watchdog claims, with explicit scheduled/inactive
  desired state and configuration-conflict rejection.
- Populate normally completed scheduler/work instruction aggregates from IC
  call-context counter type 1, and add an IcyDB-shaped fixture covering
  consumer-owned readiness, one-page work, terminal stop, and commit-guard
  re-enablement.
- Extend the PocketIC probe through upgrade, proving a fresh runtime can
  reconstruct a scheduled watchdog from consumer-owned durable authority
  before the probe's downstream-hook observation and then make progress.
- Complete the PocketIC watchdog promotion matrix with real instruction
  exhaustion, insufficient-cycle deferral and top-up, stop/resume, overdue
  coalescing, scheduler/work-gap cancellation, duplicate demand, simultaneous
  timer isolation, commit-window ensure, and a measured ordered 64-entry
  inventory.
- Add mutually exclusive baseline, Once, after-completion, and Watchdog probe
  cohorts with reproducible Rust 1.88 raw Wasm, instruction, and PocketIC cycle
  measurements; document exact hashes, deltas, complexity, and downstream
  adoption work in the 0.3 evidence report.

### Changed

- Lower the declared minimum supported Rust version from 1.91.0 to 1.88.0
  after proving the current source and locked dependency graph compile for
  both native and `wasm32-unknown-unknown` targets.
- Hard-cut the pre-1.0 value model to validated positive cadence, configured
  after-completion recurrence, closed policy-specific states, private inert
  snapshots, truthful unacknowledged outcomes, split scheduler/work counters
  and instruction aggregates, and no fabricated elapsed-time field.
- Make the provider module private and its timer handle linear. Actual provider
  binding for ordinary timers now stores each returned handle in its canonical
  entry and increments actual-arm counters only after that ownership commits.
- Read IC time and canister version through the private platform boundary using
  the already-resolved exact `ic0` 1.1.0 dependency; retain the exact
  `ic-cdk-timers` 1.0.0 provider pin.
- Extend each watchdog entry's linear ownership to at most two exact provider
  handles: one cadence successor and one dispatched work callback. Terminal
  decisions clear both as applicable, while normal continuation retains the
  already-armed successor.
- Project a retained timer that stops on a retryable failure as `failed`,
  preserving Canic's current operator condition instead of reporting it as
  generically idle.
- Clarify that watchdog trap/exhaustion recovery protects the consumer-work
  message, freeze IcyDB's exact returned-result mapping, and require downstream
  dependency unification plus a canister-wide direct-provider inventory.
- Freeze Canic's adoption boundary: its infallible timer facade must expose
  typed registry-capacity and identity-bound failures rather than panic,
  evict, truncate, or silently ignore them.
- Stage named release notes under an explicit undated version heading and
  teach the release helper to add the date automatically during the version
  bump.

## [0.2.0] - 2026-08-13

### Added

- Add the candidate 0.2 provider-neutral `snapshot` API, including bounded
  structured timer identities, policy and scheduling state, outcomes,
  counters, performance measurements, runtime epochs, and one canonical
  snapshot type.
- Preserve operational distinctions required by Canic migrations: callback
  starts versus completions, completed versus interrupted work, requested
  schedules versus platform arms, and functional consecutive expected-failure
  state.
- Add saturating counter transitions, explicit observation resets, portable
  directive conversion, and total/latest/maximum instruction and elapsed-time
  aggregates.
- Add focused tests for identity validation and ordering, outcome transitions,
  counter partitions, absent measurements, epoch resets, and a Canic-shaped
  lossless projection fixture.
- Add the 0.2 observability and Canic parity contract, an explicit safety
  boundary, and a recurring code-hygiene audit.
- Add Dependabot coverage for Cargo and GitHub Actions dependencies.
- Add tested release automation that promotes `Unreleased` notes into a dated
  version section, plus a guarded crates.io publish target.

### Changed

- Rework the README to identify `ic-timers` as a wrapper around
  `ic-cdk-timers` and explain the shared coordination and recovery rationale.
- Harden the development gate with expanded Clippy lints, rustdoc, shell
  syntax, and pinned GitHub Actions checks.
- Mark public validation errors as non-exhaustive and document the pre-1.0
  public API compatibility policy.

### Not yet implemented

- Registry storage and live inventory population, measured callback execution,
  runtime adapters, lifecycle reconstruction, and recovery watchdog behavior
  remain future work.
- The candidate API remains subject to Canic and IcyDB review; a real downstream
  Canic adapter test is required before the 0.2 contract is accepted.

## [0.1.0] - 2026-08-13

### Added

- Initial workspace, CI, release helpers, and formatting hook.
- Canic-derived deterministic timer control and one-shot platform boundary.
- Architecture and release-line documentation for the planned shared runtime.

### Changed

- Use Rust 1.97.1 for normal development while retaining Rust 1.91.0 as the
  separately checked minimum supported Rust version.
- Document the staged path from public timer snapshot types to the registry,
  lifecycle integration, and recovery watchdog.
