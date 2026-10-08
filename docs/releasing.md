![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Releasing

This guide is for maintainers preparing and publishing a new IC Timers
release. Library users do not need to follow this process.

The workspace follows semantic versioning. Before releasing, maintainers
classify the change, choose the appropriate version, run the required validation,
let the bump label the current changelog draft, and then publish the
release through the repository's release commands.

## Root workspace and dependencies

The maintainer's dependency-centralization request supersedes the independent
`testing/` workspace. The root `Cargo.toml` owns `ic-timers` and the three
unpublished packages under `testing/crates/`, with one root dependency catalog
and `Cargo.lock`. Member dependencies and package versions inherit from the
root; probe publication stays disabled. There is no `testing/Cargo.toml` or
`testing/Cargo.lock` compatibility path.

`ic-timers` remains the default member. Library check, test, lint, docs, MSRV
and Wasm targets select it explicitly. Formatting covers all four members.
The full release gate retains every probe lint configuration, watchdog/ordinary
recovery subject and policy cohort. Probe Wasm builds use `--profile timer-probe`
and the existing ignored `testing/target` and cohort target directories;
loaders select the resulting `timer-probe` output directory. The root ordinary
release profile is unchanged. Dependency resolution now uses root resolver 3;
fresh measurements and native qualification are required for the combined graph.

The current root catalog selects compatible Testkit 0.20, locked to 0.20.0,
with Host 0.3.3 transitively, and ic-metrics 0.2.7. PocketIC remains the audited
16.0.0 server. Its client's exact thiserror 2.0.18 requirement determines the
shared thiserror selection; the old library-only 2.0.21 graph is not retained.
All four member versions inherit the root release identity, and release
preparation updates their local lock records together. This is repository/test dependency preparation, not a released runtime
or measurement claim. Historical adoption evidence below remains tied to its
recorded versions, workspace shapes and hosts.

## Pre-1.0 compatibility

The implementation policy and the version boundary are separate decisions.
Superseded pre-1.0 APIs are still removed as hard cuts without deprecated
aliases, forwarding shims, dual behavior, or compatibility features. A public
removal, signature change, or incompatible public semantic change must
nevertheless advance the minor compatibility line. For example, a breaking
change after 0.3.8 targets 0.4.0, not 0.3.9. Backwards-compatible fixes may use
a patch release.

Version 0.3.7 removed the public `TimerFuture` alias in a patch release. That
was a SemVer mistake because Cargo requirements compatible with 0.3.6 may
select 0.3.7 automatically. The alias remains removed under the hard-cut
policy; the correction is to use a minor version for future public removals,
not to restore a compatibility shim.

## Repository updates versus crate releases

Run this before selecting a version:

```text
make release-impact
```

The classifier compares the worktree with the most recent reachable canonical
release tag and reports:

- `crate` when the publishable crate's source or manifests changed;
- `repository` when only paths outside the publishable crate source and
  manifests changed, such as documentation, evidence, external tests, CI, or
  release tooling; or
- `none` when no path differs.

The workspace may already have an untagged version, so its matching tag is not
required. An explicit base passed to the classifier must still resolve to a
commit. Missing release history, malformed tags and Git failures remain errors.

Tracked and untracked changes are read as NUL-delimited Git paths after both
queries complete successfully. Git's display quoting, whitespace and non-ASCII
filenames cannot change a path's classification. Duplicate records do not change
the strongest impact, so no joining, decoding or sorting is needed. A temporary
record file preserves NUL bytes and is removed on exit; a failed Git query cannot
produce an apparently unchanged subject even after emitting plausible records.
The existing impact fixture covers untracked and staged crate paths with spaces,
tabs, line breaks, quotes and UTF-8 bytes under `core.quotePath=true`, plus partial
query failures and cleanup. Fixture comparisons capture successful output before
checking its value. Shell and embedded fixture-shell syntax and source flow were
reviewed; diff whitespace checks passed. These fixture scenarios have not been
executed, and this change supplies no native macOS qualification.

This is a conservative mechanical boundary, not an API compatibility oracle.
For `crate`, review the public API and semantic contract and choose patch or
minor according to the rule above. Repository-only work normally stays
untagged: validate it with `make repository-check` plus any focused owner-local
evidence and bundle it into the next code-bearing release. This avoids forcing
exact-pinned shared-registry consumers to coordinate a package identity that
does not change runtime behavior.

An explicit maintainer-owned version-bump or release target overrides that
default. For a `repository` subject, version preparation prints an advisory;
the user-operated release targets retain the complete release gate. It never invents crate
impact or silently weakens validation. A `none` subject is still rejected.

Keep completed user-visible changes in one numbered, undated section at the top:

```text
## [0.13.2]
```

Automated contributors derive the candidate from the latest finalized release
and the complete batch under the [shared changelog rules](../rules/changelogs.md).
Reuse that section for later compatible work; before 1.0, a breaking batch needs
the next minor line. Honour a valid maintainer-selected target. This selects
notes only, without changing Cargo, either lockfile or release defaults. Do not
add `Unreleased`, an unnumbered Draft or a separate release-note queue.

The explicit user-operated bump owns the final version: it labels and dates the
single undated draft and moves it above history. No separately prepared versioned
note or handoff status marker is required. Empty or absent drafts are presentation
gaps and do not reject a changed release subject; a missing changelog is created.
The maintainer should not need to fix a heading manually before deployment.

During preparation the helper refuses competing undated release candidates,
a named draft that conflicts with the requested bump, or an already dated target.
Both finalizer calls receive the bump's validated previous package version;
undated numbered sections at or below it are retained as history. Version
components compare by length and text without floating-point conversion.
Same-date finalization is still refused. See the
[finalizer review](shared-tooling.md#changelog-finalizer-review-and-history-fix)
for the ownership and fixture scope.
The 0.14.12 draft delegates selection and pending-body formatting to the shared
AWK owner, preserving historical bytes. The local wrapper owns reader/selector
status, check-only admission, modes and atomic output; its isolated fixtures
include the canonical selector explicitly.
It cannot silently select among batches, override a chosen minor boundary or
relabel published history. It also rejects a requested version that is not a strict
canonical-SemVer increase, an existing exact release tag, or a subject with no
changes since the release-impact base. Version preparation accepts the
current worktree and does not require a preparatory commit. A
repository-only subject emits an advisory but may proceed when the maintainer
has explicitly invoked the bump or release target.

One helper, `scripts/release/workspace-version.sh`, owns reading and changing
`[workspace.package].version`. The repository manifest uses one literal,
double-quoted canonical SemVer field in that table. Table order, indentation
and trailing comments do not select a dependency version. Missing, duplicate
or noncanonical workspace versions fail before mutation or staging. The writer
requires the expected previous version and changes only that value. Both locked
workspace metadata checks also require exactly one resolved `ic-timers` package
at the workspace version; coherent lockfiles for a different package version
are rejected.

`scripts/release/readme-version.sh` projects that same workspace version into
the README API line and exact shared-registry dependency example. Bump preflight
checks those two fields before mutation; the bump updates them after the manifest
and restores the README along with other metadata on failure or interruption.
Metadata checks reject drift. Only the two structured version fields are
checked; historical links and free-form release prose are not version selectors.

The clean-worktree and release-commit guards capture untracked-file queries
before testing their output. The commit helper treats Git's staged-diff status
of 1 as pending changes, while other failures stop before commit or tagging.
Interrupted tag retries also require a successful read of the exact release
subject; matching output from a failed query is insufficient. The Git-phase
fixture injects failures at these boundaries and checks that the commit identity
is preserved. Source review, shell syntax and embedded fixture-shell syntax
checks passed; those new fixture scenarios have not been executed.

Version preparation and release commits capture the exact version's tag listing
before testing whether it is empty. A failed query cannot authorize a bump,
commit or tag, even when it emits plausible output. The release helper queries
again after committing and checking the clean worktree; failure there leaves
the prepared release commit untagged for the existing retry path. Fixtures cover
empty and matching output from failed lookups, metadata and index preservation
before preparation, and recovery using the same commit after the post-commit
lookup fails. Source and shell syntax were reviewed; these scenarios have not
been executed. The lookup uses Git and Bash without adding a host dependency
or establishing native macOS qualification.

The repository checks retain producer failure status during Git configuration
inventory and provider-source inspection. The shared structured Actions checker
captures the complete NUL inventory before reading configurations and removes
its scratch directory on exit; nested paths and spaces retain their meaning.
Provider validation rejects a failed search or sort before comparing the allowed
source path. The repository fixture injects inventory, search and ordering
failures with empty or matching records, checks inventory cleanup, and retains
accepted pinned workflows and rejected moving references. The updated shared
checker fixtures cover quoted/folded references, Docker digests, composites,
reusable workflows, malformed inputs and independent tracked lockfiles. The new
adoption fixtures have not been executed locally; this repository-only work
changes no timer runtime contract and supplies no new native macOS qualification.

Combined release targets run `bump-version.sh --check` before deployment
validation. This preflight checks the requested version, impact, unambiguous
draft selection and structured README projections without changing version
metadata or running tests. An empty exact `VERSION` is rejected before the gate.
The bump helper accepts exactly one `patch`, `minor`, `major` or canonical
`x.y.z` argument after an optional leading `--check`. Missing or extra arguments,
including a misplaced or repeated check flag, fail with usage status 2 before
reading release metadata. The preparation fixture checks these rejections against
unchanged metadata bytes and permission bits.

The version-preparation fixture copies the shared `next-release-version.sh` used
by the bump helper into its isolated repository. It checks all three increment
preflights without metadata mutation, then checks standard Makefile delegation,
selected remote/branch forwarding, resume selection and runner failure propagation
using a recording stub. The local exact-version recipe retains real preflight,
rollback and explicit staging coverage. Shared phase ordering and interruption
recovery belong to `test-release-runner.sh`, now included in `release-check`.
The maintainer reported the missing-helper failure before these changes. Shell
and embedded fixture-shell syntax and source flow were reviewed; the corrected
fixtures remain unexecuted by the automated contributor and need maintainer
qualification on the declared hosts.

The maintainer's subsequent release gate reached the missing-changelog rollback
case and reported `version preparation accepted a failed update with no changelog`.
The fixture had already restored the real lock updater, disabling its requested
failure. It now retains that injector until the case finishes, checks that the
root-update failure and rollback were reached, verifies removal of the newly
created changelog and compares the remaining metadata bytes and modes. The
production bump helper is unchanged. Source, shell and embedded fixture-shell
syntax were reviewed; rerunning the corrected fixture remains maintainer-owned.

The release-commit owner then runs its read-only `--check-before-bump` mode. It
accepts staged implementation changes and dirty metadata selected by
`release-stage`: `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`
and `README.md`. Other unstaged or untracked paths are listed with Bash escaping
and rejected before dependency fetching, validation or version mutation. Stage
the intended implementation changes yourself; the helper does not expand the
metadata staging scope or require a preparatory source commit. Plain `patch`,
`minor`, `major` and `bump-x` remain available for dirty-worktree version
preparation without this combined-release admission check.

The same owner requires every path to be staged in its normal commit/tag mode.
Both modes complete NUL-delimited Git discovery before reading records; an empty
or plausible partial result from a failed query cannot establish admission.
The normal commit mode retains README/lockfile validation, exact tag identity,
clean-worktree checks and interrupted-tag retries. The real bump repeats the
version preflight checks afterward and always advances the requested version.

The worktree fixture covers clean admission without a release commit, each dirty
metadata output, staged/partially staged/deleted source, untracked whitespace/quoted/UTF-8
paths, invalid arguments, and empty/metadata-only failed Git output. Recipe
fixtures cover every combined target rejecting the worktree before the gate and
bump. Shell and embedded fixture syntax, source flow, read-only 0.11.10 changelog
preparation and diff whitespace checks passed; these new fixture scenarios have
not been executed. This is repository-only tooling and supplies no new runtime
or native host qualification.

If a combined release already bumped and staged metadata but stopped at the
commit guard, review and stage the remaining intended paths, then run:

```text
make release-commit && make release-push
```

These local phase commands resume the prepared version. For the standard shared
workflow, rerun its original target to reconcile saved intent automatically.
A prepared date or version does not prove a release tag or publication;
an intentionally unpushed preparation may be followed by a new maintainer-selected
batch while preserving the existing metadata and index.

Cargo owns package identity. The handoff reads it directly instead of storing a
second version projection. Release and deployment checks do not read changelog
layout, release-note headings or `Status:` prose. A dated changelog records version
preparation; it does not establish tagging, publication or deployment. Keep
supporting evidence with its owner and preserve historical records, without
turning them into release prerequisites or a parallel issue tracker.

Before version mutation, the helper also scans
the compact status for target-version wording likely to become stale, such as
`candidate`, `unreleased`, or a next action to publish after release. This is
advisory: it prints a warning and always continues. Free-form prose is never a
post-mutation release blocker. Canonical Cargo identity, structured README pins,
resolved lockfiles and exact annotated tags remain enforced facts.

Use one of the standard release families:

```text
make release-patch
make release-minor
make release-major
```

For an exact version, use:

```text
make release-x VERSION=0.3.0
```

All release execution is user-owned: version bumps, tests, staging, commits,
tags, pushes and publication. Automated contributors prepare only the next
changelog draft and directly relevant evidence. The user runs `make patch`, `make
minor`, `make major`, or `make bump-x VERSION=...` when ready to update the
workspace version and the root lockfile, and `make release-stage` to stage release
metadata.

The selected shared contract includes the PR helper, but this consumer's standard
Make recipes bind `RELEASE_DELIVERY=direct`. Ambient environment or Make-variable
PR selections do not switch the patch/minor/major or resume commands. Adopting PR
delivery requires separate merged-source adapters and qualification; this refresh
grants no merge authority or contributor release effects.

Standard commands may be rerun after any interrupted attempt. Before preparation,
the shared runner starts fresh preflight and complete validation on current
source; older preparation-free records are preserved as evidence. After
preparation may have begun, a normal target selects the saved release before
computing another increment and reconciles its exact source, version, date and
destination. Plans are internal recovery records and do not require a different
command. Uncommitted preparation remains bound to its source and kind. Once the
release commit exists, newer descendant commits or a different requested
increment are allowed: finish the saved release, then run fresh preflight and
the full gate for the requested increment from the actual local version.
Identity, payload, destination and concurrency conflicts still reject.
An explicit exact recovery selector remains available:

```text
make release-resume VERSION=0.13.0
```

The vendored shared runner fixture owns restart and automatic recovery coverage;
the local Make fixtures verify direct delegation, selection forwarding and
selected-commit metadata checks. Explicit resume finishes only its saved release.
The superseded local retry wrapper and its duplicate fixtures are removed.

Released 0.14.14 adopts Shared Tooling 0.1.23's
[release-integrity repair](https://github.com/dragginzgame/shared-tooling/issues/58).
After the final consumer check, the runner independently rechecks the committed
payload, index, worktree and exact annotated-tag object before dispatch. Completed
direct resume observes the exact local/remote tag and destination branch ancestry
before reporting success, without replaying commits, tags or pushes. Conflicts or
unavailable observations retain the selected plan and failure evidence. The
canonical runner fixture owns these refusal and exact-version repair/retry cases;
the local committed-release fixture checks explicit direct-policy selection as
well as immutable metadata/receipt binding. Both remain user-operated gates.
The [adoption owner](shared-tooling.md#shared-tooling-0123-preparation) records
preparation and upstream/consumer qualification limits;
[#29](https://github.com/dragginzgame/ic-timers/issues/29) closed after the
unchanged adoption passed both complete native gates in 0.14.15.

The user-operated release targets run the complete release gate, update the
workspace version and all four local member identities in the sole root lockfile,
commit, create an annotated `vX.Y.Z` tag, and push the branch/tag atomically. If the workspace version
has no saved unfinished intent, the requested bump runs. A fresh `make release-patch`
advances the patch version; rerunning an unfinished patch release recovers it. For
example, with Cargo at 0.8.2 and a current draft, stage and commit the code-bearing
changes, then run `make release-patch` to validate, bump and release 0.8.3. The bump
itself also supports a dirty worktree without a preparatory commit. An exact
`release-x` target must be a strict version increase. Standard releases, exact
releases, validation logging and pre-commit formatting require executing Make
modes that propagate failures. The shared admission probe rejects ignore-errors,
dry-run, question, touch and version-only controls before protected effects.
Use ordinary Make modes for those commands. Outer Make with ignore-errors can
still mask a rejected recipe's exit status; it cannot admit later release phases.
Release selections and normal parallel-job controls remain inherited.
Hook and version-preparation regressions print their retained scratch paths on
failure, including per-scenario Make output and before/after fixture state.
Successful fixture runs remove their scratch directories. Exact-release failure
cases use recording phase substitutes; they do not commit, tag or push.
Release metadata and the root lockfile are checked before the release commit;
unstaged and untracked work is rejected before committing or tagging.

If a standard release stops after the version bump, rerun the same standard
target. The local exact-version `release-x` has no shared journal; finish its
prepared version with the phase targets:

```text
make release-stage
make release-commit
make release-push
make publish
```

`release-commit` commits staged release metadata when the version is untagged.
If the release commit already exists, it requires a clean `HEAD` whose subject
is exactly `Release X.Y.Z`, verifies metadata and the root lockfile, then creates
the missing annotated tag. A retry with an existing tag verifies that tag's
type and commit. Arbitrary clean commits, conflicting tags and new staged
changes for a tagged version are rejected. A push-only failure can be retried
with `make release-push`. These phase targets do not repeat deployment tests;
the completed pre-bump gate remains the evidence for the prepared code.

`make fmt` and `make fmt-check` sort manifests with cargo-sort 2.1.4 before
formatting/checking Rust for every member of the single root workspace. The exact
tool pin lives in `tool-versions.env`; `make update-dev` installs it with
`--version` and `--locked`, and hosted jobs prepare it before gates. Hooks and
format checks never install tools. Prepared standard-release metadata is checked
for manifest ordering before staging, so a commit hook does not repair the
runner's saved payload. `testing-check` uses the same formatting gate.

The vendored formatting hook exports the index to disposable scratch, formats
all root workspace members, then copies/stages only the fully staged selection. Partial
staging, formatter failure or concurrent edits reject without discarding working
changes. Unselected and unrelated edits remain untouched. `make install-hooks`
is explicit per-clone activation; the installer refuses conflicting hook paths.
The local consumer fixture covers library and probe members in one workspace, partial staging and formatter
failure isolation using the actual formatting targets. Its new scenarios remain
unexecuted by the automated contributor.

The non-release `make patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review without running build, lint or test suites.

### Host support

Released 0.14.12 is `72e8f5d9769d00fbe165b16cd6eb2d81cf1b0a67`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37639154601)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37639154602)
passed Linux checks, MSRV and the complete Apple Silicon gate. Intel macOS failed
runner acquisition after five attempts, without executing any job steps or
producing an artifact; this supplies no Intel
source qualification or evidence of a fixture defect. #24/#25/#28 retain their
native qualification requirement. The new 0.14.13 driver is outside that source.

Released 0.14.11 is `eab55f8c20f7b144551f86885aefc7dbaa9fd4ea`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37631528851)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37631528673)
passed Linux, MSRV and both complete native macOS gates, including the released
byte-exact consumer finalization fixtures. This does not qualify the 0.14.12
canonical-selector/PocketIC consolidation or incoming Testkit 0.21.1 lock edit.
No contributor validation ran for the new worktree.

Released 0.14.10 is `479c4b8c8b6412babf7c98ea17c948eacdaeadc3`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37617070319)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37617070245)
passed Linux/MSRV and subsequently both complete native macOS jobs. The preceding
0.14.9 macOS jobs were subsequently cancelled, so its previously pending result
never became complete native qualification. Neither release qualifies later
dirty lockfile changes or uncommitted Shared Tooling repairs. Earlier source-bound
records follow; normal green CI does not prove #23's hosted-failure artifacts.

Released 0.14.9 is `7e98cbc969b1e6d6098786a7f36ac3ba66d8165a`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37614583521)
passed. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37614583799)
passed Linux/MSRV; both native macOS jobs remained pending at inspection. The later
Shared Tooling 0.1.18 worktree has no hosted qualification. Earlier source-bound
evidence follows and must not be relabelled as this release's native result.

Released 0.14.8 is `7194dcdb092a33b90886d46b304545d5c09ca0a6`.
Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37601234917)
passed Linux, MSRV and both complete native macOS gates; matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37601234587)
passed. This qualifies the released two-workspace tooling source, not the current
single-workspace/Testkit dependency preparation. Green jobs do not prove the
failure-only artifact-upload acceptance in issue #23.

The maintainer subsequently committed the single-root consolidation at
`007dbe30d5e3bf64f3434d55b34cd1da5c0e08e7`. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37606855402)
passed Linux, MSRV and both complete native macOS gates. That source is untagged
0.14.9 preparation. Later fixture changes and the external lockfile refresh
remain outside this qualification; normal green CI still does not qualify #23.

macOS host workflows are required by the
[adopted engineering baseline](../DRAGGINZGAME.md#host-support).
The requirement is separate from executed qualification; canister execution
continues to target Wasm on the Internet Computer.

| Host | Current workflow configuration and evidence scope |
| --- | --- |
| Linux x86_64 | Hosted Rust/MSRV jobs use Ubuntu runners. The release gate pins the audited PocketIC 16.0.0 Linux x86_64 artifact. Recorded results remain scoped to their original subjects. |
| macOS 15, Intel x86_64 | Declared host target. PR/main job uses `macos-15-intel`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. The historical 0.14.1 missing-`rg` failure is recorded below. |
| macOS 15, Apple Silicon arm64 | Declared host target. PR/main job uses `macos-15`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. Historical failures retain their original scope below. |

Released **0.14.15** is `ae26b854a1a473c5d2d153705c2d13570f378d38`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37752855158)
passed Linux, MSRV and both complete native macOS gates; matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37752855388)
passed. The released graph selects Testkit 0.21.3, Metrics 0.2.11, Host 0.4.6
and PocketIC 16.0.0; all four local members are 0.14.15. This closes #29 for
the unchanged Shared Tooling 0.1.23 adoption, but does not qualify the incoming
Testkit 0.22 graph or #23's skipped failure-only uploader/download path.

Released 0.14.14 is `6fc76e9ffaabad575fe5f044029e6fdd063e4e32`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37750074305)
passed Linux, MSRV and Apple Silicon; Intel was cancelled. Its
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37750074677)
passed. Its root lock selects Testkit 0.21.3, Metrics 0.2.10, Host 0.4.6
and PocketIC 16.0.0; all four local packages are 0.14.14. No hosted result
for 0.14.14 is relabelled as execution evidence for a different dependency graph.

Released 0.14.13 is `0b12c8a6dbe5f359f5499df67313ffa5c02af446`.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37655294418)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37655294064)
passes Linux, MSRV and both complete native macOS gates. The released lock
contains Testkit 0.21.2, Metrics 0.2.9,
all four split Host packages at 0.4.6 and PocketIC 16.0.0. The source-bound
[adoption acceptance](shared-tooling.md#consumer-adoption-qualification)
closes #24/#25/#28 and records subsequent complete current-release host acceptance.
#23's original early/late failing jobs and six verified downloads remain required.

Host 0.4.6 `0fb05f9e18f032425188d68e1d69317a0f0127d5`
[passes all declared native owner gates](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37648086908).
Testkit 0.21.2 `2db7b4f6b616b484408695656e26207628d74c5f` now has passing
[main](https://github.com/dragginzgame/ic-testkit/actions/runs/37651807993) and
[tag](https://github.com/dragginzgame/ic-testkit/actions/runs/37651807716) CI.
Those results qualify upstream; they do not substitute for this consumer's
separate source-specific native and artifact observations.

Preparation for subsequently released 0.14.14 selected Testkit 0.21.3 and compatible
`serde_spanned`/TOML-family patches. Committed Testkit 0.21.3
`a8e83a1940e5f44927c6df95b5d1269a3ac699bc` passes its exact-source
[main native, MSRV and PocketIC-concurrency gates](https://github.com/dragginzgame/ic-testkit/actions/runs/37661836622)
and [tag CI](https://github.com/dragginzgame/ic-testkit/actions/runs/37661836641).
Review against 0.21.2 finds no change to production library implementation or
the `pic` startup APIs used by this harness: the sole library source change
removes duplicated `cfg(test)` cache-path cases. Full root
`cargo metadata --manifest-path Cargo.toml --locked --offline --format-version 1`
passed without rewriting the incoming lock. During contributor preparation all
four local packages remained 0.14.13 and inherited their dependency declarations;
Host stayed outside the timer library graph. The finalized 0.14.14 notes record
the selection, with subsequent consumer execution scoped above. The earlier
0.14.13 run qualifies its own Testkit 0.21.2 graph.

The released 0.14.14 lock selects Metrics 0.2.10. Committed upstream
`90262c3b086ee39016a6f36902a610a78a139301` passes
[its source-bound CI](https://github.com/dragginzgame/ic-metrics/actions/runs/37745384375).
Its library source is byte-equivalent to 0.2.9; the patch adopts upstream
contribution/release tooling and updates documentation. Future dirty sibling
work is excluded. Full locked offline metadata passes with this selection and
does not rewrite the lockfile. Its actual consumer native and PocketIC execution
remains part of the user-operated gate for the new graph, separately from the
released 0.14.13 graph and the Shared Tooling source/export checks.

Released 0.14.15 selects Metrics 0.2.11, committed upstream
`69b110b8fbefdac4773eac7631796f9dcb3f41a0`. Review against 0.2.10 finds no
library source change; the patch adopts the same Shared Tooling 0.1.23 release
repair and records upstream evidence. Its
[owner run](https://github.com/dragginzgame/ic-metrics/actions/runs/37748731541)
passed Linux, MSRV and both native macOS gates. Preparation's full root
locked offline metadata passed without rewriting the incoming lock; subsequent
0.14.15 consumer qualification is recorded above. This changes upstream tooling
and evidence, not metrics library implementation.

Host 0.5.1 `81f9809861159def2fd0987fcb7961cda4afd969` is committed; its
[exact-source CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37750135927)
passed Linux, MSRV and both native macOS gates.

### Testkit 0.22 adoption

The maintainer authorized adopting committed Testkit **0.22.0**
`2951fd19e58799580e60ec0f6f5864d296271a62`. Its exact-source
[tag run](https://github.com/dragginzgame/ic-testkit/actions/runs/37752946475)
passed native checks, portable-host qualification and PocketIC concurrency on
Linux, Intel macOS and Apple Silicon. The same source's
[main run](https://github.com/dragginzgame/ic-testkit/actions/runs/37752947368)
passed checks, portable-host qualification and MSRV on all three hosts; its
Intel PocketIC-concurrency job was cancelled, so the overall main run is
cancelled. The tag run supplies all three concurrency passes and executes actual
native checks, not just tag admission; its MSRV job is skipped. These exact-source
upstream results do not execute this consumer.

Initial incoming root catalog/lock selected Testkit 0.22.0 and all four Host
packages at 0.5.1; contributor preparation preserved those bytes. Full locked
offline metadata passed; every member dependency table inherits the root
catalog, all local members remain 0.14.15, and Host/Testkit remain outside the
timer library graph. Registry `pic` module/startup bytes match the reviewed
committed Testkit source.

During the subsequent [consumer audit](audits/upstream-consumer-review-2026-10-08.md),
an external lock update selected Testkit **0.22.2**
`2da9f92fbe7fc31c37136e99cac46179e08911ad` and all four Host packages
at **0.5.2** `c7014995bf0890c1df9cd9b9a6ec14ea70f98c6f`. It was preserved;
locked offline metadata passes without lock mutation, and Host/Testkit remain
outside the library graph. Testkit's relevant startup/module source is unchanged
from 0.22.0; registry startup bytes match committed 0.22.2. Its
[exact tag run](https://github.com/dragginzgame/ic-testkit/actions/runs/37762453183)
passes native checks, portable hosts and concurrency on all three hosts; tag MSRV
is skipped. Its same-source main run was still executing Intel checks at
inspection, with all three MSRV jobs passed. Host 0.5.2's
[exact CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37762087718)
passes Linux, MSRV and both native macOS gates. These observations do not
qualify the incoming consumer graph; no dependency file was written by the audit.

The maintained [harness](../testing/crates/ic-timers-pocketic/src/harness/mod.rs)
still explicitly spawns the audited PocketIC binary and connects a fresh
instance to its owned server, retaining instance-before-server destruction.
Testkit delegates server spawn, polling, termination and background reaping to
Host's `OwnedChild`; readiness, deadlines and captured diagnostics remain
Testkit's responsibility. Group signalling precedes leader reaping; cleanup
remains synchronous and unbounded, and PocketIC instance Drop remains unbounded.
The existing APIs used here are unchanged. The Host artifact API and observed
Cargo-build process-group hard cuts require Testkit's 0.22 minor, but our harness
consumes neither of those surfaces. There is no timer public/API semantic cut:
the compatible **0.14.16** draft records this test-only adoption
([Testkit #25](https://github.com/dragginzgame/ic-testkit/issues/25)). No direct
Host dependency, parallel child owner or Cargo patch is introduced. Its digest
exclusion fix has no local caller.

No consumer tests, builds, lint, PocketIC runs or release commands ran during
preparation. The full user-operated gate must qualify this graph, including
native probe lint, watchdog/recovery and policy cohorts. No production Wasm,
heap or timer instruction change is expected from this test-only selection;
no new size/cost measurement is claimed. This repository-only batch can be
bundled with later code work; a maintainer-selected patch retains the complete gate.

Inspection on 2026-10-06 of the 0.13.1 source at
`54bbcfc4985d4657578150cbe7112795297115fd` found both native macOS jobs in
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535919)
failed in `test-release-gate.sh` at the default PocketIC path comparison. The
verified artifacts and preceding snapshot/release fixtures passed on each host;
the complete gate did not pass. Make selected the physical `/private/var/...`
workspace, while the fixture expected the logical `/var/...` temporary path.
The fixture now derives its expected cache from `pwd -P` and deliberately enters
through a directory symlink on every host. It retains exact default, environment,
command-line, same-as-default and empty override checks; production provisioning
is unchanged. Source review, shell syntax and diff checks are the preparation
scope. Subsequent 0.13.2 CI qualified the repaired fixture on both native hosts,
as recorded below; the earlier failed attempt remains separate evidence.

For that same source, the Linux `checks` job passed its CI and nested-probe lint
steps. The separate MSRV job and
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535920)
failed to acquire hosted runners, without executing their steps. These service
failures do not qualify those gates and are distinct from the macOS fixture bug.

Inspection on 2026-10-06 of release commit
`134899f1620b29f51711ff479c37c681db3fb9ec` found
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37425186029)
successful: Linux checks and MSRV passed, and both macOS 15 jobs passed the
complete release gate under native Bash 3.2. The Apple Silicon job log records
138 native tests, doctests, all 13 PocketIC recovery/ordinary/churn subjects,
and policy cohorts passing with the exact audited PocketIC 16.0.0 artifact.
The Intel complete gate also passed. The matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37425186206)
passed. This qualifies that committed source and its selected dependencies;
it does not establish publication, downstream composition, other host versions,
or qualification for later source. Historical preparation-only and
failed records below retain their original scope rather than describing current
0.13.2 qualification.

For release commit `864397a7c21eec4f396fe9617dbb8d8e1f9cfc73` (0.13.3), the
[tag job](https://github.com/dragginzgame/ic-timers/actions/runs/37430538306)
passed when inspected on 2026-10-06. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342)
passed Linux checks, MSRV and both native macOS complete release gates. The
Apple Silicon log records 142 native tests, doctests, all 14 PocketIC runtime
subjects and policy cohorts passing with the audited PocketIC 16.0.0 artifact.
The [callback ownership record](design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests)
compares the exact 0.13.2/0.13.3 instruction subjects, including initial-arm and
cancellation increases. This qualifies released 0.13.3 source, not the subsequent
working-tree release-tooling and host-only measurement changes.

For release commit `aa0e933eb56ff0e3e6832ad822f77c95a1392199` (0.13.4),
the maintainer reports publication live and the
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37433514158)
passed. The matching
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37433514493)
passed MSRV but failed Linux checks and both native macOS complete gates.
Linux reached `test-release-index.sh`, reported a shallow source clone and exited
128, with the adapter output hidden in the fixture's deleted temporary file.
Source inspection identifies its next-version preflight's reachable-tag
requirement as an unmet prerequisite in the default shallow main checkout.
Both macOS hosts failed `test-committed-release.sh` with
`committed check accepted manifest ordering failure`; later runtime/PocketIC
and cohort stages were not reached. These results do not qualify the complete
0.13.4 release gate, despite the successful tag job. Registry publication was
not independently checked.

For release commit `98c4b296d7461525c15a01e30adbe33b75bcfa38` (0.13.5),
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37442171230)
passed. The matching
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942)
passed Linux checks, MSRV and both complete native macOS release gates.
The completed host logs confirm the selected-commit and real-index fixtures
passed. Each macOS job records 142 native tests, doctests,
14 PocketIC runtime subjects and policy cohorts with audited PocketIC 16.0.0.
The [cohort review](design/callback-delivery-ownership.md#released-0135-cohort-review)
records exact installed byte sizes and comparison limits. The tagged root lock
selects registry ic-metrics 0.1.7, while the testing/probe lock selects 0.1.6;
both satisfy the tagged compatible 0.1.6 requirement. Results qualify their
recorded graphs only, not later manifest edits, publication or composed consumers.

For release commit `902323a9e896ce3771044fdc23a7a2d03d49cf28` (0.14.0),
the maintainer reports publication live and the matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37476634555)
passed. Inspection on 2026-10-06 of
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37476635415)
found Linux checks, MSRV and both complete native macOS gates successful.
Both independent release locks select registry ic-metrics
0.2.0. The completed Apple Silicon log records 142 native tests, doctests,
14 PocketIC runtime subjects and policy cohorts with the audited ARM64 server
digest `781f643d4b16105e7544ca810a972f99c0ef1919016c680faa93f10909a14496`.
Its six cohort/calibration rows match the same-host 0.13.5 rows exactly, including
Wasm bytes, as recorded in the [measurement owner](design/callback-delivery-ownership.md#ic-metrics-02-adoption).
Registry publication was not independently checked; the subsequent parser/checker
adoption has its separate failed qualification below.

Release `fbd319de7a60d6475232439e398fff2263a9d666` (0.14.1) is reported live.
Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380254)
passed MSRV but failed Linux checks and both macOS complete gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380756)
also failed. All four failures report `scripts/ci/test-dependency-pins.sh:
line 45: rg: command not found`. Parser setup and the preceding host-tool
fixtures passed; the shared rejection fixture then could not inspect its output.
This is missing consumer CI provisioning, not proof of a checker admission bug
or timer runtime failure. It does not qualify the complete 0.14.1 release gate.
The compatible repair adds explicit ripgrep bootstrap through apt on Linux and
Homebrew on both macOS hosts, before fixtures, with a command-availability check.
The shared snapshot, ordinary offline validation and the complete gate stay
unchanged. Shell/source/diff inspection is repair evidence; rerunning native
qualification remains maintainer-owned. Details belong in the
[adoption owner](shared-tooling.md#structured-checker-ci-repair).

Pushed 0.14.2 at `88aedf0a5353d176037062ae262dd67bae11beae` passed
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37489635619)
and main [Linux/MSRV jobs](https://github.com/dragginzgame/ic-timers/actions/runs/37489635451).
Both complete macOS jobs were queued at inspection; latest inspected complete
all-host qualification remains 0.14.0. These results qualify the earlier
helper/CI repair on the inspected hosts, not the subsequent formatter worktree.

For release commit `e001ab98195d3c8541430a934fd76f756c0717d2` (0.14.3),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37493326312)
passed Linux checks, MSRV and both complete native macOS release gates.
The matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37493325538)
passed its exact-tag/main-ancestry checks. At that inspection it was the latest all-host
qualification, including formatter prerequisites, changelog preparation and the
earlier Cargo/IC helper wiring. Registry publication was not independently
checked. These results do not qualify the later 0.14.4 tag-checker and tooling
worktree. The [0.1.11 refresh owner](shared-tooling.md#shared-tooling-0111-refresh)
records exact upstream all-host evidence and pending consumer qualification.

For release commit `49e4e8a25025c6a6329c993ea85255341474682b` (0.14.4),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37505847432)
passed Linux checks, MSRV and both complete native macOS release gates. Matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37505847427) passed
exact tag/version and main ancestry admission. These results qualify tag-checker
delegation, the 0.1.11 logger refresh and the repaired restricted-PATH fixture.
The maintainer-authorized closure of #16 records that evidence. These results
qualify that release, independently of later source. Registry publication was not
independently checked.

For release commit `c84d4e4d26f968a9d7f2d37f30f7fed447692c0f` (0.14.5),
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526800896)
passed Linux checks, MSRV and both complete native macOS release gates. Matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526801007)
passed exact tag/version and main ancestry admission. All three hosts' logs pass
the consumer snapshot export/corruption fixture, the shared standard-release
command checker and release runner command substitutes. Both macOS gates record
142 native tests, 14 PocketIC runtime subjects, doctests and policy cohorts.
This completed consumer qualification closed #17/#18 for the actual adopted
0.1.12 wiring. The
[adoption owner](shared-tooling.md#shared-tooling-0112-adoption) records its scope
and the separate unqualified Make-mode boundary reported upstream. Normal hosted
success does not qualify ignore-errors or non-executing Make modes. Registry
publication was not independently checked. No new local validation ran during
the evidence review.

Released 0.14.6 adopts Shared Tooling 0.1.13 and moves
the three host/probe packages under the independent `testing/crates/` root.
Source-bound upstream Linux/macOS success and cheap consumer locked metadata,
path/hash and integrity inspection are recorded in the
[workspace adoption owner](shared-tooling.md#shared-tooling-0113-workspace-adoption).
At `0c90c391dff5960a7502fc15a0718b03631f2515`,
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584151377)
passed Linux, MSRV and both complete native macOS gates.
Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584150869)
passed. #19 was closed with this source-bound complete qualification; this is
the latest inspected complete consumer release. Make/CI workspace roots, package
selectors, `testing/target` artifacts and the exact PocketIC gate remain.
The preserved root-lock ic-metrics update is qualified with this release.

Released 0.14.7 adopts Shared Tooling 0.1.14 and the shared Make execution guard.
Upstream Linux and both macOS jobs passed at the exact exported revision;
consumer preparation checks cover snapshot integrity, source/mode comparison,
shell syntax, documentation and diff inspection. No contributor tests, builds,
lint or formatter ran. At released `a30bfe01d9ce82ba691f5ddd1980f9a4b7c0454a`,
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893527) passed
Linux/MSRV and both complete native macOS gates.
Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893540)
passed. The logs record Make admission, gate, hook and collector fixtures passing
on Linux and both macOS hosts; #20 closes with this complete source-bound
qualification. New .8 edits retain their own pending qualification. See the
[Make admission owner](shared-tooling.md#shared-tooling-0114-make-admission).

Version preparation uses Bash, Perl, Git, Cargo and the explicitly installed
jq/yq parser pair. It owns regular metadata
files; symlinked or non-file outputs are rejected before mutation. Applicable
Make targets use GNU Make. macOS 15 supplies Bash 3.2, GNU Make 3.81 and the
standard BSD/Unix tools used here. Required host tools are Git, Perl with core
`JSON::PP`, `File::Compare` and `Digest::SHA`, `curl`, `gzip`, and Rustup/Cargo.
The SHA-256 boundary uses `Digest::SHA`, without requiring GNU `sha256sum` or a
Homebrew tool installation. Native host prerequisites and setup must be qualified
at their owning workflow boundary.

For local macOS setup, install Rustup, then run `make update-dev` to install the
development toolchain declared in `rust-toolchain.toml`, its components, Wasm
target and pinned host/IC bundles. The full release gate also needs the MSRV toolchain:

```text
rustup toolchain install 1.88.0 --profile minimal --component clippy --component rustfmt --target wasm32-unknown-unknown
make release-verify
```

These are maintainer-operated setup and validation commands, not publication.
The macOS CI jobs install both toolchains and the Wasm target, verify the OS and
architecture against this matrix, and prepend `/bin` to `PATH` so nested
`env bash` wrappers exercise Apple's Bash 3.2. The two jobs run only for PR/main;
their complete gate includes dependency preparation, native CI, MSRV, nested
probe linting, the maintained PocketIC subjects and policy cohorts. Existing
Linux jobs and the smaller tag job remain separate. The jobs use explicit
[GitHub runner labels](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories).
CI configuration supplies a qualification path; only passing native execution
for the matching revision supplies evidence. Adding these jobs does not claim
that they have passed or qualify other macOS versions.

The version-preparation and impact fixtures use positional arguments when a
command may take no arguments, avoiding empty-array expansion under Bash 3.2's
`set -u`. Their changed scenarios have not been executed.

The repository and release-gate fixtures compare ordered newline records
directly with `cmp`. They no longer depend on Bash 4's `mapfile` or turn those
records into joined arrays. Expected gate order remains independent of Makefile
variables, including repeated PocketIC prerequisites and the exact prefix before
an injected leaf failure. Provisioning checks compare both override path and
automatic-install selection, including an empty override. Source and shell syntax
were reviewed; the changed fixtures have not been executed. This removes one
known Bash 3.2 obstacle without establishing native macOS qualification.

The preparation fixture uses the same direct record comparison for phase order
and the four staged metadata paths. It captures Git output with an ordinary
command before comparing, retaining failure propagation without an intermediate
array or process substitution. Empty-index and no-tag assertions also capture
Git output before testing it, so producer failures cannot satisfy those assertions.

Preservation fixtures retain actual file copies rather than checksum manifests.
The formatting-hook fixture compares working bytes and independently compares
the binary staged diff. The preparation fixture copies its metadata and unrelated
files with their modes, then compares bytes with `File::Compare` and permission
bits with Perl `stat`. Preflight rejection, symlink rejection, every injected
rollback failure, interruption, missing-changelog restoration and tag rejection
retain their existing subjects. Mode comparison also applies to the preflight
and tag-rejection checkpoints. Fixture mutations use Perl instead of GNU `sed -i`.
These changes remove fixture-only `sha256sum`, GNU `stat -c` and GNU in-place sed
requirements; they do not remove cryptographic verification of external binaries.
Shell and embedded Perl syntax and source flow were reviewed. The changed
fixtures have not been executed, and native macOS qualification remains open.
On 2026-10-05, read-only changelog finalization for the selected 0.11.8 target,
the current README version projection, both locked offline Cargo metadata checks
and diff whitespace checks passed. Cargo versions and both lockfiles were unchanged.

For 0.11.8, the PocketIC verification fixture compared exact ordered event records,
including both cache and download checks during rejection. Debris searches run
as ordinary commands before empty-result assertions. This tightens the fixture's
producer-failure handling without changing the audited version, digest, binary
verification or override ownership. Source and shell syntax were reviewed;
the changed fixture has not been executed.

### CI failure evidence

All four validation/tag jobs prepare a dedicated `$RUNNER_TEMP/ic-timers-fixtures` directory
after checkout and set subsequent steps' `TMPDIR` to it. Each job ends with
failure-only archive and upload steps, after installation and its final check;
the macOS matrix retains separate Intel and Apple Silicon artifacts. Earlier
failure status is never turned into success by collection.

[The collector](../scripts/ci/collect-failure-evidence.sh) selects that scratch
tree, `.git/release-state/validation-failures`, `target/validation-failures` and
failed `.tools/host-set.*` / `.tools/ic-set.*` installer candidates. It excludes
fixture `.git` entries and does not select the checkout's installed tool trees
or general build caches. Tar records symlinks without following them and
preserves modes, including the metadata modes used by release regressions.
`identity.txt` records actual checkout and event SHAs, job, host, run and attempt.
If no diagnostic payload exists yet, the archive still records that identity.

The official upload action is pinned to 7.0.1 at
[`043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`](https://github.com/actions/upload-artifact/tree/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a).
Its [documented permission behavior](https://github.com/actions/upload-artifact/blob/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a/README.md#permission-loss)
requires tar for mode preservation. The uploader selects only the tar archive
and collection log, with a unique source/job/OS/architecture/attempt name and
the repository's normal retention period. Download the artifact from the failed
run and extract its `ic-timers-failure-evidence.tar.gz` with `tar -xzpf` in a
scratch directory. A collection failure prints and uploads its diagnostic log;
its archive may be absent or incomplete. Hosted job logs remain available too.

The maintained `release-check` gate selects
[collector fixtures](../scripts/ci/test-failure-evidence.sh) for contents, identity,
mode preservation, symlink handling, exclusions, empty payloads, rejected inputs
and archive failure propagation. These released fixtures passed on Linux and
both native macOS hosts at 0.14.7 `a30bfe01d9ce82ba691f5ddd1980f9a4b7c0454a`;
see the [host record](#host-support).

The released .8 follow-up preserves failed committed-release, staging/index,
lockfile and repository-check scratch under TMPDIR, prints its location and
retains the original failure status. Successful runs still clean up. Its collector
fixture drives each actual producer through early injected tool failure, checks
status/input preservation, then compares those inputs after collection/extraction.
Those paths select substitutes before builds or real Git writes. The complete
released 0.14.15 gate passes these maintained cases on Linux and both native
macOS hosts; contributor preparation ran no tests. The original .7 evidence
does not qualify subsequent fixture additions.
Closing
[#23](https://github.com/dragginzgame/ic-timers/issues/23) requires native Linux
and both macOS fixture qualification plus downloadable evidence from
maintainer-controlled early-installer and late-check failures on the declared
hosts, preserving the failing job result. Normal green CI does not prove upload
execution. Contributor preparation dispatches no failing workflow or release.
Force-terminated runners cannot guarantee collection.

Released repository-only 0.14.13 adds an explicit manual qualification
path to the same CI workflow. The workflow is now on main; choose
**CI → Run workflow → failure_stage: early**, then repeat with **late** at the
same frozen source ref, such as released `v0.14.15`.
[GitHub's manual input contract](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#onworkflow_dispatchinputs)
requires the workflow on the default branch. The equivalent maintainer commands
are:

```sh
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.14.15 -f failure_stage=early
gh workflow run ci.yml --repo dragginzgame/ic-timers --ref v0.14.15 -f failure_stage=late
```

Each dispatch selects Linux checks and both native macOS jobs. Early qualification
runs before Rust/tool setup; it calls the real host installer in a fresh scratch
consumer with a curl substitute that retains partial download bytes and exits
22. Late qualification runs only after Linux's final nested lint or macOS's
complete release gate succeeds; it calls the real logger against a failing
scratch Make recipe, preserving Make status 2 and raw/combined failure logs.
This proves workflow retention ordering with controlled substitutes, not a real
network outage or a canister failure. The original jobs stay failed; the driver never
uses `continue-on-error`. Existing final failure-only collector/upload steps own
the artifacts. Manual dispatches have separate concurrency groups by stage and
do not cancel normal source qualification. PR/main gates retain their normal
scope; manual evidence runs skip the duplicate MSRV/tag jobs.

The [driver](../scripts/ci/qualify-failure-evidence.sh) retains
`ic-timers-fixtures/hosted-<stage>.<suffix>/` with `before.txt`, `after.txt`,
`scenario.log` and `status.txt`. Review each downloaded archive's identity against
its actual source, job, host, run and attempt; compare before/after bytes and 0640
modes, and require the selected stage and expected status in `status.txt`. Early
artifacts must include the real retained `consumer/.tools/host-set.*/bin/jq`
candidate containing the controlled rejected bytes. Late artifacts must include
the controlled error in `target/validation-failures/latest-combined.log` and its
raw per-target log. A generic setup/runner failure or missing controlled marker
does not qualify the scenario. Record both run links and all six host/stage
artifact observations on #23 before closing it.

The manual-only `failure-evidence` matrix downloads those three artifacts through
the exact-pinned download action adopted by Shared Tooling 0.1.20. It checks
the [archive verifier](../scripts/ci/verify_failure_evidence.py)'s rejection
fixtures, then verifies the downloaded tar without extracting it. Identity must
match the selected source, original job/host, run and attempt. Known input bytes,
0640 modes, controlled status, partial installer bytes or raw/combined logger
output must all be present. Duplicate/unsafe archive paths and substituted links
cannot stand in for required regular evidence files. Python 3 from the Ubuntu
runner is used only in this manual verification job; it adds no developer,
macOS, library or release-gate prerequisite.

Require all three verification jobs to pass while the original three jobs fail
at the selected injection. A missing artifact, source mismatch or unrelated
setup failure cannot qualify the run. Early and late dispatches together supply
six host/stage observations; keep their run links on #23. Rerunning only a
verification job cannot qualify artifacts from another attempt: rerun the
complete selected manual dispatch when retrying this evidence.

The existing collector fixture now exercises both driver stages in an isolated
checkout with fake CI identity, rejects invalid/nonmanual calls, and compares
retained inputs/logs after archive extraction. This is prepared local coverage;
neither those new fixtures nor the hosted dispatches ran during preparation.

Released #24/#25/#28 are closed after 0.14.13 supplies the missing complete
Intel gate for the unchanged 0.14.12 adoption. The
[source-bound acceptance](shared-tooling.md#consumer-adoption-qualification)
records exact source identity, the 82 unchanged files and each native host's
actual dependency graph. Apple Silicon subsequently passed at the exact 0.14.13
source as well. The maintained driver/collector fixture now passes all three
native gates; #23's downloaded early/late failure evidence remains separate and
was unexecuted at that historical review. No contributor rerun or dispatch was
performed during that preparation.

#### Hosted qualification at 0.14.15

On 2026-10-08 the maintainer explicitly authorized both manual qualification
dispatches and inspection of the six artifacts. Both runs select immutable
`v0.14.15`, commit `ae26b854a1a473c5d2d153705c2d13570f378d38`, attempt 1:
[early 37756952434](https://github.com/dragginzgame/ic-timers/actions/runs/37756952434)
and [late 37756960783](https://github.com/dragginzgame/ic-timers/actions/runs/37756960783).
These execute the released Testkit 0.21.3 / Host 0.4.6 graph, not the separately
prepared Testkit 0.22 adoption.

Both runs completed with all three original jobs failed at their intended
injection and all three hosted download-verification jobs successful. The overall
run conclusions are **failure by design**. Each hosted verifier also passed its
negative/rejection cases. Late Linux passed normal CI/probe lint and both native
macOS jobs passed the complete release gate before the controlled failures.

| Stage | Original host | Artifact ID | Hosted / local downloaded verification |
| --- | --- | --- | --- |
| Early | Linux/X64 | 11539929341 | Passed / passed |
| Early | macOS/ARM64 | 11541110407 | Passed / passed |
| Early | macOS/X64 | 11541391837 | Passed / passed |
| Late | Linux/X64 | 11541230299 | Passed / passed |
| Late | macOS/ARM64 | 11540443940 | Passed / passed |
| Late | macOS/X64 | 11542322043 | Passed / passed |

All six downloaded archives match exact source/job/host/run/attempt, controlled
status 22/2, known input bytes and 0640 modes. Early rejected download bytes and
late raw/combined logs survived actual upload/download. Local verification used
the unchanged released `verify_failure_evidence.py`, with each original run's
identity supplied explicitly; tar entries were read without extraction. Archives,
full original/verifier logs, run/job/artifact metadata and hash observations are
retained under ignored `target/evidence/hosted-failure-artifacts/`. Exact tar
SHA-256 values and acceptance are also recorded in the
[closure evidence](https://github.com/dragginzgame/ic-timers/issues/23#issuecomment-6057617574).

An initial local late-Linux verifier invocation preceded download completion
and refused the missing file; verification after completion passed. Original
logs were temporarily unavailable through `gh run view --log` while their run
was active; full logs were retained after completion. Intel's earlier runner
queues and partial observations are superseded by actual terminal acceptance.
No inconclusive attempt was counted as qualification.

[#23](https://github.com/dragginzgame/ic-timers/issues/23) is closed with all six
hosted and local downloaded observations. This qualifies retained diagnostic
identity/bytes/modes through controlled failure ordering, not real network outage
behavior, force-terminated runners or canister recovery. The incoming Testkit
0.22 graph remains outside these frozen-source runs. No local fixture suite,
version bump, staging, commit, tag, push or publication was performed.

Actual late-Linux download also proves the collector includes active installed
tool bundles. Artifact 11541230299 is 373,200,529 bytes; installed host/IC bundle
targets contribute over 99.99% of its 766,116,256 uncompressed regular-file bytes.
The collector's `host-set.*` / `ic-set.*` selection matches successful installer
targets as well as failed candidates. The existing fixture models an ordinary
`.tools/host` directory rather than the installers' relative symlink layout.
The measured finding and smallest consumer-owned exclusion/fixture repair are
recorded on [#30](https://github.com/dragginzgame/ic-timers/issues/30#issuecomment-6057035304).
This archive-scope follow-up is separate from #23's required upload/download
observations and from the not-yet-committed Shared Tooling archive helper.

### Formatter prerequisites

Both `make fmt` and `make fmt-check` depend on `format-tools-check`. The reviewed
shared guard requires successful exact cargo-sort 2.1.4 output using the existing
`tool-versions.env` pin and successful rustfmt availability for the selected
toolchain. Failed probes reject even if stdout looks correct. They force Cargo
offline and disable rustup automatic installation; setup remains explicit through
`make update-dev` or CI. The guard neither formats nor builds. The following
formatter recipes cover every member of the single root workspace with their
existing options. The hook's isolated index must include the guard along with the current
Makefile and versions file. Fixture wiring and pending native qualification
belong in the [adoption owner](shared-tooling.md#formatter-prerequisite-adoption).

### Structured dependency checks and host parsers

`make actions-check` retains the existing gate entry point and delegates to
the reviewed shared structured checker. It parses workflows, composite actions
and every root workspace member manifest. Git inventory failures, malformed
metadata and missing or untracked workspace lockfiles fail closed. The checker
does not resolve dependency versions, install tools or provide runtime evidence.
The live caller enables `--cargo-inheritance`: member package versions and
ordinary, development, build and target-specific dependencies must inherit their
root. The library and probes share that catalog and one lockfile.
The shared stable version reader validates the selected manifest with offline
Cargo before projecting TOML; version mutation remains consumer-owned. Parser
setup is therefore required before `make version` and release preflight too.

Before local validation, run `make update-dev` for complete development setup,
or explicitly prepare the complete pinned host bundle:

```text
make install-host-tools
make host-tools-check
```

The root [common Make include](../make/tools.mk) selects jq 1.8.2, Mike Farah yq
4.47.2, ripgrep 15.2.0 with PCRE2 and cloc 2.10 from the single
[host pin owner](../ci/tool-versions.env). The shared installer authenticates all
selected payloads before executing any, checks versions, and activates the bundle
together. Previous/failed candidates remain under ignored `.tools/`.
`actions-check` depends on this offline bundle check before reading declarations;
absent or changed tools require explicit setup. Existing parser-only bundles must
be explicitly refreshed with `make install-host-tools` once.

Make, `update-dev` and hosted CI use the same common setup/check targets and
checkout-local PATH. No separate system jq/yq/ripgrep/cloc installation is
required. The repository fixture still supplies an explicit local yq path.
PocketIC keeps its separate exact audited admission owner. Follow the
[bootstrap prerequisites](local-setup.md#bootstrap-prerequisites) for Linux and
macOS and prepare this repository's Rust toolchains/cargo-sort separately.
The reviewed installer itself requires no sudo.

`make cloc` reports all four members of the root workspace, including the
unpublished probes. `CLOC_MANIFEST=Cargo.toml` explicitly selects that same graph;
there is no independent testing manifest. The refreshed reporter isolates its
fixture workspaces and excludes configured build output, including aliases. `make cloc-tooling CLOC_PARENT=/path/to/projects` inventories
sibling CI/tooling with snapshot ownership and source hashes, without executing
consumer code. Counts do not establish instruction or Wasm savings.

The [0.1.18 adoption owner](shared-tooling.md#shared-tooling-0118-refresh)
records source inspection and pending Linux/native macOS consumer qualification.
The shared include's optional Rust-set commands are exported with their helper
but are not added to aggregate setup. Established explicit cargo-sort setup is
retained pending the [upstream path repair](https://github.com/dragginzgame/shared-tooling/issues/54)
and [consumer adoption](https://github.com/dragginzgame/ic-timers/issues/28).
No install, test/build/lint or formatter runs during contributor preparation.
Ordinary checks never download tools.

### Pinned IC tool setup

`make install-ic-tools` explicitly prepares Quill 0.5.4, ICP CLI 1.6.0, didc
0.6.2, ic-wasm 0.11.1, PocketIC 16.0.0 and wasm-opt 132 from
[`ci/ic-tools.tsv`](../ci/ic-tools.tsv). `make ic-tools-check` verifies the bundle
offline. `make install-tools` / `make tools-check` operate on both the host
and IC bundles. `update-dev` and CI use explicit setup; ordinary checks never
invoke these installers without `--check`. Make prepends `.tools/host/bin` and
`.tools/ic/bin` to PATH. Bootstrap also needs tar with xz support for IC assets.

The shared installer verifies archives before extraction, preserves Binaryen's
native runtime libraries, and activates a complete checked bundle under ignored
`.tools/`. Failed candidates and previous bundles remain available. Installed
file receipts and versions are checked before reuse; changed pins require
explicit setup. Tool installation neither deploys canisters nor selects
credentials or a network target.

Native macOS CI supplies `.tools/ic/bin/pocket-ic` as an explicit `POCKET_IC_BIN`
override to the existing release gate. The consumer's independently audited raw
hash and exact version remain required. Locally, the same selection is optional:

```text
make install-tools
POCKET_IC_BIN="$PWD/.tools/ic/bin/pocket-ic" make release-verify
```

Without an override, the required automatic single-artifact evidence cache
provisioning is unchanged. Invalid explicit overrides never download or get
replaced. The generic six-tool setup does not replace this product contract or
the [PocketIC artifact pins](#pocketic-artifact-pins). Shared fixtures are wired
into `release-check`; execution and fresh native qualification remain
maintainer-owned. Exact source and scope belong in the
[adoption record](shared-tooling.md#cargo-and-ic-helper-adoption).

### Dependency pin exceptions

[Exact exception records](../ci/dependency-pinning-exceptions.json) retain three
existing qualified selections rather than changing dependencies to make the new
checker pass. `ic-cdk-timers =1.0.0` fixes provider behavior audited in
[SAFETY](../SAFETY.md), including cancellation heap retention and dispatch limits.
`ic0 =1.2.0` preserves the production platform bindings and counter-1 reader
reviewed in the [measurement owner](design/callback-delivery-ownership.md#ic-metrics-02-adoption).
The root catalog retains `ic-cdk =0.20.3` for probe execution/suspension with
the audited PocketIC 16.0.0 server. Testkit now uses a compatible 0.20 requirement;
its former exact 0.17.3 exception is retired. These are product qualification boundaries, not blanket exact-pin policy.

Each exception matches its declaring root, dependency name and literal version.
Changing any selection requires reviewing its reason and rerunning the affected
source/runtime/host qualification; the old record cannot admit a new version.
The root lockfile and complete locked/offline metadata check remain required. Registry compatibility ranges, path inheritance and local workspace
ownership are unchanged. No exception permits a floating action or Docker image.

### PocketIC artifact pins

The verifier selects PocketIC 16.0.0 pins for Linux x86_64, Darwin x86_64 and
Darwin arm64 using independent OS and architecture queries. Unknown hosts or a
failed query reject before cache inspection. Explicit overrides use the same
host-specific binary hash and exact `pocket-ic-server 16.0.0` version check;
they are never automatically replaced. No caller-supplied digest or version can
relax these checks.

The pins below were inspected on 2026-10-05 against the official
[PocketIC 16.0.0 release](https://github.com/dfinity/pocketic/releases/tag/16.0.0)
and its [release asset metadata](https://api.github.com/repos/dfinity/pocketic/releases/tags/16.0.0).
Each downloaded gzip archive matched its published asset SHA-256 before
decompression. Binary digests were computed from those verified archives, with
ELF x86_64 and Mach-O x86_64/arm64 headers inspected. None of these binaries was
executed during this inspection. Exact version checks and maintained PocketIC subjects remain required
on each native host; artifact integrity does not establish recovery evidence or
native macOS qualification.

| Release asset | Archive SHA-256 | Binary SHA-256 |
| --- | --- | --- |
| `pocket-ic-x86_64-linux.gz` | `268ba79ec7fe9a563a575adf4983c69627093cce2711d142e476cdc7ad04249e` | `69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792` |
| `pocket-ic-x86_64-darwin.gz` | `9710b9c4ac4eaa7eb10bddaa2aba80560a59362610f1bcd8c6e23be82a39c327` | `b8233ebee53452db7465b43e7b2ff80f2e1445dc148eb2b4b237493d8d15ec66` |
| `pocket-ic-arm64-darwin.gz` | `41cf77e24effc381e21f5e07e908ed078783646e6de05ed52fd6973221f07e64` | `781f643d4b16105e7544ca810a972f99c0ef1919016c680faa93f10909a14496` |

The 0.14.12 draft reads server/archive identity from the reviewed
[`ci/ic-tools.tsv`](../ci/ic-tools.tsv) matrix through canonical admission.
The table above records provenance; it is not another executable archive catalog.
Extracted-binary digests remain consumer-owned in
[`check-pocketic.sh`](../scripts/ci/check-pocketic.sh). Canonical checksum and
binary helpers authenticate them before execution. The Make gate first checks
the sole root lock's exact PocketIC client/server alignment using prepared
locked offline Cargo metadata and jq, without fetching or updating dependencies.
The default cache path retains the current audited artifact's 16.0.0 label.

Automatic provisioning downloads over HTTPS into an adjacent temporary directory,
checks the archive digest before `gzip`, checks the decompressed binary digest
before execution, then checks its exact version before replacing the cache.
Failures preserve the existing cache and clean the temporary installation.
Candidates must resolve to regular executable files before hashing. A symlink to
a verified executable remains valid input and is retained. If verification fails,
automatic provisioning requires an absent cache path or a regular file without a
symlink at that path; directories, FIFOs and links are rejected before download.
This prevents `mv` from silently installing inside a directory while reporting
the selected cache path as installed. Explicit overrides retain their existing
read-only contract.
The maintained fixture covers each supported host's pins and URL, strict and
missing overrides, partial checksum failure output, download/decompression
failure, version rejection, unsupported hosts, failed host queries and cleanup.
It compares retained cache bytes and permission bits against a distinct cached
copy, so replacing it with the fixture's download cannot pass preservation checks.
These new scenarios and native CI jobs remain unexecuted. Workflow YAML, shell
and embedded fixture shell/Perl syntax, source flow, read-only 0.11.9 changelog
preparation and diff whitespace checks passed. No tests, builds, lint gates,
release commands or version changes were run.

The 0.11.10 cache-type scenarios cover directories, FIFOs, rejected file/directory/
dangling symlinks and accepted verified file symlinks with both installation modes.
They check failure before download, link-target preservation and empty rejected
directories. Source, shell and embedded fixture-shell syntax, read-only 0.11.10
changelog preparation and diff whitespace checks passed; these scenarios remain
unexecuted and do not supply native host qualification.

### Testkit harness qualification

The original 0.17.3 harness adoption used published exact `ic-testkit` 0.17.3,
whose complete upstream PocketIC types remain available through the shared crate.
There is no direct `pocket-ic` dependency in this workspace. Locked dependency
metadata resolves one `ic-testkit` and one PocketIC 16.0.0 package; no declared
dependency MSRV exceeds Rust 1.88. This is metadata evidence, not successful
compilation on that toolchain.

The private harness requires the gate-selected `POCKET_IC_BIN`. It uses testkit
to start a caller-owned server and construct a fresh application-subnet instance,
with a 30-second deadline for each startup phase. Fixtures retain both bindings,
dropping the instance before the server. No baseline pool, implicit download,
test serialization lock or shared IC state is introduced. Upstream synchronous
instance deletion remains unbounded; the startup deadline does not cover it.

Both recovery and cohort fixtures keep their existing canister assertions.
The artifact verifier and its independent fixtures use the new host-specific
archive/binary pins, including explicit rejection of a 15.0.0 server. Old 15
receipts and sampling measurements remain historical and do not qualify 16.
The timer crate, provider and canister source are unchanged by this harness
migration; no Wasm savings or instruction improvement is claimed.

Preparation inspected the published crate archive against the registry checksum,
the official PocketIC release metadata and all three verified archive headers.
Dependency fetching, locked offline metadata, Rust formatting/parsing, shell and
embedded fixture-shell syntax, source flow and diff whitespace checks passed.
No PocketIC server binaries, builds, lint gates, native tests or PocketIC
fixtures were executed.
The 0.13.2 native macOS gates now supply matching compilation, lint, recovery
and cohort qualification for this harness, scoped in the host matrix above.
For changed source, the maintainer runs the complete `release-verify` gate,
including MSRV, probe lint, watchdog/ordinary recovery and policy cohorts. Record fresh
Wasm/instruction subjects and native Linux/macOS qualification rather than
reusing receipts from the previous simulator. Package version and Git release
execution remain maintainer-owned.

### Deployment validation

The 0.12 callback result cut used the minor release boundary for its incompatible
public signatures. `make test` runs workspace targets followed by API doctests;
`make msrv` compiles all targets and runs those doctests on Rust 1.88.0. Thus the
positive and compile-fail policy boundaries participate in PR/main validation
and the existing complete release gate on each declared host. The recording-Cargo
fixture checks order and failure propagation for both targets; it is not compiler
evidence. Automated preparation covered source, formatting/parsing, shell syntax
and read-only metadata review. Maintainer validation owns execution and new
native host/PocketIC/cohort qualification; preparation checks alone do not supply
that evidence. Cargo versions, locks and release execution stay
maintainer-owned; see the [cut's contract](design/0.5-policy-specific-callback-authority.md#ordinary-callback-results).

Deployment validation belongs to the user. The combined `release-*` targets
run the complete release gate before bumping the version. The
gate can also be run directly before committing, tagging and pushing:

```text
make release-verify
```

The gate starts with `make fetch`: `cargo fetch --locked` for both root and
`testing/` manifests, without restricting the target. This explicitly prepares
the selected lockfiles' sources, including target-specific dependencies that
native builds may never download but unfiltered offline metadata needs. Fetching
uses the configured Cargo registry/cache and network policy; a download failure
stops before validation and version mutation. No offline failure is retried
online, no dependency version is selected anew, and no build runs in this phase.
The full validation gate follows successful preparation.

For standalone version preparation or offline work, prepare the cache while
network access is available:

```text
make fetch
```

An error such as `failed to download js-sys ... --offline was specified` means
the selected archive is absent from the cache. A different cached version is
insufficient. Populate the root lockfile's cache and retry the requested operation;
retain its selected versions. `update-dev` installs the toolchain and hook,
and does not prepare these dependency caches.

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported unpublished probe configuration, the maintained watchdog/recovery,
ordinary-await and provider-churn PocketIC subjects, and the four policy cohorts.
If `POCKET_IC_BIN` is unset, the
gate installs the pinned PocketIC 16.0.0 artifact for the current supported host
into the ignored `target/tools` cache. It verifies the archive SHA-256 before
decompression and the audited binary SHA-256 before executing any
downloaded, cached or overridden binary, then checks its version. Diagnostic
paths also leave hash-mismatched binaries unexecuted. An explicitly supplied `POCKET_IC_BIN` remains
a strict override: a missing or mismatched override fails and is never
replaced automatically.

After the version changes, the helper reads Cargo's no-deps member inventory
and updates every local package identity in `Cargo.lock` through the shared
rewriter. External selections are preserved. It then runs offline
`cargo metadata --locked` with full dependency resolution against the root
manifest through `check-lockfiles.sh`. The no-deps inventory is not the validation
gate: it does not establish lockfile coherence. Full locked resolution catches
stale member versions without building packages or repeating evidence suites.
`release-stage` stages the root lockfile automatically.

The checker captures successful Cargo output before parsing JSON. Cargo failures
retain their exit status and diagnostic without a cascading parse error from
empty or partial output. Updated root-graph fixtures retain locked-fetch failures,
failed metadata with empty or plausible output, stale package identities,
rollback/interruption, mode preservation and selected-commit admission. They
have not been executed by the contributor during this consolidation.

The maintainer's reported 0.11.8 preparation attempt
failed on uncached `js-sys 0.3.104`, then reported metadata rollback. Read-only
inspection found that archive and eight other selected testing archives absent
from the local default cache; it does not establish results in another cache or
native macOS qualification.

`release-stage` selects only the four outputs the bump owns. Workspace members
inherit their versions, so their manifests are not version-bump outputs and
remain under the maintainer's separate code-staging ownership. Stage and commit
the intended implementation and supporting evidence before the combined release;
the release commit still rejects unrelated unstaged or untracked work.

Before mutation, the helper captures only its four output files: the workspace
manifest, root lockfile, changelog and README. Failed
commands and handled `INT`/`TERM` interruptions restore their pre-bump contents
and modes, including existing user edits; a previously absent file is removed
on rollback. If restoration fails, the backup is
retained and its path is reported. These shell traps do not cover a forced kill
or machine failure. Unrelated files, handoff/evidence documents and Git staging
are untouched by the bump. Builds and evidence artifacts remain intact on success,
failure and retry; release and publication targets do not append `cargo clean`.
Explicit cleanup is a separate maintainer action.

After the release tag is pushed, publish the crate with:

```text
make publish
```

The publish target requires a clean worktree with an annotated tag in the exact
`refs/tags/vX.Y.Z` namespace at `HEAD`, verifies the package, and then publishes
`ic-timers` to crates.io. A same-named branch or lightweight tag is rejected.

Hosted full CI and MSRV validation run on pull requests and `main`. A tag push
at the same commit does not repeat those Rust builds. Its small tag-only job
instead verifies that the event tag exactly matches the Cargo version, the
tagged commit is reachable from `main`, package metadata is coherent, and the
annotated version tag points to `HEAD`. This preserves protection against an
independently pushed tag from an unmerged commit without running identical
validation twice at one SHA.

The tag job verifies its event version, main reachability and annotated tag
before fetching dependencies. It then prepares the root locked cache and invokes
`check-lockfiles.sh` against the tagged checkout itself, before running the
release fixtures. Fixture success alone does not establish coherence of the
checkout's actual locks. Failed downloads, locked resolution or resolved
`ic-timers` identity stop the job; the root graph must resolve one package at
the Cargo version. This uses the existing fetch and metadata owners and does
not add compilation to their checks. The changed workflow shell was checked
for syntax and its sequence reviewed; diff whitespace checks passed. Hosted
execution and fixture execution remain unverified for this change.

For the maintainer-selected 0.11.9 target, the named undated changelog preflight,
current README projection, both locked offline metadata checks, workflow shell
syntax and diff whitespace checks passed. Cargo versions, both lockfiles and
the maintainer's existing staging were unchanged. Tests, builds, hosted execution
and Git release effects remain maintainer-owned; this repository-only patch
retains the complete release gate despite having no runtime changes.

## Standard release runner

The three standard SemVer entry points use the [common release contract](releases.md)
with explicit `RELEASE_REMOTE=origin` and `RELEASE_BRANCH=main`. The complete local
release gate is unchanged. Consumer adapters select the four metadata outputs,
README projection and the complete root graph. Publishing stays separate.
Normal targets select unfinished preparation intent before another increment.
The 22-file snapshot is exported from committed Shared Tooling revision
`cb86188c5956866564de4fb6ec6be67b27981ab9`; the
[adoption record](shared-tooling.md) distinguishes upstream qualification from
consumer evidence. This refresh addresses
[#10](https://github.com/dragginzgame/ic-timers/issues/10).

The runner owns commit ancestry, saved source/tree/subject, destination, locking,
annotated tagging and exact atomic branch/tag publication. Local committed,
tagged and push callbacks instead inspect `RELEASE_COMMIT`: the current adapter
exports its immutable source tree into an owned temporary directory and invokes
the current workspace-version, README and lockfile check owners there. The root
workspace manifest-sort check and the exact dated changelog heading remain
required. No scripts from the older tree are executed. Failed archive reads,
including partial output, fail closed; the temporary copy is removed without
touching build artifacts, plans or validation logs. This is cold release-path
disk work and has no crate, Wasm, runtime instruction or heap impact.

Preflight reads staged, unstaged and untracked paths separately, with NUL records
and rename detection disabled, before admitting only the four metadata outputs.
Its existing bump check rejects candidate/changelog conflicts before validation
or intent creation. After admission, preflight invokes the existing `make fetch`
owner for the complete locked root graph. It does not require cached archives before
that owner can populate them. Fetching retains Cargo's configured network/cache
policy and the existing lock selection; there is no offline-to-online retry or dependency
update. The complete gate retains its existing fetch-first ordering, and a
failed fetch stops preflight before validation or release mutation.
Locked fetching prepares an already consistent graph; it does not repair stale
lockfiles after a dependency requirement changes. The maintainer's 0.13.5 attempt
from `9d10b49851620296b55878b6aafb4ef92db6d45e` passed the root fetch and stopped
at the testing fetch: the root required/selected `ic-metrics 0.1.6`, while
`testing/Cargo.lock` still selected 0.1.5. An explicitly authorized targeted
offline update corrected only that package record. Both existing locked metadata
checks passed, with all other selections preserved; tests and full native release
qualification remain separate. Future dependency preparation must trace every
affected workspace, including path-dependent probes, and align applicable locks
before cache fetching. The requested common guidance is owned by
[Shared Tooling #6](https://github.com/dragginzgame/shared-tooling/issues/6), without
unlocking release fetches or changing release phase order.
The final commit adapter checks the entire index, requires
all four outputs to be tracked and the worktree to match the index, then applies
the existing metadata checks. Staged implementation changes hidden by restoring
only the working file cannot pass either admission boundary.

The complete `release-verify` target now delegates its unchanged target list,
in the same fail-fast order, to the canonical validation runner with its root
explicitly bound to Make's current repository. Raw failed
attempts are retained uniquely under the Git directory's
`release-state/validation-failures/`; `latest.log` is only a convenience copy.
Later attempts preserve earlier raw logs. Failed retention preserves the owned
temporary logs and reports their path. This adds logging and scratch disk work
to user-operated validation without package publication, cleanup, another gate
or runtime instrumentation. The logger's optional ripgrep branch has a stock
grep fallback; no new host installation is required.

The 0.14.4 logger refresh keeps passing/ignored `error::` test names and retained
context ordinary while highlighting actual diagnostics. Dispatched child loggers
choose their own checkout; inherited release selections, log policy and nesting
depth remain. The existing release-gate fixture covers these source-reviewed
cases through the actual adapter and both prepared/stock search paths. No target
membership, ordering or gate is removed; fresh consumer execution remains pending.

The tag checker takes explicit commit/version arguments only in those late
callbacks. Its no-argument mode still checks the current workspace and HEAD for
standalone tagging and publishing. Missing commit selections, floating refs,
lightweight/wrong tags, invalid metadata or failed checks stop recovery.
The compatible 0.14.4 worktree delegates those checks to the recorded shared
`check-release-tag.sh`; the local adapter retains only identity selection.
Diagnostics come from the shared owner, including the selected SHA for a wrong
target. The [adoption record](shared-tooling.md#annotated-tag-checker-adoption)
owns source, unchanged caller contracts and pending qualification.
When newer fixes exist, completing the older release does not implicitly publish
those fixes: the runner validates them afresh before preparing the requested
next increment. Changed remote history must be established before replay;
publication, deployment and cleanup remain separate.

`test-committed-release.sh` exercises the actual local Make callbacks and metadata
owners with Git/Cargo command stubs: selected metadata despite newer HEAD,
the complete locked root graph, corruption of all four metadata outputs, tag
conflicts, archive failure before/after output, failed resolution/manifest sorting,
missing selection and temporary-copy cleanup. The real-Git tag fixture now
checks an earlier selected commit separately from HEAD. The shared runner fixture
owns the patch/minor/major and same/different-kind interruption matrix, lost
push replies, fresh-gate failure/retry, conflicts and locking. These fixtures
remain in the complete release gate on Linux and both native macOS jobs.
The real-index fixture reuses the current commit without creating commits
and checks hidden staged source, stale staged metadata, valid staged metadata,
untracked outputs and preservation of the selected index. It also injects failed
staged/unstaged/untracked producers with empty or plausible partial NUL output,
and failed index/worktree comparison. The cache scenarios exercise actual
preflight and the real fetch recipe with Cargo stubs, requiring both locked fetches
in order and stopping at either workspace's failure. Changelog, package metadata,
locks, source/index and absence of release intent remain checked independently.
These are producer and orchestration fixtures, not successful downloads or live
release execution. The existing actual
release-gate fixture now checks raw failure retention across failed/successful
attempts and execution with a tool path excluding ripgrep. Full gate sequencing,
failure propagation and PocketIC override assertions are retained.
The 0.13.4 hosted attempt failed at the subjects recorded in the
[host matrix](#host-support). The released 0.13.5 repair explicitly returns each
metadata-check failure, including workspace-version extraction, both manifest
ordering checks, README projection and locked resolution. Git path admission
also returns failed producer status explicitly. This avoids relying on Bash 3.2
errexit inside a function invoked from the selected-tree subshell. The fixture
injects each workspace's sort and lock failure independently, uses explicit
stub failure exits, checks temporary-copy cleanup and verifies sorting failures
stop before metadata resolution or tag lookup.

The index fixture deliberately uses a depth-one local transport clone and gives
only that isolated clone a controlled current-version baseline tag. It does not
fetch remote history, create commits or change production impact classification.
The existing index-preservation, failed-Git-query and ordered locked-fetch
subjects remain. Both fixtures print their captured adapter output on failure
before cleanup, so subsequent hosted failures retain the underlying error.
Source and shell/embedded-shell syntax inspection and diff checks are preparation
evidence. The later Linux and both native macOS 0.13.5 hosted gates pass the repaired
fixtures, scoped in the [host matrix](#host-support). No release
was executed during contributor repair.

The prepared host-only cohort row also appends `wasm_bytes`, measured from the
exact vector installed into PocketIC for each baseline/policy artifact. The
existing cohort gate builds and measures those artifacts as before; there is no
new build or evidence command. Earlier hosted logs have instruction subjects
but no byte counts, so no historical Wasm delta is inferred. The completed
0.13.5 native macOS gates supply executed output, scoped in the
[cohort review](design/callback-delivery-ownership.md#released-0135-cohort-review).
