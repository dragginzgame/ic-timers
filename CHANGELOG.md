# Changelog

All notable changes to this project are recorded here.

## [0.16.1]

### Development

- Use the incoming root-catalog Testkit 0.27.0 selection for the private PocketIC
  harness and server CLI. Startup errors retain the original cause, bounded
  output and separate command/server cleanup failures. Timer APIs are unchanged;
  native acceptance remains pending
  ([qualification owner](docs/releasing.md#testkit-027-preparation)).

## [0.16.0] - 2026-10-09

### Breaking

- Adopt registry `ic-metrics 0.3` through the root dependency catalog. The public
  `MeasurementSummary` now has the 0.3 package identity; consumers exchanging
  summaries with a direct Metrics dependency must align it to 0.3 or use
  `ic_timers::MeasurementSummary`. Arithmetic source is unchanged from 0.2.20.
  No compatibility alias is retained
  ([adoption evidence](docs/design/callback-delivery-ownership.md#ic-metrics-03-adoption)).

## [0.15.0] - 2026-10-09

### Breaking

- Delegate PocketIC setup, artifact pins and offline admission to the root-lock
  selected Testkit CLI. Remove the local server downloader, client alignment and
  binary checkers, their tests, and the `POCKET_IC_BIN` Make override. Explicit
  `make install-testkit-server` prepares the owner CLI/server; `pocketic-check`
  checks it offline. Release preparation and CI select this route while retaining
  watchdog/recovery and policy-cohort gates
  ([shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)).
- Adopt committed Shared Tooling 0.2.1 through all three snapshots and remove
  PocketIC from the shared IC bundle. Run explicit `make install-ic-tools` to
  replace the old six-tool selection; old bundles and evidence remain retained.
  Timer APIs are unchanged.

### Development

- Remove obsolete split-workspace fixture cleanup and single-manifest loops.
  Keep full root-graph resolution, failed-Cargo propagation and lock preservation.
- Retain incoming Testkit 0.26.0, Host 0.9.0 and Metrics 0.2.20 selections through
  the one root catalog/lock. The server adapter follows the same Testkit selection.
- Process every IC pin row during setup/check, including a final row without
  a newline ([shared #87](https://github.com/dragginzgame/shared-tooling/issues/87)).
- Adopt the shared single-document dependency-exception fix and checkout-local
  hook executable lookup ([shared #86](https://github.com/dragginzgame/shared-tooling/issues/86),
  [shared #85](https://github.com/dragginzgame/shared-tooling/issues/85)).
- Retain failed Testkit provisioning attempts and selected CLI build evidence
  without archiving admitted server bundles.
- Report compressed failure-archive bytes and archive creation time in collector
  logs, so hosted compact-evidence qualification can measure the actual output.
  Failed archives do not report completed measurements
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30)).

## [0.14.23] - 2026-10-09

### Development

- Run fleet tooling inventories centrally in Shared Tooling, removing the unused
  reporter and its regression suite from this consumer. Keep local workspace LOC
  and pinned tool setup/check commands
  ([#33](https://github.com/dragginzgame/ic-timers/issues/33),
  [shared #83](https://github.com/dragginzgame/shared-tooling/issues/83)).
- Adopt the committed Shared Tooling PocketIC path fix for inherited `CDPATH`,
  unusual directory names and directories that disappear before Cargo starts
  ([shared #82](https://github.com/dragginzgame/shared-tooling/issues/82)).

## [0.14.22] - 2026-10-09

### Development

- Keep failure archives compact by rechecking active host/IC tool bundles with
  the consumer's pins. Verified bundles retain check logs, pins and IC receipts;
  failed, changed and unselected bundles retain their full payloads. Preserve
  original job identity, failure status, fixture bytes and validation logs
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #66](https://github.com/dragginzgame/shared-tooling/issues/66)).
- Reuse verified IC tool bundles across comment-only and reordered pin catalogs,
  preserving original installation receipts. Keep compact download verification
  aligned with the shared pin admission and retain native CI for every pushed
  commit while cancelling superseded PR runs
  ([shared #79](https://github.com/dragginzgame/shared-tooling/issues/79),
  [shared #80](https://github.com/dragginzgame/shared-tooling/issues/80)).
- Pass explicit absolute manifest/pin paths to PocketIC alignment so inherited
  `CDPATH` cannot redirect its directory observation
  ([shared #82](https://github.com/dragginzgame/shared-tooling/issues/82)).
- Update the release-gate fixture to require those absolute paths as separate
  arguments, using a physical workspace with spaces and a symlink entry point.
  Report unexpected arguments rather than failing silently.
- Retain incoming lock selections for ic-host 0.8.5, ic-metrics 0.2.16, TOML 1.1.8
  and its parser 1.1.5 through the existing root dependency catalog and Testkit.

## [0.14.21] - 2026-10-08

### Development

- Use reviewed Shared Tooling 0.1.29 to report every staged, unstaged and
  untracked path that prevents release, distinguishing failed Git observations
  from dirty source. Replace the private checker while preserving metadata
  allowances; initial preflight failures explain that validation and version
  preparation have not started for that attempt
  ([#32](https://github.com/dragginzgame/ic-timers/issues/32),
  [shared #74](https://github.com/dragginzgame/shared-tooling/issues/74)).
- Retain the incoming ic-metrics 0.2.15 lock selection through the root catalog.

## [0.14.20] - 2026-10-08

### Development

- Refresh Shared Tooling to reviewed 0.1.28: reject malformed active tool links
  before execution or downloads and keep consumer release-runner fixtures
  simulation-only. Include the maintenance catalog without activating scheduled
  work; preserve the consumer-owned PocketIC pins and complete release gate
  ([#31](https://github.com/dragginzgame/ic-timers/issues/31),
  [shared #75](https://github.com/dragginzgame/shared-tooling/issues/75),
  [shared #70](https://github.com/dragginzgame/shared-tooling/issues/70)).
- Retain the incoming Testkit 0.25.2 lock selection through the root catalog;
  the maintained PocketIC adapter and timer public APIs are unchanged.

## [0.14.19] - 2026-10-08

### Development

- Refresh the reviewed Shared Tooling 0.1.27 follow-up for literal consumer/pin
  paths and explicit installer-fixture dependencies. Keep full evidence retention
  while native compact-policy acceptance remains pending
  ([shared #66](https://github.com/dragginzgame/shared-tooling/issues/66),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67)).
- Align the retained PocketIC 16.1.0 client with independently reviewed server
  archives/binary digests for Linux and both macOS architectures. Keep the strict
  gate, automatic provisioning and override refusal; transfer the existing IC pin
  matrix to consumer ownership so snapshot refreshes preserve the selected pair.
  Prior 16.0.0 runtime evidence remains scoped to its original pair.
- Record completed native late-failure transport and Intel early-failure evidence
  at frozen 0.14.17, retaining the separate source and acceptance boundaries
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30)).

## [0.14.18] - 2026-10-08

### Development

- Preserve evidence collection and qualification paths under inherited `CDPATH`
  and newline-ending checkout or temporary directories. Keep workspace identity,
  original failure status and archive retention checks
  ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67)).
- Refresh Shared Tooling to reviewed 0.1.27 for shell path bootstrap, complete
  host/IC evidence fixtures and dotted snapshot LOC reporting. Exercise the real
  installer and logger in the evidence path regression; full archive retention
  remains selected while compact-policy qualification is pending
  ([shared #66](https://github.com/dragginzgame/shared-tooling/issues/66),
  [shared #67](https://github.com/dragginzgame/shared-tooling/issues/67),
  [shared #69](https://github.com/dragginzgame/shared-tooling/issues/69)).
- Select Testkit 0.24 through the existing root dependency catalog; preserve the
  incoming Host 0.7.1 and Metrics 0.2.13 lock selections. The maintained PocketIC
  harness and timer public APIs are unchanged.

## [0.14.17] - 2026-10-08

### Development

- Select Testkit 0.23 and IC Host 0.6 through the root catalog, gaining startup
  cleanup diagnostics and wrapper-process cleanup without changing timer APIs
  ([Testkit #30](https://github.com/dragginzgame/ic-testkit/issues/30),
  [Host #5](https://github.com/dragginzgame/ic-host-tooling/issues/5)).
- Refresh Shared Tooling to reviewed 0.1.26, including safe local tracking updates
  after release and expanded tooling reports
  ([shared #62](https://github.com/dragginzgame/shared-tooling/issues/62),
  [shared #61](https://github.com/dragginzgame/shared-tooling/issues/61)).
- Delegate failure archiving to the shared helper; preserve partial archives and
  reject overwrites, while retaining source/status admission and all selected
  installer evidence ([#30](https://github.com/dragginzgame/ic-timers/issues/30),
  [shared #59](https://github.com/dragginzgame/shared-tooling/issues/59)).

## [0.14.16] - 2026-10-08

### Development

- Select Testkit 0.22 and IC Host 0.5.2 through the root dependency catalog.
  The existing PocketIC harness now uses the shared child-process owner;
  timer APIs, runtime behavior and the audited PocketIC version are unchanged
  ([upstream #25](https://github.com/dragginzgame/ic-testkit/issues/25)).
- Record the consumer audit of Shared Tooling and IC Host, including measured
  failure-artifact overhead and archive-adapter requirements. Implementations
  and reviewed snapshots remain unchanged
  ([upstream #66](https://github.com/dragginzgame/shared-tooling/issues/66)).

## [0.14.15] - 2026-10-08

### Development

- Preserve the incoming compatible Metrics 0.2.11 root lock selection. Its
  changes affect upstream release tooling and evidence; the metrics library
  implementation and timer APIs are unchanged.

## [0.14.14] - 2026-10-08

### Development

- Adopt Shared Tooling 0.1.23: recheck payload and exact tag identity after final
  release hooks, and verify published refs on completed release resume. Keep
  direct delivery explicit and preserve maintainer-owned commands
  ([#29](https://github.com/dragginzgame/ic-timers/issues/29),
  [upstream #58](https://github.com/dragginzgame/shared-tooling/issues/58)).
- Refresh the test-only root lock selection to compatible Testkit 0.21.3 and
  TOML dependency patches. Keep the existing root dependency catalog and
  PocketIC harness; timer behavior and public APIs are unchanged.
- Preserve the incoming Metrics 0.2.10 lock selection; its changes affect
  upstream tooling and documentation, with unchanged library implementation.

## [0.14.13] - 2026-10-07

### Development

- Add manual early/late failure qualification to the existing Linux and native
  macOS CI jobs, using the maintained installer/logger and normal artifact
  collector. Download and check each host's source identity, controlled failure,
  retained bytes and file modes. Retain original failed-job status; keep normal
  PR/main gates and release commands unchanged
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).
- Select compatible IC Host 0.4.6 through Testkit in the root lockfile, including
  the [macOS compilation repair](https://github.com/dragginzgame/ic-host-tooling/issues/18);
  retain the [native filename qualification](https://github.com/dragginzgame/ic-host-tooling/issues/19)
  and keep Host outside the timer library graph. Preserve the incoming Metrics 0.2.9
  selection, which changes upstream tooling/evidence rather than library behavior.
- Select the compatible Testkit 0.21.2 documentation and host-qualification patch
  through the existing root dependency catalog.

## [0.14.12] - 2026-10-07

### Development

- Adopt committed Shared Tooling 0.1.19, including Rust-tool path admission
  repairs and complete failed-batch logs
  ([#28](https://github.com/dragginzgame/ic-timers/issues/28)).
- Replace the private changelog selector with the shared owner; retain local
  check-only admission, mode preservation and atomic replacement. Cover failed
  readers and producers without rewriting original notes
  ([#24](https://github.com/dragginzgame/ic-timers/issues/24)).
- Read PocketIC archive pins from the shared matrix and reuse canonical checksum
  and binary verification. Check locked client/server alignment before admission
  or provisioning; preserve automatic single-artifact installation and strict
  explicit overrides ([#25](https://github.com/dragginzgame/ic-timers/issues/25)).
- Select the compatible Testkit 0.21.1 host-qualification patch in the sole
  root lockfile; keep all member dependency declarations inherited from the
  root catalog.

## [0.14.11] - 2026-10-07

### Development

- Add byte-exact changelog finalization coverage for historical notes with no
  terminal newline or retained trailing whitespace, including imported undated
  history and releases with no pending notes. Prepare the consumer contract for
  shared-selector adoption ([#24](https://github.com/dragginzgame/ic-timers/issues/24)).

## [0.14.10] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.18: reject Make options and assignments passed as
  validation targets, preserve optional complete logs and timing summaries,
  and fix LOC fixture isolation and build-output exclusion. Keep the single
  root dependency catalog and complete audited PocketIC release gate
  ([Shared Tooling #30](https://github.com/dragginzgame/shared-tooling/issues/30),
  [#37](https://github.com/dragginzgame/shared-tooling/issues/37),
  [#31](https://github.com/dragginzgame/shared-tooling/issues/31),
  [#47](https://github.com/dragginzgame/shared-tooling/issues/47),
  [#50](https://github.com/dragginzgame/shared-tooling/issues/50),
  [#53](https://github.com/dragginzgame/shared-tooling/issues/53)).
- Refresh shared audit and setup guidance, including the canister application
  addendum. Timer behavior and dependency selections are unchanged.
- Keep `make build` scoped to the library after workspace consolidation;
  probe cohorts retain separate builds for their mutually exclusive features.

## [0.14.9] - 2026-10-07

### Development

- Consolidate library and unpublished probe dependencies in the root Cargo
  catalog and lockfile; every member inherits dependencies with `workspace = true`.
  Retire the independent testing manifest/lock and update release, formatting
  and fixture owners for the single graph. Keep library-only default builds and
  preserve probe optimization settings in the `timer-probe` profile.
  Reject stale library or probe identities in the shared lockfile.
- Resolve the current Testkit 0.20 selection and ic-metrics 0.2.7 together.
  Preserve the audited PocketIC 16.0.0 gate and the complete probe qualification.

## [0.14.8] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.15 and its common Make setup/check/LOC commands.
  Prepare and verify pinned jq, yq, ripgrep with PCRE2 and cloc together;
  CI uses that bundle instead of separately installing system ripgrep.
  Consolidate host/IC setup under the root snapshot, remove copied Make
  recipes and preserve the audited PocketIC release gate. This is
  repository-only work; timer behavior and dependency selections are unchanged.
- Remove the retired nested installer directory from shell syntax checks;
  repository fixtures keep it absent so a stale wildcard cannot pass unnoticed.
- Record and substitute host-tool admission in release-gate fixtures, checking
  that its failure stops later checks and retaining per-scenario Make output.
- Retain failed committed-release, staging/index, lockfile and repository-check
  fixtures for CI artifact collection instead of deleting their diagnostic
  inputs. Preserve original failure status and clean up successful runs
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).

## [0.14.7] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.14: reject Make modes that skip execution or mask
  failures before release, validation and pre-commit formatting. Protect the
  local exact-version release path and cover isolated fixture exports
  ([#20](https://github.com/dragginzgame/ic-timers/issues/20)).
  Check that each failed exact-release phase stops the sequence, and retain
  failed hook/version fixtures with scenario logs for diagnosis.
  This is repository-only work; timer behavior and dependency selections are unchanged.
- Upload failed CI fixture, validation and installer evidence after every job's
  checks, preserving file modes and source/host identity
  ([#23](https://github.com/dragginzgame/ic-timers/issues/23)).

## [0.14.6] - 2026-10-07

### Development

- Adopt Shared Tooling 0.1.13 and place the three testing packages under their
  independent workspace's `crates/` directory, preserving package identities,
  both lockfiles and the complete release gate
  ([#19](https://github.com/dragginzgame/ic-timers/issues/19)).
  This is repository-only work; timer behavior and dependency selections are unchanged
  by the layout adoption.

## [0.14.5] - 2026-10-06

### Development

- Replace the copied standard-release smoke fixture with the reviewed shared
  checker, retaining local metadata, lockfile and recovery coverage
  ([#17](https://github.com/dragginzgame/ic-timers/issues/17)).
- Refresh Shared Tooling to 0.1.12: bind standard release pushes to the recorded
  destination URL and verify snapshots without executing inspected checksum helpers. Add
  isolated corruption checks for all three consumer snapshots
  ([#18](https://github.com/dragginzgame/ic-timers/issues/18)).
  This batch is repository-only; timer behavior and dependency selections are unchanged.

## [0.14.4] - 2026-10-06

### Development

- Include external `echo` in the release-gate fixture's restricted tool PATH,
  so Make's direct execution of logging recipes reaches the intended assertions.
  This repair is repository-only.
- Refresh the reviewed Shared Tooling snapshots to 0.1.11. Passing `error::`
  test names remain ordinary validation output, while actual diagnostics retain
  highlighting and failure logs. Adopt the paired maintenance policy and stricter
  host-tool version admission; validation and release execution remain user-owned.
  This is repository-only work.
- Delegate annotated release-tag validation to the reviewed shared checker,
  preserving HEAD checks for publication and saved-commit checks for release
  recovery ([#16](https://github.com/dragginzgame/ic-timers/issues/16)).
  This is repository-only work; timer behavior and dependency selections are
  unchanged.

## [0.14.3] - 2026-10-06

### Development

- Finalize release notes using the bump's known previous version. Preserve older
  undated sections as history, retaining strict rejection of competing pending notes
  and already dated targets. Compare version components exactly, retain complete
  candidate admission, and preserve file ownership and rollback. This is
  repository-only work.
- Check the reviewed cargo-sort version and prepared rustfmt before `fmt`,
  `fmt-check` and hook qualification. Reject failed version commands even when
  they print the expected version, and keep preparation explicit and offline
  checks free of tool installation. This is repository-only work; timer behavior
  and dependency selections are unchanged.

## [0.14.2] - 2026-10-06

### Fixed

- Prepare ripgrep before regression fixtures in Linux, MSRV, native macOS and
  tag CI jobs. This fixes 0.14.1's missing-tool failures while retaining the
  shared structured checker and complete release gate
  ([#13](https://github.com/dragginzgame/ic-timers/issues/13)).
  This repair is repository-only.

### Development

- Use the shared exact local-package lock rewrite for both release lockfiles,
  preserving external identities and checking the complete candidate before
  replacement ([#14](https://github.com/dragginzgame/ic-timers/issues/14)).
- Enforce Cargo inheritance against each independent workspace root and use
  the shared structured workspace-version reader. Version mutation and release
  recovery remain locally owned ([#15](https://github.com/dragginzgame/ic-timers/issues/15)).
- Add explicit `install-ic-tools` / offline `ic-tools-check` and combined
  `install-tools` / `tools-check` setup for the pinned six-tool IC bundle.
  Development setup and CI prepare it before validation; native macOS evidence
  still admits PocketIC by its independently audited binary hash
  ([#12](https://github.com/dragginzgame/ic-timers/issues/12)).
  This batch is repository-only; runtime and dependency selections are unchanged.

## [0.14.1] - 2026-10-06

### Development

- Replace line-based Actions validation with the reviewed shared YAML/TOML
  checker. Quoted and folded SHA references are accepted; moving references,
  unpinned Docker actions and malformed declarations are rejected. Preserve
  both workspace lockfiles and the existing qualified IC dependency pins
  ([#13](https://github.com/dragginzgame/ic-timers/issues/13)).
- Add explicit, checksum-verified jq/yq setup with `make install-host-tools`
  and offline verification with `make host-tools-check`. Explicit `update-dev`
  and CI prepare the parsers
  before validation; ordinary checks never download them. This is the parser
  prerequisite of [#12](https://github.com/dragginzgame/ic-timers/issues/12),
  without changing PocketIC provisioning or installing the broader IC toolset.
  This batch is repository-only.

## [0.14.0] - 2026-10-06

### Breaking

- Use published `ic-metrics 0.2` for measurement arithmetic. The publicly exposed
  `MeasurementSummary` now belongs to the 0.2 package identity; consumers that
  exchange it with a direct ic-metrics dependency must update that dependency to
  0.2 or use IC Timers' re-export. Summary values and saturation are unchanged
  ([ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10)).

### Changed

- Read instructions through the existing `ic0::performance_counter(1)` platform
  adapter and remove ic-metrics' retired `ic` feature. Both independent workspace
  locks select registry 0.2.0, preserving scheduler/work attribution,
  registration identity and native test fakes
  ([ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10)).

### Development

- Adopt the committed shared audit methods with an IC Timers overlay, preserving
  runtime safety obligations and historical reports while removing automatic
  repair and broad-validation instructions. Verify the audit snapshot alongside
  the existing tooling baseline ([#11](https://github.com/dragginzgame/ic-timers/issues/11)).
  This audit-tooling slice is repository-only.

## [0.13.5] - 2026-10-06

### Fixed

- Stop committed release metadata checks at the first failure on Bash 3.2,
  preserving manifest ordering and locked-resolution rejection on macOS.
- Exercise release-index checks in a controlled shallow checkout with its own
  baseline tag, removing their dependence on hosted checkout history. This
  addresses the hosted qualification failures in the release-tooling repair for
  [#10](https://github.com/dragginzgame/ic-timers/issues/10). These release-check
  repairs leave timer behavior unchanged.

### Changed

- Align both workspace locks with the published `ic-metrics 0.1.6` requirement,
  preserving all other dependency selections and allowing locked release
  preflight to resolve the testing workspace.

## [0.13.4] - 2026-10-06

### Fixed

- Recover an interrupted committed release through the normal release commands
  after newer fixes are committed. Check the saved release's metadata and tag at
  its exact commit, then run fresh validation for the requested next increment.
  This addresses [#10](https://github.com/dragginzgame/ic-timers/issues/10) using
  the reviewed Shared Tooling recovery fix. This is repository-only release
  tooling; package publication and cleanup remain separate.
- Reject staged implementation edits hidden by a restored working file, require
  the release index to match prepared metadata, and retain complete validation
  failure logs across retries through the shared validation runner.
- Prepare both locked dependency caches during release preflight through the
  existing fetch target, allowing cold-cache releases to reach validation without
  a separate manual fetch. Fetch and Git admission failures stop before mutation.

### Testing

- Report exact loaded Wasm byte sizes alongside policy-cohort instruction costs
  in the existing qualification gate, enabling baseline and release comparisons
  without separate builds or measurement commands.

## [0.13.3] - 2026-10-06

### Fixed

- Release removed and rejected callback captures after registry access ends,
  with removed timers' provider cleanup completed before capture destruction.
  This permits normal cancellation and unregistration destructors to inspect
  the registry without receiving an internal borrow error.

### Changed

- Leave bound provider handles in place for rejected and coalesced control
  requests, avoiding temporary identity copies and handle reinstallation.
  Scheduling, cancellation arbitration and Watchdog prearming are unchanged.
- Add native and PocketIC capture-release coverage and reconcile current
  qualification documentation with the successful 0.13.2 hosted gates.

## [0.13.2] - 2026-10-06

### Fixed

- Correct release-gate fixture comparisons when the workspace is reached through
  a directory symlink, including macOS's `/var` temporary paths. Exercise the
  alias on every host while retaining exact default and override PocketIC
  selection checks. Production provisioning and the complete release gate are
  unchanged; this is repository-only test tooling.

## [0.13.1] - 2026-10-05

### Changed

- Use published `ic-metrics 0.1.5` for the Wasm call-context instruction reader
  and shared summary, retaining callback attribution, registration identity and
  consumer-owned native/test behavior
  ([#9](https://github.com/dragginzgame/ic-timers/issues/9)).

### Development

- Align the handoff, callback contract and Shared Tooling adoption notes with
  the tagged 0.13.0 release and published registry `ic-metrics 0.1.3`. Separate
  release identity from recorded validation and remove obsolete pending-release
  and sibling-dependency instructions. This is repository-only documentation;
  runtime behavior and the public API are unchanged.

## [0.13.0] - 2026-10-05

### Changed

- Re-export `ic_metrics::MeasurementSummary` as the canonical summary while retaining
  the existing snapshot accessors, callback attribution and registration identity.
  Select published registry `ic-metrics 0.1.3` in both lockfiles, removing the
  sibling-checkout requirement ([#9](https://github.com/dragginzgame/ic-timers/issues/9)).
- Standardize the three SemVer release commands on the reviewed Shared Tooling
  runner, including automatic recovery when the same target is rerun and an atomic push of the selected branch
  and tag. Release preparation uses UTC dates and preserves both locked dependency selections
  and build artifacts.

### Fixed

- Keep the ordinary delivery guard captured until callback completion, preserving
  discard retirement and normal completion handling. Drop queued and suspended
  native fixture work before thread-local teardown, with the runtime still
  available and the task-map borrow released, preventing teardown aborts.

### Breaking

- Retire confirmed ordinary deliveries dropped without normal completion instead
  of retaining false Scheduled or Running authority. Add `InactiveReason::Abandoned`
  and project it as Failed; record Unacknowledged without fabricating a completion
  or measurement. Retained claims can explicitly rearm; transient declarations
  and pending unregistrations release their entries. Live futures still complete
  normally before pending control is applied; no retry or interruption is added.
  Consumers must handle the new `InactiveReason` variant and treat ordinary
  Unacknowledged observations as abandoned deliveries, without inferring
  Watchdog recovery or application rollback.

### Development

- Show the repository name first in VS Code window titles, followed by the active filename.
- Restart failed preflight/validation through the normal release target, and
  automatically reconcile prepared attempts at their saved version before any
  new increment. Remove the local retry wrapper in favour of the shared owner.
- Adopt the shared pre-commit hook to format and refresh only fully staged
  selected files, preserving unrelated edits and rejecting partial staging.
  Pin `cargo-sort` 2.1.4 in developer/CI setup and include manifest sorting for
  both workspaces in formatting and prepared-release checks.
- Use exact `ic-testkit` 0.17.3 for the host-side real-canister suites, with
  bounded startup and caller-owned servers for fresh IC instances. Update the
  strict PocketIC artifact gate to 16.0.0 on Linux and both macOS architectures;
  preserve override rejection and all timer assertions. Existing PocketIC 15
  receipts remain historical; the new harness requires renewed qualification.
- Repair isolated version-preparation checks after shared-runner adoption: include
  the shared version calculator, verify all three increments and current Makefile
  delegation, and keep failure injection active through missing-changelog rollback.
  Retain local exact-version rollback and staging coverage. Run
  the shared runner's phase/failure/resume fixtures in the existing release gate.
- Add native and PocketIC fixtures covering discarded ordinary deliveries,
  traps before and after an await, pending commands, stale authority and capture
  release for both ordinary policies and lifetimes. Keep native assertions and
  identity ownership compatible with warning-denied Clippy. Qualification remains pending;
  see the [callback contract](docs/design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
- Reconcile current README, safety, callback-contract, release and handoff
  documentation with the completed 0.12 minor bump. Remove pending-bump
  instructions and describe the policy-specific return API as current, keeping
  automated preparation evidence separate from maintainer validation. This is
  repository-only documentation work; no crate behavior or package identity
  changes.
- Clarify in the public result rustdoc and README that success, no work and
  retryable failure preserve the supplied scheduling decision rather than
  choosing recurrence or retry automatically. Explain invariant-failure Stop
  normalization and checked relative-delay validation after authoritative
  command arbitration. Runtime behavior is unchanged.

## [0.12.0] - 2026-10-05

### Changed

- Hard-cut ordinary callback returns to `OnceRunResult` / `OnceDecision` and
  `AfterCompletionRunResult` / `AfterCompletionDecision`. Update both registration
  and lifecycle reconciliation entry points; Once keeps explicit continuation,
  retry and absolute deadlines but cannot request configured recurrence.
- Remove the shared public `TimerRunResult` and `TimerDirective` surface without
  aliases or conversions between policy result types. Retain one private erased
  result and scheduling owner for registry arbitration, checked overflow handling,
  invariant-failure stopping and inert snapshot projection. Keep Watchdog's
  callback contract and snapshot meanings unchanged.
- Update current examples, native fixtures and both probe canisters to typed
  results. Project either public decision into `TimerDirectiveSnapshot` through
  the existing checked conversion owner; retain no reverse conversion.

### Development

- Use unqualified `pub` for the internal ordinary callback, directive and erased
  result declarations inside private root modules, addressing the reported
  `redundant_pub_crate` lint while retaining their absence from the public API.
- Add positive and compile-fail API doctests covering all four ordinary entry
  points, crossed decisions/results and unavailable Once recurrence. Run doctests
  in the existing test and MSRV targets, and extend recording-Cargo gate fixtures
  to reject failures at either recipe command. Extend native runtime coverage
  across legal scheduling choices, overflow and authoritative reconciliation;
  keep private erased-policy corruption coverage. Execution remains pending
  maintainer validation, including the full PocketIC and policy-cohort gate.
- Cover callback-borrow rejection for both typed ordinary policies and both
  declaration lifetimes, including retained failure observations, transient
  removal and absent work measurements. Add live native recurrence cases for
  success, no work, retryable failure and invariant failure; correct the README
  to distinguish a normal return requesting recurrence from success-only
  recurrence. These fixture additions remain unexecuted.

## [0.11.10] - 2026-10-05

### Development

- Check unstaged implementation and untracked paths before combined release
  commands run validation or bump the version. Reuse the release-commit guard
  with NUL-delimited Git records, reject failed queries even after partial
  metadata-only output, and report Bash-escaped paths. Allow the five metadata
  outputs owned by the bump and release staging; preserve dirty-worktree support
  for standalone version preparation and the complete user-operated release gate.
  Extend worktree-admission and orchestration fixtures without staging files
  automatically.
- Reject nonregular PocketIC candidates and invalid automatic-install cache
  destinations before downloading. Preserve directories, FIFOs and rejected
  symlinks; continue accepting verified executable file symlinks without replacing
  them. Extend cache-type and link-preservation fixtures while retaining exact
  host-specific artifact pins.
- Require exactly one version argument after the bump helper's optional leading
  `--check`. Reject extra arguments and misplaced or repeated check flags before
  reading or changing release metadata; extend metadata-preservation fixtures.

## [0.11.9] - 2026-10-05

### Development

- Validate the tagged checkout's root and testing lockfiles in hosted tag CI
  after confirming the exact annotated tag and main reachability. Prepare the
  selected dependency caches before offline metadata checks; retain fixture
  validation without repeating the full Rust and MSRV builds owned by PR/main.
- Classify release impact from NUL-delimited Git paths so filenames containing
  tabs, line breaks, quotes or non-ASCII bytes cannot hide crate-source changes.
  Remove display-path joining and sorting; require both Git queries to succeed
  before classifying their records. Extend the existing fixture across untracked
  and staged paths, partial query failures and temporary-record cleanup.
- Verify the pinned PocketIC archive digest before decompression, then retain
  hash-before-execution and exact version checks for downloaded, cached and
  overridden binaries. Pin separate PocketIC 15.0.0 artifacts for Linux x86_64,
  macOS Intel and Apple Silicon; use core Perl SHA-256 support on both hosts.
  Extend failure-order, host-selection and cache-preservation fixtures.
- Declare macOS 15 Intel and Apple Silicon host targets and add native PR/main
  jobs that run the complete release gate under Apple's Bash 3.2 with both
  pinned Rust toolchains. Replace empty-array fixture argument expansion with
  positional arguments for Bash 3.2. Native qualification remains pending these
  jobs; tag pushes continue to avoid duplicate full validation.

## [0.11.8] - 2026-10-05

### Development

- Replace checksum manifests and separate mode tables in preservation fixtures
  with copies of the actual files. Compare bytes directly and preserve permission
  assertions for preflight rejection, rollback, interrupted preparation and tag
  rejection; retain independent Git index preservation checks.
- Compare preparation phases and staged paths as ordered records, removing joined
  arrays. Capture Git output directly so command failure cannot become an empty
  staging or tag result.
- Replace fixture-only GNU `sed -i` mutations with Perl, removing that host-specific
  dependency. PocketIC's audited binary hash verification remains unchanged;
  native macOS qualification is still pending.
- Stop clean-worktree and release-commit guards when Git queries fail. Distinguish
  a staged diff from a failed staged-diff query, and reject failed release-subject
  reads even when they emit a matching subject. Extend rejection fixtures across
  staged preparation and clean release retries without changing retry identity.
- Compare PocketIC fixture verification events as exact ordered records for
  accepted binaries and rejected overrides/downloads. Capture debris searches
  before asserting cleanup so a failed search cannot appear empty.
- Capture exact release-tag listings before checking absence during version
  preparation, release commits and interrupted tag retries. Reject failed queries
  even with plausible output; retain the prepared commit when its post-commit
  lookup fails and resume tagging that same commit on retry.
- Reject failed workflow discovery and provider-source searches even when they
  produce apparently valid records. Preserve nested workflow paths and temporary
  file cleanup, and extend the repository fixture across discovery, search and
  ordering failures with empty or matching output.
- Fetch the selected root and testing lockfiles' dependencies before the
  user-operated release gate, including target-specific sources needed by offline
  metadata checks. Add `make fetch` for cache preparation without building or
  changing versions. Preserve Cargo metadata failures before JSON parsing so a
  missing archive does not produce a second, misleading parse error.

## [0.11.7] - 2026-10-05

### Development

- Compare gate-fixture records directly instead of converting them through Bash
  4's `mapfile` and joined arrays. Preserve exact check order, duplicate PocketIC
  prerequisites, empty overrides and failure-stop verification without that
  newer-shell dependency. Check the full repository-gate prefix on failure rather
  than only its last recorded check.
- Refresh the reviewed Shared Tooling documentation baseline to `ca319ba`, using
  committed source bytes and `DRAGGINZGAME.md` as the shared entry point. Retain
  local contributor instructions and explicit maintainer ownership of validation
  and release execution.
- Adopt exact-symbol cleanup reporting and record required macOS host workflows
  and qualification gaps without changing the pinned PocketIC release gate.
- Keep successful changelog-finalization fixtures quiet while showing child
  diagnostics on unexpected failure. Preserve empty-note warnings during actual
  release preparation.

## [0.11.6] - 2026-10-05

### Changed

- Decide terminal declaration removal from the entry already selected by the
  ordinary request, Watchdog request or Watchdog scheduler transition. Remove
  the separate registry lookup while preserving lifetime rules, typed failures,
  accounting and provider cleanup ordering.

### Development

- Extend the Watchdog immediate-request coalescing fixture across both declaration
  lifetimes, checking that successful initial, replacement and duplicate requests
  retain transient declarations.
- Record terminal-removal ownership and pending verification in the
  [architecture reference](docs/architecture.md#terminal-removal-verification).

## [0.11.5] - 2026-10-05

### Changed

- Let ordinary and Watchdog work delivery consume the matching fired handle and
  accept work through the same registry entry lookup. Remove the separate runtime
  consumption pass while retaining exact claim, role, generation and state checks.
- Combine Watchdog scheduler handle consumption and detachment in one registry
  operation before its transition. Retain empty detachment for removed or
  superseded claims and cleanup before terminal declaration removal.

### Development

- Adopt the reviewed Shared Tooling engineering baseline with an explicit local
  overlay preserving maintainer ownership of tests and release execution.
- Keep one versionless changelog draft until a release is selected. Let the
  bump helper label and date it without requiring a separate versioned note or
  handoff status marker. Remove changelog presentation from deployment gates,
  retaining package, lockfile, tag and PocketIC validation.
- Preserve metadata contents, modes and prior file absence during failed bumps;
  reject symlinked or non-file outputs before mutation.
- Stage only the five release metadata outputs, leaving member manifests and
  unrelated documentation under the maintainer's separate staging ownership.
  Consolidate repeated contributor testing and release-ownership instructions.
- Extend provider-role fixtures with mismatched claims as well as generations,
  and deliver the stale-identity callback through the native mock queue. Check that
  it records stale delivery without consuming the replacement's handle or work.
- Check that suspended after-completion work owns no provider wakeup.
- Record scope and pending verification in the
  [callback delivery evidence](docs/design/callback-delivery-ownership.md) and
  [Shared Tooling adoption record](docs/shared-tooling.md).

## [0.11.4] - 2026-10-04

### Changed

- Record Watchdog request coalescing once after the selected transition, removing
  duplicate scheduled, dispatched and running updates. Terminal failures remain
  distinct from successful requests satisfied without another arm.
- Commit ordinary request scheduling mode once for successful arms or exact
  reconciliation, preserving mode changes when an exact deadline coalesces and
  mode retention for coalesced ensures.

### Development

- Extend existing fixtures to check dispatched and running request counts,
  non-coalesced terminal failures and exact ordinary mode observations at an
  unchanged deadline.
- Record scope and pending verification in the
  [0.11.4 release note](docs/changelog/0.11.4.md).

### Documentation

- Restore the README's single structured API-line projection after the overview
  rewrite, allowing version-bump and release-truth checks to maintain it.

## [0.11.3] - 2026-10-04

### Changed

- Consolidate ordinary completion's directive and generation failures into one
  terminal finalization branch and one declaration-removal exit. Preserve command
  precedence, validation order, typed failures and completion accounting.
- Give Watchdog initial and replacement requests one scheduling-mode update after
  successful arming. Preserve observations for coalesced, pending and failed
  requests, and calculate cadence deadlines only when required.
- Remove the private callback-context forwarding wrapper. Policy-specific public
  contexts carry their exact work token directly and use the existing shared
  authorization and provider-transition path.

### Development

- Extend the ordinary terminal-failure matrix across both policies and lifetimes
  to cover oversized retry delays and deadline overflow at exhausted generations.
  Check that retained failures record a stopped directive and invariant completion.
- Extend existing Watchdog fixtures to check mode retention for equal deadlines,
  pending commands and exhausted replacements, including cadence coalescing at
  maximum time without unnecessary deadline calculation.
- Record scope and pending verification in the
  [0.11.3 release note](docs/changelog/0.11.3.md).

### Documentation

- Add the shared helper navigation menu linking the eight maintained helper
  repositories through centrally hosted icons.
- Remove stale merge-conflict text from the README and align its technical
  overview with the `0.11` API line.

## [0.11.2] - 2026-10-04

### Changed

- Return ordinary and Watchdog work callbacks directly from the registry's
  acceptance transition. Remove the separate callback lookups, acceptance-status
  enum and second dispatch borrow while retaining exact claim, role, generation
  and state authorization.
- Fold the remaining context-only read lookup into context validation, retaining
  the shared running-work predicate and independent completion authorization.
- Decide cancellation lifetime removal once from the final state for both
  ordinary and Watchdog declarations. Remove the ordinary pre-state copy and
  fold its remaining generation-allocation helper into checked arming.

### Development

- Update acceptance fixtures to assert callback availability and reject duplicate
  acceptance across all three policies. Retain delegated-context and completion
  rejection coverage; remove assertions for the deleted lookup paths.
- Extend fresh cancellation coverage across both lifetimes, checking unchanged
  retained inventories and expiration of transient claims for every policy.
- Record scope and pending verification in the
  [0.11.2 release note](docs/changelog/0.11.2.md).

## [0.11.1] - 2026-10-04

### Changed

- Consolidate Watchdog cancellation state selection, handle cleanup and immediate
  cancellation accounting in the registry command owner. Remove the separate
  cancellation helper and boolean cleanup selector while preserving running
  commands, generation history and declaration lifetimes.

### Development

- Extend running Watchdog unregistration coverage to check that a later
  cancellation preserves removal and does not count as an immediate stop.
- Record scope and pending verification in the
  [0.11.1 release note](docs/changelog/0.11.1.md).

## [0.11.0] - 2026-10-04

### Changed

- Make cancellation stop existing work without allocating callback generations.
  Remove ordinary and Watchdog cancellation exhaustion paths; preserve pending
  commands for running work, actual handle cleanup and non-wrapping allocation
  when scheduling new callbacks.
- Treat cancellation at exhausted generations as `Cancelled` rather than a
  generation control failure. Subsequent generation values no longer include
  cancellation increments. This is a pre-1.0 semantic hard cut.
  Reconciliation to `None` and non-running unregistration share this behavior.
- Replace separate Watchdog scheduler and work-attempt allocation counters with
  one generation shared by each dispatched pair. Preserve role-scoped authority,
  separate handles and checked generation/deadline validation. Work and successor
  now share the observed dispatch generation rather than independent clocks.

### Removed

- Remove `WatchdogAttemptSnapshot` and the `AwaitingWork.attempt` wrapper. The
  awaiting-work snapshot now exposes `attempt_status` directly and represents
  its shared generation once through `successor_generation`.

### Development

- Update cancellation and exhaustion fixtures for state-based invalidation,
  cancellation at maximum generation and stale delivery after rearming.
- Cover shared-generation role boundaries and pending requests that preserve the
  dispatched/running pair until completion. Remove the independent attempt-counter
  exhaustion cases and retain allocation, deadline, cleanup and recovery fixtures.
- Add a runtime fixture rejecting mixed-generation dispatch pairs before provider
  arms, preserving inventory and testing both mismatched-pair orientations.
- Record the contract and pending verification in the
  [0.11.0 release note](docs/changelog/0.11.0.md).

### Documentation

- Add plain-English introductions to the safety, architecture, observability,
  and release guides, and reorganize the documentation index by audience while
  preserving the detailed technical contracts and historical records. Add
  graphics for module relationships, safety responsibilities, and Watchdog
  failure behavior, and use the shared IC Timers banner across all reader-facing
  documentation.

## [0.10.23] - 2026-10-04

### Changed

- Move ordinary claim-policy validation into reconciliation and delete its
  single-caller helper. Preserve claim and policy rejection before cancellation
  or schedule resolution.
- Finalize Watchdog scheduler generation and deadline failures through one
  terminal path, preserving generation-first error selection, atomic allocation,
  queued-work cleanup and declaration lifetimes.

### Development

- Extend the Watchdog cancellation fixture to assert that ordinary reconciliation
  rejects inactivity, valid deadlines and oversized delays without changing the
  armed declaration or its inventory.
- Extend the scheduler exhaustion fixture for simultaneous generation and
  deadline overflow, retaining terminal-state, cleanup and unchanged-counter
  assertions.
- Record the cleanup and pending verification in the
  [0.10.23 release note](docs/changelog/0.10.23.md).

## [0.10.22] - 2026-10-04

### Changed

- Use the canonical claim lookup for late measurement accounting and provider
  handle consumption, preserving no-op behavior for missing or superseded claims
  and the independent policy, role and handle-generation checks.
- Detach provider-handle pairs through their registry entry during Watchdog
  failure cleanup. Remove runtime pair assembly and the separate pair constructor,
  preserving cleanup order and harmless cleanup of empty or missing entries.
- Pass validated ordinary control directly to terminal-failure finalization,
  removing its repeated policy check and unreachable fallback. Preserve terminal
  reasons, wakeup cleanup and declaration-lifetime decisions.

### Development

- Extend the provider-cleanup fixture for empty and missing entries, and update
  restoration cleanup to use registry-owned pair detachment.
- Record the lookup cleanup and focused verification subjects in the
  [0.10.22 release note](docs/changelog/0.10.22.md).

## [0.10.21] - 2026-10-04

### Changed

- Use the canonical claim lookup for provider installation and effect
  confirmation, removing their duplicate identity and claim-generation lookup.
  Preserve stale-callback errors and independent role, state and generation checks.

### Development

- Cover lifecycle reconciliation rejection through the public APIs: mismatched
  identity for Once, transient lifetime and mismatched cadence for AfterCompletion,
  and an expired Watchdog claim after identity reuse. Assert typed errors and
  preservation of inventory, armed handles and the live callback. Verify the
  retained AfterCompletion callback still runs at its original deadline.
- Extend the duplicate-registration fixture to cover reconstruction from an
  empty consumer slot when the identity is already occupied. Preserve the empty
  slot, live declaration, armed wakeup and original callback on rejection.
- Cover provider installation and effect confirmation after claim removal and
  identity reuse, preserving rejected-handle ownership and unchanged inventory.
- Record the cleanup scope and pending verification in the
  [0.10.21 release note](docs/changelog/0.10.21.md).

## [0.10.20] - 2026-10-04

### Changed

- Move lifecycle declaration checks into the shared reconciliation operation and
  callback-context checks into the shared claim-transition operation. Remove two
  single-caller validation helpers while preserving rejection before timer
  mutation or provider-handle detachment.
- Store the latest outcome and reported work count as one private terminal event.
  Derive their public observations together while retaining independent success,
  failure and unacknowledged timestamps and failure-streak accounting.

### Development

- Add a focused outcome-history fixture covering empty and zero-work observations,
  interrupted attempts, subsequent completions and preservation of prior timestamps.
- Record the validation and outcome ownership changes and focused verification in
  the [0.10.20 release note](docs/changelog/0.10.20.md).

## [0.10.19] - 2026-10-04

### Changed

- Express ordinary pending-command precedence in one match, removing the
  duplicated sticky-unregistration branch while preserving exact reconciliation
  and earliest-demand scheduling.
- Build ordinary successors and Watchdog dispatch tokens from their validated
  claims through the existing token constructor, without repeating ownership
  fields in each transition.
- Remove the forwarding cleanup-effect constructor. Select Watchdog cancellation
  cleanup once and share conditional wakeup cleanup on generation exhaustion.

### Development

- Extend ordinary lifecycle fixtures for unregistration followed by control
  requests and for preservation of schedule metadata at equal deadlines.
- Share repeated follow-up reconciliation setup in the Watchdog precedence
  fixture, retaining all nine cases without a function-size lint exception.
- Record the cleanup and pending verification in the
  [0.10.19 release note](docs/changelog/0.10.19.md).

## [0.10.18] - 2026-10-04

### Changed

- Borrow claim and callback identities during registry cancellation, completion,
  unregistration and provider cleanup. Remove temporary owned copies while
  retaining independent identities in queued tokens, detached handles, effects
  and snapshots.
- Create the ordinary cleanup effect's owned identity only when a wakeup needs
  clearing; inactive cancellation and no-effect terminal paths keep it borrowed.

### Development

- Record the identity ownership cleanup and focused verification subjects in
  the [0.10.18 release note](docs/changelog/0.10.18.md).

## [0.10.17] - 2026-10-04

### Changed

- Initialize the canonical registry lazily in its existing runtime slot. Read
  canister version and time only when creating the first epoch; repeated
  initialization returns the original epoch without reconstructing a discarded one.

### Development

- Extend the initialization fixture to check typed borrow-conflict rejection
  and preservation of the original inventory epoch.
- Record the cleanup and pending verification in the
  [0.10.17 release note](docs/changelog/0.10.17.md).

## [0.10.16] - 2026-10-04

### Development

- Extend the identity-reuse fixture to check that stale measurements cannot
  update a replacement registration, while its own callback still records
  instruction and memory observations.
- Strengthen existing ordinary and Watchdog failure-accounting assertions to
  distinguish completion outcomes from terminal control failures.
- Record the test-only scope and pending verification in the
  [0.10.16 release note](docs/changelog/0.10.16.md).

## [0.10.15] - 2026-10-04

### Changed

- Derive `TimerCounters::work_completed()` from the saturating sum of its four
  classified completion counters. Remove the separately maintained total while
  preserving public counter values and separate start and interruption counters.
- Apply wakeup arms and Watchdog dispatch directly in the validated effect
  branches. Remove two single-caller binding helpers and their redundant variant
  checks while preserving provider ordering, confirmation and failure cleanup.
- Carry inert `MemoryPageExtent` values directly from platform reads through
  callback measurement accounting. Remove the identical private page-count type
  and conversion helper while preserving units and sampling order.

### Development

- Replace the test-only completion-partition checker with direct outcome
  assertions. Cover mixed completion outcomes reaching saturation in the counter
  fixture; retain runtime failure, cleanup and lifecycle assertions.
- Check successful callback completion separately from successor-generation
  failure in the Watchdog lifetime fixture. A failed replacement does not
  reclassify the callback's committed success as an invariant failure.
- Record the cleanup and verification scope in the
  [0.10.15 release note](docs/changelog/0.10.15.md).

## [0.10.14] - 2026-10-04

### Changed

- Carry the exact registration claim inside private callback tokens and borrow
  it for context control, Watchdog completion and provider cleanup. Remove claim
  reconstruction and its forwarding helpers while retaining exact running-work
  validation and non-clone public registration capabilities.

### Development

- Adapt the existing running-work negative fixture to construct stale claim and
  callback generations through the token constructor. Keep context-expiration,
  identity-reuse, cross-claim dispatch and provider cleanup subjects unchanged.
- Record the cleanup and verification scope in the
  [0.10.14 release note](docs/changelog/0.10.14.md).

## [0.10.13] - 2026-10-04

### Changed

- Return canonical control failures directly from ordinary directive resolution.
  Remove the private directive-error wrapper and registry translation while
  preserving typed failures, command precedence and completion cleanup.

### Development

- Extend the existing schedule fixture to check oversized retry delays and
  overflowing recurrence deadlines at the directive boundary. Test execution
  remains maintainer-owned.
- Record the cleanup and verification scope in the
  [0.10.13 release note](docs/changelog/0.10.13.md).

## [0.10.12] - 2026-10-04

### Changed

- Keep callback generation authority in the ordinary and Watchdog control
  counters. Remove repeated generations from private active states while
  preserving snapshot values, stale-callback rejection and checked allocation.
- Select the provider-handle slot in the registry's policy, role, state and
  generation validation match. Remove the second role lookup while preserving
  stale-token rejection, occupied-slot errors and rejected-handle cleanup.

### Development

- Seed exhaustion fixtures before arming callbacks so they reach the generation
  limit with real active tokens. Preserve cancellation, completion, successor
  retention and declaration-lifetime assertions.
- Format and check Rust in both workspaces through `make fmt` and `fmt-check`.
  Reuse the same check from `testing-check`, the commit hook and repository gates.
- Extend the existing staged-snapshot hook fixture to reject unformatted nested
  Rust while preserving the index and unrelated working edits.
- Keep provider churn page growth diagnostic rather than requiring a larger
  memory extent for test success. Preserve cancellation, continued work and stale
  deadline checks, plus the page measurements.
- Remove the release-prose advisory's obsolete `Open release line:` exemption
  and use the current workspace-version marker in its clean fixture.

### Documentation

- Describe ordinary arbitration as current behavior and link historical downstream
  qualification receipts instead of repeating their verdict in the observability
  contract. Record the cleanup and pending verification in the
  [0.10.12 release note](docs/changelog/0.10.12.md).

## [0.10.11] - 2026-10-04

### Development

- Extend the native Watchdog dispatch-failure fixture to cover successor binding,
  work binding and effect confirmation for both declaration lifetimes. Check
  provider cleanup, absent consumer work, truthful retained counters and transient
  claim expiration across identity reuse. Test execution remains maintainer-owned.
- Record the test-only scope and verification subject in the
  [0.10.11 release note](docs/changelog/0.10.11.md).

## [0.10.10] - 2026-10-04

### Changed

- Decide Watchdog completion's declaration removal once from its final inactive
  state, lifetime and pending unregister command. Remove the separate decisions
  in normal stop and failed successor replacement while preserving continuation,
  terminal reasons and provider cleanup.

### Development

- Extend the Watchdog terminal-failure lifetime fixture to cover generation
  exhaustion during immediate and exact-deadline completion replacement.
- Check that remove-on-stop Watchdogs retain their declaration when keeping or
  replacing a successor, including retention with an exhausted generation counter.
- Record the cleanup and verification scope in the
  [0.10.10 release note](docs/changelog/0.10.10.md).

## [0.10.9] - 2026-10-04

### Changed

- Finalize ordinary callback results and callback-borrow failures through one
  completion helper. Remove the separate invariant-stop completion path and the
  unreachable ownership-mismatch recovery branch after successful acceptance.
  Preserve terminal cleanup, declaration lifetimes and measurement behavior.

### Development

- Add a focused native callback-borrow failure fixture covering terminal reasons,
  both declaration lifetimes, provider cleanup and absent work measurements.
- Record the cleanup and verification scope in the
  [0.10.9 release note](docs/changelog/0.10.9.md).

## [0.10.8] - 2026-10-04

### Changed

- Store ordinary inactive reasons and pending commands in their corresponding
  inactive and running states. Remove the independent entry fields, paired
  terminal assignments and manual pending-command clearing.
- Use the same checked arming operation for ordinary requests and authorized
  completion successors. Stop directly with its selected reason without a
  separate optional-successor control operation.

### Development

- Correct redundant type visibility qualifiers inside the private control module;
  the crate facade and consumer visibility remain unchanged.
- Update control fixtures for reason-bearing inactive state and verify that
  failed generation allocation preserves a running attempt's pending command.
- Keep the compact handoff focused on current architecture and unresolved work;
  release state remains authoritative in Cargo, changelog and release-note status.
- Record the live 0.10.7 baseline and cleanup in the
  [0.10.8 release note](docs/changelog/0.10.8.md).

## [0.10.7] - 2026-10-04

### Changed

- Finalize detached public-claim transitions in one helper. Remove the separate
  success-path helper while preserving restoration, retirement and typed errors.
- Compute ordinary completion's removal-on-stop rule once for normal stopping,
  invariant failure and checked failures. Derive cancellation removal directly
  from execution state and declaration lifetime without a mutable removal flag.

### Development

- Remove ordinary completion's obsolete `too_many_lines` lint expectation after
  the simplification brings it below Clippy's limit.
- Extend the existing transition-error fixture to check failed restoration,
  provider-handle cleanup, claim retirement and restoration-error precedence.
- Record the pushed 0.10.6 baseline and private cleanup in the
  [0.10.7 release note](docs/changelog/0.10.7.md).

## [0.10.6] - 2026-10-04

### Changed

- Give ordinary scheduling and completion one checked arming operation that
  allocates a generation and installs its deadline atomically.
- Store Watchdog pending commands inside the awaiting-work state. Remove the
  independent field and explicit clearing when leaving that state; preserve
  reconciliation in the dispatched gap and running-work command precedence.
- Build ordinary provider effects from successful transition inputs and the
  allocated generation. Remove unreachable contradictory-state recovery paths
  while retaining input, callback-authority and provider-ownership validation.

### Development

- Use `if let` for ordinary completion's optional successor selection, addressing
  the maintainer-reported Clippy diagnostic without changing transition behavior.
- Retain behavioral transition and failure fixtures; remove obsolete assertions
  about an independent Watchdog pending field. Extend attempt-retirement coverage
  to check that its queued command cannot affect successor work.
- Record the live 0.10.5 baseline and this cleanup in the
  [0.10.6 release note](docs/changelog/0.10.6.md).

## [0.10.5] - 2026-10-04

### Changed

- Store the Watchdog inactive reason inside its private inactive state. Remove
  the independent reason field and paired assignments; active states no longer
  carry irrelevant inactive metadata. Public snapshots and terminal behavior
  retain their existing contracts.
- Finalize cancellation removal once from its policy decision. Remove the
  redundant follow-up failure cleanup; preserve terminal errors, declaration
  lifetimes and provider cleanup effects.
- Route recurring ordinary coalescing through the shared scheduling-request
  handler. Remove the separate counter/metadata update path while retaining
  the existing deadline without allocating a generation or recalculating an
  unused cadence deadline.
- Remove the redundant expected-role argument from private callback-claim lookup.
  Provider installation and effect confirmation retain their operation-specific
  role, generation and state validation.

### Development

- Check the public inactive reason in the existing Watchdog generation-exhaustion
  and deadline-overflow fixtures while retaining atomicity and cleanup assertions.
- Extend the Watchdog failure/lifetime matrix to cover exhausted cancellation
  before and after work dispatch, including selected callback cleanup and the
  absence of successful cancellation counts.
- Extend recurring coalescing coverage to cadence and exact schedules with
  exhausted generations and overflowing hypothetical successors; preserve mode,
  request observations, counters and callback authority.
- Record the maintainer-reported pushed 0.10.4 baseline and this cleanup in the
  [0.10.5 release note](docs/changelog/0.10.5.md).

## [0.10.4] - 2026-10-04

### Changed

- Keep ordinary completion authorization at the registry's exact running-work
  boundary. Remove the repeated generation argument and stale-completion check
  from the private control transition, whose only production caller has already
  validated that authority within the same atomic operation.
- Return canonical `TimerControlFailure` values directly from checked ordinary
  control. Delete the private error enum, conversion helper and unreachable
  stale-completion result branch; preserve generation-exhaustion handling.

### Development

- Update control fixtures to the authorized-completion operation and remove its
  superseded stale-generation fixture. The maintained registry matrix retains
  stale-token rejection and unchanged-observation coverage at the live boundary.
- Record the maintainer-reported live 0.10.3 baseline and this cleanup in the
  [0.10.4 release note](docs/changelog/0.10.4.md).

## [0.10.3] - 2026-10-03

### Changed

- Give each registry entry one predicate for exact running-work ownership.
  Callback lookup, delegated control and ordinary/Watchdog completion share
  claim, role, generation and running-state validation; remove the separate
  completion checks for the same invariant.
- Keep callback acceptance, provider binding and post-completion measurements
  on their distinct validation boundaries.

### Development

- Cover rejection of unstarted, wrong-role, stale-claim, stale-generation,
  completed and removed work tokens across every policy without observation
  changes. Existing context-expiration and identity-reuse fixtures remain.
- Record the maintainer-reported pushed 0.10.2 baseline and this cleanup in the
  [0.10.3 release note](docs/changelog/0.10.3.md).

## [0.10.2] - 2026-10-03

### Changed

- Give runtime provider binding sole ownership of clearing rejected handles.
  Initial arms, Watchdog successor/work dispatch and detached-handle restoration
  use the same consuming operation instead of repeating rejection cleanup.
- Use one Watchdog dispatch cleanup exit for work-binding and confirmation
  failures, confirming the dispatch only after work installation succeeds.
- Preserve registry validation, successor-before-work ordering, partial-dispatch
  cleanup, effect confirmation and callback rollback rules.

### Development

- Add focused binding coverage for unavailable runtime state and expired claims;
  retain installation and restoration fixtures. Extend the Watchdog dispatch
  fixture to cover work-binding and confirmation failures, complete handle cleanup
  and the absence of confirmed or started work.
- Record the live 0.10.1 baseline and this private cleanup in the
  [0.10.2 release note](docs/changelog/0.10.2.md).

## [0.10.1] - 2026-10-03

### Changed

- Use one private resolved schedule for explicit requests, ordinary completion
  directives and pending commands. Resolve deadline, requested delay and scheduling
  mode together; remove duplicate intermediate models and registry conversions.
- Represent Stop as the absence of a successor schedule. Remove the private
  directive-snapshot mode mapping and its fallback to earlier entry metadata;
  public snapshots remain inert observations.

### Development

- Update existing schedule fixtures to check resolved scheduling metadata and
  preserve public snapshot conversion and validation coverage.
- Record the maintainer-reported live 0.10.0 baseline and this cleanup in the
  [0.10.1 release note](docs/changelog/0.10.1.md).

## [0.10.0] - 2026-10-03

### Fixed

- Give Watchdog invariant failures precedence over pending cancellation when
  reporting the inactive reason and cancellation count. Such failures report
  `Failed`, matching ordinary completion, while retaining successor cleanup and
  declaration-lifetime behavior. This observable semantic change uses a new minor
  line.

### Changed

- Remove the private scheduling-action model that duplicates control generations
  and deadlines. Scheduling returns only an optional initial/replacement arm kind;
  the registry builds effects from authoritative control state.
- Derive owned provider roles from entry policy and handle slot instead of storing
  a second role. Preserve exact claim, generation and role validation and complete
  tokens on detached handles.

### Development

- Update existing control and arbitration fixtures and add focused coverage for
  malformed same-claim provider roles, cleanup and failure classification.
- Record scope and pending maintainer validation in the
  [0.10.0 release note](docs/changelog/0.10.0.md).

## [0.9.5] - 2026-10-03

### Changed

- Remove unused absolute deadlines from private provider effects. Policy control
  state remains the deadline owner; effects retain resolved provider delays,
  callback authority and arm kind.
- Make ordinary cancellation return checked success or a typed error instead of
  a generic scheduling action. The registry owns cleanup and pending commands;
  remove the impossible arm-on-cancel and clear-on-schedule branches.

### Development

- Observe authoritative snapshot deadlines in the existing registry fixtures,
  preserving provider-delay, generation and malformed-token coverage.
- Update existing control fixtures to check cancellation state, repeat-cancel
  generation stability and atomic failure on generation exhaustion. Existing
  registry lifetime and runtime handle-cleanup fixtures remain the boundary checks.
- Use one workspace-version projection in the release handoff. Remove the
  redundant latest-release marker and its checks and fixtures.
- Remove the historical README sentence prohibition from release validation.
  Keep canonical API-line and dependency-pin checks, and verify that editorial
  prose does not block structurally valid releases.

### Documentation

- Replace stale preparation instructions with scoped historical evidence and
  current follow-up ownership. Record the completed 0.9.4 release and this
  cleanup in the [0.9.5 release note](docs/changelog/0.9.5.md).

## [0.9.4] - 2026-10-03

### Changed

- Build ordinary completion effects from the checked control state and selected
  schedule. Remove the duplicate completion-action translation, pass-through
  cancellation flag and fabricated fallback schedule metadata. Keep cancellation
  policy in the registry, checked generations and existing failure/lifetime rules.

### Development

- Update control fixtures to observe resulting state and generations. Extend the
  existing nested cancellation/ensure fixture to check that only a winning
  cancellation increments its counter.

### Documentation

- Record the maintainer-reported live 0.9.3 release, matching artifacts and
  successful hosted checks/MSRV/tag validation. Close the shipped README issue
  [#3](https://github.com/dragginzgame/ic-timers/issues/3).
- Track scope and pending maintainer validation in the
  [0.9.4 release note](docs/changelog/0.9.4.md).

## [0.9.3] - 2026-10-03

### Changed

- Remove ordinary completion's duplicate missing-cadence check. The directive
  resolver remains authoritative; illegal Once recurrence keeps the same typed
  terminal failure, Stop observation and declaration-lifetime behavior.
- Share effect-confirmation deduplication, armed-delay observation and wakeup
  counting after role-specific validation. Watchdog dispatch still validates its
  paired work attempt before recording either counter, including on repeated
  confirmation. Preserve provider binding and recovery ordering.

### Fixed

- Align the README API line and exact shared-registry dependency example with
  the current workspace version, addressing
  [GitHub issue #3](https://github.com/dragginzgame/ic-timers/issues/3).

### Development

- Project both README version fields from workspace truth during bumps, include
  the README in metadata rollback and check it for release drift. Extend the
  existing preparation and release-truth fixtures without changing gate scope.

### Documentation

- Record the live 0.9.2 release, successful hosted checks and MSRV/tag validation,
  and the disposition of the lifecycle-test advisory in GitHub issue #8.
- Refresh read-only downstream adoption evidence: Toko Miner's current dirty
  lockfile selects both timer 0.8.1 and 0.9.2; coherent adoption remains blocked.
- Record scope and pending maintainer validation in the
  [0.9.3 release note](docs/changelog/0.9.3.md).

## [0.9.2] - 2026-10-03

### Changed

- Select ordinary arm kinds and allocate completion generations directly,
  removing derived optional planning values while preserving checked transitions.
- Use instruction sample count as the sole empty-summary marker. Public latest
  and maximum getters still distinguish no sample from a zero-valued sample;
  latest and maximum observations continue after count or total saturation.
  Derived Debug output reflects the new private field representation.
- Apply callback-role recovery to every unexpected binding error without repeating
  the complete error-variant list. Watchdog work still traps for rollback;
  ordinary callbacks and scheduler binding failures retain their cleanup paths.
- Share ordinary failed-completion bookkeeping between consumer invariant failures
  and checked control failures. Preserve their distinct inactive reasons, error
  results and reported work counts.
- Store each registry entry's control, callback and cadence in one policy-specific
  payload. Derive policy observations from it and remove the independent callback
  variants and missing-callback state. Preserve scheduling and recovery contracts.

### Development

- Satisfy Clippy with a positive initial-case branch in the ordinary terminal
  fixture, `Option::map_or_else` for optional completion failures and a const
  private entry constructor. Preserve
  exhaustion/lifetime coverage and distinct failure outcomes without suppressions.
- Follow up [GitHub issue #8](https://github.com/dragginzgame/ic-timers/issues/8)
  by separating readiness mapping, repeated message driving and measurement
  assertions from the ordered IcyDB-shaped lifecycle fixture. Read callback
  readiness once. Preserve lifecycle coverage and gate scope; supplemental MSRV
  lint validation remains pending.
- Verify cached and downloaded PocketIC artifacts through one hash-first path.
  Rejection diagnostics use that verification result without hashing or executing
  the rejected binary again. Explicit overrides remain untouched; rejected
  downloads preserve the cache and clean temporary artifacts.
- Give workspace-version reading and mutation one table-scoped owner across
  version display, bumping, staging, release-truth, commit and tag checks. Preserve
  dependency versions and reject failed version selection before staging. Require
  both locked workspaces to resolve one timer package at the workspace version.
- Exercise PocketIC path and automatic-install selection through the actual Make
  recipe instead of matching its source layout. Cover default, environment,
  command-line, identical-path and empty overrides while retaining audited pins.

### Documentation

- Record the maintainer-reported live 0.9.1 release and matching local artifacts,
  removing obsolete bump instructions from the handoff and preparation note.
- Clarify user-owned lockfile staging and mark the original 0.5 Watchdog
  reconciliation restriction as superseded by the 0.8 deadline contract.
- Track the next private implementation changes and pending validation in the
  [0.9.2 simplification note](docs/changelog/0.9.2.md).

## [0.9.1] - 2026-10-03

### Changed

- Record running unregistration directly in the registry's pending command,
  removing unreachable control actions and error branches. Consolidate control
  tests around their local transitions; registry tests retain command arbitration.
- Deduplicate Watchdog effect confirmation with its unique successor generation,
  removing the redundant work-generation marker. Preserve idempotent arm and dispatch
  counters and validate work tokens before accepting a repeated confirmation.
- Let Rust enforce provider export and alias visibility, with non-suppressible
  private-interface checks. Keep direct-provider confinement and restricted
  platform declarations enforced by the repository checker.

### Development

- Remove historical Canic adoption status from release validation and automatic
  staging. Release truth remains owned by current package and release metadata.
- Use Make execution fixtures for release gates and all combined release flavours
  instead of recipe-text matching. Cover failure ordering, repeated
  bump invocation, metadata-only staging and the exact tag namespace.
- Exercise provider visibility fixtures on both hosted Rust toolchains, including
  alias leaks, local lint suppression and external access through glob imports.
- Remove unused deserialization derives and the runtime probe's direct `serde`
  dependency; query encoding, host decoding and stable state remain unchanged.

### Documentation

- Refresh the handoff, safety evidence wording and 0.9.0 delivery record for the
  tagged release, removing obsolete instructions to bump from 0.8.4. Release
  artifacts do not substitute for missing validation output.
- Scope the historical lifecycle-composition blocker to its original adoption
  subject and distinguish the later frozen receipt from current qualification.
- Track this work and pending maintainer validation in the
  [0.9.1 simplification note](docs/changelog/0.9.1.md).

## [0.9.0] - 2026-10-03

### Changed

- Hard-cut ordinary completion arbitration: a pending exact reconciliation
  replaces the callback scheduling proposal before policy and deadline validation.
  Discarded proposals cannot terminate a valid exact successor. Explicit consumer
  invariant failure remains terminal. `latest_directive()` projects the effective
  exact `ScheduleAt` directive when reconciliation wins.
- Check formatting against a temporary snapshot of staged files during commit.
  The hook no longer formats working copies or rejects partial staging itself.

### Fixed

- Reject conflicting or duplicated workspace/latest-release markers, release-note
  headings/statuses, and current-release changelog headings, even when a correct
  marker also exists. Free-form release prose remains advisory.
- Resume an interrupted release commit/tag phase from the matching clean release
  commit without creating another commit. Validate an existing annotated tag at
  `HEAD`; reject arbitrary commits, conflicting tags and additional staged changes
  for an already tagged release. Combined release targets still always bump.
- Replace unconditional native polling of suspended futures with real wake
  notifications so unrelated due timers can progress.
- Document the suspension helper's intentionally non-`Send` future with a scoped
  Clippy expectation, matching the single-threaded runtime executor.
- Keep the PocketIC ordinary-work gate pending through awaited self-call replies
  instead of a stored waker across ingress contexts. Require an observed reply
  before interleaving assertions and report gate errors in fixture observations.

### Evidence and documentation

- Add native suspended-work/control and exact-reconciliation precedence matrices,
  real-canister await/ingress interleaving and provider-churn PocketIC fixtures,
  and shell regression fixtures for release recovery, contradictory metadata and
  preservation of staged/unstaged files by the hook. Execution remains user-owned.
- Document that registry and owned-handle bounds do not bound provider queue
  memory: cancellation in the pinned provider retains future deadline entries.
  Update the released 0.8.4 handoff and separate recorded release evidence from
  unexecuted 0.9.0 fixtures. See the [0.9.0 note](docs/changelog/0.9.0.md).

## [0.8.4] - 2026-10-03

### Fixed

- Select the release target from the requested bump and changelog without
  requiring duplicate handoff markers or particular release-note status words.
- Preserve the complete Watchdog snapshot when an initial cadence request
  fails deadline validation, including its validated-request counter.
- Verify PocketIC binary hashes before executing cached, overridden or
  downloaded binaries, including diagnostic paths.
- Require an annotated release tag in the exact `refs/tags/` namespace at
  `HEAD`; reject same-named branches and lightweight tags.
- Restore pre-bump release metadata and both lockfiles when version preparation
  fails or receives a handled interruption, preserving existing user edits.

### Development

- Update both pinned Rust-toolchain action references to the revision proposed
  by Dependabot PR #5, retaining the existing toolchains and validation gates.
- Preflight the requested version, impact, changelog and release markers before
  the deployment gate. Combined release targets still always bump afterward.
- Remove the redundant arm-variant check and add regression fixtures for
  rejected requests, tag identity, binary verification order, preflight and
  bump rollback. Test execution remains user-owned.

### Documentation

- Corroborate 0.8.1 and 0.8.3 registry publication and refresh Toko Miner's dated
  dependency record without extending earlier managed evidence to its later graph.
- Record the decision to retain the coherent lifecycle fixture and current MSRV
  lint scope for advisory IC-TIMERS-003.
- Refresh the handoff and 0.8.3 note to reflect the completed release. Track
  follow-up scope and pending validation in the
  [0.8.4 release note](docs/changelog/0.8.4.md).

## [0.8.3] - 2026-10-03

### Development

- Keep combined release targets explicit: `release-patch`, `release-minor`,
  `release-major` and `release-x` always run their matching version bump after
  deployment validation.
- Classify pending release changes against the most recent reachable release
  tag, so an untagged workspace version does not block the next bump. Explicit
  missing bases and Git failures still fail validation.
- Validate release metadata and both lockfiles before the release commit;
  reject unstaged or untracked work before committing and tagging.

### Documentation

- Reserve version bumps, tests, staging, commits, tags, pushes and publication
  for the maintainer. Automated contributors prepare only the next undated
  changelog section and release-line note.

## [0.8.2] - 2026-10-03

### Fixed

- Remove transient declarations on checked terminal control failures, including
  Watchdog scheduling generation exhaustion and scheduler deadline overflow.
  Clear owned provider callbacks before removing the declaration; retained
  declarations remain observable and inactive.
- Resolve both workspace dependency graphs when checking locked metadata after
  a version bump, so stale root or testing lockfiles fail validation.
- Propagate Git failures during release-impact classification and stop
  repository validation when classification or a required check fails.

### Changed

- Use one public-control handle-detachment path and one Watchdog scheduler-arm
  implementation, removing the duplicated reconciliation and completion flows.
- Restrict private platform items to crate visibility, preventing indirect
  public re-exports of timer authority.

### Development

- Check every shell script individually for syntax errors.
- Prepare versions from the current worktree without a preparatory commit or
  test run. Keep the complete gate in user-operated release targets. Tests,
  commits, release tags and pushes remain user-owned.
- Reject qualified, grouped, multiline and locally aliased public platform
  exports, including type aliases. Add executable regression fixtures for
  provider visibility, validation failures and stale lockfiles.

### Evidence

- Pass 108 native tests, including terminal transient cleanup and queued
  callback removal. These boundary-failure cases have native coverage.
- Pass all nine maintained PocketIC Watchdog subjects and all four policy
  cohorts with the audited PocketIC 15.0.0 binary. CI, Rust 1.88.0 MSRV,
  nested-probe lint, strict Wasm Clippy and package checks pass.

### Documentation

- Correct stale 0.8.1 pre-bump wording in the handoff and release note, and
  document the scope of native cleanup evidence in `SAFETY.md`.
- Record the runtime and tooling changes and validation results in the
  [0.8.2 release note](docs/changelog/0.8.2.md).

## [0.8.1] - 2026-10-02

### Development

- Update the development and hosted CI toolchains to Rust 1.99.0 while
  retaining Rust 1.88.0 as the MSRV. Run warning-denied nested-probe linting
  on both toolchains in hosted CI, including every supported policy feature.
- Fix Rust 1.99 Clippy's empty-collection assertions in the native inventory
  test to include collection contents on failure, and collapse the nested
  unregister check in the runtime probe. No runtime or public API change.
- Remove the obsolete scheduler line-count suppression and replace the nine
  active Clippy suppressions with explained lint expectations, so obsolete
  exceptions fail warning-denied checks (Toko Miner IC-TIMERS-002).

### Documentation

- Align current guidance with the tagged 0.8.0 API, exact dependency example,
  Rust 1.99.0 toolchain and nine maintained PocketIC Watchdog subjects. Correct
  stale pending-release wording, distinguish historical downstream adoption
  evidence, and clarify implemented observation semantics. Repository-only;
  no runtime or public API change.
- Refresh the handoff and adoption records with dated 0.8.0 publication and
  scoped Canic/IcyDB/Toko Miner composition evidence. Retire the old lifecycle
  blocker while keeping current deployment and cost comparisons unverified
  (Toko Miner IC-TIMERS-001).

## [0.8.0] - 2026-09-20

### Added

- Expose an inert `TimerRegistrationId` on every timer snapshot, combining the
  runtime epoch and checked registration sequence so consumers can detect
  unregister/re-register counter resets within one epoch.
- Add exact Watchdog deadlines through registration/context
  `reconcile_schedule`, `WatchdogDecision::ScheduleAt`, and
  `WatchdogReconcileState::ScheduledAt`. Sleeping work owners can use one
  Watchdog without an auxiliary deadline timer.

### Changed

- Extend the current snapshot shape and exhaustive Watchdog enums at the 0.8
  minor boundary. Reuse the existing claim sequence, pending-command owner and
  pre-armed successor without compatibility paths or another runtime.
- Define continuity, saturation and source-window requirements for downstream
  interval measurements; update the local consumer projection and probe DTOs.

### Evidence

- Pass 105 native tests and nine audited PocketIC Watchdog subjects, including
  deadline sleeping/replacement, registration replacement with counter regrowth,
  upgrade reset, and trap/instruction-exhaustion recovery after a nested deadline
  proposal. Clippy, rustdoc, Wasm, MSRV, nested-probe lint and package checks pass.

## [0.7.1] - 2026-09-15

### Changed

- Update the exact `ic0` dependency from 1.1.0 to 1.2.0 in both workspace
  lockfiles. The upstream additions do not change the bindings used by the
  private platform module; no runtime or public API adaptation is needed.
- Update the test canisters' exact `ic-cdk` dependency and its macros from
  0.20.2 to 0.20.3. No probe source changes are required; this CDK update only
  affects the nested testing workspace.

## [0.7.0] - 2026-08-28

### Added

- Add progress-sensitive Watchdog continuation through
  `WatchdogDecision::ContinueImmediately`, claim- and context-scoped immediate
  ensure operations, and `WatchdogReconcileState::ScheduledImmediately` for
  the first actionable wake-up.

### Changed

- Hard-cut `reconcile_watchdog` from the generic `TimerReconcileState` to the
  policy-specific `WatchdogReconcileState`. Immediate demand replaces the one
  authoritative pre-armed successor at deadline now; cadence `Continue`, stop,
  cancellation, unregistration, generation ownership, and the two-message
  pre-arm protocol retain their existing meanings.
- Project an immediate successor through the existing snapshot fields as
  continuation mode with zero requested/armed delay. Equivalent or earlier
  deadlines and repeated/dispatched/running requests coalesce without another
  provider handle or snapshot format.
- Make release-truth finalization validate the separate latest-release and
  named-target markers semantically, then advance release history and remove
  the target marker. It no longer requires one exact candidate-prose sentence.

### Documentation

- Define immediate scheduling transitions, pending-command precedence,
  replicated rollback requirements, IcyDB integration calls, state-space
  change, and provider-call cost in a dedicated design record.

### Evidence

- Extend the pinned PocketIC Watchdog matrix with zero-delay initial scheduling
  and successful immediate continuation without cadence-time advancement. The
  complete eight-test trap, exhaustion, lifecycle, isolation, capacity, and
  cancellation matrix passes.
- Freeze native snapshot semantics when an immediate ensure coalesces with an
  overdue cadence wake-up: cadence scheduling mode and armed delay remain
  unchanged while the immediate request and coalescing counters advance.
- Record immediate-versus-cadence instruction/cycle observations and current
  compiler, optimized, and deterministic-gzip size cohorts. The controlled
  Watchdog final Wasm is 877 bytes (0.335%) above after-completion.

## [0.6.1] - 2026-08-15

### Changed

- Treat repository-only release impact as an advisory when the maintainer
  explicitly invokes a version-bump or release target. Empty release subjects
  still fail, and repository-only releases retain the complete validation
  gate.

### Documentation

- Clarify that callback instruction aggregates cover the accepted
  `ic-timers` execution interval, not the complete IC message: provider
  entry/exit, page reads, and the post-interval summary write remain outside.
- Require consumers that need a durable terminal audit receipt for a removed
  transient timer to own it outside the volatile registry; no tombstone or
  parallel authority is added.
- Freeze IcyDB's 0.6 performance-rebaseline and single-package qualification
  requirements without claiming that downstream adoption has occurred.

### Evidence

- Extend the PocketIC cohort with deterministic minimal and bounded
  representative Watchdog callbacks. Report scheduler/work instruction
  intervals and cycle deltas while marking complete-message instructions and
  their residual difference unavailable on PocketIC 15's public test surface.

## [0.6.0] - 2026-08-15

### Added

- Add `TimerInventorySnapshot` and `timer_inventory()` so one atomic bounded
  observation carries the runtime epoch even when the initialized registry has
  no timer declarations.

### Changed

- Define instruction observations as the accepted `ic-timers` execution
  interval, including acceptance, completion processing, and any successor
  binding rather than application code alone. Memory reads bracket that
  interval; provider entry/exit and the summary write remain outside its
  instruction delta.
- Document that terminal `RemoveWhenStopped` completion may remove its timer
  before the final measurement can be retained.

### Removed

- Remove the bare-vector `timer_snapshots()` inventory function. Consumers use
  `timer_inventory()` and read or consume its ordered timer vector; no alias or
  deprecated forwarding surface is retained.

### Documentation

- Refresh the README and compact handoff for the 0.6 API with a
  minimal registration example, scannable policy, ownership, observability,
  safety, development, and documentation tables, and clearer shared-registry
  guidance.
- Record IcyDB's completed exact-0.5.0 hard-cut adoption while preserving its
  tagged 0.3.4 and post-tag 0.3.8 evidence as historical subjects.
- Record Canic's completed exact-0.5.0/schema-3 adoption, including its memory
  projection and one-package graph, while preserving 0.3.8/schema 2 as
  historical evidence and keeping combined-framework qualification open.
- Add a public memory-summary projection example that distinguishes no
  completed sample from an observed zero-growth sample.

### Evidence

- Add a probe-only PocketIC cohort measurement for the start/end page reads
  excluded from callback instruction aggregates. PocketIC 15.0.0 reports an
  empty bracket and the four-read sampled bracket at 200 call-context
  instructions each, an observed delta of zero without changing the runtime
  API or measurement interval.
- Rerun the complete PocketIC watchdog matrix and policy cohorts against the
  hard-cut inventory API; recovery, isolation, capacity, ordering, and policy
  behavior remain green.

## [0.5.0] - 2026-08-15

### Changed

- Hard-cut the shared callback control capability into `OnceContext`,
  `AfterCompletionContext`, and `WatchdogContext`. Each callback now receives
  only the mutation operations legal for its declared policy, while exact
  callback-generation validation and expiry semantics remain unchanged.
- Give all callback contexts the same policy-shaped `ensure_scheduled` naming
  used by their registration capabilities. `OnceContext` accepts an explicit
  schedule, while after-completion and Watchdog contexts use their configured
  cadence.
- Keep nested request arbitration in the registry's existing pending-command
  state and callback generations. Remove request counters that were incremented
  but never read, compared, exposed, or used to select a winning command.
- Add allocation-free start/end Wasm-memory and stable-memory page-extent
  sampling for normally completed scheduler and work callbacks. Snapshots keep
  only the latest extent pair and maximum observed start-to-end growth for each
  role; they never total absolute page counts or fabricate samples for trapped
  or instruction-exhausted work.
- Route completed measurements from the callback token's canonical role rather
  than maintaining separate scheduler and work recording paths. An impossible
  policy/role pairing now fails as an ownership invariant instead of silently
  discarding the record.

### Removed

- Remove the public `TimerContext` type and its policy-probing `ensure_once`
  and `ensure_recurring` methods. No alias or deprecated forwarding surface is
  retained.
- Remove public `TimerError::WrongPolicy`; an invalid policy operation is no
  longer expressible through the callback API. Defensive internal policy
  mismatches fail as ownership invariants.
- Remove `TimerControlFailure::RequestSequenceExhausted` and its stable label.
  Generation, deadline, directive, and provider-binding failures remain.

## [0.4.1] - 2026-08-15

### Changed

- Share ordinary schedule resolution and retained lifecycle registration
  verification instead of maintaining policy-specific copies.
- Encode Once ensure, after-completion ensure, and authoritative reconciliation
  as one closed ordinary request kind, removing a boolean policy gate and its
  pass-through scheduling helper.
- Derive watchdog cancellation and transient-removal decisions from canonical
  post-transition state rather than an additional boolean result.
- Bind provider operations from one complete registry effect rather than
  passing duplicate token and delay arguments beside that effect.
- Validate provider-effect shape before cleanup or platform calls, including
  exact identity and claim-generation agreement between a Watchdog successor
  and its queued work.
- Centralize exact callback-claim ownership checks across dispatch,
  measurement, provider installation, and provider-handle consumption.
- Carry one closed initial/replacement arm kind from ordinary control through
  provider binding instead of splitting and rejoining duplicate arm variants.
  Clear effects likewise carry a non-empty callback selection, so "clear
  nothing" cannot be emitted and a Watchdog scheduler replacement is rejected
  before provider calls.
- Share the detach/transition/restore path used by cancellation and explicit
  unregistration, and remove compatibility-only annotations from private
  control errors.
- Correct public control documentation to distinguish armed wake-ups, pending
  successors, and running work that cancellation cannot interrupt.

### Fixed

- Remove a fresh `RemoveWhenStopped` declaration when it is cancelled before
  its first schedule. Once, after-completion, and Watchdog claims now expire
  consistently on cancellation regardless of whether they ever owned a
  provider handle.
- Reject malformed cross-registration Watchdog dispatch effects before any
  provider callback is armed or observability counter is confirmed.
- Prevent a late callback from an expired claim from consuming the provider
  handle of a replacement registration that reused the same identity, role,
  and callback generation.

## [0.4.0] - 2026-08-15

### Changed

- Correct the pre-1.0 release policy: hard cuts still remove superseded APIs
  without compatibility shims, but any public removal or incompatible public
  semantic change advances the minor compatibility line rather than a patch.
  The `TimerFuture` removal in 0.3.7 is retained and explicitly acknowledged
  as a SemVer mistake.
- Classify changes since the current version tag as crate-impacting,
  repository-only, or absent. Version bumps now reject repository-only
  publication before expensive validation, while `repository-check` provides
  the focused non-publishing validation path.
- Record that the current uncommitted Canic and IcyDB adoption worktrees both
  exact-pin 0.3.8. Development-time package alignment is complete; released
  combined composition still requires one-package qualification from tagged
  downstream subjects.
- Start the 0.4 hard-cut cleanup of the public facade and private runtime:
  remove superseded compatibility surface, consolidate duplicated paths, and
  keep only one canonical authority for each timer operation.
- Make the crate root the only public import path. The former public
  `schedule` and `snapshot` module paths are removed while their retained
  provider-neutral values remain available from `ic_timers::*`.
- Collapse identity validation into `TimerIdentity::try_new`, return identity
  components directly as `&str`, and report field-specific failures from one
  `TimerIdentityError` enum. The component bound is now named
  `MAX_TIMER_IDENTITY_COMPONENT_BYTES`.
- Keep observation construction registry-owned: observation fragments no
  longer implement `Default`, and policy/state helper projections that merely
  duplicated the canonical `TimerSnapshot` surface are private.
- Correct inactive-state documentation to distinguish retained callback
  authority from the absence of a scheduled or running callback generation.
- Consolidate exact-claim lifecycle verification, ordinary scheduled
  transitions, earliest/exact deadline mutation, provider-role slot selection,
  and lazy provider-handle detach/clear selection without adding another
  authority or compatibility layer.
- Mark private platform and detached provider capabilities `must_use` so new
  internal paths cannot silently ignore the obligation to bind, restore, or
  clear them.
- Consolidate watchdog terminal failure transitions so they clear pending
  commands consistently, and allocate the paired scheduler/work generations
  atomically before changing canonical state.
- Keep directive/policy mismatch internal to the registry instead of exposing
  it through the public scheduling-error vocabulary.
- Borrow exact registration claims during private unregistration and derive
  callback claims from one complete callback token, avoiding redundant
  identity allocation and independently supplied authority fields.

### Fixed

- Drain every detached watchdog provider handle before returning the first
  restoration failure. A failed wake-up restoration can no longer skip the
  remaining work-handle capability; focused fault injection covers the
  two-handle path.
- Avoid re-borrowing the registry when an exact detached provider capability
  is already available, eliminating eager alternate-handle evaluation.
- Restore all provider handles after any unexpected synchronous post-detach
  registry error. If restoration itself fails, retire the exact claim instead
  of dropping a linear capability or leaving false scheduled state.

### Removed

- Remove `TimerLabel`, `TimerLabelError`, `TimerIdentity::new`, and
  `MAX_TIMER_LABEL_BYTES`; the intermediate label abstraction duplicated the
  only supported identity constructor.
- Remove conversion from inert `TimerDirectiveSnapshot` observations back to
  executable `TimerDirective` commands.
- Remove duplicate `consecutive_expected_failures` forwarding methods from
  `TimerSnapshot` and `TimerObservabilitySnapshot`; the value remains on
  `TimerOutcomeSnapshot`, and the cheap identity-scoped runtime query remains
  public.
- Remove `TimerPolicy::cadence_ns`, `TimerCounters::provider_arms`, the public
  completion-partition checker, and public helpers on nested directive/runtime
  state. Consumers can use the typed cadence, separate committed arm counters,
  and coherent top-level snapshot projections directly.
- Remove the unreachable public `ScheduleError::MissingCadence` variant.
  Missing recurrence cadence is an illegal internal policy/directive pairing,
  not an error a consumer schedule request can produce.

## [0.3.8] - 2026-08-15

### Changed

- Record Canic's validated uncommitted exact-0.3.6 hard cut: one shared
  inventory now replaces its provider, registry/control state, timer counters,
  and timer-specific performance storage, while combined Canic+IcyDB
  qualification remains blocked on exact patch alignment.
- Clarify that Canic's removed global `TimerScheduled` counter was test-only,
  not a public parity surface; the maintained per-timer `schedules` field maps
  to committed `wakeups_armed`.
- Distinguish initializing an empty shared registry from declaring
  framework-owned jobs, allowing Fleet Coordinator to participate without
  inventing inactive Canic timers.
- Stop freezing Canic's maintained adoption status to one lifecycle state;
  release truth now requires only one structural status marker and does not
  interpret its prose.

## [0.3.7] - 2026-08-15

### Changed

- Make the crate hierarchy explicit: the crate root remains the convenience
  facade, `schedule` and `snapshot` remain public value groupings, and the
  control, registry, runtime implementation, and provider boundary remain
  private.
- Use defining-module imports internally, share ordinary callback erasure and
  duration validation, and document why atomic registry/runtime transitions
  remain cohesive.
- Correct lifetime, scheduling-mode, provider-arm, and work-count comments so
  observations do not overstate retained authority or delivery guarantees.
- Run full hosted Rust/MSRV validation once on pull requests and `main`; tag
  pushes now run only release-truth, exact-tag, and main-ancestry checks instead
  of duplicating the same builds at one commit.
- Add a pre-bump compact-status prose advisory for wording likely to become
  stale at release. It warns before expensive validation and always fails open;
  structural post-mutation release truth remains the only enforced boundary.

### Removed

- Hard-cut the accidental public `TimerFuture` erasure alias. Ordinary
  registration APIs continue to accept any compatible future; callback
  erasure is now entirely internal.

## [0.3.6] - 2026-08-14

### Changed

- Update the maintained IcyDB record: tagged IcyDB 0.226.1 adopted exact
  `ic-timers` 0.3.4, while its validated post-tag integration upgrades to
  0.3.5 and uses claim-scoped armed-wakeup observation without check-then-arm
  control flow.
- Keep release-truth validation structural: exact Cargo, changelog, release-note,
  and compact-status version markers remain enforced without interpreting
  free-form status prose or blocking a finalized version bump.

## [0.3.5] - 2026-08-14

### Added

- Add claim-scoped `has_armed_wakeup` observation to all three registration
  capabilities. The result reflects ownership of the exact future provider
  wake-up handle without making snapshots or observations into scheduling
  authority.
- Add a maintained IcyDB adoption record for its accepted 0.226.1 candidate,
  including the exact dependency, shared-registry inventory, recovery
  evidence, and measured costs.

## [0.3.4] - 2026-08-14

### Changed

- Hard-cut lifecycle reconciliation to retained declarations. The three
  reconciliation helpers no longer accept a declaration lifetime;
  `RemoveWhenStopped` callbacks remain available through direct registration.
- Make runner-executed release and provider-boundary checks portable without
  requiring `rg` on GitHub-hosted runners.

### Fixed

- Add policy-wide provider-binding fault evidence for initial and replacement
  arms, after-completion, partial watchdog binding, effect confirmation, and
  remove-on-stop cleanup.
- Finalize and validate mutable release truth mechanically, including the
  workspace status and version-specific release note, while keeping README and
  the Canic contract version-neutral.

## [0.3.3] - 2026-08-14

### Fixed

- Bind each consumer-work `TimerContext` to its exact callback generation and
  role. Context mutation remains available during nested work, while a context
  retained after completion expires and cannot control a successor or later
  registration.
- Fail closed when a public registration or lifecycle operation cannot bind
  its emitted provider effect. Newly armed and replaced handles are cleared,
  retained declarations become inactive with `ProviderBindingFailed`, and a
  later explicit ensure can recover without a false scheduled snapshot.

### Changed

- Reject explicit release versions that are equal to or lower than the
  workspace version, reject leading-zero SemVer components, and resolve tag
  collisions against the exact tag namespace.

## [0.3.2] - 2026-08-14

### Changed

- Correct the Canic adoption contract: all five dynamic built-ins are retained
  `Once` declarations, root canister-pool maintenance is retained
  `AfterCompletion`, public/lifecycle timers keep remove-on-stop lifetimes, and
  no Canic timer uses `Watchdog` yet.
- Permit one bounded Canic claim-custody collection for quiescence while
  forbidding it from duplicating scheduling state, counters, provider handles,
  generations, pending commands, or reconciliation authority.
- Freeze exact Canic metric projection: schedules use committed
  `wakeups_armed`, executions use `work_started`, successes combine successful
  and no-work completions, stale roles combine explicitly, latest delay uses
  the armed delay, and generation remains optional.
- Remove blanket dead-code suppression from the private platform and registry;
  pure registry fixture helpers are now compiled only for tests.

### Fixed

- Retain freshly reconciled inactive `Once`, `AfterCompletion`, and `Watchdog`
  declarations in the canonical inventory so fixed owners reserve bounded
  capacity before application hooks.
- Bring README, status, adoption, and dedicated release notes in line with the
  tagged and published 0.3.1 release.

## [0.3.1] - 2026-08-14

### Added

- Add exact `Once` and `AfterCompletion` schedule reconciliation, including
  the missing synchronous `reconcile_once` lifecycle helper, with focused
  deadline replacement and nested-request tests.
- Freeze a one-page Canic adapter contract covering identity and policy
  mapping, fallible facade changes, claim-scoped lifecycle composition,
  metrics projection, dependency unification, and the pre-1.0 hard cut.

### Changed

- Gate every release bump on normal CI, Rust 1.88, all supported probe lint
  configurations, the full watchdog PocketIC suite, and comparable policy
  cohorts; fail immediately and before version mutation unless PocketIC is the
  exact audited 15.0.0 binary.
- Automatically install the pinned PocketIC artifact into the ignored
  repository tool cache when no override is supplied, while retaining strict
  version/hash validation and never replacing an explicit override.
- Update and stage the nested `testing/Cargo.lock` during version bumps, then
  perform cheap locked-metadata checks for both workspaces without rerunning
  the already-completed behavioral evidence suite.
- Enforce in normal CI that `ic-cdk-timers` remains used only inside the
  private `platform` module and is wrapped rather than publicly re-exported.
- Treat every superseding pre-1.0 contract change as a hard cut by default,
  without deprecated, dual, fallback, or feature-gated legacy paths.
- Mark all three sole-owner registration capabilities `must_use` so accidental
  loss is diagnosed at compile time.
- Make the registry the sole owner of pending ordinary commands and scheduling
  metadata; private `TimerControl` now receives the registry's already
  arbitrated completion decision instead of maintaining a parallel pending
  machine.
- Explicitly keep Canic log-retention and cycle-top-up work on the ordinary
  after-completion path, reserve `Watchdog` for pre-armed recovery work, and
  reject a shared global suspend switch in favor of owner-specific claims.

### Fixed

- Trap callback messages on unexpected internal completion, accounting, or
  cleanup failures instead of silently discarding them; watchdog work thereby
  relies on message rollback and retains its earlier committed successor.
- Drive IcyDB-shaped retryable and terminal results through real scheduler and
  work callbacks, verifying continuation, successor ownership, truthful
  counters, and terminal clearing.
- Describe the published runtime as 0.3 rather than "unreleased 0.3" in crate
  documentation.

## [0.3.0] - 2026-08-13

### Added

- Add the 0.3 production runtime, centered on a bounded shared registry and a
  two-message watchdog that commits its successor before fallible consumer
  work.
- Freeze the 0.3 Patch 1 runtime contract: a 64-entry canonical registry,
  policy-specific callback and state models, linear provider ownership,
  truthful dispatch and completion counters, lifecycle composition, and
  comparable Wasm and instruction measurement subjects.
- Add the pure fixed-capacity registry engine with unique identity claims,
  deterministic inventory ordering, retained and remove-on-stop lifetimes,
  checked generation and request arbitration, and policy-specific ordinary
  and watchdog transitions.
- Add focused registry tests for the complete directive/decision matrix,
  duplicate and capacity failures, idempotent ensure and cancellation,
  claim-consuming unregistration, stale generations, nested commands,
  unacknowledged watchdog dispatches, and coherent Canic-shaped projection.
- Add one canister-local volatile runtime with a synchronous, idempotent
  initialization seam, callback-owning `Once` and `AfterCompletion`
  registrations, exact-claim control, and bounded live snapshot access.
- Add a native one-shot provider harness and focused runtime tests for linear
  handle replacement/cancellation, execution without a registry borrow,
  after-completion recurrence, nested callback commands, and declaration
  lifetime.
- Add the live synchronous `Watchdog` registration and its two-message
  one-shot protocol: a bounded scheduler owns the next cadence successor and
  queues separate immediate work before returning.
- Add owner-local watchdog execution tests plus an isolated PocketIC probe
  proving that an explicitly trapped work message leaves the previously
  committed successor able to retire the attempt and make later progress; the
  probe also verifies rejection of external CDK timer-executor ingress.
- Add synchronous, idempotent reconciliation helpers for retained
  after-completion and watchdog claims, with explicit scheduled/inactive
  desired state and configuration-conflict rejection.
- Populate normally completed scheduler/work instruction aggregates from IC
  call-context counter type 1, and add an IcyDB-shaped fixture covering
  consumer-owned readiness, one-page work, terminal stop, and commit-guard
  re-enablement.
- Extend the PocketIC probe through upgrade, proving a fresh runtime can
  reconstruct a scheduled watchdog from consumer-owned durable authority
  before the probe's downstream-hook observation and then make progress.
- Complete the PocketIC watchdog promotion matrix with real instruction
  exhaustion, insufficient-cycle deferral and top-up, stop/resume, overdue
  coalescing, scheduler/work-gap cancellation, duplicate demand, simultaneous
  timer isolation, commit-window ensure, and a measured ordered 64-entry
  inventory.
- Add mutually exclusive baseline, Once, after-completion, and Watchdog probe
  cohorts with reproducible Rust 1.88 raw Wasm, instruction, and PocketIC cycle
  measurements; document exact hashes, deltas, complexity, and downstream
  adoption work in the 0.3 evidence report.

### Changed

- Lower the declared minimum supported Rust version from 1.91.0 to 1.88.0
  after proving the current source and locked dependency graph compile for
  both native and `wasm32-unknown-unknown` targets.
- Hard-cut the pre-1.0 value model to validated positive cadence, configured
  after-completion recurrence, closed policy-specific states, private inert
  snapshots, truthful unacknowledged outcomes, split scheduler/work counters
  and instruction aggregates, and no fabricated elapsed-time field.
- Make the provider module private and its timer handle linear. Actual provider
  binding for ordinary timers now stores each returned handle in its canonical
  entry and increments actual-arm counters only after that ownership commits.
- Read IC time and canister version through the private platform boundary using
  the already-resolved exact `ic0` 1.1.0 dependency; retain the exact
  `ic-cdk-timers` 1.0.0 provider pin.
- Extend each watchdog entry's linear ownership to at most two exact provider
  handles: one cadence successor and one dispatched work callback. Terminal
  decisions clear both as applicable, while normal continuation retains the
  already-armed successor.
- Project a retained timer that stops on a retryable failure as `failed`,
  preserving Canic's current operator condition instead of reporting it as
  generically idle.
- Clarify that watchdog trap/exhaustion recovery protects the consumer-work
  message, freeze IcyDB's exact returned-result mapping, and require downstream
  dependency unification plus a canister-wide direct-provider inventory.
- Freeze Canic's adoption boundary: its infallible timer facade must expose
  typed registry-capacity and identity-bound failures rather than panic,
  evict, truncate, or silently ignore them.
- Stage named release notes under an explicit undated version heading and
  teach the release helper to add the date automatically during the version
  bump.

## [0.2.0] - 2026-08-13

### Added

- Add the candidate 0.2 provider-neutral `snapshot` API, including bounded
  structured timer identities, policy and scheduling state, outcomes,
  counters, performance measurements, runtime epochs, and one canonical
  snapshot type.
- Preserve operational distinctions required by Canic migrations: callback
  starts versus completions, completed versus interrupted work, requested
  schedules versus platform arms, and functional consecutive expected-failure
  state.
- Add saturating counter transitions, explicit observation resets, portable
  directive conversion, and total/latest/maximum instruction and elapsed-time
  aggregates.
- Add focused tests for identity validation and ordering, outcome transitions,
  counter partitions, absent measurements, epoch resets, and a Canic-shaped
  lossless projection fixture.
- Add the 0.2 observability and Canic parity contract, an explicit safety
  boundary, and a recurring code-hygiene audit.
- Add Dependabot coverage for Cargo and GitHub Actions dependencies.
- Add tested release automation that promotes `Unreleased` notes into a dated
  version section, plus a guarded crates.io publish target.

### Changed

- Rework the README to identify `ic-timers` as a wrapper around
  `ic-cdk-timers` and explain the shared coordination and recovery rationale.
- Harden the development gate with expanded Clippy lints, rustdoc, shell
  syntax, and pinned GitHub Actions checks.
- Mark public validation errors as non-exhaustive and document the pre-1.0
  public API compatibility policy.

### Not yet implemented

- Registry storage and live inventory population, measured callback execution,
  runtime adapters, lifecycle reconstruction, and recovery watchdog behavior
  remain future work.
- The candidate API remains subject to Canic and IcyDB review; a real downstream
  Canic adapter test is required before the 0.2 contract is accepted.

## [0.1.0] - 2026-08-13

### Added

- Initial workspace, CI, release helpers, and formatting hook.
- Canic-derived deterministic timer control and one-shot platform boundary.
- Architecture and release-line documentation for the planned shared runtime.

### Changed

- Use Rust 1.97.1 for normal development while retaining Rust 1.91.0 as the
  separately checked minimum supported Rust version.
- Document the staged path from public timer snapshot types to the registry,
  lifecycle integration, and recovery watchdog.
