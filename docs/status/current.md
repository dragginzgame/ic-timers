![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.9**, `b472937`. The checkout was clean at the start of the
accepted IC Tooling adoption. The undated **0.31.10** draft records that batch;
Cargo package versions remain 0.31.9. Staging, commits, pushes and release
execution remain maintainer-owned.

This is a compatible tooling-only batch: no published timer API, implementation,
protocol or deployed dependency selection changes. Bundle repository-only work
with the next code-bearing release by default; an explicit maintainer-owned bump
can select a repository-only patch under the existing release exception.

## Adoption and ownership

The root catalog and lock select published Build/Host/Testkit **0.69.0**.
Official sparse-index metadata establishes publication, and the downloaded Build
archive matches its registry checksum. All five consumed packages' published
`src/` trees are identical to 0.68.12. The supplier's minor boundary aligns its
Platform requirement with 0.7, including Principal identity in optional Registry
APIs; Timers already selects that line and exposes none of those APIs.
No consumer adapters or compatibility paths were needed.

Platform **0.6.21** is removed from the root lock. There is now exactly one
resolved Platform package, **0.7.0**, including native simulator tests. Metrics
0.16.3 and Wire 0.7.18 are unchanged. The deployed library graph still selects
only Platform's executor/ic0 features, one executor 2.0.0 and ic0 1.2.0; Candid,
Wire and host tooling remain excluded. Candid remains transitive in native
simulator tests through Platform/PocketIC, not in timer application protocols.

Shared Tooling remains **0.26.22**, revision
`5fc904a935d82ad31f0fba14434b71ccb8c10345`, with the same 131 payloads and
existing gates. No exported files, shared tool catalogs or sibling repositories
were changed in this batch. The [adoption guide](../shared-tooling.md) retains
the distinction between governance and executable enforcement.

Build continues to own retained acquisition/freshness through
`WasmBuildEvidence`. Timers owns recipes, subject discovery, artifact cardinality,
exact loaded-Wasm/report admission and durable report publication. Platform owns
external IC dependencies; Testkit owns server admission and lifecycle.

## Verification and limits

Before-images, registry/checksum evidence and focused logs are retained at
`/tmp/ic-timers-tooling-03110.BwVqaF/`. Rust 1.91 strict Clippy passes for both
coordinator and test targets. All six selected native build-driver/report tests
pass, covering failed discovery, ignored subjects, process cleanup/deadlines,
baseline aliases, malformed/incomplete reports, context identity and publication.
Locked metadata, formatting, changelog, pin/inheritance, external-IC declarations
and native/Wasm deployment-graph checks pass. The duplicate-graph observation
and metadata confirm the old Platform identity is gone.

Selected release editor and Testkit CLI 0.69.0 setup and offline admission pass;
Testkit admits the existing PocketIC 16.1.0 server. The first Testkit setup
reached its 60-second contention limit on a sibling-owned installer. A subsequent
setup observed slot release and reused the completed build; no foreign lock or
previous tool/evidence generation was removed. Both attempts remain in the logs.

These are focused local Linux checks. No fresh full release gate, Wasm compiler
execution, installed-IC/PocketIC measurement or native macOS run was performed;
production source and its dependency graph are unchanged. The full user-operated
release gate and required native-host matrix remain intact. Prior Platform/shared
adoption evidence remains at `/tmp/ic-timers-adoption-0319.yLi7Qi/`, with its
original input and qualification scope.
