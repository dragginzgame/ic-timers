# Current status

Last updated: 2026-10-03

## Purpose

This is the compact session handoff. Historical implementation and validation
belongs in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md)
and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.9.2`.
- Latest release line: `0.9.2`.
- The maintainer reports 0.9.2 live. Release commit
  `153770c004611a0c9020f36bfc95348ee5293a2e`, annotated `v0.9.2`, local
  `origin/main`, Cargo, both lockfiles and the dated changelog agree. Cached
  registry VCS metadata identifies the same commit. See the
  [0.9.2 delivery record](../changelog/0.9.2.md).
- Hosted main checks/MSRV and tag-truth jobs succeeded for that exact SHA.
  They do not establish fresh downstream or PocketIC qualification.
- The [0.9.0 note](../changelog/0.9.0.md) records the ordinary-completion
  semantic hard cut. Removing a public item or semantic contract requires the
  next minor line; private behavior-preserving simplifications may use a patch.
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

- GitHub [#8](https://github.com/dragginzgame/ic-timers/issues/8) is closed:
  its fixture simplification and ordered-sequence rationale shipped in 0.9.2.
  Supplemental root MSRV cognitive-complexity lint was not rerun here; no lower
  numerical score is claimed and the maintained gate scope is unchanged.
- [#3](https://github.com/dragginzgame/ic-timers/issues/3) is fixed in the local
  worktree: the README now advertises API line 0.9 and exact timer 0.9.2. A small
  projection helper uses workspace-version truth during bumps and release checks;
  the existing bump rollback includes README.md. Existing fixtures are updated.
  The issue remains open until the maintainer lands the fix.
- The focused registry review removes ordinary completion's duplicate
  missing-cadence check and shares effect-confirmation bookkeeping after its
  role-specific validation. Typed failure, pending precedence, idempotent
  counters and provider binding/recovery ordering are preserved. Existing
  behavior fixtures were inspected; runtime validation remains user-owned.
- Ordinary/Watchdog pending commands, the confirmation marker and the separate
  binding/confirmation stages remain necessary for their distinct suspension,
  cadence recovery and partial-binding failure contracts.
- The combined follow-up now includes private crate changes alongside repository
  tooling and documentation. Target release: `0.9.3`. Its undated changelog section
  is directly below the empty `Unreleased` heading. The
  [0.9.3 note](../changelog/0.9.3.md) records scope and pending maintainer validation.
  Package versions and lockfiles remain unchanged. No compatibility path or new
  public contract is added.

## Evidence

The released baseline combines the maintainer's live report, matching local
artifacts, cached registry provenance and inspected hosted jobs. Current local
follow-up checks cover shell syntax, read-only release metadata and diff whitespace.
Changed Rust is formatted; no runtime test result is claimed for the refactor.
The existing runtime fixtures, prepared release-tooling fixtures and deployment
gates remain user-owned; no new suites, builds, lint gates or dependency resolution
were run here.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims.

## Downstream state

Read-only inspection on 2026-10-03 found:

- Canic HEAD `e327ed6f01a07f27f030eaf7e0dc04802e01c17c`, dirty, requests timer
  0.9 and locks one 0.9.1 package. Its custody/suspension owner uses native claims;
  it is not evidence of a newly published coherent consumer graph.
- IcyDB HEAD `d35d8ffda192c52d2dfda183b55251fcbc5a0cee`, dirty, requests timer
  0.9 and locks one 0.9.2 package. Its generator owns one retained startup Watchdog.
- Toko Miner HEAD `dba00bbf3cb8faa6fa007ad23bdf5a67209a6c34`, with a dirty lock,
  selects both 0.8.1 and 0.9.2. Application/Canic Core 0.110.51 use 0.8.1;
  IcyDB 0.264.6 uses 0.9.2. Cached registry manifests confirm the incompatible
  dependency requirements. This cannot qualify one shared timer registry.

[Canic #33](https://github.com/dragginzgame/canic/issues/33) remains open for
published graph alignment and composed lifecycle qualification. The
[Toko Miner record](../adoption/toko-miner.md) records exact lock provenance,
its still-separate checkpoint deadline/work registrations and required scheduling
semantics. Package alignment comes before removing that auxiliary deadline.
Historical [Canic](../adoption/canic.md), [IcyDB](../adoption/icydb.md) and Toko Miner
receipts remain scoped to their subjects. No sibling files were edited or gates run.

## Next action

0.9.2 is released; do not repeat its bump. The 0.9.3 changelog and release note are
prepared. The maintainer owns validation and the patch bump, which dates the
changelog and updates package/lockfile versions and the README example.
Close #3 once the README fix lands. Toko Miner remains blocked until Canic pushes
and publishes the aligned dependency update; then qualify one timer package across
framework/database/application before checkpoint cleanup. All release execution
remains user-owned.
