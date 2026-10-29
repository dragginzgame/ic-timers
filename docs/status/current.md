![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-29

## Source and accepted work

Local main is **0.31.16**, `a7ae6ae34f93c9a64daaf372149323e01bb7919d`.
The maintainer requested latest published Wire, Metrics, Platform and IC Tooling.
Only Cargo.lock was dirty on entry. Preserve its incoming AWS LC 1.19.0/sys
0.47.0, Platform 0.7.4 and Wire 0.7.21 selections. This task updates the catalog
and raises the incoming Metrics 0.16.7/Tooling 0.69.8 selections below.
No staging, commits, pushes, package version mutation, release commands or sibling
edits ran. Cargo package versions remain 0.31.16. The undated **0.32.0** draft is
required because public Metrics Rust type identities change; this is not a patch.

## Published dependencies and ownership

Official crates.io index records and checksum-verified archives establish the
selected releases; Rust 1.91 remains supported. Evidence/source reviews are in
`/tmp/ic-timers-deps-03117._b1jgizn/` (its scratch name predates minor selection).

| Owner | Catalog and resolved selection | Published source revision |
| --- | --- | --- |
| Metrics | **0.17.0** | `c45bb5acd680ab1933c98b1e7c9fcd966a11eaff` |
| Platform and macros | **0.7.4** | `ca2ac1eb3b65a4e3adec2eb64f06bf942ca22af9` |
| Wire and derive | **0.7.21** | `15c8b7ac881c008b2af0bd2c3d79502605a36d0d` |
| Build, Host Artifacts/FS/Process, Testkit | **0.69.9** | `ce660e30e20ec407baf446fc7c4f2b7587779761` |

The library still directly selects only Metrics, Thiserror and Platform
`executor`/`ic0`. Metrics 0.17 makes validated Serde transport mandatory, plus
Wire TypeScript metadata on native hosts. Counter, aggregate and summary
arithmetic source remains unchanged from the previously selected 0.16.3.
Consumers exchanging those public types with direct Metrics dependencies must
select 0.17; no local aliases or compatibility path were added. Optional heap
recording is unselected. Storage/memory remain out of scope.

Graph policy now admits Metrics' Serde and native Wire path while retaining
Wasm exclusion of Wire/Postcard/JSON parsing and the native coordinator's codec
and simulator exclusions. Existing dev-only library Wire use remains admitted;
renamed target/build dependencies and Platform feature leaks remain rejected.
A new real-Cargo negative fixture covers JSON leakage after Serde admission.
Deployment probe graphs also contain no JSON parser. The library and probes
remain Candid/full-CDK-free. Candid 0.10.39 remains in the native test worker's
PocketIC transport graph. Executor **2.0.0** and ic0 **1.2.0** remain singular
Platform-owned identities. Platform's selected CDK/executor/IC0 implementations
and Wire derive implementation are unchanged; the eight-byte internal dispatch,
framed probe protocols, resource limits and callback/trap contracts remain intact.

Tooling 0.69.9's complete baseline-snapshot restoration fix is supplier-owned;
Timers uses fresh Testkit instances, not custom snapshot recipes. The canonical
editor/Testkit setup and offline admission select 0.69.9; Testkit still owns
server **16.1.0**. The Build 0.69.6 macOS mode correction is retained, with no
local workaround or exact-pin exception. Shared **0.27.3** is unchanged; this
request selected IC Tooling. Thiserror remains **2.0.18**.

## Earlier issue handoff (not re-triaged during this adoption)

- **#77 is closed**: 0.31.15 delivered the required installed queued-scheduler
  stop/restart regression. Fresh qualification on the final graph passes all six
  initial/successor and owner-mode trajectories, preserving registration identity,
  completed work and unacknowledged state. Earlier causal negative control remains
  at `/tmp/ic-timers-scheduler-pressure.YxSpot/`. Scheduler-body normal return is
  still the protocol assumption; arbitrary traps/permanently live futures are not
  qualified. This closure grants no uncertain-effect replay authority.
- **#78/#80 are closed**: global-timer conflict detection and the application-call
  ownership boundary are delivered. Detection retains its documented limitations.
- **#79 remains open** for Canic consumer delivery. Its ingress/cancellation patch
  is prepared and qualified at `/tmp/ic-timers-backlog-03114.GlZdEf/canic` with
  `canic-code.patch` in the parent directory. Original Canic main is still
  `5c236ab130c0d63892ef281b45bdc3fa353e012f`; its dirty working checkout was untouched.
  Canic #695 likewise awaits delivery; #688/#689 were already closed.
- **Canic #700's supply blocker is resolved**: published **ic-call 0.7.2** owns
  dynamic method/argument custody over Platform 0.7.2. A registry-only standalone
  witness passes Rust 1.91 native execution and Wasm compilation, with singular
  Platform/executor/ic0 and no Candid/full CDK. It is not added below Timers.
  The public Call/CallBuilder/CallResult retirement is a breaking minor; Canic's
  delivery-cadence Human Minor Closeout Gate requires a maintainer-requested and
  accepted 0.112 closeout before next-minor implementation. This exact blocker
  and the witness are recorded on #700. Platform #75 also records publication
  and its migration document's stale unpublished-supplier sentence. No new
  compatibility builder or mirror waiting issue was introduced.

## Current qualification

Evidence: `/tmp/ic-timers-deps-03117._b1jgizn/`. Passes: locked/offline metadata;
strict Rust 1.91 library/runtime-probe/protocol/coordinator Clippy; six Metrics
forwarding/overflow/role tests; wide completed/stale snapshot integration;
all three build-owner native tests; library-only MSRV Wasm; canonical editor and
Testkit setup/admission; version preflight/rollback/dirty-worktree preparation;
formatting; direct IC/pin/provider checks; native/Wasm transitive/feature/provider
identity exclusions; updated positive/negative dependency fixtures; changelog and
diff checks. The library/probe target trees separately confirm no Wasm JSON,
PocketIC or Candid paths. No product runtime implementation changed.

A disposable registry-only Build 0.69.9 consumer built the actual optimized runtime
probe and held its evidence through four installed PocketIC subjects: all six
queued-scheduler stop/restart trajectories, malformed Wire admission,
ingress/foreign-call authentication and response-trap cleanup. All passed;
source/build evidence remained current throughout. Retained Wasm SHA-256:
`6531c5535241a7afcfb987c61168434d3c504469ecc73ad2a05f2664deea9424`.
The first temporary witness attempt lacked its diagnostics parent and stopped
before server startup; its failure is retained separately. Creating that scratch
directory fixed the witness; no repository behavior was changed for the retry.

Fresh native macOS, full local release/cohort gates and live IC were not run.
The retained focused evidence does not replace normal maintainer-operated
release and host requirements. Older evidence retains its original source/graph
scope at the paths recorded in previous handoffs.
