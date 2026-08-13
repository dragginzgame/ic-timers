# Architecture

## Current foundation

The initial crate intentionally keeps two responsibilities separate:

1. `control` is a pure state machine for one logical timer identity. It owns
   generations, request ordering, cancellation while running, reconciliation,
   and stale-completion rejection.
2. `platform` is the only direct boundary to `ic-cdk-timers`. It exposes
   one-shot arm and clear operations and owns no recurrence policy.

`schedule` contains typed post-run directives and checked conversion from a
delay to an absolute nanosecond deadline.

The copied Canic code was adapted into generic library types; Canic-specific
domain work, storage, and metrics were intentionally not copied.

## Intended runtime

The next implementation should add one canister-local registry with structured
identity (`owner`, `subsystem`, `name`) and a complete snapshot for every
declared timer. Framework and application schedulers must contribute to this
same registry rather than building private inventories.

The runtime should eventually provide:

- one-shot, after-completion interval, and pre-armed watchdog policies;
- serial execution with overdue work coalesced into one pending run;
- cancellation requested safely by the running callback;
- synchronous lifecycle restoration followed by deferred application work;
- runtime-start counters, last outcomes, deadlines, instruction consumption,
  and callback duration;
- bounded labels and allocation-conscious hot paths; and
- portable snapshot DTOs so Canic, IcyDB, and standalone canisters can expose
  the same operator view.

Metrics should be collected once at the registry boundary. Consumers may adapt
the snapshot to their own status endpoint or metrics encoder without wrapping
every callback independently.

The canonical snapshot must be a semantic superset of Canic's current timer
status, scheduling counters, and instruction metrics before runtime
instrumentation is implemented. In particular, callback starts and completions
remain separate because traps and instruction exhaustion can prevent post-run
measurement, and consecutive expected failures remain cheap functional state
because recovery decisions consume them. See the
[observability and Canic parity contract](design/observability.md).

## Implementation sequence

Keep the runtime work in independently testable layers:

1. Review the 0.2 observability contract, then define bounded structured
   identity, scheduling-policy, execution-state, outcome, counter,
   measurement, scope, and snapshot value types. Settle their semantics,
   ordering, and portable shape before storing them.
2. Add a pure serial registry above `TimerControl`. It should reject duplicate
   identities, own deterministic snapshot ordering, and translate registry
   commands into platform-neutral effects.
3. Connect those effects to `platform`, then add measured callback execution
   and the one-shot and after-completion policies.
4. Add synchronous lifecycle reconstruction and participant composition.
5. Implement pre-armed watchdog recurrence only together with the PocketIC
   recovery and isolation evidence described below.

The first slice should stop after value types and their unit tests. In
particular, it should not choose global storage or callback ownership before
the public snapshot contract is reviewable.

## Safety boundary

An after-completion interval is not a watchdog: if its callback traps or runs
out of instructions, code after the callback cannot arm a successor. A
watchdog policy must commit the successor before invoking fallible work while
still preventing concurrent logical execution. That behavior requires
PocketIC evidence for trap, instruction exhaustion, upgrade reconstruction,
external-ingress rejection, independent timers, and insufficient cycles.

Until those tests exist, consumers with recovery-critical timers should retain
their proven provider rather than adopting this scaffold.

## Consumer integration

- Canic should use the crate for framework timers and register its synchronous
  post-restore lifecycle participant before deferred user hooks.
- IcyDB should use the watchdog policy for replicated recovery driving once the
  pre-arm and reconstruction guarantees are proven.
- A canister using both should see one inventory. Ownership labels distinguish
  scheduling clients; they do not create separate timer runtimes.
