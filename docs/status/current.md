![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-20

## Source and accepted work

Released base: **0.29.4**, `b03ce58`; own package versions remain unchanged.
The existing undated **0.29.5** draft contains the complete pending batch.
Adopt published Platform/macros **0.4.0** and reviewed Shared Tooling **0.25.4**,
`db555ab6cf4a93af9a183f18d921d3b0ee97c725`, with the same 138 payloads.
The incoming Platform 0.4 selection is preserved. Adopt Host Process 0.31.1 with
Build 0.22.1. Testkit 0.57.0, Wire 0.5.0 and unrelated pending Rust work remain.
The concurrent rustls-webpki 0.103.16 lock selection is preserved.

Official registry indexes and cached archives confirm Platform source
`26a07c510ab74413ec30779cd69af84db13cff11` and Host 0.31.1 source
`950669df9d2c91497e99ae1e3f1c5a77f8f80ef8`. Migration guidance and published
signatures were reviewed. Platform's new RejectCode ownership and transport-only
calls require no changes to the existing raw Wire probes. Executor/IC0 exports
remain unchanged at registry **2.0.0**/**1.2.0**; neither identity nor any Principal,
Account or call type crosses the published timer API. Native PocketIC callers
retain explicit `principal::ExternalPrincipal` without wrappers or aliases.

Shared 0.25.4 changes evidence guidance, not installer policy or tool pins.
The collector now retains redirected `target`/`.tools` links and diagnostics,
skips their children and preserves independent logs with incomplete status.
The existing failure-evidence fixture covers both parents and failed selectors.

## Host and Build alignment

Build **0.22.1** was published during adoption and now selects Host **0.31**.
Its archive matches the registry and source
`1d21935cd6779229a07db435691a1e86598691f2`. Require at least 0.22.1 in the catalog:
Build 0.22.0 requires the distinct Host 0.30 `OwnedChild` and reproduces E0271 with
the 0.31 spawner. This matching published pair needs no conversion, alias or
unpublished override. The earlier [Build #9](https://github.com/dragginzgame/ic-build/issues/9)
report is unnecessary; nothing was posted. Testkit retains its independent Host
0.30 subtree, which does not cross the injected-spawner boundary.

## Ownership and qualification

Timers owns volatile scheduling, claims, observations and private IC dispatch.
Applications own durable intent and lifecycle/health policy. Platform owns runtime
bindings, Wire codecs, Testkit server admission/lifecycle, and Build/Host artifact
and process mechanics. Native Candid and Platform 0.2.2 remain through Testkit;
library and canister graphs are Candid/full-CDK-free. The protocol retains only
Wire/Serde. See [architecture](../architecture.md#capability-ownership),
[Shared adoption](../shared-tooling.md) and [SAFETY.md](../../SAFETY.md).

Focused Linux qualification passes for the final pair: Rust 1.91 strict probe
lint, native/Wasm library checks, three protocol tests, 27 real-Wasm watchdog
subjects and the four-policy cohort check. Graph/source/feature admission and
rejection fixtures, packaging/license, offline tool checks, version preparation,
release-index, formatting and failure-evidence regressions pass. The Build 0.22.1
release editor is prepared and admitted from the root lock. Before-images,
registry/source checks, the superseded Host refusal and focused logs are at
`/tmp/ic-timers-platform-host-shared.kc_mnr7r/`.
Earlier Platform 0.3 and Shared 0.25.3 receipts remain at
`/tmp/ic-timers-platform-03.pbk51_dm/` and `/tmp/ic-timers-shared-0253.kz5bdnmj/`;
they do not qualify the changed Platform runtime. Native macOS, live IC and the
complete maintainer-operated release gate remain separate. Version bumps, staging,
commits, pushes, tags and publication remain user-owned.

Historical evidence remains in [archive-2026-10-20.md](archive-2026-10-20.md), the
[2026-10-18 archive](archive-2026-10-18.md), [Shared history](../evidence/shared-tooling-history.md)
and [release history](../evidence/release-history.md). Historical pending statements
are not authority to restore retired paths.
