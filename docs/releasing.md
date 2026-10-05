![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Releasing

This guide is for maintainers preparing and publishing a new IC Timers
release. Library users do not need to follow this process.

The workspace follows semantic versioning. Before releasing, maintainers
classify the change, choose the appropriate version, run the required validation,
let the bump label the current changelog draft, and then publish the
release through the repository's release commands.

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

Keep completed user-visible changes in one undated section at the top:

```text
## [Draft]
```

Automated contributors maintain this draft through the accepted coherent batch
without selecting a new version for each focused change. If the maintainer names
a target, label that same draft `## [x.y.z]`. Do not add `Unreleased` or a separate
release-note queue. Historical notes remain evidence of their recorded subjects.

The explicit user-operated bump owns the final version: it labels and dates the
single undated draft and moves it above history. No separately prepared versioned
note or handoff status marker is required. Empty or absent drafts are presentation
gaps and do not reject a changed release subject; a missing changelog is created.
The maintainer should not need to fix a heading manually before deployment.

During preparation the helper refuses competing undated release candidates,
a named draft that conflicts with the requested bump, or an already dated target.
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

The repository checks also retain producer failure status during workflow
discovery and provider-source inspection. Actions validation completes a NUL
record listing before reading workflows and removes its temporary listing on
exit; nested paths and spaces retain their existing meaning. Provider validation
rejects a failed search or sort before comparing the allowed source path. The
repository fixture independently injects discovery, search and ordering failures
with empty or matching records, checks discovery cleanup, and retains accepted
pinned workflows and rejected unpinned workflows. Source, shell and embedded
fixture-shell syntax were reviewed, and diff whitespace checks passed. These
fixture scenarios have not been executed. This repository-only work changes no
runtime contract and supplies no native macOS qualification.

Combined release targets run `bump-version.sh --check` before deployment
validation. This preflight checks the requested version, impact, unambiguous
draft selection and structured README projections without changing version
metadata or running tests. An empty exact `VERSION` is rejected before the gate.
The bump helper accepts exactly one `patch`, `minor`, `major` or canonical
`x.y.z` argument after an optional leading `--check`. Missing or extra arguments,
including a misplaced or repeated check flag, fail with usage status 2 before
reading release metadata. The preparation fixture checks these rejections against
unchanged metadata bytes and permission bits.
The release-commit owner then runs its read-only `--check-before-bump` mode. It
accepts staged implementation changes and dirty metadata selected by
`release-stage`: `Cargo.toml`, `Cargo.lock`, `testing/Cargo.lock`, `CHANGELOG.md`
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

This resumes the prepared version. Running `release-patch` again requests another
patch bump. A prepared date or version does not prove a release tag or publication;
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
workspace version and both lockfiles, and `make release-stage` to stage release
metadata.

The user-operated release targets run the complete release gate, update the
workspace version plus both the root and nested testing lockfiles, commit,
create an annotated `vX.Y.Z` tag, and push with tags. If the workspace version
has no release tag yet, the requested bump still runs. `make release-patch`
always advances the patch version; it never reuses the current version. For
example, with Cargo at 0.8.2 and a current draft, stage and commit the code-bearing
changes, then run `make release-patch` to validate, bump and release 0.8.3. The bump
itself also supports a dirty worktree without a preparatory commit. An exact
`release-x` target must be a strict version increase.
Release metadata and both lockfiles are checked before the release commit;
unstaged and untracked work is rejected before committing or tagging.

If a combined release stops after the version bump, finish that version with
the phase targets instead of rerunning the combined target, which always bumps:

```text
make release-stage
make release-commit
make release-push
make publish
```

`release-commit` commits staged release metadata when the version is untagged.
If the release commit already exists, it requires a clean `HEAD` whose subject
is exactly `Release X.Y.Z`, verifies metadata and both lockfiles, then creates
the missing annotated tag. A retry with an existing tag verifies that tag's
type and commit. Arbitrary clean commits, conflicting tags and new staged
changes for a tagged version are rejected. A push-only failure can be retried
with `make release-push`. These phase targets do not repeat deployment tests;
the completed pre-bump gate remains the evidence for the prepared code.

`make fmt` and `make fmt-check` cover both the root and `testing/` workspaces.
`testing-check` uses that same formatting check before its nested probe lints.
The formatting hook checks both workspaces in the staged snapshot in a temporary
directory. It never formats or stages files. Unrelated working edits and partial
staging are preserved; unformatted staged Rust is rejected even when its working
copy is formatted. Run formatting and stage the intended content before retrying.

The non-release `make patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review without running build, lint or test suites.

### Host support

macOS host workflows are required by the
[adopted engineering baseline](shared-tooling/DRAGGINZGAME.md#host-support).
The requirement is separate from executed qualification; canister execution
continues to target Wasm on the Internet Computer.

| Host | Current workflow configuration and evidence scope |
| --- | --- |
| Linux x86_64 | Hosted Rust/MSRV jobs use Ubuntu runners. The release gate pins the audited PocketIC 15.0.0 Linux x86_64 artifact. Recorded results remain scoped to their original subjects. |
| macOS 15, Intel x86_64 | Declared host target. PR/main job uses `macos-15-intel`, Apple's Bash 3.2 and the complete release gate. Native execution and qualification for this change remain pending. |
| macOS 15, Apple Silicon arm64 | Declared host target. PR/main job uses `macos-15`, Apple's Bash 3.2 and the complete release gate. Native execution and qualification for this change remain pending. |

Version preparation uses Bash, Perl, Git and Cargo. It owns regular metadata
files; symlinked or non-file outputs are rejected before mutation. Applicable
Make targets use GNU Make. macOS 15 supplies Bash 3.2, GNU Make 3.81 and the
standard BSD/Unix tools used here. Required host tools are Git, Perl with core
`JSON::PP`, `File::Compare` and `Digest::SHA`, `curl`, `gzip`, and Rustup/Cargo.
The SHA-256 boundary uses `Digest::SHA`, without requiring GNU `sha256sum` or a
Homebrew tool installation. Native host prerequisites and setup must be qualified
at their owning workflow boundary.

For local macOS setup, install Rustup, then run `make update-dev` to install the
development toolchain declared in `rust-toolchain.toml`, its components and the
Wasm target. The full release gate also needs the pinned MSRV toolchain:

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
and the five staged metadata paths. It captures Git output with an ordinary
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

### PocketIC artifact pins

The verifier selects PocketIC 15.0.0 pins for Linux x86_64, Darwin x86_64 and
Darwin arm64 using independent OS and architecture queries. Unknown hosts or a
failed query reject before cache inspection. Explicit overrides use the same
host-specific binary hash and exact `pocket-ic-server 15.0.0` version check;
they are never automatically replaced. No caller-supplied digest or version can
relax these checks.

The pins below were inspected on 2026-10-05 against the official
[PocketIC 15.0.0 release](https://github.com/dfinity/pocketic/releases/tag/15.0.0)
and its [release asset metadata](https://api.github.com/repos/dfinity/pocketic/releases/tags/15.0.0).
Each downloaded gzip archive matched its published asset SHA-256 before
decompression. The Linux binary retained the existing audited digest; the macOS
binary digests were computed from those verified archives, with Mach-O x86_64
and arm64 formats inspected. None of these binaries was executed during this
inspection. Exact version checks and maintained PocketIC subjects remain required
on each native host; artifact integrity does not establish recovery evidence or
native macOS qualification.

| Release asset | Archive SHA-256 | Binary SHA-256 |
| --- | --- | --- |
| `pocket-ic-x86_64-linux.gz` | `972d592975bdd0f046b05b5414ed6f9a044c676ef912b8ac81d359dd4982b038` | `29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb` |
| `pocket-ic-x86_64-darwin.gz` | `1d133a07c08c8e8ce25a2d08e6e7a590c27a5621735a0ab95c19f111d73f9f72` | `e0a93fdd0b11345096797807002fcccecf7004c9ebfcdb1c0a3f4bcab2344964` |
| `pocket-ic-arm64-darwin.gz` | `e6a96df4559949091411877f562512e132b75916bb704c03494bc7877fcfea0e` | `3635e41075cded4c0fcfe4bb80b55d03324f8fa85a1a99a0f9eed2a097c50eb0` |

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

### Deployment validation

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
insufficient. Populate both lockfiles' caches and retry the requested operation;
retain their selected versions. `update-dev` installs the toolchain and hook,
and does not prepare these dependency caches.

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported nested probe configuration, the maintained watchdog/recovery,
ordinary-await and provider-churn PocketIC subjects, and the four policy cohorts.
If `POCKET_IC_BIN` is unset, the
gate installs the pinned PocketIC 15.0.0 artifact for the current supported host
into the ignored `target/tools` cache. It verifies the archive SHA-256 before
decompression and the audited binary SHA-256 before executing any
downloaded, cached or overridden binary, then checks its version. Diagnostic
paths also leave hash-mismatched binaries unexecuted. An explicitly supplied `POCKET_IC_BIN` remains
a strict override: a missing or mismatched override fails and is never
replaced automatically.

After the version changes, the helper updates `Cargo.lock` and
`testing/Cargo.lock`, then runs offline `cargo metadata --locked` with dependency
resolution against both manifests through `check-lockfiles.sh`. This catches
stale path-package versions without building either workspace or repeating
the evidence suite. `--no-deps` is not sufficient because it skips lockfile
validation. `release-stage` stages both lockfiles automatically.

The checker captures successful Cargo output before parsing JSON. Cargo failures
retain their exit status and diagnostic without a cascading parse error from
empty or partial output. The release-gate fixture records both locked fetch
commands and rejection at either download boundary, and checks that preparation
runs first and stops the gate on failure. The lockfile fixture covers empty and
matching JSON from failed metadata commands for each manifest while preserving
both locks. Shell and embedded fixture-shell syntax, recipe inspection and diff
whitespace checks passed; these fetch/metadata fixtures and real Cargo fetch
commands have not been run by an automated contributor during their preparation.
The maintainer's reported 0.11.8 preparation attempt
failed on uncached `js-sys 0.3.104`, then reported metadata rollback. Read-only
inspection found that archive and eight other selected testing archives absent
from the local default cache; it does not establish results in another cache or
native macOS qualification.

`release-stage` selects only the five outputs the bump owns. Workspace members
inherit their versions, so their manifests are not version-bump outputs and
remain under the maintainer's separate code-staging ownership. Stage and commit
the intended implementation and supporting evidence before the combined release;
the release commit still rejects unrelated unstaged or untracked work.

Before mutation, the helper captures only its five output files: the workspace
manifest, both lockfiles, changelog and README. Failed
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
before fetching dependencies. It then prepares both locked caches and invokes
`check-lockfiles.sh` against the tagged checkout itself, before running the
release fixtures. Fixture success alone does not establish coherence of the
checkout's actual locks. Failed downloads, locked resolution or resolved
`ic-timers` identity stop the job; both workspaces must resolve one package at
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
