# Releasing

The workspace follows semantic versioning.

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
strict canonical-SemVer increase, the exact release tag already exists, or the
worktree is not clean.

Before the clean-worktree and expensive evidence gates, the helper also scans
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

These maintainer-owned targets run the complete release gate, update the
workspace version plus both the root and nested testing lockfiles, commit,
create an annotated `vX.Y.Z` tag, and push with tags. The non-release `make
patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review.

Before changing any version file, every bump target runs the complete release
gate:

```text
make release-verify
```

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported nested probe configuration, the six-test watchdog/recovery
PocketIC suite, and the four policy cohorts. If `POCKET_IC_BIN` is unset, the
gate installs the pinned PocketIC 15.0.0 Linux x86_64 artifact into the ignored
`target/tools` cache. It verifies the downloaded or cached binary's version
and audited SHA-256 before use. An explicitly supplied `POCKET_IC_BIN` remains
a strict override: a missing or mismatched override fails and is never
replaced automatically.

After the version changes, the helper updates `Cargo.lock` and
`testing/Cargo.lock`, then runs offline `cargo metadata --locked` against both
manifests. This cheap structural check catches stale path-package versions
without repeating the expensive evidence suite. `release-stage` stages both
lockfiles automatically.

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
