# Observability and Canic parity contract

Status: candidate value types implemented for Canic and IcyDB feedback

## Purpose

`ic-timers` should replace duplicated timer instrumentation, not merely add a
pleasant inventory beside it. Before the registry or metrics collection is
implemented, its canonical snapshot must be defined as a semantic superset of
the timer information Canic exposes today.

This contract describes provider-neutral runtime data. `ic-timers` owns the
identity, counters, measurements, and snapshot semantics. Canic, IcyDB, and
standalone canisters adapt that snapshot into their own Candid DTOs, status
responses, or metric rows. This crate must not depend on Canic's Candid types,
unified metrics DTOs, or presentation conventions.

## Canonical snapshot

Every declared timer has one canonical snapshot with the following groups.
Names below describe required semantics; the 0.2 Rust API may refine the exact
type and field names.

| Group | Required content |
| --- | --- |
| Identity | Bounded `owner`, `subsystem`, and `name` labels forming one ordered `TimerIdentity`. |
| Policy | Configured scheduling policy and cadence, plus the latest post-run directive when it differs from the configured policy. |
| Scheduling | Latest requested delay, latest actually armed delay, next absolute deadline, and any pre-armed successor. |
| State | Enabled state, registration, process condition, current generation, and whether work is in flight. |
| Outcome | Latest classified outcome, work count, last success and failure timestamps, and consecutive expected failures. |
| Counters | Requests, arms, starts, completions, classified outcomes, cancellations, stale callbacks, coalescing, and observed interruptions. |
| Performance | Total, latest, and maximum instruction consumption and elapsed callback duration. |
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
| `requested` | Validated schedule or reconciliation requests, including requests later coalesced or satisfied without another platform arm. |
| `armed` | Actual one-shot arm operations sent to the platform, including replacements and pre-armed watchdog successors. |
| `started` | Non-stale callbacks that win generation arbitration and enter logical execution. |
| `completed` | Started callbacks that return and commit completion accounting. |
| `succeeded` | Completed callbacks classified as successful work. |
| `no_work` | Completed callbacks that validly find no work to perform. |
| `retryable_failure` | Completed callbacks with an expected failure that permits retry policy. |
| `invariant_failure` | Completed callbacks reporting an unexpected invariant or terminal failure. |
| `cancelled` | Logical registrations or runs ended because cancellation wins arbitration; this is not merely the number of cancel API calls. |
| `stale` | Provider callbacks or completions rejected because their generation no longer owns execution. |
| `coalesced` | Scheduling demand merged into existing scheduled or pending work rather than producing another logical run. |
| `interrupted` | A started generation later observed to be unable to complete, for example during watchdog takeover or lifecycle reconstruction. |

Every completed callback has exactly one classified completion outcome, so:

```text
completed = succeeded + no_work + retryable_failure + invariant_failure
```

An actively running callback is not interrupted merely because `started` is
temporarily greater than `completed`. `interrupted` changes only when the
runtime can establish that the prior generation will not complete. The design
must define how reconstruction attributes an interruption that began in a
previous epoch.

`started` and `completed` must never be collapsed into one execution count.
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

Instruction and elapsed-duration aggregates contain total, latest, and
maximum values. They update only for completed callbacks with a valid end
measurement. A missing completion remains visible through state and counters;
the runtime must not synthesize a zero measurement for trapped or exhausted
work.

The snapshot carries a runtime epoch identifier and epoch start timestamp.
Every counter, timestamp, and aggregate must state whether it is scoped to that
epoch or persisted across epochs. The initial contract uses epoch-scoped
counters so resets after upgrade are explicit to operators and adapters;
persistent scheduling state used for reconstruction is a separate concern.

All arithmetic must define overflow behavior. Hot-path counters and aggregates
must not trap because an operator metric reached its numeric limit.

## Candidate decisions for review

The first 0.2 implementation makes the following choices. They remain open to
Canic and IcyDB feedback until the 0.2 API is accepted:

- Each identity component is non-empty, limited to 64 UTF-8 bytes, and rejects
  surrounding whitespace and control characters. Exact label text is retained
  and ordered lexicographically by owner, subsystem, then name.
- Portable process conditions preserve Canic's disabled, idle, active,
  retrying, failed, and missing-registration states. Completion outcomes
  preserve success, no-work, retryable-failure, and invariant-failure classes;
  an interruption is a separate non-completion terminal event with unknown,
  rather than synthetic zero, work and performance measurements.
- Success, valid no-work, and invariant failure reset
  `consecutive_expected_failures`; a retryable failure increments it and an
  interruption preserves it. This matches current Canic recovery-state
  transitions.
- An interruption is counted in the epoch where recovery or reconstruction
  establishes it, even if the interrupted generation started in an earlier
  epoch. It therefore has no arithmetic invariant with the observing epoch's
  start counter.
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

Trap, instruction-exhaustion, and upgrade cases require later PocketIC
evidence. The 0.2 types and transition tests must nevertheless reserve and
define the state needed to represent those cases before runtime wiring starts.

Only after the adapters pass may Canic remove its separate `TimerMetrics`,
timer-specific `PerfKey`, and duplicated workflow counters.

The local 0.2 tests include a Canic-shaped projection fixture proving the
candidate fields are available. Acceptance criterion 2 remains open until the
real downstream Canic adapter tests pass.
