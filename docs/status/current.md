# Current status

Last updated: 2026-10-03

## Purpose

This is the compact session handoff. Historical implementation and validation
belongs in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md)
and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.10.1`.
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

The maintainer reports 0.10.0 live. Local HEAD, the peeled 0.10.0 tag and origin/main
agree at `11edb5286b7d8853496da29d50a49201a3f3aa58`. Its arbitration hard cut,
ordinary action-model deletion and derived provider roles are recorded in the
[0.10.0 note](../changelog/0.10.0.md).

The [0.10.1 note](../changelog/0.10.1.md) records the prepared private cleanup.
Explicit schedules and ordinary directives now resolve to one `ResolvedSchedule`
carrying deadline, requested relative delay and scheduling mode. Pending commands
retain it directly; Stop resolves to no successor. The separate directive result,
registry wrapper, completion conversion and snapshot-derived mode fallback are
deleted. Public API, snapshot shape, ordering, typed failures and request-time
relative resolution remain unchanged.

Existing schedule fixtures cover every resolved mode and Stop. Snapshot conversion
keeps independent public duration-range validation with a negative fixture. Existing
registry tests cover completion, exact/earliest arbitration, discarded proposals
and Watchdog metadata. Test execution remains maintainer-owned.

Ordinary/Watchdog command machines, effect confirmation and handle-restoration
stages remain required by their distinct suspension and recovery contracts.
Downstream work is deferred at the maintainer's request. Historical adoption
records remain scoped to their recorded subjects; do not treat them as current
composed qualification.

## Evidence

The latest inspected hosted validation remains scoped to 0.9.4 as recorded in
its delivery note; the maintainer's 0.10.0 live report and local references do not
establish new hosted or PocketIC results. No new runtime, recovery or performance
result is claimed for this cleanup. Changed Rust is formatted. Diff whitespace,
current release-truth and target-note structural preflight checks passed during
preparation.
Tests, builds, lint gates and deployment validation remain user-owned.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims.

## Next action

Review the changes and prepared changelog, then run the maintainer-owned
focused checks and deployment validation. The automated contributor leaves Cargo
versions, both lockfiles and Git release state unchanged. Release commands always
perform the requested bump; preparing notes does not advance the workspace version.
