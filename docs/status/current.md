![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-06

## Purpose

This is the compact session handoff. Historical implementation, delivery
references and validation belong in [release notes](../changelog/README.md),
[audits](../audits/code-hygiene.md) and the [safety boundary](../../SAFETY.md).

## Release state

- Read `[workspace.package].version` in [Cargo.toml](../../Cargo.toml) for
  package identity. The top [changelog section](../../CHANGELOG.md) records the
  accepted batch under its automatically selected, undated next version.
  The maintainer's bump finalizes and dates it; a dated section alone does not prove
  tagging, publication or deployment.
- The maintainer reports 0.13.4 live. Cargo, both lockfiles, finalized changelog
  and local tag identify release commit `aa0e933eb56ff0e3e6832ad822f77c95a1392199`.
  Publication was not independently checked. Tag CI and main MSRV passed, but
  main Linux checks and both native macOS complete release gates failed in the
  new release fixtures. The last complete qualification remains 0.13.3 at
  `864397a`: Linux/MSRV and both macOS gates passed, with 142 native tests,
  14 PocketIC runtime subjects and policy cohorts in the Apple Silicon log.
  Neither source qualifies the pending repair. See the
  [source-bound host record](../releasing.md#host-support).
- Released 0.13.3 covers a compatible callback-capture destruction fix
  and removal of handle detachment/reinstallation for rejected/coalesced public
  requests. API, snapshots, recurrence, generations and dependencies are unchanged.
  Removed captures remain in their returned transition until provider cleanup
  finishes; rejected factories retain a local Rc across registry access. Native
  and PocketIC capture-access fixtures passed in matching hosted qualification.
  Duplicate ensure costs fell about 27–30%; initial arm rose about 4%, and
  recurring cancellation about 5% in the maintained operation probes. These are
  operation intervals, not a universal speedup or total-message cost.
  Tests, lint/build, MSRV, PocketIC, cohort measurement
  and release execution remain maintainer-owned. The
  [delivery ownership contract](../design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests)
  records implementation, baseline cost, temporary allocation and acceptance.
- Released 0.13.4 covers compatible repository-only release recovery
  for [#10](https://github.com/dragginzgame/ic-timers/issues/10), using reviewed
  Shared Tooling `cb86188`. Late adapters inspect exact `RELEASE_COMMIT` metadata
  and tags separately from HEAD; normal commands finish an older committed
  release before fresh validation for newer fixes or another requested increment.
  Current adapter code owns checks, with no execution of old snapshot scripts.
  The 22-file snapshot includes the maintenance rule, hook failure fix and
  canonical validation logger. Preflight rejects hidden staged implementation
  edits, commit admission checks the exact prepared index, and real release-gate
  failures retain unique raw logs across retries outside tracked release inputs.
  Preflight now delegates selected dependency-cache preparation to `make fetch`
  before validation, avoiding an offline cache check that blocked cold checkouts.
  The real-index fixture covers failed Git producers/partial output and both
  locked fetches, preserving metadata and failing before release intent. No
  dependency fetching was performed during contributor preparation.
  The host-only cohort now appends exact loaded `wasm_bytes` to its existing
  baseline/policy rows. Old hosted logs omitted byte sizes; no historical Wasm
  delta is inferred. This changes evidence output only, with no target Wasm
  instrumentation or runtime behavior change.
  Contributor source/snapshot/syntax/diff inspection was preparation evidence;
  the subsequent hosted attempt failed as recorded above. Timer runtime and
  dependency selections are unchanged. A repository-only patch retains the
  complete gate rather than using tag success as complete qualification.
- The undated 0.13.5 draft repairs those compatible repository-only failures.
  Metadata and Git admission functions explicitly propagate failed commands
  instead of relying on Bash 3.2 subshell errexit. Sort and lock failures are
  injected independently for both workspaces. The index fixture deliberately
  uses a shallow clone with its own controlled baseline tag, satisfying impact
  classification without remote history or production changes. Both fixtures
  report captured adapter output before failure cleanup. Source, shell syntax
  and diff inspection remain preparation evidence; no tests/builds/lint gates
  were run. The release-check repair leaves Cargo and runtime sources unchanged.
  A separate concurrent dependency update now requires `ic-metrics 0.1.6` and
  selects it in the root lock; the inspected testing lock still selects 0.1.5
  and needs alignment before locked validation. Those edits are preserved and
  are outside the repair's qualification. Native and hosted qualification remains
  user-owned; #10 stays open until it succeeds.
- Apply the pinned [Shared Tooling baseline and local overlay](../../AGENTS.md).
  Its [adoption record](../shared-tooling.md) scopes provenance and exceptions.
  Historical release notes retain evidence; no new versioned note or mutable
  handoff release marker is required for version preparation.
  During the 0.13.0 adoption, shared measurement arithmetic selected published
  registry `ic-metrics 0.1.3`; attribution and registration identity remained local.
  The root dependency and both lockfiles selected one registry package, preserving
  every other lock record and consumer package version. Locked offline metadata and manifest
  sorting pass for both workspaces. Warning-denied library Clippy, four measurement
  tests and focused registration/reset identity, stale delivery, discarded delivery,
  binding-failure and normal-completion checks pass on Linux. The initial Clippy
  gate reported three guard diagnostics; the complete guard is now captured until
  normal completion, and fallible borrowing uses `let ... else`. A completion
  fixture initially aborted at native thread-local teardown with a timer left
  armed; explicit unregistration corrects fixture cleanup. Two earlier filters
  matched no tests; exact names supplied the recorded evidence. This is focused
  native-substitute evidence, with no broad suite or PocketIC qualification.
  [Registry adoption](https://github.com/dragginzgame/ic-timers/issues/9) was
  committed by the maintainer at `685b4ff` during these checks, with the compatible
  `0.1.3` requirement and both locks still selecting the verified package. That
  adoption is included in the tagged 0.13.0 source; the focused evidence remains
  separate from broad native, release and CI qualification.
  The maintainer's subsequent test-target lint reported an empty-slice assertion,
  missing unit-expression semicolon and redundant identity clone in delivery
  fixtures. Those are corrected without changing fixture semantics; source
  formatting and diff checks pass, while the lint rerun remains user-owned.
  The subsequent broad native run aborted at TLS destruction in a liveness
  fixture with its replacement timer still armed. All runtime setup fixtures now
  retain scoped cleanup for queued and suspended fake tasks before TLS teardown,
  dropping outside the task-map borrow while runtime TLS remains available.
  A focused cleanup fixture is added; production behavior is unchanged. Native
  execution remains user-owned and pending, with scope recorded in the
  [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
  Standard SemVer releases use the refreshed shared runner: preflight/validation
  failures restart on current source, while normal targets automatically
  reconcile prepared intent at its saved version before a new increment. The
  local retry wrapper and its duplicate fixtures are removed; no release was
  executed during adoption. The shared hook formats only fully staged selected
  files, and the local formatting/gate owners cover both workspaces with pinned
  cargo-sort 2.1.4. Local hook activation remains separate from snapshot adoption.
  The 0.13.0 minor boundary covers the public Abandoned variant and ordinary
  delivery-retirement semantic cut; consumers adopting it must handle both.
  Snapshot integrity, shell syntax and metadata-preserving manifest sorting
  passed during snapshot adoption; both lockfiles were then byte-identical. The GitHub description matches
  current scope. New recovery/hook fixture execution remains user-owned; source/snapshot scope
  belongs in the [release guide](../releasing.md).
  The reviewed baseline is now `cb86188c5956866564de4fb6ec6be67b27981ab9`, with
  shared rules in `DRAGGINZGAME.md`. Maintainer-owned validation and release
  exceptions remain explicit; the [local host matrix](../releasing.md#host-support)
  records required macOS workflows and their unresolved qualification.
  Repository and release-gate fixtures now compare ordered records directly,
  removing their `mapfile` dependency; scoped verification remains in that matrix.
  Preservation fixtures retain file copies and compare bytes and permission bits,
  without checksum manifests, separate mode tables or GNU in-place sed. Preparation
  phase/staging comparisons use direct records and retain Git failure propagation.
  Clean-worktree and release-commit guards reject failed Git queries; their new
  rejection fixtures remain unexecuted. PocketIC fixture events use exact record
  comparisons, with scoped source/syntax review recorded in the host matrix.
  Version preparation and release commits now capture exact tag listings before
  checking absence, rejecting failed lookups before mutation and after a release
  commit. Fixtures cover empty/matching failure output and retrying the same
  untagged commit; source/syntax review is recorded in the release guide, and
  those scenarios remain unexecuted.
  Repository checks also reject failed workflow discovery and provider-source
  searches/orderings before accepting records. The existing repository fixture
  covers producer failures and discovery cleanup; its pending execution and
  syntax/source-review scope are recorded in the release guide.
  The user-operated release gate now prepares both locked dependency caches
  before validation, after the reported offline `js-sys` metadata failure during
  version preparation. Metadata parsing preserves Cargo's original failure.
  Fetch-order and metadata-failure fixtures remain unexecuted; source/syntax
  scope and the failed attempt are recorded in the release guide.
  Hosted tag CI now validates the tagged checkout's actual root/testing lockfiles
  using those existing fetch and metadata owners after tag/main checks and before
  fixtures. This repository-only continuation is in the current changelog section.
  Workflow syntax/source-review scope and pending hosted execution are recorded
  in the release guide.
  Release-impact classification now reads NUL-delimited Git paths after both
  queries succeed, preserving crate classification for display-quoted filenames
  without joining or sorting. Filename, partial-failure and cleanup fixtures
  remain unexecuted; their source/syntax scope is recorded in the release guide.
  PocketIC provisioning now verifies pinned archives before decompression and
  retains exact host-specific binary/version checks for Linux x86_64 and macOS
  Intel/Apple Silicon. macOS 15 PR/main jobs run the complete release gate under
  Apple's Bash 3.2 with both pinned Rust toolchains. Artifact-pin provenance and
  the declared host matrix belong in the release guide; fixtures and native
  qualification remain unexecuted. Empty-argument fixtures use positional
  arguments to avoid Bash 3.2 nounset behavior. No runtime API, Cargo version,
  lockfile or release execution changed.
  Combined release commands now reuse the commit guard's read-only worktree
  admission before validation and bumping. Only the five bump-owned metadata
  outputs may remain unstaged; other paths are reported without auto-staging.
  Standalone version preparation keeps its dirty-worktree contract. Source and
  syntax scope and pending fixture execution belong in the release guide.
  The bump helper now rejects missing or extra arguments and misplaced check
  flags before reading metadata. PocketIC requires regular executable candidates
  and rejects invalid provisioning destinations before downloading, preserving
  verified file symlinks as read-only input. Updated rejection/preservation
  fixtures remain unexecuted; source/syntax scope belongs in the release guide.
- The current ordinary callback API is policy-specific: Once and
  AfterCompletion entry points require separate result/decision types.
  Shared public results/directives are removed; the private erasure and canonical
  arbitration remain. Current probes and fixtures use the typed API, and both
  test/MSRV owners include API doctests. The implementation and validation
  scope belong in the [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-callback-results).
  Cargo identity and both lockfiles remain maintainer-owned. The return-type cut
  uses a minor release with coordinated downstream adoption, without shims.
  Borrow-rejection fixtures now cover both ordinary policies/lifetimes, and
  public recurrence fixtures cover each completion classification. The README
  describes requested recurrence after normal return. Automated review did not
  execute these fixtures; their evidence scope belongs to the same callback
  contract.
- Public removals or incompatible semantic changes require the next minor line;
  private behavior-preserving simplifications may use a patch. See the
  [ordinary arbitration hard cut](../changelog/0.9.0.md).
- Version mutation, staging, commits, tags, pushes, publication, release commands,
  tests and build/lint gates are user-owned. Automated contributors implement
  requested changes and prepare changelogs/notes without executing those gates.
- Direct provider: exact `ic-cdk-timers` 1.0.0; exact `ic0` 1.2.0. Probe canisters
  use exact `ic-cdk` 0.20.3. MSRV is Rust 1.88.0; development/hosted CI uses 1.99.0.
- The host-only test harness now uses published exact `ic-testkit` 0.17.3 and
  one resolved PocketIC 16.0.0 client, with caller-owned servers and bounded
  startup from the verified binary. The nested dependency lock is updated.
  Official Linux/macOS archive hashes and executable headers were inspected
  without executing binaries; compilation, lint, host recovery and cohort
  qualification remain pending in the [release guide](../releasing.md#testkit-harness-qualification).
- The isolated version-preparation fixture now includes the shared increment
  helper and tests current Makefile delegation instead of the removed standard
  recipe sequence. The shared runner fixture is included in `release-check`;
  corrected fixture execution remains maintainer-owned, with scope recorded in
  the [release guide](../releasing.md).
  The maintainer's later gate passed the preceding release fixtures but failed
  the missing-changelog rollback assertion because its lock-update injector had
  already been restored. The injector now remains active through that case;
  the case verifies the injected failure, rollback and metadata preservation.
  Source/syntax checks pass; corrected fixture execution remains pending. The
  production bump helper and workspace versions are unchanged by this repair.

## Canonical runtime

- The 0.13 runtime implements retirement of confirmed ordinary deliveries dropped
  before normal completion. A queued-token guard is created before the first poll;
  exact ownership checks exclude stale, cancelled, replaced and unconfirmed work.
  Retained state becomes Abandoned/Failed with Unacknowledged accounting; transients
  and pending unregister are removed, without provider calls or fabricated completion.
  Normal live-await control and Watchdog prearming remain separate. Native drop and
  PocketIC pre-await/continuation trap fixtures are unexecuted; cleanup qualification
  and cost deltas remain pending with the [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
  The finalized 0.13.0 section records the minor semantic cut; finalized 0.13.1
  records the shared instruction reader and documentation. Package versions and
  lockfiles remain maintainer-owned.

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
  claim-transition validation boundary before applying control; see the
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
  Late measurements use that lookup while retaining no-op behavior for missing or
  superseded claims; see the
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
  Running cancellation remains a pending command applied on normal completion;
  confirmed ordinary abandonment retires the delivery and discards that command
  under the 0.13 contract above.
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
  Work delivery consumes the matching fired handle through that same selected
  entry. Watchdog scheduler consumption and remaining-handle detachment share
  one lookup before the transition; see the
  [callback delivery evidence](../design/callback-delivery-ownership.md).
- Watchdog completion decides lifetime removal once from final inactive state
  and pending unregister, after selecting its successor or terminal transition;
  see the [Watchdog completion removal note](../changelog/0.10.10.md).
  Terminal request and scheduler failures decide lifetime removal from their
  already selected entry, without another lookup; see the
  [ownership and verification scope](../architecture.md#terminal-removal-verification).
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

Latest inspected release: 0.13.4 at `aa0e933`, tag CI and main MSRV passed, while
main Linux checks and both native macOS gates failed in release fixtures.
The pending 0.13.5 repair has no matching remote CI. The last complete hosted
qualification is 0.13.3 at `864397a`; Apple Silicon records 142 native tests,
14 PocketIC runtime subjects, doctests and policy cohorts. The preceding
0.13.2 qualification at `134899f` retains its own 138 native/13 PocketIC scope.
Exact
links and qualification scope belong in the [host matrix](../releasing.md#host-support).
The maintainer authorized closing [#9](https://github.com/dragginzgame/ic-timers/issues/9)
with that integration/CI evidence; it is closed. Issue #10 is the only open issue
and no open PR covers it; the released consumer repair needs the pending fixture
corrections and successful native qualification. No GitHub writes were made
during this repair. The earlier authorized ic-metrics comments on #1, #3 and #4
record IC Timers 0.13.3 consumer qualification; they do not qualify 0.13.4.
The committed Shared Tooling head and remote main matched adopted `cb86188`;
its upstream Linux regression and lint/security passed, but both macOS jobs
failed at snapshot-distribution fixture source-path admission after passing the
shared logger tests. The diagnosis and remaining qualification boundary belong
in the [adoption record](../shared-tooling.md). The preceding `9437bab` Linux and
both macOS upstream gates passed, qualifying that recovery source alone.
Later sibling changes were initially dirty and excluded; after they were
committed at `cb86188`, a separate clean reviewed export adopted them.
No tests were executed locally during this continuation. Shared Tooling's only
new inspected changes are uncommitted snapshot-fixture path correction and its
changelog; they are not a new adopted baseline. The working-tree 0.13.5 repair
has no matching remote CI evidence. See the
[release runner evidence owner](../releasing.md#standard-release-runner).

Earlier inspection records follow; their pending/failure language describes
those earlier sources and times, not the current released baseline.


On 2026-10-06 the committed Shared Tooling head and remote main both remained
`f52c0e2476aee094359ed21de91c468540d3969f`; the 20-file snapshot verified. The
dirty upstream maintenance rules remain outside the reviewed snapshot. Both
0.13.1 macOS jobs failed the release fixture's logical-versus-physical default
cache comparison, while Linux checks passed. MSRV and tag jobs did not acquire
hosted runners. The 0.13.2 fixture repair retains all gate and override assertions;
its source/syntax scope and pending native rerun are recorded in the
[host matrix](../releasing.md#host-support). No tests, builds, lint gates, version
mutation or release execution were performed during this repair.

The extraction workflow also ran the previously authorized focused qualification
for the shared-reader wiring: native and Wasm library Clippy with warnings denied,
Rust 1.88 Wasm compilation, and four named native tests for role-specific
measurement saturation, completion totals, registration identity and abandoned
work. They passed while Cargo still identified the package as 0.13.0. The
maintainer subsequently committed that exact platform source and selected
0.13.1; its release commit changed metadata only. Both locked workspace graphs
resolve one registry ic-metrics 0.1.5 with `ic`, preserving every external
selection except metrics. These checks use the consumer/test-owned native
substitute; they do not establish timer PocketIC or full release qualification.


A 2026-10-05 source review traced the 0.13.1 reader through the downloaded registry
source to `ic0::performance_counter(1)`, and reviewed ordinary delivery ownership
and scoped native fixture cleanup. This is source evidence, with no new test,
build, lint, PocketIC or cost measurements. GitHub's main and tag CI runs for
`54bbcfc4985d4657578150cbe7112795297115fd` were queued when inspected:
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535919) and
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535920).
At that earlier inspection, [issue #9](https://github.com/dragginzgame/ic-timers/issues/9)
was open for owning native release/CI qualification; the later 0.13.2 gates
supplied it and the maintainer authorized closure.
No open PR was listed at that earlier inspection. Shared Tooling's committed
head then remained the adopted revision; its maintenance-rule draft was outside
that earlier reviewed snapshot.

Inspected hosted validation for 0.9.4 is scoped in its
[delivery note](../changelog/0.9.4.md). Release reports and Git references alone do
not establish new hosted or PocketIC results. Each change's verification record
belongs with its design/evidence owner rather than a repeated handoff claim.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims. Tests, builds,
lint gates and deployment validation remain user-owned.
Measurement ownership across identity reuse and policy-specific completion
failure assertions are scoped in the [0.10.16 note](../changelog/0.10.16.md).
Public lifecycle rejection fixtures cover identity, lifetime, cadence, expired
claims and occupied-identity reconstruction from an empty slot; their pending
verification is scoped in the [0.10.21 note](../changelog/0.10.21.md).

## Next action

Establish package identity from Cargo and release state from actual Git and
publication evidence, then continue the maintainer's requested work within the
ownership boundaries above. Update the one changelog draft and record scoped
verification with its owner. Do not select another release version during ordinary continuation.
Leave Cargo versions, both lockfiles and Git release execution to the maintainer.
Fresh release commands perform the requested bump. Retries first reconcile saved
intent, then advance only when the common recovery contract selects a follow-up.
Preparing notes does not advance the workspace version.
