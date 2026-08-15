# AGENTS.md

This file is normative for automated contributors.

## Session handoff

- Read `docs/status/current.md` first in a new session. Continue from that
  compact handoff instead of reconstructing repository state from chat
  history.

## Scope

- Make changes only in this repository unless the maintainer explicitly names
  another exact target and authorizes mutation there.
- Inspection, review, audit, diagnosis, design, and feedback requests for other
  repositories are read-only.
- Preserve unrelated dirty worktree state.

## Status

- This is a pre-1.0 timer runtime. Describe only behavior backed by maintained
  native or PocketIC evidence, at the level of guarantee that evidence proves.
- Keep the direct `ic-cdk-timers` dependency solely behind the private
  `platform` module. Wrap it; never publicly re-export the provider crate,
  provider module, provider handles, or provider functions.

## Pre-1.0 hard cuts

- Every superseding change before 1.0 is a hard cut by default. Delete the
  replaced API, behavior, snapshot shape, storage path, and tests in the same
  change; do not preserve a pre-1.0 contract merely because it was released.
- Do not add deprecated forwarders, compatibility aliases, dual readers,
  migrations, feature-gated legacy paths, or fallback behavior for an earlier
  pre-1.0 release unless the maintainer explicitly authorizes that one
  compatibility exception.
- Update current tests, documentation, and downstream adapters in the same
  change. Historical changelogs may describe removed behavior but do not
  justify keeping executable compatibility code.
- A hard cut governs implementation, not version selection. Within pre-1.0
  Cargo compatibility, removing or incompatibly changing a public item or
  public semantic contract requires the next minor line (`0.x` to `0.(x+1)`),
  never a patch. Do not restore a shim to repair an already-published version;
  record the mistake and apply the correct boundary to future changes.

## API and safety hygiene

- Distinguish inert snapshots from runtime authority. Snapshot values may be
  projected by consumers but must not become an alternate mutation path into
  timer control or platform handles.
- Public validation boundaries need negative tests. Production timer paths
  should return typed errors rather than panic when input or recoverable state
  can be invalid.
- Keep safety claims aligned with `SAFETY.md`; reserve recovery claims for
  behavior backed by the required PocketIC evidence.
- A shared-registry adoption is atomic: require one resolved `ic-timers`
  package ID and remove the consumer's direct provider path, parallel timer
  registry, pending-command machine, and timer instrumentation in the same
  pre-1.0 hard cut. Never endorse a staged dual-runtime migration.

## Delivery

- Update `CHANGELOG.md` and the open release-line note for every meaningful
  change without waiting for a separate changelog request.
- Distinguish crate-impacting work from repository-only documentation,
  evidence, CI, and release-tooling updates. Repository-only work is committed
  without changing the package version, creating a tag, or publishing; keep
  its notes under `Unreleased` and bundle them into the next code-bearing
  release. `make release-impact` reports the mechanical classification and
  every bump refuses a repository-only subject. Do not manufacture crate
  impact merely to bypass that guard.
- Once the maintainer names a target release, immediately create and maintain
  an undated `## [x.y.z]` section directly below `## [Unreleased]`. Put that
  release's notes there and keep `Unreleased` empty; do not leave named-release
  notes only under `Unreleased`.
- When the maintainer asks for the version bump, run the matching bump target.
  The release helper must validate the staged section, run `release-verify`,
  and add its date automatically. `release-verify` must retain the normal CI,
  MSRV, nested-probe lint, watchdog PocketIC, and policy-cohort gates; it fails
  closed unless `POCKET_IC_BIN` matches the exact audited PocketIC version and
  hash. When no override is supplied, provision that pinned artifact in the
  ignored repository tool cache automatically; never weaken validation or
  overwrite an explicit override. After version mutation, update both root and
  `testing/` lockfiles, verify both with cheap locked metadata checks, and stage
  both. Do not require the maintainer to edit a changelog heading by hand,
  prepare a test binary manually, or remember a separate evidence command.
- A pre-bump check may warn about free-form release prose that is likely to
  become stale, but it must remain advisory and run before version mutation.
  Never make interpreted prose a post-mutation release blocker.
- Run full hosted Rust and MSRV validation on pull requests and `main`. A tag
  push may use a smaller release-truth job only when it also verifies the exact
  version tag and that the tagged commit is reachable from `main`; do not
  duplicate identical full builds for a same-SHA main-and-tag push.
- Run targeted checks for changed behavior. Do not run broad external suites
  unless the maintainer requests them.
- Release targets are maintainer-owned: do not commit, tag, push, or publish
  unless explicitly asked.
- Use Rust edition 2024 and directory modules when adding multi-file modules.
