![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.5**, `8b91bf0`. The undated **0.31.6** section includes
Shared Tooling **0.26.21** and the accepted complexity cleanup. The batch preserves
published timer APIs and semantics, so the next patch remains appropriate.
Cargo package versions remain 0.31.5. Commits, staging, pushes and release
execution remain maintainer-owned.

Incoming `Cargo.lock` and `scripts/ci/test-repository-checks.sh` edits remain
byte-identical to their before-images. Focused checks below use that selected
graph (including Build/Host/Testkit 0.68.9); they are not a separate broad
upstream-dependency audit or release qualification.

## Completed cleanup and ownership

Fresh lifecycle creation shares one private reconciliation transaction. Ordinary
directives produce executable schedules and inert observations directly, with
unchanged error precedence. Negative lifecycle tests share preservation and
original-callback assertions while keeping distinct policy and stale-claim cases.

The unpublished protocol owns named ordinary/all-policy and retention selectors,
replacing numeric and boolean policy arguments. Probe and host callers migrate
together; rebuild both sides. It still depends only on Wire/Serde and owns no
runtime authority. A private cohort enum owns names, features and Wasm environment
selection; measurement and report consumers use it without changing schema 2.

CI omits redundant library `check` dispatch while retaining lint and executed
tests. Dependency fixtures derive copy/reset membership from admitted root
workspace metadata. Shared's canonical same-revision exporter retires its generic
evidence-test payload and root; the manual workflow keeps Timers' own adapter
regressions and downloaded-artifact admission. The snapshot now has **135**
payloads at `66c91c4412856baa147482e648fc80428db8b179`.

## Upstream boundary and deliberate retention

[IC Tooling #255](https://github.com/dragginzgame/ic-tooling/issues/255) requests
borrowed record access from published `WasmBuildEvidence`: Timers needs artifact,
compiler and recipe facts for cohort admission. Keep the existing retained-build
and freshness checks until a supplying API is published; no duplicate build or
local wrapper was introduced.

The proposed Wire binding generator is not pursued. The delivered
[Platform #31 discussion](https://github.com/dragginzgame/ic-platform/issues/31)
explicitly retains consumer codec hooks; published Platform 0.6.19 and Wire
0.7.17 support that boundary. Both tiny adapters, pre-copy guards and explicit
entrypoint bindings remain. No new generator framework or runtime is justified.

## Verification and limits

Evidence and incoming before-images are in `/tmp/ic-timers-cleanup-0316.GVZnfK/`.
Rust 1.91 strict Clippy passes for library, runtime probe, protocol and native
runner normal/test targets. All 33 focused native lifecycle, schedule, directive,
callback, build/report and protocol tests pass. Optimized runtime-probe Wasm
compilation and ten selected real-Wasm Linux PocketIC tests pass, including
unknown selector rejection before effects, failed initialization, capture cleanup,
ordinary suspension/traps and Watchdog upgrade recovery. Four optimized cohort
builds and the PocketIC cohort subject (four policies/two calibrations) pass;
the validated receipt is `cohorts.json`. No baseline was replaced or speedup claimed.

Dependency-boundary, repository-boundary, snapshot corruption, evidence-reader and
release-ordering fixtures pass. Formatting, snapshot integrity, provider ownership,
changelog and diff checks pass. Initial lint findings were fixed; both failed
and passing attempts remain retained. Earlier evidence remains at
`/tmp/ic-timers-adoption-0315/` and `/tmp/ic-timers-platform-wire-0314/` with its
original scope. Full release validation, native macOS and live deployment remain
separate; Linux simulator evidence does not qualify those environments.
