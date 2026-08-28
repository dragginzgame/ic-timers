# Changelog

All notable changes to this project are recorded here.

## [Unreleased]

## [0.7.0]

### Added

- Add progress-sensitive Watchdog continuation through
  `WatchdogDecision::ContinueImmediately`, claim- and context-scoped immediate
  ensure operations, and `WatchdogReconcileState::ScheduledImmediately` for
  the first actionable wake-up.

### Changed

- Hard-cut `reconcile_watchdog` from the generic `TimerReconcileState` to the
  policy-specific `WatchdogReconcileState`. Immediate demand replaces the one
  authoritative pre-armed successor at deadline now; cadence `Continue`, stop,
  cancellation, unregistration, generation ownership, and the two-message
  pre-arm protocol retain their existing meanings.
- Project an immediate successor through the existing snapshot fields as
  continuation mode with zero requested/armed delay. Equivalent or earlier
  deadlines and repeated/dispatched/running requests coalesce without another
  provider handle or snapshot format.

### Documentation

- Define immediate scheduling transitions, pending-command precedence,
  replicated rollback requirements, IcyDB integration calls, state-space
  change, and provider-call cost in a dedicated design record.

### Evidence

- Extend the pinned PocketIC Watchdog matrix with zero-delay initial scheduling
  and successful immediate continuation without cadence-time advancement. The
  complete eight-test trap, exhaustion, lifecycle, isolation, capacity, and
  cancellation matrix passes.
- Freeze native snapshot semantics when an immediate ensure coalesces with an
  overdue cadence wake-up: cadence scheduling mode and armed delay remain
  unchanged while the immediate request and coalescing counters advance.
- Record immediate-versus-cadence instruction/cycle observations and current
  compiler, optimized, and deterministic-gzip size cohorts. The controlled
  Watchdog final Wasm is 877 bytes (0.335%) above after-completion.

## [0.6.1] - 2026-08-15

### Changed

- Treat repository-only release impact as an advisory when the maintainer
  explicitly invokes a version-bump or release target. Empty release subjects
  still fail, and repository-only releases retain the complete validation
  gate.

### Documentation

- Clarify that callback instruction aggregates cover the accepted
  `ic-timers` execution interval, not the complete IC message: provider
  entry/exit, page reads, and the post-interval summary write remain outside.
- Require consumers that need a durable terminal audit receipt for a removed
  transient timer to own it outside the volatile registry; no tombstone or
  parallel authority is added.
- Freeze IcyDB's 0.6 performance-rebaseline and single-package qualification
  requirements without claiming that downstream adoption has occurred.

### Evidence

- Extend the PocketIC cohort with deterministic minimal and bounded
  representative Watchdog callbacks. Report scheduler/work instruction
  intervals and cycle deltas while marking complete-message instructions and
  their residual difference unavailable on PocketIC 15's public test surface.

## [0.6.0] - 2026-08-15

### Added

- Add `TimerInventorySnapshot` and `timer_inventory()` so one atomic bounded
  observation carries the runtime epoch even when the initialized registry has
  no timer declarations.

### Changed

- Define instruction observations as the accepted `ic-timers` execution
  interval, including acceptance, completion processing, and any successor
  binding rather than application code alone. Memory reads bracket that
  interval; provider entry/exit and the summary write remain outside its
  instruction delta.
- Document that terminal `RemoveWhenStopped` completion may remove its timer
  before the final measurement can be retained.

### Removed

- Remove the bare-vector `timer_snapshots()` inventory function. Consumers use
  `timer_inventory()` and read or consume its ordered timer vector; no alias or
  deprecated forwarding surface is retained.

### Documentation

- Refresh the README and compact handoff for the 0.6 API with a
  minimal registration example, scannable policy, ownership, observability,
  safety, development, and documentation tables, and clearer shared-registry
  guidance.
- Record IcyDB's completed exact-0.5.0 hard-cut adoption while preserving its
  tagged 0.3.4 and post-tag 0.3.8 evidence as historical subjects.
- Record Canic's completed exact-0.5.0/schema-3 adoption, including its memory
  projection and one-package graph, while preserving 0.3.8/schema 2 as
  historical evidence and keeping combined-framework qualification open.
- Add a public memory-summary projection example that distinguishes no
  completed sample from an observed zero-growth sample.

### Evidence

- Add a probe-only PocketIC cohort measurement for the start/end page reads
  excluded from callback instruction aggregates. PocketIC 15.0.0 reports an
  empty bracket and the four-read sampled bracket at 200 call-context
  instructions each, an observed delta of zero without changing the runtime
  API or measurement interval.
- Rerun the complete PocketIC watchdog matrix and policy cohorts against the
  hard-cut inventory API; recovery, isolation, capacity, ordering, and policy
  behavior remain green.

## [0.5.0] - 2026-08-15

### Changed

- Hard-cut the shared callback control capability into `OnceContext`,
  `AfterCompletionContext`, and `WatchdogContext`. Each callback now receives
  only the mutation operations legal for its declared policy, while exact
  callback-generation validation and expiry semantics remain unchanged.
- Give all callback contexts the same policy-shaped `ensure_scheduled` naming
  used by their registration capabilities. `OnceContext` accepts an explicit
  schedule, while after-completion and Watchdog contexts use their configured
  cadence.
- Keep nested request arbitration in the registry's existing pending-command
  state and callback generations. Remove request counters that were incremented
  but never read, compared, exposed, or used to select a winning command.
- Add allocation-free start/end Wasm-memory and stable-memory page-extent
  sampling for normally completed scheduler and work callbacks. Snapshots keep
  only the latest extent pair and maximum observed start-to-end growth for each
  role; they never total absolute page counts or fabricate samples for trapped
  or instruction-exhausted work.
- Route completed measurements from the callback token's canonical role rather
  than maintaining separate scheduler and work recording paths. An impossible
  policy/role pairing now fails as an ownership invariant instead of silently
  discarding the record.

### Removed

- Remove the public `TimerContext` type and its policy-probing `ensure_once`
  and `ensure_recurring` methods. No alias or deprecated forwarding surface is
  retained.
- Remove public `TimerError::WrongPolicy`; an invalid policy operation is no
  longer expressible through the callback API. Defensive internal policy
  mismatches fail as ownership invariants.
- Remove `TimerControlFailure::RequestSequenceExhausted` and its stable label.
  Generation, deadline, directive, and provider-binding failures remain.

## [0.4.1] - 2026-08-15

### Changed

- Share ordinary schedule resolution and retained lifecycle registration
  verification instead of maintaining policy-specific copies.
- Encode Once ensure, after-completion ensure, and authoritative reconciliation
  as one closed ordinary request kind, removing a boolean policy gate and its
  pass-through scheduling helper.
- Derive watchdog cancellation and transient-removal decisions from canonical
  post-transition state rather than an additional boolean result.
- Bind provider operations from one complete registry effect rather than
  passing duplicate token and delay arguments beside that effect.
- Validate provider-effect shape before cleanup or platform calls, including
  exact identity and claim-generation agreement between a Watchdog successor
  and its queued work.
- Centralize exact callback-claim ownership checks across dispatch,
  measurement, provider installation, and provider-handle consumption.
- Carry one closed initial/replacement arm kind from ordinary control through
  provider binding instead of splitting and rejoining duplicate arm variants.
  Clear effects likewise carry a non-empty callback selection, so "clear
  nothing" cannot be emitted and a Watchdog scheduler replacement is rejected
  before provider calls.
- Share the detach/transition/restore path used by cancellation and explicit
  unregistration, and remove compatibility-only annotations from private
  control errors.
- Correct public control documentation to distinguish armed wake-ups, pending
  successors, and running work that cancellation cannot interrupt.

### Fixed

- Remove a fresh `RemoveWhenStopped` declaration when it is cancelled before
  its first schedule. Once, after-completion, and Watchdog claims now expire
  consistently on cancellation regardless of whether they ever owned a
  provider handle.
- Reject malformed cross-registration Watchdog dispatch effects before any
  provider callback is armed or observability counter is confirmed.
- Prevent a late callback from an expired claim from consuming the provider
  handle of a replacement registration that reused the same identity, role,
  and callback generation.

## [0.4.0] - 2026-08-15

### Changed

- Correct the pre-1.0 release policy: hard cuts still remove superseded APIs
  without compatibility shims, but any public removal or incompatible public
  semantic change advances the minor compatibility line rather than a patch.
  The `TimerFuture` removal in 0.3.7 is retained and explicitly acknowledged
  as a SemVer mistake.
- Classify changes since the current version tag as crate-impacting,
  repository-only, or absent. Version bumps now reject repository-only
  publication before expensive validation, while `repository-check` provides
  the focused non-publishing validation path.
- Record that the current uncommitted Canic and IcyDB adoption worktrees both
  exact-pin 0.3.8. Development-time package alignment is complete; released
  combined composition still requires one-package qualification from tagged
  downstream subjects.
- Start the 0.4 hard-cut cleanup of the public facade and private runtime:
  remove superseded compatibility surface, consolidate duplicated paths, and
  keep only one canonical authority for each timer operation.
- Make the crate root the only public import path. The former public
  `schedule` and `snapshot` module paths are removed while their retained
  provider-neutral values remain available from `ic_timers::*`.
- Collapse identity validation into `TimerIdentity::try_new`, return identity
  components directly as `&str`, and report field-specific failures from one
  `TimerIdentityError` enum. The component bound is now named
  `MAX_TIMER_IDENTITY_COMPONENT_BYTES`.
- Keep observation construction registry-owned: observation fragments no
  longer implement `Default`, and policy/state helper projections that merely
  duplicated the canonical `TimerSnapshot` surface are private.
- Correct inactive-state documentation to distinguish retained callback
  authority from the absence of a scheduled or running callback generation.
- Consolidate exact-claim lifecycle verification, ordinary scheduled
  transitions, earliest/exact deadline mutation, provider-role slot selection,
  and lazy provider-handle detach/clear selection without adding another
  authority or compatibility layer.
- Mark private platform and detached provider capabilities `must_use` so new
  internal paths cannot silently ignore the obligation to bind, restore, or
  clear them.
- Consolidate watchdog terminal failure transitions so they clear pending
  commands consistently, and allocate the paired scheduler/work generations
  atomically before changing canonical state.
- Keep directive/policy mismatch internal to the registry instead of exposing
  it through the public scheduling-error vocabulary.
- Borrow exact registration claims during private unregistration and derive
  callback claims from one complete callback token, avoiding redundant
  identity allocation and independently supplied authority fields.

### Fixed

- Drain every detached watchdog provider handle before returning the first
  restoration failure. A failed wake-up restoration can no longer skip the
  remaining work-handle capability; focused fault injection covers the
  two-handle path.
- Avoid re-borrowing the registry when an exact detached provider capability
  is already available, eliminating eager alternate-handle evaluation.
- Restore all provider handles after any unexpected synchronous post-detach
  registry error. If restoration itself fails, retire the exact claim instead
  of dropping a linear capability or leaving false scheduled state.

### Removed

- Remove `TimerLabel`, `TimerLabelError`, `TimerIdentity::new`, and
  `MAX_TIMER_LABEL_BYTES`; the intermediate label abstraction duplicated the
  only supported identity constructor.
- Remove conversion from inert `TimerDirectiveSnapshot` observations back to
  executable `TimerDirective` commands.
- Remove duplicate `consecutive_expected_failures` forwarding methods from
  `TimerSnapshot` and `TimerObservabilitySnapshot`; the value remains on
  `TimerOutcomeSnapshot`, and the cheap identity-scoped runtime query remains
  public.
- Remove `TimerPolicy::cadence_ns`, `TimerCounters::provider_arms`, the public
  completion-partition checker, and public helpers on nested directive/runtime
  state. Consumers can use the typed cadence, separate committed arm counters,
  and coherent top-level snapshot projections directly.
- Remove the unreachable public `ScheduleError::MissingCadence` variant.
  Missing recurrence cadence is an illegal internal policy/directive pairing,
  not an error a consumer schedule request can produce.

## [0.3.8] - 2026-08-15

### Changed

- Record Canic's validated uncommitted exact-0.3.6 hard cut: one shared
  inventory now replaces its provider, registry/control state, timer counters,
  and timer-specific performance storage, while combined Canic+IcyDB
  qualification remains blocked on exact patch alignment.
- Clarify that Canic's removed global `TimerScheduled` counter was test-only,
  not a public parity surface; the maintained per-timer `schedules` field maps
  to committed `wakeups_armed`.
- Distinguish initializing an empty shared registry from declaring
  framework-owned jobs, allowing Fleet Coordinator to participate without
  inventing inactive Canic timers.
- Stop freezing Canic's maintained adoption status to one lifecycle state;
  release truth now requires only one structural status marker and does not
  interpret its prose.

## [0.3.7] - 2026-08-15

### Changed

- Make the crate hierarchy explicit: the crate root remains the convenience
  facade, `schedule` and `snapshot` remain public value groupings, and the
  control, registry, runtime implementation, and provider boundary remain
  private.
- Use defining-module imports internally, share ordinary callback erasure and
  duration validation, and document why atomic registry/runtime transitions
  remain cohesive.
- Correct lifetime, scheduling-mode, provider-arm, and work-count comments so
  observations do not overstate retained authority or delivery guarantees.
- Run full hosted Rust/MSRV validation once on pull requests and `main`; tag
  pushes now run only release-truth, exact-tag, and main-ancestry checks instead
  of duplicating the same builds at one commit.
- Add a pre-bump compact-status prose advisory for wording likely to become
  stale at release. It warns before expensive validation and always fails open;
  structural post-mutation release truth remains the only enforced boundary.

### Removed

- Hard-cut the accidental public `TimerFuture` erasure alias. Ordinary
  registration APIs continue to accept any compatible future; callback
  erasure is now entirely internal.

## [0.3.6] - 2026-08-14

### Changed

- Update the maintained IcyDB record: tagged IcyDB 0.226.1 adopted exact
  `ic-timers` 0.3.4, while its validated post-tag integration upgrades to
  0.3.5 and uses claim-scoped armed-wakeup observation without check-then-arm
  control flow.
- Keep release-truth validation structural: exact Cargo, changelog, release-note,
  and compact-status version markers remain enforced without interpreting
  free-form status prose or blocking a finalized version bump.

## [0.3.5] - 2026-08-14

### Added

- Add claim-scoped `has_armed_wakeup` observation to all three registration
  capabilities. The result reflects ownership of the exact future provider
  wake-up handle without making snapshots or observations into scheduling
  authority.
- Add a maintained IcyDB adoption record for its accepted 0.226.1 candidate,
  including the exact dependency, shared-registry inventory, recovery
  evidence, and measured costs.

## [0.3.4] - 2026-08-14

### Changed

- Hard-cut lifecycle reconciliation to retained declarations. The three
  reconciliation helpers no longer accept a declaration lifetime;
  `RemoveWhenStopped` callbacks remain available through direct registration.
- Make runner-executed release and provider-boundary checks portable without
  requiring `rg` on GitHub-hosted runners.

### Fixed

- Add policy-wide provider-binding fault evidence for initial and replacement
  arms, after-completion, partial watchdog binding, effect confirmation, and
  remove-on-stop cleanup.
- Finalize and validate mutable release truth mechanically, including the
  workspace status and version-specific release note, while keeping README and
  the Canic contract version-neutral.

## [0.3.3] - 2026-08-14

### Fixed

- Bind each consumer-work `TimerContext` to its exact callback generation and
  role. Context mutation remains available during nested work, while a context
  retained after completion expires and cannot control a successor or later
  registration.
- Fail closed when a public registration or lifecycle operation cannot bind
  its emitted provider effect. Newly armed and replaced handles are cleared,
  retained declarations become inactive with `ProviderBindingFailed`, and a
  later explicit ensure can recover without a false scheduled snapshot.

### Changed

- Reject explicit release versions that are equal to or lower than the
  workspace version, reject leading-zero SemVer components, and resolve tag
  collisions against the exact tag namespace.

## [0.3.2] - 2026-08-14

### Changed

- Correct the Canic adoption contract: all five dynamic built-ins are retained
  `Once` declarations, root canister-pool maintenance is retained
  `AfterCompletion`, public/lifecycle timers keep remove-on-stop lifetimes, and
  no Canic timer uses `Watchdog` yet.
- Permit one bounded Canic claim-custody collection for quiescence while
  forbidding it from duplicating scheduling state, counters, provider handles,
  generations, pending commands, or reconciliation authority.
- Freeze exact Canic metric projection: schedules use committed
  `wakeups_armed`, executions use `work_started`, successes combine successful
  and no-work completions, stale roles combine explicitly, latest delay uses
  the armed delay, and generation remains optional.
- Remove blanket dead-code suppression from the private platform and registry;
  pure registry fixture helpers are now compiled only for tests.

### Fixed

- Retain freshly reconciled inactive `Once`, `AfterCompletion`, and `Watchdog`
  declarations in the canonical inventory so fixed owners reserve bounded
  capacity before application hooks.
- Bring README, status, adoption, and dedicated release notes in line with the
  tagged and published 0.3.1 release.

## [0.3.1] - 2026-08-14

### Added

- Add exact `Once` and `AfterCompletion` schedule reconciliation, including
  the missing synchronous `reconcile_once` lifecycle helper, with focused
  deadline replacement and nested-request tests.
- Freeze a one-page Canic adapter contract covering identity and policy
  mapping, fallible facade changes, claim-scoped lifecycle composition,
  metrics projection, dependency unification, and the pre-1.0 hard cut.

### Changed

- Gate every release bump on normal CI, Rust 1.88, all supported probe lint
  configurations, the full watchdog PocketIC suite, and comparable policy
  cohorts; fail immediately and before version mutation unless PocketIC is the
  exact audited 15.0.0 binary.
- Automatically install the pinned PocketIC artifact into the ignored
  repository tool cache when no override is supplied, while retaining strict
  version/hash validation and never replacing an explicit override.
- Update and stage the nested `testing/Cargo.lock` during version bumps, then
  perform cheap locked-metadata checks for both workspaces without rerunning
  the already-completed behavioral evidence suite.
- Enforce in normal CI that `ic-cdk-timers` remains used only inside the
  private `platform` module and is wrapped rather than publicly re-exported.
- Treat every superseding pre-1.0 contract change as a hard cut by default,
  without deprecated, dual, fallback, or feature-gated legacy paths.
- Mark all three sole-owner registration capabilities `must_use` so accidental
  loss is diagnosed at compile time.
- Make the registry the sole owner of pending ordinary commands and scheduling
  metadata; private `TimerControl` now receives the registry's already
  arbitrated completion decision instead of maintaining a parallel pending
  machine.
- Explicitly keep Canic log-retention and cycle-top-up work on the ordinary
  after-completion path, reserve `Watchdog` for pre-armed recovery work, and
  reject a shared global suspend switch in favor of owner-specific claims.

### Fixed

- Trap callback messages on unexpected internal completion, accounting, or
  cleanup failures instead of silently discarding them; watchdog work thereby
  relies on message rollback and retains its earlier committed successor.
- Drive IcyDB-shaped retryable and terminal results through real scheduler and
  work callbacks, verifying continuation, successor ownership, truthful
  counters, and terminal clearing.
- Describe the published runtime as 0.3 rather than "unreleased 0.3" in crate
  documentation.

## [0.3.0] - 2026-08-13

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
