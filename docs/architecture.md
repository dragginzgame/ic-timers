# Architecture

## Current runtime

The crate root is the public convenience facade. It re-exports the runtime
operations and the provider-neutral values that also remain grouped under the
public `schedule` and `snapshot` modules. Implementation modules are private;
in particular, neither `platform` nor `registry` is a consumer API.

The module hierarchy keeps six responsibilities separate:

1. `schedule` owns validated cadence, requested schedules, post-run
   directives, and checked nanosecond/deadline conversion.
2. `snapshot` owns inert public identity and observation values. Its private
   `identity`, `model`, and `metrics` children separate validation, closed
   state/outcome types, and saturating measurements.
3. `control` is the private ordinary generation/registration state machine. It
   owns checked generations and request sequences, immediate schedule,
   reconciliation and cancellation transitions, and stale completion
   rejection. It owns no pending command.
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
   two-role watchdog protocol. Its
   delegated work context is valid only for the exact running callback token.
   Claim-originated effect failures retire the declaration instead of leaving
   registry state scheduled without a provider handle.

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
- runtime-start counters, last outcomes, deadlines, and instruction
  consumption;
- exact-claim observation of armed provider wake-up ownership without a
  snapshot-derived control path;
- bounded labels and allocation-conscious hot paths; and
- portable snapshot DTOs so Canic, IcyDB, and standalone canisters can expose
  the same operator view.

Metrics are collected once at the registry boundary. Consumers may adapt
the snapshot to their own status endpoint or metrics encoder without wrapping
every callback independently.

The canonical snapshot is designed as a semantic superset of Canic's current
timer status, scheduling counters, and instruction metrics. In particular,
callback starts and completions
remain separate because traps and instruction exhaustion can prevent post-run
measurement, and consecutive expected failures remain cheap functional state
because recovery decisions consume them. See the
[observability and Canic parity contract](design/observability.md).

## Frozen 0.3 contract

Patch 1 freezes the registry, callback, ownership, state, counter, lifecycle,
provider, MSRV, and measurement decisions before runtime mutation. The
canonical registry holds at most 64 declarations. Ordinary work may be async;
watchdog work is synchronous so its separate work message cannot cross an
`await`. Provider handles are private, linear capabilities owned only by the
registry. See the [Patch 1 runtime contract](design/0.3-patch-1-contract.md).

The live snapshot omits elapsed IC time: message time cannot truthfully measure
synchronous callback duration. It instead distinguishes scheduler
and work instructions, and it will call a dispatched watchdog attempt without
a committed completion *unacknowledged*, not definitely trapped.

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
recreated explicitly by their owner if later desired.

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

- Canic should use the crate for framework timers and register its synchronous
  post-restore lifecycle participant before deferred user hooks.
- Tagged IcyDB 0.226.1 uses the watchdog policy for replicated recovery
  driving; its validated post-tag integration uses claim-scoped armed-wakeup
  observation on exact `ic-timers` 0.3.5.
- A canister using both should see one inventory. Ownership labels distinguish
  scheduling clients; they do not create separate timer runtimes.

The downstream gate must prove one resolved `ic-timers` package ID and
inventory remaining direct `ic-cdk-timers` calls across the complete canister.
The provider's 250 outstanding-dispatch limit is canister-wide; the registry's
128-handle maximum bounds only handles owned by this crate. Canic must also
make its currently infallible application timer facade return typed capacity
and identity errors before adopting the shared registry.

The exact hard-cut mapping, including authoritative deadline reconciliation,
policy choices, and the intentional absence of global suspension, is frozen in
the [Canic adapter contract](adoption/canic.md).

Canic may keep one bounded collection whose only values are opaque registration
claims. That custody makes Canic-owned application timers enumerable for its
authority-snapshot fence without duplicating registry state. Scheduling,
deadlines, generations, counters, pending commands, provider handles, and
reconciliation authority remain exclusively in `ic-timers`.

Consumer callbacks may issue nested control through `TimerContext`, but that
delegation is checked against the exact running generation and work role. A
stored context expires at callback completion and cannot become another entry
in a consumer custody collection.

IcyDB's exact dependency, removed parallel timer state, and downstream
real-canister evidence are recorded in the
[IcyDB adoption record](adoption/icydb.md). Canic adoption remains pending
under the separate adapter contract. A combined application still needs to
prove that both consumers resolve this same package instance.
