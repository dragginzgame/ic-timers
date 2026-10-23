![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-23

## Source and accepted work

Released base: **0.30.4**, `9cf5f81`; package versions remain unchanged. The
undated **0.30.5** draft adopts aligned Build/Host/Testkit **0.65.0**, qualifies
incoming Wire/derive **0.6.5**, and refreshes Shared Tooling to **0.26.7**.
Public timer APIs, native probe records, method IDs, codec budgets, callback/trap
semantics and runtime ownership are unchanged. There is one catalog/lock and
five maintained packages; no local Rust source or generator changes are needed.

Official registry indexes and checksum-verified published archives establish
Build, Host Artifacts/FS/Process and Testkit 0.65.0, source
`378ad71296db81436b38346b697960199bdb00e1`; Wire/derive 0.6.5 is source
`e10196ee7a504984cc65152805ff0e030b0c6964`. All compiled-cache files match the
seven archives. Only the five Tooling packages change from the incoming lock;
Wire 0.6.5 and all other pending selections are preserved. Platform/macros remain
**0.6.8**, Metrics **0.15.6**, executor **2.0.0**, IC0 **1.2.0**. No unpublished
sibling source is selected. Host/Build exchange matching `OwnedChild` identities.

## Shared Tooling refresh

Canonical preview/apply retains all 134 payloads and roots at reviewed revision
`3e8d7cd3e7e9b7b79a5e800c30daf0be2710dab0` (Shared 0.26.7). Only the selected
Cargo installer, store helper and generic setup guide change. Exported files are
supplier-owned and verify exactly; no generic test matrix or schedule is restored.

Explicit Cargo setup waits up to 60 seconds for an admitted slot owner, then
rechecks source, build, lock and receipts before reuse. The documented invocation
budget is 0–300 seconds; zero refuses immediately. Checks/preflight and other
lock classes remain immediate. No owner lock is deleted or partial generation
admitted. A one-time isolated export of the owner's focused wait fixture passes
release/reuse, timeout, interruption and invalid-owner/acquisition failures.

Both new root-lock CLI selections are explicitly prepared with `cli` and pass
offline admission. The first Testkit setup reached the default wait ceiling while
ic-backup compiled the shared selection. After completion, canonical Timers setup
reused the verified result. Foreign locks, source and candidates were untouched;
the first failure and successful setup logs are retained. Old tools are retained.

## Product and codec boundaries

Timers owns scheduling, claims and observations; Metrics arithmetic, Platform IC
bindings/client identities, Wire codecs, Testkit simulator lifecycle and Build/Host
artifact/process custody remain separate. Applications own durable intent/lifecycle
policy. The native harness imports PocketIC directly from Platform. Testkit's new
optional WireCallSpec/native helpers are not selected here; existing raw calls
retain their exact error handling, method translation and malformed-byte evidence.
No obsolete finite API, compatibility facade or protocol rewrite is introduced.

Wire 0.6.5 changes only the u64 serialization fast path from 0.6.4; original u64
admission remains. Two isolated Rust 1.91 consumer graphs compare identical code
against each published version, reusing root-lock transitive versions. All 4,078
request/response-frame and typed-error observations match, including checkpoint,
snapshot/cohort records, u64 thresholds/lists, identity/direction, truncation,
trailing bytes and byte/allocation/node/depth refusals. The initial attempt to
resolve both compatible patch versions in one graph was rejected by Cargo; the
separate-graph proof passes. All attempts remain outside maintained source.

The library selects only Platform executor/IC0; protocol records use Wire/Serde
without runtime authority. Independent native/Wasm graph admission passes, with
14 library and 53 coordinator package identities. Candid/management remains
native-only through Testkit/PocketIC; production library/probe graphs exclude
Candid, full CDK and host tooling. No storage/memory migration is included.

## Current qualification

Focused Linux checks pass: locked metadata and boundary policy, library Clippy,
Rust 1.91 protocol/driver/probe lint and all four size features, grouped formatting,
native/Wasm library MSRV, all three protocol tests, exact-frame/error comparison,
metadata/version fixtures, two driver build/process tests and one report test.
CLI adapter, hook/index, snapshot corruption and focused wait fixtures pass.
Tool setup/offline admission, snapshot integrity and whitespace checks pass.
Live qualification passes 27 product subjects plus three host fixtures, all four
policy cohorts and both calibrations. The new cohort receipt matches the final
lock; no cross-context performance improvement is inferred. Native macOS,
installed IC and the full maintainer-operated release gate remain separate.

Before-images, registry indexes/archives, source comparisons and qualification:
`/tmp/ic-timers-upstream-refresh-zs_qs4gq/`. Prior Wire migration evidence remains
at `/tmp/ic-timers-wire-adopt-2rq455y3/`. No package version changes, staging,
commits, pushes or releases are performed; sibling repositories remain read-only.
