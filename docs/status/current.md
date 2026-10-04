# Current status

Last updated: 2026-10-04

## Purpose

This is the compact session handoff. Historical implementation, delivery
references and validation belong in [release notes](../changelog/README.md),
[audits](../audits/code-hygiene.md) and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.10.10`.
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
- Once and AfterCompletion accept async work. Watchdog accepts one synchronous
  bounded unit after its scheduler commits a cadence successor.
- Only private `platform` calls the provider and IC system facts. Rust visibility
  and repository checks enforce provider confinement and restricted declarations.
- Snapshots and armed-wakeup observations are inert. Contexts expire with their
  exact work token; retained claims own longer-lived control. Claims and generations
  do not wrap. Consumer durable authority reconstructs volatile retained declarations
  synchronously before downstream hooks; shared-registry adoption is atomic.
- Ordinary inactive state owns its reason; running state owns its pending command.
  Watchdog inactive state owns its reason; awaiting-work state owns its pending
  command through dispatch and execution. Leaving either running or awaiting-work
  state discards its command. Public snapshots project these states without
  mutation authority.
- Registry arbitration owns ordinary pending-command order and exact running-work
  authorization. Exact reconciliation replaces a discarded callback scheduling
  proposal before validation. Invariant failure remains terminal; unregister is
  sticky; ensure selects earliest demand. The effective exact directive is observed.
- Ordinary requests and authorized completion successors share checked arming;
  stopping selects its reason and allocates no generation. Lifetime removal is
  decided once per transition. See the [ordinary state ownership note](../changelog/0.10.8.md).
- Ordinary dispatch shares completion finalization for callback results and
  callback-borrow failures. Work measurements are recorded only after executed
  work; see the [ordinary callback finalization note](../changelog/0.10.9.md).
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

## Unresolved scope

Downstream work is deferred at the maintainer's request. Historical adoption
records remain scoped to their recorded subjects; do not treat them as current
composed qualification. Ordinary and Watchdog command machines, effect confirmation
and handle-restoration stages retain their distinct suspension and recovery roles.

## Evidence

Inspected hosted validation for 0.9.4 is scoped in its
[delivery note](../changelog/0.9.4.md). Release reports and Git references alone do
not establish new hosted or PocketIC results. Each change's verification record
belongs in its release note rather than a repeated handoff claim.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims. Tests, builds,
lint gates and deployment validation remain user-owned.

## Next action

Establish release state from Cargo, the changelog and matching release notes,
then continue the maintainer's requested work within the ownership boundaries
above. Record each change and its scoped verification in the release notes.
Leave Cargo versions, both lockfiles and Git release execution to the maintainer.
Release commands always perform the requested bump; preparing notes does not
advance the workspace version.
