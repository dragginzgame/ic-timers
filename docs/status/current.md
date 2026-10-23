![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-22

## Source and accepted work

Released base: **0.30.3**, `55d7f1b`; package versions remain unchanged. The
undated **0.30.4** draft adopts published Wire/derive **0.6.2**. This remains
compatible at the published Timers boundary: Wire is qualification-only and
all callers already use the retained bounded native Serde codec. No local Rust
source, protocol record, method identity, limit, hook or generator changes.
There is one root dependency catalog/lock and five maintained packages.

The root catalog now requires Wire **0.6**. Official indexes confirm non-yanked
Wire/derive 0.6.2, MSRV 1.91. Both archive checksums match registry and lock;
compiled-cache files match published source `1deec75f8aae722b2d62b5d966eaad11485a285c`.
Only these two packages change from the incoming lock. Pending Build/Host/Testkit
**0.64.1**, Platform/macros **0.6.8**, Metrics **0.15.6**, Tokio **1.54.1** and
the native management dependency selections are preserved. Executor **2.0.0** and
IC0 **1.2.0** remain unchanged. No unpublished sibling implementation is used.

## Wire ownership and compatibility

Wire 0.6 removes its finite codec, serializer-generating derive, Shape/Method
contracts and typed retained-byte API. Timers has no callers of those paths.
The probe protocol and both canister/native adapters retain native Serde frames,
explicit IDs, bounded request copying, codec budgets, typed errors and traps.
No fallback reader, compatibility facade, new serializer or metadata generator
is added. The selected Testkit's optional finite Wire helpers remain disabled;
only Wire/derive 0.6.2 is resolved in the maintained workspace graph.

The native encoder/decoder adapters are source-identical to incoming 0.5.22;
the native facade differs only in its module documentation. A temporary Rust
1.91 fixture checks exact request/response bytes, cross-decoding and typed error
parity for control tuples, wide checkpoints, snapshots and cohort records. It
covers truncation, trailing bytes, direction/method mismatch and byte/allocation/
node/depth refusals. The fixture reuses root-lock transitive versions; its only
extra codec identities are the comparison's old Wire/derive 0.5.22. The initial
scratch helper assumed PartialEq on a cohort record; comparing decoded canonical
frames fixes that proof without changing product types. Both attempts are kept.

## Retained integration boundaries

Native tests import PocketIC clients directly from Platform; Testkit owns server
admission/startup, instance lifetime, install extensions and bounded progress.
The pending Testkit 0.64.1 selects Platform Candid/management support for native
provider fixtures, in addition to PocketIC's Candid transport. Current architecture
and adoption guides now reflect that incoming graph. Those features remain
excluded from deployed library/probe graphs; protocol records carry no platform
or timer authority. No memory/storage feature or migration is introduced here.

The library selects only Platform executor/IC0; Metrics owns arithmetic and
applications retain durable intent/lifecycle policy. Build/Host own compilation,
artifact custody and process supervision. Current root-lock Build/Testkit CLIs
are explicitly prepared with `cli`; library dependencies remain CLI-free. Offline
tool and Testkit server admission pass. Existing installations are retained.
Shared **0.26.6**, `4cb9bc907154225a3f7e4c33fd8d75fbeaa125c8`, is unchanged.
No generic producer suite, optional IC tool bundle, audit or schedule is restored.

## Current qualification

Focused Linux checks pass: full locked metadata, declaration/feature/graph
policies, library Clippy, Rust 1.91 probe/protocol/driver lint and all four size
features, grouped formatting, native/Wasm library MSRV, all three protocol tests, and
the old/new compatibility fixture. Tool setup and offline admission pass.

Live qualification passes 27 product subjects plus three host fixtures, all four
policy cohorts and both calibrations. The generated cohort receipt matches the
final lock. No performance improvement is inferred across changed lock contexts.
Native macOS, installed-IC qualification and the full maintainer-owned release
gate remain separate; neither a native fixture nor cross-build qualifies them.

Incoming copies, official indexes/archives, source comparisons and logs:
`/tmp/ic-timers-wire-adopt-2rq455y3/`. Prior 0.64 integration evidence remains at
`/tmp/ic-timers-host-shared-vd2x90u6/`. No package version changes, staging,
commits, pushes or releases are performed; sibling repositories remain read-only.
