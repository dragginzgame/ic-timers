# Canic adapter contract

Status: validated downstream adoption worktree; Canic release remains pending.

## Inspected downstream adoption

The uncommitted Canic worktree inspected read-only on 2026-08-15 is based on
tagged `v0.102.1` commit
`86763c5f16478e2e548e2059e5efaa963bf9a966` and resolves exact
`ic-timers = "=0.3.8"`. It implements the hard cut described below:

- Canic's direct `ic-cdk-timers` dependency, provider wrapper, `TIMERS` map,
  `TimerControl`, timer metrics table, and timer-specific performance storage
  are removed;
- runtime status and metrics project the complete shared `ic-timers`
  inventory, including other framework and application owners;
- application timer creation and consuming cancellation are fallible, and
  macro callers must propagate or explicitly handle failure;
- runtime introspection advances to schema version 2;
- one bounded Canic custody collection retains only opaque registration
  capabilities; and
- runtime metrics obtain the inventory once per request and derive both timer
  and timer-performance rows from that same snapshot vector.

The move from exact 0.3.6 to 0.3.8 required no adapter change. Canic's
maintained status reports affected package checks, strict targeted Clippy,
inventory and lifecycle guards, three adapter unit tests, and PocketIC
cancellation, recurrence, and upgrade reconstruction passing. It also reports
one resolved 0.3.8 package and no direct provider edge. This repository did not
rerun those downstream suites, and the adoption batch has no numerical
before/after performance benchmark. Landing, versioning, and release remain
Canic-owned.

## Boundary

The inspected worktree replaces Canic's timer provider, `TIMERS` map,
`TimerControl`, provider handles, timer counters, and timer-specific
performance accounting in one pre-1.0 hard cut. Landing must not restore those
paths beside `ic-timers` as a fallback or compatibility facade.

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

`OnceContext` and `AfterCompletionContext` are not custody values. Each
delegates only policy-valid control while its exact callback generation is
running and expires on completion, so Canic must not retain one as a
substitute for the policy-specific registration claim. Canic currently owns no
Watchdog callback.

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
duplicate human labels distinct without another registry. The former 96-byte
public name allowance hard-cuts to `MAX_TIMER_IDENTITY_COMPONENT_BYTES` (64
UTF-8 bytes), and invalid components, exhausted IDs, duplicate identities, and
exhausted capacity must be returned as typed errors. Do not truncate, hash
silently, evict another timer, or allocate a second unbounded identity table.

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

Shared-registry initialization and declaration of Canic-owned work are distinct
operations. Every participating canister initializes the runtime once from its
existing lifecycle owner before framework and application hooks. A canister
declares only fixed retained timers it genuinely owns, even when durable
authority currently selects them inactive. The root control plane also declares
canister-pool maintenance before application hooks. Fleet Coordinator
initializes the empty shared registry but does not invent unrelated inactive
Canic jobs.

Retained-only lifecycle reconciliation installs fresh inactive declarations
for genuine owners, making their inventory complete and reserving critical
capacity. Canic retains one policy-specific registration claim per declared
timer and delegates as follows:

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
Lifecycle reconciliation always creates `Retained` declarations. Public and
lifecycle timers mapped to `RemoveWhenStopped` use direct registration and must
not be stored in a reconciliation slot after removal.
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

Canic's former global `TimerScheduled` counter was test-only and was not part of
its public status or metric projection. The adoption correctly removes it
instead of preserving parallel instrumentation. The public per-timer
`schedules` field maps to actual committed `wakeups_armed`, not
`schedule_requests`; the latter includes coalesced demand and remains a
distinct canonical signal. The Canic DTO hard-cuts `generation: u64` to
`generation: Option<u64>`; an inactive timer has no generation and must not
invent zero.

The open 0.5 snapshot adds scheduler/work memory-page summaries without
changing this legacy projection. If Canic exposes them as new rows, it must
label start/end extents and observed growth in 64 KiB pages. It must not sum
absolute extents, convert them into an exact live-byte claim, or attribute an
async ordinary interval exclusively to timer work.

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
owner/group capability is therefore unnecessary for this contract; if later
required, it is new public functionality for a 0.4.0 design, not a patch
addition or permission to retain Canic's parallel registry.

## Adoption gate

The inspected worktree satisfies the Canic-side adapter gate: status and metric
rows derive from one shared snapshot scan without parallel timer
instrumentation; genuine fixed owners appear before their first schedule;
authority-snapshot quiescence acts only on Canic-owned claims; lifecycle order
is preserved; timer errors are typed; direct provider use is removed; and the
workspace resolves one exact `ic-timers` 0.3.8 package.

Two gates remain. First, Canic must land and release the validated worktree.
Second, tagged combined Canic+IcyDB qualification must prove one exact package
in the final Wasm. The current uncommitted Canic and IcyDB worktrees both pin
0.3.8, so development-time alignment is complete. Their tagged releases do not
yet provide a combined one-package subject, and no released-composition claim
is made.
