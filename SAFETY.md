![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Safety boundary

This document explains what IC Timers promises, what it cannot promise, and
what the application using it must still do. The summaries below are an
introduction. The detailed sections that follow define the exact technical
contract.

## In plain English

IC Timers keeps a limited number of timers organized in one place. It checks
timer identities and scheduling values, safely ignores old callbacks, and
provides read-only information about timer activity.

Watchdog mode prepares another attempt before it starts the application's
work. This means another attempt can remain scheduled if that work crashes or
uses too many instructions. It does not guarantee that work happens exactly
once, and it cannot recover if its own small scheduling step fails.

![Watchdog prepares another attempt before queueing work, so the prepared attempt remains if that work fails](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-watchdog-failure.svg)

The application is still responsible for:

- keeping the lasting record of what work is needed;
- rebuilding its timers after an install or upgrade;
- making repeated work safe to run again;
- keeping each Watchdog task small and bounded; and
- ensuring every component uses the same version of IC Timers.

IC Timers is built on `ic-cdk-timers` and shares the Internet Computer's
platform limits. Its own timer and handle limits do not place a fixed limit on
the underlying provider's memory use or guarantee that every scheduled task
will be delivered.

![The application, IC Timers, and the platform provider each own a different part of timer safety](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-safety-responsibilities.svg)

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
- inert registration identity combining the runtime epoch and a checked,
  non-wrapping registration sequence, preserving counter continuity through
  cancellation and detecting unregister/re-register within the same epoch;
- one volatile canister-local owner initialized through a synchronous,
  idempotent seam;
- live `Once` and `AfterCompletion` callback execution without retaining a
  registry borrow across consumer work or an `await`;
- a live synchronous watchdog whose scheduler arms the next cadence successor
  before it queues a separate immediate work callback and returns without
  invoking consumer work;
- progress-sensitive Watchdog continuation that replaces that exact pre-armed
  successor with a deadline of now after normal completion, plus initial
  immediate scheduling through the same claim, generation, and handle owner;
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
- native boundary evidence that checked terminal control failures also expire
  transient declarations and release capacity, with retained declarations
  staying inactive and observable; a queued-work fixture verifies that actual
  provider handles clear before transient scheduler failure removal;
- exact ordinary reconciliation whose pending command has one canonical owner
  in the registry and can replace a live deadline in either direction. The
  0.9.0 arbitration hard cut discards overridden callback scheduling proposals
  before validation; its new precedence fixtures require recorded successful
  execution before being treated as evidence;
- exact Watchdog reconciliation and `ScheduleAt` completion through the same
  successor owner, preserving the cadence recovery wake-up until normal work
  completion commits the requested deadline;
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
  pending nested command and check generation/deadline arithmetic before
  advancing the allocation counter. Provider-effect shape is checked before platform
  calls, including exact identity and claim-generation agreement between a
  Watchdog successor and its queued work, with one shared dispatch generation.
  Arm effects distinguish initial from
  replacement ownership, while clear effects name a non-empty handle set; a
  no-op clear is not representable. Callback dispatch cannot consume a provider
  handle until the identity, registration claim generation, callback
  generation, and role all match the canonical owner;
- zero-delay Watchdog requests that coalesce when work is already dispatched
  or a scheduler deadline is already earlier/equivalent, and that replace one
  later successor rather than adding another. An immediate pending request
  applies only to the exact running attempt, cannot be downgraded by cadence
  ensure, and remains subordinate to later cancellation or sticky
  unregistration;
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

Callback generations are allocated only for new deliveries. Each Watchdog
dispatch gives its successor and work one fresh generation with distinct roles
and provider slots. Requests while dispatched or
running cannot replace that pair before completion; a recovery scheduler expires
the interrupted attempt before allocating a new pair. Cancellation selects
inactive state or queues a running-work stop without advancing a counter.
Inactive state rejects old delivery; rearming requires a fresh checked generation.
Cancellation at exhausted counters and the changed cancel/rearm and shared-dispatch
sequences have native fixtures awaiting deployment validation. The existing
PocketIC cancellation and stop/resume
subjects must qualify this change before extending the evidence claims above.
See the [0.11.0 verification scope](docs/changelog/0.11.0.md).

## Limits and consumer obligations

- Compare registration identities within one canister history before deriving
  counter deltas; require advancing source times and unsaturated endpoints.
  A runtime epoch or callback generation alone does not prove continuity.
  `u64::MAX` is conservatively saturated, and reinstalls or restored/forked
  histories require a fresh observation baseline. Identity supplies neither
  complete-message cost accounting nor balance-transfer attribution.
- Exact deadline requests for already dispatched/running Watchdog work select
  its successor on normal completion. They do not postpone queued work or alter
  the committed recovery cadence; an interrupted attempt's pending proposal
  is retired at recovery. Derive renewed demand from consumer-owned authority.
- `Once` and `AfterCompletion` do not pre-arm a successor. A trap or
  instruction exhaustion before their callback returns can leave no future
  wake-up. Recovery-critical work must use `Watchdog`.
- Ordinary delivery abandonment now has a private guard constructed before the
  provider's first poll. If a confirmed current delivery is dropped, it retires
  scheduled/running authority without provider calls, records Unacknowledged and
  selects `InactiveReason::Abandoned` for retained declarations. Transients and
  pending unregister are removed; pending schedules do not become retries. No
  completion or performance sample is synthesized. Normal cancellation and
  replacement retire authority before dropping old provider futures, so those
  drops and unconfirmed binding failures do not count as abandonment.
  Native drop fixtures and PocketIC pre-await/continuation trap subjects passed
  in the 0.13.2 hosted release gates, with source/host scope in the
  [release qualification record](docs/releasing.md#host-support). Successful CDK/provider destruction
  and available canonical registry ownership remain assumptions; capture Drop
  code must be bounded, nontrapping and safe in cleanup context, and must not
  schedule provider work there. Ordinary cleanup does not repair application
  effects across awaits or terminate a future that remains alive and pending.
- Watchdog work is synchronous and must remain one bounded unit. The runtime
  does not permit it to cross an `await`.
- Watchdog recovery covers traps and instruction exhaustion in the later
  consumer-work message, not in the scheduler message that creates the next
  successor. The scheduler is fixed and bounded, but its normal return remains
  a protocol assumption.
- `ContinueImmediately` is a consumer scheduling classification, not inferred
  progress or a configurable retry policy. Consumers use it after successful
  bounded progress, retain `Continue` for cadence-delayed retryable failures,
  and keep work idempotent.
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
- Registry and owned-handle bounds do not bound provider queue memory. In the
  pinned provider, `clear_timer` removes the callback task but leaves its deadline
  record in the provider heap until that deadline is processed. Replacing distant
  deadlines or rapidly continuing a Watchdog can accumulate cancelled records,
  extra cleanup work and a larger memory high-water extent. The native mock does
  not model provider heap allocation. A maintained churn fixture records this
  subject on real Wasm in 0.9.0. The maintainer reported ten PocketIC subjects
  passing before the ordinary-await correction, but supplied no page measurements
  in that report. This wrapper has no provider-heap compaction authority and
  supplies no global memory cap. See the [pinned provider cancellation source](https://docs.rs/ic-cdk-timers/1.0.0/src/ic_cdk_timers/lib.rs.html#197).
- Recorded recovery evidence uses PocketIC 15.0.0 and `ic-cdk-timers` 1.0.0.
  The 0.13.2 host harness used `ic-testkit` 0.17.3 with pinned PocketIC 16.0.0;
  its complete release gate passed on native macOS 15 Intel and Apple Silicon.
  The released 0.14.19 harness selects Testkit 0.25.1 and audited PocketIC 16.1.0
  artifact bytes. Complete native qualification of that pair is pending;
  artifact integrity alone does not establish recovery behavior. Source-bound
  qualification is recorded in the release guide; pending changes need fresh evidence. A provider or evidence-binary change requires a renewed source and
  recovery audit; older receipts do not qualify the new harness.
- The recorded IcyDB and Canic adoption subjects independently supply exact-0.5.0
  shared-registry evidence, recorded separately from this library's
  owner-local proof. Canic's schema-3 adapter exports timer, instruction, and
  memory observations without a parallel runtime. The historical Toko Miner
  receipt supplies scoped combined-composition evidence for one exact 0.8.0
  dependency graph. Every new dependency combination still requires one final
  Wasm proving one registry, both owners in one inventory, synchronous lifecycle
  reconstruction, IcyDB Watchdog recovery, and continued Canic timer progress.

The frozen [0.3 Patch 1 contract](docs/design/0.3-patch-1-contract.md) defines
the protocol and the
[closeout report](docs/audits/0.3-runtime-evidence-2026-08-13.md) maps every
promotion case to direct evidence. The
[IcyDB adoption record](docs/adoption/icydb.md) and
[Canic adapter contract](docs/adoption/canic.md) identify which additional
claims come from maintained downstream evidence. The
[Toko Miner record](docs/adoption/toko-miner.md) preserves the scoped combined
receipts and the later dependency-graph blocker without qualifying newer
combinations.

## Failure and measurement semantics

The ordinary callback return contract is policy-specific: Once uses
`OnceRunResult` / `OnceDecision`, and AfterCompletion uses
`AfterCompletionRunResult` / `AfterCompletionDecision`. Only the latter permits
configured recurrence. Explicit rescheduling remains legal for Once, and
invariant failures force Stop. The private erased completion boundary still
checks inconsistent policy data; snapshots remain observations. Automated
preparation and maintainer-owned compile-fail, native and PocketIC validation
are scoped in the [callback contract](docs/design/0.5-policy-specific-callback-authority.md#ordinary-callback-results).

Callback starts and completions are deliberately separate. A trap or
instruction exhaustion can prevent all post-run code, so the runtime must not
invent a completion, zero instruction cost, memory-page sample, elapsed
duration, or zero work count. Memory observations report runtime-epoch-local
monotonic Wasm and stable page extents plus observed start-to-end growth, not
exact live bytes, sub-page allocator liveness, or exclusive work attribution.
The accepted instruction interval starts immediately before callback
acceptance and ends after completion processing and any successor binding, so
it includes `ic-timers` runtime work and is not exclusive application-code
attribution. It excludes provider dispatch before entering `ic-timers`, the
provider return/reply tail, page reads, and the post-interval summary write;
the aggregate is not a complete IC-message measurement or sufficient evidence
for the IC message instruction limit. An async ordinary callback's interval
may also include canister activity interleaved while its future is awaiting. A
terminal `RemoveWhenStopped` callback can remove its declaration before the
final measurement is retained; no timer remains from which to observe that
sample. Consumers needing a durable terminal audit receipt must own it outside
the volatile registry; the runtime deliberately keeps no tombstones.

The live watchdog records only that an earlier committed dispatch lacks a
committed completion when a later scheduler retires it. It cannot infer a trap,
instruction exhaustion, delay, or interruption from that fact alone.

All hot-path observation counters and aggregates saturate rather than trap.
Saturation protects timer execution; it does not make a saturated metric exact.
The runtime epoch identifies the reset scope so operators can distinguish a
reset from a genuine lifetime zero.

## Evidence maintenance

The recovery suite is deliberately outside the fast default CI gate. Run
`make pocketic-watchdog` with the pinned PocketIC 16.1.0 binary after changes
to provider binding, registry transitions, lifecycle reconstruction, or
watchdog dispatch. Run `make pocketic-cohorts` after changes that can affect
linked Wasm, instruction cost, or provider-call count. Native mocks remain
necessary for exhaustive state transitions but are never a substitute for IC
commit/rollback evidence.
