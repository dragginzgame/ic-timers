![IC Timers — Internet Computer helper library](assets/ic-timers-readme-header.svg)

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

Combined release targets run `bump-version.sh --check` before deployment
validation. This preflight checks the requested version, impact, unambiguous
draft selection and structured README projections without changing version
metadata or running tests. An empty exact `VERSION` is rejected before the gate.
The real bump repeats these cheap
checks afterward and always advances the requested version.

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
| macOS | Required; supported versions and architectures still need a declared and natively qualified matrix. No macOS CI job or native qualification is recorded here. |

Version preparation uses Bash, Perl, Git and Cargo. It owns regular metadata
files; symlinked or non-file outputs are rejected before mutation. Applicable
Make targets use GNU Make; release fixtures additionally rely on GNU utilities
whose macOS setup must be qualified at their owning workflow boundary.

The repository and release-gate fixtures compare ordered newline records
directly with `cmp`. They no longer depend on Bash 4's `mapfile` or turn those
records into joined arrays. Expected gate order remains independent of Makefile
variables, including repeated PocketIC prerequisites and the exact prefix before
an injected leaf failure. Provisioning checks compare both override path and
automatic-install selection, including an empty override. Source and shell syntax
were reviewed; the changed fixtures have not been executed. This removes one
known Bash 3.2 obstacle without establishing native macOS qualification.

The current PocketIC verifier accepts only the pinned Linux binary's hash, even
for an explicit override, and automatic installation is limited to Linux x86_64.
A macOS release gate needs a separately audited host artifact and its own pinned
digest while retaining exact version/hash validation and strict override
ownership. An available upstream download or a Linux pass does not qualify that
gate. This baseline refresh changes no executable or validation boundary and
does not establish macOS support evidence. The
[adoption record](shared-tooling.md) scopes the refresh and local exceptions.

### Deployment validation

Deployment validation belongs to the user. The combined `release-*` targets
run the complete release gate before bumping the version. The
gate can also be run directly before committing, tagging and pushing:

```text
make release-verify
```

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported nested probe configuration, the maintained watchdog/recovery,
ordinary-await and provider-churn PocketIC subjects, and the four policy cohorts.
If `POCKET_IC_BIN` is unset, the
gate installs the pinned PocketIC 15.0.0 Linux x86_64 artifact into the ignored
`target/tools` cache. It verifies the audited SHA-256 before executing any
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
