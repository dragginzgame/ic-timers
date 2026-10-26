![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-26

## Source and pending work

Released base: **0.31.4**, `b04401c`. The undated **0.31.5** section covers
published Build/Host/Testkit **0.68.7** and Shared Tooling **0.26.20** adoption.
All five Tooling packages were already selected in the incoming lockfile;
it is preserved byte for byte. Registry metadata and checksum-verified published
archives establish availability and Rust 1.91 support. Cargo package versions
remain 0.31.4; commits, staging, pushes and release execution remain
maintainer-owned. No timer API or probe wire-contract change is required.

## Ownership and adoption

Shared's canonical exporter applied revision
`a7f85d45d4212d4c39225f58597580560a333550` with unchanged roots and 136 payloads.
The compact baseline adds task-specific engineering details, focused batch
validation and opt-in line/symbol reporting. Selected executable helpers are
unchanged. Optional npm checks remain unselected; Timers has no npm consumer.
Normal releases start fresh; saved-attempt recovery remains `release-resume`.

Published Tooling 0.68.7 consolidates Cargo completion tracking and native error
sources without changing public APIs or report/cache formats. Testkit now refuses
synchronous instance completion after the startup deadline, matching async
admission. Server ownership and cleanup remain upstream. Host Rust sources are
unchanged. Both lock-selected CLIs use canonical explicit setup and offline
admission; no moving sibling implementation or local replacement is introduced.
Testkit's selected server remains PocketIC **16.1.0**.

The library still uses only Metrics, Platform and Thiserror directly. Platform
**0.6.18**, Wire **0.7.16**, Metrics **0.16.2** and Thiserror **2.0.18** remain
selected. Executor **2.0.0** and IC0 **1.2.0** identities remain unique. Library
and probe deployment graphs exclude Candid and host tooling; native
Testkit/PocketIC transport intentionally retains Candid. The unpublished
Wire/Serde protocol package owns probe records, not runtime authority.

## Verification and limits

Evidence, published archives, source diffs, exporter logs and incoming lockfile
are retained in `/tmp/ic-timers-adoption-0315/`. Rust 1.91 Clippy passes for the
affected host runner's normal and test targets. All six host build/report tests
pass. The actual cohort driver builds four optimized policy Wasms, runs the
Linux PocketIC subject and publishes a validated report for four policies plus
two watchdog calibrations. No baseline was replaced and no performance
improvement is claimed.

Both 0.68.7 CLIs were explicitly prepared and passed offline admission. Release
version-preparation rollback/dirty-worktree and Testkit adapter setup/check
failure-propagation fixtures pass. Snapshot integrity/corruption, provider,
direct and deployed dependency boundaries, pins, formatting, changelog and
diff checks pass. No missing enforcement was identified for this adoption.
Earlier Platform/Wire evidence remains in `/tmp/ic-timers-platform-wire-0314/`
and is not relabelled for the new Tooling selection.

The complete release gate, native macOS and live deployment remain separately
required. Local Linux PocketIC evidence does not qualify those environments.
