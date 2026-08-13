# Observability and Canic parity contract

Status: canonical runtime observations live; downstream Canic adapter pending

## Purpose

`ic-timers` should replace duplicated timer instrumentation, not merely add a
pleasant inventory beside it. Its canonical snapshot is defined as a semantic
superset of the timer information Canic exposes today.

Ordinary and watchdog state, scheduler dispatch, work, stale,
unacknowledged, and instruction observations are live. Normally completed
accepted scheduler and work callbacks record IC call-context instruction
deltas. Trapped and exhausted work record no sample. The real downstream Canic
adapter gate remains open.

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
| Performance | Separate sample count, total, latest, and maximum instructions for schedulers and normally completed work. |
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
for scheduler and work roles. They update only when the measured callback path
returns with a valid end measurement. A missing work completion remains visible
through committed dispatch and later `unacknowledged` observation; the runtime
does not synthesize a zero measurement for trapped or exhausted work. Elapsed
IC time is absent because message time is not a truthful synchronous duration.

The snapshot carries a runtime epoch identifier and epoch start timestamp.
Every counter, timestamp, and aggregate must state whether it is scoped to that
epoch or persisted across epochs. The initial contract uses epoch-scoped
counters so resets after upgrade are explicit to operators and adapters;
persistent scheduling state used for reconstruction is a separate concern.

All arithmetic must define overflow behavior. Hot-path counters and aggregates
must not trap because an operator metric reached its numeric limit.

## Implemented value decisions

The runtime makes the following choices. Canic and IcyDB may still provide
downstream adapter feedback before 0.3 is released:

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
- Counts, work, instructions, and nanosecond values use `u64`. Hot-path
  counters, streaks, sample counts, and totals saturate; latest and maximum
  measurements continue to update after total saturation.
- The crate exposes provider-neutral Rust values and stable enum labels but no
  Candid or Serde contract. Consumers own serialization adapters. Public API
  evolution follows crate SemVer rather than a consumer's wire format.

## Canic parity baseline

The 0.2 contract must cover the following Canic operator information without
parallel timer instrumentation:

| Current Canic surface | Required projection from `ic-timers` |
| --- | --- |
| Timer executions and latest delay in `crates/canic-core/src/ops/runtime/metrics/timer.rs` | Started count, configured cadence, latest requested and armed delays, and next deadline. |
| Completed count and total instructions in `crates/canic-core/src/ops/runtime/perf.rs` | Completed count and total, latest, and maximum instructions. |
| Detailed state in `crates/canic-core/src/dto/runtime.rs` | One canonical snapshot containing semantically equivalent identity, policy, state, outcome, and timing fields. |
| Global `TimerScheduled` count | Separate requested and actually armed counters, so coalescing and replacement are visible. |

The compatibility requirement is semantic rather than type-level. Canic may
keep its public DTO shape during migration, but its adapter must be able to
derive every existing field from `ic-timers` data alone.

## Acceptance criteria

The design slice is not accepted until all of the following are true:

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

Trap, instruction-exhaustion, and upgrade behavior now has focused PocketIC
evidence. The remaining acceptance gap is the real Canic adapter, not missing
runtime observation state.

Only after the adapters pass may Canic remove its separate `TimerMetrics`,
timer-specific `PerfKey`, and duplicated workflow counters.

The local tests include a Canic-shaped projection fixture proving the fields
are available. Acceptance criterion 2 remains open until the
real downstream Canic adapter tests pass.
