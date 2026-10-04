![IC Timers — Internet Computer helper library](../assets/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-04

## Purpose

This is the compact session handoff. Historical implementation, delivery
references and validation belong in [release notes](../changelog/README.md),
[audits](../audits/code-hygiene.md) and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.11.3`.
- Cargo owns this version; the release helper updates the single projection above.
  Dated changelog sections and release-note statuses own release state. Read those
  sources to distinguish preparation from a completed release; do not duplicate
  that distinction or current commit/tag references in handoff prose.
- Public removals or incompatible semantic changes require the next minor line;
  private behavior-preserving simplifications may use a patch. See the
  [ordinary arbitration hard cut](../changelog/0.9.0.md).
- Version mutation, staging, commits, tags, pushes, publication, release commands,
  tests and build/lint gates are user-owned. Automated contributors implement
  requested changes and prepare changelogs/notes without executing those gates.
- Direct provider: exact `ic-cdk-timers` 1.0.0; exact `ic0` 1.2.0. Probe canisters
  use exact `ic-cdk` 0.20.3. MSRV is Rust 1.88.0; development/hosted CI uses 1.99.0.

## Canonical runtime

- One volatile canister-local registry owns at most 64 structured identities,
  declaration claims, callback generations, policy states, pending commands,
  callbacks, observations and provider handles. Entries own matching control,
  callback and cadence in one typed payload; policy is derived.
- Runtime initialization creates that registry and samples its epoch only when
  the slot is empty. Repeated initialization returns the existing epoch; borrow
  conflicts remain typed errors. See the [initialization note](../changelog/0.10.17.md).
- Once and AfterCompletion accept async work. Watchdog accepts one synchronous
  bounded unit after its scheduler commits a cadence successor.
- Only private `platform` calls the provider and IC system facts. Rust visibility
  and repository checks enforce provider confinement and restricted declarations.
- Snapshots and armed-wakeup observations are inert. Contexts expire with their
  exact work token; retained claims own longer-lived control. Claims and generations
  do not wrap. Consumer durable authority reconstructs volatile retained declarations
  synchronously before downstream hooks; shared-registry adoption is atomic.
  Policy-specific contexts store their token directly and use the shared
  claim-transition validation boundary before provider-handle detachment; see the
  [control simplification note](../changelog/0.11.3.md).
- Ordinary inactive state owns its reason; running state owns its pending command.
  Watchdog inactive state owns its reason; awaiting-work state owns its pending
  command through dispatch and execution. Leaving either running or awaiting-work
  state discards its command. Public snapshots project these states without
  mutation authority.
- Each control owns one callback-generation allocation history; active states
  do not repeat it. Watchdog dispatch stamps successor and work with one fresh
  generation, distinguished by role and separate provider slots. Requests while
  awaiting work retain the pair until completion or recovery expires that attempt.
  Awaiting-work snapshots project the shared generation and direct attempt status
  without an attempt wrapper; tokens and handles retain independent
  delivery stamps. The 0.11.0 contract and evidence scope are recorded in its
  [release note](../changelog/0.11.0.md).
- Private callback tokens carry their registration claim. Context control and
  callback cleanup borrow it without claim reconstruction; running-work and
  provider ownership checks remain distinct. See the
  [callback claim ownership note](../changelog/0.10.14.md).
  Provider installation and effect confirmation use the canonical mutable claim
  lookup with callback-specific stale-error translation; see the
  [claim lookup note](../changelog/0.10.21.md).
  Late measurements and provider-handle consumption share that lookup while
  retaining no-op behavior for missing or superseded claims; see the
  [lookup and detachment note](../changelog/0.10.22.md). Watchdog failure cleanup
  obtains paired handles through the entry's existing detachment owner, with
  provider clearing after the registry borrow is released. Ordinary terminal
  request failures stop their validated control directly in the request owner.
  The shared claim-transition operation validates context authority before
  detachment; lifecycle reconciliation directly verifies retained declarations.
  See the [runtime validation ownership note](../changelog/0.10.20.md).
  Ordinary reconciliation owns claim-policy validation directly, before
  cancellation or schedule resolution; see the
  [validation and scheduler failure note](../changelog/0.10.23.md). Watchdog
  scheduler allocation and deadline errors share terminal finalization while
  retaining generation-first validation and checking the deadline before allocation.
- Registry transitions borrow identity from their claim or token for local
  lookup and removal. Queued effects, snapshots and detached capabilities retain
  owned identities; see the [identity borrowing note](../changelog/0.10.18.md).
- Classified completion counters own completed-work accounting. The public total
  projects their saturating sum; starts and unacknowledged attempts remain
  separate events. See the [completion counter note](../changelog/0.10.15.md).
  Latest outcome and work count project from one recorded event; historical
  timestamps and the failure streak remain independent. See the
  [outcome ownership note](../changelog/0.10.20.md).
- Registry arbitration owns ordinary pending-command order and exact running-work
  authorization. Exact reconciliation replaces a discarded callback scheduling
  proposal before validation. Invariant failure remains terminal; unregister is
  sticky; ensure selects earliest demand. The effective exact directive is observed.
  Pending scheduling commands select precedence in one match and retain the
  existing schedule metadata at equal deadlines; see the
  [ordinary command precedence note](../changelog/0.10.19.md).
- Ordinary requests and authorized completion successors share checked arming;
  allocation is checked directly in that operation. Stopping selects its reason
  and allocates no generation. Cancellation applies lifetime removal once from
  the final policy state. See the [state ownership note](../changelog/0.11.2.md).
- Cancellation allocates no callback generation. Scheduled/dispatched cancellation
  selects inactive state and clears the existing handles; stale delivery is rejected
  by state, claim and role. Rearming and dispatch still require fresh non-wrapping
  generations.
  Running cancellation remains a pending command applied on normal completion.
  The 0.11.0 semantic hard cut and its verification scope are recorded in the
  [release note](../changelog/0.11.0.md).
- Ordinary directive resolution returns canonical control failures directly.
  Explicit scheduling requests retain schedule errors at their input boundary;
  the registry owns terminal state and completion accounting. Directive and
  generation failures share one completion finalization branch and removal exit,
  preserving validation before allocation. See the
  [completion failure ownership note](../changelog/0.11.3.md).
- Watchdog initial and replacement requests share one scheduling-mode update after
  successful arming. Coalesced, pending and failed requests preserve it; requested
  delays still record accepted demand, and cadence deadlines are checked only for
  inactive control. See the [request observation note](../changelog/0.11.3.md).
  Coalescing is recorded once for a no-effect transition without failure. Ordinary
  request mode updates have one owner for successful arms or exact reconciliation;
  coalesced ensures preserve the existing mode. See the
  [request accounting note](../changelog/0.11.4.md).
- Ordinary dispatch shares completion finalization for callback results and
  callback-borrow failures. Work measurements are recorded only after executed
  work; see the [ordinary callback finalization note](../changelog/0.10.9.md).
  Ordinary and Watchdog work acceptance returns the typed callback directly from
  the validated entry. Dispatch releases that borrow before consumer work and
  has no second callback lookup. Context and completion authorization remain
  independent; see the [callback acceptance note](../changelog/0.11.2.md).
- Watchdog completion decides lifetime removal once from final inactive state
  and pending unregister, after selecting its successor or terminal transition;
  see the [Watchdog completion removal note](../changelog/0.10.10.md).
- Registry and owned-handle bounds do not bound the provider heap. Cancelled future
  deadline records remain queued. Page extents do not establish allocator bounds.
- Public control failures retire false scheduled state. One detached-claim finalizer
  restores handles after registry errors and retires claims after unexpected
  restoration or provider failures. Unexpected Watchdog work completion failures
  trap for IC rollback. Effect confirmation uses one validated wakeup-generation
  marker. These distinct failure rules must remain separate.
- Effect application binds wakeups and Watchdog dispatch in its validated match
  branches; confirmation retains its independent state and generation checks.
  Successor tokens share claim-based construction after authorization. Watchdog
  cancellation selects cleanup and immediate accounting together from its active
  state in the registry command owner; see the
  [cancellation decision note](../changelog/0.11.1.md).
- Platform page reads and callback measurements share inert `MemoryPageExtent`
  values. The registry pairs start/end extents without a second representation.

## Unresolved scope

Downstream work is deferred at the maintainer's request. Historical adoption
records remain scoped to their recorded subjects; do not treat them as current
composed qualification. Ordinary and Watchdog command machines, effect confirmation
and handle-restoration stages retain their distinct suspension and recovery roles.
The native dispatch-failure fixture and its verification scope are documented in
the [0.10.11 note](../changelog/0.10.11.md). Native injection does not simulate IC
rollback; deployment validation remains maintainer-owned.

Generation ownership, provider binding, workspace formatting and audit follow-up
are recorded in the [0.10.12 note](../changelog/0.10.12.md).

## Evidence

Inspected hosted validation for 0.9.4 is scoped in its
[delivery note](../changelog/0.9.4.md). Release reports and Git references alone do
not establish new hosted or PocketIC results. Each change's verification record
belongs in its release note rather than a repeated handoff claim.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims. Tests, builds,
lint gates and deployment validation remain user-owned.
Measurement ownership across identity reuse and policy-specific completion
failure assertions are scoped in the [0.10.16 note](../changelog/0.10.16.md).
Public lifecycle rejection fixtures cover identity, lifetime, cadence, expired
claims and occupied-identity reconstruction from an empty slot; their pending
verification is scoped in the [0.10.21 note](../changelog/0.10.21.md).

## Next action

Establish release state from Cargo, the changelog and matching release notes,
then continue the maintainer's requested work within the ownership boundaries
above. Record each change and its scoped verification in the release notes.
Leave Cargo versions, both lockfiles and Git release execution to the maintainer.
Release commands always perform the requested bump; preparing notes does not
advance the workspace version.
