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

The helper refuses to continue if the changelog shape is ambiguous, the
target notes are empty, the target is already dated, or the worktree is not
clean.

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

These maintainer-owned targets run the CI gate, update workspace version and
lock files, commit, create an annotated `vX.Y.Z` tag, and push with tags. The
non-release `make patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review.

After the release tag is pushed, publish the crate with:

```text
make publish
```

The publish target requires a clean worktree with the current version tag at
`HEAD`, verifies the package, and then publishes `ic-timers` to crates.io.
