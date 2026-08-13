# ic-timers

`ic-timers` is an early workspace for observable, recovery-aware timer
orchestration on Internet Computer canisters.

The repository currently contains a compiling foundation extracted from
Canic's timer implementation:

- deterministic generation, cancellation, reconciliation, and stale-callback
  arbitration;
- typed scheduling directives and overflow-safe deadline calculation; and
- a deliberately thin one-shot boundary over `ic-cdk-timers` 1.0.0.

This is a pre-alpha scaffold, not a production timer runtime. In particular,
pre-armed watchdog recurrence, lifecycle reconstruction, a shared canister-wide
inventory, metrics export, and PocketIC failure evidence are not implemented
yet. See [the architecture note](docs/architecture.md) for the intended
boundary.

## Development

```text
make update-dev
make ci
```

`make update-dev` installs the pinned Rust toolchain, Clippy, rustfmt, the Wasm
target, and this repository's single formatting hook. Normal development uses
Rust 1.97.1; `make msrv` checks the declared Rust 1.91.0 minimum separately.
`make help` lists the smaller component targets.

## Proposed use

Once the runtime API is complete, both Canic and IcyDB should depend on this
crate and declare timers into one canister-local registry. Canic can contribute
framework timers and lifecycle integration; IcyDB can contribute its recovery
watchdog. The registry—not either consumer—should own scheduling, snapshots,
and measurements so operators can answer “what timers exist in this canister?”
from one surface.

## License

MIT
