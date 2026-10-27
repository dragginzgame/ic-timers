![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.6**, `12c9f28`. The undated **0.31.7** section records the
accepted release-tooling cleanup. This is repository-only work, normally bundled
with the next code-bearing release. The compatible patch heading does not request
an independent release. Cargo package versions remain 0.31.6; manifests, lockfile
and library/probe source are unchanged. Commits, staging, pushes and release
execution remain maintainer-owned. The checkout was clean when this batch began.

## Completed cleanup and ownership

Shared Tooling **0.26.21**, revision
`66c91c4412856baa147482e648fc80428db8b179`, owns the generic dependency-pin,
Cargo-metadata, changelog and release-change regression matrices. The canonical
same-revision exporter retires their four roots/payloads; **131** payloads remain.
The local impact fixture no longer dispatches the generic collector suite.
Actual pin/graph checks and Timers' metadata, changelog, source/tag/index and
failure-propagation integration tests remain. The coverage map is in
[Shared adoption](../shared-tooling.md#consumer-qualification-selection).

Committed-release and tag fixtures use `prepare-script-fixture.sh` for explicit
entrypoints and their declared closure, replacing copied transitive helper lists.
Local fixtures retain their substitutions and assertions. Testkit's dedicated
fixture owns setup/check forwarding and aggregate owner-admission failures;
release-gate tests retain parallel admission-before-compilation barriers and
correct driver/toolchain dispatch, with redundant standalone smoke checks removed.
No product validation or recovery guarantee is weakened.

## Upstream boundary and deliberate retention

[IC Tooling #255](https://github.com/dragginzgame/ic-tooling/issues/255) remains
open. The registry's latest stable Build release is still **0.68.9**; its retained
`WasmBuildEvidence` does not expose the record needed for artifact/compiler/recipe
facts. Keep Timers' existing retained-build and freshness checks until a supplying
API is published. No dependency update or local replacement is part of this batch.

The released 0.31.6 runtime/probe simplifications remain intact. Explicit Wire
codec hooks retain the supported boundary recorded in
[Platform #31](https://github.com/dragginzgame/ic-platform/issues/31).

## Verification and limits

Before-images and canonical export logs are in
`/tmp/ic-timers-tooling-0317.4tYjxn/`. All seven focused consumer fixtures pass:
selected-commit metadata, annotated tags/publication admission, version-preparation
rollback/dirty-worktree handling, release ordering, Testkit adapter admission,
impact policy and snapshot corruption. Actual dependency-pin, coordinated metadata
graph, changelog, shell syntax, snapshot integrity and diff checks pass. The real
classifier reports `repository`; the root lockfile is byte-identical to its
before-image. No Rust compilation or PocketIC execution is needed for this
tooling-only batch.

The 0.31.6 runtime/MSRV/Wasm and Linux PocketIC evidence remains in
`/tmp/ic-timers-cleanup-0316.GVZnfK/` with its original scope. Earlier Tooling
and Platform/Wire evidence remains in `/tmp/ic-timers-adoption-0315/` and
`/tmp/ic-timers-platform-wire-0314/`. Full release validation, native macOS and
live deployment remain separate; earlier Linux evidence is not new qualification.
