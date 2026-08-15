# Safety boundary

`ic-timers` is a higher-level wrapper around `ic-cdk-timers`. The CDK remains
the platform timer provider; this crate owns only the coordination it can
actually enforce and test.

## Current guarantees

The current runtime provides:

- a provider-call-free fixed-capacity registry with unique bounded identities
  and deterministic snapshot ordering;
- checked positive cadence, deadline, and callback-generation arithmetic;
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
  `RemoveWhenStopped` declarations whose capability expires on terminal
  completion or cancellation, including cancellation before the first
  provider wake-up is armed;
- exact ordinary reconciliation whose pending command has one canonical owner
  in the registry and can replace a live deadline in either direction;
- work-scoped `OnceContext`, `AfterCompletionContext`, and `WatchdogContext`
  delegation validated against the exact callback generation and role. Each
  exposes only policy-valid operations, and a context retained after
  completion cannot mutate a successor or later registration;
- normally completed scheduler and work instruction samples from IC
  call-context counter type 1, paired with allocation-free start/end Wasm and
  stable memory extents in 64 KiB pages;
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
  message instead of returning after losing the committed successor. Detached
  failure paths restore or clear every linear provider capability before
  returning an error. Terminal watchdog control failures also clear any
  pending nested command and check paired scheduler/work generations before
  mutating either counter. Provider-effect shape is checked before platform
  calls, including exact identity and claim-generation agreement between a
  Watchdog successor and its queued work. Arm effects distinguish initial from
  replacement ownership, while clear effects name a non-empty handle set; a
  no-op clear is not representable. Callback dispatch cannot consume a provider
  handle until the identity, registration claim generation, callback
  generation, and role all match the canonical owner;
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
- A policy-specific callback context may be used for its nested ensure,
  reconciliation, or cancellation operations only while its exact
  consumer-work attempt is running. Retaining it provides inert identity
  metadata, not a second long-lived registration capability.
- The provider's 250 outstanding-dispatch limit is canister-wide. The
  registry's 64-entry and 128-owned-handle bounds do not reserve provider
  capacity; consumers must inventory remaining direct provider users and
  tolerate provider deferral as an operational retry condition.
- The current evidence uses the pinned PocketIC 15.0.0 binary and
  `ic-cdk-timers` 1.0.0 provider. A provider or evidence-binary change requires
  a renewed source and recovery audit.
- IcyDB and Canic independently supply maintained exact-0.5.0
  shared-registry evidence, recorded separately from this library's
  owner-local proof. Canic's schema-3 adapter exports timer, instruction, and
  memory observations without a parallel runtime. Combined composition still
  requires one final Wasm proving one registry, both owners in one inventory,
  synchronous lifecycle reconstruction, IcyDB Watchdog recovery, and
  continued Canic timer progress.

The frozen [0.3 Patch 1 contract](docs/design/0.3-patch-1-contract.md) defines
the protocol and the
[closeout report](docs/audits/0.3-runtime-evidence-2026-08-13.md) maps every
promotion case to direct evidence. The
[IcyDB adoption record](docs/adoption/icydb.md) and
[Canic adapter contract](docs/adoption/canic.md) identify which additional
claims come from maintained downstream evidence.

## Failure and measurement semantics

Callback starts and completions are deliberately separate. A trap or
instruction exhaustion can prevent all post-run code, so the runtime must not
invent a completion, zero instruction cost, memory-page sample, elapsed
duration, or zero work count. Memory observations report runtime-epoch-local
monotonic Wasm and stable page extents plus observed start-to-end growth, not
exact live bytes, sub-page allocator liveness, or exclusive work attribution.
The accepted measurement envelope starts before callback acceptance and ends
after completion processing and any successor binding, so it includes
`ic-timers` runtime work and is not exclusive application-code attribution. An
async ordinary callback's interval may also include canister activity
interleaved while its future is awaiting. A terminal `RemoveWhenStopped`
callback can remove its declaration before the final measurement is retained;
no timer remains from which to observe that sample.

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
