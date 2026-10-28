![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-28

## Source and accepted work

Verified remote/local main: **0.31.15**, `4a77ac185bf71489efcbc60ed9be15bea2a19c05`.
The maintainer requested latest Platform/Wire and IC Tooling adoption, plus
continued issue work.
The existing Platform/Wire adoption and supplier selections are preserved.
The Tooling-fix follow-up raises all five Tooling catalog minimums to **0.69.6**;
its sole additional lock change is Build 0.69.4 → 0.69.6. The incoming lock's
Platform 0.7.3 and Host/Testkit 0.69.6 selections remain intact. Cargo package
versions remain 0.31.15; the undated **0.31.16** draft covers this batch. No staging,
commits, pushes, package version mutation, release commands or sibling source
edits ran.

## Published dependencies and boundary

Root catalog minimums are **Platform 0.7.2** and **Wire 0.7.19**, including
matching macros. The incoming lock now resolves Platform **0.7.3** and Wire
**0.7.19**; those selections are preserved. The selected Platform CDK source is
unchanged from 0.7.2. The earlier Platform/Wire publication/source review remains
in `/tmp/ic-timers-platform-wire-03116.PlTAOv/` with its original graph scope.

The library still selects only Platform `executor`/`ic0`; both selected source
subtrees, runtime adapters and entrypoint macro bodies are unchanged from 0.7.1.
Wire adds caller-slice/payload APIs and metadata improvements; existing framed
codec bodies, probe method identities and resource limits remain unchanged.
No local wrapper, new application call dependency or codec migration was added.
The library and both Wasm probes remain Candid/full-CDK-free. Executor **2.0.0**
and ic0 **1.2.0** remain singular transitive Platform-owned identities. Candid
**0.10.39** remains in the native test worker's external PocketIC/control graph;
the normal build coordinator excludes it. Storage/memory remain out of scope.

Shared **0.27.3** (`7ee5c83744aa822ecd070703dbf0a7d95076d5f6`) is unchanged.
Metrics stays **0.16.3**, PocketIC **16.1.0** and Thiserror **2.0.18**.
Official crates.io sparse-index records verify **0.69.6** is the highest
non-yanked stable release for all five used Tooling packages: Build,
Host Artifacts/FS/Process and Testkit. Their archives/checksums and source
revision `cba072ee8fe288eb92bb4a4ddf9753b8abc4b580` were inspected. Rust 1.91
remains supported. No unpublished supplier checkout is selected.

## macOS source-inventory correction

Main CI [40681579067](https://github.com/dragginzgame/ic-timers/actions/runs/40681579067)
passed Linux checks/MSRV but failed both macOS tool-preparation jobs: released
Build 0.69.5 put Darwin u16 mode bits into a u32 source record (E0308).
Canonical owner: [IC Tooling #262](https://github.com/dragginzgame/ic-tooling/issues/262).

Published **0.69.6** retains the native `Mode` through inventory creation and
converts its bits only at numeric JSON serialization, preserving the permission
mask and stored shape. The temporary Build 0.69.4 exact pin and its exception
are removed; the reviewed exception list is empty again. No local cast, fork
or weaker host gate was introduced. The selected 0.69.6 release editor passes
explicit setup/admission and the actual Timers preparation fixture. Testkit
0.69.6 retains server 16.1.0 and the canonical setup/check path.
Fresh native macOS execution remains unverified locally; Linux checks do not
qualify either required macOS host.

## Issue delivery and next work

- **#77 is closed**: 0.31.15 delivered the required installed queued-scheduler
  stop/restart regression. Fresh qualification on the final graph passes all six
  initial/successor and owner-mode trajectories, preserving registration identity,
  completed work and unacknowledged state. Earlier causal negative control remains
  at `/tmp/ic-timers-scheduler-pressure.YxSpot/`. Scheduler-body normal return is
  still the protocol assumption; arbitrary traps/permanently live futures are not
  qualified. This closure grants no uncertain-effect replay authority.
- **#78/#80 are closed**: global-timer conflict detection and the application-call
  ownership boundary are delivered. Detection retains its documented limitations.
- **#79 remains open** for Canic consumer delivery. Its ingress/cancellation patch
  is prepared and qualified at `/tmp/ic-timers-backlog-03114.GlZdEf/canic` with
  `canic-code.patch` in the parent directory. Original Canic main is still
  `5c236ab130c0d63892ef281b45bdc3fa353e012f`; its dirty working checkout was untouched.
  Canic #695 likewise awaits delivery; #688/#689 were already closed.
- **Canic #700's supply blocker is resolved**: published **ic-call 0.7.2** owns
  dynamic method/argument custody over Platform 0.7.2. A registry-only standalone
  witness passes Rust 1.91 native execution and Wasm compilation, with singular
  Platform/executor/ic0 and no Candid/full CDK. It is not added below Timers.
  The public Call/CallBuilder/CallResult retirement is a breaking minor; Canic's
  delivery-cadence Human Minor Closeout Gate requires a maintainer-requested and
  accepted 0.112 closeout before next-minor implementation. This exact blocker
  and the witness are recorded on #700. Platform #75 also records publication
  and its migration document's stale unpublished-supplier sentence. No new
  compatibility builder or mirror waiting issue was introduced.

## Current qualification

Current Tooling evidence: `/tmp/ic-timers-tooling-fix-03116.OVRUJz/`.
Passes: Rust 1.91 locked/offline metadata; strict coordinator Clippy; all three
build-owner native tests; canonical editor/Testkit setup and admission;
version preflight/rollback/dirty-worktree preparation; formatting; dependency
pin checks; direct Candid/CDK boundary and library/probe/protocol/coordinator
transitive feature/identity exclusions; changelog admission and diff checks.

A disposable registry-only Build 0.69.6 consumer built the actual optimized
runtime probe with Rust 1.91 and retained its build evidence through four
installed PocketIC subjects: all six scheduler recovery trajectories, malformed
Wire admission, ingress/foreign-call authentication and response-trap cleanup.
All four passed and source/build evidence remained current throughout.
Retained optimized Wasm SHA-256:
`48d8b7f4800d89b9bff4595b3b14ab56b4af04b94b4a38e3b6f0b0917979274a`.
Testkit 0.69.6 owns the admitted server; Platform 0.7.3/Wire 0.7.19 are the
resolved probe graph. The disposable witness is evidence, not a new maintained
integration or alternate build owner.

Earlier library/probe/protocol strict lint and library-only MSRV Wasm evidence
remain at `/tmp/ic-timers-platform-wire-03116.PlTAOv/` with their original
Platform 0.7.2 graph scope. Native macOS, full local release gates and live IC
were not rerun. Preserve all normal maintainer-operated release/host requirements.
