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

Released 0.14.21's root catalog selects compatible Testkit 0.25, locked to 0.25.3,
with all four Host crates at 0.8.4 transitively, and ic-metrics 0.2.15. The PocketIC client/server pair selects
16.1.0; prior runtime evidence remains bound to its earlier pair. Its client's exact thiserror 2.0.18 requirement determines the
shared thiserror selection; the old library-only 2.0.21 graph is not retained.
All four member versions inherit the root release identity, and release
preparation updates their local lock records together. Publication reported by
the maintainer does not establish complete native or measurement qualification.
Historical adoption evidence below remains tied to its
recorded versions, workspace shapes and hosts.

The [host record](#host-support) and
[Shared Tooling adoption](shared-tooling.md#shared-tooling-committed-0132-follow-up)
record current source and qualification gaps. Released 0.14.21 passes the complete
hosted native gates and matching tag truth, including its selected Metrics graph.
The earlier missing-package offline inspection remains historical. The compatible
0.14.22 draft prepares repository-only compact failure collection, with Cargo
identity unchanged. Later external lock edits selecting Metrics 0.2.16, TOML 1.1.8
and toml_parser 1.1.5, plus all four Host crates at 0.8.5, are retained separately;
that graph needs fresh qualification.
Normal fetch still prepares the selected lock before validation; no contributor
dependency update, lock edit, fetch or local qualification ran.

Release-source admission delegates to the reviewed shared checker. The adapter
permits only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` as release outputs;
it reports all other staged, unstaged and untracked paths, quoting unusual names.
Failed Git observations preserve their own error rather than claiming dirty
source. The runner adds the initial preflight context only before that attempt
starts validation or version preparation. Later prepared/committed checks retain
their own phase semantics. No guard repairs files or stages a lock to make release
proceed. [#32](https://github.com/dragginzgame/ic-timers/issues/32) owns native
consumer acceptance.

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

README versions, examples, layout and presence are not release prerequisites.
Release preparation leaves README bytes and modes unchanged and does not stage
it. The former local version-projection helper and test are removed from every
release phase and CI. README edits follow ordinary source admission and explicit
maintainer staging; stale examples alone cannot refuse a release. Periodic
advisory documentation review is requested in
[Shared Tooling #100](https://github.com/dragginzgame/shared-tooling/issues/100),
using the shared maintenance catalog rather than another local gate.

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
draft selection without changing version
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
`release-stage`: `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`.
Other unstaged or untracked paths are listed with Bash escaping
and rejected before dependency fetching, validation or version mutation. Stage
the intended implementation changes yourself; the helper does not expand the
metadata staging scope or require a preparatory source commit. Plain `patch`,
`minor`, `major` and `bump-x` remain available for dirty-worktree version
preparation without this combined-release admission check.

The same owner requires every path to be staged in its normal commit/tag mode.
Both modes complete NUL-delimited Git discovery before reading records; an empty
or plausible partial result from a failed query cannot establish admission.
The normal commit mode retains lockfile validation, exact tag identity,
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
post-mutation release blocker. Canonical Cargo identity, resolved lockfiles and
exact annotated tags remain enforced facts.

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

The selected shared contract includes the PR helper. Standard entrypoints come
from `make/release.mk`; this consumer uses separate override assignment and
export for `RELEASE_DELIVERY=direct`, preserving GNU Make 3.81 parsing.
Target-specific bindings fix snapshot routing to `$(CURDIR)`.
Ambient environment or Make-variable
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
Use ordinary Make modes for those commands. The reviewed Shared 0.2.11 include
independently probes MAKEFLAGS and retained MFLAGS, and refuses assignments
that erase MFLAGS. Clearing/replacing MAKEFLAGS cannot admit hidden unsafe modes.
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
tool pin lives in `ci/tool-versions.env`; `make update-dev` installs it with
`--version` and `--locked`, and hosted jobs prepare it before gates. Hooks and
format checks never install tools. Prepared standard-release metadata is checked
for manifest ordering before staging, so a commit hook does not repair the
runner's saved payload. `testing-check` uses the same formatting gate.

Formatting success prints one line. Failure prints the failing command's exit
status and the path to complete retained stdout/stderr; Make then returns its
normal recipe-failure status. In CI, the wrapper uses RUNNER_TEMP and the local
failure collector archives `formatting.*` there. Local runs otherwise use TMPDIR
or /tmp. Successful logs remove themselves. The
[0.2.11 adoption owner](shared-tooling.md#shared-tooling-0211-formatting-and-make-admission)
records source identity and pending consumer qualification.

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

The released 0.16.3 workflow adds Linux execution of the existing PocketIC
recovery and policy-cohort targets after CI/probe lint. Testkit's root-lock
selected CLI prepares and admits the server; the product harness then starts
fresh managed servers and instances. Its matching Linux product run now passes;
both native macOS gates remain queued. Keep
[#34](https://github.com/dragginzgame/ic-timers/issues/34) open for those results.
See [the qualification scope](#linux-product-qualification-for-0163).

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
| Linux x86_64 | Hosted Rust/MSRV jobs use Ubuntu runners. Released 0.16.3 passes product recovery and policy cohorts with the prepared internal toolchain; Testkit owns server admission from the root-lock selection. 0.16.4 also passes at its own Testkit 0.28.0 / Metrics 0.3.5 / Host 0.10.1 graph; newer transitive lock changes remain separate. |
| macOS 15, Intel x86_64 | Declared host target. PR/main job uses `macos-15-intel`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. 0.16.3 fails before the gate on Make override/export parsing; the repaired 0.16.4 native gate passes with actual recovery and policy-cohort subjects. Historical failures retain their original scope below. |
| macOS 15, Apple Silicon arm64 | Declared host target. PR/main job uses `macos-15`, Apple's Bash 3.2 and the complete release gate. The gate passed for released 0.14.15. 0.16.3 fails before the gate on Make override/export parsing; the repaired 0.16.4 native gate passes with actual recovery and policy-cohort subjects. Historical failures retain their original scope below. |

Released **0.14.21** is `5c6b7f45d72f9051b4e155410ec47339885dc4f2`;
the maintainer reports it pushed. Matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37819731178)
and [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37819730920)
pass. Linux checks/probe lint, MSRV and both complete native macOS release gates
qualify Shared 0.1.29's actual consumer release-source adapter and its
Testkit 0.25.3 / Host 0.8.4 / Metrics 0.2.15 / PocketIC 16.1.0 graph.
All three host logs report passing release-index, release-runner and installer/
evidence fixtures. [#32](https://github.com/dragginzgame/ic-timers/issues/32) is
closed. This does not qualify the new 0.14.22 compact collector or relabel the
cancelled upstream run; registry publication was not independently checked.

Released **0.14.20** is `40611eff87b3165e58528c597debaa95427cdf5a`;
the maintainer reports it pushed. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37807464542)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37807464532)
passes Linux checks/probe lint, explicit MSRV and both complete native macOS
gates. This completes [#31](https://github.com/dragginzgame/ic-timers/issues/31)'s
consumer acceptance. Its graph is Testkit 0.25.2 / Host 0.8.2 / Metrics 0.2.14 /
PocketIC 16.1.0; later results do not relabel this source's evidence.

Released **0.14.19** is `e637224e018afd758175e21de3afe3b95fa1a9ed`;
the maintainer reports it live. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37803792774)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37803792258)
passes Linux checks/probe lint and explicit MSRV. Intel is running and Apple
Silicon is queued at inspection, so complete native acceptance is pending. The
selected graph is Testkit 0.25.1 / Host 0.8.2 / Metrics 0.2.14 / PocketIC 16.1.0.
The subsequent 0.14.20 tooling adoption requires its own source-bound qualification;
it does not relabel these results or qualify compact evidence collection.

Released **0.14.18** is `c8d670d1e3181bebb1e67fcfc3eed0dcde0bddd2`.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37790176635)
passes Linux CI/probe lint and MSRV; Apple Silicon is running and Intel is queued
at inspection. Matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37790175542)
passes. This graph selects Testkit 0.24.0 / Host 0.7.1 / Metrics 0.2.13 and
PocketIC 16.0.0. Released **0.14.17**'s
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37774925568)
now passes Linux, MSRV and both complete native macOS gates at `031e6c67`.
That supplies complete normal-gate acceptance of its Testkit 0.23 / Host 0.6 /
Metrics 0.2.12 / PocketIC 16.0.0 graph; separate manual failure transport remains
scoped in the [evidence owner](#evidence-path-repair-and-01417-qualification).
Neither source qualifies the incoming PocketIC 16.1.0 client or db039 tooling.

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

### Incoming PocketIC lock alignment

After 0.14.18 release, the retained root lock selects PocketIC client 16.1.0
under Testkit 0.24.0's compatible requirement. The maintainer chose to qualify
that pair rather than restore client 16.0.0. The first release attempt correctly
failed exact admission against the old server pin at 16.0.0; its raw failure
remains in `.git/release-state/validation-failures/20261008T143516Z-2304745-1-pocketic-check.log`.

The 0.14.19 draft now prepares matching 16.1.0 server archives and independently
computed binary digests for Linux x86_64 and both native macOS architectures.
The [artifact review](#pocketic-artifact-pins) checks published archive identity
before decompression and inspects binary headers without execution. The existing
`ci/ic-tools.tsv` becomes the one consumer-owned matrix, removed from the shared
audit file roster through a fresh canonical export; installers and admission
helpers remain exact shared bytes. This keeps product qualification choices
from being overwritten by a generic snapshot refresh. Other tool pins are unchanged.

Make's default cache now selects `target/tools/pocket-ic/16.1.0/pocket-ic`.
Automatic provisioning verifies both hashes and exact version; explicit overrides
remain read-only and must match 16.1.0. The independent consumer fixture updates
all three expected host hashes/URLs and rejects hash-matching server 16.0.0.
No lock or Cargo package version is changed by this repair. The actual shared
alignment helper now reports **16.1.0** using cheap locked/offline metadata.
This check executes neither server binaries nor canisters. Snapshot integrity,
Bash syntax, source pin/cache/fixture consistency and diff checks also pass.
Fresh native startup, watchdog/rollback and policy-cohort execution remains required in the complete
user-operated release gate; source/hash/metadata inspection cannot supply those
results. Old 16.0.0 receipts and frozen artifact transport retain their own scope.

### Testkit 0.24 selection

During the 0.14.18 path repair, a concurrent root catalog/lock update selected
registry Testkit 0.24.0; the current incoming lock selects all four Host crates at
0.7.1 and Metrics 0.2.13. Those edits are preserved; this contributor ran no update
or fetch. Registry Rust files for Host 0.7.1 match 0.7.0, and Metrics 0.2.13 match
0.2.12; these latest patches have no Rust source delta. Cheap full locked offline metadata
passes, and the selected registry Rust source matches Testkit's released owning
checkout `e7a9c6c`. The harness has no `artifacts::read_wasm` call or exhaustive
Host process-error matches, so neither upstream public cut needs an adapter here.
The root catalog remains the only direct-dependency owner; Host/Testkit stay
outside the timer library graph. Timer runtime/API, audited PocketIC 16.0.0 and
expected production Wasm/instruction cost are unchanged. This selection requires
its own full user-operated consumer gate; the frozen 0.14.17 observations below
execute Testkit 0.23 / Host 0.6 and do not qualify this new graph.

### Testkit 0.23 adoption

The maintainer reports Testkit 0.23.0 live. The incoming root catalog/lock selects
that registry version and all four Host crates at 0.6.0; this preparation
preserves those existing dependency edits. Full locked offline metadata passes
without rewriting either file. All 38 selected Testkit Rust source files match
released commit `59b1c1de924116752282eac48c6531dce159ccc9`.

The actual harness calls `spawn`, `start_managed_server`, `url`, `connect` and
`try_build`, then drops the instance before its server. None of those call
signatures changed; there are no startup-error variant patterns, Host
`ExecutionError` literals or direct Host dependencies here. No harness shim or
second process owner is needed. Host/Testkit remain outside the timer library
graph, so no production Wasm, heap or timer instruction delta is expected.
The startup diagnostic changes and version-probe group cleanup stay with
[Testkit #30](https://github.com/dragginzgame/ic-testkit/issues/30) and
[Host #5](https://github.com/dragginzgame/ic-host-tooling/issues/5).

Released 0.23.0 expands the existing startup-error enum; it does not include the
accepted common error record from our handed-off candidate. That candidate is
still unapplied and unqualified. Applying that public cut after 0.23.0 requires
a new Testkit minor and fresh notes; do not overwrite finalized 0.23.0 history.
Our selection consumes only the actual released contract. Server cleanup and
PocketIC instance Drop remain synchronous and unbounded.

At inspection, Testkit's exact-source
[main](https://github.com/dragginzgame/ic-testkit/actions/runs/37772507706) and
[tag](https://github.com/dragginzgame/ic-testkit/actions/runs/37772507963) runs are
queued. Host 0.6 source `6f066e727c977e0b7ec8d3d77821df8508b95c64` has an
[in-progress native run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37769906817).
Earlier Testkit/Host passes do not qualify this graph. This compatible,
repository-only 0.14.17 draft keeps the full maintainer-operated native/PocketIC
gate; no contributor tests, builds, lint or release commands ran.

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
Linux jobs and the smaller tag job remain separate. Linux checks also run the
maintained PocketIC subjects/cohorts with the prepared internal toolchain; the
separate MSRV job retains minimum-version library and probe qualification.
The jobs use explicit
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

The released 0.15.0 collector reports `archive_bytes` (the completed compressed
tar's byte size) and `archive_seconds` (time spent in the archiver, using Bash's
whole-second counter) in its collection log. The existing upload includes that
log, so fresh hosted evidence retains the measurements without another artifact
or archive pass. Zero seconds means completion within the counter's resolution;
this is not a subsecond benchmark. Selection/check time, upload/download time and
the outer artifact ZIP size are separate. Failed archiving emits no completed
measurements. With the maintainer's explicit #30 instruction, the focused
collector and downloaded-verifier fixtures passed locally on 2026-10-09. Native
qualification of this measurement addition remains separate from
the frozen 0.14.23 compact round trips [below](#compact-hosted-qualification-at-01423).

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
22. Late qualification runs only after Linux's CI, nested lint and product
PocketIC targets or macOS's complete release gate succeed; it calls the real
logger against a failing
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

#### Shared failure archiver adoption

Current follow-up: all six frozen 0.14.17 hosted observations below are complete,
and released 0.14.21 passes the collector/path fixtures on all native hosts. The
compatible **0.14.22** draft now calls the already-vendored shared compact selector
with `ci/tool-versions.env` and `ci/ic-tools.tsv`. It retains full failed/changed/
unselected bundles; freshly verified exact active bundles retain their pins,
check logs, selection identity and IC receipts under `tool-evidence/`. Original
source/job/run/attempt identity, failure status, raw logs, fixture payloads,
partial-archive retention and upload/download names remain. A selector failure
aborts before archiving and retains partial selection output and metadata.
No executable compatibility path or separate full-mode caller remains locally.

The reviewed committed 0.1.32 follow-up also reuses equivalent validated IC pin
records across comments/order, preserving original installation receipts. The
download verifier uses the canonical AWK admission for those receipt records while
still requiring exact source-selected caller-pin evidence. Changed, malformed
and duplicate selections fail. Both local collector and downloaded-byte fixtures
cover distinct caller/installed provenance; no receipt rewrite occurs.

[Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66) has complete
native compact upload/download acceptance at `db039347d2372b877c1c46dcdd2b5c3aa9412009`;
the currently selected selector is byte-identical. The actual consumer fixture
adds tiny authenticated host/IC omission, corrupt-active retention, pins/receipt/
identity/log comparisons and failed-producer coverage. Existing path, status,
mode/link and archive-refusal cases remain. The existing downloaded verifier now
requires late compact selection records, exact caller pins/IC receipts and no
verified payload; negative fixtures reject missing/changed evidence and retained
active payloads. These checks qualify the current collector at its own source,
without relabelling the frozen 0.14.17 observations. No test/build/lint or hosted dispatch
ran during preparation. #30 remains open for the new collector's complete
user-operated native gate and source-bound early/late hosted transport, then
measurement of actual bytes and collection time. Old large-artifact measurements
prove the opportunity, not this collector's saving. The collector changes no
timer API or dependency selection; incoming lock edits are separate. No timer
Wasm/instruction/heap change or new IC recovery guarantee is claimed.

#### Compact hosted qualification at 0.14.23

On 2026-10-09 the maintainer's instruction to complete #30 authorized the focused
collector/download-verifier fixtures and two new manual dispatches at frozen
**v0.14.23**, `10a392f98d42701959d0c1d2deddfbef5c96144a`, attempt 1:
[early 37912633994](https://github.com/dragginzgame/ic-timers/actions/runs/37912633994)
and [late 37912637422](https://github.com/dragginzgame/ic-timers/actions/runs/37912637422).
These qualify the released compact caller; they do not execute the uncommitted
0.14.24 archive measurement output or incoming lock selections. No duplicate
observation was dispatched.

Both Linux producers reached their selected controlled failure (early status 22,
late status 2) and archived/uploaded successfully. The late producer first passed
its normal CI and probe lint. Actual downloaded ZIPs passed the maintained verifier
from the frozen source, including original identity, modes, status and compact
pin/receipt admission. Measurements are separate for the outer ZIP and inner tar:

| Stage / host | Artifact | ZIP bytes | tar.gz bytes | Regular payload bytes |
| --- | --- | ---: | ---: | ---: |
| Early Linux/X64 | 11606299473 | 1,341 | 872 | 779 |
| Late Linux/X64 | 11606958330 | 5,118 | 4,649 | 20,555 |
| Early macOS/ARM64 | 11610964142 | 10,711 | 10,240 | 781 |
| Early macOS/X64 | 11615583808 | 10,711 | 10,240 | 779 |
| Late macOS/X64 | 11617154805 | 10,711 | 10,240 | 20,699 |
| Late macOS/ARM64 | 11619245834 | 10,711 | 10,240 | 20,700 |

The late Linux ZIP is 99.9986% smaller than the previously measured 373,200,529-byte
0.14.15 ZIP. This is an observed cross-release artifact comparison, not a
same-source benchmark: retained diagnostics and tool selections differ. The
current verifier proves the verified active payloads are omitted while required
logs/pins/receipts and controlled failure evidence remain.

GitHub's archive-step timestamps fall within one whole-second bucket on Linux
at both stages. This measures the entire collection step, including selection,
and gives no subsecond archiver timing. The frozen source has no `archive_seconds`
output; the released 0.15.0 instrumentation needs its own complete native gate.
Local download/unpack/verification timings are separate from hosted collection.

Early ARM reaches its controlled status 22 and archives/uploads successfully;
its actual downloaded ZIP passes the frozen-source verifier. ZIP SHA-256 is
`ac6b1567b831cae7805e351266be2a8ebf07be79682b0b1c8f244033432d2aa8`; inner tar
SHA-256 is `a364c4887a9b4a03665d33341bdbbc4ef6d4cd853284651ad8d55ad1989c8f27`.
Early Intel and both late macOS producers subsequently reach their selected
failures; actual downloads also pass the frozen-source verifier. Both late macOS
jobs first pass their complete native release gate. All six original producers
archive/upload successfully, and all six hosted verifier jobs pass: early jobs
113819934118 / 113819934339 / 113819934371 and late jobs
113844785934 / 113844786086 / 113844786094. The overall workflows intentionally
remain failed because their producers preserve statuses 22 and 2.

The remaining macOS archive-step timestamp spans are one second for early Intel,
three seconds for late Intel and one second for late ARM. These are whole-step
API observations, including selection, not precise archiver measurements. The
completed six observations satisfy #30's compact transport acceptance, without
relabelling #23's earlier six observations or qualifying the later 0.15.0
five-tool/Testkit handoff and measurement instrumentation. Retained identities,
ZIPs, SHA-256 hashes, logs, API step timestamps and measurement reports are under
`/tmp/ic-timers-issue30.vdZlUd/`; hosted artifact retention still has its normal
expiry. Focused local fixtures passed, and Cargo manifest/lock plus the existing
worktree were byte-preserved through those checks. No commit, release, version
mutation or dependency update ran.

The following records describe the earlier archiver-only preparation.

The 0.14.17 draft prepares
[#30](https://github.com/dragginzgame/ic-timers/issues/30) through the reviewed
0.1.26 [snapshot](shared-tooling.md#shared-tooling-0126-preparation).
`collect-failure-evidence.sh` still owns checkout/event/job/host/run/attempt
identity and selection: fixture scratch, release-state validation logs, native
validation logs and all host/IC candidate sets. It passes root/relative-path
pairs to `archive-evidence.sh`, replacing its inline tar/exclusion assembly.
The shared helper's payload is unchanged from the all-host-qualified archive
source `eeb72e7`.

The helper refuses occupied output paths and returns status 1 on an archive
failure, preserving original inputs and any partial archive. A retry must use
fresh runner scratch or preserve/relocate the previous archive first; collection
does not overwrite retained output. Original controlled command status remains
in the scenario's `status.txt` and job outcome, independently of helper failure.
The existing workflow output name/upload/download identity is unchanged.

The download verifier admits one conventional `./` prefix before canonical-name
and duplicate checks. `identity.txt` plus `./identity.txt` is still a duplicate;
absolute, traversal, empty and internal-dot components remain refused. Both
released bare-name archives and newly prefixed archives have the same evidence
identity contract. No additional wire discriminator or alternate verifier exists.

Maintained fixtures now include colon/newline filenames, actual managed relative
activation symlinks, retained selected/unselected bundles, partial tar failure,
occupied-output refusal and fresh paths for independent collection attempts.
The manual verifier adds prefixed acceptance and alias/traversal rejection.
Those fixtures have not run during preparation. Native Linux/Intel/ARM consumer
qualification and fresh early/late hosted round trips remain acceptance work;
the closed #23 observations below retain their original source and scope.

Full installer retention remains the default. The measured >99.99% installed-tool
overhead is not fixed by this archiver adoption, and no size reduction is claimed.
[Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66) owns the
compact-selection policy: exclude a managed active bundle only after successful
verification bound to that exact selection; retain it on failed verification,
changed selection or unknown ownership. Host lacks the IC receipt shape, so
glob-based exclusions would lose useful diagnostics. Keep #30 open until the
adapter's consumer acceptance and the selection obligation are resolved.

No named function, method or type is deleted by the adapter. Inline archive
mechanics are replaced by their shared owner. No timer runtime/API, provider,
PocketIC version or release-gate selection changes; expected production
Wasm/instruction impact is zero. Source, syntax and snapshot checks are
preparation evidence only.

#### Evidence path repair and 0.14.17 qualification

The compatible **0.14.18** draft repairs consumer-owned evidence bootstrap for
[#30](https://github.com/dragginzgame/ic-timers/issues/30), following the reviewed
pattern from [Shared #67](https://github.com/dragginzgame/shared-tooling/issues/67).
The collector, qualification driver and fixture anchor relative operands before
physical `cd`. Captured PWD carries a non-newline suffix, so command substitution
cannot strip legal terminal newline bytes or mix CDPATH diagnostics into a path.
The driver still rejects a different physical workspace and accepts a matching
alias. Temporary roots are resolved explicitly; Git object-path capture preserves
the producer's framing separately. No shared payload is patched or new path API
introduced. No named function, method or type is removed.

The maintained fixture adds copied consumer scripts under a physical checkout
ending in a newline, inherited CDPATH, a matching workspace alias, a different
workspace rejection and a newline-ending runner directory. It drives both stages
and compares retained status/input/log bytes after actual shared archiving.
The reviewed [Shared Tooling 0.1.27 refresh](shared-tooling.md#shared-tooling-0127-preparation)
repairs the canonical bootstrap, so these additional cases now copy the real
installer/logger and their companions. The temporary path-only substitutes are
removed. Both normal and newline-ending path cases check the real retained
candidate bytes, controlled status and late combined log. This coverage does not
claim that every vendored script works from a newline-ending checkout. Fixture
execution, Bash 3.2 and native qualification of the local fix remain user-owned;
preparation ran integrity/source/syntax/diff checks only.

The maintainer's explicit request to do the proposed hosted qualification
authorized two new frozen-source dispatches at **v0.14.17**,
`031e6c67dccdd043ff11e20d9978359a4ec6afc8`, attempt 1:
[early 37776644653](https://github.com/dragginzgame/ic-timers/actions/runs/37776644653)
and [late 37776654992](https://github.com/dragginzgame/ic-timers/actions/runs/37776654992).
These execute the released shared archiver and Testkit 0.23 / Host 0.6 graph,
not the uncommitted path repair or incoming dependencies. Do not redispatch a
lost reply or relabel old #23 acceptance. Original job failures are intentional;
acceptance requires each expected failure, successful archive/upload and exact
downloaded identity/bytes/modes/status.

Both Linux checks jobs reached their selected controlled failure and passed
archive/upload. Early artifact **11549169587** is 1,342 bytes; late artifact
**11550069656** is 373,203,770 bytes (GitHub-reported ZIP sizes). Both Linux downloads pass the byte-exact released verifier. The native
late macOS observations now reach the intended controlled failure after complete
native release gates, archive/upload successfully, and pass the hosted downloaded
verifier: Intel artifact **11555254656** (228,372,951-byte ZIP), verifier job
**113361599275**; Apple Silicon artifact **11556628023** (213,944,791-byte ZIP),
verifier job **113361599424**. Linux hosted verifier **113361599302** also passes.
The early Intel job **113309082998** reaches its controlled failure and uploads
artifact **11553654328** (10,711-byte ZIP); its local download passes the byte-exact
released verifier. Early Apple Silicon subsequently reached its intended failure
and uploaded artifact **11557756245** (10,711-byte ZIP,
`sha256:b30b088be65b1836b7e2ef9a2fd95fa17092b4f5799c188f912f9d58b238be91`).
All three early hosted verifier jobs **113366289284 / 113366289320 / 113366289845**
pass, including rejection cases, actual download and exact source/status/bytes/
modes. The Apple Silicon verifier log explicitly names that artifact and reports
`Downloaded early failure evidence qualified for macos macOS/ARM64`.
Both runs remain at attempt 1 with their intentional failed producer outcomes:
**all six** source-bound host/stage observations are now complete. The macOS
late and early ARM archives were verified by maintained hosted jobs, without
another local download. Earlier local Linux/Intel receipts keep their scope.
Artifacts, original job logs and the frozen verifier remain under
`target/evidence/hosted-failure-artifacts/`; the final ARM verifier log inspected
on 2026-10-09 is retained at `/tmp/ic-timers-01417-early-arm-verifier-api.log`.
No new dispatch, local verifier execution or release operation ran in that review.

At the frozen source, full installer retention remains selected. The compact
selector's corrected db039 full/compact transport now passes all three native
hosts; current consumer preparation is recorded in the
[adoption owner](shared-tooling.md#compact-consumer-evidence-preparation).
No consumer archive-size saving or stronger IC recovery guarantee is claimed.
The frozen observations qualify 0.14.17's archiver and original graph, not later
path changes, compact selection, unrelated outages or other package identities.

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

The subsequently completed **0.16.3** native jobs
[ARM 113880561216](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561216)
and [Intel 113880561677](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561677)
both fail during tool preparation at `Makefile:270: multiple target patterns`,
before any native release gate. Logs retained under
`/tmp/ic-timers-0165-review.5lms07fg/` bind that failure to the combined
target-specific override/export syntax. **0.16.4** uses separate global override
assignment and export for direct delivery, removing that exact parse defect.
Its two native jobs are queued at inspection, so neither the source repair nor
older Linux checks qualify native acceptance. Do not retain the superseded
"0.16.3 native jobs queued" description as current evidence or rerun those jobs.

### Formatter prerequisites

Both `make fmt` and `make fmt-check` depend on `format-tools-check`. The reviewed
shared guard at `scripts/ci/check-format-tools.sh` requires successful exact
cargo-sort 2.1.4 output using the `ci/tool-versions.env` pin and successful
rustfmt availability for the selected toolchain. Failed probes reject even if
stdout looks correct. They force Cargo
offline and disable rustup automatic installation; setup remains explicit through
`make update-dev` or CI. The guard neither formats nor builds. The following
formatter recipes in `make/rust-format.mk` cover every member of the single root
workspace with their existing options. The hook's isolated index must include
both guards, `scripts/ci/run-formatting.sh`, all four Make includes, the current
Makefile and `ci/tool-versions.env`.
These formatting entrypoints also bind snapshot routing to their current root;
an inherited external snapshot cannot replace the indexed prerequisite checker.
Their pin input is bound to the indexed `ci/tool-versions.env`, preserving the
previous consumer-owned formatter selection rather than ambient host pin routing.
Fixture wiring and pending native qualification belong in the
[adoption owner](shared-tooling.md#shared-tooling-0211-formatting-and-make-admission).

The adjacent `make/execution.mk` admits the running Make executable before
recipes, using the probe beside the selected include. Shared 0.2.11 probes
MAKEFLAGS and MFLAGS independently through GNU Make's option parser. It rejects
ignore-errors, dry-run, touch and question modes even when MAKEFLAGS is cleared
or replaced, and refuses assignments that erase Make's retained MFLAGS evidence.
[Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30) records the
completed upstream repair. Recursive `MAKE` arguments stay with the consumer invocation;
they do not enter the isolated probe. Runtime snapshot/delivery/pin bindings
remain authoritative at their existing target boundaries. All scratch callers
copy the execution companion and probe before their first Make parse; the
index-hook fixture stages them too. This requires no tool installation or Cargo
selection and does not qualify actual consumer execution until the gate runs.

The consumer hook fixture keeps different index and unstaged `Cargo.lock` bytes.
Successful selected-file formatting and direct checks must preserve both; existing
mode-refusal, partial-staging and failed-formatter comparisons also include the
working lock. This closes #35's missing lock-preservation assertion without
changing hook behavior. These additional pending 0.16.6 cases remain unexecuted
by the contributor.

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
macOS and prepare this repository's Rust toolchain before the common aggregate.
The reviewed installer itself requires no sudo.

`make cloc` reports all four members of the root workspace, including the
unpublished probes. `CLOC_MANIFEST=Cargo.toml` explicitly selects that same graph;
there is no independent testing manifest. The refreshed reporter isolates its
fixture workspaces and excludes configured build output, including aliases.
Fleet tooling inventories now run centrally from Shared Tooling; this consumer
does not select the optional reporter or its dedicated regression suite. The
[current adoption owner](shared-tooling.md#shared-tooling-committed-0134-follow-up)
records that selection boundary. Counts do not establish instruction or Wasm
savings.

The [0.1.18 adoption owner](shared-tooling.md#shared-tooling-0118-refresh)
records source inspection and pending Linux/native macOS consumer qualification.
Current [Shared 0.3 adoption](shared-tooling.md#shared-tooling-030-complete-toolset)
includes the complete Cargo toolset in both aggregates. `update-dev` bootstraps
the pinned Rust toolchain, calls `install-tools` then `tools-check`, and installs
the formatting hook. CI prepares its declared toolchain before the same ordered
aggregate, exposing `.tools/rust/bin` alongside host and IC paths. There is no
separate global cargo-sort installation. The selected pin file already contains
cargo-sort 2.1.4, cargo-sort-derives 0.13.0 and candid-extractor 0.1.6.
No install, test/build/lint or formatter runs during contributor preparation.
Ordinary checks never download tools.

### Pinned IC tool setup

`make install-ic-tools` explicitly prepares Quill 0.5.4, ICP CLI 1.6.0, didc
0.6.2, ic-wasm 0.11.1 and wasm-opt 132 from
[`ci/ic-tools.tsv`](../ci/ic-tools.tsv). `make ic-tools-check` verifies this
five-tool bundle offline. `make install-tools` / `make tools-check` run host,
IC and Cargo tools, then `install-testkit-server` / `pocketic-check` through the
ordered local target lists. Standalone owner targets remain available, and the
ordered release preflight/validation rosters retain their established boundaries.
Shared Tooling 0.2.0 removes PocketIC from its IC policy. An existing
six-tool bundle fails admission until explicit setup selects a new bundle;
previous bundles, pins and receipts are retained.

`make install-testkit-server` invokes the [local adapter](../scripts/dev/testkit-server.sh),
which reads the single registry Testkit selection from the root lockfile. The
reviewed shared Cargo installer prepares only that package's `ic-testkit-server`
binary in release profile and checks its exact receipt/bytes. The CLI owns
server setup under `.tools/testkit-server`, asset hashes and compatibility.
`make pocketic-check` invokes both offline admissions and prints the admitted
absolute server path. No global CLI, local server catalog, client alignment rule
or Make binary override is retained. Failed admission never invokes setup.
Missing or invalid CLI admission retains the canonical installer's exact selection
diagnostic and failure status; the local adapter adds `run make install-testkit-server`.

Standard release preflight now calls `fetch`, `install-testkit-server`, then
`pocketic-check`, after admitting source/version/notes and before validation or
version mutation. Each call finishes before the next, including when the outer
Make is parallel. Existing saved-release reconciliation decides whether this
preflight is needed; no setup prerequisite bypasses it. Ordinary checks remain
offline and do not install. Explicit Cargo offline policy is inherited by setup.

Developer update and the Linux/MSRV/native CI preparation sites explicitly set
up the CLI/server after host prerequisites. `release-verify` runs locked fetch,
explicit Testkit setup, then offline admission before the complete existing
product gates, preserving direct user invocation without a prior preflight.
Valid selected installations are reused. Each watchdog/cohort command first
depends on offline `pocketic-check`, then rechecks after compiling its probes
and passes only the admitted path to the test harness. Product tests still own
fresh servers/instances and their existing startup deadlines; this handoff adds
no shared-server reuse or recovery guarantee.

Shared archive selection preserves failed CLI builds and Testkit `.setup-v1-*`
attempts without selecting successful server bundles. Scope and pending native
qualification belong in the [adoption record](shared-tooling.md#shared-tooling-020-hard-cut).
Ordinary validation never downloads. Setup neither deploys a canister nor selects
credentials or a network target.

### Testkit 0.27 preparation

After released IC Timers 0.16.0
`984c2f0a92f7e3ebde604f88895b12fb2b78cb2c`, incoming maintainer edits select
Testkit 0.27.0 in the sole root catalog/lock. Contributors preserved those edits.
The undated 0.16.1 notes prepare this repository-only update; production timer
source, public APIs and canister dependencies are unchanged.

The downloaded registry startup owner and CLI match Testkit's tagged 0.27.0
source at `f2d9fc6f197bcb9e669a7abaa1135f4cdffdd9ee`. Its startup error is a
record with `failure()`, bounded `output()` and independent command/server cleanup
reports. The private [harness](../testing/crates/ic-timers-pocketic/src/harness/mod.rs)
uses retained spawn/connect/build APIs and `expect`, without matching retired
error variants. Its error's `Debug` includes the original cause and cleanup/output
fields; adding downstream error reconstruction would duplicate the owner.
The CLI adapter already derives its version from the lock. No consumer rewrite,
old error alias or second CLI selection is needed.

Locked offline metadata on 2026-10-09 resolves one registry Testkit 0.27.0,
Metrics 0.3.1 and PocketIC 16.1.0, with all four local members still at 0.16.0.
Registry source comparison and metadata are retained at the directory referenced
by `/tmp/ic-timers-testkit027-current`; manifest and lock bytes are preserved.
These are preparation checks, not startup or compilation qualification.

[Released Testkit 0.27 CI](https://github.com/dragginzgame/ic-testkit/actions/runs/37923315911)
passes Linux portable-host, MSRV and PocketIC concurrency jobs; its Linux complete
checks remain in progress and macOS gates queued at inspection. The corresponding
[second run](https://github.com/dragginzgame/ic-testkit/actions/runs/37923315803)
also has pending complete/native checks. Keep adoption acceptance pending until
upstream native gates and the selected consumer graph's setup/offline check,
probe compilation, actual startup/recovery and policy cohorts pass on all three
hosts. The existing 0.16.0 runs use Testkit 0.26.0 and cannot qualify 0.27.0.
No duplicate hosted dispatch, new test/build/lint, tool setup or release ran in
this preparation. No production Wasm, instruction or heap delta is expected
from this host-only dependency update; none was measured.

The maintainer subsequently released **IC Timers 0.16.1** at
`6b508cc0ebcb215d04c2c54ce234d984cb771eda`, selecting Testkit 0.27.0, Host 0.9.2
and Metrics 0.3.2. Its [main Linux/MSRV](https://github.com/dragginzgame/ic-timers/actions/runs/37938451791)
and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37938451752)
pass; both complete native macOS jobs remain queued. Setup and fixtures alone
do not establish actual Linux product startup or all-host #34 acceptance.

A later incoming lock selects **Testkit 0.27.1**. The owner's entire library/CLI
source is unchanged from 0.27.0; this patch repairs its own Make CLI build ordering
and heavy branch/tag CI duplication. Timers already builds the lock-selected
published CLI through the shared installer and needs no new adapter. All 40
downloaded Testkit source files match tagged 0.27.1; all eight Metrics 0.3.2
source files match its tag and are unchanged from 0.3.1. Locked offline metadata
resolves one identity for every selected owner, preserving Cargo bytes and all
four local package versions at 0.16.1. These are source/graph checks at
`/tmp/ic-timers-shared025.3v_vh3a0/`, not native execution qualification for this
incoming graph. Its full maintainer gate remains required.

A concurrent later lock update advances all four Host crates to **0.9.3**.
Their complete library source trees are unchanged from tagged 0.9.2. A fresh
locked offline metadata check resolves one of each at 0.9.3 and preserves this
new incoming lock's bytes; `metadata-host093.json` retains the later graph
separately from the earlier 0.9.2 inspection. No contributor dependency update
ran, and neither graph check establishes execution qualification.

### Linux product qualification for 0.16.3

Issue [#34](https://github.com/dragginzgame/ic-timers/issues/34) requires actual
consumer setup, offline admission and product startup/recovery/cohort execution
on Linux and both native macOS hosts. Released 0.16.2 at
`1e1255dc489dead90bbcbf7cd404deedf89300d0` passes
[Linux checks/MSRV](https://github.com/dragginzgame/ic-timers/actions/runs/37941328947),
but its Linux workflow stops after probe lint; setup and adapter fixtures cannot
establish product startup. Both matching macOS gates remain queued at inspection.

The pending 0.16.3 workflow adds one step in the existing Linux checks job:
`make pocketic-watchdog pocketic-cohorts MSRV="$IC_TIMERS_INTERNAL_TOOLCHAIN"`.
It follows CI and probe lint and precedes controlled late failure qualification.
The already-prepared internal toolchain supplies Cargo, rustfmt and the Wasm
target; no second Linux toolchain install or duplicate MSRV check is added.
The separate MSRV job retains its minimum-version checks. macOS keeps the
complete release gate, including its existing MSRV-selected probe execution.

Both product targets use the existing [Make owners](../Makefile): offline
Testkit admission precedes builds, admission is rechecked afterward, and only
the resulting server path reaches the probes. The private
[harness](../testing/crates/ic-timers-pocketic/src/harness/mod.rs) starts a fresh
managed server and IC instance for each subject. The watchdog target selects
the existing `tests::` subjects; the cohort target selects the existing size and
instruction subject with its measurement output. Normal step failure stops the
job and uses the existing failure collector/uploader. No new harness, alternate
server catalog, test fallback or dispatch is introduced.

This change adds native build/execution time to Linux CI; it has no production
Wasm, instruction or heap impact. During preparation, duration and hosted
execution remained unmeasured.
Source/workflow parsing, embedded shell syntax, document links and unchanged
locked graph checks are preparation evidence only, retained with the dirty
worktree diff at `/tmp/ic-timers-0163-issues/`. No contributor test/build,
lint, installation or release ran. Keep #34 open until the matching source's
Linux product step and both macOS complete gates supply setup/admission/startup
and recovery/cohort evidence. Earlier released runs retain their original scope.

Released **0.16.3** at `25957e206fbd351656870e9f87a23c47eed0c015` now passes
[Linux checks](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561620),
MSRV (113880561635) and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37948400870).
The retained Linux log proves Testkit setup/admission of its 16.1.0 Linux server,
adapter and staged-hook/Make fixtures, **14** actual product recovery/lifecycle
subjects and the policy-cohort subject with all four installed-Wasm/instruction
rows. The product step uses Rust **1.99.0**, `timer-probe` profile, Testkit **0.27.1**,
Host **0.9.3** and Metrics **0.3.2**. This completes Linux product acceptance for
that source; both macOS jobs (113880561216/113880561677) remain queued.
Raw logs and job identities are retained at `/tmp/ic-timers-0164-review/`.
These measurements must not be compared as a runtime optimization against
historical Rust 1.88 builds, or relabelled for the incoming Host 0.9.4 graph.
No contributor tests or duplicate workflow dispatch ran; only existing hosted
results were inspected. Keep #34/#35 open for native macOS acceptance.

### Host 0.9.4 graph preparation

After released IC Timers **0.16.3**
`25957e206fbd351656870e9f87a23c47eed0c015`, an incoming maintainer lock selects
all four Host crates at **0.9.4** instead of released **0.9.3**. Preserve that
lock. The sole root catalog still selects Testkit 0.27; no direct Host dependency
or alternate process owner is added. Testkit remains the private harness and
published CLI/server owner.

Host 0.9.4 at `4e3daebd5df07c6449279535668436024a45c02b` changes its repository's
Shared Make wiring. All **66** registry library source files across artifacts,
fs, process and tools are byte-identical to 0.9.3 and match tag `v0.9.4`. Its
[exact hosted run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37947548868)
remains pending at inspection. Library source equality avoids an unnecessary
Timers adapter rewrite; it does not qualify the new package selections.

Full locked offline metadata resolves one of each Host crate at 0.9.4, Testkit
0.27.1, Metrics 0.3.2 and PocketIC 16.1.0, with all four local members still at
0.16.3. Cargo bytes are unchanged through inspection. Source comparison and graph
evidence are retained at `/tmp/ic-timers-0164-review/`. This describes the root
test graph; the published Testkit CLI's own locked build remains with that owner.

The undated 0.16.4 notes contain repository-only dependency maintenance. No
production Wasm/instruction/heap change is expected or measured. Prefer bundling
this with the next code-bearing release unless the maintainer explicitly selects
a repository-only patch. No contributor dependency update, test/build/lint,
installation, staging, commit or release ran. The new graph still requires the
normal native gate and must not inherit 0.16.3's source-bound CI evidence.

On the later 2026-10-09 upstream review, crates.io lists Testkit **0.27.2**,
Metrics **0.3.4** and all four Host crates **0.9.7**. Committed Testkit
`1a8f2ff570ea1c3bd58b215e28af52e5d99870e8` has no library/CLI source change from
released 0.27.1; Metrics `5a5f1dab1f7ee3e1e5624c9d889148c39abf45f2` has no
crate source change from released 0.3.2. Host
`ca62e661918db2f4320743b9042a4993a5fff2aa` changes only
`crates/ic-host-tools/src/response/mod.rs` relative to incoming 0.9.4: it folds
empty-text validation into the existing hex digit scan, retaining typed errors
and JSON empty-response acceptance. Our harness uses Testkit's PocketIC startup,
not this response decoder. Testkit already re-exports the Host families for
actual host consumers; adding direct dependencies would create no benefit here.
The custom timer memory-page summary retains paired extents and growth maximums,
not the cumulative sample sum owned by Metrics. Keep those distinct semantics.
No new adapter, timer feature or production code deletion follows from this
review, and dependency selection/qualification remains user-owned.

The incoming lock at the start of this adoption already selects those latest
Testkit/Metric/Host releases. Preserve it without a contributor Cargo update.
Full locked offline metadata resolves one Testkit 0.27.2, Metrics 0.3.4, each
Host crate 0.9.7 and PocketIC 16.1.0, with four local members still at 0.16.3.
Both the saved incoming lock and metadata are retained under
`/tmp/ic-timers-shared028.zqdya2y6/`; they supersede the earlier graph preparation,
not the released execution evidence. Source equivalence does not qualify the new
package identities or their published CLI build. The complete user-owned gate
remains required.

### Testkit 0.28 and Host 0.10 released graph

The maintainer's pushed **0.16.4** at
`da921fc899d2c7c99ed0a6cacac6e2111307ac71` actually selects Testkit **0.28.0**,
all four Host crates **0.10.1**, Metrics **0.3.5** and PocketIC **16.1.0**.
Four root-owned members remain 0.16.4. This supersedes earlier preparation graph
selection, including the versions described by the finalized development prose;
it does not relabel any earlier execution evidence. Registry publication is not
independently checked. The sole root catalog owns
Testkit 0.28 and Metrics 0.3; members retain workspace inheritance.

Review Testkit commit `48cfff270d5e228c0e09d6ee452786fb14c9a66e` and Host
`c7bdc3d4e1c658957202eebd76bff2c51e22f645`: Host consolidates durable writes on
`write_with(path, options, producer)` and preserves publication/cleanup failure
state. Testkit updates those calls and preserves error evidence in cache and
server provisioning paths. Host 0.10.1 also synchronizes newly observed parent
directories when a competing creator wins mkdir. Our private harness consumes
PocketIC managed startup, never the changed durable-write APIs or cache error
matches. Therefore no timer adapter, direct Host dependency, compatibility shim,
public timer cut or retained-data reset follows. The published CLI is still
installed/check-admitted by its selected Testkit version; that owner CLI's locked
build remains distinct from the root test graph. Metrics 0.3.5 changes inspector
fixtures/docs, not measurement library arithmetic.

Full locked offline metadata is retained at `/tmp/ic-timers-0165-metadata.json`.
The current root Cargo/pin bytes are preserved through Shared 0.2.9 export under
`/tmp/ic-timers-shared029.00umv319/`.

[Exact Linux job 113946212101](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212101)
has now passed with this pushed source/graph. The actual log proves Testkit server
setup/offline admission, standard release adapters/gate, corrected version
preparation, actual staged-hook formatting/failure isolation, 142 native library
tests, 14 maintained PocketIC recovery subjects and all four policy cohorts.
Product/cohort commands use internal Rust **1.99.0** and the root `timer-probe`
profile; the separate MSRV job passes at **1.88.0**. Raw evidence remains at
`/tmp/ic-timers-0165-review.5lms07fg/0164-linux-job.log`. Tag truth also passes;
both native macOS jobs remain queued at inspection. No contributor test/build/
lint/setup/Cargo update/release runs for this evidence review. These results
qualify the released Linux graph, not native macOS behavior or the pending
Shared 0.2.9 consumer selection. Keep #34/#35 open for their remaining acceptance.

The four retained 0.16.4 cohort rows exactly match the earlier 0.16.3 Linux
observations at the same internal Rust 1.99.0 and profile: Wasm sizes are
241280 / 286066 / 286835 / 288401 bytes for baseline / once / after-completion /
watchdog, and all reported instruction/cycle fields are unchanged. This is a
comparison of existing hosted observations, not a new benchmark execution or a
claim about unmeasured workloads. The older raw log remains at
`/tmp/ic-timers-0164-review/linux-checks.log`; current measurements remain tied to
the newer graph above. No optimization or footprint regression is evidenced in
these maintained subjects.

Both native jobs subsequently complete successfully at the exact 0.16.4 source:
[Intel 113946212038](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212038)
and [ARM 113946212124](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569/job/113946212124).
Their raw logs prove host-specific Testkit PocketIC 16.1.0 provisioning/check
admission, the complete native release gate, corrected version preparation,
actual staged-hook formatting/isolation, 142 library tests, 14 recovery subjects
and all four cohorts. Logs are retained at `/tmp/ic-timers-0164-native.cutpdaam/`.
Native canister commands explicitly use **Rust 1.88.0** despite the internal host
toolchain being 1.99.0. Linux canister subjects use 1.99.0. Different native and
Linux cohort rows therefore are not an equal-input performance comparison.
No contributor tests, builds or deployment checks ran to obtain these results.
[Issue #34 closes with completed three-host acceptance](https://github.com/dragginzgame/ic-timers/issues/34#issuecomment-6095003127).
#35's original Make/hook acceptance also completes; its new uncommitted upstream
concise-formatting follow-up remains open. These results qualify the tagged graph
and its Shared 0.2.8 consumer, not the pending 0.2.9 selection.

An incoming lock on 2026-10-10 updates cc, smallvec and syn to **1.7.0 / 1.16.3 /
3.0.7**. Preserve this maintainer input. Cheap locked offline metadata at
`/tmp/ic-timers-0165-incoming-metadata.json` retains one Testkit/Metrics/PocketIC
selection and four local 0.16.4 members; it does not execute or qualify the new
transitive graph. Source-bound release evidence above remains tied to the old
lock. No contributor Cargo mutation, version change or validation rerun occurs.

### 0.16.5 Linux and tag acceptance

Released **0.16.5** `0d7b85ec6658f91421bd13c44fa19592d8cc029e` selects
Testkit **0.28.0**, Host **0.10.1**, Metrics **0.3.6**, PocketIC **16.1.0** and
four local 0.16.5 members, with all three snapshots at committed Shared **0.2.11**
`83efac446348dea024798a331d77933b24b429dc` (55/30/11 files).
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38036823188)
passes Linux checks **114168872952**, MSRV **114168873087**, ARM
**114168873084** and Intel **114168873187**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38036823190)
**114168872998** passes at the same source.

Actual raw logs `/tmp/ic-timers-0165-linux-job.log` and
`/tmp/ic-timers-0165-tag-job.log` prove the repaired hook output assertion,
sorter failure isolation, complete retained-log archive case, cleared/replaced
Make-mode refusal and release/version-preparation fixtures. Linux additionally
runs 142 library tests, 14 maintained PocketIC recovery subjects and all four
policy cohorts under its explicit internal Rust **1.99.0** canister selection.
This qualifies the released Linux graph, including the Metrics/cc/smallvec/syn
refresh; it does not alone prove native consumer acceptance. Shared's exact
0.2.11 upstream Linux/native acceptance is complete
and its #30/#91/#92 are closed, separately from consumer qualification.

ARM's raw `/tmp/ic-timers-0165-arm-job.log` additionally proves actual hook
formatting/failure isolation, release/version preparation, 142 library tests,
14 recovery subjects and all four policy cohorts. Its canister builds explicitly
select Rust **1.88.0**, while the host test toolchain is 1.99.0. This qualifies
the released ARM graph; do not compare its cohort values against Linux's
different canister toolchain as an optimization measurement.

Intel's `/tmp/ic-timers-0165-intel-job.log` now proves the same actual hook,
release/version, 142 library-test, 14 recovery-subject and four-cohort acceptance
at this exact source, with explicit Rust 1.88.0 canister builds. This completes
execution of the released formatting/Make adoption across all three hosts.
#35 retains the new explicit working/index lock-preservation proof; the 0.16.6
lock/path/preflight cases and graph remain outside that released evidence.

Pending 0.16.6 adds explicit index/working lock-preservation assertions to the
consumer fixture and updates formatter prerequisites. Those new assertions are
unexecuted; the released 0.16.5 evidence does not qualify them. No contributor
build/test/lint/setup or release runs; observations above inspect existing jobs.

### 0.17.1 pushed source and pending graph

Pushed release source **0.17.1** is
`2a8710834c9283ccc46daa4e4c93b1cc4629c6bc`. Its committed root lock selects Metrics
**0.5.0**, Testkit **0.31.0**, all four Host **0.11.0** packages and PocketIC
**16.1.0**, with four local 0.17.1 packages and Shared **0.3.1** snapshots
`fa452afaa5012866eb1c20820dfa8038c106e7ec` (55/30/11).
[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38051205368)
passes Linux checks **114210531711** and MSRV **114210531705**; ARM
**114210531587** and Intel **114210531746** remain queued at observation.
[The tag run](https://github.com/dragginzgame/ic-timers/actions/runs/38051205322)
is still running at the initial observation. These are job-state observations,
not downloaded fixture proof or complete host qualification.

The incoming maintainer-owned root lock now selects Metrics **0.5.1** under the
existing compatible `0.5` requirement. Inspection preserves those exact bytes,
all manifest/pin inputs and the real index. Full `cargo metadata --locked
--offline` stops because that archive is uncached; retained stderr at
`/tmp/ic-timers-shared032.rXciNI/metadata.err` records the failed attempt. There is
no contributor fetch, dependency mutation, rollback to 0.5.0 or claim that the
new full graph resolved. The normal maintainer-owned fetch/setup/release gate
retains its existing roles. Pending Shared 0.3.2 work is repository-only; no timer
source, production dependency requirement or measured optimization is added.

### 0.17.2 preparation inputs

During the subsequent Shared 0.3.3 inspection, the incoming maintainer-owned root
catalog selects Testkit `0.32` and its lock resolves **0.32.0**, all four Host
packages at **0.12.2**, Metrics **0.5.1** and PocketIC **16.1.0**. All four local
members remain **0.17.1**. Full `cargo metadata --locked --offline` now succeeds;
its graph and preserved inputs are retained at
`/tmp/ic-timers-shared033.0vrxoaq1/`. This supersedes the earlier cache blockage
and Testkit preparation observation for the current worktree, without changing
the released 0.17.1 graph or its evidence. No contributor fetch, catalog/lock
mutation, setup or qualification runs. Metadata resolution alone establishes
neither selected CLI/server admission nor native or PocketIC acceptance; the
normal user-operated gate must qualify this selected graph and new snapshots.

### Local fixture completion follow-up

After the maintainer reported **0.17.2** live at
`1cc87a467c4552b95d8a9728334e4e29eff0bc4c`, pending **0.17.3** repairs fourteen
consumer-owned fixture EXIT boundaries. Cleanup requires both successful status
and explicit completion after the assertions; incomplete/failed fixtures retain
their inputs, diagnostics and actual nonzero status. Release-gate and hook
admission now precede their first helper calls. Selected shared files are
unchanged and remain governed by their three immutable 0.3.3 snapshots.

The new [boundary check](../scripts/ci/test-fixture-completion.sh) copies the
actual initialization and EXIT admission of all sixteen local CI/release
fixtures, including itself and Testkit. It truncates each copy at the trap and
injects nounset, failed command, explicit nonzero, premature-zero, completed-zero
and completed-but-failed exits before any fixture body. It checks exact status,
successful cleanup and preservation of an evidence file for failure. Each probe
gets a fresh path marker and scratch paths containing spaces. The check is first
in `release-check`, so existing Linux/macOS gates run it with their selected Bash;
the standalone Testkit exit probes are consolidated there. No shared fixture is
patched or independently copied into the local boundary roster. These cases are
written, not run; [#38](https://github.com/dragginzgame/ic-timers/issues/38) remains
open for current-Bash and native Bash 3.2 qualification.

The incoming lock selects Testkit **0.32.1** under the existing `0.32` requirement.
During inspection external lock updates advance all four Host packages from
0.12.2 to **0.12.3** and Metrics from 0.5.1 to **0.5.2**. Preserve those newer
selections; no contributor lock write or rollback occurs. The root catalog,
pin catalogs and index compare exactly with the initial copies. This is
repository-only work with no runtime/API or measured Wasm/instruction change;
dependency and release execution remain maintainer-owned. Preparation inputs and
downloaded historical native logs are retained under
`/tmp/ic-timers-fixture-completion.9z8952mm/`.

Permitted syntax inspection covers sixteen local scripts, 31 embedded Bash
bodies and all 96 generated probe prefixes using the actual awk generator;
none of those prefixes or fixture bodies is executed. All three snapshot
integrity checks, seven added local documentation links and diff whitespace pass.
Initial locked offline metadata
resolves the Testkit 0.32.1/Host 0.12.2 selection. A fresh locked offline check
also resolves the externally updated Testkit 0.32.1/Host 0.12.3/Metrics 0.5.2 graph,
without changing its lock bytes. This is resolution, not execution qualification.

The same batch also repairs the production collector's status-only EXIT cleanup,
tracked separately by [#39](https://github.com/dragginzgame/ic-timers/issues/39).
`collect-failure-evidence.sh` requires explicit completion after final archive
diagnostics before successful cleanup. Incomplete zero-status exits become
failure; actual nonzero status and scratch evidence are retained. Archive
selection, paths, byte/mode handling and ordinary successful output are unchanged.
The existing evidence fixture adds six exit cases against copies of the actual
collector initialization/trap, truncated before Git inspection, tool selection
or archiving. Cases check status, successful cleanup, retained evidence bytes and
the retained-path diagnostic; full collection cases stay in their existing owner.
Syntax for both affected scripts and six generated collector prefixes plus diff
whitespace pass. No prefix or fixture executes. Cargo, lock, pins and index match
the separate preserved inputs at `/tmp/ic-timers-collector-completion.vb52tc9h/`.
Both #38 and #39 remain open for user-operated current/native Bash qualification.

The last local preflight repair captures the workspace-version reader's exit
status before comparing its output with `RELEASE_PREVIOUS`. Failed empty or
matching output cannot authorize source admission, fetch or setup. The existing
real-index fixture wraps that reader with a Bash stub and checks status 23,
untouched fetch/preparation logs, unchanged metadata/index and no release intent;
a successful mismatching read still refuses with status 1. The normal successful
preflight remains covered by its existing cases. Syntax for the adapter, index
fixture and four embedded Bash stubs plus diff checks pass; cases are written,
not run. Preserved inputs are at `/tmp/ic-timers-preflight-reader.1hfppy0_/`.
The same unchecked-observation pattern in the immutable pre-commit hook is
reported to [Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106).
There is no active downstream hook patch or duplicate hook path; canonical
producer repair and source-bound qualification must precede its refresh.

### Host 0.12.2 and Testkit 0.32 review

Remote Host's latest committed release is **0.12.2**
[`e1ef99e6a4c6d05f0b0d8364f8586c6cc358dadc`](https://github.com/dragginzgame/ic-host-tooling/commit/e1ef99e6a4c6d05f0b0d8364f8586c6cc358dadc).
All tracked Rust source is unchanged from 0.11.0. The 0.12 line's changes cover
setup, release-tool ownership, jobserver propagation and fixture retention;
the applicable shared Make/fixture fixes are adopted through Shared 0.3.2 rather
than a new direct Host dependency. Its
[CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38051203749) passes
Linux and MSRV with both macOS jobs queued at observation; source equality is
not new runtime or native qualification.

Testkit's local clean preparation commit `fb62fee8696443676c92a3f86d413251dffd9327`
selects Host 0.12 through its four public reexports and prepares 0.32.0. Remote
main remains released **0.31.0** `f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a` at
inspection. The prepared release changes no Rust source but changes public
Host package identity, so its 0.32 minor boundary remains Testkit-owned.
Timers consumes startup/CLI calls through Testkit and exposes no Host types.
Wait for published, qualified Testkit 0.32 before reviewing a root catalog/lock
update; keep the current 0.31/Host 0.11 path without a sibling dependency, patch,
second native route or borrowed unreleased library bytes. No canister Wasm or
timer instruction savings follow from these native tooling changes.

### 0.17.0 release acceptance

The maintainer reports **0.17.0** live at
`5e0d0865248f6ebfc1f98c896581f21e2ce67831`; its annotated `v0.17.0` tag resolves to
that same commit. All four local packages are 0.17.0. The released root graph
selects one Metrics **0.5.0**, Testkit **0.31.0**, each Host **0.11.0** package and
PocketIC **16.1.0**, with Shared **0.3.0** snapshots at
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` (55/30/11). The
[measurement owner](design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
records the public type identity and unchanged published arithmetic source.
Testkit's separately installed CLI uses its packaged lock, also selecting Host
0.11; no direct Host dependency is added to the timer library.

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38048325468)
passes Linux checks **114202244232** and MSRV **114202244227**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38048325379)
**114202243658** passes. Intel **114202244037** and ARM **114202244196** now pass.
No job is redispatched. Existing Linux/MSRV raw job logs and metadata are retained under
`/tmp/ic-timers-shared031.aFkkRB/`; completed-job REST readback supplies Linux and
MSRV logs during the earlier unfinished run. Newly downloaded Intel and ARM logs
are retained under `/tmp/ic-timers-fixture-completion.9z8952mm/`.

Linux logs establish actual execution of selected-commit and real-index release
checks, version preparation, root lock coherence, ordered Testkit adapter
failure propagation, 142 library tests, 14 maintained recovery subjects and all
four policy cohorts. Both native macOS logs now show complete tool setup/check,
Shared 0.3.0 snapshot integrity, ordered Testkit adapter failure propagation,
142 library cases, all 14 maintained recovery subjects, four policy cohorts and
the final `VALIDATION PASSED` marker, selecting Metrics 0.5.0 and Timers 0.17.0.
Together these qualify the released graph and README-gate removal on all three
hosts; they do not qualify later graphs or fixture changes.
[#36](https://github.com/dragginzgame/ic-timers/issues/36) and
[#37](https://github.com/dragginzgame/ic-timers/issues/37) now have complete
source-bound adoption qualification. The contributor inspected existing hosted
results and locked offline metadata; no new test/build/lint/setup or release ran.
Historical 0.17.0 notes describe the earlier 0.4/0.30 preparation; current usage
must follow the delivered graph rather than those version examples.

### 0.16.6 release acceptance

The maintainer reports **0.16.6** live at release commit
`0b929539686a5c428a6a3af96c2a88139cc5553d`. All four local packages are 0.16.6;
the root graph selects Testkit **0.28.1**, Host **0.10.2**, Metrics **0.3.7** and
PocketIC **16.1.0**. All three released snapshots select Shared **0.2.13**
`5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e`, retaining **55/30/11** files.
The separately installed Testkit CLI uses its packaged lock, including Host
0.10.1; the root graph does not change that installation contract.

[Matching main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38041494927)
passes MSRV **114182479493**, Linux checks **114182479646**,
Intel **114182479613** and ARM **114182479680**.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38041495172)
**114182480244** passes at the same SHA, with duplicate full tag jobs skipped.
Release availability, metadata and tag truth do not qualify the new execution
paths. The completed execution evidence below establishes consumer acceptance
for this released graph, independently of the new incoming selections.

This source adds explicit staged/working lock-preservation assertions, forbidden
directory cases, selected-CLI failure isolation and ordered preflight preparation.
The [Shared adoption owner](shared-tooling.md#shared-tooling-0213-selected-cli-preparation)
records their scope; prior 0.16.5 logs qualify only that earlier source.
Downloaded `/tmp/ic-timers-0166-{linux,intel,arm}-job.log` files now prove actual
hook formatting and failure isolation, directory admission, Testkit adapter
failure handling, release/version preparation, 142 library tests, 14 maintained
recovery subjects and all four cohorts on each host. The delivered hook fixture
contains the explicit staged/working lock-preservation assertions, so these
logs complete [#35](https://github.com/dragginzgame/ic-timers/issues/35)'s remaining
three-host proof. This does not qualify the later Shared 0.3 or incoming graph.
No contributor build/test/lint/setup or release commands ran; this records
existing hosted jobs.
[Completion feedback](https://github.com/dragginzgame/ic-timers/issues/35#issuecomment-6096552698)
closes #35 with its source-bound all-host proof.

### Dependency pin exceptions

[Exact exception records](../ci/dependency-pinning-exceptions.json) retain three
existing qualified selections rather than changing dependencies to make the new
checker pass. `ic-cdk-timers =1.0.0` fixes provider behavior audited in
[SAFETY](../SAFETY.md), including cancellation heap retention and dispatch limits.
`ic0 =1.2.0` preserves the production platform bindings and counter-1 reader
reviewed in the [measurement owner](design/callback-delivery-ownership.md#ic-metrics-02-adoption).
The root catalog retains `ic-cdk =0.20.3` for probe execution/suspension.
Current source-bound Linux PocketIC 16.1.0 recovery/cohort qualification is
recorded [above](#testkit-028-and-host-010-released-graph), including both complete
native macOS gates at their explicit canister toolchain. The historical 16.0.0 observations retain their own
source and do not define today's server selection. Testkit uses a compatible 0.20 requirement;
its former exact 0.17.3 exception is retired. These are product qualification boundaries, not blanket exact-pin policy.

Each exception matches its declaring root, dependency name and literal version.
Changing any selection requires reviewing its reason and rerunning the affected
source/runtime/host qualification; the old record cannot admit a new version.
The root lockfile and complete locked/offline metadata check remain required. Registry compatibility ranges, path inheritance and local workspace
ownership are unchanged. No exception permits a floating action or Docker image.

### PocketIC artifact pins

The following table is historical 0.14.19 artifact-review evidence. It is not
an executable pin catalog for the pending 0.15.0 handoff. Testkit now owns current
server selection, authentication and compatibility; the consumer adapter uses
its setup/check contract as described [above](#pinned-ic-tool-setup).

On 2026-10-08, official [release metadata](https://api.github.com/repos/dfinity/pocketic/releases/tags/16.1.0)
and the [PocketIC 16.1.0 release](https://github.com/dfinity/pocketic/releases/tag/16.1.0)
identify source `e9e42a6dd74acb9049017863e944cd5799be5b2b`. Assets **621918359**
(Linux x86_64), **621918384** (Darwin x86_64) and **621918361** (Darwin arm64)
were downloaded by exact asset ID. Each gzip matched the published archive hash
and byte size before decompression. The decompressed files supplied independent
binary hashes and ELF/Mach-O architecture checks. None was executed. Original
archives, non-executable decompressed bytes and `review.json` remain under
`target/evidence/pocketic-16.1.0-artifact-review/`. Artifact integrity does not
establish runtime recovery, startup/version execution or native macOS acceptance.
The complete user-operated gate must supply that fresh qualification.

| Release asset | Archive SHA-256 | Binary SHA-256 |
| --- | --- | --- |
| `pocket-ic-x86_64-linux.gz` | `131219d90dcf9bf6f3ed8ee02d8f55504ea24a6db382f5680675b7bd6b7ed3bf` | `b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db` |
| `pocket-ic-x86_64-darwin.gz` | `af9ad2d781530a43556ef2d1c8f93db99a2425c6cabdc78520f61922399ed530` | `a2ad872a5d84778b25a254c4eb4a8df99917b5792edaa7702d730de2d7de664d` |
| `pocket-ic-arm64-darwin.gz` | `9ae843fbb7ae6c6eb30137671c8629a80ef53b3a3652eb85b344847ac39f9fcd` | `2ffd9d5ae103cbb85289424056e68920459e702bf27317e38003b019a0a959d1` |

The former consumer server matrix, extracted-binary checker, downloader and
verification fixture are removed together in the hard cut. Their prior
qualification describes the historical route only; do not use the table to
recreate server admission alongside Testkit.

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

The gate starts with `make fetch`: `cargo fetch --manifest-path Cargo.toml --locked`
for the single root workspace, without restricting the target. This explicitly prepares
the selected lockfile's sources, including target-specific dependencies that
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
Release preparation explicitly installs the root-lock selected Testkit CLI and
its authenticated server. The subsequent offline admission and watchdog/cohort
commands use Testkit's admitted path. No custom binary override, consumer server
catalog or implicit validation-time downloader remains. See the
[setup contract](#pinned-ic-tool-setup).

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

`release-stage` selects only the three outputs the bump owns. Workspace members
inherit their versions, so their manifests are not version-bump outputs and
remain under the maintainer's separate code-staging ownership. Stage and commit
the intended implementation and supporting evidence before the combined release;
the release commit still rejects unrelated unstaged or untracked work.

Before mutation, the helper captures only its three output files: the workspace
manifest, root lockfile and changelog. Failed
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
release gate retains its product qualification. Consumer adapters select the
three metadata outputs and the complete root graph. Publishing stays separate.
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
the current workspace-version and lockfile check owners there. The root
workspace manifest-sort check and the exact dated changelog heading remain
required. No scripts from the older tree are executed. Failed archive reads,
including partial output, fail closed; the temporary copy is removed without
touching build artifacts, plans or validation logs. This is cold release-path
disk work and has no crate, Wasm, runtime instruction or heap impact.

Preflight reads staged, unstaged and untracked paths separately, with NUL records
and rename detection disabled, before admitting only the three metadata outputs.
Its existing bump check rejects candidate/changelog conflicts before validation
or intent creation. After admission, preflight invokes the existing `make fetch`,
`make install-testkit-server` and `make pocketic-check` owners sequentially for
the complete locked root graph and selected CLI/server. It does not require cached archives before
that owner can populate them. Fetching retains Cargo's configured network/cache
policy and the existing lock selection; there is no offline-to-online retry or dependency
update. The complete gate retains its existing fetch-first ordering, and a
failed fetch, setup or admission stops preflight before validation or release mutation.
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
all three outputs to be tracked and the worktree to match the index, then applies
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
the complete locked root graph, corruption of all three metadata outputs, tag
conflicts, archive failure before/after output, failed resolution/manifest sorting,
missing selection and temporary-copy cleanup. New cases cover stale, noncanonical
and absent README content independently of metadata integrity; these cases have
not been executed by the contributor. The real-Git tag fixture checks an earlier
selected commit separately from HEAD. The shared runner fixture
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

### Metrics 0.3.6 incoming graph

The preserved incoming root lock for pending 0.16.5 selects Metrics **0.3.6**,
Testkit **0.28.0**, Host **0.10.1** and PocketIC **16.1.0**, with four local
members still **0.16.4**. Metrics 0.3.5 and 0.3.6 have byte-identical sets of
all seven published `src/**/*.rs` files in the Cargo registry cache; upstream
0.3.6 changes repository snapshot diagnostics and the private Wasm inspector's
Host selection. This requires no timer arithmetic adapter or public semantic cut.
The incoming lock also updates cc, smallvec and syn. Locked offline metadata is
preparation evidence only; released 0.16.4's tests/cohorts do not qualify the new
graph. No contributor Cargo mutation, build/test/lint or measurement runs.

### 0.16.6 incoming Host and Metrics graph

The preserved incoming root lock selects Testkit **0.28.1**, all four Host crates
**0.10.2**, Metrics **0.3.7** and PocketIC **16.1.0**; all four local members
remain **0.16.5**. Full locked offline metadata is retained at
`/tmp/ic-timers-0166-incoming-metadata.json`. It proves one resolved package per
selection, not execution qualification of this incoming graph.

Relative to Host 0.10.1, the three other Host libraries' published Rust sources
are byte-identical. Host Fs changes only durable pathname writers and their tests:
reject `/` or `/.` suffixes before normalization, parent creation, staging or the
producer, preserving existing bytes and BeforePublication/InvalidInput errors.
[Host 0.10.2](https://github.com/dragginzgame/ic-host-tooling/commit/6b755763aca71b7ea8c3de40edf032093dfdc62c)
retains that source identity. Our harness uses Testkit's managed startup, whose
startup/server/provisioning sources match 0.28.0 in the published 0.28.1 package;
it needs no writer adapter or direct Host dependency. Host belongs to the native
probe graph and is absent from the production ic-timers package's dependencies.

The selected CLI is independently installed with `cargo install --locked`.
Testkit 0.28.1's packaged Cargo.lock selects Host **0.10.1**, whereas Testkit 0.28.0
selected **0.10.0**. Thus CLI setup gains the 0.10.1 competing-parent sync fix by
selecting the new Testkit package; our root Host 0.10.2 update alone does not put
the directory-suffix repair into that installed executable. Preserve the owner's
locked build rather than adding a second catalog, local override or unlocking it.
The existing ordered release roster already performs explicit selected-CLI setup
and offline admission before CI; [Shared #96](https://github.com/dragginzgame/shared-tooling/issues/96)
identifies Timers as an existing prevention example.

All seven published Metrics Rust source files match 0.3.6 exactly; 0.3.7 changes
its repository formatter/admission tooling. No arithmetic adapter or public
semantic hard cut is required. New package identities still require the normal
user-operated gate; no Wasm/instruction improvement is measured or claimed.

[Host exact-source CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38037252544)
passes Linux native, MSRV and Intel, with ARM queued at inspection.
These upstream results and metadata do not qualify the consumer's new graph.
No contributor Cargo mutation, build/test/lint/setup, commit or release runs.
Snapshot directory admission belongs to the
[0.2.12 adoption owner](shared-tooling.md#shared-tooling-0212-directory-admission).

### Host 0.11 ownership review

Latest committed Host **0.11.0** is
[`1d768c80a5bb87e3330a6b7bacfdfc543063968f`](https://github.com/dragginzgame/ic-host-tooling/commit/1d768c80a5bb87e3330a6b7bacfdfc543063968f).
Locked offline trees confirm all four Host crates at **0.10.2** enter this
workspace through the unpublished `ic-timers-pocketic` harness's Testkit **0.28.1**
dependency. The published `ic-timers` library graph contains none of them.
The harness imports only Testkit's PocketIC API and uses its managed-server child
owner; the CLI additionally uses Host artifact inspection, filesystem publication
and process admission. Testkit's independently installed CLI still uses its own
packaged lock, as scoped above. No Host type enters the public timer facade.

The Host 0.11 hard cut unifies output limits, adds per-stream terminate/truncate
retention and composes cleanup errors. Published Testkit 0.28.1 still uses removed
`CommunicationLimits`, old output-limit fields and individual execution-error
cleanup fields. Its 0.10 requirements cannot select Host 0.11. Adoption belongs
to [Testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47), including
its public Host reexports and resulting minor compatibility boundary. Timers has
no direct Host caller to migrate or reason to add a parallel dependency.
[Consumer feedback](https://github.com/dragginzgame/ic-testkit/issues/47#issuecomment-6095989932)
records this trace at that existing owner.
Our probes build through Make/Cargo, not Testkit's Wasm-cache build API; Host's
bounded live build diagnostics therefore supply no immediate timer optimization.

[Host release CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/38040092044)
was queued at inspection. Source review and graph checks do not qualify Host
0.11 or a future Testkit adoption. Current Shared **0.2.13** remains the adopted
remote revision; no further snapshot refresh or Cargo mutation is required by
this inspection. Required native/product gates remain with the future selected
graph's maintainer-operated validation.

### Testkit 0.29 and Host 0.11 preparation

During pending **0.16.7** preparation after released 0.16.6, maintainer catalog
and lock edits select published Testkit **0.29.0** and all four Host **0.11.0**
packages. Testkit's remote release is
[`e15cc2acfd9324f6877f854415005f91d169a031`](https://github.com/dragginzgame/ic-testkit/commit/e15cc2acfd9324f6877f854415005f91d169a031).
All 40 published Rust source files match that committed release byte for byte.
Complete locked offline metadata at `/tmp/ic-timers-0167-testkit029-metadata.json`
retains one Testkit/Metrics/PocketIC identity and four local **0.16.6** members.
Metrics remains **0.3.7**, PocketIC **16.1.0**. Incoming Cargo bytes are retained
under `/tmp/ic-timers-shared0214.axid05wo/incoming/`; the contributor does not
mutate dependency selection or package versions.

The unpublished native harness uses Testkit's unchanged `pic` startup/builder
calls. Version-probe output quotas and deadlines are unchanged by the Host
migration: Testkit expresses them with per-stream `OutputLimit::Terminate` and
`timeout: Some(...)`. Provisioning likewise keeps its prior quotas/deadlines.
Host's composed cleanup errors stay with Testkit; Timers does not flatten or
reconstruct them. No harness adapter or direct Host dependency is needed; the
library dependency closure contains no Host/Testkit package. Testkit's Cargo
build runner preserves its existing output policy and no deadline, and our
Make/Cargo probe builds do not invoke it. No timer Wasm/instruction saving is
measured or claimed.

The root lock also selects CLI **0.29.0** through the existing adapter. Its
published packaged Cargo.lock selects all four Host **0.11.0** packages, unlike
the released 0.28.1 CLI's independent 0.10.1 graph. Keep `cargo install --locked`,
the owner's setup/check commands and admitted absolute server path. PocketIC
selection stays 16.1.0. No retained server bundle/cache reset or compatibility
route is introduced. Testkit's public Host reexport hard cut requires its 0.29
minor release; Timers exposes none of those types, so this remains compatible
0.16.7 repository-only qualification work.

[Matching Testkit CI](https://github.com/dragginzgame/ic-testkit/actions/runs/38041559012)
is queued at inspection. [Testkit #47](https://github.com/dragginzgame/ic-testkit/issues/47)
owns upstream migration acceptance. Source equality and metadata are preparation,
not executed startup/recovery or native qualification. The normal user-operated
release gate must prepare/admit the selected CLI/server and execute library,
MSRV, probe lint, watchdog/recovery and policy-cohort checks on supported hosts.
No contributor builds/tests/lint/setup or release effects run for this review.
[Consumer feedback](https://github.com/dragginzgame/ic-testkit/issues/47#issuecomment-6096202942)
records the published selection and unchanged harness at its migration owner.
