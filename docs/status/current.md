![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-25

## Source and pending work

Released base: **0.31.0**, `c7ff54c`. The undated **0.31.1** changelog section
covers the current complexity cleanup. Cargo versions and the root lockfile are
unchanged. Staging, commits, pushes and release execution remain maintainer-owned.

The accepted ten-item cleanup is implemented inside this repository:

- Fresh and existing lifecycle registrations share each policy's desired-state
  decision. Creation retirement and unused-factory drop ordering remain distinct.
- Watchdog completion selects its effective stop reason and exact-schedule
  metadata alongside command precedence, without another state-machine type.
- Native cancellation matrices share lifetime assertions; their policy-specific
  setup, generation checks and callback rejection cases remain.
- Ordinary and batch probes share one method-bound Wire gate exchange. Each
  caller retains its own loop, observations and error/trap handling.
- Mixed and primary probes share watchdog installation and fault mechanics.
  Mixed-policy timing and upgrade-without-durable-intent evidence remains separate.
- Cohort build context is separate from completed reports, whose measurements
  are required. Schema-2 JSON, strict admission and recipe checks remain. Each
  build lease stays with its own checked inputs. Duplicate metric text records
  are removed; JSON receipts and receipt-derived comparison output remain.
- Dependency-boundary fixture cases reset all maintained manifests and the lock
  together, instead of maintaining separate original-file copies.
- Shared's generic snapshot, checksum and workflow-parser matrices are retired
  from the release roster and snapshot through the canonical exporter. The local
  fixture-completion test remains: it checks actual Timers/macOS assertions.

## Ownership and maintained contracts

Metrics **0.16.0**, Build/Host/Testkit **0.68.1**, Platform **0.6.15**, Wire **0.7.2**
and Thiserror **2.0.18** remain selected. The library's three direct dependencies
are Metrics, Platform and Thiserror. Platform owns executor **2.0.0** and IC0
**1.2.0** access. Library/probe deployment graphs remain Candid-free; native
Testkit/PocketIC transport intentionally retains external Candid.

The protocol package owns Wire/Serde records only. Timer policy, authority,
provider-handle cleanup and callback recovery remain local. No public API,
protocol record, storage contract or dependency selection changes in this batch.
Native substitutes do not prove IC rollback; installed-IC evidence is separate.

Shared Tooling stays at reviewed revision
`4c0f198e56d4c6d27b12a5c2d515e0e01070151e`, now **134** payloads. The supplier
VERSION is 0.26.10 with committed 0.26.11 setup notes. No supplier files were
patched. CI caching remains disabled; all product and native-host gates remain.

## Verification and limits

Current logs, exporter impact/apply evidence and a separate cohort receipt are
in `/tmp/ic-timers-complexity-0311/`. Rust 1.91 Clippy passes for the library,
runtime probe and host runner; library Wasm checking and the optimized runtime
probe build pass. Focused native selections pass: 40 registry tests, 19
reconciliation selections and 14 watchdog runtime selections (overlapping
filters), all three receipt tests, all three build-driver tests and eight
doctests. Strict receipt admission also verifies that completing build context
preserves the schema-2 serialized shape.

Linux PocketIC passes 11 affected trap/exhaustion, mixed-policy upgrade,
ordinary-await/abandonment and maximum-capacity dispatch scenarios. The changed
cohort driver passes all four policy artifacts and both calibration subjects;
its separate receipt is `/tmp/ic-timers-complexity-0311/cohorts.json`. The existing
`testing/target/cohorts.json` and reviewed baselines were not replaced. Initial
sandbox server-start failures are retained; the passing retry used loopback
permission and the Testkit-admitted executable, with Cargo offline.

Snapshot/provider checks, actual snapshot corruption, local fixture completion,
release/repository wiring and real-Cargo dependency-boundary fixtures pass.
Earlier dependency evidence remains in `/tmp/ic-timers-deps-refresh/`; historical
handoff details remain in released Git history. Preserve all retained failures.

Native macOS, live deployment and the complete maintainer-operated release gate
remain unqualified by these focused Linux checks. No release operation was run.
