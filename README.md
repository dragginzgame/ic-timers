![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

<!-- helper-navigation:start -->
<p align="center">
  <a href="https://github.com/dragginzgame/canic"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/canic.svg" width="18" height="18" alt=""> <strong>canic</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/icydb"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/icydb.svg" width="18" height="18" alt=""> <strong>icydb</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-timers"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-timers.svg" width="18" height="18" alt=""> <strong>ic-timers</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-memory"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-memory.svg" width="18" height="18" alt=""> <strong>ic-memory</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-query"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-query.svg" width="18" height="18" alt=""> <strong>ic-query</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-backup"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-backup.svg" width="18" height="18" alt=""> <strong>ic-backup</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-blob-storage"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-blob-storage.svg" width="18" height="18" alt=""> <strong>ic-blob-storage</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-testkit"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-testkit.svg" width="18" height="18" alt=""> <strong>ic-testkit</strong></a>
</p>
<!-- helper-navigation:end -->

> A shared scheduler for background tasks in applications running on the
> Internet Computer.

## What is IC Timers?

The Internet Computer is a network that runs software. Programs on it are
called *canisters*. Like other applications, a canister may need to do some
work later or repeat work in the background.

IC Timers is a tool that developers can add to a canister to organize that
work. You can think of it as a shared alarm clock and task list for the whole
application. It can tell the application when to run a task, keep related
timers in one place, and report what happened when they ran.

For example, an application might use IC Timers to:

- remove expired records;
- process a queue a few items at a time;
- run regular database maintenance;
- try important work again after an interrupted attempt; or
- show operators which background tasks are waiting, running, or stopped.

People using the application do not interact with IC Timers directly. They
benefit from background work that is easier for the application's developers
to organize, monitor, and recover.

IC Timers does not permanently store an application's tasks. The application
keeps the lasting record of what needs to happen and rebuilds its timers after
an upgrade. Its Watchdog mode can preserve another attempt when work fails,
but it does not promise that a task will happen exactly once.

![Application tasks flow through IC Timers to the Internet Computer timer system and one shared status view](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-how-it-helps.svg)

## Technical overview

| Field | Value |
| --- | --- |
| API line | `0.14` |

IC Timers is written in Rust 2024 and supports Rust
1.88.0 and newer. It uses `ic-cdk-timers` 1.0.0 as its private, underlying
timer service.

One IC Timers runtime can manage up to 64 named timers and 128 timer handles.
It supports one-time work, work that repeats after returning normally, and
Watchdog work that prepares another attempt before it starts. It also reports
timer status, outcomes, and bounded resource measurements.

![Application needs matched to IC Timers scheduling and observation capabilities](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-application-needs.svg)

## Why wrap `ic-cdk-timers`?

`ic-cdk-timers` is the right low-level mechanism for scheduling a simple
callback. The operational problem changes when a framework, a database, and
application code all schedule work independently.

![Separate component timers compared with one shared IC Timers registry and status view](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-shared-registry.svg)

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

## Choose how a task should run

IC Timers offers three modes. `Once` runs a task one time unless explicitly
rescheduled. `AfterCompletion` can repeat after a normal return when its result
requests recurrence. `Watchdog` prepares another attempt before starting important
work. The timeline below shows the difference.

![Timelines showing when Once, AfterCompletion, and Watchdog schedule their work and successor](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-policy-timelines.svg)

`Watchdog` deliberately uses two messages. The small scheduler callback
validates its generation, arms the next cadence successor, queues immediate
work, and returns. Only the later work callback invokes consumer code.

> **Watchdog limit:** Watchdog protects against failure in the task itself. It
> cannot promise recovery if its small scheduling step fails or exceeds the
> Internet Computer's work limit.

A Watchdog can drain successful bounded work without waiting a full cadence:

```rust,ignore
let decision = match outcome {
    SuccessfulProgress { more_work: true } => WatchdogDecision::ContinueImmediately,
    RetryableFailure => WatchdogDecision::Continue,
    Quiescent | TerminalFailure => WatchdogDecision::Stop,
};
```

Use `WatchdogReconcileState::ScheduledImmediately` when lifecycle
reconstruction discovers inactive actionable debt, or call
`WatchdogRegistration::ensure_scheduled_immediately()` when a retained claim
discovers new debt later. “Immediately” arms a zero-delay scheduler for a later
replicated message; it never calls consumer work synchronously. The scheduler
still pre-arms the cadence successor before every work attempt, and normal
`ContinueImmediately` replaces that exact successor rather than adding one.

The 64-registration and 128-owned-handle limits bound this runtime's inventory
and live capabilities. They do not cap the provider's queue memory: the pinned
provider keeps cancelled deadline records until those deadlines are processed.
Frequent deadline replacement and immediate continuation can therefore grow
memory and incur later cleanup work. See [the safety limits](SAFETY.md).

Ordinary recurrence is cheaper and is the correct default when work must
finish normally before another invocation is allowed. Use `Watchdog` only
when committing the next wake-up before fallible synchronous work is the
required protocol.

Ordinary callback results follow their policy. `register_once` and
`reconcile_once` require futures returning `OnceRunResult` with a `OnceDecision`;
the after-completion entry points require `AfterCompletionRunResult` with an
`AfterCompletionDecision`. Both permit Stop, immediate continuation, a relative
retry and an absolute deadline. Only `AfterCompletionDecision` includes
`RecurAfterCompletion`, which uses the configured cadence. A Once declaration
can still explicitly reschedule itself. Invariant-failure results force Stop,
and authoritative nested commands keep their existing arbitration precedence.
Configured recurrence can follow success, no work or a returned retryable
failure. A trap or instruction exhaustion prevents ordinary completion from
scheduling it; recovery-critical work needs Watchdog.

When the provider or CDK drops a confirmed ordinary delivery without normal
completion, its guard retires the exact generation. A retained declaration becomes
inactive with `InactiveReason::Abandoned`, Failed condition and an Unacknowledged
observation. Its owner can explicitly rearm it. Transient declarations and pending
unregistrations are removed; other pending schedules are discarded. No completion,
work count or performance sample is fabricated. A future that remains alive and
pending still owns Running state; cancellation does not interrupt it.

This abandonment path is covered by native drop and real-await PocketIC fixtures.
The 0.13.2 hosted release gates passed those subjects on both supported macOS
hosts; [qualification is bound to that source](docs/releasing.md#host-support). It does
not recover application effects committed before an await or automatically retry
work. Capture destructors must remain bounded, nontrapping and safe in CDK cleanup.

Completion classification records what happened; the supplied decision selects
what to schedule. A retryable failure paired with `Stop` does not automatically
retry, and success paired with `Stop` does not recur. The result constructors
preserve the decision for success, no work and retryable failure, and force
`Stop` for invariant failure.

Result construction does not check relative delays. After authoritative commands
have been applied, the runtime checks any selected delay's nanosecond encoding
and successor deadline. An invalid selected delay stops the timer with a typed
control failure, observable in a retained declaration's snapshot. A winning
exact reconciliation can discard an invalid callback delay before that check.

## Minimal `Once` example

The examples below use the policy-specific callback results introduced in 0.12.

Add one exact package version when this crate participates in a shared
framework/application registry:

```toml
[dependencies]
ic-timers = "=0.14.8"
```

Every framework and application crate linked into the same canister must use
that same exact package version. Mixing this pin with an older exact release
can resolve two package identities and therefore create two independent timer
registries.

Initialize the runtime from the canister's existing lifecycle owner, declare a
timer, retain its non-clone registration capability, and schedule it:

```rust
use ic_timers::{
    DeclarationLifetime, OnceRegistration, TimerCompletion, OnceDecision,
    TimerIdentity, OnceRunResult, TimerSchedule, initialize_runtime,
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
            OnceRunResult::new(TimerCompletion::success(1), OnceDecision::Stop)
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

## Runtime ownership

![Ownership and control flow from durable application state through the IC Timers runtime to its private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-runtime-ownership.svg)

The registry is volatile. It stores no stable timer policy, provider handle,
generation, snapshot, epoch, or application recovery authority. After an
upgrade, consumers derive desired timers from their own durable state and
reconstruct them synchronously before downstream post-upgrade work.

Callbacks run without a registry borrow. Nested ensure, reconcile, cancel,
and unregister requests are arbitrated by one canonical pending command and
the exact callback generation.

Since the 0.8 API, a sleeping Watchdog can reconcile its own exact
wake-up with `reconcile_schedule(Some(TimerSchedule::At(deadline_ns)))` and
return `WatchdogDecision::ScheduleAt(next_deadline_ns)` after successful work.
The scheduler still commits the cadence recovery successor before work runs.
Return `Stop` when idle and reconstruct with `WatchdogReconcileState::ScheduledAt`
when durable demand supplies a deadline. See the
[0.8 contract](docs/design/0.8-registration-continuity-and-deadlines.md).

## Truthful observability

![Timer state, outcomes, instruction use, memory growth, and registration identity flowing into one read-only operational snapshot](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-observability.svg)

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
| Registration identity | Runtime epoch plus registration sequence; changes when counters are replaced, including within one epoch |

Before subtracting cumulative measurements, compare `registration_id()` within
the same canister and require advancing source times. Cancellation preserves the
identifier; unregister/re-register changes it. Neither endpoint may be
`u64::MAX`: saturation makes exact deltas unavailable. Callback `generation()`
does not identify a counter lifetime.

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
demand requires a timer, call `ensure_scheduled()` or
`ensure_scheduled_immediately()` unconditionally instead of using the
observation as a check-then-arm guard.

## Lifecycle and shared-registry rules

![Upgrade lifecycle from durable application state through timer reconstruction to resumed background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-upgrade-lifecycle.svg)

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

## Guarantees and limits

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

## Development and evidence

| Command | Purpose |
| --- | --- |
| `make update-dev` | Install the pinned toolchain, components, host and IC tools, Wasm target, and formatting hook |
| `make install-host-tools` / `make host-tools-check` | Install pinned jq/yq/ripgrep/cloc or verify the complete bundle offline |
| `make install-ic-tools` / `make ic-tools-check` | Install the pinned six-tool IC bundle or verify it offline |
| `make install-tools` / `make tools-check` | Prepare or verify both host and IC bundles |
| `make cloc` | Report Rust LOC/test counts for the publishable root workspace |
| `make cloc-tooling` | Inventory sibling CI/tooling (`CLOC_PARENT=/path/to/projects`) |
| `make fmt` / `make fmt-check` | Sort manifests and format or check Rust in the root and `testing/` workspaces |
| `make ci` | Run the normal warning-denied checks, native tests, Wasm build, and package checks |
| `make msrv` | Check the workspace with Rust 1.88.0 |
| `make testing-check` | Check workspace formatting and lint supported unpublished probes with Rust 1.88.0 |
| `make repository-check` | Validate repository-only documentation, evidence, or tooling work |
| `make pocketic-watchdog` | Run the focused real-canister watchdog recovery matrix |
| `make pocketic-cohorts` | Compare real-canister policy cohorts and measurements |
| `make release-impact` | Classify changes as crate-impacting, repository-only, or absent |

Normal development and hosted CI use Rust 1.99.0. Hosted CI also lints every
supported probe configuration with both Rust 1.99.0 and Rust 1.88.0.
Prepare the complete pinned host bundle through `make update-dev` or `make install-host-tools`
before validation. `make actions-check` verifies the bundle offline and delegates
Actions and Cargo declaration checks to the reviewed shared parser; it never
downloads tools. See the [setup and pin boundaries](docs/releasing.md#structured-dependency-checks-and-host-parsers).
Both `fmt` and `fmt-check` first require the exact cargo-sort pin and prepared
rustfmt for the selected toolchain; missing tools require explicit setup.
To run the development-toolchain probe checks locally, use
`make testing-check MSRV=1.99.0`. The host-side real-canister suites use exact
`ic-testkit` 0.20.0 and the pinned PocketIC 16.0.0 server on Linux x86_64 or
macOS Intel/Apple Silicon. The first run downloads it into the ignored
`target/tools` cache; later runs verify its version and SHA-256. Set
`POCKET_IC_BIN=/path/to/pocket-ic` only for an explicitly managed binary. Testkit
starts a caller-owned server and fresh IC instance for each fixture, with a
30-second deadline for each startup phase. The instance is dropped before its
server; upstream instance deletion itself remains unbounded.

Recorded PocketIC 15 evidence covers real work traps, 40-billion-instruction
exhaustion, insufficient cycles followed by top-up, upgrade reconstruction, stop/resume,
overdue coalescing, cancellation gaps, duplicate demand, simultaneous timers,
trap isolation, and rejection of external executor ingress. The testkit/PocketIC
16 harness requires renewed execution of the full recovery and cohort gates;
source review and verified artifact hashes do not qualify those behaviors.

## Documentation map

| Document | Purpose |
| --- | --- |
| [Architecture](docs/architecture.md) | Canonical ownership, module boundaries, and lifecycle model |
| [Safety boundary](SAFETY.md) | Exact guarantees, assumptions, and non-guarantees |
| [Observability design](docs/design/observability.md) | Snapshot and measurement semantics |
| [0.5 design](docs/design/0.5-policy-specific-callback-authority.md) | Policy-specific callback capabilities and migration boundary |
| [0.6 release note](docs/changelog/0.6.0.md) | Atomic inventory epoch and hard-cut migration |
| [0.7 release note](docs/changelog/0.7.0.md) | Immediate progress-sensitive Watchdog continuation |
| [0.8 release note](docs/changelog/0.8.0.md) | Registration continuity and exact Watchdog deadlines |
| [Runtime evidence](docs/audits/0.3-runtime-evidence-2026-08-13.md) | PocketIC protocol, isolation, size, and instruction evidence |
| [Current status](docs/status/current.md) | Compact maintainer handoff |
| [Release guide](docs/releasing.md) | Versioning, validation, and publication workflow |

## License

MIT
