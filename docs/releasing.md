# Releasing

The workspace follows semantic versioning.

Keep completed user-visible changes in the populated `Unreleased` section of
`CHANGELOG.md`. The release bump automatically moves those notes under the new
dated version heading and leaves a fresh `Unreleased` section behind. It
refuses to continue if that section is missing or empty, the target version
already exists, or the worktree is not clean.

Use one of the standard release families:

```text
make release-patch
make release-minor
make release-major
```

For an exact version, use:

```text
make release-x VERSION=0.2.0
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
