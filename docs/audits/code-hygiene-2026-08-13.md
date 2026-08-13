# Code-hygiene audit — 2026-08-13

This report began at the 0.2 value-model boundary. The 0.3 runtime closeout
addendum below supersedes statements that the crate has no authority-bearing
registry or Wasm measurement subject.

## Summary

0.2 post-fix risk score: 2/10. The crate was small, its direct provider
dependency was isolated, production code had no panic or unsafe path, and the
candidate 0.2 snapshot values were inert rather than runtime authority. The
remaining risk was primarily unreviewed cross-consumer API semantics, not
mechanical hygiene.

## Improvements adopted

- Enabled Clippy's nursery group and focused API/style lints; the complete
  workspace passes with warnings denied.
- Added rustdoc warnings and broken intra-doc-link checks to the normal gate.
- Added shell syntax and full-SHA GitHub Actions checks to the normal gate.
- Added Dependabot coverage for Cargo and GitHub Actions dependencies.
- Marked public error enums non-exhaustive so adding a classified error does
  not force downstream exhaustive matches.
- Documented the pre-1.0 hard-cut policy and the distinction between inert
  snapshots and timer authority.
- Added `SAFETY.md` so implemented guarantees, missing recovery behavior, and
  required PocketIC evidence have one canonical boundary.
- Added a tested changelog finalizer and a guarded crates.io publish target for
  the requested 0.2 release flow.

## Audit findings

No high-severity findings were identified.

The candidate 0.2 API still needs Canic and IcyDB review of label bounds,
policy/mode semantics, failure-streak reset behavior, interruption epoch
attribution, and adapter ergonomics. This is design work and remains explicitly
deferred before registry implementation.

Compile-fail capability tests and Wasm size budgets were also deferred. The
crate does not yet expose an authority-bearing registry capability, and a size
budget would be arbitrary before the runtime path exists.

## Evidence

The audit covered the public API, production panic paths, validation tests,
dependencies, provider imports, package contents, CI workflow pins, release
helpers, and documentation claims. The local CI gate, MSRV check, rustdoc,
Wasm check, tests, and offline package verification pass after the adopted
changes. `cargo tree --duplicates` found no duplicate dependency versions, and
`cargo audit` scanned 13 locked dependencies against a fresh 1,216-advisory
RustSec database without reporting a vulnerability.

## 0.3 runtime closeout addendum

Post-fix risk score: 3/10. The substantial registry/runtime implementation is
bounded and well isolated, but its larger state machine and pre-1.0 downstream
API surface warrant IcyDB and Canic review.

- Production scans find no explicit `panic!`, `unwrap`, `expect`, `todo`, or
  `unimplemented` path; matches remain in tests only.
- The private `platform` module is still the sole direct `ic-cdk-timers`
  caller, and exact provider handles are linear.
- The normal dependency graph has no duplicates. Exact `ic0` 1.1.0 is the only
  added direct dependency and was already transitively resolved.
- Snapshot construction remains registry-only; callback tokens and provider
  handles remain private; the crate exports no lifecycle or Candid endpoint.
- Fifty-six native tests cover pure/live owner behavior. PocketIC 15 supplies
  the separate commit/rollback and isolation matrix.
- Rust 1.88 native/Wasm compilation, strict Clippy, rustdoc, offline package
  construction, and the normal local CI gate pass.
- Comparable raw-Wasm, instruction, cycle, and complexity values are recorded
  in the [0.3 runtime evidence report](0.3-runtime-evidence-2026-08-13.md).

The remaining risk is downstream integration: Canic must prove its existing
operator surfaces project without parallel instrumentation, and IcyDB must run
its maintained recovery/admission suites against the adapter before either
removes its current timer authority.
