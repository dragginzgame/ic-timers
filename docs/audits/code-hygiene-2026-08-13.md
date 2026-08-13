# Code-hygiene audit — 2026-08-13

## Summary

Post-fix risk score: 2/10. The crate is small, its direct provider dependency is
isolated, production code has no panic or unsafe path, and the candidate 0.2
snapshot values are inert rather than runtime authority. The remaining risk is
primarily unreviewed cross-consumer API semantics, not mechanical hygiene.

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

## Audit findings

No high-severity findings were identified.

The candidate 0.2 API still needs Canic and IcyDB review of label bounds,
policy/mode semantics, failure-streak reset behavior, interruption epoch
attribution, and adapter ergonomics. This is design work and remains explicitly
deferred before registry implementation.

The release helpers do not yet have simulated failure-path tests comparable to
the more mature peer repositories. They are maintainer-owned, small, and do
not publish packages automatically, so that machinery is deferred until a real
0.2 release/publish flow is requested.

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
