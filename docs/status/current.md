![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-22

## Source and accepted work

Released base: **0.30.2**, `000a5e4`; package versions remain unchanged. The
undated **0.30.3** draft combines incoming Wire/Metrics adoption with aligned
Build/Host/Testkit **0.64.0**. This remains compatible at the published Timers
boundary: only unpublished native qualification imports/features change.
There is one root dependency catalog/lock and five maintained packages.

Official registry indexes and checksum-verified published archives select Build,
Host Artifacts/FS/Process and Testkit **0.64.0**, source
`036da0cdf3d3b35297fbc26031e373f386a2f2de`. All compiled-cache files match those
archives. Only these five package versions change from this task's incoming lock;
Platform/macros **0.6.7**, Metrics **0.15.5**, Wire/derive **0.5.20**, Tokio
**1.54.1**, executor **2.0.0** and IC0 **1.2.0** are preserved.
Build and the three Host packages have unchanged Rust sources versus 0.63.3;
matching package identities still matter at the injected `OwnedChild` boundary.

## Published 0.64 ownership migration

Native tests import PocketIC clients directly from `ic_platform::pocket_ic`.
Their existing dev-dependency now selects `pocket-ic`, including Principal support;
`principal::ExternalPrincipal` retains PocketIC's external API identity. Testkit
owns server admission/startup, instance lifetime, install extensions and progress
helpers. Its removed PocketIC forwarding imports are gone from all three callers.
The coordinator's normal dependencies remain simulator/codec-free.

Timers does not use Testkit's removed Fake/Performance wrappers, diagnostics batch
error, or optional Wire calls. Its raw PocketIC calls and bounded native Wire
codec retain method identities, limits, traps and error precedence. No compatibility
facade or local replacement is added. The protocol remains Wire/Serde-only, and
no storage/memory migration or full CDK dependency is introduced.

Build's editor and Testkit's server CLI retain explicit `cli` selections through
the existing root-lock installers. New 0.64 selections require explicit setup;
old installations and failure attempts are retained. Both CLIs are prepared.
Initial Testkit setup refused a live shared-store lock owned by ic-jobs. After
the owner completed and published its verified selection, canonical Timers setup
succeeded. No foreign lock, candidate or source was modified.

## Shared and product boundaries

Shared **0.26.6**, `4cb9bc907154225a3f7e4c33fd8d75fbeaa125c8`, is still current
on upstream main. Canonical preview reports only KEEP; the existing 134-payload
snapshot verifies and is unchanged. No generic test suites, audit execution,
optional IC executables or schedules are restored. Timers selects an empty IC
list for compact evidence and leaves PocketIC setup/admission with Testkit.

Timers owns scheduling, claims and observations; Metrics arithmetic, Platform IC
bindings/client identities, Wire codecs, Testkit simulator lifecycle and Build/Host
artifact/process machinery retain separate owners. Applications own durable intent
and lifecycle policy. The library selects only Platform executor/IC0; canister
probes retain raw explicit codecs. Candid remains native-only through PocketIC;
production library/probe graphs reject it and full CDK. Protocol records carry no
runtime authority. No timer-provider or executor implementation changes.

## Current qualification

Focused Linux checks pass: locked metadata, declaration/feature/graph policies,
library Clippy, Rust 1.91 probe/protocol/driver lint and all four size features,
grouped formatting, native/Wasm library MSRV builds, two driver process/build
tests, one report identity test, CLI adapter and metadata/version fixtures.
Shared no-change preview, snapshot integrity and whitespace checks pass.
Offline tool admission passes for both selected 0.64 CLIs and the existing
PocketIC server. Live qualification passes 27 product subjects plus three host
fixtures, all four policy cohorts and both calibrations. The new cohort receipt
matches the final lock; no cross-context performance improvement is claimed.
Library/coordinator normal/build graphs remain 14/53 package identities.

Incoming copies, official indexes/archives, source comparisons and logs:
`/tmp/ic-timers-host-shared-vd2x90u6/`. The preceding Wire/Metrics codec tests and
PocketIC receipt remain at `/tmp/ic-timers-wire-metrics-7ghjoy9w/`; that receipt
has the old lock context. Native macOS, installed IC and the full maintainer-owned
release gate remain separate. No package version changes, staging, commits,
pushes or releases are performed; sibling repositories remain read-only.
