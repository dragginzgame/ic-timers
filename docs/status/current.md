![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-25

## Source and pending work

Released base: **0.31.1**, `6dd07a1`. The accepted complexity cleanup is released.
The undated **0.31.2** section covers compatible dependency qualification and
Shared Tooling adoption. Public timer APIs, runtime semantics, probe records and
MSRV remain unchanged; no minor release is required for this batch. Cargo package
versions remain 0.31.1. Commits, staging, pushes and releases remain user-owned.

The incoming lockfile already selected published Metrics **0.16.2**,
Build/Host/Testkit **0.68.2** and Wire/derive **0.7.9**. It is preserved byte for
byte. Registry metadata and checksum-verified published archives confirm those
selections. Platform/macros **0.6.16** is now published and was already selected
in the incoming lockfile for the subsequent Platform adoption. Clap **4.6.7**, Serde
**1.0.229** and Serde JSON **1.0.154** are current. Thiserror **2.0.18** remains
required by PocketIC 16.1.0's exact constraint despite newer 2.0.21 publication.

## Ownership and adoption

The library still depends directly only on Metrics, Platform and Thiserror.
Platform retains executor **2.0.0** and IC0 **1.2.0** identities. Library and probe
deployment graphs remain Candid-free; the native Testkit/PocketIC test transport
intentionally retains Candid. The normal host coordinator excludes that transport.
The protocol package owns Wire/Serde records only. Timer policy, authority,
provider cleanup and callback recovery remain local.

Metrics and Host Rust sources match their previous selections. Wire changes are
in unselected TypeScript tooling; derive sources and runtime codecs are unchanged.
Build refactors metadata editing and shared-incremental acquisition; Timers keeps
isolated builds. Testkit consolidates startup/fixture internals without changing
its selected PocketIC 16.1.0 server. Both root-lock-selected 0.68.2 CLIs have been
explicitly prepared through their existing adapters and admitted offline.

Platform 0.6.16 preserves structured Principal deserializer errors and refactors
entrypoint validation/emission. Published archive review confirms unchanged
executor/IC0 bindings and feature declarations; probe codec hooks, export names,
guard ordering and executor-context selection are preserved. No local Rust or
manifest changes were needed. The root range remains compatible, and the pending
0.31.2 notes cover this adoption without a public timer contract change.

Shared Tooling **0.26.16**, reviewed revision
`2077c934efb82b181bea58fe50078dbad289c1aa`, supplies **135** payloads through its
canonical exporter. Existing roots and producer-fixture retirements are preserved.
The added RPC-budget rule does not introduce a browser client or product gate.
Local adapter fixtures now verify propagation of busy admission status 75.
Optional read-only waits are not enabled in local adapters. CI caching stays off;
all release, native-host and PocketIC recovery/cohort gates remain required.

## Verification and limits

Latest Platform evidence and the incoming lockfile are retained in
`/tmp/ic-timers-platform-refresh-0312/`. Rust 1.91 Clippy passes for the library,
both probes and host runner; library Wasm checking, runtime-probe compilation,
all four size-probe Wasm feature checks and the native runtime-probe unit test
pass. Four Linux PocketIC tests pass: typed Wire rejection, trapped-work successor
and executor privacy, shared-watchdog trap/upgrade recovery, and ordinary async
await with concurrent ingress. Formatting and direct/deployed dependency and
provider boundaries pass. The selected graph contains one Platform 0.6.16,
executor 2.0.0 and IC0 1.2.0; native test transport Candid stays outside deployment.

Earlier batch evidence and its incoming lockfile are retained in
`/tmp/ic-timers-adoption-0312/`. Rust 1.91 Clippy passes for all five affected
packages/targets. Library Wasm checking and optimized runtime-probe compilation
pass. Protocol, build-driver and report tests pass (three each). Three Linux
PocketIC subjects pass: typed Wire rejection, trapped-work successor/executor
privacy, and shared-watchdog trap/upgrade recovery.

Formatting, direct/deployed dependency boundaries, provider ownership, snapshot
integrity/corruption, local Testkit adapter and release metadata/gate fixtures pass.
The reviewed supplier's release-runner command-stub fixture also passes from the
temporary clone; it was not added to Timers' snapshot or routine gate. An initial
sandbox offline admission overlapped an inaccessible installation lock and refused;
subsequent admission used the completed installation without repair or lock removal.

No full release suite, fresh cohort comparison, native macOS or live deployment
qualification was run. Previous cleanup evidence remains in
`/tmp/ic-timers-complexity-0311/`; it is not relabelled as new dependency evidence.
