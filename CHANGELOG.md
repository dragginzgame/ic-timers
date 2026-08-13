# Changelog

All notable changes to this project are recorded here.

## [Unreleased]

### Added

- Add the candidate 0.2 provider-neutral `snapshot` API, including bounded
  structured timer identities, policy and scheduling state, outcomes,
  counters, performance measurements, runtime epochs, and one canonical
  snapshot type.
- Preserve operational distinctions required by Canic migrations: callback
  starts versus completions, completed versus interrupted work, requested
  schedules versus platform arms, and functional consecutive expected-failure
  state.
- Add saturating counter transitions, explicit observation resets, portable
  directive conversion, and total/latest/maximum instruction and elapsed-time
  aggregates.
- Add focused tests for identity validation and ordering, outcome transitions,
  counter partitions, absent measurements, epoch resets, and a Canic-shaped
  lossless projection fixture.
- Add the 0.2 observability and Canic parity contract, an explicit safety
  boundary, and a recurring code-hygiene audit.
- Add Dependabot coverage for Cargo and GitHub Actions dependencies.
- Add tested release automation that promotes `Unreleased` notes into a dated
  version section, plus a guarded crates.io publish target.

### Changed

- Rework the README to identify `ic-timers` as a wrapper around
  `ic-cdk-timers` and explain the shared coordination and recovery rationale.
- Harden the development gate with expanded Clippy lints, rustdoc, shell
  syntax, and pinned GitHub Actions checks.
- Mark public validation errors as non-exhaustive and document the pre-1.0
  public API compatibility policy.

### Not yet implemented

- Registry storage and live inventory population, measured callback execution,
  runtime adapters, lifecycle reconstruction, and recovery watchdog behavior
  remain future work.
- The candidate API remains subject to Canic and IcyDB review; a real downstream
  Canic adapter test is required before the 0.2 contract is accepted.

## [0.1.0] - 2026-08-13

### Added

- Initial workspace, CI, release helpers, and formatting hook.
- Canic-derived deterministic timer control and one-shot platform boundary.
- Architecture and release-line documentation for the planned shared runtime.

### Changed

- Use Rust 1.97.1 for normal development while retaining Rust 1.91.0 as the
  separately checked minimum supported Rust version.
- Document the staged path from public timer snapshot types to the registry,
  lifecycle integration, and recovery watchdog.
