# Current status

Last updated: 2026-08-13

## Purpose

This is the compact handoff for a new session. The repository is an initial
pre-alpha scaffold for a higher-level wrapper around `ic-cdk-timers` and a
shared Internet Computer timer runtime.

## Current foundation

- Workspace package version: `0.1.0`.
- Direct timer provider: exact `ic-cdk-timers` 1.0.0.
- `control` contains the Canic-derived deterministic state machine for
  generations, stale callbacks, cancellation, scheduling, and reconciliation.
- `platform` is the only direct provider boundary and exposes asynchronous
  one-shot arm and clear operations.
- `schedule` owns typed post-run directives and checked deadlines.
- `snapshot` contains the candidate 0.2 provider-neutral identity, policy,
  state, outcome, counter, performance, epoch, and canonical snapshot values.
- Basic GitHub CI, one formatting-only pre-commit hook, SemVer release helpers,
  development-tool setup, README, changelog, and architecture docs exist.
- Version bumps automatically promote populated `Unreleased` notes to a dated
  release heading; the guarded publish target requires a clean tagged `HEAD`.
- CI also checks rustdoc, shell syntax, and full-SHA GitHub Actions pins;
  Dependabot covers Cargo and Actions dependencies.
- `SAFETY.md` is the canonical boundary for implemented guarantees and missing
  PocketIC recovery evidence; a recurring code-hygiene checklist and initial
  audit report exist under `docs/audits`.
- The 0.2 observability contract and a local Canic-shaped projection test
  require the canonical snapshot to preserve Canic's timer status, counters,
  scheduling, and performance information without importing Canic-specific
  DTOs. A real downstream Canic adapter test remains required.
- Normal development is pinned to Rust 1.97.1; Rust 1.91.0 remains the
  separately checked minimum supported Rust version.

The local gate passes formatting, native check and Clippy, 32 unit
tests, Wasm compilation, and offline package verification on Rust 1.97.1. The
native MSRV check also passes on Rust 1.91.0.

## Not implemented

- The shared canister-local timer registry and live inventory population.
- Measured callback execution and metrics adapters.
- Pre-armed recovery watchdog recurrence.
- Upgrade reconstruction and lifecycle participant composition.
- PocketIC recovery, isolation, ingress, and insufficient-cycle evidence.

Do not migrate recovery-critical consumers to this crate until the relevant
guarantees and tests exist.

## Next action

Publish the 0.2 design slice, then review its
[observability contract](../design/observability.md) and public `snapshot` API
from Canic and IcyDB. In particular, validate the 64-byte labels, policy/mode
split, no-work failure-streak reset, interruption epoch attribution,
saturation, and adapter ergonomics. Revise and accept that contract before
implementing registry storage or runtime instrumentation.

The maintainer owns release tags and all package-publication actions.
