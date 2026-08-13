# Current status

Last updated: 2026-08-13

## Purpose

This is the compact handoff for a new session. The repository is an initial
pre-alpha scaffold for a shared Internet Computer timer runtime.

## Current foundation

- Workspace package version: `0.1.0`.
- Direct timer provider: exact `ic-cdk-timers` 1.0.0.
- `control` contains the Canic-derived deterministic state machine for
  generations, stale callbacks, cancellation, scheduling, and reconciliation.
- `platform` is the only direct provider boundary and exposes asynchronous
  one-shot arm and clear operations.
- `schedule` owns typed post-run directives and checked deadlines.
- Basic GitHub CI, one formatting-only pre-commit hook, SemVer release helpers,
  development-tool setup, README, changelog, and architecture docs exist.
- Normal development is pinned to Rust 1.97.1; Rust 1.91.0 remains the
  separately checked minimum supported Rust version.

The initial local gate passes formatting, native check and Clippy, 15 unit
tests, Wasm compilation, and offline package verification on Rust 1.97.1. The
native MSRV check also passes on Rust 1.91.0.

## Not implemented

- The shared canister-local timer registry and inventory snapshot.
- Measured callback execution and metrics adapters.
- Pre-armed recovery watchdog recurrence.
- Upgrade reconstruction and lifecycle participant composition.
- PocketIC recovery, isolation, ingress, and insufficient-cycle evidence.

Do not migrate recovery-critical consumers to this crate until the relevant
guarantees and tests exist.

## Next action

Specify bounded identity, scheduling-policy, execution-state, outcome,
measurement, and snapshot value types first. Settle their ordering and public
serialization shape in unit tests. Then implement one pure serial registry
above `TimerControl`, with duplicate detection and deterministic snapshots,
before connecting it to the platform boundary.

The maintainer owns release tags and all package-publication actions.
