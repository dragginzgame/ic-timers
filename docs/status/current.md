![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-20

## Source and accepted work

Released base: **0.29.6**, `50c8450`; package versions remain unchanged.
The undated **0.29.7** draft contains Platform, Host and Shared adoption together.
Preserve the incoming Platform **0.5.0**, Build **0.24.0**, Host **0.32.0**,
Metrics **0.14.11**, Testkit **0.60.0** and Wire **0.5.2** catalog/lock selections.
Package versions and the published timer API are unchanged. Incoming staged and
unstaged work is preserved; this dependency qualification adds only documentation.

## Published Platform adoption

Adopt registry Platform/macros **0.5.0**, non-yanked, with Platform MSRV 1.88.
The official registry indexes confirm both archive checksums. All 61 facade and
nine macro package files in the compiled cache match those archives. Both record
source `b840423928f0ca7f14d7fc860508730c87356640`. Published features, APIs and
migration guidance were reviewed against actual consumers, not sibling drafts.

The library retains only executor/IC0 features. Probe entrypoints keep `cdk`,
which now includes `runtime`; policy admits that feature only for probes and
adds a negative library-leak case. Executor **2.0.0**, IC0 **1.2.0**, their export
modules and the maintained future-context modules remain unchanged. No source
consumer uses the removed timeout setter, hidden option, storage helpers or
Candid service generation. No shim, storage migration or codec rewrite is added.

Testkit 0.60 and the native driver now share Platform **0.5.0**, removing the
older 0.4.1 package. The same registry external Principal/PocketIC identities,
Wire frames and explicit limits remain. Independent library and probe-canister
graphs remain Candid/full-CDK-free; Candid stays native through Testkit/PocketIC.

## Published tooling adoption

Official registry indexes confirm latest non-yanked Build **0.24.0**, Testkit
**0.60.0**, Host **0.32.0** and Wire **0.5.2**. Metrics 0.14.11 and Platform 0.5.0
remain latest. Prepared archives and cached sources match official checksums.
Build records source `c84c64e42a6bbbf06eba4490cf1cf6988caf5866`, Testkit
`65c99c9a05108b5e473f538ec72917d7a7304deb`; both support Rust 1.91.

Build 0.24 exchanges Host 0.32 `OwnedChild` values, resolving the prior E0271
without a shim, alias, conversion or unpublished override. Testkit 0.60 retains
Host 0.31.2 internally; no exchanged value crosses that boundary. Its sibling's
0.61 adoption notes are unpublished and are not a consumer dependency. Host
provider mechanics stay native; all five members retain the one root catalog.
Timers selects no optional Shared IC executable bundle. Testkit alone selects,
installs and admits PocketIC; do not activate unrelated tools to update this graph.

Selected Build editor and Testkit CLI/server preparation and offline admission
pass through the canonical installers. Rust 1.91 probe/driver/protocol Clippy,
all four size feature sets, three host fixtures, Cargo metadata fixtures and
grouped formatting pass. Dependency-role/inheritance and native/Wasm graph
checks pass. Fresh watchdog qualification passes all 27 product subjects plus
three host fixtures; all four cohorts and both calibrations pass with a refreshed
`testing/target/cohorts.json` bound to the selected lock. No new blocker was found.
Evidence and incoming before-images: `/tmp/ic-timers-deps.ftdt0lxm/`.
Earlier Host qualification remains at `/tmp/ic-timers-host-032._9e1zaz1/`.

## Focused qualification

Earlier Platform Linux checks pass for strict library lint, probe/protocol lint
and all four size features on Rust 1.91, native/Wasm MSRV library builds, 7,776
generated native traces, graph/feature admission and negative boundary fixtures. Fresh
Platform-0.5 Wasm passes all 27 maintained watchdog subjects plus three host
fixtures, including awaits, traps, cancellation, pressure and upgrades. All four
policy cohorts and both calibrations passed at that earlier lock. Current
dependency qualification is recorded above. New source/lock
identities require explicit baseline review, not cross-context speed/size claims.
Evidence and incoming before-images: `/tmp/ic-timers-platform-05.snizl9v_/`.
Native macOS, installed IC and the complete maintainer-operated release gate
remain separate. No missing Platform API or adoption issue was encountered.

## Shared Tooling and ownership

Reviewed Shared **0.25.7**, `d39e8a84bf4fd7e42776d2bb8e589ad09aacb02b`, retains
113 entrypoints and 138 payloads. CI lints before compilation, tests and packages;
publication serializes source/tag admission, Clippy, packaging and external
publication. Parallel `-j4` failure fixtures retain the complete lint scope and
remaining release/probe gates. Shared's failed Cargo metadata evidence reaches
the existing CI archive. Earlier source-bound qualification is at
`/tmp/ic-timers-shared-0257.tf16fmhr/` and `/tmp/ic-timers-shared-0256.pnwelhot/`.

Timers owns volatile scheduling, claims, observations and private IC dispatch;
applications own durable intent and lifecycle policy. Platform owns IC bindings,
Wire application codecs, Testkit simulator admission/lifecycle, and Build/Host
artifact/process mechanics. Protocol records select only Wire/Serde. See
[architecture](../architecture.md#capability-ownership),
[Shared adoption](../shared-tooling.md) and [SAFETY.md](../../SAFETY.md).

Historical evidence does not qualify changed providers or authorize restoring
retired paths. Version bumps, staging, commits, pushes, tags and publication
remain maintainer-owned. No sibling checkout is modified.
