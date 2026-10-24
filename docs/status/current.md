![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and pending work

Released base: **0.30.12**, `798f559a`. Own package versions remain 0.30.12;
the undated **0.30.13** section records Platform adoption qualification. Staging,
commits, pushes and release execution remain maintainer-owned.

The incoming lockfile already selects published Platform/macros **0.6.15**.
Their official archive checksums and release source revision
`6dcf54a229201f85775d218b7a9f41809297878f` are verified. Both crates' complete
Rust source trees match 0.6.14. The dependency change is Platform's optional
native Wire metadata selection from 0.6.7 to 0.7.2; Timers does not enable it.
No local source, feature or import migration is needed.

The selected graph retains one Platform identity, executor **2.0.0** and IC0
**1.2.0**, with unchanged callback/context and unsafe-registration boundaries.
The incoming lockfile is preserved byte-for-byte. Build/Host/Testkit remain
**0.68.0**, Wire/derive remain **0.7.2**, and Metrics remains **0.15.8**. Wire
0.7.2 was already in the released lockfile; earlier handoff prose saying 0.7.1
was stale. No unrelated dependency upgrade is performed.

## Ownership and maintained contracts

The published timer library selects only Platform executor/IC0 plus Metrics and
Thiserror. No member declares Candid or CDK-family crates directly. Library and
probe deployment graphs remain Candid-free. Native Testkit/PocketIC transport
retains its necessary external Candid dependency through Platform.

Wire/Serde belong to the unpublished probe protocol; Testkit 0.68 owns native
typed calls and errors and accepts its Wire limits directly. Decoding needs only
Deserialize. Nonminimal varints reject as Malformed; collections/custom visitors
may normalize input, so successful decoding does not prove byte canonicality.
Current records use structs, tuples, scalars and vectors, with no signature/hash
or exact-frame retry protocol. Existing records, method IDs, directional limits,
trap translation and fixture checkpoints are unchanged.

The private platform module remains the sole timer/executor owner. Handles stay
private and callback registration stays in FFI. Native substitutes do not prove
installed-canister rollback behavior. No storage migration or compatibility
route is introduced.

## Tooling and qualification

Both root-lock-selected CLIs pass offline admission. Testkit retains PocketIC
server ownership and admission. Cohort receipts require schema 2 and nonempty
Build recipe identities; source/cache, lock/compiler/server and artifact guards
remain independent. New dependency contexts require reviewed baselines rather
than automatic baseline rewrites.

Shared Tooling remains reviewed commit
`4c0f198e56d4c6d27b12a5c2d515e0e01070151e`, with 137 payloads. Its committed VERSION
says 0.26.10; committed 0.26.11 notes contain setup phase/duration diagnostics.
The supplier's dirty VERSION was excluded. CI tool caching remains disabled.

## Verification and limits

Current before-images, official archives and focused logs:
`/tmp/ic-timers-platform-adoption/`. Rust 1.91 passes library Clippy, 165 timer
tests, eight doctests, all supported native probe Clippy selections, and library/
runtime/protocol plus all four size-probe Wasm checks. Formatting, declaration/
deployment graph guards and provider ownership pass. Wire metadata is absent
from the resolved Platform feature set. Linux PocketIC passes all 33 watchdog/
recovery selections and the policy-cohort test, including four policy artifacts
and both calibrations. The fresh schema-2 receipt at `testing/target/cohorts.json`
matches the selected lock and contains all four recipe identities.

Prior Tooling/Wire evidence remains at `/tmp/ic-timers-tooling-wire-adoption/`;
its receipt describes its own lock context. Older delivered evidence remains at
`/tmp/ic-timers-upstream-refresh/` and `/tmp/ic-timers-cleanup-0311/`. Retained
failed artifacts, including `/tmp/timer-qualification-971360-0/`, must not be
removed. Prior issue acceptance and hosted matrix evidence remain with their
owning GitHub issues; a pushed commit alone is insufficient closure evidence.

Native macOS, live deployments and the complete maintainer-operated release gate
are not qualified by Linux checks or cross-compilation.
