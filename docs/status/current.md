![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-23

## Source and accepted work

Released base: **0.30.5**, `de0e9a5`; package versions remain unchanged. The
undated **0.30.6** draft adopts Build/Host/Testkit **0.66.0** and reviewed Shared
Tooling **0.26.8**. This is compatible at the published Timers boundary: only
unpublished native test access changes. Public timer types, probe records/IDs,
codec limits, callback/trap semantics and timer ownership are unchanged.
There is one root dependency catalog/lock and five maintained packages.

Official registry indexes and checksum-verified archives establish all five
Tooling packages at 0.66.0, source `d71f4690c7e8e3ffe99f72467bc35cbadce33042`.
Every compiled-cache file matches its archive. Only these five packages change
from the incoming lock; Wire/derive **0.6.7**, cc **1.10.0**, certificate roots
**1.0.11** and all unrelated selections are preserved. Platform/macros remain
**0.6.8**, Metrics **0.15.6**, executor **2.0.0**, IC0 **1.2.0**. Host and Build
Rust sources match 0.65.0; matching package identities still matter at their
injected `OwnedChild` boundary. No unpublished sibling implementation is selected.

## PocketIC ownership migration

Testkit makes client/server fields private. All 28 watchdog-test and two cohort
accesses now use `owned.instance()`, returning the same shared client reference.
Owner bindings and scopes remain unchanged, so each managed server is retained
through client teardown. There are no direct field accesses, client extractions,
borrowed-server constructions or ownership-transfer adapters to migrate elsewhere
in this harness. No `into_parts` call, cloned owner or lifecycle shim is added.

Platform retains client/DTO identities; Testkit owns startup/admission, instance
lifetime, installation extensions and bounded progress. Existing raw PocketIC/
native Wire calls retain error handling and explicit method translation. Optional
Testkit Wire helpers are not selected. The library retains executor/IC0-only
Platform selection; protocol records select Wire/Serde without runtime authority.
Candid/management remains native-only through Testkit/PocketIC; deployed library/
probe graphs exclude Candid, full CDK and Host tools. No storage/memory migration.

## Shared Tooling refresh and preparation

Canonical preview/apply retains the 134-payload selection at reviewed revision
`0b07c2a3163a5a66328e100b553baf93afd8ed07` (Shared 0.26.8). Only the store helper,
Cargo build/source helper and generic setup guide change. All exported payloads
verify exactly; no generic suite, optional tool bundle, audit or schedule returns.

The installer tolerates record cleanup only for the same admitted physical lock,
rechecking inputs after release and retaining timeout/invalid-owner failures.
The default selected-tool setup wait remains 60 seconds, configurable to 0–300;
checks/preflight stay immediate. No owner lock is removed. Source-cache bootstrap
now uses admitted idempotent private creation and Cargo's download serialization,
retiring the global source-preparation lock. Old slots and generations remain.

The changed build policy requires explicit setup. Common Cargo tools, the root-
lock Build editor and Testkit CLI were prepared through existing targets; CLI
selections retain `cli` and library dependencies remain CLI-free. Live contention
passed through two admitted owners, then release/revalidation completed after
nine seconds. Old selections and all diagnostics are retained. A one-time
isolated owner fixture covers cleanup races, invalid claims, timeout, interruption
and atomic-acquisition failure against the exact adopted helper.

## Current qualification

Focused Linux checks pass: full locked metadata, declaration/feature/graph
policy, library Clippy, Rust 1.91 driver/protocol/probe lint and all four size
features, grouped formatting, native/Wasm library MSRV, CLI adapter and snapshot
corruption fixtures. The focused owner wait fixture and ShellCheck/whitespace
checks, common-tool/CLI setup, offline admission and hook/index/metadata/version
fixtures pass. Live qualification passes 27 product subjects plus three host
fixtures, all four policy cohorts and both calibrations. The new receipt matches
the final lock; library/coordinator normal/build graphs remain 14/53 identities.
No cross-context performance improvement is inferred. Native macOS, installed IC
and the full maintainer-operated release gate remain separate.

Incoming copies, official indexes/archives, source comparisons and qualification:
`/tmp/ic-timers-host-refresh-qlwtv9kw/`. Prior tool/codec compatibility evidence
remains at `/tmp/ic-timers-upstream-refresh-zs_qs4gq/`; its cohort lock context is
older. No package version changes, staging, commits, pushes or releases are
performed. Sibling source remains read-only.
