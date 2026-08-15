# Observability and Canic parity contract

Status: canonical observations live; IcyDB and downstream Canic adapters validated

## Purpose

`ic-timers` should replace duplicated timer instrumentation, not merely add a
pleasant inventory beside it. Its canonical snapshot is defined as a semantic
superset of the timer information Canic exposes today.

Ordinary and watchdog state, scheduler dispatch, work, stale,
unacknowledged, instruction, and memory-page observations are live. Normally
completed accepted scheduler and work callbacks record IC call-context
instruction deltas plus start/end Wasm and stable memory extents. Trapped and
exhausted work record no sample. A downstream Canic worktree now validates the
real adapter without parallel instrumentation; landing and tagged combined
qualification remain external gates.

This contract describes provider-neutral runtime data. `ic-timers` owns the
identity, counters, measurements, and snapshot semantics. Canic, IcyDB, and
standalone canisters adapt that snapshot into their own Candid DTOs, status
responses, or metric rows. This crate must not depend on Canic's Candid types,
unified metrics DTOs, or presentation conventions.

## Implemented 0.3 decisions

The frozen [0.3 Patch 1 runtime contract](0.3-patch-1-contract.md) replaced two
candidate 0.2 terms when the values became live registry observations:

- a committed watchdog dispatch without committed completion is
  `unacknowledged`, not definitely interrupted or trapped; and
- elapsed callback duration is omitted because IC message time cannot measure
  synchronous work duration truthfully.

The 0.3 runtime also splits scheduler wake-up arms from watchdog work dispatch
arms, and scheduler instruction aggregates from consumer-work instruction
aggregates. Those decisions do not weaken the Canic semantic-superset gate.

## Canonical snapshot

Every declared timer has one registry-constructed canonical snapshot with the
following groups.

| Group | Required content |
| --- | --- |
| Identity | Bounded `owner`, `subsystem`, and `name` labels forming one ordered `TimerIdentity`. |
| Policy | Configured scheduling policy and cadence, plus the latest post-run directive when it differs from the configured policy. |
| Scheduling | Latest requested delay, latest actually armed delay, scheduling mode, next absolute deadline, and any watchdog successor. |
| State | Closed policy-specific state, registration projection, process condition, current generation, and watchdog attempt status. |
| Outcome | Latest classified outcome, work count, last success and failure timestamps, and consecutive expected failures. |
| Counters | Requests, wake-up arms, work dispatch arms, scheduler starts, work starts/completions, classified outcomes, cancellations, stale callbacks, coalescing, and unacknowledged attempts. |
| Performance | Separate sample count, total, latest, and maximum instructions for schedulers and normally completed work; per role, bounded latest start/end Wasm/stable page extents and maximum per-callback growth. |
| Scope | Runtime epoch and start timestamp defining the reset boundary for every counter and aggregate. |

Configured recurrence and callback directives are related but distinct. The
configured policy should distinguish one-shot, after-completion, and watchdog
behavior. The latest directive should retain deadline, retry, immediate
continuation, recurrence, and stop decisions without pretending that a retry
temporarily changes the timer's configured policy.

All identities and enum values must have deterministic ordering. Labels must
be bounded before they enter registry storage or metric labels. The snapshot
must remain portable, but portability does not require a dependency on a
consumer's serialization model.

## Counter semantics

The counter contract distinguishes requests made to the wrapper from effects
performed against `ic-cdk-timers` and from callbacks that actually execute.

| Counter | Required meaning |
| --- | --- |
| `schedule_requests` | Validated schedule or reconciliation requests, including requests later coalesced or satisfied without another platform arm. |
| `wakeups_armed` | Actual ordinary-work or watchdog-scheduler one-shots whose handle ownership committed. |
| `work_dispatched` | Immediate watchdog-work one-shots committed by scheduler messages. |
| `scheduler_started` | Non-stale watchdog scheduler callbacks accepted by generation arbitration. |
| `work_started` | Non-stale consumer-work callbacks that enter logical execution. |
| `work_completed` | Consumer work that returns and commits completion accounting. |
| `succeeded` | Completed callbacks classified as successful work. |
| `no_work` | Completed callbacks that validly find no work to perform. |
| `retryable_failure` | Completed callbacks with an expected failure that permits retry policy. |
| `invariant_failure` | Completed callbacks reporting an unexpected invariant or terminal failure. |
| `cancelled` | Logical registrations or runs ended because cancellation wins arbitration; this is not merely the number of cancel API calls. |
| `stale_wakeups` | Ordinary or scheduler callbacks rejected because their generation no longer owns execution. |
| `stale_work` | Watchdog work callbacks rejected because their generation no longer owns execution. |
| `coalesced` | Scheduling demand merged into existing scheduled or pending work rather than producing another logical run. |
| `unacknowledged` | An older committed watchdog dispatch retired by its successor without a committed completion. |

Every completed callback has exactly one classified completion outcome, so:

```text
work_completed = succeeded + no_work + retryable_failure + invariant_failure
```

`work_started` and `work_completed` must never be collapsed into one execution
count.
Canic currently records a callback start before it can record post-run
instructions. A trap or instruction exhaustion can prevent completion
instrumentation, so preserving both counters is necessary to expose incomplete
work rather than silently treating it as a zero-cost completion.

## Outcomes and functional state

`consecutive_expected_failures` is runtime state, not a presentation-only
metric. It must be stored with the timer entry and available without scanning
or rebuilding the complete snapshot. Registry lookup by `TimerIdentity` should
make it cheap enough for renewal and cycle-top-up decisions.

The outcome taxonomy must define which completed outcomes increment or reset
that streak. At minimum, retryable expected failures increment it; successful
work resets it. The treatment of valid no-work outcomes must be settled before
the 0.2 types are accepted and covered by focused transition tests.

`work_count` is separate from callback starts and completions. One callback may
find no work, perform one unit, or perform a bounded batch, so an adapter must
not infer work count from callback counters.

## Measurements and scope

Instruction aggregates contain sample count, total, latest, and maximum values
for scheduler and work roles. The corresponding memory summaries contain a
saturating sample count, the latest start/end Wasm and stable extents in 64 KiB
pages, and maximum non-negative observed start-to-end growth for each memory.
They never total absolute page counts. Ordinary callbacks may await, so their
sample interval can include interleaved canister activity and is not exclusive
allocation attribution; Watchdog work and scheduler paths are synchronous.
For each role, memory and instruction sample counts advance together on the
same normal-completion record.

Both measurement kinds update only when the measured callback path returns
with a valid end measurement. A missing work completion remains visible through
committed dispatch and later `unacknowledged` observation; the runtime does not
synthesize zero instructions or a memory sample for trapped or exhausted work.
Elapsed IC time is absent because message time is not a truthful synchronous
duration. Page reads bracket the instruction-delta interval from outside, so
memory observation does not change which instructions that aggregate covers.

Within one runtime epoch, Wasm and stable page counts are monotonic
extent/high-water observations. They are not exact live bytes: allocator
liveness within the final page is invisible at this boundary. Consumers
needing a byte-level bound must derive it from their allocator or storage owner
rather than asking `ic-timers` to fabricate one.

The snapshot carries a runtime epoch identifier and epoch start timestamp.
Every counter, timestamp, and aggregate must state whether it is scoped to that
epoch or persisted across epochs. The initial contract uses epoch-scoped
counters so resets after upgrade are explicit to operators and adapters;
persistent scheduling state used for reconstruction is a separate concern.

All arithmetic must define overflow behavior. Hot-path counters and aggregates
must not trap because an operator metric reached its numeric limit.

## Claim-scoped wake-up observation

Each policy-specific registration capability exposes `has_armed_wakeup()`.
It returns whether that exact current claim owns the registry's future
provider wake-up handle. It does not derive liveness from snapshot fields. For
a watchdog, the separately queued immediate work handle does not count; the
already committed cadence successor does. For ordinary work running before a
successor is installed, the result is false.

This helper is an inert volatile observation, not a check-then-arm protocol,
durable authority, or a guarantee that provider delivery will occur.
Consumers whose durable authority requires future work must call the
idempotent ensure operation unconditionally. An expired remove-on-stop claim
returns `RegistrationExpired` rather than observing a later registration with
the same identity.

## Implemented value decisions

The runtime makes the following choices. Downstream adapters may continue to
provide feedback as the pre-1.0 API evolves:

- Each identity component is non-empty, limited to 64 UTF-8 bytes, and rejects
  surrounding whitespace and control characters. Exact label text is retained
  and ordered lexicographically by owner, subsystem, then name.
- Portable process conditions preserve Canic's disabled, idle, active,
  retrying, and failed states. Completion outcomes preserve success, no-work,
  retryable-failure, and invariant-failure classes. `Unacknowledged` is a
  separate non-completion event with unknown, rather than synthetic zero, work
  and performance measurements.
- Success, valid no-work, and invariant failure reset
  `consecutive_expected_failures`; a retryable failure increments it and an
  interruption preserves it. This matches current Canic recovery-state
  transitions.
- An unacknowledged attempt is counted in the epoch and scheduler message that
  retires its committed dispatch. It has no arithmetic invariant with
  `work_started`, because a trapping work-message start mutation rolls back.
- Counts, work, instructions, page extents, and nanosecond values use `u64`.
  Hot-path counters, streaks, sample counts, and instruction totals saturate;
  latest and maximum measurements continue to update after count or total
  saturation. Absolute memory extents have no total.
- The crate exposes provider-neutral Rust values and stable enum labels but no
  Candid or Serde contract. Consumers own serialization adapters. Public API
  evolution follows crate SemVer rather than a consumer's wire format.

## Canic parity baseline

The contract covers the following Canic operator information without parallel
timer instrumentation:

| Current Canic surface | Required projection from `ic-timers` |
| --- | --- |
| Timer executions and latest delay in `crates/canic-core/src/ops/runtime/metrics/timer.rs` | Started count, configured cadence, latest requested and armed delays, and next deadline. |
| Completed count and total instructions in `crates/canic-core/src/ops/runtime/perf.rs` | Completed count and total, latest, and maximum instructions. |
| Detailed state in `crates/canic-core/src/dto/runtime.rs` | One canonical snapshot containing semantically equivalent identity, policy, state, outcome, and timing fields. |
| Former test-only global `TimerScheduled` count | No public projection requirement; remove it rather than retain parallel instrumentation. Canonical requested and armed counters remain separately observable. |

The compatibility requirement is semantic rather than type-level. The
validated Canic worktree advances its runtime introspection schema to version
2 and derives the maintained fields from `ic-timers` data alone.

The exact existing-field projection is now frozen:

| Canic field | Canonical source |
| --- | --- |
| schedules | `wakeups_armed` |
| executions | `work_started` |
| successes | saturating `succeeded + no_work` |
| expected failures | `retryable_failure` |
| invariant failures | `invariant_failure` |
| stale callbacks | saturating `stale_wakeups + stale_work` |
| latest delay | `latest_armed_delay_ns` converted to milliseconds |
| generation | `TimerSnapshot::generation()` as `Option<u64>` |
| completed instruction count | work-instruction sample count |
| total/latest/maximum instructions | matching work-instruction aggregate |

`schedule_requests` is intentionally not the legacy schedule count: it also
includes coalesced demand that did not commit a provider arm. The validated
Canic worktree hard-cuts its unconditional `generation: u64` to `Option<u64>`
so inactive declarations do not fabricate generation zero.

The removed global `TimerScheduled` counter was a test-only implementation
detail, not a public Canic metric or status field. Keeping it would have
created misleading duplicate instrumentation. The per-timer `schedules` field
is the operator surface that retains the committed-arm meaning.

## Acceptance criteria

The accepted contract requires all of the following:

1. For every Canic timer, the `ic-timers` snapshot is a semantic superset of
   the existing timer status, timer counter, scheduling counter, and timer
   instruction metrics.
2. Focused Canic adapter tests demonstrate projection of the existing operator
   surfaces without separate timer instrumentation.
3. Unit tests fix the meanings and invariants of every counter, including
   request-versus-arm and start-versus-completion cases.
4. Outcome transition tests fix the increment and reset behavior of
   `consecutive_expected_failures`.
5. Snapshot ordering, label bounds, epoch reset semantics, numeric saturation,
   and measurement aggregation are explicit and tested.
6. `ic-timers` has no dependency on Canic-specific DTO, Candid, or metric-row
   types.

Trap, instruction-exhaustion, and upgrade behavior has focused PocketIC
evidence. The inspected downstream Canic worktree satisfies the real-adapter
criterion and removes its separate `TimerMetrics`, timer-specific performance
storage, duplicated workflow counters, and direct provider path.

The local tests include a Canic-shaped projection fixture proving the fields
are available. Canic's maintained downstream status reports focused adapter,
lifecycle, inventory, protocol, and PocketIC timer evidence passing. That
worktree is still uncommitted. The current Canic and IcyDB development
worktrees both resolve exact 0.3.8; tagged combined qualification must still
prove one package in the final Wasm.
