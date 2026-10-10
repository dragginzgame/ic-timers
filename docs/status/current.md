![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

This compact handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and acceptance

Latest release commit **0.16.7** is `999d9b5c3a84ec5abd729ca72b8f259abbb060e1`.
The finalized notes and matching tag job exist. Its released graph selects
Testkit 0.29.0, Host 0.11.0, Metrics 0.3.7 and PocketIC 16.1.0, four local 0.16.7
members and Shared 0.2.14 snapshots (55/30/11). Matching
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38042864177) passes
Linux/MSRV, with Intel running and ARM queued at inspection;
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38042864373) passes.
Do not count this source as complete native acceptance or redispatch jobs.

Released **0.16.6** now has all-host and tag acceptance. Downloaded raw logs
prove its actual lock-preservation, directory/preflight/adapter cases, library
and product gates. [The source-bound owner](../releasing.md#0166-release-acceptance)
records that scope; #35 is closed with its remaining proof complete. Earlier results do not
qualify the new Shared selection or incoming dependency graph.

## Pending 0.17.0

The undated draft adopts committed Shared **0.3.0**
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` through all three canonical snapshots,
retaining 55/30/11 files. The [adoption owner](../shared-tooling.md#shared-tooling-030-complete-toolset)
records complete host → IC → Cargo aggregates, followed by existing Testkit
setup/check targets. CI/update-dev prepare Rust first; the separate global
cargo-sort route is removed. CI retains complete aggregate failure logs.
Local parallel-Make extension and collector assertions are written, not run.
[#36](https://github.com/dragginzgame/ic-timers/issues/36) owns delivery/qualification.

Incoming maintainer Cargo edits select Metrics **0.4.0**, Testkit **0.30.0**,
Host **0.11.0** and PocketIC **16.1.0**. They also report local version **0.16.6**,
although the preceding release is 0.16.7; preserve those user-owned bytes. Contributor
preparation does not repair or bump Cargo metadata. The next user-operated minor
preparation must select **0.17.0**.

The minor boundary covers the changed tooling contract and Metrics' public Rust
package identity. Consumers exchanging public summaries must use Metrics 0.4 or
`ic_timers::MeasurementSummary`. [The identity/graph owner](../design/callback-delivery-ownership.md#ic-metrics-04-adoption)
records unchanged arithmetic, published source comparisons and remaining execution
qualification. [#37](https://github.com/dragginzgame/ic-timers/issues/37) stays open
for delivered alignment. Testkit's existing startup/CLI calls need no adapter;
its packaged CLI lock selects Host 0.11, and the timer library has no Host/Testkit
edge. No timer runtime mechanism, persistence or optimization is added.

The maintainer's minor-release attempt stopped before validation because README
examples did not match Cargo 0.16.6. The prepared repair removes the local
freshness helper/test and all phase/CI callers, automatic rewriting and README
release-output ownership.
Only Cargo.toml, Cargo.lock and CHANGELOG.md remain release outputs. Ordinary
README edits retain source/staging guards; stale or missing examples are accepted.
Existing preparation/Git/selected-commit fixtures cover that distinction and
README preservation; they are written, not run. The
[release owner](../releasing.md) records the contract and
[Shared #100](https://github.com/dragginzgame/shared-tooling/issues/100) requests a
periodic read-only advisory task. No Cargo or lock bytes are changed by this repair.

## Next action and authority

Permitted integrity/source/syntax/documentation/locked-metadata checks pass.
Leave the normal full gate to the maintainer. New source has no hosted result;
Linux/Bash source inspection does
not qualify macOS. All tests/builds/lint/setup and Cargo/release effects remain
user-owned. Preserve pins, index, existing installations and failure evidence.
The tooling alone is repository-only work; incoming Metrics identity alignment
is crate-impacting and must not be published as a 0.16 patch.
