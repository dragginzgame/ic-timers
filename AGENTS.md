# AGENTS.md

This file is normative for automated contributors.

## Shared baseline and local overlay

- Apply the [reviewed Shared Tooling baseline](DRAGGINZGAME.md)
  from revision `db039347d2372b877c1c46dcdd2b5c3aa9412009` (0.1.27). Its provenance and
  refresh boundary are recorded in [the adoption record](docs/shared-tooling.md).
  The remainder of this file is the IC Timers local overlay; a moving sibling
  checkout is not authority.
- Apply the [shared audit methods](audits/README.md) from revision
  `db039347d2372b877c1c46dcdd2b5c3aa9412009`, recorded separately in
  [.shared-tooling-audits.snapshot](.shared-tooling-audits.snapshot). That
  supplemental snapshot also supplies pinned host/IC setup at the same
  reviewed revision. The isolated
  [Cargo helper snapshot](.shared-tooling/helpers/.shared-tooling.snapshot)
  supplies the structured dependency checker, Cargo readers/rewrites, formatter prerequisite guard, annotated-tag checker and release-command
  adoption checker from `db039347d2372b877c1c46dcdd2b5c3aa9412009`. Apply the
  [dependency pinning rules](rules/dependency-pinning.md) with the exact local
  [qualification exceptions](docs/releasing.md#dependency-pin-exceptions).
  The [local hygiene overlay](docs/audits/code-hygiene.md) and adoption record scope
  their product obligations. This supplements the pinned baseline's review
  contract. Host and IC setup are explicit; verification is offline. The local
  audited PocketIC admission and automatic single-artifact provisioning contract
  remain separate from generic IC setup. The paired baseline and maintenance
  rule are adopted together; all three snapshots use the same reviewed revision.
  Common setup/check/LOC commands come from [make/tools.mk](make/tools.mk);
  Make, update-dev and CI select the complete pinned host bundle.
  The consumer-owned [IC pin matrix](ci/ic-tools.tsv) selects the product-selected
  PocketIC pair and is excluded from the immutable audit export. Shared installers
  still receive that one explicit matrix; the upstream default catalog cannot
  overwrite local artifact admission during a snapshot refresh.
  Optional helpers described in shared guides do not become local commands
  without a separate caller adoption.
- Maintainer-approved validation exception: tests, builds, lint gates and
  deployment validation are user-owned, including focused tests. Do not run
  them without an explicit request. Read-only inspection, script syntax, diff,
  documentation and cheap release-metadata checks remain allowed. This preserves
  the maintainer's deployment workflow rather than duplicating its validation.
- Maintainer-approved contribution exception: all commits remain user-owned,
  including contribution PR commits. The shared contribution rule does not
  supersede this established workflow. Prepare local edits and reviewable PR
  descriptions without committing on the maintainer's behalf.
- All release execution remains user-owned: version bumps, release commands,
  staging, commits, tags, pushes and publication. An ordinary request to prepare
  a release authorizes changelog preparation only. This is a maintainer-approved
  command-authority exception to the common baseline, preserving the established
  deployment workflow alongside the validation exception above.
- Standard Make release commands select direct delivery explicitly. The shared
  PR helper is included with the reviewed release contract, but PR release
  delivery and merged-source adapters require separate maintainer adoption.
- macOS host workflows are required by the shared baseline. The local
  [host matrix](docs/releasing.md#host-support) records current qualification
  gaps; Linux evidence does not qualify macOS. A baseline refresh alone does
  not close those gaps or authorize weakening the pinned PocketIC gate.
- The native `platform` substitute is test-only evidence, not simulated IC
  recovery. Production platform paths and PocketIC evidence retain their own
  contracts; never use a test configuration to change production guarantees.

## Session handoff

- Read `docs/status/current.md` first in a new session. Continue from that
  compact handoff instead of reconstructing repository state from chat
  history.

## Scope

- Make changes only in this repository unless the maintainer explicitly names
  another exact target and authorizes mutation there.
- The maintainer requested one root dependency catalog on 2026-10-07,
  superseding the former independent testing-workspace exception. `Cargo.toml`
  owns all four maintained packages and every direct dependency; all member
  dependency tables use `workspace = true`. There is one selected `Cargo.lock`.
  Retain the previously approved probe locations under `testing/crates/`;
  those directories no longer introduce a workspace or dependency catalog.
- Keep `ic-timers` as the default member and select it explicitly in library
  validation, docs, MSRV and Wasm commands. Probes remain unpublished, inherit
  the root package version, retain their Rust-only lint selection, and use the
  root `timer-probe` profile for their prior Wasm optimization settings. The
  complete release gate still qualifies probe lint, watchdog/recovery and
  policy cohorts. Release preparation updates every local member in the one
  lockfile; do not retain a second manifest, lock or compatibility path.

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

- Update the current `CHANGELOG.md` draft for every meaningful change without
  waiting for a separate changelog request. Supporting design and evidence
  belongs with its owner, not in a second release queue or local issue ledger.
- Distinguish crate-impacting work from repository-only documentation,
  evidence, CI, and release-tooling updates. Repository-only work normally
  remains untagged and is bundled into the next code-bearing release because
  exact-pinned consumers must coordinate every package identity. However, an
  explicit maintainer-owned version-bump or release target is sufficient
  authority to publish a repository-only patch: warn clearly and retain the
  complete user-operated release gate. Reject a subject with no changes. Do not
  manufacture crate impact merely to silence the advisory.
- Maintain one numbered, undated pending changelog section under the shared
  automatic next-version rules. Reuse it for the complete batch, honour a valid
  maintainer-selected target and keep Cargo version mutation user-owned.
- For a new release request, automated contributors prepare only the next
  changelog section. Keep it undated. Do not mutate Cargo versions
  or lockfiles, stage changes, or run version-bump or release targets.
  The user runs the matching bump target. The version helper labels the current
  draft and adds its date automatically without running tests. Changelog layout,
  empty notes, or a missing chosen version must not block deployment; ambiguous
  release selection is resolved during preparation. The user runs `release-verify`
  as part of deployment; it must retain the normal CI,
  MSRV, probe lint, watchdog PocketIC, and policy-cohort gates; it fails
  closed unless `POCKET_IC_BIN` matches the exact audited PocketIC version and
  hash. When no override is supplied, provision that pinned artifact in the
  ignored repository tool cache automatically; never weaken validation or
  overwrite an explicit override. After version mutation, update every local member in the root lockfile
  and verify the complete graph with a cheap locked metadata check. The user
  stages the root lockfile. Do not require the maintainer to edit a changelog heading
  by hand, prepare a test binary manually, or remember a separate evidence command.
- A pre-bump check may warn about free-form release prose that is likely to
  become stale, but it must remain advisory and run before version mutation.
  Never make interpreted prose a post-mutation release blocker.
- Run full hosted Rust and MSRV validation on pull requests and `main`. A tag
  push may use a smaller package/tag job only when it also verifies the exact
  version tag and that the tagged commit is reachable from `main`; do not
  duplicate identical full builds for a same-SHA main-and-tag push.
- Fresh combined release targets run their requested version bump. A normal
  target reconciles saved unfinished intent at its exact version first. After
  that release is committed, newer descendant fixes or a different requested
  increment trigger fresh validation for the requested next release. Late
  checks inspect `RELEASE_COMMIT`, independently of HEAD. Changelog preparation
  alone neither advances Cargo nor creates runtime release intent.
- Version-bump helpers must support preparing a release from the current
  worktree without requiring a preparatory commit. Preserve unrelated changes,
  leave test execution to deployment. The user stages release metadata.
- Use Rust edition 2024 and directory modules when adding multi-file modules.
