# Releasing

The workspace follows semantic versioning.

Before a release, move the relevant notes out of `Unreleased` in
`CHANGELOG.md` and add the exact target version. The release bump refuses to
continue unless that heading already exists and the worktree is clean.

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
