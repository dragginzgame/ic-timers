![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-15

## Prepared 0.25 compiler and contributor-owner cleanup

Released base is 0.24.0 at `50b535a5936e885a727e6dade20f029d8a9113c2`.
The maintainer requested Rust 1.91.0 as MSRV. Manifests, Make/probe commands,
CI and current consumer instructions now agree. The supported build requirement
change uses one undated 0.25.0 draft; Cargo package versions remain unchanged.
Incoming Metrics 0.11, Testkit 0.46 and Host 0.19 Cargo selections are preserved.

One canonical snapshot selects reviewed
`61ddfdc25ad0f4342b5e324e5798897f16c43996` from
[Shared PR #217](https://github.com/dragginzgame/shared-tooling/pull/217).
The verified handover retires eighteen nested copies and two old manifests;
127 payloads retain the prior canonical union plus four new helper/fixture roots.
The [adoption guide](../shared-tooling.md) records ownership and qualification;
its [full prior receipts](../evidence/shared-tooling-history.md) remain historical.

Shared's read-only changelog preflight replaces the local independent writer;
coordinated metadata preparation remains the sole writer. Its generic fixture
is upstream; Timers keeps actual bump/rollback and source/index assertions.
Workflow bodies use stable IDs and Shared's unique complete-input extractor.
Timers retains ordering, logging, tag and upload assertions. One corruption
fixture tests the single export, restoring each mutated payload before the next.
Timer runtime source and semantics are unchanged by these reductions.

## Evidence and remaining acceptance

Before-images, canonical preview/apply and focused logs are retained at
`/tmp/ic-timers-owner-consolidation.eggohebc/`. Rust 1.91 library native/Wasm
and probe compilation, 155 native tests and doctests pass. Shared's new owner
fixtures and existing retirement checks pass on Bash 5/genuine Bash 3.2;
full snapshot distribution, affected ShellCheck and owner doc links pass. The
interrupted-publication retry fixture also passes on both shells.
Timers' completion, corruption, failure evidence, version rollback, committed/tag,
index, gate and repository fixtures pass; the six directly affected Bash 3.2
subjects also pass. All four cohorts and the runtime probe compile to Wasm on 1.91. Strict library/
probe Clippy passes on prepared Rust 1.99; the 1.91 Clippy component is absent,
so its lint gate remains unrun. Moved-link results are retained with their source
scope in the same evidence directory.

The last recorded all-host acceptance remains [0.23.1](../evidence/release-history.md#completed-0231-native-acceptance),
not this source/compiler/graph. Full owner/native macOS, hosted upload and live
PocketIC qualification remain separate. No setup, hosted dispatch, full release
gate or release execution ran. Git index/refs and sibling normal checkouts remain
untouched; the isolated Shared contribution branch alone was committed/pushed.

[Snapshot #63](https://github.com/dragginzgame/ic-timers/issues/63) has its local
implementation prepared and upstream PR linked; delivered/native-host acceptance
remains outstanding. Earlier release and license receipts retain their source
scope in [release history](../evidence/release-history.md).
Normal staging, commits, versions, tags, publication and cleanup stay user-owned.
