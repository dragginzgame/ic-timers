# Current status

Last updated: 2026-10-03

## Purpose

This is the compact session handoff. Historical implementation and validation
belongs in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md)
and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.9.5`.
- Cargo owns this version; the release helper updates the single projection above.
  Dated changelog sections and released note statuses record completed releases.
- The last inspected delivery record includes the maintainer's live report and
  scoped hosted validation. See the [delivery record](../changelog/0.9.4.md).
  Those jobs do not establish fresh PocketIC or downstream qualification.
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
- Ordinary exact reconciliation replaces a discarded callback scheduling proposal
  before validation. Invariant failure remains terminal; unregister is sticky;
  ensure selects earliest demand. The effective exact directive is observed.
- Registry and owned-handle bounds do not bound the provider heap. Cancelled future
  deadline records remain queued. Page extents do not establish allocator bounds.
- Public control failures retire false scheduled state. Unexpected Watchdog work
  completion failures trap for IC rollback; detached handles are restored or cleared.
  Effect confirmation uses one validated wakeup-generation marker.

## Current follow-up

The [timer-control and release-handoff note](../changelog/0.9.5.md) records the
current private cleanup. Provider effects carry resolved delays without copying
absolute deadlines from control state. Registry fixtures observe deadlines through
snapshots; provider-delay and malformed-token checks remain. Public API and
snapshot contracts are preserved.

Ordinary cancellation returns checked success or a typed error, leaving provider
cleanup and pending commands with the registry. The scheduling action no longer
permits a clear; impossible cancel/arm and schedule/clear branches are removed.
Existing control fixtures observe state, repeated-cancel generation stability and
atomic exhausted cancellation; registry lifetime and runtime cleanup coverage remain.

Release finalization and validation use the single workspace-version projection.
The duplicate latest-release field and its fixtures are removed. Structural
README version checks remain; editorial release prose is outside the gate.
Preparation evidence is historical rather than another current release selector.
The changelog owns the prepared release target and its notes.

Ordinary/Watchdog command machines, effect confirmation and handle-restoration
stages remain required by their distinct suspension and recovery contracts.
Downstream work is deferred at the maintainer's request. Historical adoption
records remain scoped to their recorded subjects; do not treat them as current
composed qualification.

## Evidence

The inspected released baseline has matching local artifacts and successful hosted
main checks/MSRV and tag-truth jobs. The delivery record names the exact subject.
No new runtime, recovery or performance result is claimed for this cleanup.
Changed Rust is formatted. Shell syntax, diff whitespace and the read-only
release-truth and target-note structural preflight checks passed during preparation.
Tests, builds, lint gates and deployment validation remain user-owned.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims.

## Next action

Review the private cleanup and prepared changelog, then run the maintainer-owned
focused checks and deployment validation. The automated contributor leaves Cargo
versions, both lockfiles and Git release state unchanged. Release commands always
perform the requested bump; preparing notes does not advance the workspace version.
