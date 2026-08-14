# Canic adapter contract

Status: downstream contract for the unreleased 0.3.1 candidate; Canic has not
adopted `ic-timers`

## Boundary

Canic must replace its timer provider, `TIMERS` map, `TimerControl`, provider
handles, timer counters, and timer-specific performance accounting in one
pre-1.0 hard cut. It must not leave those paths beside `ic-timers` as a
fallback or compatibility facade.

The final canister must resolve exactly one `ic-timers` Cargo package ID. Two
resolved versions create two library statics and therefore two registries.
Before migration, inventory the complete linked canister for direct
`ic-cdk-timers` users; its 250 outstanding-call limit is canister-wide.

## Identity and policy mapping

Each `TimerKey` maps to one fixed `TimerIdentity`:

| Canic key | `owner` | `subsystem` | `name` |
| --- | --- | --- | --- |
| `AuthRenewal` | `canic` | `auth_renewal` | `run` |
| `CycleTopup` | `canic` | `cycles` | `topup` |
| `IntentCleanup` | `canic` | `intent_cleanup` | `run` |
| `LogRetention` | `canic` | `log_retention` | `run` |
| `PlacementReceiptAcknowledgement` | `canic` | `placement` | `receipt_ack` |

Application timers use owner `canic`, subsystem `application-{id}`, and the
validated caller label as their name. The checked monotonic allocation ID keeps
duplicate human labels distinct without another registry. The current 96-byte
public name allowance must hard-cut to `MAX_TIMER_LABEL_BYTES` (64 UTF-8
bytes), and invalid labels, exhausted IDs, duplicate identities, and exhausted
capacity must be returned as typed errors. Do not truncate, hash silently,
evict another timer, or allocate a second unbounded identity table.

Application one-shots map to `Once` with `RemoveWhenStopped`; application
intervals map to `AfterCompletion` with `RemoveWhenStopped`. Canic built-ins
are retained declarations. Log retention and cycle top-up belong on the
ordinary `AfterCompletion` path and may continue to return explicit
deadlines, retries, or immediate continuation; they must not use `Watchdog`.
A real positive domain cadence must be supplied rather than a dummy value.
No current Canic built-in needs watchdog pre-arming or its extra provider
dispatch. `Watchdog` remains for bounded synchronous work whose recovery
contract genuinely requires a committed successor before consumer work.

## Operation mapping

Canic initializes the runtime once from its existing lifecycle owner before
framework and application hooks. It retains one policy-specific registration
claim per timer and delegates as follows:

| Current Canic operation | `ic-timers` operation |
| --- | --- |
| earliest `schedule` / `schedule_at` | `OnceRegistration::ensure_scheduled` or the appropriate callback directive |
| authoritative `reconcile_at(Some(deadline))` | ordinary registration `reconcile_schedule(Some(TimerSchedule::At(deadline)))` |
| authoritative `reconcile_at(None)` | ordinary registration `reconcile_schedule(None)` |
| lifecycle reconstruction of one-shot work | `reconcile_once` |
| lifecycle reconstruction of recurrence | `reconcile_after_completion` |
| cancel and forget application handle | consume the claim with `unregister` |
| status, metrics, instruction totals, failure streak | project `timer_snapshot(s)` and `consecutive_expected_failures` |

`ensure_scheduled` retains an already scheduled earlier deadline;
`reconcile_schedule` is authoritative and may move it in either direction.
The canonical registry is the only pending-command machine. Canic must not
port its `TimerControl` or pending reconciliation state into the adapter.

The public `TimerApi` must become fallible before migration. `set`,
`defer_lifecycle`, and `set_interval` return `Result<TimerHandle, TimerError>`;
consuming cancellation returns `Result<(), TimerError>`. Macro expansion and
all direct callers must handle those errors explicitly. The sole-owner
`TimerHandle` wraps one `OnceRegistration` or `AfterCompletionRegistration`,
never a copied provider ID.

## Suspension and lifecycle composition

`ic-timers` intentionally has no global suspend/resume operation. In a shared
registry, a Canic global switch would also suspend IcyDB and application-owned
timers. Canic must iterate only its retained claims:

1. use inert snapshots only to reject a currently running Canic claim;
2. cancel each Canic-owned claim through its policy-specific capability;
3. on resume, rerun the existing Canic domain reconcilers against their durable
   authority and retained claims; and
4. define application timers as cancelled across the authority-snapshot fence
   unless their application owner explicitly registers them again.

This is composition, not a second timer state machine. Canic must not restore
provider handles, generations, pending commands, or snapshot values as
mutation authority. If exact resumption of transient application deadlines is
required instead, that is a new claim-level pause-token design for
`ic-timers`, not permission to retain Canic's parallel registry.

## Adoption gate

Adoption is complete only when focused Canic tests prove the existing status
and metric rows derive from `ic-timers` snapshots without parallel timer
instrumentation; lifecycle order is preserved; capacity, identity, and
provider errors are typed; direct provider use is removed from production;
and `cargo tree -d` shows one resolved `ic-timers` package ID.
