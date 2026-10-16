![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-16

## Local 0.26 owned timer backend

The [0.26 design](../design/0.26-owned-timer-backend.md) now has a local
implementation candidate. The old provider is removed from manifests and the
lock graph. Independently authored MIT code owns bounded one-shot scheduling,
authenticated self-call dispatch and eager cancellation; executor 2.0.0 and
ic0 1.2.0 remain selected. No upstream timer source was imported. This is not a
strict clean-room claim: the earlier audit inspected upstream source.

The backend admits 130 pending records and 128 outstanding dispatch contexts.
It returns typed capacity/identity failures, retries resource deferral after one
second, and keeps consumer work separate from the committed Watchdog scheduler.
Only the FFI module permits unsafe IC exports/callback registration; callback
contexts use bounded table indices rather than raw pointers. Cargo's links key
rejects packages declaring that same key; older releases require graph checks.
Composed canisters must remove other providers.
The runtime's registration APIs, volatile reconstruction and durable consumer
records remain intact. Jobs core needs no timer dependency or record migration.

Evidence is retained under `target/qualification/0.26-backend/`. Before-images
include the source commit, manifest/lock/toolchain and runtime/four-cohort Wasm.
Rust 1.91 passes 160 native library tests, the optimized runtime/four-cohort
Wasm builds and public Rustdoc with warnings denied. Rust 1.99 strict Clippy
passes library, runtime probe and PocketIC targets. All 18 affected PocketIC
subjects pass through Testkit's admitted 16.1.0 server, including malformed
self-call admission, consumed-identity replay and ordinary work surviving a
300-second dispatch timeout. The low-cycle test now proves the one-second
post-top-up delay. Failed attempts are retained: the sandbox blocked localhost,
the old low-cycle timing assertion failed, and the new malformed-payload fixture
initially encoded integers instead of bytes. None is counted as passing evidence.

Matched before/after cohorts use Rust 1.91 and the same timer-probe profile;
the lock diff removes only the timer provider and changes the library edge to
the already-selected executor. Wasm bytes change as follows:

| Cohort | Before | After | Delta |
| --- | ---: | ---: | ---: |
| Baseline | 253953 | 241052 | -12901 |
| Once | 308651 | 295578 | -13073 |
| After-completion | 309489 | 296428 | -13061 |
| Watchdog | 311078 | 297890 | -13188 |

Start instructions fall by 1483/1561/1561 for Once/After-completion/Watchdog.
Measured dispatch cycles fall by 2162/1974/5529. Cancellation instructions change
by -4/+702/+677: eager removal and timer rearming trade cancellation cost for
immediate queue reclamation. Some snapshot/inventory intervals also vary; the
complete reports retain them. These are probe observations, not an application
throughput guarantee. The final runtime churn subject stays at 19 Wasm pages
through 32768 replacements and 64 immediate continuations; it does not establish
a global memory cap or compare that whole modified probe against older artifacts.

The live provider-boundary/repository fixtures, source-following ShellCheck,
formatting, locked metadata and package-content checks pass. The runtime Wasm
exports one global timer and the new internal dispatcher. Package contents include
the MIT license and build script. Cargo remains 0.25.2; one undated 0.26.0 section
covers this batch. Index, refs, releases and sibling checkouts remain user-owned.

The cleanup follow-up is recorded under `target/qualification/0.26-cleanup/`,
with arrival source copies and current manifest/lock/toolchain/Wasm hashes.
The incoming workspace now selects Testkit 0.49 (including Host 0.22.1);
the earlier matched comparison above remains bound to its retained 0.48 graph.
The selected CLI was prepared through the repository setup target and admitted
PocketIC 16.1.0. All 19 recovery subjects pass on Rust 1.91, including a real
response-callback trap caused by a deliberately trapping capture destructor.
Both ordinary policies preserve their pending registration through cleanup,
retry after one second, retire after disabling the destructor trap, and permit
independent later dispatch. This exceeds the supported nontrapping-destructor
contract and proves only that specific failure sequence. The final probe builds;
affected strict Clippy and formatting pass. The churn subject still reports
19 pages throughout. No production runtime change belongs to this follow-up.

The transport follow-up is recorded under `target/qualification/0.26-transport/`,
with arrival sources and current input hashes. The unpublished probe maintains
real self-call queue pressure without changing production code. The final test
observes 100 CDK submission failures and 3000 replies while the timer remains
pending with more than half its initial cycles. Draining pressure does not bypass
the one-second retry delay; subsequent dispatch completes exactly once. With one
pending timer and ample cycles, this exercises nonzero backend call_perform by
source-level inference, without a production counter or injected return value.
The initial burst-only experiment drained too early and failed its deferral
assertion; that log is retained separately. The sustained-pressure subject and
all other recovery subjects pass (20 total) on Rust 1.91 / Testkit 0.49 / admitted
PocketIC 16.1.0. The optimized probe build, affected strict Clippy and formatting
pass; churn still reports 19 pages throughout. Production code is unchanged.

The deferred-control follow-up is recorded under
`target/qualification/0.26-deferred-control/`, with arrival sources and current
input hashes. Its focused PocketIC subject passes cancellation and replacement
during real submission deferral, stale authenticated delivery of the removed
backend identity, and rescheduling the retained cancelled registration. The
original cancelled work remains stopped beyond the retry deadline; replacement
work completes once. This uses the unchanged production runtime and probe Wasm.
Affected strict Clippy and formatting pass. An initial test incorrectly attempted
to register over the retained cancelled registration; its failure is retained,
and the corrected test uses the existing schedule API. The prior 20-subject suite
and this new focused subject are separate receipts, not a new combined suite run.

Remaining qualification is explicit: the backend's 128-context table saturation
is tested natively, independently of the IC queue pressure now covered on PocketIC.
An attempted real context-saturation fixture could not reuse running
registrations: unregister retains them until completion, so replacing batches
correctly rejects duplicate identities. That fixture was removed; its source and
failed compile/decoder/runtime attempts are retained only as experimental logs.
Jobs candidate composition now passes in a disposable copy under
`target/qualification/0.26-jobs/`. Arrival hashes cover the current dirty Jobs
0.13.2 source at HEAD `c497c0f7d2ac5c812c5592bb54a04b4d7350f15a`.
A scratch path patch selects this local Timers candidate, still versioned 0.25.2;
it does not constitute published 0.26 adoption. Rust 1.91 passes 42 native consumer
tests, builds Wasm, and passes all seven existing live recovery subjects through
Testkit 0.49 / admitted PocketIC 16.1.0. The resolved graph contains one Timers
package and no CDK timer provider; final Wasm has one global timer and the owned
dispatcher. No consumer source change was required. Copied Jobs inputs still
match the sibling checkout after qualification. Logs, graph, exports, scratch
lock and source hashes retain the evidence; no sibling file was changed.

Canic/IcyDB candidate checks are retained under
`target/qualification/0.26-composition/`. Disposable copies include their incoming
worktree edits: Canic 0.111.0 at HEAD
`54be77f7fb4ffeb4754781b34f1e6b1a10938354` and IcyDB 0.276.0 at HEAD
`996498ad4edb5569964b68e0c0e3f5bb9df7d1c6`. File hashes identify exact inputs;
both real checkouts changed concurrently after capture, so these are snapshot
receipts, not claims about their latest source. No sibling edits were made here.
Both scratch graphs resolve one local candidate and no CDK timer provider.
Rust 1.99 Wasm checks pass for Canic core/control-plane; IcyDB's generated
lifecycle-participant development Wasm builds and exports one global timer plus
the owned dispatcher. Canic's runtime probe refuses direct Cargo Wasm builds as
designed; its required driver was not bypassed.

Both downstream artifact validators reject a fixture carrying the new dispatcher
under their existing rules. Exact update-endpoint exceptions in the disposable
copies pass both focused validator tests. The design lists required dispatcher
exceptions/fixtures, IcyDB ingress probes and Canic's obsolete one-CDK-provider
graph assertion. Minimal validator experiment patches are retained alongside
failure logs. No full consumer adoption or live composition is claimed here.

Closeout work is retained under `target/qualification/0.26-closeout/`.
Canic/IcyDB source-adoption patches include the planned 0.26 requirement,
dispatcher exceptions, negative validator cases, ingress probes and the Canic
zero-old-provider graph assertion. They apply cleanly to their retained bases;
the current dirty sibling checkouts were not modified. Qualification used the
local 0.25.2-versioned candidate, so downstream locks remain pending the actual
0.26 version selection/publication.

IcyDB's supported build pipeline passes seven focused live subjects: three
lifecycle, three startup-timer, and one private-dispatch/upgrade SQL-fixture test.
Its first attempt stopped at missing optimizer admission; the rerun supplies its
existing Binaryen 133 through the supported digest-checking selector. Canic's
graph guard and both consumers' positive/negative validator subjects pass.
Canic's supported driver instead exposes fixture Candid-export resolution errors
for PerfProbeMetric/PerfProbeObservation in runtime_probe and
ApplicationCheckpointBinding/Phase/CheckpointView in managed_lifecycle_probe.
Those independent failures block live Canic qualification and were not bypassed.

Backend review found no production correctness defect. Admission-failure tests
now cover both Watchdog arms and cleanup after successor installation. Cargo
resolver evidence confirmed that unkeyed older packages can coexist despite the
new ownership key; the earlier overly broad documentation claim is corrected.
Final maintained evidence passes 160 native tests, all 21 PocketIC recovery
subjects, strict library Clippy, formatting, provider boundaries and package
contents. A 64-registration cancellation-churn experiment under 300 self-calls
did not saturate the response table: its final 64 callbacks ran on drain, without
retry. Its source and failure log are retained outside maintained tests. This
does not close real 128-context saturation. No version, commit or release ran.

Native macOS, complete Canic composition, coordinated downstream version adoption and the complete
maintainer-operated release gate have not run. Do not call this candidate
release-qualified or extend recovery claims beyond the recorded subjects.
The previous preparation narrative below retains its original source scope.

## Prepared 0.25.2 qualification ownership cleanup

Starting source is released 0.25.1 at
`32ea72e253fdf2adbeda76b320fdc70be885d1ed`. One undated 0.25.2 draft covers
repository/test cleanup; Cargo versions, dependency selections and release
execution remain user-owned. The selected workspace currently uses Testkit 0.48.

The current snapshot selects
[Shared PR #241 — Keep exhaustive qualification at Shared owners](https://github.com/dragginzgame/shared-tooling/pull/241)
at `2e6a3885a1fd8b6e74bfcc61b15148a547d44677`, based on released Shared
0.15.8 (`9add0ecfdb8a6d2556f6d7da3d06bcbca8b3029f`), with 135 payloads.
The contribution's pending 0.16.0 notes do not identify a released version.
The existing format/release fixtures use its per-target admission probe and retain actual
target/effect/routing checks. Shared owns exhaustive flag permutations. Reviewed
refresh changes also require fresh exit receipts, admit evidence roots explicitly,
validate fixture paths before copying and preserve saved metadata identities.

The size probe sends only the empty and sampled instruction intervals; the native
cohort report derives overhead once. All four producer features and the decoder
use that one unpublished wire record. Sampling boundaries and published API stay
unchanged. This follow-up's before-images, export and focused logs belong to
`/tmp/ic-timers-make-wire.ig_e12r0/`.

The approved two-file contribution moves dependency-inventory cases to their
Shared owner. Its exact commit is canonically adopted and the qualified local
reduction is applied. Timers keeps a positive/negative caller smoke check and its
orchestration/provider boundaries. Arrival images and canonical preview/apply
belong to `/tmp/ic-timers-make-wire.ig_e12r0/adoption-turn/`. The preview changes
only the owner fixture; all 135 payloads keep their existing entrypoints.
The actual adopted pin, repository and completion fixtures pass on Bash 5 and
genuine Bash 3.2. ShellCheck, snapshot integrity, changelog and doc checks pass.
Normal Git index/refs and Cargo inputs match this adoption's arrival, including
the incoming lockfile edit. Earlier Rust/PocketIC receipts retain their original
graph scope; no release or additional Rust qualification ran for this adoption.

Native probe/protocol checks pass on Rust 1.91, as do all four optimized Wasm
cohorts; probe Clippy passes on Rust 1.99. The maintained PocketIC cohort subject
passes through Testkit 0.48 and its admitted 16.1.0 server. Its sandboxed attempt
failed to bind localhost before canister creation; the permitted loopback rerun
passes. These are current probe observations, not a comparative Wasm saving.
Affected hook/gate/completion and owner Make/inventory fixtures pass on Bash 5
and genuine Bash 3.2. Snapshot corruption, evidence, metadata, version/index/tag,
committed-release and hosted-preparation callers also pass focused local checks.
Linux execution does not qualify native macOS or hosted upload/download.

Generic Git-hook tree-observation failures and snapshot formatter rollback stay
with Shared's existing `scripts/ci/test-git-hooks.sh` at the exact adopted source.
Timers retains actual Make/compiler-tool integration, both root workspace members,
lockfile preservation, diagnostics and nested-probe partial staging. Metrics owns
summary arithmetic; local tests retain forwarding, role isolation, empty/zero and
wide observations plus memory pairing. Those earlier reductions required no new
upstream helper or test.

Make now checks for an existing commit and directly calls Shared's source checker.
The tag/publication fixture exercises that real target, including failed Git
observations, dirty/staged/untracked inputs and an unborn checkout. The former
forwarding script is removed; its historical receipt links to released source.

Before-images and focused logs belong to `/tmp/ic-timers-owner-tests.fq6_sccc/`.
This batch changes no published runtime behavior or public API.
All five focused native measurement tests pass on Rust 1.91; library/test Clippy
passes on 1.99. Actual Timers hook, tag/publication and completion fixtures pass
on Bash 5 and genuine Bash 3.2, as does Shared's exact-source hook owner suite.
Affected ShellCheck, Rust formatting, changelog preflight and local doc links pass.
Normal Git index/refs and Cargo/toolchain bytes match arrival. No release ran.

Generic logger qualification now stays with Shared's existing exact-source
`test-validation-target-runner.sh`. Timers keeps one real Make-adapter failure,
its saved status, selected raw/combined logs, summary, unique retained attempts
and nested checkout/jobserver integration. Release-impact admission is inlined
at its only production caller; preparation fixtures cover diagnostics, exact
rejection statuses and unchanged metadata in both preflight and preparation.
Release-index and committed-release cleanup reuse Shared's best-effort log replay
while retaining their saved exit status and existing scratch-retention policy.

This follow-up's arrival images and focused receipts belong to
`/tmp/ic-timers-logger-impact.qxt8azql/`. It changes repository tooling only;
the published runtime, dependency graph and Shared snapshot are unchanged.
Affected release, completion and evidence-retention fixtures pass on Bash 5
and genuine Bash 3.2, as do Shared's exact-source logger and log-replay suites.
Affected ShellCheck, snapshot integrity and changelog preflight pass. Git index,
refs and Cargo/toolchain bytes match arrival. No broad or release gate ran.

The next follow-up removes duplicate CI ordering/failure checks from the
repository fixture; the release-gate fixture remains their owner. Version
preparation now exercises one actual reader rejection in both check/prepare
modes, comparing the original failure status and preserving all metadata.

The approved six-file Shared contribution is committed and open in #241 at the
exact source above. It adds reader-owner structural cases and moves only the
exhaustive Make-mode matrix into the producer fixture. IC Metrics uses the
parallel-dispatch helper, which remains exported and passes its actual consumer
fixture. The removed documented export selects a Shared 0.16.0 draft; Timers
keeps 0.25.2 because its published library contract is unchanged.
Owner and Timers/Metrics caller candidates pass on Bash 5 and genuine Bash 3.2;
ShellCheck, doc links and local changelog checks pass. Earlier candidate source
substitutions were confined to disposable copies and did not establish adoption.
The committed source is now canonically adopted: preview changes five existing
payloads and retains all 135. The 0.15.8 base adds automatic cleanup refusal;
local callers already use the standard exit runner without extra injections.
Before-images, the complete contribution diff and initial focused receipts are
at `/tmp/ic-timers-ci-owners.rgd6ipto/`. This adoption's arrival images and
preview/apply logs are under its `adoption-t1hx9nyi/` directory. The normal
Timers staged entries, refs and Cargo/toolchain bytes match this adoption's
arrival; ordinary Git observations refreshed index stat-cache metadata only.
Only the authorized isolated Shared branch was committed and pushed. No merge
or release was performed. The actually adopted metadata, preparation, release-
gate, hook, completion, repository and snapshot fixtures pass on Bash 5 and
genuine Bash 3.2, as does Shared's exact-source retention fixture. Affected
ShellCheck, snapshot integrity, changelog preflight and local doc links pass.
These focused Linux receipts do not qualify macOS or a complete release gate.

## Released 0.25.1 probe and evidence cleanup receipts

Starting source is 0.25.0 at `11e73bff33676e6d829baa502fb943ffa312b988`.
One 0.25.1 draft covered that batch. Cargo package versions and the incoming
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
