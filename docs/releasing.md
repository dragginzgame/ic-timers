# Releasing

The workspace follows semantic versioning.

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

The classifier compares the worktree with the tag matching the current
workspace version and reports:

- `crate` when the publishable crate's source or manifests changed;
- `repository` when only paths outside the publishable crate source and
  manifests changed, such as documentation, evidence, external tests, CI, or
  release tooling; or
- `none` when no path differs.

This is a conservative mechanical boundary, not an API compatibility oracle.
For `crate`, review the public API and semantic contract and choose patch or
minor according to the rule above. Repository-only work normally stays
untagged: validate it with `make repository-check` plus any focused owner-local
evidence and bundle it into the next code-bearing release. This avoids forcing
exact-pinned shared-registry consumers to coordinate a package identity that
does not change runtime behavior.

An explicit maintainer-owned version-bump or release target overrides that
default. For a `repository` subject, the helper prints an advisory and then
runs the same complete release gate before continuing. It never invents crate
impact or silently weakens validation. A `none` subject is still rejected.

As soon as a target version is known, keep completed user-visible changes in
an explicit undated section directly below the empty `Unreleased` heading:

```text
## [Unreleased]

## [0.3.0]
```

Automated contributors create and maintain that section as part of the work;
the maintainer should not need a separate changelog-heading edit. The release
bump validates that the staged section is populated and automatically adds the
release date. For work without a named target, populated `Unreleased` notes
remain supported and are promoted automatically when a version is selected.

The helper refuses to continue if the changelog shape is ambiguous, the target
notes are empty, the target is already dated, the requested version is not a
strict canonical-SemVer increase, the exact release tag already exists, no
changes exist since the current version tag. Version preparation accepts the
current worktree and does not require a preparatory commit. A
repository-only subject emits an advisory but may proceed when the maintainer
has explicitly invoked the bump or release target.

Before version mutation, the helper also scans
the compact status for target-version wording likely to become stale, such as
`candidate`, `unreleased`, or a next action to publish after release. This is
advisory: it prints a warning and always continues. Free-form prose is never a
post-mutation release blocker; exact Cargo, changelog, release-note, status
marker, and tag structure remain the enforced truth.

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

Commits, tags, pushes and publication are user-owned. Automated contributors
use `make patch`, `make minor`, `make major`, or `make bump-x VERSION=...` to
prepare requested releases from the current worktree, then `make release-stage`
to stage release metadata for review. They leave commits, tags and pushes to
the user.

The user-operated release targets run the complete release gate, update the
workspace version plus both the root and nested testing lockfiles, commit,
create an annotated `vX.Y.Z` tag, and push with tags. The non-release `make
patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review without running build, lint or test suites.

Deployment validation belongs to the user. The combined `release-*` targets
run the complete release gate before preparing the version. If the version was
already prepared, run the gate directly before committing, tagging and pushing:

```text
make release-verify
```

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported nested probe configuration, the nine-test watchdog/recovery
PocketIC suite, and the four policy cohorts. If `POCKET_IC_BIN` is unset, the
gate installs the pinned PocketIC 15.0.0 Linux x86_64 artifact into the ignored
`target/tools` cache. It verifies the downloaded or cached binary's version
and audited SHA-256 before use. An explicitly supplied `POCKET_IC_BIN` remains
a strict override: a missing or mismatched override fails and is never
replaced automatically.

After the version changes, the helper updates `Cargo.lock` and
`testing/Cargo.lock`, then runs offline `cargo metadata --locked` with dependency
resolution against both manifests through `check-lockfiles.sh`. This catches
stale path-package versions without building either workspace or repeating
the evidence suite. `--no-deps` is not sufficient because it skips lockfile
validation. `release-stage` stages both lockfiles automatically.

After the release tag is pushed, publish the crate with:

```text
make publish
```

The publish target requires a clean worktree with the current version tag at
`HEAD`, verifies the package, and then publishes `ic-timers` to crates.io.

Hosted full CI and MSRV validation run on pull requests and `main`. A tag push
at the same commit does not repeat those Rust builds. Its small tag-only job
instead verifies that the event tag exactly matches the Cargo version, the
tagged commit is reachable from `main`, release truth is coherent, and the
annotated version tag points to `HEAD`. This preserves protection against an
independently pushed tag from an unmerged commit without running identical
validation twice at one SHA.
