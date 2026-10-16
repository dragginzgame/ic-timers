![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-16

## Prepared 0.25.1 probe and evidence cleanup

Starting source is 0.25.0 at `11e73bff33676e6d829baa502fb943ffa312b988`.
One undated 0.25.1 draft covers this batch. Cargo package versions and the incoming
root lockfile changes are preserved. The workspace selects Rust 1.91, Metrics
0.11 and Testkit 0.47; no dependency update belongs to this batch.

One canonical snapshot selects `12eb03d28ba8814000639091a44392d08db21657`
from [Shared PR #234](https://github.com/dragginzgame/shared-tooling/pull/234),
stacked on #230/#227/#224. The 133 payloads keep the same entrypoints. Shared now
accepts Timers' prepared archive and original collection log; the private upload
action is removed. Product evidence selection and downloaded source/run checks
remain local. Shared uploads available files before enforcing completeness.
See [adoption boundaries](../shared-tooling.md) and [release evidence](../releasing.md#ci-failure-evidence).

The runtime probe stores one existing Watchdog decision instead of three pending
flags. The latest control request wins; normal execution consumes it once, after
the failure window. Stop clears saved scheduling intent only when consumed.
Native regression dispatch joins the existing `make test` target; the exact-
deadline PocketIC subject also exercises superseded Stop/immediate requests.

The size probe uses one private instruction bracket returning the result and
instruction count. Wire records, sampling boundaries and optimization policy
remain local. Published runtime code and API are unchanged.

Follow-up cleanup inlines three single-use native Candid wrappers. One compile-
time cohort assertion replaces the missing-feature and pairwise conflict guards;
all four single-feature selections compile natively and for Wasm, and all twelve
invalid combinations fail the count assertion. Optimized cohort byte sizes are
unchanged by this follow-up. The release-gate fixture now accounts for the probe
native test command added in the preceding batch.
The three affected PocketIC subjects (cycle depletion, inventory measurement and
policy cohorts), Rust 1.91 probe compilation and Rust 1.99 probe Clippy pass.

The Shared Testkit CLI adapter is prepared against the earlier `e5430e1` source
at `/tmp/ic-timers-cli-cleanup.5_htzoxa/shared/`. Its reviewed diff, PR description
and qualified Timers caller patch are alongside that directory. Generic CLI
admission/offline/failure checks move upstream; exact local server directories,
aggregate ordering and jobserver checks remain with Timers. Owner and disposable
consumer fixtures pass on Bash 5 and genuine Bash 3.2. Contribution commit/PR
and canonical adoption await the explicit approval required by AGENTS.md; the
normal adapter and selected snapshot remain unchanged by this pending work.

The next follow-up removes the tag-selection adapter. Make resolves current
HEAD/version with separately checked observations and passes frozen release
identities directly to Shared's exact checker. Tag/publication/workflow,
committed-release and release-gate fixtures pass on Bash 5 and genuine Bash 3.2.
Probe fatal-result matches now use the same explicit IC traps through
`unwrap_or_else`; native/Wasm compilation, probe Clippy and trap/upgrade,
inventory and policy-cohort PocketIC subjects pass. This follow-up reduces
Baseline/Once/Watchdog artifacts by 7 bytes each and After-completion by 49.
After-completion start/snapshot intervals decrease by 70/72 instructions and its
work envelope increases by 2; other cohort instruction subjects are unchanged.
These are probe observations, with no published runtime or optimization-policy change.

The approved five-file Shared exit-runner contribution is committed in #234 and
canonically adopted at the exact source above. It centralizes completion-variable/
scratch-root handling and preserves return-based assertion guards. Timers now
uses that runner for its thirteen-source completion roster and collector exits,
retaining local scratch contexts, diagnostics and actual evidence integration.
Before-images, export preview/apply and focused qualification are retained at
`/tmp/ic-timers-exit-tag-cleanup.bi2qkcb8/adoption-turn/`. This adoption does not
include the separately pending Testkit CLI contribution above.
The actual adopted completion/collector fixtures pass on Bash 5 and genuine
Bash 3.2; affected ShellCheck, snapshot integrity and local doc links pass.
The normal Git index/refs, Cargo inputs and toolchain selection are unchanged.

## Evidence and limits

Before-images, canonical preview/apply and focused logs are retained at
`/tmp/ic-timers-probe-actions.p74abcf5/`. Rust 1.91 native control tests and
optimized Wasm builds pass; Rust 1.99 probe Clippy passes. The four-cohort
before/after PocketIC comparison uses the same incoming root lock, Rust 1.91,
`timer-probe` profile and admitted PocketIC 16.1.0 server:

| Cohort | Before Wasm bytes | After | Delta |
| --- | ---: | ---: | ---: |
| Baseline | 254056 | 254159 | +103 |
| Once | 309725 | 308870 | -855 |
| After-completion | 309846 | 309718 | -128 |
| Watchdog | 311284 | 311284 | 0 |

Start, duplicate scheduling, cancellation, snapshot, scheduler/work, memory-sampling
and calibration instruction subjects are unchanged. Inventory changes by +1
instruction for Baseline and -2 for Once; the other two are unchanged. These are
probe artifacts and intervals, not an application-Wasm optimization claim.

Shared host/tool-evidence and retention fixtures pass on Bash 5 and genuine Bash
3.2, using substitutes. Timers' collector/completion fixtures pass on both shells;
both downloaded-evidence Python suites, affected Clippy, ShellCheck, actionlint,
repository adapters, snapshot integrity and documentation checks pass. Four focused
PocketIC subjects pass: mixed decisions/exact deadlines, immediate continuation,
Stop/resume and trap recovery. Full owner/native macOS, hosted upload/download and
the complete release gate remain separate qualification. Normal Git index/refs and
Cargo inputs stay user-owned; only the isolated Shared PR branch was committed.

## Earlier receipts and remaining context

[Prior Shared adoption receipts](../evidence/shared-tooling-history.md) and
[release history](../evidence/release-history.md) remain source-bound historical
evidence. The last recorded all-host acceptance remains
[0.23.1](../evidence/release-history.md#completed-0231-native-acceptance), not this
source/compiler/graph. Snapshot [#63](https://github.com/dragginzgame/ic-timers/issues/63)
still requires delivered/native-host acceptance.

The earlier Testkit driver patch is retained at
`/tmp/ic-timers-probe-reductions.461i34bo/pending-testkit-047-driver.patch`;
0.47 is now selected, but that separate caller adoption is not part of this batch.
The earlier instruction-helper move remains rejected: it broke native compilation
and increased two cohorts. This batch keeps measurement code in the probe.
