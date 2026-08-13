# Changelog

All notable changes to this project are recorded here.

## [Unreleased]

### Added

- Add a proposed 0.2 canonical snapshot and observability contract requiring
  semantic parity with Canic's existing timer operator surfaces.

### Changed

- Rework the README to identify `ic-timers` as a wrapper around
  `ic-cdk-timers` and explain the shared coordination and recovery rationale.

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
