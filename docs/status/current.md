![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and pending work

Released/default-branch base: **0.30.11**, `4c87d1cb`. Own package versions remain
0.30.11; the undated **0.30.12** section combines the current adoption batch.
Staging, commits, pushes and release execution remain maintainer-owned.

Adopt published Build/Host/Testkit **0.68.0** and Wire/derive **0.7.1**. Official
registry metadata and archive checksums verify both releases; source revisions
are `8f281177faaeee0adc51edc1031d4a7b268b0ce0` (Tooling) and
`2978dee37ddaf62c09ce1df411a0b1b6c9748bcd` (Wire). Build and the three used Host
crates have unchanged Rust source from 0.67.0. Testkit updates its Wire boundary.
Only these seven dependency identities change in this task's lockfile update.
Incoming Platform/macros **0.6.14** selections are preserved; Metrics stays 0.15.8.

## Wire ownership and semantics

Testkit 0.68 accepts the protocol's Wire 0.7 limits directly. The registry-only
0.7.0/Testkit 0.67 compiler refusal retained at `/tmp/ic-timers-wire-refresh/`
explains the earlier blocker; it no longer describes the selected graph. See
[ic-tooling #248](https://github.com/dragginzgame/ic-tooling/issues/248) for its
producer-owned implementation and acceptance evidence.

One resolved Wire/derive **0.7.1** serves the protocol, both probe canisters and
Testkit's native typed transport. Remove decode-only Serialize bounds from probe
and native adapters. Keep record layouts, method IDs, byte ceilings, traversal
budgets, raw malformed-ingress calls and trap translation. No compatibility
bridge, local codec or alternative transport is introduced.

Wire 0.7 decodes without reserialization, requiring only Deserialize. Nonminimal
varints reject as Malformed. Maps, sets and custom visitors may normalize input;
successful decoding does not establish byte canonicality. Timers' probe records
use structs, tuples, scalars and vectors, with no signature/hash or exact-frame
retry protocol. Such protocols must retain original frames and own semantic
validation. Encoder/framing bytes are unchanged. Wire 0.7.1's optional host schema
advice and packed signed-byte fields are not selected by this adoption.

The published timer library has no Wire dependency. Wire/Serde belong to the
unpublished probe protocol; Testkit owns native typed calls and errors. Platform
owns external IC APIs and the executor. Private platform handles and FFI ownership
remain unchanged; native substitutes do not prove installed-IC rollback behavior.

## Tooling and qualification

The root-lock-selected Build editor and Testkit CLI are refreshed through their
canonical setup adapters. Testkit retains PocketIC server ownership/admission.
Cohort receipts still require schema 2 and nonempty Build recipe identities;
source/cache, lock/compiler/server and artifact guards remain independent.
Dependency changes require new reviewed baselines; no baseline is rewritten.

Shared Tooling remains reviewed commit
`4c0f198e56d4c6d27b12a5c2d515e0e01070151e`, with 137 payloads. Its committed VERSION
says 0.26.10; committed 0.26.11 notes contain setup phase/duration diagnostics.
The supplier's dirty VERSION was excluded. CI tool caching remains disabled.

## Verification and limits

Current source before-images, official archives and focused logs:
`/tmp/ic-timers-tooling-wire-adoption/`. Rust 1.91 passes 165 timer unit tests,
three shared protocol roundtrips, all supported native probe Clippy selections,
and library/runtime/protocol plus all four size-probe Wasm checks. Formatting,
external-IC/deployment graph guards, provider ownership and Testkit setup/check
adapter fixtures pass. Installed-IC ingress proof now checks nonminimal-varint
rejection before timer mutation and a Deserialize-only native response.
Both refreshed CLIs pass offline admission. Linux PocketIC passes all 33
watchdog/recovery selections and the policy-cohort test, including all four
policy artifacts and both watchdog calibrations. The fresh schema-2 receipt is
`testing/target/cohorts.json`; it retains the current lock and recipe identities.

Earlier delivered 0.67 evidence remains at `/tmp/ic-timers-upstream-refresh/`;
cleanup evidence remains at `/tmp/ic-timers-cleanup-0311/`. Retained failed
artifacts, including `/tmp/timer-qualification-971360-0/`, must not be removed.
Prior issue acceptance and hosted matrix evidence remain with their owning
GitHub issues; a pushed commit alone is insufficient closure evidence.

Linux PocketIC qualifies the maintained installed-canister fixtures. Native
macOS, live deployments and the complete maintainer-operated release gate were
not run in this task; Linux execution and cross-compilation do not qualify them.
