![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Released **0.17.9** is `46785c3830786ccb17da1987cc9519c046b900d9`.
Its committed graph selects Metrics **0.6.2**, Testkit **0.33.5**, all four Host
packages at **0.13.1**, PocketIC **16.1.0** and Shared **0.5.0**
`a306180ce534227fcb98b882fba9a526d5d747e0` through 56/32/11 exports.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38134351101)
passes. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38134350782)
is queued at inspection; tag-only success does not qualify the new graph or
shared-store fixtures on Linux/Intel/ARM. Keep
[#44](https://github.com/dragginzgame/ic-timers/issues/44) open for that acceptance.

Earlier 0.17.8 `afb773f20ed7bf4e11d86f8d2e5db916fe9f1654` passes Linux, MSRV
and both native macOS gates. Its logs, metadata and hashes are retained at
`/tmp/ic-timers-0178-acceptance.ncHCWdZo/` with matched references at the
[0.17.8 evidence owner](../releasing.md#0178-release-evidence).
All three hosts execute the diagnostic fixture, 144 library tests, 14 recovery
subjects and four policy cohorts, closing
[#42](https://github.com/dragginzgame/ic-timers/issues/42) and
[#43](https://github.com/dragginzgame/ic-timers/issues/43). Those older tool/graph
bytes do not qualify 0.17.9 or an intentional live-failure artifact round trip.

The [Metrics identity owner](../design/callback-delivery-ownership.md#ic-metrics-06-released-graph)
records the public boundary delivered in the 0.17.7 patch in error. Future public
hard cuts require a minor release; do not restore a shim. Native reset evidence
is not IC upgrade/rollback qualification; existing PocketIC contracts retain that role.

## Adopted tooling and pending repair

The single undated **0.17.10** draft now adopts Shared **0.6.0**
`75180980a0c4e4252d8a915aaecfaefd92bf9a4e` through **56/33/11** canonical exports.
Remote main and committed `VERSION` agree. The audit export explicitly adds the
required `scripts/dev/cargo-tool-build.sh`. Source, previews, apply logs and the
preserved incoming dirty changes/catalog/lock/pins/index are retained at
`/tmp/ic-timers-shared060.g0xadlzs/`; the
[adoption owner](../shared-tooling.md#shared-tooling-060-cargo-storage-adoption)
records the boundary.

Cargo tools now join host/IC tools in Shared's per-user store using exact compiler,
native environment, source/package and packaged-lock identities. Tool builds use
the standard environment; application flags and wrappers remain local. Prepare
the selected compiler, run `make install-tools` then `make tools-check`, and use
the same `SHARED_TOOLING_STORE` setting for setup/checks/evidence. Old installations
and failed attempts remain retained but do not pass as a fallback. Common PATH
stays `.tools/rust/common/bin`; Testkit already executes the installer's returned
root-lock-selected path and keeps server/PocketIC admission. Consumer Binaryen
132 and the released dependency graph remain unchanged.

The refreshed selector owns local Rust evidence and referenced shared generations.
The collector removes its duplicate successful-path Rust selection and retains
independent safe local evidence on selector failure. The earlier local repair for
[Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121) stays in the
same draft: ignore partial selector output, archive local evidence/diagnostics,
mark shared collection incomplete and return its original failure status. New
coverage creates a failed Rust generation/reference through the canonical store
owner and checks extracted bytes/modes/links, including exclusion of its physical
target after store rejection. Successful build identity and cross-checkout reuse
remain owned by the selected shared Rust installer fixture.

The upstream action/selector also fix #121. Shared #122's optional distribution
fixture and #123's Cargo-extension guidance add no Timers callers. The
[release owner](../releasing.md#shared-060-cargo-storage-adoption) owns preparation
and pending native acceptance. Exact-source Shared CI run
[38136593268](https://github.com/dragginzgame/shared-tooling/actions/runs/38136593268)
is queued at inspection. Keep [Timers #44](https://github.com/dragginzgame/ic-timers/issues/44)
open; older/tag-only success does not qualify these new bytes.

Tests/builds/lint/setup, Cargo mutation, staging/commits and release remain
user-owned. This repository-only compatible batch changes no timer API/runtime
contract and claims no Wasm/instruction saving; it can remain untagged for the
next code-bearing release unless the maintainer chooses a repository-only patch.
Earlier rejection-repair source/syntax evidence remains at
`/tmp/ic-timers-01710-retention.jxfvm93b/`, with progress on
[Shared #121](https://github.com/dragginzgame/shared-tooling/issues/121#issuecomment-6108483079)
and [Timers #44](https://github.com/dragginzgame/ic-timers/issues/44#issuecomment-6108483459).
Those checks do not qualify this subsequent 0.6.0 adoption.

The 0.6.0 preparation passes all three snapshot checks, changed/embedded Bash
syntax, new owner anchors, diff whitespace and full locked offline metadata.
Catalog/lock/pins and raw index match preserved incoming inputs. Its evidence and
reviewable diff are retained at the 0.6.0 preparation path above, with current
progress on [#44](https://github.com/dragginzgame/ic-timers/issues/44#issuecomment-6108726659).
These permitted checks do not execute the new cases or establish native acceptance.
