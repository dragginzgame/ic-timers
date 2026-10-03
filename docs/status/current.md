# Current status

Last updated: 2026-10-03

## Purpose

This is the compact session handoff. Historical implementation and validation
belongs in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md)
and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.9.3`.
- Latest release line: `0.9.3`.
- The maintainer reports 0.9.3 live. Release commit
  `0c004e55a931aadf0861e902b221938e0e7d9985`, annotated `v0.9.3`, local
  `origin/main`, Cargo, both lockfiles and the dated changelog agree. The README
  example was updated by the actual bump. See the
  [0.9.3 delivery record](../changelog/0.9.3.md).
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

- GitHub [#3](https://github.com/dragginzgame/ic-timers/issues/3) is closed:
  the README projection and rollback fix shipped in 0.9.3. The actual release
  README advertises API line 0.9 and exact timer 0.9.3. The subsequent issue scan
  found no open ic-timers issues. [#8](https://github.com/dragginzgame/ic-timers/issues/8)
  remains closed for the 0.9.2 fixture simplification and documented rationale.
- Ordinary completion now returns a checked state change without a separate
  completion action or pass-through cancellation argument. The registry builds
  its effect from the resulting control state and selected schedule. Fabricated
  fallback schedule metadata is removed; unexpected state/metadata pairs keep
  typed terminal cleanup. Public contracts and failure/lifetime rules are preserved.
- Existing control fixtures observe resulting registration/generation rather than
  the removed action. The nested cancellation/ensure fixture now checks counter
  ownership too. These changes are formatted and inspected, not test-executed.
- Ordinary/Watchdog pending commands, the confirmation marker and separate
  binding/confirmation stages remain necessary for their suspension, cadence
  recovery and partial-binding failure contracts. The previous focused validation
  and confirmation simplifications shipped in 0.9.3.
- Target release: `0.9.4`. Its undated changelog section is directly below the
  empty `Unreleased` heading. The [0.9.4 note](../changelog/0.9.4.md) records this
  private crate cleanup, focused review and pending maintainer validation.
  The review found no additional confirmed fix for this patch. Versions and
  lockfiles remain unchanged; no public or compatibility path is added.

## Evidence

The 0.9.3 baseline combines the maintainer's live report, matching local artifacts
and successful inspected hosted jobs. Independent registry publication evidence
was not inspected for this release. Changed Rust is formatted; current local
follow-up checks cover read-only metadata, links and diff whitespace. No runtime
test result is claimed for the new completion refactor. Focused fixtures and
deployment gates remain user-owned; no new suites, builds, lint gates or dependency
resolution were run here.
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

0.9.3 is released; do not repeat its bump. The 0.9.4 changelog and release note
are prepared. The maintainer owns validation, the patch bump and release execution;
the automated contributor has not changed versions, lockfiles or Git release state.
Toko Miner remains blocked until Canic pushes and publishes the aligned dependency
update; then qualify one timer package across framework/database/application
before checkpoint cleanup. All release execution remains user-owned.
