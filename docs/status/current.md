![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-07

## Purpose

This is the compact session handoff. Historical implementation, delivery
references and validation belong in [release notes](../changelog/README.md),
[audits](../audits/code-hygiene.md) and the [safety boundary](../../SAFETY.md).

## Current tooling batch

Prepared compatible, undated **0.14.7** for
[#20](https://github.com/dragginzgame/ic-timers/issues/20): adopt committed
Shared Tooling 0.1.14 at `25e7ce83149e081e4dcc52c55c33724e44153f2a` across all three
snapshots, with 27/19/17 files. Exact-source
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37586649650)
passed Linux, lint/security and both native macOS jobs. Exported bytes/modes are
retained; dirty sibling changes are excluded and no sibling files are modified.

The shared Make guard precedes release-runner, logger and hook effects and is
included in source snapshots and consumer fixture exports. Local `release-x`
uses it first in one recursive `set -e` shell sequence, preventing later phases
under inherited ignore-errors or non-executing modes. Outer Make -i can still
ignore that recipe's status; no release effects are admitted. Version-only outer
Make dispatches nothing. Fixture coverage checks inherited short/long modes,
index/metadata preservation, restricted PATH and nested `-j2` release selections.
There is no local mode parser or patched shared source.

The continued .7 batch adds local exact-release bump/stage/commit/push failure
cases, checking that only the phase prefix executes and the failure propagates.
Hook and version-preparation fixtures now retain failed scratch directories and
print their paths; successful runs still clean up. Separate scenario logs retain
Make admission and exact-release output, and normal formatter failures also print
their captured output. These are prepared fixtures, not new executed evidence.
No new committed upstream revision was available at the follow-up re-check;
dirty Shared Tooling/metrics maintenance remains excluded.

The maintainer authorized working through #20/#22/#23. The #20 implementation
is complete in this draft; source-bound consumer qualification remains pending.
[#23](https://github.com/dragginzgame/ic-timers/issues/23) now has final failure
collection/upload steps in all four workflow jobs, including both macOS matrix
hosts. Job-owned TMPDIR collects fixture scratch; a local collector archives
that tree, raw validation failures and failed host/IC installer candidates,
preserving modes and symlinks without fixture Git metadata. It does not select
the checkout's installed tools or build caches. Artifacts identify source, job,
host and attempt, using the reviewed
official upload action 7.0.1 at `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`.
The maintained release gate selects collector fixtures for contents, modes,
identity, symlinks, exclusions, empty inputs and archive failure propagation.
No fixtures or controlled failing hosted runs have executed during preparation;
#23 stays open for native and hosted artifact qualification. See
[CI failure evidence](../releasing.md#ci-failure-evidence).

Preparation checks snapshot integrity, exact export bytes/modes, shell syntax,
documentation and diff whitespace. Tests, builds, lint, formatter and deployment
validation remain maintainer-owned and have not run for this draft. #20 remains
open pending complete source-bound consumer qualification. This is repository-only
work; runtime/probe source, Cargo versions, dependency selections, both lockfiles,
public API and the audited PocketIC gate remain unchanged. No functions, methods
or types are removed. No Wasm/instruction/heap savings are claimed.
See the [adoption owner](../shared-tooling.md#shared-tooling-0114-make-admission).

Released 0.14.6 at `0c90c391dff5960a7502fc15a0718b03631f2515` adopted Shared Tooling
0.1.13 and moved the three testing packages under their independent `crates/`
root, preserving package identities and both lock graphs. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584151377)
passed Linux, MSRV and both complete native macOS gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584150869) passed.
#19 was closed with this complete qualification. The maintainer reports it live;
registry publication was not independently checked. Released changelog history
is preserved.

Read-only review of ic-metrics identifies committed release 0.2.4 at
`21e980b3ed4f1a8d9b6203079ef1e457a3fea588` and passing
[upstream CI](https://github.com/dragginzgame/ic-metrics/actions/runs/37587072330).
It adds bounded histograms without changing our existing summary implementation.
The dirty next 0.2.5 draft is tooling-only and excluded. Root/testing locks still
select 0.2.3/0.2.0. [#22](https://github.com/dragginzgame/ic-timers/issues/22) records
work-histogram cost and retained-sample bias: histogram storage would replace the
summary internally and its recording cost lies outside our current instruction
envelope. The maintainer-authorized evaluation concludes without adopting
histograms; #22 closes as not planned. Current probes use summary samples/totals
and do not establish production histogram bounds or an acceptable per-timer
recording/storage budget. Upstream publication evidence is now recorded in #22;
availability is not a blocker. Reopen only for a named application need and cost
qualification. The
[measurement owner](../design/callback-delivery-ownership.md#histogram-evaluation)
records the decision. No dependency update or runtime instrumentation ran.

## Released 0.14.5 tooling

Released compatible 0.14.5 at `c84d4e4` implements
[#17](https://github.com/dragginzgame/ic-timers/issues/17) and
[#18](https://github.com/dragginzgame/ic-timers/issues/18). That release's three exact snapshots
identify clean committed Shared Tooling 0.1.12
`33c2a6f0018a94915f819ff219e270500ed5b73b`: 25 baseline files, 19 audit/setup files
and 17 nested helpers. Overlapping root integrity records were refreshed together.
The paired maintenance policy is adopted while tests/builds/lint and all release
execution remain maintainer-owned. Sibling file edits still require their own
authorization. The tooling batch leaves timer source and dependency selections
unchanged.

`release-check` directly selects the shared release-command checker with the
actual root and `tool-versions.env`. The copied `test-standard-release.sh` is
removed with no wrapper; local metadata/index/lock/recovery fixtures remain.
No named function/type was deleted. Standard release pushes now recheck and use
the captured sole destination URL. Snapshot verification hashes files without
executing inspected code. A new isolated fixture checks all three actual exports
and rejects payload-only, helper-only and combined corruption without executing
a changed helper. Failed scratch/logs remain retained.

The referenced read-only CI helper and flat governance file list are adopted;
existing guides are checked in the export. The list does not automatically widen
consumer manifests. Optional shared suite prerequisites, standalone yq setup and
upstream artifact-upload workflows are not introduced. Local host/IC pins, system
ripgrep preparation, independent workspaces and audited PocketIC gate remain.
See the [adoption owner](../shared-tooling.md#shared-tooling-0112-adoption).

Exact-source [upstream 0.1.12 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37511845192)
passed Linux portable regression, lint/security and both native macOS jobs.
Consumer source/export/modes, snapshot integrity, shell syntax, local/exported
documentation references and diff checks are preparation evidence. No contributor
tests, builds, lint, installation or release effects ran during preparation.
The maintainer subsequently released the batch. Exact-source
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526800896)
passed Linux, MSRV and both complete native macOS gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526801007) passed.
The consumer snapshot corruption, shared command checker and release runner
fixtures pass on all three hosts. #17/#18 were closed with this evidence;
#11–#16 remain closed. No contributor validation ran during this review.

Released 0.14.4 at `49e4e8a` passed main Linux/MSRV and both complete native macOS
release gates plus matching tag CI. It qualifies the previous tag-checker, logger
and restricted-PATH fixture repair. #16 was closed with the maintainer's explicit
authorization and exact hosted evidence; #11–#15 remain closed. The earlier local
fixture failure and its repair retain their scoped
[evidence](../shared-tooling.md#consumer-restricted-path-fixture-repair).
The maintainer reports publication live; registry publication was not independently
checked. No upstream files were changed.

## Consumer-owned instruction reader

Released 0.14.0 includes the consumer-owned reader and registry ic-metrics
0.2.0 in both independent lock graphs. Production uses the already-owned ic0
counter-1 binding; test-only native counter handling is unchanged. The retired
`ic` feature is absent. Summary arithmetic is unchanged, but the exposed
`MeasurementSummary` has a new Rust package identity: consumers passing it to
or from a direct ic-metrics dependency must align that dependency to 0.2 or use
IC Timers' re-export. This public type cut requires the next minor line and
supersedes the entire unpublished 0.13.6 draft; it introduces no compatibility
alias. Scheduler/work attribution, sample admission and registration identity
remain unchanged. See the [adoption owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption)
and [ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10).

Earlier extraction qualification used 0.1 arithmetic. With the maintainer's
then-existing explicit extraction permission, strict native
library/tests and Wasm library Clippy passed. The two named role-specific
saturation and memory-projection tests passed. Both independent locked/offline
graphs resolved without changing package selections; testing's metrics lock edge
to ic0 was removed. Source/manifest/lock hashes stayed unchanged through checks.
Native tests use the consumer-owned fake and provide no IC execution evidence.
Logs and identities remain in ic-metrics' `target/evidence/arithmetic-cut-020/`.
Those checks do not qualify the subsequent 0.2 dependency selection. The current
adoption ran only a targeted offline testing-lock update, both cheap locked
metadata checks, dependency-tree inspection and diff/source checks. Root already
selected 0.2.0 and was preserved; every non-metrics testing lock record is
unchanged. No new tests, builds, lint gates, commit, release or publication ran;
that adoption's preparation evidence remains separate from hosted qualification.
The maintainer subsequently released 0.14.0 at `902323a`; matching tag CI and
main Linux/MSRV and both complete macOS gates passed. See
the [source-bound host record](../releasing.md#host-support).

## Release state

- Read `[workspace.package].version` in [Cargo.toml](../../Cargo.toml) for
  package identity. The top [changelog section](../../CHANGELOG.md) records the
  released batch or, when present, its automatically selected, undated next version.
  The maintainer's bump finalizes and dates it; a dated section alone does not prove
  tagging, publication or deployment.
- Cargo, both local-package lock entries, finalized changelog and tag identify
  release commit `0c90c391dff5960a7502fc15a0718b03631f2515` (0.14.6).
  Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584151377)
  passed Linux, MSRV and both complete native macOS gates.
  Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584150869)
  passed. This is the latest inspected complete consumer qualification; it does
  not qualify the later 0.14.7 worktree. Registry publication was not
  independently checked. Earlier results and prior failures retain their exact
  source scope in the [host record](../releasing.md#host-support).
- During the 2026-10-07 evidence review, an external root-lock edit selected
  ic-metrics 0.2.3 while the independent testing lock still selected 0.2.0.
  That edit was preserved during preparation and is now committed in 0.14.6;
  its complete hosted qualification remains separate from the subsequent
  tooling worktree. The independently selected graphs remain distinct; no contributor
  dependency update ran during the layout or evidence review.
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
- Released 0.13.5 repairs those compatible repository-only failures.
  Metadata and Git admission functions explicitly propagate failed commands
  instead of relying on Bash 3.2 subshell errexit. Sort and lock failures are
  injected independently for both workspaces. The index fixture deliberately
  uses a shallow clone with its own controlled baseline tag, satisfying impact
  classification without remote history or production changes. Both fixtures
  report captured adapter output before failure cleanup. Contributor source,
  shell syntax and diff inspection was preparation evidence; subsequent Linux
  and both native macOS hosted gates passed both repaired fixtures. The release-check
  repair leaves runtime sources unchanged.
  The separate dependency update requires `ic-metrics 0.1.6`; the maintainer's
  release preflight stopped at the stale testing lock before version mutation.
  With explicit authorization, a targeted offline Cargo update aligned that lock
  to 0.1.6 and preserved every other record. Both existing locked metadata checks
  pass; each independent graph selects one registry 0.1.6 package. This is metadata
  preparation, separate from subsequent hosted qualification. Tagged 0.13.5's
  root lock selects compatible registry 0.1.7 and its testing lock selects 0.1.6;
  both satisfy the tagged 0.1.6 requirement. Independent graphs need not select
  identical compatible packages. The current manifest requires 0.2; a later
  concurrent root lock change selects 0.2.1, while testing retains 0.2.0.
  The original 0.2.0 adoption lies outside 0.13.5 qualification and its matching
  0.14.0 hosted qualification passed as recorded above. The concurrent 0.2.1
  lock selection has no qualification in this adoption. The requested
  prevention feedback is filed as
  [Shared Tooling #6](https://github.com/dragginzgame/shared-tooling/issues/6).
  No upstream filesystem change or contributor release effect was performed.
  Both macOS gates subsequently passed; the authorized closure of
  [#10](https://github.com/dragginzgame/ic-timers/issues/10#issuecomment-6014163192)
  records the exact source and fixture/PocketIC evidence limits.
- The requested dependency-preparation guidance is adopted through the explicit
  revision-bound rule in [AGENTS.md](../../AGENTS.md) from Shared Tooling
  `a7efade1a68e43f148252a1a73908a46c4cbe9e9`. The full snapshot remains reviewed
  `cb86188` (21 files), with the adoption guide transferred into the separate
  original 17-file audit slice at `a37771f`, subsequently extended to 22 files
  for the structured checker/parser adoption. The audit adoption preserves baseline
  executables and product code; the subsequent metrics adapter/dependency change
  has its separate evidence owner above.
  That batch added only checker/host-parser setup. The current draft adopts the
  isolated Cargo/IC helpers above; newer logger and host-installer changes remain
  unadopted. Consumer qualification of the new slice remains pending.
  Scope belongs in the
  [adoption record](../shared-tooling.md).
  Its repository-only audit-method adoption and additional snapshot verification
  shipped in the same 0.14.0 batch, selected for the public measurement
  type's ic-metrics 0.2 identity cut. The maintainer performed version mutation
  and release execution.
- The 0.13.5 Apple Silicon cohort review records 263,433 baseline Wasm bytes and
  316,941/317,563/318,293 bytes for Once/AfterCompletion/Watchdog. Every previously
  emitted instruction/cycle subject matches the 0.13.3 row. Baseline already
  includes runtime inventory/snapshot and shared probe code; these are cohort
  comparisons, not total library size. No further runtime change or performance
  release is supported. Details belong in the
  [measurement owner](../design/callback-delivery-ownership.md#released-0135-cohort-review).
  Intel records the same instruction fields and baseline-relative byte costs,
  with different absolute byte sizes and dispatch cycles retained separately.
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

Current complete consumer qualification is released 0.14.5 at `c84d4e4`, as
recorded in Release state above; subsequent worktree edits remain unqualified.
The following earlier inspections retain their historical source scope.

At the earlier inspection, release 0.14.2 at `88aedf0` had tag CI and main Linux/MSRV passes,
while both complete macOS gates were queued. Exact source and scope are recorded
in the [adoption owner](../shared-tooling.md#formatter-prerequisite-adoption).
The 0.14.1 missing-`rg` failure remains historical evidence; it does not describe
the successful 0.14.2 Linux rerun or qualify later formatter changes.
The then-latest complete all-host qualification was 0.14.0 at `902323a`: tag CI, main
Linux/MSRV and both native macOS gates passed. Apple Silicon
records 142 native tests, doctests, 14 PocketIC subjects and cohorts; its six
measurement rows match 0.13.5 exactly, including Wasm bytes. Exact source and
comparison scope belong in the [measurement owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption).
Earlier 0.13.5 at `98c4b29` passed tag CI, main Linux/MSRV and both
complete native macOS gates passed. The repaired metadata/index fixtures pass
on Linux and both macOS hosts. Each macOS gate records 142 native tests,
14 PocketIC runtime subjects, doctests and cohorts. The preceding
0.13.2 qualification at `134899f` retains its own 138 native/13 PocketIC scope.
Exact
links and qualification scope belong in the [host matrix](../releasing.md#host-support).
The maintainer authorized closing [#9](https://github.com/dragginzgame/ic-timers/issues/9)
with that integration/CI evidence; it is closed. The authorized closure of #10
now records complete 0.13.5 qualification. Open adoption obligations remain in
[GitHub](https://github.com/dragginzgame/ic-timers/issues); no open PR was listed.
The requested [#11](https://github.com/dragginzgame/ic-timers/issues/11) adoption
is released from committed `a37771f`: six
unchanged shared methods, a product-only hygiene overlay, preserved reports and
a separate verified audit manifest. The obligation map and representative
historical-report walk belong in the
[adoption owner](../shared-tooling.md#audit-method-adoption-review). This is
documentation adoption, not a fresh product audit; no local test/build/lint or
complete Make gate was executed. GitHub closure is not authorized by the repair
request alone. The audit slice is repository-only within released 0.14.0;
the metrics dependency makes the complete batch crate-impacting.
The earlier authorized ic-metrics comments on #1, #3 and #4
record IC Timers 0.13.3 consumer qualification; they do not qualify 0.13.4.
The committed Shared Tooling head and remote main matched adopted `cb86188`;
its upstream Linux regression and lint/security passed, but both macOS jobs
failed at snapshot-distribution fixture source-path admission after passing the
shared logger tests. The diagnosis and remaining qualification boundary belong
in the [adoption record](../shared-tooling.md). The preceding `9437bab` Linux and
both macOS upstream gates passed, qualifying that recovery source alone.
Later sibling changes were initially dirty and excluded; after they were
committed at `cb86188`, a separate clean reviewed export adopted them.
No tests were executed locally during this continuation. Shared Tooling committed
its newer batch at `a7efade`; upstream Linux regression and lint/security passed,
while both macOS jobs were queued at inspection. Only its
reviewed dependency-preparation section is adopted through an explicit reference;
the baseline executables and runtime are unchanged. The later clean `a37771f`
export supplies the audit snapshot only, with linked setup/pinning material
scoped as reference rather than full policy/tool adoption. Its upstream Linux
regression, lint/security and both native macOS jobs passed. Both consumer
snapshots, 154 local links/anchors and diff checks pass as adoption evidence.
Those documentation/Make and dependency changes are now included in released
`902323a`; its matching hosted run remains incomplete at inspection. See the
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
