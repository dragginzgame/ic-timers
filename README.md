# ⏱️ ic-timers

> A bounded, observable timer runtime for Internet Computer canisters, built on
> top of [`ic-cdk-timers`](https://crates.io/crates/ic-cdk-timers).

`ic-timers` wraps the CDK timer provider; it does not replace it.
`ic-cdk-timers` still arms and clears platform timers, while `ic-timers` adds
logical identity, scheduling policy, callback arbitration, lifecycle
reconstruction, and one coherent operational snapshot.

The provider is a private implementation dependency. It is kept behind the
crate's `platform` module and is never re-exported.

## 🌟 At a glance

| | Current contract |
| --- | --- |
| 🧩 API line | `0.6` atomic-inventory hard cut |
| 🦀 Rust | Edition 2024; MSRV 1.88.0 |
| ⚙️ Provider | Exact `ic-cdk-timers` 1.0.0, private and wrapped |
| 🗂️ Capacity | 64 logical timers; at most 128 owned provider handles |
| 🔄 Policies | `Once`, `AfterCompletion`, and pre-armed `Watchdog` |
| 💾 Persistence | None; consumers retain durable application authority |
| 🔎 Observation | Bounded snapshots, counters, instructions, and memory-page extents |

## 💡 Why wrap `ic-cdk-timers`?

`ic-cdk-timers` is the right low-level mechanism for scheduling a simple
callback. The operational problem changes when a framework, a database, and
application code all schedule work independently.

| Concern | Scattered direct timers | `ic-timers` |
| --- | --- | --- |
| Identity | Private names and handles | Structured `owner / subsystem / name` identity |
| Ownership | Distributed across subsystems | One bounded canister-local registry |
| Recurrence | Usually an interval or local loop | Explicit policy with a documented failure boundary |
| Upgrades | Each owner invents restoration | Synchronous, idempotent lifecycle reconciliation |
| Cancellation | Provider handle knowledge leaks outward | Claim-scoped cancellation with exact handle ownership |
| Metrics | Parallel counters and partial inventories | One inert, policy-specific snapshot |
| Stale callbacks | Consumer-specific handling | Generation-checked harmless no-ops |

We thought this wrapper was worthwhile because these are canister-wide
questions:

- Which logical timers exist, and who owns each one?
- Is a timer inactive, armed, running, overdue, cancelled, or stale?
- What happened on its last run, and what did that run cost?
- What should be reconstructed after an upgrade?
- Must recurrence wait for normal completion, or survive failed work?

If every consumer builds a registry, recurrence loop, metrics table, and
upgrade protocol, operators still lack one reliable inventory and the most
failure-sensitive logic is duplicated. `ic-timers` puts that coordination
above the CDK provider without creating another provider.

## 🧩 Choose the policy that matches the failure boundary

| Policy | Consumer work | Successor timing | Trap or exhaustion behavior |
| --- | --- | --- | --- |
| `Once` | Asynchronous | Only when explicitly requested | No automatic recovery |
| `AfterCompletion` | Asynchronous | Armed after normal work completion | No automatic recovery |
| `Watchdog` | Synchronous and bounded | Committed by a scheduler message before a separate work message | The committed successor survives failed consumer work |

`Watchdog` deliberately uses two messages. The small scheduler callback
validates its generation, arms the next cadence successor, queues immediate
work, and returns. Only the later work callback invokes consumer code.

> ⚠️ The recovery guarantee covers the consumer-work message. It does not claim
> recovery if the fixed scheduler message itself traps or exhausts its
> instructions.

Ordinary recurrence is cheaper and is the correct default when work must
finish normally before another invocation is allowed. Use `Watchdog` only
when committing the next wake-up before fallible synchronous work is the
required protocol.

## 🚀 Minimal `Once` example

Add one exact package version when this crate participates in a shared
framework/application registry:

```toml
[dependencies]
ic-timers = "=0.6.0"
```

Initialize the runtime from the canister's existing lifecycle owner, declare a
timer, retain its non-clone registration capability, and schedule it:

```rust
use ic_timers::{
    DeclarationLifetime, OnceRegistration, TimerCompletion, TimerDirective,
    TimerIdentity, TimerRunResult, TimerSchedule, initialize_runtime,
    register_once,
};
use std::{error::Error, time::Duration};

fn declare_cleanup_timer() -> Result<OnceRegistration, Box<dyn Error>> {
    initialize_runtime()?;

    let timer = register_once(
        TimerIdentity::try_new("my-canister", "maintenance", "cleanup")?,
        DeclarationLifetime::Retained,
        |_context| async {
            // Perform one bounded unit of application work.
            TimerRunResult::new(TimerCompletion::success(1), TimerDirective::Stop)
        },
    )?;

    timer.ensure_scheduled(TimerSchedule::After(Duration::from_secs(30)))?;
    Ok(timer)
}
```

The returned registration is the sole-owner control capability. Keep it in
volatile owner state so that code can inspect, ensure, reconcile, cancel, or
unregister that exact claim. The capability types are `#[must_use]` and are
intentionally not cloneable.

For fixed declarations, use `reconcile_once`,
`reconcile_after_completion`, or `reconcile_watchdog` during both `init` and
`post_upgrade`. Those helpers always create retained declarations, including
inactive ones. Transient `RemoveWhenStopped` declarations use the direct
registration functions.

## 🏗️ Runtime ownership

| Layer | Owns |
| --- | --- |
| Consumer | Durable demand, application outcomes, and lifecycle composition |
| Registration capability | Claim-scoped control for one logical declaration |
| `ic-timers` runtime | Callback execution, provider effects, and lifecycle reconciliation |
| Canonical registry | Identities, generations, policy state, arbitration, callbacks, counters, and handles |
| Private `platform` module | The only direct `ic-cdk-timers` and IC system-fact calls |

The registry is volatile. It stores no stable timer policy, provider handle,
generation, snapshot, epoch, or application recovery authority. After an
upgrade, consumers derive desired timers from their own durable state and
reconstruct them synchronously before downstream post-upgrade work.

Callbacks run without a registry borrow. Nested ensure, reconcile, cancel,
and unregister requests are arbitrated by one canonical pending command and
the exact callback generation.

## 📊 Truthful observability

`timer_snapshot` and `timer_inventory` return inert values; snapshots never
become mutation authority. The inventory carries the runtime epoch even when
its ordered timer slice is empty. `timer_inventory` is the 0.6 replacement for
the removed bare-vector `timer_snapshots` function.

| Observation | Meaning |
| --- | --- |
| Identity and policy | Deterministic ownership and the configured execution contract |
| Runtime state | Policy-specific inactive, armed, running, successor, and watchdog-attempt state |
| Counters | Requested, armed, started, completed, outcomes, cancellations, stale work, coalescing, and unacknowledged work |
| Instructions | Completed scheduler/work sample count plus total, latest, and maximum measurements |
| Memory | Latest start/end Wasm and stable page extents plus maximum observed growth |
| Epoch | The volatile runtime boundary to which counters and samples belong |

Each instruction measurement covers the accepted `ic-timers` execution
interval: it begins immediately before callback acceptance and ends after
completion processing and any successor binding. It is not an exclusive
measurement of application code, nor is it the complete IC message. Provider
dispatch before entering the runtime, provider return/reply work, page reads,
and the bounded summary write remain outside the instruction delta. Do not use
the library aggregate alone as proof against the IC message instruction limit.

Trapped or instruction-exhausted work produces no fabricated sample. A
terminal `RemoveWhenStopped` callback can delete its declaration before the
final measurement is retained; no timer then remains to expose that sample.
Consumers that require a durable terminal audit receipt must store it outside
the volatile timer registry rather than treating a snapshot as authority.

Wasm and stable-memory values are 64 KiB page extents: they are runtime
high-water observations, not exact live bytes. Sub-page allocator liveness
remains an owner-derived measurement. Async ordinary samples may also include
canister activity interleaved while the callback future awaits.

Read scheduler and work memory independently, and preserve `Option` when
projecting the values:

```rust
use ic_timers::MemoryPageSummary;

struct CompletedMemoryObservation {
    samples: u64,
    wasm_start_pages: u64,
    wasm_end_pages: u64,
    stable_start_pages: u64,
    stable_end_pages: u64,
    maximum_wasm_growth_pages: u64,
    maximum_stable_growth_pages: u64,
}

fn completed_memory(
    summary: MemoryPageSummary,
) -> Option<CompletedMemoryObservation> {
    let latest = summary.latest()?;
    Some(CompletedMemoryObservation {
        samples: summary.samples(),
        wasm_start_pages: latest.start().wasm_pages(),
        wasm_end_pages: latest.end().wasm_pages(),
        stable_start_pages: latest.start().stable_pages(),
        stable_end_pages: latest.end().stable_pages(),
        maximum_wasm_growth_pages: summary.maximum_wasm_growth_pages()?,
        maximum_stable_growth_pages: summary.maximum_stable_growth_pages()?,
    })
}

// Given a TimerSnapshot named `snapshot`:
let performance = snapshot.observability().performance();
let scheduler = completed_memory(performance.scheduler_memory_pages());
let work = completed_memory(performance.work_memory_pages());
```

Here `None` means no callback of that role completed normally. `Some` with
equal start/end extents and maximum growth of zero is a real completed sample
that observed no page growth.

`has_armed_wakeup()` is a claim-scoped observation of canonical provider-handle
ownership. It is not a delivery guarantee or durable demand. When durable
demand requires a timer, call `ensure_scheduled()` unconditionally instead of
using the observation as a check-then-arm guard.

## 🔄 Lifecycle and shared-registry rules

1. The canister's existing lifecycle owner calls `initialize_runtime()`.
2. Frameworks and applications reconcile their retained declarations from
   consumer-owned durable authority.
3. Application hooks run only after required reconstruction.
4. The same sequence runs during `init` and `post_upgrade`.

The crate exports no lifecycle hook, lifecycle macro, public Candid endpoint,
or consumer-work executor.

> 🚨 Every consumer linked into one canister must resolve the same
> `ic-timers` Cargo package ID. Two resolved versions create two independent
> library statics and therefore two registries.

The registry's 128-handle ceiling does not reserve capacity in the provider's
canister-wide limit of 250 outstanding dispatches. A composed canister must
also inventory or migrate every remaining direct `ic-cdk-timers` user.

Canic, IcyDB, and application timers motivated this shared model. The
[Canic adapter contract](docs/adoption/canic.md) and
[IcyDB adoption record](docs/adoption/icydb.md) preserve the exact downstream
mapping and evidence without making patch-specific worktree state part of
this README.

For a canister with one simple callback and no need for shared inventory,
metrics, lifecycle reconciliation, or a recovery policy, using
`ic-cdk-timers` directly remains the simpler choice.

## 🛡️ Guarantees and limits

| Guarantee | Boundary |
| --- | --- |
| Deterministic bounded identity | Three non-empty components, each at most 64 UTF-8 bytes |
| Unique canonical ownership | One declaration per identity within one resolved runtime |
| Checked scheduling | Positive cadence and checked deadline arithmetic |
| Harmless stale delivery | Identity, claim, callback generation, and role are validated |
| Fail-closed binding | Failed provider effects cannot leave false scheduled state |
| Safe cancellation | Owned provider handles are cleared; already-running work is not interrupted |
| Watchdog recovery | Pre-armed successor survives consumer-work trap or exhaustion |
| No catch-up storm | Overdue watchdog cadence coalesces from current dispatch time |

Read [SAFETY.md](SAFETY.md) before relying on the watchdog protocol or
operational measurements.

## 🧪 Development and evidence

| Command | Purpose |
| --- | --- |
| `make update-dev` | Install the pinned toolchain, components, Wasm target, and formatting hook |
| `make ci` | Run the normal warning-denied checks, native tests, Wasm build, and package checks |
| `make msrv` | Prove the Rust 1.88.0 workspace and supported probe configurations |
| `make repository-check` | Validate repository-only documentation, evidence, or tooling work |
| `make pocketic-watchdog` | Run the focused real-canister watchdog recovery matrix |
| `make pocketic-cohorts` | Compare real-canister policy cohorts and measurements |
| `make release-impact` | Classify changes as crate-impacting, repository-only, or absent |

Normal development uses Rust 1.97.1. The real-canister suites use the audited
PocketIC 15.0.0 Linux x86_64 binary. The first run downloads it into the
ignored `target/tools` cache; later runs verify its version and SHA-256. Set
`POCKET_IC_BIN=/path/to/pocket-ic` only for an explicitly managed binary.

The evidence covers real work traps, 40-billion-instruction exhaustion,
insufficient cycles followed by top-up, upgrade reconstruction, stop/resume,
overdue coalescing, cancellation gaps, duplicate demand, simultaneous timers,
trap isolation, and rejection of external executor ingress.

## 📚 Documentation map

| Document | Purpose |
| --- | --- |
| [Architecture](docs/architecture.md) | Canonical ownership, module boundaries, and lifecycle model |
| [Safety boundary](SAFETY.md) | Exact guarantees, assumptions, and non-guarantees |
| [Observability design](docs/design/observability.md) | Snapshot and measurement semantics |
| [0.5 design](docs/design/0.5-policy-specific-callback-authority.md) | Policy-specific callback capabilities and migration boundary |
| [0.6 release note](docs/changelog/0.6.0.md) | Atomic inventory epoch and hard-cut migration |
| [Runtime evidence](docs/audits/0.3-runtime-evidence-2026-08-13.md) | PocketIC protocol, isolation, size, and instruction evidence |
| [Current status](docs/status/current.md) | Compact maintainer handoff |
| [Release guide](docs/releasing.md) | Versioning, validation, and publication workflow |

## 📄 License

MIT
