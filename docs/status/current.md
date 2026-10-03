# Current status

Last updated: 2026-10-03

## Purpose

This is the compact handoff for a new session. `ic-timers` wraps the CDK timer
provider with one bounded registry, policy arbitration, observations and explicit
lifecycle reconstruction. Historical implementation and validation detail belongs
in [release notes](../changelog/README.md), [audits](../audits/code-hygiene.md) and
the [safety boundary](../../SAFETY.md).

## Release state

- Workspace package version: `0.9.1`.
- Latest release line: `0.9.1`.
- Local release commit `9afb9a1be93c159400c1ff3cb63515f73d2a1d7e` has annotated
  `v0.9.0`, a dated changelog and matching root/testing lockfiles. The existing
  local `origin/main` ref points to the same commit. Publication and successful
  final deployment output were not verified in this review. See the
  [0.9.0 note](../changelog/0.9.0.md).
- The preceding feedback review corroborated 0.8.4 publication and hosted checks;
  those are historical baseline evidence in the [0.8.4 note](../changelog/0.8.4.md).
- The in-depth audit follow-up is included in 0.9.0. Exact ordinary completion
  arbitration and its directive observation change public semantics, requiring
  the next minor line. No legacy switch, alias or alternate contract remains.
  Cargo and both lockfiles now select 0.9.0.
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

## 0.9.0 audit follow-up

- Native suspended futures use wake notifications and no longer block unrelated
  due timers. New fixtures cover suspension, outside control, context expiry and
  completion-time recurrence, plus exact-command precedence over invalid proposals.
  The suspension helper now carries a scoped `future_not_send` expectation for
  its intentional single-threaded `Rc` state, correcting the reported lint failure;
  the maintainer's rerun remains pending.
- Real-canister fixtures keep ordinary work awaiting self-call gate replies while
  ingress and another Watchdog proceed. Provider-churn fixtures exercise distant
  replacement, bounded immediate work, cancellation and eventual deadline cleanup.
  The stored-waker gate was replaced after the maintainer reported no committed
  suspension. Closed-gate replies now lead to another call, and observations
  expose completed gate replies and call/decode errors. The corrected subject
  awaits the maintainer's rerun.
- Release validators count markers independently of their value and reject
  conflicting workspace/latest-release markers, duplicated headings and statuses.
  Free-form prose remains advisory, never a post-mutation blocker.
- The commit hook checks an index snapshot without rewriting working copies or
  staging files. Partial staging is allowed when the staged snapshot is formatted.
- Release commit/tag retries verify the matching clean release commit and exact
  annotated tag. Arbitrary commits and conflicting tags are rejected. Combined
  release targets still always bump; interrupted final phases use their documented
  phase targets. Full deployment gates and pinned PocketIC verification remain.

## 0.9.1 simplification follow-up

- Running unregistration records the registry's pending request directly;
  unreachable ordinary control-failure and scheduler-role branches are removed.
  Control tests cover local transitions; registry arbitration coverage remains.
- Effect confirmation retains idempotence with one wakeup-generation marker.
  The redundant Watchdog work-generation marker is removed; work-token validation
  precedes replay deduplication. Existing fixtures cover repeated dispatch
  confirmations and malformed work after a valid confirmation.
- Release truth and automatic staging no longer depend on the historical Canic
  adoption document. Make fixtures now observe gate execution, phase order and
  repeated bumps for every release flavour instead of matching recipe text.
- Rust owns provider alias/export visibility. The facade forbids private-interface
  leaks; the checker retains direct-provider confinement and restricted platform
  declarations. Compiler fixtures include suppression and external glob access,
  and hosted CI runs them on the current toolchain and MSRV.
- Probe output deserialization and its direct serde dependency are removed.
  The nested lockfile changes only that dependency edge; versions are unchanged.
- Architecture prose scopes the old composition blocker and later frozen receipt.
- These changes await maintainer validation. The
  [0.9.1 note](../changelog/0.9.1.md) records scope and evidence.
  Formatting, shell syntax, diff whitespace and both locked offline metadata
  checks passed; test, build and lint gates were not run.

## Evidence

The 0.8.4 note records published baseline evidence. Its hosted checks and earlier
native/PocketIC results do not qualify these subsequent changes. The new native,
PocketIC and shell fixtures are written and wired into existing gates but have not
been run by the automated contributor. No new recovery, performance, allocator or
memory-growth result is claimed. The native mock does not simulate IC rollback or
provider heap allocation; real-canister subjects remain necessary for those facts.
The maintainer reported ten PocketIC subjects passing and one ordinary-await
subject failing before the gate correction; this does not qualify that correction.

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

0.9.0 has already been bumped, committed and tagged; do not repeat its bump.
Successful output for the corrected ordinary-await subject has not been supplied
in this conversation. The maintainer owns any further validation and publication.
Use the phase recovery in [releasing](../releasing.md) when finishing an existing
release; combined release targets always request a new bump.

The earlier release-prose correction is retained alongside the private runtime,
boundary-enforcement, probe and release-tooling simplifications. Their notes are
under an undated `0.9.1` section immediately below the empty `Unreleased`
heading. The maintainer owns validation and the patch release. Stage and commit
the prepared work, then run `make release-patch` to validate and bump to 0.9.1.
No package version was changed; the nested lockfile only drops the removed direct
probe dependency. The automated contributor did not stage, commit, tag or push
this follow-up.
