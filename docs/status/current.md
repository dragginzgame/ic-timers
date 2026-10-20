![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-20

## Source and pending work

Released base: **0.29.3**, `94f41a4`. Package versions remain 0.29.3. The
maintainer's incoming catalog selects Testkit 0.57 and Wire 0.5; the incoming
lock selects Platform 0.2.2, Build 0.22.0 and Host Process 0.30.1. Those dependency
updates are retained. This batch adds the native driver's direct Clap edge using
the already selected package; release metadata/version changes remain user-owned.
Compatible repository/tooling notes are collected under undated **0.29.4**.

- Adopt reviewed Shared Tooling 0.25.2 revision
  `546d32e48be5a5d4e360299f6a94901df6acc577` through the canonical exporter.
  Shared owns Cargo graph observation and release candidate rendering; Timers
  retains selected roots, feature exclusions and exact runtime source identities.
- Check the library, both canister probes and the protocol independently for Wasm
  and the actual native host. Probe graphs must remain Candid-free; the protocol
  must remain free of Platform and timer runtime authority. Native Testkit/PocketIC
  Candid transport is intentional and outside those canister exclusions.
- Probe callers select the existing `Method` enum, which supplies both endpoint
  name and frame ID. The reverse lookup and unknown-name branch are removed.
  IDs, Wire limits, DTOs and malformed-payload tests keep their existing contracts.
- IC Build's root-lock-selected release editor replaces retired Shared Perl
  renderers. Explicit setup is `make install-release-editor`; offline admission
  is `make release-editor-check`. The per-document limit is in
  `ci/release-editor.env`. Fresh release preflight prepares this tool after source
  admission; standalone metadata preparation does not install it implicitly.

## Ownership and validation

Timers owns volatile scheduling, claims, observations and private IC dispatch.
Applications own durable intent, lifecycle ordering and health policy. Platform
owns the selected runtime/system boundary; Wire owns codecs; Testkit owns server
admission/lifecycle; Build and Host own artifact/process mechanics. See
[architecture](../architecture.md#capability-ownership) and [SAFETY.md](../../SAFETY.md).
The protocol package carries Wire/Serde values only and has no runtime authority.

Focused qualification passes for this worktree and its incoming Testkit/Wire
selections: all eight root/target graph checks and negative fixtures, strict Rust
1.91 probe lint, three protocol tests, standalone library Wasm compilation,
27 real-Wasm watchdog subjects and the four-policy cohort check. Packaging
verifies the prepared archive's license. Release preparation/index/admission,
formatting, hook and snapshot fixtures pass; these exercise disposable fixtures,
not an actual release. Before-images and receipts are retained at
`/tmp/ic-timers-three-improvements.7qqc7zen/`. Native macOS, live IC and the
complete maintainer-operated release gate remain separate.
Follow-through adds the editor to standalone release validation setup and covers
editor/Testkit ordering and first-failure propagation under parallel Make in the
existing adapter fixture. Focused receipts are in
`/tmp/ic-timers-followthrough.72zdplyr/`. Real offline `make tools-check` passes
after preparing the unchanged common tool versions for refreshed build inputs.
Version bumps, staging, commits, pushes, tags and publication remain user-owned.

## Historical evidence

The preceding handoff is preserved verbatim in
[archive-2026-10-20.md](archive-2026-10-20.md), including the released Platform 0.2 /
Build 0.22 adoption and earlier superseded blockers. Older receipts remain in the
[2026-10-18 archive](archive-2026-10-18.md), [Shared history](../evidence/shared-tooling-history.md)
and [release history](../evidence/release-history.md). Their original pending and
validation statements apply only to their named source/graph; they are not a
current release queue or authority for restoring retired paths.
