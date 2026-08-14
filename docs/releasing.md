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

These maintainer-owned targets run the complete release gate, update the
workspace version plus both the root and nested testing lockfiles, commit,
create an annotated `vX.Y.Z` tag, and push with tags. The non-release `make
patch`, `make minor`, `make major`, and
`make bump-x VERSION=...` targets stop after the version-file update for
review.

Before changing any version file, every bump target runs the complete release
gate:

```text
POCKET_IC_BIN=/path/to/pocket-ic make release-verify
```

That gate includes `make ci`, the Rust 1.88 MSRV check, warning-denied linting
of every supported nested probe configuration, the six-test watchdog/recovery
PocketIC suite, and the four policy cohorts. The bump fails before mutation if
the binary is missing, does not report `pocket-ic-server 15.0.0`, does not
match the audited SHA-256, or any evidence fails. Supply the same
`POCKET_IC_BIN` variable to `make minor` or `make release-minor`; the nested
make calls inherit it.

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
