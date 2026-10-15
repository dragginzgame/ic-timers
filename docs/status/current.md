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
`314a1a97854be8523e54273284533de4dd5f6f58` from
[Shared PR #227](https://github.com/dragginzgame/shared-tooling/pull/227), stacked
on #224 and building on merged #217/#218. Its 133 payloads include common hosted
preparation, Make-fixture setup and focused verifier/evidence fixtures; the
earlier handover retired eighteen copies and two manifests.
The [adoption guide](../shared-tooling.md) records ownership and qualification;
its [full prior receipts](../evidence/shared-tooling-history.md) remain historical.

Shared's read-only changelog preflight replaces the local independent writer;
coordinated metadata preparation remains the sole writer. Its generic fixture
is upstream; Timers keeps actual bump/rollback and source/index assertions.
Workflow bodies use stable IDs and Shared's unique complete-input extractor.
Timers retains ordering, logging, tag and upload assertions. One corruption
fixture tests the single export, restoring each mutated payload before the next.
Timer runtime source and semantics are unchanged by these reductions.

Testkit 0.46 multi-result helpers replace four local raw transport/codec flows;
reply shapes and the raw provider-ingress rejection test remain. Collector EXIT
probes reuse Shared's renderer, preserving all six failure/completion cases.
Explicit local/hosted Rust setup shares one installer; `update-dev` prepares
MSRV Clippy/rustfmt and Wasm too, before common tools. Checks remain offline.

The release-prose advisory now calls its canonical checker directly. The local
forwarder and forwarding-only fixture are removed; version preparation still
proves argument binding, pre-mutation execution and non-blocking failure.

Version and graph callers now use canonical readers directly. Two forwarding
scripts and the graph adapter-only fixture are removed; stable-version admission,
worktree selection and original failures remain at actual preparation/tag callers.
Generic snapshot/archive rejection cases have one Shared owner. Timers retains
actual exported payload corruption, collector integration, identity and pin checks.
The common hosted action and its execution assertions now belong to Shared;
six Timers fixtures use its Make bootstrap while retaining local inputs and checks.
Tracked Python bytecode is removed and interpreter cache directories are ignored.

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
probe Clippy passes on prepared Rust 1.99. Follow-up at `a18a288`: explicitly
installing Rust 1.91 Clippy resolves the missing component; complete offline
`testing-check` passes, including formatting and all four cohort lint selections.
Moved-link results retain their original source scope in the evidence directory.

This follow-up's before-images, canonical preview/apply and focused logs are at
`/tmp/ic-timers-next-reductions.7p99r2wx/`. Rust 1.91 probe compilation/strict
Clippy and protocol tests pass. Shared preparation and Timers setup/collector/
completion fixtures pass on Bash 5 and genuine Bash 3.2; affected ShellCheck,
actionlint and snapshot integrity pass. Installations were replaced with stubs.
This changes harness/setup code, not published runtime behavior or canister Wasm.

The next reduction's evidence is at `/tmp/ic-timers-probe-reductions.461i34bo/`.
Prose/version/completion fixtures pass on Bash 5 and genuine Bash 3.2. The
`pending-testkit-047-driver.patch` applies cleanly and passes isolated native
compilation/Clippy against the pending producer helper, preserving advancement
and tick counts. Normal adoption awaits Testkit #98 publication; selected 0.46
has no such API. The instruction-helper move was rejected: it broke native
probe compilation and increased two optimized cohorts by 478 and 203 bytes.
All five measured artifacts remain isolated; no dependency,
probe sampling or optimization policy change is applied from that experiment.

This verifier batch's before-images, preview/apply and focused logs are at
`/tmp/ic-timers-verifier-reductions.4sv8ej5n/`. Shared verifier/distribution/
retention and Timers snapshot/completion/collector/version/tag/index/committed
fixtures pass on Bash 5 and genuine Bash 3.2. Both Python suites, affected lint,
snapshot integrity and locked graph admission pass. No Rust or dependency change
is part of this batch. Normal index, refs and incoming Cargo inputs are preserved.

Common preparation and fixture logs are at
`/tmp/ic-timers-common-fixtures.sq_01fs0/`. Shared action/Make-consumer/retention
and affected Timers fixtures pass on Bash 5 and genuine Bash 3.2, with substitute
installers. These repository-only reductions leave Rust source and Cargo inputs
unchanged; they do not qualify hosted execution or native macOS.

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
