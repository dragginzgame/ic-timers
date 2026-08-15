# Architecture

## Current runtime

The crate root is the only public facade. It re-exports runtime operations and
provider-neutral values directly; every implementation and value-grouping
module is private. In particular, neither `platform`, `registry`, `schedule`,
nor `snapshot` is a consumer import path.

The module hierarchy keeps six responsibilities separate:

1. `schedule` owns validated cadence, requested schedules, post-run
   directives, and checked nanosecond/deadline conversion.
2. `snapshot` owns inert public identity and observation values. Its private
   `identity`, `model`, and `metrics` children separate validation, closed
   state/outcome types, and saturating measurements. Only the registry builds
   the top-level timer and atomic inventory snapshots or initializes their
   observation fragments; nested values expose no alternate construction or
   control path.
3. `control` is the private ordinary generation/registration state machine. It
   owns checked callback generations, immediate schedule, reconciliation and
   cancellation transitions, and stale completion rejection. It owns no
   pending command.
4. `registry` is the provider-call-free fixed-capacity canonical owner for
   structured identities, callback closures, claim generations,
   policy-specific state, the sole pending ordinary-command machine, nested
   arbitration, deterministic snapshots, and provider-neutral effects. It also
   owns every bound provider handle; only `runtime` invokes `platform` to
   create or clear those handles.
5. `platform` is the private direct boundary to `ic-cdk-timers` and required
   IC system facts. Its handle is linear and it owns no recurrence policy.
6. `runtime` owns the one canister-local registry static, erases consumer
   callbacks for registry storage, exposes registration claims, applies
   provider effects, and drives live `Once`/`AfterCompletion` dispatch and the
   two-role watchdog protocol. Its three policy-specific delegated work
   contexts wrap one private mechanism and are valid only for the exact
   running callback token.
   Claim-originated effect failures retire the declaration instead of leaving
   registry state scheduled without a provider handle. Lifecycle verification
   checks the exact claim and immutable declaration metadata directly rather
   than reconstructing authority from a snapshot. Synchronous operations that
   detach handles restore all of them before returning an unexpected registry
   error, retiring the exact claim if restoration cannot recover ownership.
   Each provider binding consumes one complete registry effect; its shape is
   validated before cleanup or platform calls rather than duplicating token
   and delay arguments beside the canonical effect. One initial/replacement
   arm kind flows from ordinary control through provider binding, and the
   non-empty set of callbacks to clear is also a closed value rather than
   independent booleans. The entry-local exact-claim predicate is shared by
   callback acceptance, measurements, provider installation, and handle
   consumption, so identity reuse cannot transfer handle authority to a stale
   callback.

The registry and runtime transition functions remain cohesive even where they
are long: each audited function owns one atomic transition or one provider
binding path. Their tests live in directory-local `tests.rs` files so test
volume does not obscure production flow.

The copied Canic code was adapted into generic library types; Canic-specific
domain work, storage, and metrics were intentionally not copied.

## Canonical runtime

One canister-local runtime owns the bounded registry and its structured
identity (`owner`, `subsystem`, `name`). Framework and application schedulers
must contribute to that same live registry rather than building private
inventories. All three policies, lifecycle reconstruction, measurements, and
the real-canister watchdog evidence matrix are implemented.

The runtime provides:

- one-shot, after-completion interval, and pre-armed watchdog policies;
- serial execution with overdue work coalesced into one pending run;
- cancellation requested safely by the running callback;
- synchronous lifecycle restoration followed by deferred application work;
- runtime-start counters, last outcomes, deadlines, instruction consumption,
  and bounded memory-page extent/growth observations;
- exact-claim observation of armed provider wake-up ownership without a
  snapshot-derived control path;
- bounded identity components and allocation-conscious hot paths;
- one atomic inventory snapshot carrying the runtime epoch even when empty;
  and
- portable snapshot DTOs so Canic, IcyDB, and standalone canisters can
  expose the same operator view.

Metrics are collected once at the registry boundary. Consumers may adapt
the atomic inventory to their own status endpoint or metrics encoder without
wrapping every callback independently.

The canonical snapshot is designed as a semantic superset of Canic's current
timer status, scheduling counters, and instruction metrics. In particular,
callback starts and completions remain separate because traps and instruction
exhaustion can prevent post-run measurement, and consecutive expected failures
remain cheap functional state because recovery decisions consume them. See the
[observability and Canic parity contract](design/observability.md).

## Frozen 0.3 contract

Patch 1 freezes the registry, callback, ownership, state, counter, lifecycle,
provider, MSRV, and measurement decisions before runtime mutation. The
canonical registry holds at most 64 declarations. Ordinary work may be async;
watchdog work is synchronous so its separate work message cannot cross an
`await`. Provider handles are private, linear capabilities owned only by the
registry. See the [Patch 1 runtime contract](design/0.3-patch-1-contract.md).

The live snapshot omits elapsed IC time: message time cannot truthfully measure
synchronous callback duration. It instead distinguishes scheduler and work
instructions, plus latest start/end and maximum-growth Wasm/stable memory-page
observations. Page extent is not exact allocator liveness. A dispatched
watchdog attempt without a committed completion is *unacknowledged*, not
definitely trapped, and has no fabricated measurement.

## Implementation sequence

Keep the runtime work in independently testable layers:

1. **Complete:** freeze downstream invariants, API/state decisions, provider
   behavior, Rust 1.88 feasibility, and measurement subjects.
2. **Complete:** add the coherent value model and pure bounded policy-specific
   registry.
3. **Complete:** bind linear platform handles and implement live `Once` and
   `AfterCompletion` policies.
4. **Complete:** implement the one-shot two-message `Watchdog` protocol and
   prove its initial explicit-trap commit boundary in PocketIC.
5. **Complete:** add lifecycle reconstruction, live observations, measurement,
   and the IcyDB-shaped fixture.
6. **Complete:** close the PocketIC recovery/isolation matrix and report Wasm,
   instruction, cycle, MSRV, and complexity results.

Each layer lands with its owner-local correctness tests. Cross-owner
real-canister evidence closes the final patch rather than substituting for
pure transition coverage.

Lifecycle reconciliation always installs retained declarations so a
caller-owned `Option<Registration>` cannot outlive a remove-on-stop canonical
entry. Transient `RemoveWhenStopped` timers use direct registration and are
recreated explicitly by their owner if later desired; cancellation removes a
transient declaration even before its first schedule. One private lifecycle
seam installs and verifies the exact retained claim for all three policies.

## Safety boundary

An after-completion interval is not a watchdog: if its callback traps or runs
out of instructions, code after the callback cannot arm a successor. A
watchdog policy commits the successor before invoking fallible work while
preventing concurrent logical execution. PocketIC 15 proves that behavior for
trap, 40-billion-instruction exhaustion, upgrade reconstruction,
external-ingress rejection, independent timers, insufficient cycles followed
by top-up, stop/resume, overdue coalescing, and cancellation in the
scheduler/work gap. These proofs apply to synchronous `Watchdog`, not
after-completion recurrence.

## Consumer integration

- Canic uses exact `ic-timers` 0.5.0 for framework and application timers and
  exposes schema-3 timer, instruction, and memory observations. Canisters
  initialize the shared registry independently of declaring jobs: Fleet
  Coordinator initializes an empty registry, while genuine owners reserve
  their fixed declarations before application hooks.
- IcyDB uses exact `ic-timers` 0.5.0 and the watchdog policy for replicated
  recovery driving with claim-scoped armed-wakeup observation.
- A canister using both should see one inventory. Ownership labels distinguish
  scheduling clients; they do not create separate timer runtimes.

The combined downstream gate must prove one resolved `ic-timers` package ID,
both owners in one inventory, synchronous lifecycle reconstruction, IcyDB
Watchdog recovery, continued Canic timer progress, and no remaining direct
`ic-cdk-timers` calls across the complete canister. That evidence is currently
blocked on Canic's lifecycle-composition seam, not the timer scheduler.
The provider's 250 outstanding-dispatch limit is canister-wide; the registry's
128-handle maximum bounds only handles owned by this crate. Canic must also
retain the validated hard cut that makes its application timer facade return
typed capacity and identity errors.

The exact implemented hard-cut mapping, including authoritative deadline
reconciliation, policy choices, and the intentional absence of global
suspension, is maintained in the [Canic adapter contract](adoption/canic.md).

Canic may keep one bounded collection whose only values are opaque registration
claims. That custody makes Canic-owned application timers enumerable for its
authority-snapshot fence without duplicating registry state. Scheduling,
deadlines, generations, counters, pending commands, provider handles, and
reconciliation authority remain exclusively in `ic-timers`.

Consumer callbacks may issue nested control through `OnceContext`,
`AfterCompletionContext`, or `WatchdogContext`. Each type exposes only the
operations legal for its policy, while the shared private delegation is
checked against the exact running generation and work role. A stored context
expires at callback completion and cannot become another entry in a consumer
custody collection.

IcyDB's exact dependency, removed parallel timer state, and downstream
real-canister evidence are recorded in the
[IcyDB adoption record](adoption/icydb.md). The
[Canic adapter contract](adoption/canic.md) records its exact-0.5.0/schema-3
adoption. Both owners independently resolve 0.5.0; combined qualification
remains open until Canic's lifecycle-composition seam can host one final
single-registry Wasm subject.
