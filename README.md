# ic-timers

`ic-timers` is a higher-level wrapper around
[`ic-cdk-timers`](https://crates.io/crates/ic-cdk-timers) for Internet Computer
canisters. It does not replace the CDK timer provider: `ic-cdk-timers` still
arms and clears the platform timers. This crate is intended to add one place
for timer identity, scheduling policy, execution arbitration, observability,
and lifecycle recovery.

The current release contains the complete bounded runtime, PocketIC
recovery-watchdog evidence, and post-0.3 hardening. Tagged IcyDB 0.226.1
hard-cuts to exact `ic-timers` 0.3.4, and its validated post-tag integration
upgrades to exact 0.3.5; Canic has not adopted the runtime yet.

## Why wrap `ic-cdk-timers`?

`ic-cdk-timers` provides the low-level mechanism a canister needs to schedule
callbacks. That is the right boundary for a simple timer. The abstraction gets
harder to operate when a framework, a database, and application code all
schedule recurring work independently.

Direct, scattered use gives each subsystem its own private answers to
questions such as:

- Which logical timers exist in this canister, and who owns them?
- Is a callback scheduled, running, overdue, cancelled, or stale?
- When did it last run, what happened, and how expensive was it?
- What should happen after an upgrade or a trapped callback?
- Does “recurring” mean after-completion scheduling or a recovery watchdog?

We thought a wrapper was worthwhile because those are canister-wide concerns.
If every consumer builds its own registry, recurrence loop, metrics, and
upgrade restoration, operators still cannot obtain one reliable inventory and
the most failure-sensitive logic is duplicated. `ic-timers` is intended to put
that coordination above the proven CDK provider while keeping the provider
dependency behind a small platform boundary.

The wrapper deliberately uses one-shot provider timers. A higher layer can
then decide when a successor becomes authoritative: after successful work for
ordinary recurrence, or before fallible work for a recovery watchdog. Those
policies have different failure guarantees and should not be hidden behind the
same interval helper.

`ic-cdk-timers` is an implementation dependency, not part of this crate's
public API. Repository CI enforces that direct provider references remain in
the private `platform` module and that neither the provider nor that module is
re-exported.

## What exists today

The current crate contains:

- one volatile, canister-local 64-entry registry with unique structured
  identity ownership and claim generations;
- synchronous, idempotent runtime initialization plus callback-owning `Once`,
  `AfterCompletion`, and synchronous `Watchdog` registrations;
- one exact private provider handle per scheduled ordinary timer and at most
  two per watchdog (cadence successor plus dispatched work), including real
  replacement and cancellation through `ic-cdk-timers`;
- live policy-specific state transitions, including stale-callback and nested
  ensure/cancel arbitration;
- work-scoped `TimerContext` delegation whose mutation authority expires when
  the exact callback generation finishes;
- fail-closed provider-effect binding for public control calls, preventing a
  failed arm or replacement from leaving a declaration falsely scheduled;
- claim-scoped `has_armed_wakeup` observation on every registration
  capability, reflecting exact future provider-handle ownership without
  turning snapshots into control authority;
- validated positive cadence, typed directives, and checked deadline
  calculation;
- live, inert policy-specific snapshots, with split scheduler/work counters,
  truthful unacknowledged dispatches, functional expected-failure state, and
  normally completed scheduler/work instruction aggregates;
- synchronous idempotent reconciliation helpers that always retain fixed
  lifecycle declarations, whose caller-owned volatile registration slot
  prevents duplicate callback replacement, and whose desired state remains
  derived from consumer durable authority;
- exact ordinary reconciliation that can replace an earlier or later deadline,
  plus a `reconcile_once` lifecycle helper;
- a private, linear one-shot provider boundary over `ic-cdk-timers` 1.0.0.

The watchdog scheduler arms its successor and queues a separate zero-delay work
callback before returning. PocketIC 15 evidence on Rust 1.88 covers explicit
trap, actual 40-billion-instruction exhaustion, insufficient cycles followed
by top-up, upgrade reconstruction before a downstream-hook observation,
stop/resume, overdue coalescing, terminal and scheduler/work-gap cancellation,
duplicate demand, two simultaneous timers, trap isolation, rejection of
external executor ingress, and subsequent progress. Trapped or exhausted work
contributes no fabricated completion or instruction sample.

That recovery guarantee applies to the later consumer-work message. It does
not claim recovery if the small scheduler message itself traps or exhausts its
instructions; the scheduler is deliberately fixed, bounded, and contains no
consumer work.

These guarantees apply only to `Watchdog`. Ordinary after-completion recurrence
arms its successor after normal return and therefore cannot survive a trap or
instruction exhaustion in consumer work. The remaining release work is
downstream adapter and adoption feedback, not another timer runtime. See
[the architecture note](docs/architecture.md) for the intended boundary and
implementation order, the frozen
[0.3 Patch 1 contract](docs/design/0.3-patch-1-contract.md) for the decisions
that preceded implementation, the implemented
[observability contract](docs/design/observability.md), and the
[0.3 evidence report](docs/audits/0.3-runtime-evidence-2026-08-13.md).
[The safety boundary](SAFETY.md) defines the guarantees and their limits.

## Policies

| Policy | Work | Successor timing | Failure boundary |
| --- | --- | --- | --- |
| `Once` | asynchronous | only when explicitly requested | no automatic recovery after a trap |
| `AfterCompletion` | asynchronous | after normal callback completion | no automatic recovery after a trap |
| `Watchdog` | synchronous | committed by a separate scheduler message before work | successor survives trapped or exhausted consumer work |

## Intended use

Canic and IcyDB motivated the shared wrapper. Canic needs framework timers and
lifecycle integration; IcyDB needs a recovery watchdog; an application may add
more timers of its own. All of them should eventually declare timers into one
canister-local registry so an operator can answer “what timers exist in this
canister?” from one snapshot.

For a canister with one simple callback and no need for shared inventory,
metrics, or recovery policy, using `ic-cdk-timers` directly remains the simpler
choice.

The lifecycle owner calls `initialize_runtime` synchronously before any
registration, then invokes each consumer's reconciliation during `init` and
`post_upgrade` before downstream hooks. Consumers persist their own desired
state; `ic-timers` persists no policy, handle, generation, epoch, or application
authority.

Lifecycle reconciliation is intentionally retained-only so an inactive fixed
owner remains observable and keeps its capacity reservation. Transient
`RemoveWhenStopped` callbacks use the direct registration functions and are
registered again by their owner if later desired.

All consumers must resolve to the same `ic-timers` Cargo package ID. Two
resolved versions contain two independent library statics and do not share a
registry. The crate exports no lifecycle hook, macro, Candid endpoint, or
consumer-work executor. The pinned provider's internal executor export rejects
non-self callers, which the PocketIC suite verifies.

The 64-entry registry and maximum 128 owned handles do not reserve capacity in
the provider's canister-wide 250 outstanding-dispatch limit. An adoption must
also inventory or migrate every remaining direct `ic-cdk-timers` user in the
final canister and prove one resolved `ic-timers` package ID.

Canic has not adopted the crate. Its proposed hard-cut mapping is recorded in
the [Canic adapter contract](docs/adoption/canic.md); notably, Canic composes
claim-specific cancellation and domain reconciliation instead of gaining a
global switch that could suspend other owners in the shared registry.

Tagged IcyDB 0.226.1 uses the shared registry and its pre-armed watchdog. A
validated post-tag integration upgrades to `has_armed_wakeup()` on exact
0.3.5. The dependencies, removed parallel state, downstream PocketIC evidence,
measurements, and remaining landing boundary are recorded in the
[IcyDB adoption record](docs/adoption/icydb.md).

## Development

```text
make update-dev
make ci
```

`make update-dev` installs the pinned Rust toolchain, Clippy, rustfmt, the Wasm
target, and this repository's single formatting hook. Normal development uses
Rust 1.97.1; `make msrv` checks the declared Rust 1.88.0 minimum separately.
`make help` lists the smaller component targets.

The focused real-canister evidence is intentionally separate from the normal
CI gate. The first run automatically downloads the exact audited PocketIC
15.0.0 Linux x86_64 binary into the ignored `target/tools` cache, then every
run verifies its reported version and SHA-256:

```text
make pocketic-watchdog
make pocketic-cohorts
```

Set `POCKET_IC_BIN=/path/to/pocket-ic` only to use an explicitly managed
binary; overrides are validated strictly and are never replaced automatically.

## License

MIT
