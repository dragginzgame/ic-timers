![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-23

## Source and accepted work

Released base: **0.30.6**, `23c87a4`; package versions remain unchanged. The
undated **0.30.7** draft qualifies incoming Platform/macros **0.6.10** and Metrics
**0.15.7**. No local manifest, import, Rust source, protocol, generator or feature
migration is needed. One root catalog/lock still owns five maintained packages;
the incoming lock is preserved byte-for-byte.

Official registry indexes confirm both latest non-yanked releases, requiring
Rust **1.91**, matching Timers' existing floor. Three archive checksums match
registry/lock and compiled-cache files. Platform/macros source is
`429ce3d56ad124ad17e2bc3920e22a4c2cf288e0`; Metrics source is
`69dcae8b52defbe834dc970cfb8f98cd3eeabcfc`. Every Rust file is byte-identical to
released selections 0.6.9 and 0.15.6 respectively; no Rust source files disappear.
Platform's only dependency change is the optional native Wire metadata minimum
from 0.6.4 to 0.6.7, plus coordinated macro selection. That feature is not enabled
here. Public measurement APIs, arithmetic and timer semantics remain compatible;
no downstream minor bump, data reset or source adaptation is required.

## Retained dependency and ownership boundaries

The library selects only Platform executor/IC0, retaining exactly one registry
executor **2.0.0** and IC0 **1.2.0**. Independent native/Wasm policy passes; the
library normal/build graph remains **14** package identities, excluding Principal,
Serde, crypto, Wire, Candid/full CDK and native tooling. The native build
coordinator remains **53**, with simulator/codec dependencies dev-only.
Protocol records select Wire/Serde without Platform/runtime authority.

Metrics owns measurement arithmetic; Timers owns attribution, scheduling, claims
and observations. Platform owns IC bindings/client identities; Wire owns codecs;
Testkit owns server admission and lifecycle; Build/Host own artifact/process
custody. Applications retain durable intent and lifecycle policy. Native provider
fixtures and PocketIC keep Candid/management support outside deployed graphs.
No memory/storage feature, migration, compatibility adapter or runtime fork is added.

Build/Host/Testkit remain **0.66.0**, with shared-client borrowing through
`PocketIcInstance::instance()` and retained owners through teardown. Wire/derive
remain **0.6.7**. Shared **0.26.8** at
`0b07c2a3163a5a66328e100b553baf93afd8ed07` and its snapshot are unchanged.
Previously prepared Cargo tools/CLIs still pass offline admission; this library
selection change triggers no new tool installation or server provisioning.

## Current qualification

Focused Linux checks pass: full locked metadata, declaration/feature/graph
boundaries, library Clippy, Rust 1.91 protocol/driver/probe lint and all four size
features, grouped formatting, offline tool admission, native/Wasm library MSRV
builds, six Metrics projection tests and the 7,776 generated native command traces.
The native substitute proves no IC recovery guarantee.

Live qualification passes 27 product subjects plus three host fixtures, all four
policy cohorts and both calibrations. The new receipt matches the incoming lock.
Native macOS, installed IC and the complete maintainer-operated release gate
remain separate. No
cross-context performance improvement is inferred from graph counts or cohort data.

Incoming before-images, official indexes/archives, source/manifest comparisons
and focused logs: `/tmp/ic-timers-platform-metrics-1hvmn8g1/`. Prior Host/Shared
migration evidence remains at `/tmp/ic-timers-host-refresh-qlwtv9kw/`; its lock
context is older. No package version changes, staging, commits, pushes or
releases are performed; sibling source remains read-only.
