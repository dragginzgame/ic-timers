![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-21

## Source and accepted work

Released base: **0.30.0**, `41e1e2f`; package versions remain unchanged. The undated
**0.30.1** draft combines the incoming Platform 0.6 adoption with IC Tooling 0.63.
The root lock selects Build/Host/Testkit **0.63.0**, Platform/macros **0.6.0** and
Wire/derive **0.5.6**. The Wire selection and all prior local work are preserved.
Metrics advanced from 0.15.0 to **0.15.1** after initial Tooling resolution; preserve
that observed lock update. Its arithmetic Rust sources match 0.15.0. The initial
and observed lock copies are retained with the current evidence below.
There is still one root dependency catalog/lock and five maintained packages.
Shared remains reviewed **0.25.7**, `d39e8a84bf4fd7e42776d2bb8e589ad09aacb02b`.
No Cargo package version, index, release state or sibling checkout is mutated.

## Published IC Tooling adoption

Official registry indexes confirm non-yanked Build, Host Artifacts/FS/Process and
Testkit **0.63.0**, all MSRV 1.91. Registry checksums and all compiled-cache files
match the five published archives, source
`c342bb281dc82ff8878c26b5f5449972e9f5341d`. Published manifests, APIs and migration
notes were inspected rather than relying on the sibling's stale handoff.

Build and the direct spawner exchange matching Host 0.63 `OwnedChild` values.
Testkit now selects Platform 0.6 with Principal explicitly; the old native 0.5.1
package is removed. It no longer selects Platform's Candid codec feature or Metrics.
Timers does not use the removed Candid calls or benchmark error type. Its existing
raw PocketIC/Wire requests, limits and traps remain. PocketIC still brings Candid
transitively; library and canister graphs remain Candid/full-CDK-free.

Build's editor and Testkit's server binary now require **cli**. Local setup/check
adapters pass `--features cli` to the existing Shared selected-tool installer.
Workspace library dependencies leave that feature off. Explicit preparation and
offline admission pass for both new selections; prior installations are retained.
The local adapter fixture verifies exact feature forwarding, failure propagation
and ordered parallel-Make setup. Real-editor metadata/version-preparation fixtures
pass in isolated repositories; no real version-bump or release command was run.

Use `make install-release-editor install-testkit-server`, then `make tools-check`.
Shared's exported generic setup guide still has featureless examples; do not patch
that snapshot payload. Prefer the local adapters or add `--features cli` to a
manual selected-tool command for these 0.63 binaries. Shared adoption is independent
of the producer's own baseline update.

## Retained runtime boundary

The library keeps only Platform **executor + ic0**, registry executor **2.0.0** and
IC0 **1.2.0**. Its independent Wasm normal/build graph remains **14** name/version
identities including the root, excluding Principal, Serde, SHA-2/digest and text
support. The native coordinator remains **53** identities. These are graph counts,
not linked-size or speed measurements. Simulator/codec dependencies stay dev-only;
protocol records select Wire/Serde only. Probe runtime/entrypoint profiles retain
their implied Principal support. No provider fork, shim or protocol rewrite is added.

The API/source identity and live qualification of Platform 0.6 from the preceding
pass remain at `/tmp/ic-timers-platform-lean.ld0o7b43/`. Current root lock changes
require their own integration qualification and a new cohort context; do not infer
comparability from an older receipt. Timers owns scheduling/claims/observations,
Platform IC bindings, Wire codecs, Testkit simulator lifecycle, and Build/Host the
artifact/process machinery. Applications retain durable intent and lifecycle policy.

## Current qualification

Rust 1.91 driver/protocol/probe lint, all four cohort feature selections, grouped
formatting and native/Wasm dependency-role/feature admission pass against the final
lock, including Metrics 0.15.1. Native/Wasm MSRV library builds pass. The metadata,
real-editor version-preparation and CLI adapter fixtures, ShellCheck, selected-tool
setup and offline recheck pass. Live watchdog qualification passes all 27 product
subjects plus three host fixtures; all four cohorts and both calibrations pass.
The current receipt at `testing/target/cohorts.json` matches the final lock.
Evidence and incoming copies:
`/tmp/ic-timers-tooling-adoption.5yntjine/`.

Native macOS, installed IC and the complete maintainer-operated release gate remain
separate. Version bumps, staging, commits, tags, pushes and publication remain
maintainer-owned. Other repositories remain read-only.
