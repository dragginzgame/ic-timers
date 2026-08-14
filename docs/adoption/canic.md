# Canic adapter contract

Status: downstream contract updated for the unreleased 0.3.2 candidate; Canic
has not adopted `ic-timers`

## Boundary

Canic must replace its timer provider, `TIMERS` map, `TimerControl`, provider
handles, timer counters, and timer-specific performance accounting in one
pre-1.0 hard cut. It must not leave those paths beside `ic-timers` as a
fallback or compatibility facade.

The final canister must resolve exactly one `ic-timers` Cargo package ID. Two
resolved versions create two library statics and therefore two registries.
Before migration, inventory the complete linked canister for direct
`ic-cdk-timers` users; its 250 outstanding-call limit is canister-wide.

Canic may own one bounded custody collection whose values are opaque
`OnceRegistration` or `AfterCompletionRegistration` capabilities. This is not
a second timer registry: it contains no scheduling state, counters, provider
handles, generations, pending commands, reconciliation state, or snapshot
copies. It exists only so Canic can enumerate and control claims it owns.
Its capacity cannot exceed `MAX_TIMER_REGISTRATIONS`.

## Identity and policy mapping

Each `TimerKey` maps to one fixed `TimerIdentity`:

| Canic key | `owner` | `subsystem` | `name` | Policy | Lifetime |
| --- | --- | --- | --- | --- | --- |
| `AuthRenewal` | `canic` | `auth_renewal` | `run` | `Once` | `Retained` |
| `CycleTopup` | `canic` | `cycles` | `topup` | `Once` | `Retained` |
| `IntentCleanup` | `canic` | `intent_cleanup` | `run` | `Once` | `Retained` |
| `LogRetention` | `canic` | `log_retention` | `run` | `Once` | `Retained` |
| `PlacementReceiptAcknowledgement` | `canic` | `placement` | `receipt_ack` | `Once` | `Retained` |

Application timers use owner `canic`, subsystem `application-{id}`, and the
validated caller label as their name. The checked monotonic allocation ID keeps
duplicate human labels distinct without another registry. The current 96-byte
public name allowance must hard-cut to `MAX_TIMER_LABEL_BYTES` (64 UTF-8
bytes), and invalid labels, exhausted IDs, duplicate identities, and exhausted
capacity must be returned as typed errors. Do not truncate, hash silently,
evict another timer, or allocate a second unbounded identity table.

The complete policy inventory is:

| Timer family | Identity | Policy | Lifetime |
| --- | --- | --- | --- |
| Five dynamic Canic built-ins above | fixed table identity | `Once` | `Retained` |
| Root canister-pool maintenance | `canic:canister_pool:maintain` | `AfterCompletion` | `Retained` |
| Public one-shots | allocated application identity | `Once` | `RemoveWhenStopped` |
| Public intervals | allocated application identity | `AfterCompletion` | `RemoveWhenStopped` |
| Lifecycle deferrals | allocated lifecycle identity | `Once` | `RemoveWhenStopped` |

The five dynamic built-ins do not own a genuine fixed cadence. Their normal
completion selects `ScheduleAt`, `RetryAfter`, `ContinueImmediately`, or
`Stop`, so fabricating an `AfterCompletion` cadence would make policy snapshots
untruthful. The canister-pool maintenance loop does own a real fixed cadence.
No current Canic timer uses `Watchdog`; it remains reserved for bounded
synchronous work whose recovery contract genuinely requires a committed
successor before consumer work.

## Operation mapping

Canic initializes the runtime once from its existing lifecycle owner before
framework and application hooks. It declares every fixed retained timer before
application hooks, even when its durable authority currently selects inactive.
The 0.3.2 reconciliation helpers install fresh inactive declarations, making
the inventory complete and reserving critical registry capacity. Canic retains
one policy-specific registration claim per timer and delegates as follows:

| Current Canic operation | `ic-timers` operation |
| --- | --- |
| earliest `schedule` / `schedule_at` | `OnceRegistration::ensure_scheduled` or the appropriate callback directive |
| authoritative `reconcile_at(Some(deadline))` | ordinary registration `reconcile_schedule(Some(TimerSchedule::At(deadline)))` |
| authoritative `reconcile_at(None)` | ordinary registration `reconcile_schedule(None)` |
| lifecycle reconstruction of one-shot work | `reconcile_once` |
| lifecycle reconstruction of recurrence | `reconcile_after_completion` |
| cancel and forget application handle | remove its custody entry and consume the claim with `unregister` |
| status, metrics, instruction totals, failure streak | project `timer_snapshot(s)` and `consecutive_expected_failures` |

`ensure_scheduled` retains an already scheduled earlier deadline;
`reconcile_schedule` is authoritative and may move it in either direction.
The canonical registry is the only pending-command machine. Canic must not
port its `TimerControl` or pending reconciliation state into the adapter.

The public `TimerApi` must become fallible before migration. `set`,
`defer_lifecycle`, and `set_interval` return `Result<TimerHandle, TimerError>`;
consuming cancellation returns `Result<(), TimerError>`. Macro expansion and
all direct callers must handle those errors explicitly. The public
`TimerHandle` carries an opaque key into the bounded Canic claim-custody
collection, never a copied provider ID or a second copy of registration state.

## Exact metrics projection

Canic projects its existing operator fields from one `TimerSnapshot` as
follows. Additions to the canonical snapshot remain available separately; they
must not be substituted for these established meanings.

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

Canic's global `TimerScheduled` counter likewise projects actual committed
ordinary/scheduler arms, not `schedule_requests`. The latter includes
coalesced demand and remains a distinct, additional operator signal. The Canic
DTO must hard-cut `generation: u64` to `generation: Option<u64>`; an inactive
timer has no generation and must not invent zero.

## Suspension and lifecycle composition

`ic-timers` intentionally has no global suspend/resume operation. In a shared
registry, a Canic global switch would also suspend IcyDB-owned timers. Canic
must iterate its bounded claim-custody collection:

1. use inert snapshots only to reject a currently running Canic claim;
2. cancel fixed retained claims through their policy-specific capability;
3. remove transient application and lifecycle claims from custody and consume
   them with `unregister`;
4. on resume, rerun the existing Canic domain reconcilers against their durable
   authority and retained claims; and
5. require owners to register transient application timers again if still
   desired after the authority-snapshot fence.

This custody collection is composition, not a second timer state machine.
Canic must not restore provider handles, generations, deadlines, counters,
pending commands, or snapshot values as mutation authority. A typed
owner/group capability is therefore unnecessary for 0.3.2; if later required,
it is new public functionality for a 0.4.0 design, not a patch addition or
permission to retain Canic's parallel registry.

## Adoption gate

Adoption is complete only when focused Canic tests prove the existing status
and metric rows derive from `ic-timers` snapshots without parallel timer
instrumentation; every configured fixed built-in, including canister-pool
maintenance, appears in the canonical inventory before its first schedule;
authority-snapshot quiescence cancels or unregisters every Canic-owned claim
without affecting another owner; lifecycle order is preserved; capacity,
identity, and provider errors are typed; direct provider use is removed from
production; and `cargo tree -d` shows one resolved `ic-timers` package ID.
