![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-26

## Source and pending work

Released base: **0.31.3**, `083a919`. The undated **0.31.4** section covers
published Platform/macros **0.6.18** and Wire/derive **0.7.16** adoption. Both were
already selected in the incoming lockfile, which is preserved byte for byte.
Registry metadata and checksum-verified archives confirm publication and Rust
1.91 support. Cargo package versions remain 0.31.3; commits, staging, pushes and
release execution remain maintainer-owned. This compatible dependency adoption
changes no public timer API, wire contract or supported compiler requirement.

## Ownership and adoption

Platform adds optional BLAKE3 exports. Timers does not enable that feature and
no BLAKE3 package resolves. Selected executor/IC0 and macro sources are unchanged;
executor **2.0.0** and IC0 **1.2.0** identities remain unique. The existing graph
checker derives forbidden Platform features from its published catalog, so the
new feature is excluded without another local allowlist or dependency wrapper.

Wire replaces the u16/i16 varint tail loop with explicit byte groups. Published
source review confirms unchanged shortest-form, overflow and truncation refusal,
encoder bytes and resource limits. Probe records contain no u16/i16 fields; no
performance improvement is claimed for Timers. Wire derive sources are unchanged.
The protocol remains owned by the unpublished Wire/Serde package; runtime timer
policy, authority and callback recovery stay local. No local Rust or manifest
changes were needed.

The library still uses only Metrics, Platform and Thiserror directly. Library
and probe deployment graphs exclude Candid; native Testkit/PocketIC transport
intentionally retains it. Metrics **0.16.2**, Build/Host/Testkit **0.68.6** and
Thiserror **2.0.18** remain selected. Testkit's selected server is PocketIC 16.1.0.
Shared Tooling remains **0.26.19**, revision
`c830c33447f0a84e015bda7fa14b6c096a58368e`, with 135 payloads. Normal releases
start fresh; saved-attempt recovery remains explicit through `release-resume`.

## Verification and limits

Evidence, archives, source diffs and incoming before-images are retained in
`/tmp/ic-timers-platform-wire-0314/`. Rust 1.91 Clippy passes for all five affected
packages. Library and default size-probe Wasm checks, optimized runtime-probe
compilation and three protocol tests pass. Three Linux PocketIC tests pass:
typed Wire rejection before mutation, bounded/deterministic full inventory
replies, and shared-watchdog trap/upgrade recovery. Formatting, pin/direct/deployed
dependency boundaries, provider ownership and changelog checks pass.

Initial offline CLI admission refused an unprepared selection for the current
compiler/build inputs before any tests ran. Explicit Testkit setup prepared the
selected 0.68.6 CLI through its owner, and the subsequent offline admission and
integration tests passed. The failure note and setup log remain retained; no
fallback server or weaker admission was introduced.

No full release gate, fresh cohort comparison, native macOS, live deployment or
performance qualification ran. Previous Tooling/Shared/cohort evidence remains
in `/tmp/ic-timers-adoption-0313/` and is not relabelled for the new selection.
