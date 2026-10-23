![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-23

## Source and accepted work

Released base remains **0.30.7**, `89a2f87`; reuse the undated **0.30.8** draft.
Incoming Shared 0.26.9, Platform/macros 0.6.11 and Wire/derive 0.6.10 adoption is
preserved. The incoming lock also selects the latest published Build/Host/Testkit
**0.66.2**. Root requirements already admit these selections. No product Rust,
manifest, protocol, limit, generator or feature change is needed; the incoming
Cargo.lock and every unrelated pending file remain intact.

Official registry checks confirm current direct selections: Metrics **0.15.7**,
Platform/macros **0.6.11**, Wire/derive **0.6.10**, Build/Host/Testkit **0.66.2**,
Clap **4.6.7**, Serde **1.0.229** and JSON **1.0.154**. Thiserror is the one
blocked direct patch update, described below. No blanket transitive upgrade or
unpublished sibling source is selected. The Rust floor remains **1.91**.

The five published Tooling archives match registry checksums and compiled-cache
files at source `4bc810c84ba45baa191030eb625b8e793124581e`. Host and Testkit
Rust sources match 0.66.0. Build changes only its executable examples to standard
Clap argument/help/version handling; library/editor implementation is unchanged.
Matching Host package identities still matter at the injected OwnedChild boundary.

## Thiserror constraint

The latest direct-compatible release is **2.0.21**, MSRV 1.77. Its official
release notes fix generic-unit-variant display parsing. Published source was
inspected, but the targeted Cargo update refuses because PocketIC **16.1.0**
declares **thiserror = "=2.0.18"**. Cargo cannot select both compatible patch
identities in this workspace. The failed attempt changes no lock entry.

Retain **2.0.18** and its matching implementation. This is a transitive simulator
constraint, not a new local pin; root `thiserror = "2.0"` is unchanged. Do not
fork the SDK, split the root catalog, patch registry manifests or replace error
implementations just for this unused patch fix. There is no demonstrated timer
error defect. The official archive/release notes and precise resolver failure
are retained with this task's evidence; no claim of adopting 2.0.21 is made.

## Existing ownership and admission

Timers owns scheduling/claims/observations, Metrics arithmetic, Platform IC
bindings/client identities, Wire codecs, Testkit simulator lifetime and Build/Host
artifact/process custody. Applications retain durable intent/lifecycle policy.
Native tests retain each PocketIcInstance owner through borrowed client teardown;
Testkit's optional Wire helpers remain disabled. Protocol records use Wire/Serde
without Platform/runtime authority. No compatibility layer or packed record is added.

The library keeps only Platform executor/IC0, one registry executor **2.0.0** and
IC0 **1.2.0**. Native/Wasm graph policy remains unchanged; deployed library/probe
graphs exclude Candid/full CDK and Host tools. Candid/management remains native-only
through Testkit/PocketIC. No storage/memory migration is included.

Shared **0.26.9**, `3657c044c0e91321effe8835989491a4cd0b5cba`, and its pending
snapshot refresh are preserved. Existing common Cargo tools remain valid; the
new lock-selected Build/Testkit CLIs are prepared via the existing explicit
installers with `cli`. Prior tool generations and setup evidence are retained.
No new installer, source cache, retry mode or generic producer gate is introduced.

## Current qualification

Focused Linux checks pass: full locked metadata and declaration/feature/graph
policy, library Clippy, Rust 1.91 driver/protocol/probe lint and all four size
features, grouped formatting, native/Wasm library MSRV, two driver process/build
tests, one report test, CLI adapters and metadata/version fixtures. Both new CLI
selections are prepared and pass offline tool/server admission. Live qualification
passes 27 product subjects plus three host fixtures, all four policy cohorts and
both calibrations. The new receipt matches the incoming lock; library/coordinator
normal/build graphs remain 14/53 package identities.

Before-images, official indexes/archives, source comparisons, blocked patch
attempt and qualification logs: `/tmp/ic-timers-new-deps-thq4p0u8/`. The previous
IC codec/Shared adoption evidence remains at `/tmp/ic-timers-ic-refresh-m18eo8ed/`;
its runtime receipt has the older Tooling lock context. Native macOS, installed
IC and the complete maintainer-owned release gate remain separate. No package
version changes, staging, commits, pushes or releases are performed. Siblings
remain read-only; no performance claim is inferred across changed lock contexts.
