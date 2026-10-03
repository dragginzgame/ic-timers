# Current status

Last updated: 2026-10-03

## Purpose

This is the compact handoff for a new session. `ic-timers` wraps the CDK timer
provider with one bounded registry, policy arbitration, observations and explicit
lifecycle reconstruction. Historical implementation and validation detail belongs
in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md) and
the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.8.4`.
- Latest release line: `0.8.4`.
- The maintainer reports 0.8.4 live. Its release commit is
  `058f968d012168bdae69d23f9ddb5cc46d9e34dd`, with annotated `v0.8.4`, dated
  changelog and matching root/testing lockfiles. The preceding feedback review
  corroborated registry publication and successful hosted CI/MSRV/tag checks;
  see the [0.8.4 note](../changelog/0.8.4.md).
- The in-depth audit follow-up is prepared for 0.9.0. Exact ordinary completion
  arbitration and its directive observation change public semantics, requiring
  the next minor line. No legacy switch, alias or alternate contract remains.
  Cargo and both lockfiles have not been bumped. See the
  [undated 0.9.0 note](../changelog/0.9.0.md).
- Version mutation, staging, commits, tags, pushes, publication, all release
  commands, tests and build/lint gates are user-owned. Automated contributors
  implement requested work and prepare changelogs/notes without executing those
  gates unless explicitly requested.
- Direct provider: exact `ic-cdk-timers` 1.0.0; exact `ic0` 1.2.0. Probe canisters
  use exact `ic-cdk` 0.20.3. MSRV is Rust 1.88.0; development/hosted CI uses 1.99.0.

## Canonical runtime

- One volatile canister-local runtime owns at most 64 structured identities,
  declaration claims, callback generations, policy states, pending commands,
  callbacks, observations and provider handles.
- Once and AfterCompletion accept asynchronous work. Watchdog accepts one
  synchronous bounded unit and uses two messages: the scheduler commits the
  cadence successor before separately queued consumer work.
- Only the private platform module calls the provider and IC system facts.
  Snapshots and `has_armed_wakeup()` are inert observations, never scheduling
  authority or delivery guarantees. Callback contexts expire with their exact
  work token; retained registration claims own longer-lived control.
- Consumer-owned durable authority reconstructs retained declarations through
  synchronous lifecycle helpers before downstream hooks. Timer authority is
  volatile and never restored from persisted snapshots or handles.
- Ordinary exact reconciliation now replaces a discarded callback scheduling
  proposal before validation. Explicit invariant failure remains terminal;
  unregister is sticky. The latest directive projects the effective exact
  ScheduleAt value when reconciliation wins. Ensure still selects earliest
  demand. New precedence assertions await maintainer execution.
- Registry/handle bounds do not cap the pinned provider's heap. Cancelled future
  deadline records remain queued until processing. Frequent replacement and
  immediate continuation can grow memory and cleanup cost; there is no provider
  compaction authority here. Prepared churn fixtures measure page extents without
  promising a global memory or cost bound.

## Prepared audit follow-up

- Native suspended futures use wake notifications and no longer block unrelated
  due timers. New fixtures cover suspension, outside control, context expiry and
  completion-time recurrence, plus exact-command precedence over invalid proposals.
  The suspension helper now carries a scoped `future_not_send` expectation for
  its intentional single-threaded `Rc` state, correcting the reported lint failure;
  the maintainer's rerun remains pending.
- Real-canister fixtures hold an ordinary callback across a self-call await while
  ingress and another Watchdog proceed. Provider-churn fixtures exercise distant
  replacement, bounded immediate work, cancellation and eventual deadline cleanup.
- Release validators count markers independently of their value and reject
  conflicting workspace/latest-release markers, duplicated headings and statuses.
  Free-form prose remains advisory, never a post-mutation blocker.
- The commit hook checks an index snapshot without rewriting working copies or
  staging files. Partial staging is allowed when the staged snapshot is formatted.
- Release commit/tag retries verify the matching clean release commit and exact
  annotated tag. Arbitrary commits and conflicting tags are rejected. Combined
  release targets still always bump; interrupted final phases use their documented
  phase targets. Full deployment gates and pinned PocketIC verification remain.

## Evidence

The 0.8.4 note records published baseline evidence. Its hosted checks and earlier
native/PocketIC results do not qualify these subsequent changes. The new native,
PocketIC and shell fixtures are written and wired into existing gates but have not
been run by the automated contributor. No new recovery, performance, allocator or
memory-growth result is claimed. The native mock does not simulate IC rollback or
provider heap allocation; real-canister subjects remain necessary for those facts.

## Downstream state

The preceding 2026-10-03 inspection recorded Toko Miner at `3354dfc6`, selecting
registry timers 0.8.1, Canic 0.110.51 and IcyDB 0.264.4 with one timer package.
Canic's published exact 0.8.1 timer pin remained an adoption blocker under issue
#33. Later inspected Canic and IcyDB worktrees selected newer timer lines; that
does not establish their publication or a deployed Toko artifact. The dated
[Toko record](../adoption/toko-miner.md) separates source inspection from earlier
managed qualification. [Canic](../adoption/canic.md) and [IcyDB](../adoption/icydb.md)
adoption records remain scoped evidence. No sibling repository was edited.

Application-owned work remains auxiliary checkpoint deadlines, registration
identity projection and qualification against actual artifacts. Registration
continuity alone proves neither balance attribution nor complete-message costs;
Wasm/stable page extents do not prove allocator live-byte bounds.

## Next action

Review the prepared 0.9.0 implementation and unexecuted fixtures. The maintainer
stages and commits the work, then runs `make release-minor` to execute the full
deployment gate and bump 0.8.4 to 0.9.0. The helper dates the changelog and updates
both lockfiles. This semantic hard cut cannot be released as a patch. If release
execution stops after the bump, use the phase recovery in
[releasing](../releasing.md) rather than requesting another bump.
