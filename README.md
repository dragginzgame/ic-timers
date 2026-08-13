# ic-timers

`ic-timers` is a higher-level wrapper around
[`ic-cdk-timers`](https://crates.io/crates/ic-cdk-timers) for Internet Computer
canisters. It does not replace the CDK timer provider: `ic-cdk-timers` still
arms and clears the platform timers. This crate is intended to add one place
for timer identity, scheduling policy, execution arbitration, observability,
and lifecycle recovery.

This repository is currently a pre-alpha foundation, not a production timer
runtime.

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

## What exists today

The current crate contains a compiling foundation extracted from Canic's timer
implementation:

- deterministic generation, cancellation, reconciliation, and stale-callback
  arbitration;
- typed scheduling directives and overflow-safe deadline calculation; and
- a deliberately thin one-shot boundary over `ic-cdk-timers` 1.0.0; and
- candidate 0.2 provider-neutral identity, policy, state, outcome, counter,
  measurement, epoch, and canonical snapshot value types.

The shared canister-wide registry, live snapshot population, measured
execution, lifecycle reconstruction, pre-armed watchdog recurrence, metrics
adapters, and PocketIC recovery evidence are not implemented yet.
Recovery-critical consumers should continue using their proven timer
implementation until those guarantees exist. See
[the architecture note](docs/architecture.md) for the intended boundary and
implementation order, and the proposed
[observability contract](docs/design/observability.md) for the 0.2 snapshot and
Canic metrics-parity requirements. [The safety boundary](SAFETY.md) lists the
guarantees that are and are not currently backed by implementation evidence.

## Intended use

Canic and IcyDB motivated the shared wrapper. Canic needs framework timers and
lifecycle integration; IcyDB needs a recovery watchdog; an application may add
more timers of its own. All of them should eventually declare timers into one
canister-local registry so an operator can answer “what timers exist in this
canister?” from one snapshot.

For a canister with one simple callback and no need for shared inventory,
metrics, or recovery policy, using `ic-cdk-timers` directly remains the simpler
choice.

## Development

```text
make update-dev
make ci
```

`make update-dev` installs the pinned Rust toolchain, Clippy, rustfmt, the Wasm
target, and this repository's single formatting hook. Normal development uses
Rust 1.97.1; `make msrv` checks the declared Rust 1.91.0 minimum separately.
`make help` lists the smaller component targets.

## License

MIT
