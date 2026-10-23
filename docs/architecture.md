![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Architecture

This document is a technical reference for developers working on or
integrating IC Timers. For a general introduction, start with the
[project README](../README.md).

## Overview

The application owns the lasting record of what work needs to happen. IC
Timers owns the temporary scheduling state used while the canister is running.
After an upgrade, the application uses its lasting state to rebuild the timers
it still needs.

A registration capability lets one owner control one named timer. The runtime
coordinates callbacks and scheduling, while one shared registry holds timer
names, states, counters, callbacks, and provider handles. Only the private
platform boundary owns one-shot IC dispatch and integrates the selected executor.

![Ownership and control flow from durable application state through the IC Timers runtime to its private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-runtime-ownership.svg)

The sections below describe these boundaries in implementation-level detail.

## Capability ownership

The published [crate facade](../crates/ic-timers/src/lib.rs) exports registration,
reconciliation, callback contexts, checked schedules and inert snapshots. Its
implementation modules and platform handles are private. The Cargo `links` key
rejects competing packages with the same global-timer ownership key; complete
canister graphs must still exclude older providers that lack that key.

| Capability | Canonical owner | Local integration |
| --- | --- | --- |
| Volatile scheduling, claim/generation authority, policy arbitration and timer observations | `crates/ic-timers/src/{runtime,registry,control,schedule,snapshot}` | Public crate facade |
| One-shot dispatch, pending queue and authenticated IC callbacks | Private `crates/ic-timers/src/platform`; `ic-platform` exposes the shared CDK executor for future polling and call-context cleanup | Only `platform/ffi.rs` owns unsafe exports/callback registration |
| Measurement arithmetic | `ic-metrics` | Timer-specific event names, recording/sample boundaries and memory extents in `snapshot/metrics.rs` |
| Durable jobs, readiness, lifecycle order and operator-health labels | Consuming application/framework | Registration claims and snapshots; no durable task database or application lifecycle exports here |
| Wasm compilation and artifact-cache custody | `ic-build`, with explicit child custody from `ic-host-process` | `testing/crates/ic-timers-pocketic/src/build/mod.rs` selects packages, profile, features and artifact lifetime |
| PocketIC client types and raw call transport | `ic-platform` | Native tests import `pocket_ic` directly and retain bounded Wire requests |
| PocketIC server selection, admission, lifecycle and bounded progress | `ic-testkit` | `scripts/dev/testkit-server.sh` selects the root-lock CLI; the native harness supplies instance configuration |
| Controlled probe codecs | Published `ic-wire::native` and Serde | The unpublished protocol owns method identities/limits; Platform hooks and the native harness own transport integration |
| Common tools, fixture dependency closure, evidence transport and release mechanics | Reviewed Shared Tooling snapshot | Local Make selections, product release/source policy and evidence identity; see [adoption boundaries](shared-tooling.md) |

The [root manifest](../Cargo.toml) owns all five maintained packages and every
direct dependency, with one lockfile. The four unpublished packages own runtime
and size probe canisters, the native PocketIC driver, and shared probe wire
records. The protocol package selects Wire and Serde only: it carries values,
not scheduling authority. The native platform substitute is test-only; it does
not replace IC execution or prove recovery. Runtime subjects and policy cohorts
remain separate evidence selections in the [release guide](releasing.md).

## Candid and CDK dependency boundary

Registry-published `ic-platform` is the selected boundary; the current lock and
source-qualified receipts are recorded in [current status](status/current.md).
Published Platform 0.6 owns the maintained non-cryptographic runtime and
explicit-codec generator. It selects no full CDK package and copies no executor.
The root catalog disables default Platform features. The library selects only
`executor` and `ic0`; the two canister probes select `cdk` (including its new
`runtime` feature) for entrypoint contexts,
protected calls/spawning, traps, cycle burning and stable byte I/O. The native
harness selects `pocket-ic` explicitly, including Principal support, and uses
`principal::ExternalPrincipal`
for PocketIC's external API identity. The protocol package has no Platform edge.
Parser, Agent, certification and ledger capabilities remain unselected.

The private platform uses `ic_platform::ic0` for its existing system calls,
retaining context placement, method handles, protected spawning and callback/trap cleanup. The lockfile still
resolves one `ic-cdk-executor` 2.0.0 shared with Platform's runtime. Native substitutes cannot
qualify those IC paths. The library's executor/system-only graph excludes
Principal/Account, Serde, SHA-2, text encoding and Candid. Probe entrypoint/runtime
features imply `principal`, retaining their existing identities and behavior.
None of those identities cross the published timer API or probe wire records;
this adoption changes no stored data or codec representation.

Probe records use Serde and published Wire's native codec. The shared
`test-protocol::rpc` roster owns explicit method IDs; both endpoint hooks and
native callers bind to those identities. Request limits admit at most 1 KiB,
16 KiB allocation charge, 1,024 nodes and depth 16; replies retain a 64 KiB byte
ceiling with a 1 MiB allocation charge, 16,384 nodes and depth 32. These are Wire
budgets, not renamed Candid quotas. Pre-copy guards check incoming IC size before
Platform copies arguments. Decode failure prevents the handler; reply encoding
failure traps rather than committing a partial response. Empty RPCs still carry
a method-bound frame. Init and upgrade hooks take raw empty arguments.

Platform's explicit-codec entrypoints retain query/update contexts and protected
spawning; redundant `hidden` flags are removed. The two private canister
adapters translate codec rejection to traps; they own no serialization arithmetic
or timer authority. Real self-call gates and queue pressure use raw Wire frames,
retaining their IC suspension and call-failure behavior. The production dispatch
remains its authenticated eight-byte payload, without a Wire envelope.

Platform owns the exact `ic-cdk-executor = "=2.0.0"` and `ic0 = "=1.2.0"`
constraints. Their direct declarations and local pin exceptions are removed.
The executor/IC0 exports preserve upstream identities and safe/unsafe signatures;
all callback registration remains in the private FFI owner. This completes the
published boundaries requested in [Platform #7](https://github.com/dragginzgame/ic-platform/issues/7)
and [#8](https://github.com/dragginzgame/ic-platform/issues/8) without a local facade.

The runtime fixture's four-field `(bool, bool, u64, u64)` checkpoint now uses a
bounded Wire frame with a checked little-endian length prefix in its existing
stable-memory fixture page. Both upgrade participants use this one format;
there is no Candid reader or cross-release probe migration. Database/application
storage is outside this fixture. Stable byte I/O and cycle burning remain
Platform's maintained runtime adapters, originally adopted under
[Platform #19 — Timers stable-helper and cycle-burn boundary](https://github.com/dragginzgame/ic-platform/issues/19).

No workspace member declares Candid or any CDK-family package directly.
The library and both qualification canisters have no Candid or full CDK/macros
in their normal/build graphs. Probe graphs retain Platform's generator and the
shared executor; only the canisters select `cdk`. Raw counter/page observations
use Platform's exact IC0 exports. Its maintained spawning adapter preserves the
unfinished-task drop policy. Call/error/stable-error types now belong to Platform;
no such runtime types cross the published timer API or the probe wire boundary.

The native driver retains published Testkit for server admission, lifecycle and
bounded progress. PocketIC clients/DTOs are imported directly from Platform;
Testkit no longer forwards them. Probe RPCs use raw query/update APIs plus the shared
Wire identities/limits. Candid and management types remain upstream transport
implementation details. The retired `ic-testkit-candid` package remains absent.
The driver selects Platform, Testkit, Wire and the probe protocol only for its
test binary. Its normal build coordinator uses Build and Host for artifacts and
process custody, plus Clap/Serde/JSON for commands and receipts. It does not link
the simulator or codecs. Build, Host and Testkit share the published 0.65 family;
the driver and Testkit 0.65 select the same Platform 0.6 identity. The incoming
Testkit also selects Platform's Candid/management support for native provider
fixtures; PocketIC retains Candid for its simulator protocols. These are native
qualification dependencies. Independent library and canister graphs
remain Candid-free and select only Platform 0.6.
These host-only providers do not enter deployed graphs; there is no unpublished
override.
The local provider check also rejects Candid codec paths and default-codec
query/update attributes in maintained probe source.

The Candid-free runtime introduced in Platform 0.2.0 was requested in
[#31 — Support Candid-free raw and Wire canister entrypoints](https://github.com/dragginzgame/ic-platform/issues/31).
The two small adapters translate consumer-owned Wire limits/errors into Platform
hooks. Wire 0.6 retains this native Serde codec and retires its finite API;
the current probe records need no layout or limit changes. Testkit 0.65's optional
helpers also support native Serde calls; they are not selected by this harness.
Keep the direct raw-call/native-codec integration; the protocol package must not
acquire a Platform dependency to hide these adapters. Host metadata generation
is not selected by probes or the protocol package.

The published library retains three direct dependencies: Metrics owns counter
arithmetic, Platform owns executor/system bindings, and Thiserror owns the
maintained typed error implementations. Reimplementing those capabilities to
remove their declarations would create competing owners. Platform 0.6 supplies
the lean executor/IC0 feature boundary requested in
[#40 — Keep executor/IC0-only graphs free of Principal, Serde and crypto dependencies](https://github.com/dragginzgame/ic-platform/issues/40);
Timers consumes it without bypassing Platform or copying bindings. Its library
normal/build graph has 14 rather than 30 name/version identities including the
root. That graph reduction is not a Wasm-size or runtime performance measurement.

The [dependency policy](../ci/cargo-dependency-boundaries.json) is enforced through
Shared's canonical checker in `actions-check`, alongside pin/inheritance checks.
It forbids these direct upstream declarations in every member. Metadata covers aliases and all
normal/build/dev and target declarations; enabled features and transitive
reachability for the published library are enforced by its local all-feature
normal/build graph check for Wasm and the actual native host. Probe graph
qualification remains separate. See
[deployment exclusions](shared-tooling.md#deployment-dependency-exclusions).

## Current runtime

Direct registration uses one inert `TimerStart` configuration. Retained starts
carry an optional initial schedule; remove-on-stop starts carry a required one.
The existing initial-registration owner decodes that configuration once,
validates requested time before allocating a claim, and returns scheduled
authority only after provider binding commits its successful-arm facts. `DeclarationLifetime`
remains a snapshot observation. See the [typed setup contract](design/0.20-typed-registration.md).

First lifecycle reconciliation allocates and initializes its claim through the
same creation owner as direct registration, under one registry borrow. It applies
provider effects before publishing authority into the owner slot. There is no
public inactive-registration/control detour. Failed creation uses the common
cleanup owner to remove its exact declaration and handles before releasing captures. Existing registrations retain their callback
and failure observations on later control errors; an unused replacement factory
is dropped outside registry borrows before applying that control.

The crate root is the only public facade. It re-exports runtime operations and
provider-neutral values directly; every implementation and value-grouping
module is private. In particular, neither `callback`, `platform`, `registry`, `schedule`,
nor `snapshot` is a consumer import path.

The private `callback` module owns public `OnceDecision` / `OnceRunResult` and
`AfterCompletionDecision` / `AfterCompletionRunResult`. Public futures cannot
return the other policy's result; only after-completion decisions contain
configured recurrence. The existing runtime erasure owner creates the consumer
future before wrapping its result conversion, preserving synchronous callback
factory timing and suspension across the original future. One private
`OrdinaryRunResult` and `OrdinaryDirective` feed the existing arbitration. There
is no public generic policy marker or conversion between result policies.

The remaining module hierarchy keeps six responsibilities separate:

![The schedule, snapshot, and control modules connect to the registry; the runtime coordinates callbacks and effects through the private platform boundary](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-architecture-modules.svg)

1. `schedule` owns validated cadence, requested schedules, post-run
   private erased directives, and checked nanosecond/deadline conversion. Explicit requests
   and ordinary directives resolve to one private `ResolvedSchedule` carrying
   deadline, requested delay and mode; Stop resolves to no successor. Pending
   commands retain that value directly, and completion does not derive control
   metadata from a directive snapshot.
2. `snapshot` owns inert public identity and observation values. Its private
   `identity`, `model`, and `metrics` children separate validation, closed
   state/outcome types, and saturating measurements. Only the registry builds
   the top-level timer and atomic inventory snapshots and initializes their
   observation aggregates. Platform reads construct inert `MemoryPageExtent`
   values directly. Those aggregates and extents expose no public construction
   or control path.
   Outcome observations project their latest classification and work count from
   one stored terminal event. Success, failure and unacknowledged timestamps and
   the expected-failure streak remain independent history.
3. `control` is the private ordinary generation/registration state machine. It
   owns checked callback generations, immediate schedule, reconciliation and
   stopping transitions. One counter owns ordinary allocation history and
   the active callback generation; state variants do not store another copy.
   Its inactive state owns its reason, its running
   state owns its pending command, and scheduled state carries neither. The
   registry owns exact running-work authorization and command arbitration.
   Completion arms its selected successor through the shared checked arming
   operation or stops with its selected reason, in the same atomic transition.
   Checked arming returns canonical `TimerControlFailure`
   values directly, without a private error conversion.
   Ordinary directive resolution also returns `TimerControlFailure` directly;
   explicit scheduling requests retain `ScheduleError` at their input boundary.
   The schedule owner classifies invalid successor proposals, while the registry
   owns terminal state and completion accounting.
   Ordinary completion selects its schedule, directive observation and
   cancellation outcome together before successor allocation. Authoritative
   reconciliation reuses its checked schedule, preserving request metadata;
   cancel/unregister discard the callback proposal. Pending ensures still validate
   the callback and preserve its directive observation even when the ensure wins.
   Resolution and allocation failures share one terminal finalization branch and
   the common removal exit; explicit consumer invariant failure still wins.
   Cancellation in the registry stops scheduled state without allocating a
   generation and queues a command for running work. Stopping with a reason
   discards the running command through state replacement.
   Ordinary Once and after-completion admission retain their policy checks before
   selecting the shared ensure/reconcile command. The arbitration engine does not
   repeat the caller's policy distinction.
   The private `registry/effect` module carries ordinary-work or Watchdog-scheduler
   wakeup variants through runtime dispatch. Only ordinary deliveries receive an
   abandonment guard, constructed before the future's first poll. A Watchdog
   dispatch stores one successor identity and
   derives its work token with the same claim and generation. Runtime application
   admits live claim ownership, policy and generation during handle installation;
   structurally malformed token pairs need no second validation path.
   The registry builds provider effects and retains cancellation policy without a
   separate cancellation or completion action. Scheduling requests return only
   an optional initial or replacement arm kind; generation and deadline remain
   in control state. Scheduling and completion share one checked arming operation.
   Within the same atomic transition, the registry builds each successful arm
   from its selected deadline and the allocated generation without a second
   registration-consistency recovery path.
4. `registry` is the provider-call-free fixed-capacity canonical owner for
   structured identities, callback closures, claim generations,
   policy-specific state, the sole pending ordinary-command machine, nested
   arbitration, deterministic snapshots, and provider-neutral effects. It also
   owns every bound provider handle; only `runtime` invokes `platform` to
   create or clear those handles. Each entry has one policy-specific payload
   containing its control and correctly typed callback. An ordinary payload
   without cadence is Once; one with cadence is AfterCompletion. Public policy
   observations are derived from that payload rather than stored separately.
   Both policy state machines own their inactive reasons; active states carry
   no inactive reason, and snapshots project the reason from the state itself.
   Watchdog awaiting-work state owns its pending command through both dispatch
   and execution. Work acceptance preserves it; leaving that state discards it,
   including when the scheduler retires an unacknowledged attempt.
   Watchdog control retains one generation counter. Each dispatch gives its
   successor and work the same fresh generation, distinguished by role. Scheduling
   requests while work is dispatched or running cannot replace that pair: exact
   scheduling waits for normal completion, and ensures coalesce or become pending
   commands. Completion either retains the successor, replaces it after expiring
   the work attempt, or stops. A later recovery scheduler also expires the old
   attempt before advancing the generation. No state repeats allocation history.
   Authorization checks generation together with state, claim and role; awaiting
   snapshots expose that generation once and project `attempt_status` directly,
   without a separate attempt wrapper. Tokens and owned handles keep their
   independent delivery stamps for stale-callback rejection and separate slots.
   Ordinary completion computes its removal-on-stop decision once from pending
   unregister and declaration lifetime; every terminal exit uses it. A successful
   arm retains its declaration. Cancellation derives immediate removal once
   from final inactive state and lifetime for both ordinary and Watchdog
   declarations; running work retains pending authority.
   A private ordinary delivery guard exists before the first provider poll and
   owns the queued token without another identity clone. Normal return disarms it.
   A dropped confirmed delivery retires only its exact scheduled/running generation,
   records Unacknowledged and leaves retained state Abandoned. Transient lifetime
   or pending unregister removes the entry. The guard performs no provider calls
   and releases removed callbacks outside the registry borrow. Explicit cancellation,
   replacement and unconfirmed installation failure remain distinct from abandonment.
   The native drop and PocketIC trap subjects passed in the 0.13.2 hosted
   release gates; [source and host scope](releasing.md#host-support) remain
   separate from Watchdog recovery and pending source qualification.
   Watchdog completion decides removal once after selecting its final state:
   inactive declarations follow their lifetime and pending unregister command,
   while retained or replaced successors keep their declaration.
   Ordinary request, Watchdog request and Watchdog scheduler failures decide
   terminal removal through the entry already selected and authorized by their
   transition. The shared entry predicate uses that transition's failure and
   declaration lifetime; no second lookup reconstructs the decision. The registry's
   existing removal exit transfers the removed entry into its transition.
   Public control leaves handles bound for no-op/rejected requests and detaches
   them after actual transitions, including from removed entries. Runtime applies
   provider cleanup before releasing removed captures outside registry access.
   Watchdog dispatch/completion retain their separate pre-transition detachment
   and rollback rules; see the [capture ownership contract](design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests).
   Watchdog cancellation selects the handles to clear in one state match. An
   immediate stop uses that selection for state replacement, cancellation
   accounting and provider cleanup; running work keeps its pending command, and
   inactive cancellation preserves the existing reason. No generation is allocated.
   Ordinary schedule requests share counter, request-metadata and coalescing
   updates. Recurring ensure submits an existing scheduled deadline unchanged;
   it calculates a cadence deadline only for inactive or running declarations.
   The request owner commits mode once for successful arms or exact reconciliation;
   coalesced ensures retain it, while equal exact requests update it.
   Watchdog requests commit scheduling mode once after a successful initial or
   replacement arm; coalesced, pending and failed arms preserve it while recording
   requested delay. A cadence deadline is calculated only for an inactive Watchdog.
   The common Watchdog request exit records coalescing for a no-effect transition
   without failure, keeping exhausted initial arms distinct from coalesced demand.
   Owned provider roles follow entry policy and handle slot rather than a copied
   field. Installation and consumption reject policy/role mismatches before using
   a slot; detached handles retain complete tokens for restoration and cleanup.
5. `platform` owns pending one-shot tasks, authenticated self-call dispatch and
   IC system facts. Its handle is linear and it owns no recurrence policy.
   The bounded queue removes cancelled records immediately. The FFI module alone
   registers callbacks and exports the global timer/internal dispatch entrypoints;
   see the [backend design](design/0.26-owned-timer-backend.md).
   Page reads return the same inert extent representation used in snapshots,
   without a parallel platform value or a registry conversion.
6. `runtime` owns the one canister-local registry static, erases consumer
   callbacks for registry storage, exposes registration claims, applies
   provider effects, and drives live `Once`/`AfterCompletion` dispatch and the
   two-role watchdog protocol. Its three policy-specific delegated work
   contexts store their exact private callback token directly. Their control
   methods use the shared claim-transition boundary, which validates running-work
   authority before handle detachment and the registry operation.
   Each private token contains its registration claim, callback generation and
   role. Context control and callback cleanup borrow that claim instead of
   reconstructing it. Owned delivery tokens remain cloneable; registration
   capabilities remain non-clone. Borrowed claims still require the operation's
   exact running-work or provider-ownership validation.
   Claim-originated effect failures retire the declaration instead of leaving
   registry state scheduled without a provider handle. Lifecycle verification
   checks the exact claim and immutable declaration metadata directly rather
   than reconstructing authority from a snapshot. Synchronous operations that
   detach handles restore all of them before returning an unexpected registry
   error, retiring the exact claim if restoration cannot recover ownership.
   One detached-claim finalizer owns registry-error restoration and successful
   transition application, including retirement after unexpected provider failure.
   Callback finalization remains separate to preserve Watchdog rollback rules.
   One consuming binding operation installs a raw provider handle or clears it
   on rejection after releasing the registry borrow. Arm and restoration callers
   receive only the typed error; Watchdog dispatch separately clears its installed
   successor if later work binding fails. Final handle installation and successful
   arm accounting share one registry borrow and one role/generation/state check.
   A staged Watchdog successor and restored handles do not commit arm counters;
   successful work installation commits both Watchdog dispatch counters.
   Effect application consumes one complete registry effect; its shape is
   validated before cleanup or platform calls. Its matched branches perform
   binding directly without passing the effect through another variant match or
   duplicating token and delay arguments beside it. One initial/replacement
   arm kind flows from ordinary control through provider binding, and the
   non-empty set of callbacks to clear is also a closed value rather than
   independent booleans. The entry-local exact-claim predicate is shared by
   callback acceptance and fired-handle consumption. Measurements and provider
   installation retain canonical claim lookup with their own error semantics;
   identity reuse cannot transfer handle authority to a stale callback.
   Work acceptance checks the exact claim, policy, work role, generation and
   eligible state, then transitions to running state and returns that entry's
   correctly typed callback. Dispatch releases the registry borrow before
   invoking consumer work; there is no separate callback lookup or second borrow.
   Ordinary and Watchdog work delivery consume the matching fired handle through
   that selected entry before acceptance, without a separate registry consumption
   pass. Scheduler delivery consumes its handle and detaches remaining capabilities
   through one lookup before the Watchdog transition can remove a declaration.
   Duplicate or stale acceptance returns no callback and keeps its existing stale
   accounting. Installation owns the required role, generation and state checks;
   pure transition tests model successful application through those same predicates.
   Delegated control and both work-completion paths share one entry-local
   running-work predicate for the exact claim, work role, generation and running
   state. Completion checks it before any mutation; provider binding and completed
   measurements retain their distinct validation boundaries.

Absolute deadlines remain authoritative in policy control state and its snapshot
projection. Provider arm effects carry only the resolved delay, callback authority
and arm kind; they do not store another deadline representation.

The registry and runtime transition functions remain cohesive even where they
are long: each audited function owns one atomic transition or one provider
binding path. Their tests live in directory-local `tests.rs` files so test
volume does not obscure production flow.

## Canonical runtime

One canister-local runtime owns the bounded registry and its structured
identity (`owner`, `subsystem`, `name`). Framework and application schedulers
must contribute to that same live registry rather than building private
inventories. All three policies, lifecycle reconstruction, measurements, and
the real-canister watchdog evidence matrix are implemented.

The runtime provides:

- one-shot, after-completion interval, and pre-armed watchdog policies;
- serial execution with overdue work coalesced into one pending run;
- cancellation requested safely by the running callback;
- synchronous lifecycle restoration followed by deferred application work;
- registration-lifetime counters, last outcomes, deadlines, instruction consumption,
  and bounded memory-page extent/growth observations;
- exact-claim observation of armed provider wake-up ownership without a
  snapshot-derived control path;
- bounded identity components and allocation-conscious hot paths;
- one atomic inventory snapshot carrying the runtime epoch even when empty;
  and
- portable snapshot DTOs so Canic, IcyDB, and standalone canisters can
  expose the same operator view.

Diagnostic event storage and arithmetic use published Metrics `MeasurementCounter`
values. Timers retains named events and registration/reset identity; it has no
parallel integer counter arithmetic. Instruction observations already use Metrics
summaries. The 0.29 getter cut requires downstream projections to preserve wide
totals and actual-overflow quality, or use checked narrowing. Page extent sampling
and functional failure streaks remain timer-specific. Probe wire records only
project values; the protocol package depends on Wire and Serde alone.

Operator-health labels belong to consumers. Snapshots expose runtime state,
scheduling mode and callback outcomes for a single consumer-owned projection.

Metrics are collected once at the registry boundary. Consumers may adapt
the atomic inventory to their own status endpoint or metrics encoder without
wrapping every callback independently.

The canonical snapshot is designed as a semantic superset of Canic's current
timer status, scheduling counters, and instruction metrics. In particular,
callback starts and completions remain separate because traps and instruction
exhaustion can prevent post-run measurement, and consecutive expected failures
remain cheap functional state because recovery decisions consume them. See the
[observability and Canic parity contract](design/observability.md).

## Lifecycle and observations

The registry holds at most 64 declarations. Ordinary work may be async;
Watchdog work is synchronous so its separate work message cannot cross an
`await`. Provider handles are private, linear capabilities owned by the registry.

The live snapshot omits elapsed IC time: message time cannot truthfully measure
synchronous callback duration. It distinguishes scheduler and work instructions,
plus start/end and maximum-growth Wasm/stable memory-page observations. Page
extent is not exact allocator liveness. A dispatched Watchdog attempt without a
committed completion is *unacknowledged*, not definitely trapped, and has no
fabricated measurement.

Lifecycle reconciliation always installs retained declarations so a
caller-owned `Option<Registration>` cannot outlive a remove-on-stop canonical
entry. Transient `RemoveWhenStopped` timers use direct registration and are
recreated explicitly by their owner if later desired; cancellation removes a
transient declaration even before its first schedule. One private lifecycle
seam installs and verifies the exact retained claim for all three policies.

## Safety boundary

An after-completion interval is not a watchdog: if its callback traps or runs
out of instructions, code after the callback cannot arm a successor. A
watchdog policy commits the successor before invoking fallible work while
preventing concurrent logical execution. Maintained
[PocketIC subjects](../testing/crates/ic-timers-pocketic/src/tests.rs) exercise
trap, actual instruction exhaustion, upgrade reconstruction, external-ingress
rejection, independent timers, insufficient cycles followed by top-up,
stop/resume, overdue coalescing and cancellation in the scheduler/work gap.
[SAFETY.md](../SAFETY.md) owns the guarantees and evidence limits; historical
server versions and receipts do not qualify a new source/artifact combination.

After normal progress, a Watchdog may replace that exact cadence successor with
a deadline of now. The replacement remains a scheduler callback in a later
replicated message; that scheduler again pre-arms cadence safety before it
queues work. Initial immediate reconciliation and running-work requests use the
same declaration, claim, generation, pending-command, and provider-handle path.
See the [immediate continuation design](design/immediate-watchdog-continuation.md).

Exact Watchdog reconciliation can also move a sleeping deadline
earlier or later. Dispatched or running work retains the committed cadence
recovery successor until normal completion applies the pending schedule or
`WatchdogDecision::ScheduleAt`. Snapshots carry a registration identity so
consumers can detect counter replacement within one runtime epoch. See the
[continuity and deadline contract](design/0.8-registration-continuity-and-deadlines.md).

## Consumer integration

Read-only inspection on 2026-10-18 found one registry `ic-timers` 0.27.1 package
in each available Canic, IcyDB and Jobs lockfile, with no `ic-cdk-timers` package.
These are source/graph observations, not downstream build or deployment results;
dirty sibling work was preserved and no sibling suite was run for this review.

- Canic's [`workflow/runtime/timer`](https://github.com/dragginzgame/canic/blob/5583496bfa95069809249e8e5d16c77f1ffd744f/crates/canic-core/src/workflow/runtime/timer/mod.rs)
  integrates native claims and framework authority/suspension policy, including
  a recovery Watchdog. Its pending
  `crates/canic-core/src/ops/runtime/timer_status/mod.rs` owns operator-health
  projection in the inspected worktree; that extraction is not yet in its base
  commit. Timers must not absorb those application decisions.
- IcyDB's SQL, lifecycle and startup-timer test canisters import Timers. Its
  [manifest](https://github.com/dragginzgame/icydb/blob/14051952f449ab0f730c97512a42011b3e06061e/Cargo.toml)
  also selects a pinned `canic-icydb` adapter; that adapter retains database
  readiness and lifecycle policy. Available source still contains the standalone
  primitive timer probe. Its prepared retirement is separate from the already
  pending mixed-policy qualification here; no sibling deletion is claimed.
- Jobs' [`apps/job-consumer`](https://github.com/dragginzgame/ic-jobs/blob/312ff2486f7d15d60be92a14c58cc2bbd45e3d05/apps/job-consumer/src/canister.rs)
  uses a Watchdog to dispatch its consumer-owned queue. The published Jobs
  library owns job transitions, not a timer registry or global provider.

Composed canisters must qualify one resolved Timers package and global-timer
owner, synchronous reconstruction from durable authority, and progress/recovery
for each participating policy against their actual linked artifact. The backend
bounds 130 pending records and 128 outstanding dispatches; these do not bound
executor or application memory.

A consumer may retain a bounded custody collection of opaque registration
claims. It makes claims enumerable without duplicating scheduling state,
deadlines, generations, counters, pending commands or provider handles. Callback
contexts delegate only policy-valid operations for the exact running generation
and role; they expire at completion and cannot substitute for stored claims.

The [Canic](adoption/canic.md), [IcyDB](adoption/icydb.md) and
[Toko Miner](adoption/toko-miner.md) adoption records preserve older exact-version
subjects. Their API tables and historical provider limits are not current
integration instructions. The numbered design records preserve decisions that
preceded the owned backend; current behavior is defined by the facade,
implementation and maintained tests above.
