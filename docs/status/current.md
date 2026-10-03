# Current status

Last updated: 2026-10-03

## Purpose

This is the compact session handoff. Historical implementation and validation
detail belongs in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md)
and the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.9.2`.
- Latest release line: `0.9.2`.
- The maintainer reports 0.9.1 live. Local release commit
  `b51dae91b563e6d0e427950c89d8006c375162e0` has annotated `v0.9.1`; the existing
  local `origin/main` ref points to the same commit. Cargo, both lockfiles and
  the dated changelog match. See the [0.9.1 delivery record](../changelog/0.9.1.md).
- The [0.9.0 note](../changelog/0.9.0.md) records the preceding ordinary-completion
  semantic hard cut. Removing a public item or semantic contract still requires
  the next minor line; private behavior-preserving simplifications may use a patch.
- Version mutation, staging, commits, tags, pushes, publication, release commands,
  tests and build/lint gates are user-owned. Automated contributors implement
  requested changes and prepare changelogs/notes without executing those gates.
- Direct provider: exact `ic-cdk-timers` 1.0.0; exact `ic0` 1.2.0. Probe canisters
  use exact `ic-cdk` 0.20.3. MSRV is Rust 1.88.0; development/hosted CI uses 1.99.0.

## Canonical runtime

- One volatile canister-local registry owns at most 64 structured identities,
  declaration claims, callback generations, policy states, pending commands,
  callbacks, observations and provider handles.
- Once and AfterCompletion accept async work. Watchdog accepts one synchronous
  bounded unit in a work message after its scheduler commits a cadence successor.
- Only private `platform` calls the provider and IC system facts. Rust visibility
  and forbidden private-interface leaks prevent public provider authority;
  repository checks enforce direct-provider confinement and restricted declarations.
- Snapshots and armed-wakeup observations are inert, not control authority or
  delivery guarantees. Contexts expire with their exact work token; retained
  registration claims own longer-lived control. Claims and generations do not wrap.
- Consumer-owned durable authority reconstructs retained declarations synchronously
  before downstream hooks. Timer authority is volatile, never restored from
  persisted snapshots or handles. Shared-registry adoption remains atomic.
- Ordinary exact reconciliation replaces a discarded callback scheduling proposal
  before validation. Invariant failure remains terminal; unregister is sticky;
  ensure selects earliest demand. The latest directive projects an effective exact
  ScheduleAt when reconciliation wins.
- Registry and owned-handle bounds do not bound the pinned provider's heap.
  Cancelled future deadline records remain queued until processing. Churn fixtures
  observe page extents without promising global memory, allocator or cost bounds.
- Public control failures retire false scheduled state. Unexpected Watchdog work
  completion failures trap for IC rollback; detached handles are restored or cleared.
  Effect confirmation remains idempotent with one validated wakeup-generation marker.

## Open work

- Ordinary control selects arm kind and allocates successor generations directly,
  removing derived optional planning values. Stopping at generation exhaustion
  remains possible; rearming fails before state mutation.
- Instruction summaries use sample count as their sole empty marker. Public
  latest/maximum getters preserve empty versus zero-valued samples and continue
  after saturation. Boundary coverage is prepared.
- Callback binding applies the existing role-specific recovery rule to all
  unexpected errors without repeating the error-variant list.
- Ordinary failed-completion paths share stop bookkeeping while preserving
  distinct consumer-invariant and typed control-failure outcomes.
  The maintainer reported Clippy rejecting the lifetime fixture's negated
  initial-case branch and the shared stop helper's optional-failure match.
  They now use an initial equality branch and `Option::map_or_else`, preserving
  coverage and failure outcomes without lint suppressions.
  The lint rerun remains user-owned and pending.
- Registry entries own control, matching callback and cadence in one typed
  payload; public policy is derived. Separate policy/callback discriminants and
  the callback-absent state are removed. Pure fixtures use inert typed callbacks.
  `Entry::new` is now const following the maintainer's Clippy report; rerun pending.
- Release version display, mutation, staging, truth, commit and tag checks share
  one workspace-package-scoped reader/writer. Locked metadata also checks the
  resolved timer package version. Ordering, malformed metadata, rejected staging
  and coherent-but-wrong-version fixtures are prepared.
- PocketIC provisioning selection is exercised through the actual Make recipe
  with a recording checker instead of source-line assertions. Audited pins and
  independent binary verification remain. Staging ownership and the superseded
  historical Watchdog restriction are clarified.
- PocketIC cache and download checks share one hash-first verifier; diagnostics
  no longer re-execute rejected candidates. Explicit overrides and temporary
  download/cache cleanup rules are preserved, with extended fixtures prepared.
- The 2026-10-03 GitHub review found one open issue, advisory
  [#8](https://github.com/dragginzgame/ic-timers/issues/8). Its IcyDB-shaped test
  cleanup separates readiness mapping, repeated message driving and measurement
  checks while keeping the ordered lifecycle sequence. The issue remains open;
  supplemental MSRV lint and unit execution await maintainer validation. Gate
  scope is unchanged.
- Target release: `0.9.2`. Its undated changelog section is directly below the
  empty `Unreleased` heading. The [0.9.2 note](../changelog/0.9.2.md) records scope
  and pending validation. Cargo and both lockfiles remain at 0.9.1 until the user
  runs the patch bump.

## Evidence

The 0.9.1 release state combines local artifact inspection with the maintainer's
live report. Exact final gate output and independent publication evidence were
not inspected in this follow-up. The automated contributor has not run new
test/build/lint gates. Earlier recorded evidence remains scoped to its subjects;
release artifacts do not establish new recovery, performance or memory results.
The audit follow-up formatted changed Rust and checked shell syntax and diff
whitespace. Its registry and release-tooling changes await maintainer execution
of the prepared fixtures and existing behavior gates; no new result is claimed.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims.

## Downstream state

The preceding 2026-10-03 inspection recorded Toko Miner at `3354dfc6`, selecting
timers 0.8.1, Canic 0.110.51 and IcyDB 0.264.4 with one timer package. Canic's
published exact 0.8.1 pin remained an adoption blocker under issue #33. Later
inspected worktrees selected newer timer lines without proving publication or
deployment. The dated [Toko record](../adoption/toko-miner.md),
[Canic](../adoption/canic.md) and [IcyDB](../adoption/icydb.md) records remain
scoped evidence; no sibling repository was edited.

Application-owned work remains auxiliary checkpoint deadlines, registration
identity projection and qualification against actual artifacts. Registration
continuity does not prove balance attribution or complete-message costs;
Wasm/stable page extents do not prove allocator live-byte bounds.

## Next action

0.9.1 is already bumped, committed and tagged; do not repeat its bump. The 0.9.2
changelog and release note are prepared. The maintainer owns targeted checks,
supplemental MSRV lint validation and the deployment gate, then the patch release.
The bump helper dates the staged section and updates package/lockfile versions;
the automated contributor has not executed it or staged any files. All release
execution remains user-owned.
