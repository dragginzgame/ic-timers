# Safety boundary

`ic-timers` is a higher-level wrapper around `ic-cdk-timers`. The CDK remains
the platform timer provider; this crate owns only the coordination it can
actually enforce and test.

## Current guarantees

The 0.3 runtime provides:

- a provider-call-free fixed-capacity registry with unique bounded identities
  and deterministic snapshot ordering;
- checked positive cadence, deadline, generation, and request arithmetic;
- policy-specific ordinary and watchdog states that cannot express a watchdog
  successor as ordinary running state;
- deterministic arbitration among scheduling, cancellation, unregistration,
  callback completion, and stale generations;
- saturating completion, stale, coalescing, and unacknowledged-event state;
- inert snapshots with private construction and read-only accessors;
- one volatile canister-local owner initialized through a synchronous,
  idempotent seam;
- live `Once` and `AfterCompletion` callback execution without retaining a
  registry borrow across consumer work or an `await`;
- a live synchronous watchdog whose scheduler arms the next cadence successor
  before it queues a separate immediate work callback and returns without
  invoking consumer work;
- private, non-copyable provider handles owned by canonical entries (one for
  ordinary timers, at most successor plus work for watchdogs), with terminal
  cancellation clearing the actual handles and all direct `ic-cdk-timers` use
  isolated in the private, non-re-exported `platform` module;
- synchronous idempotent reconstruction of retained declarations from a
  caller-owned volatile claim slot and caller-supplied desired state, without
  persisted library authority;
- fresh inactive lifecycle reconciliation that retains an observable
  declaration and consumes no provider handle, allowing fixed owners to reserve
  bounded inventory capacity before application hooks;
- direct registration, rather than lifecycle reconciliation, for transient
  `RemoveWhenStopped` declarations whose capability expires on removal;
- exact ordinary reconciliation whose pending command has one canonical owner
  in the registry and can replace a live deadline in either direction;
- work-scoped `TimerContext` delegation validated against the exact callback
  generation and role, so a context retained after completion cannot mutate a
  successor or later registration;
- normally completed scheduler and work instruction samples from IC
  call-context counter type 1;
- focused PocketIC evidence that explicit trap and actual instruction
  exhaustion roll back work completion while the committed successor remains,
  retires the attempt as unacknowledged, and permits later progress;
- focused PocketIC evidence for upgrade reconstruction before a downstream
  hook, stop/resume, insufficient cycles followed by top-up, a 300-second
  overdue jump without replay, two simultaneous timers, and isolation when one
  timer traps;
- terminal cancellation both normally and in the committed scheduler/work gap,
  leaving later provider delivery unable to invoke consumer work;
- external rejection of the provider's internal timer-executor route; and
- fail-closed handling of unexpected internal callback completion, provider
  ownership, cleanup, and accounting errors. Watchdog work traps its current
  message instead of returning after losing the committed successor;
- fail-closed public and lifecycle effect application: a retained declaration
  whose provider arm cannot establish canonical ownership becomes inactive
  with `ProviderBindingFailed` rather than remaining falsely scheduled; and
- claim-scoped observation of whether the exact registration owns an armed
  provider wake-up handle. For watchdogs this counts the cadence successor,
  not the separately queued work callback.

Snapshot values describe runtime observations. They are not authority to arm,
clear, restore, or mutate a timer and must not become an alternate control
path. `has_armed_wakeup` has the same observational boundary: it is neither
durable authority nor a delivery guarantee, and consumers must still invoke
the idempotent ensure operation whenever their authority requires a wake-up.

## Limits and consumer obligations

- `Once` and `AfterCompletion` do not pre-arm a successor. A trap or
  instruction exhaustion before their callback returns can leave no future
  wake-up. Recovery-critical work must use `Watchdog`.
- Watchdog work is synchronous and must remain one bounded unit. The runtime
  does not permit it to cross an `await`.
- Watchdog recovery covers traps and instruction exhaustion in the later
  consumer-work message, not in the scheduler message that creates the next
  successor. The scheduler is fixed and bounded, but its normal return remains
  a protocol assumption.
- A committed successor provides another attempt, not exactly-once application
  effects. Consumer work remains idempotent and owns its durable authority.
- Initialization and reconciliation are explicit lifecycle calls. The single
  lifecycle export owner must invoke them synchronously before downstream
  hooks on install and upgrade.
- Timers are volatile. Consumers derive desired reconstruction from existing
  durable state; they must not persist provider handles, callback generations,
  or timer snapshots as mutation authority.
- All timer consumers must resolve the same `ic-timers` Cargo package ID. Two
  resolved versions create two independent registries.
- There is deliberately no global suspend/resume switch. A consumer may cancel
  only claims it owns and must reconstruct them from its own authority; it
  cannot suspend another owner's watchdog through this crate.
- A consumer may keep a bounded custody collection of its opaque claims for
  enumeration. That collection must not copy deadlines, generations, pending
  commands, counters, snapshots, or provider handles into a parallel authority.
- `TimerContext` may be used for nested ensure, reconciliation, or cancellation
  only while its exact consumer-work attempt is running. Retaining it provides
  inert identity metadata, not a second long-lived registration capability.
- The provider's 250 outstanding-dispatch limit is canister-wide. The
  registry's 64-entry and 128-owned-handle bounds do not reserve provider
  capacity; consumers must inventory remaining direct provider users and
  tolerate provider deferral as an operational retry condition.
- The current evidence uses the pinned PocketIC 15.0.0 binary and
  `ic-cdk-timers` 1.0.0 provider. A provider or evidence-binary change requires
  a renewed source and recovery audit.
- Tagged IcyDB 0.226.1 and its validated post-tag exact-0.3.8 integration
  supply maintained downstream shared-registry evidence, recorded separately
  from this library's owner-local proof. A validated uncommitted Canic
  exact-0.3.8 worktree supplies the real metrics/status adapter and removes its
  parallel timer runtime. The development graphs align; released composition
  still requires one-package qualification from tagged downstream subjects.

The frozen [0.3 Patch 1 contract](docs/design/0.3-patch-1-contract.md) defines
the protocol and the
[closeout report](docs/audits/0.3-runtime-evidence-2026-08-13.md) maps every
promotion case to direct evidence. The
[IcyDB adoption record](docs/adoption/icydb.md) identifies which additional
claims come from the tagged release and validated post-tag integration.

## Failure and measurement semantics

Callback starts and completions are deliberately separate. A trap or
instruction exhaustion can prevent all post-run code, so the runtime must not
invent a completion, zero instruction cost, elapsed duration, or zero work
count.
The live watchdog records only that an earlier committed dispatch lacks a
committed completion when a later scheduler retires it. It cannot infer a trap,
instruction exhaustion, delay, or interruption from that fact alone.

All hot-path observation counters and aggregates saturate rather than trap.
Saturation protects timer execution; it does not make a saturated metric exact.
The runtime epoch identifies the reset scope so operators can distinguish a
reset from a genuine lifetime zero.

## Evidence maintenance

The recovery suite is deliberately outside the fast default CI gate. Run
`make pocketic-watchdog` with the audited PocketIC 15.0.0 binary after changes
to provider binding, registry transitions, lifecycle reconstruction, or
watchdog dispatch. Run `make pocketic-cohorts` after changes that can affect
linked Wasm, instruction cost, or provider-call count. Native mocks remain
necessary for exhaustive state transitions but are never a substitute for IC
commit/rollback evidence.
