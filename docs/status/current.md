![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-19

## Dependency refresh notes — pending 0.29.2

The maintainer's incoming catalog and lock select Build 0.21.0 with Host Process
0.29.0, Platform/macros 0.1.18 and Wire/derive 0.4.6. Build's published manifest
requires Host Process `^0.29`, matching the native harness declaration. Testkit
still resolves its separate Host Process 0.27.6; no type crosses that boundary
in the harness's Build spawner call. These incoming files are preserved.

The new undated 0.29.2 changelog records this batch. No timer source or public
contract changes; the tooling refresh is repository-only and normally bundles
with a code-bearing release. The maintainer explicitly selected this patch.
Changelog admission and whitespace checks pass. No build or runtime qualification
of the refreshed graph is claimed here; earlier receipts below qualify their
named dependency selections. Cargo versions remain 0.29.1. Version mutation,
the complete release gate, staging, commits and push remain maintainer-owned.

## Harness Host alignment — pending 0.29.1

The reported `testing-check` E0271 is fixed by selecting Host Process `^0.27.3`,
matching published Build 0.19's exposed spawner type. The single resolved Host
Process 0.27.6 now serves Build, Testkit and the native harness. No Rust adapter,
conversion or parallel process owner is added. Unused Host Process/FS/Artifacts
0.28.1 copies leave the lock; every other incoming package identity is preserved,
including Platform/macros 0.1.16 and Wire/derive 0.4.5. Cargo versions stay 0.29.0.

The actual Rust 1.91 `make testing-check` passes for the complete probe/driver/
protocol selection and all four size features. Locked metadata, normal Testkit
server admission and the reverse Host graph also pass. The normal root
`make pocketic-watchdog` passes all 27 runtime subjects, and `make pocketic-cohorts`
passes the four-artifact policy subject on this repaired graph. All three
protocol tests pass. This closes the former isolated-only Wire qualification
gap with Platform 0.1.16 and Wire 0.4.5; native macOS, live IC and the full
maintainer-operated release gate remain separate. Current receipts are in
`target/qualification/host-alignment/`; incoming dirty bytes are captured under
`/tmp/ic-timers-host-alignment.gxh5z_6s/`. Full release execution remains user-owned.

## Probe Wire cut — pending 0.29.1

This task starts from clean released 0.29.0 (`8f4b72f`). All authored probe RPCs,
self-call gate replies and the four-field fixture checkpoint now use published
Wire 0.4.4's native Serde codec. Its runtime/derive archives match registry
checksums and are un-yanked; normal registry resolution confirms the selection.
The protocol package depends only on Wire and Serde. The harness no longer
selects Platform Candid, and there are no local Candid records, codec calls,
checkpoint helpers or generated Candid query/update defaults. Platform's hidden
custom-codec hooks preserve CDK contexts and protected async calls. The production
library, public API and authenticated eight-byte dispatch are unchanged.

One protocol roster owns method identities; the cohort's distinct start contract
is `cohort_start`. Requests are checked before copying IC arguments and decoded
with explicit Wire byte/allocation/node/depth budgets. Replies preserve the prior
64 KiB byte ceiling. Wire errors cannot enter a handler, and reply encoding failure
traps. The checkpoint retains the same four values in a bounded Wire frame with
a checked length prefix. Both upgrade participants must be rebuilt together;
there is no Candid fallback or cross-release probe checkpoint migration. These
are qualification fixtures, not application/database storage. Details belong in
[architecture](../architecture.md#candid-and-cdk-dependency-boundary).

The undated 0.29.1 notes classify this as repository-only qualification work;
Cargo package versions remain 0.29.0. Wire/Postcard are test/tooling-only.
The new Serde entry is inherited only by probes, but Serde already appears in the
production graph through Platform's Principal, so its catalog role is production. The library's standalone Wasm graph still excludes Candid, full CDK and
Wire; the protocol's standalone graph excludes Platform/CDK/Candid. Qualification
graphs still contain Candid 0.10.38 via Platform's `cdk` capability and
PocketIC/Testkit. Source-owner checks reject local Candid codec references and
query/update attributes that omit admission/custom-codec hooks; they do not
claim to remove or forbid those intentional transitive packages.

Initial Wire qualification was interrupted when the maintainer independently
selected Build 0.19 and Host Process 0.28. That mixed spawner boundary failed
before tests; the alignment above resolves this consumer failure. Its original
report remains on
[Build #9 — Qualify consumers against the exposed Host boundary](https://github.com/dragginzgame/ic-build/issues/9#issuecomment-6263501759).

The following Wire qualification receipts precede the current Host alignment:
Linux/Rust 1.91 probe/protocol tests, native probe/protocol lint, library native
compilation, library/runtime-probe Wasm lint and source/dependency checks passed.

All 27 real-Wasm runtime subjects pass in an explicitly isolated source copy,
`/tmp/ic-timers-wire-qualification.kgk8ej5r`, using the arrival Build 0.18 / Host
0.27 pair with Wire 0.4.4 and admitted PocketIC 16.1.0. This tests the migrated
RPCs, strict rejection before effects, real gate awaits/queue pressure, traps,
checkpoint upgrades and reconstruction. The four-artifact policy-cohort subject
also passes in that same isolated source/graph. Comparable loaded probe sizes
fall from 242,853 to 78,809 bytes (baseline), 292,712 to 136,654 (Once), 292,903
to 136,600 (after-completion) and 293,978 to 137,427 (Watchdog). These are full
stripped qualification artifacts on Rust 1.91's unchanged `timer-probe` profile,
not the published library's standalone size or complete-message cost. Watchdog
scheduler instructions stay at 12,539; its work envelope changes from 8,851 to
8,878. Full operation/cycle rows are in `isolated-cohorts.log`, compared with
`cohorts-before.log`; no timer execution speed-up is claimed. This is auxiliary implementation evidence,
not qualification of the maintained 0.19/0.28 graph. No native macOS, live IC,
hosted acceptance, sibling mutation or complete release gate is claimed.

Source before-images are `/tmp/ic-timers-wire-arrival.rn42k31j/before/`; focused
receipts live in `target/qualification/wire-probes/`. The pre-change cohort
baseline is source/graph-bound. Older sections below retain their historical
receipts and do not describe current pending work or dependency selections.

## Metrics event delegation — delivered 0.29.0

Current source base: `8d69266` (`fix`), following released 0.28.2. The only
incoming edit was Cargo.lock, selecting Platform/macros 0.1.15 alongside Metrics
0.14.4, Testkit 0.54.1, Build 0.18.0 and Host 0.27.6. Its bytes and all manifests
are preserved. Cargo package versions remain 0.28.2; the new undated 0.29.0
changelog records the breaking getter contract. Version mutation and delivery
remain maintainer-owned.

Timers now stores its fourteen diagnostic events in published Metrics
`MeasurementCounter` values and delegates increment/merge/checked arithmetic.
All fifteen getters return the shared counter, reexported at the public facade;
`work_completed()` merges four outcome classifications with overflow provenance.
There are no legacy integer getters or parallel arithmetic. Timers retains event
meaning, recording boundaries and full registration/reset identity. Functional
failure streaks, scheduling, callback authority and memory-page observations are
unchanged. The unpublished protocol carries total/overflow wire records only;
it retains its Platform-Candid-only dependency and owns no arithmetic.

Consumers must preserve the `u128` total and actual-overflow flag, or explicitly
reject unrepresentable narrow projections. Compare full registration identities
and source continuity before checked deltas. Canic's existing counter owner has
[the adoption details](https://github.com/dragginzgame/canic/issues/535#issuecomment-6262598164),
coordinated with its existing prepared metrics PR. IcyDB's two actual fixture
projections and their wire/native callers are covered by
[#386 — Adopt Timers 0.29 tracked event counters](https://github.com/dragginzgame/icydb/issues/386).
Sibling files remain untouched. Incompatible timer minor lines cannot coexist
through the global-timer links key: coordinate Canic/IcyDB/application selection
when this version is published, rather than adding a parallel provider. Jobs'
inspected maintained source has no diagnostic counter accessor use.

Focused Linux qualification with actual Rust 1.91 passes 161 library tests, both
protocol tests, the runtime-probe unit, all nine doctests, strict library and
probe/driver/protocol Clippy (including four size features), library/runtime-probe
Wasm Clippy and warning-denied library Rustdoc. All 27 real-Wasm runtime subjects
and the four-artifact policy-cohort subject pass on Testkit-admitted PocketIC
16.1.0. Provider, direct dependency boundaries, pin/inheritance, formatting and
0.29 changelog checks pass. The unchanged selected lock is used throughout.
Receipts are retained under `target/qualification/counter-delegation/`, with
before-images under `/tmp/ic-timers-counter-delegation.iclso78v/before/`.
One cohort acquisition correctly rejected an edit during input capture; the
subsequent frozen-source run passes. No native macOS, live IC, sibling builds,
hosted checks or complete maintainer release gate is claimed.

Delegation has a measured cost, not a performance-saving claim. Native
`size_of::<TimerCounters>()` rises from 112 to 448 bytes per registration
(+336 bytes of counter value storage; excludes registry/allocator layout).
Comparable before/after cohorts keep the same lock, Rust 1.91 profile and server:

| Cohort | Loaded Wasm bytes before → after | Scheduler / work envelope instruction delta |
| --- | --- | --- |
| Baseline | 240,937 → 242,853 | 0 / 0 |
| Once | 290,389 → 292,712 | 0 / +17 |
| After-completion | 289,552 → 292,903 | 0 / +29 |
| Watchdog | 290,788 → 293,978 | +36 / +17 |

These artifacts include the necessary private wire projection changes. Snapshot
and inventory costs also increase; full rows are in `cohorts-before.log` and
`cohorts-after.log`. Envelope intervals and dispatch cycle observations are not
complete-message accounting, arithmetic-only costs or installed-canister proof.
Older receipts below qualify only their named historical sources/graphs.

## Runtime coverage expansion — delivered 0.28.2

This task starts at released 0.28.1 (`c9cc95c259cb46637aacccf6f1fbed6a8280aa39`).
The incoming manifest selects Build 0.18 and the incoming lock selects Build
0.18.0, Platform 0.1.13, Testkit 0.54.1, Host 0.27.6 and Metrics 0.14.4.
Those manifest/lock bytes are preserved; package versions remain unchanged.

Two real-Wasm subjects extend the maintained runtime suite. A batch registers
64 simultaneously due timers under real self-call queue pressure, cancels and
replaces pending work, then reaches 64 callbacks awaiting real self-calls at
once. Per-callback counts establish complete progress without replay, continued
dispatch and retained-claim reuse. A separate trace checks distinct deadlines,
callback-created timers, incremental and overdue delivery, and both one-shot and
periodic self-cancellation. These fixtures add no production backend hooks or
published library changes. They do not demonstrate exhaustion of all 128 backend
transport slots or add Wasm64 support.

On Linux, actual Rust 1.91 native all-target and Wasm probe Clippy pass with
warnings denied; both protocol tests and all 27 runtime PocketIC subjects pass.
The normal lock-selected Testkit setup prepares its CLI and admits PocketIC
16.1.0 before execution. Runtime output is retained in
`target/qualification/coverage-parity/runtime.log`. No policy-cohort rerun,
native macOS, live IC or full maintainer-operated release gate is claimed.
Earlier sections retain receipts bound to their named source/graph inputs;
their historical pending/delivery statements do not qualify the current batch.

## Source and ownership

The source base is released 0.29.0 (`8f4b72f`). The working tree prepares the
undated 0.29.1 probe codec cut above. Cargo versions, staging, commits, pushes
and releases remain user-owned.

Timers owns volatile registration, scheduling, observation and private one-shot
IC dispatch. Consumers retain durable intent, lifecycle order and operational
health policy. [Architecture](../architecture.md#capability-ownership) maps
implementation, dependencies and inspected consumers; [SAFETY.md](../../SAFETY.md)
owns guarantees. Runtime and product qualification must not be inferred from a
historical version or a native platform substitute.

One root workspace/catalog and lockfile select all five maintained packages.
The root manifest uses the canonical production and test/tooling role groups;
matching grouped sorting is active in Make, hooks and all release metadata
checks. The current catalog selects Metrics 0.14, Build 0.19, Testkit 0.54,
Host Process 0.27.3, Platform 0.1 and qualification-only Wire 0.4.4. The lock
selects Host Process 0.27.6, Platform 0.1.16, Wire 0.4.5, Metrics 0.14.4 and
Testkit 0.54.1. The harness's spawner family now matches Build 0.19.
Shared 0.24.0 (`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`) now supplies 132
canonical payloads. Generic installer/release-runner matrices now stay at Shared;
their three entrypoint roots and unused cache-key companion are canonically retired.
Timers retains actual tool admission, release integration and all runtime gates.
The Make-only copier is retired; the supplier preparation
fixture stays at its owner because its new closure includes Shared's CI workflow.
Timers' actual CI workflow is preserved. Common Cargo setup selects two sorters;
Candid extraction is not selected. Host/Cargo tools and Testkit remain the only
selected setup; there is no IC executable bundle.

## Earlier published Platform adoption — 0.1.12

The selected published facade is **0.1.12**, SHA-256
`e0ab384614a5d8fe251bc2c27c508263d4001d776edf31382b5b985a1a4ced2d`,
VCS `338b14baf97690abdfc0e386d6d5261b309a3ac8`, MSRV 1.88. Its coordinated
macro helper 0.1.12 matches SHA-256
`9b5de9771169e34ca8da9bbb04c9c6196b1d2de8d2d0d21bc79d5921984ce6be`.
Both downloaded archives match the registry. Generator source is unchanged from
0.1.11. The lock updates only these two packages and removes the runtime probe's
direct CDK edge; all other incoming selections are preserved.

Both canister probes use Platform's `cdk` capability for entrypoints, calls,
traps and protected spawning. The runtime probe's final three direct CDK calls
now use its exact upstream `storage::{stable_save, stable_restore}` and
`api::cycles_burn` exports. The existing four-field upgrade tuple, checkpoint
bytes, decoder/error behavior, trap messages and low-cycle semantics are unchanged.
The direct dependency, local exact-pin exception and dependency-policy allowance
are removed. The published delivery completes
[Platform #19](https://github.com/dragginzgame/ic-platform/issues/19).
No local wrapper or storage migration is
introduced; existing counter/page observations retain their Platform IC0 paths.

Default features stay disabled. The published library selects only `executor`
and `ic0`; DTO/native decoding selects `candid`, and only the canister probes
select `cdk`. Parser, Agent, crypto/signature, certification, management and ledger
features are unselected. Candid/CDK/macros intentionally remain transitive in the
qualification graph; production library graph admission is a separate check.
Shared's direct checker forbids Candid/CDK-family, Platform-macro, retired decoder,
IC0 and executor declarations in every member, including aliases and
normal/build/dev/target declarations. Transitive feature/artifact exclusion
remains a separate deployment-owner obligation; see
[deployment exclusions](../shared-tooling.md#deployment-dependency-exclusions).

Current receipts belong to `target/qualification/platform0112/`, compared with
`/tmp/ic-timers-platform0112-arrival.ct2vmuet`. Prior 0.1.10 and 0.1.11
qualification remains source-bound in `target/qualification/platform0110/` and
`target/qualification/platform0111/`; it does not by itself qualify this selection.

Rust 1.91 strict probe/driver/protocol and four-feature cohort lint, native
all-target and Wasm library checks, 160 library tests and both protocol tests
pass. All 25 runtime PocketIC subjects and the four-artifact policy-cohort subject
pass on Testkit-admitted server 16.1.0, including stable upgrade/reconstruction,
low-cycle refusal/retry, callback/trap/cancellation and typed Candid rejection.
The original minimal helper-import consumer now compiles. Native/Wasm standalone
library graphs exclude Candid, full CDK, generators and host tooling; the full
qualification graph retains one CDK 0.20.3, executor 2.0.0 and IC0 1.2.0 identity.
Direct-boundary, pin/inheritance, provider, formatting, changelog and local-doc
link checks pass. These are Linux simulator/MSRV results; native macOS, live IC,
hosted acceptance and the complete maintainer-operated release gate remain
separate. Platform's API-gap issue is closed with published/consumer evidence;
Timers' adoption remains local pending maintainer delivery.

## Development compiler

Development and explicit hosted selectors now use Rust 1.99.1; the library and
probe MSRV remains Rust 1.91.0. Local Linux checks with the actual 1.99.1 compiler
pass formatting, strict library/probe/driver/protocol and four-feature cohort
Clippy, library/protocol/runtime-probe tests, doctests, Rustdoc and library Wasm
compilation. Explicit Rust 1.91 native/Wasm library checks also pass. Dependency
and package versions are unchanged. Compiler preparation/workflow routing tests
pass; hosted native macOS and actual-canister acceptance belongs to the matching
PR checks, not these local compile/unit results.

## Runtime implementation and preserved evidence

The 0.28.0 registry completion change chooses checked schedule, directive
observation and cancellation together. The runtime probe shares Watchdog
installation while preserving independent callbacks, counters and durable tuple.
The mixed-policy probe and three subjects are now local to Timers; probe startup,
Candid calls and Wasm builds adopt the already-selected provider interfaces.
These edits were present before the ownership cleanup and are preserved.

Their recorded evidence includes 160 native library subjects, strict Clippy,
Rust 1.91 library Wasm/Rustdoc checks, 24 runtime PocketIC subjects and the policy
cohort subject. Each receipt retains its tested source/graph scope; this handoff
is not a fresh qualification of all accumulated changes. See
`target/qualification/completion-compact-cleanup/`,
`target/qualification/issues-adoption/` and
`target/qualification/fleet-timer-convergence/` for existing receipts.

The 2026-10-18 cleanup routes four remaining local Make/hook/setup fixtures through
Shared's declared dependency closure, corrects the packaged README's old version
and pending-backend advice, and removes completed migration checklists from active
guidance. The earlier handoff is preserved [verbatim in the archive](archive-2026-10-18.md).
No timer API, library behavior, dependency selection or snapshot payload changes.
Task before-images are `/tmp/ic-timers-sediment-arrival.ia4ut44f`. Four affected
fixture suites, fixture completion/retention, provider boundaries, source-following
ShellCheck, snapshot integrity, locked offline metadata and local documentation
links pass on Linux. Both non-ignored README Rust examples compile against
published exact 0.27.1 on Rust 1.91, and the size probe compiles to Wasm with its
baseline feature. That earlier selection retained size-probe `candid` because
upstream CDK macros generated direct references. The subsequent Platform
migration replaced those macros and removed that edge. No full gate, live PocketIC, native macOS
or sibling build was run for this tooling/documentation cleanup.

## Ownership convergence closeout

Fresh Linux qualification for the owning mixed-policy proof passes all 24 runtime
PocketIC subjects on Testkit-admitted server 16.1.0. The first sandbox attempt
could not bind loopback; the permitted rerun passes. This is simulator evidence,
not native macOS, live IC or complete release acceptance. Receipts and captured
source identities are in `target/qualification/convergence-closeout/`.

Sibling repositories remain read-only by the maintainer's explicit decision.
The older IcyDB removal no longer admitted after intervening changes; a refreshed
ten-file candidate now passes patch admission against current source and locked
workspace metadata. [IcyDB #379](https://github.com/dragginzgame/icydb/issues/379)
owns retirement after the Timers proof is delivered. Actual artifact tests remain
blocked in both the candidate and unchanged before-image by the same selected
Build 0.17.2 constructor/removed-environment-method errors; the reproducer is on
[the existing tooling issue](https://github.com/dragginzgame/icydb/issues/307#issuecomment-6249021145).
The tool packages require Rust 1.96; these failing attempts use Rust 1.99.0.
No database recovery or lifecycle proof is replaced by the primitive suite.

Published Testkit 0.54.0 now replaces the earlier unpublished-source trial. Its
registry archive matches SHA-256
`787c2ea232d8dd36586d7bd2fb72d78d098a2b892cedd1a319450b2ecea7347d`, identifies
source `ff88087d6757121ac92e0dec4705ecaccfd8ebed` and retains Rust 1.91 support.
The published calls and typed errors consume Platform's identities directly.
The incoming root catalog had already selected 0.54 and removed the old decoder;
the completed cut removes its remaining member declaration, switches the actual
harness import and forbids the retired dependency in every workspace member.
All five ceilings are unchanged. The lock drops Testkit 0.53, the old decoder and
the now-unused Metrics 0.13 copy while preserving other selected versions.
[Platform #2 — Coordinate published Testkit limits identity](https://github.com/dragginzgame/ic-platform/issues/2)
owns the upstream coordination. No compatibility wrapper or alternate transport
is introduced. Current receipts and captured incoming bytes are in
`target/qualification/testkit054/`.

The actual Rust 1.91 all-target driver/protocol Clippy, both protocol tests,
all 24 runtime PocketIC subjects and the four-artifact cohort subject pass
against the published release. Normal
Testkit 0.54 CLI preparation and offline server admission pass for PocketIC
16.1.0. Complete metadata/tree inspection contains no retired decoder package
and one Metrics 0.14.1 identity. The library's standalone Wasm normal/build graph
remains free of Candid/full CDK. Direct dependency and pin/inheritance checks pass.
These are Linux simulator/MSRV results; native macOS, live IC and complete
release qualification remain separate.

## Coordination and limits

The 0.24 adoption uses committed supplier bytes, not the older prepared grouped
patch. Snapshot integrity, grouped formatting and its idempotent disposable
write/negative check, hook isolation, current/committed-source metadata checks,
release routing/tag/publication fixtures, Rust/Testkit setup, evidence fixtures,
provider boundaries and dependency pins pass on Linux. Common Rust setup was
explicitly refreshed through the shared installer after the old three-tool
receipt failed two-tool admission. Task receipts are in
`target/qualification/shared024/`, compared with
`/tmp/ic-timers-shared024-arrival.0djwogzx`. Full release/native macOS and hosted
acceptance remain unrun for these inputs; normal staging/commits remain user-owned.

The library's locked all-feature normal/build graphs exclude all current
qualification-only catalog dependencies for Wasm and Linux. Rust 1.91 Wasm
compilation passes. The existing checks enforce named direct ownership but do not automatically
enforce transitive role exclusions or bind them to deployed artifact bytes. Final-canister admission
belongs to the consuming build/deployment owner; this graph inspection is not that
gate. [Adoption boundaries](../shared-tooling.md#deployment-dependency-exclusions)
record the scope and remaining enforcement gap.

Read-only sibling inspection found one Timers 0.27.1 package and no CDK timer
provider in each Canic, IcyDB and Jobs lockfile. These observations do not qualify
composed Wasm or deployments. Canic's current worktree owns health projection;
its base already owns a recovery Watchdog. Older adoption tables describe frozen
subjects and do not assign current ownership.

Previously prepared Canic/IcyDB patches remain at
`target/qualification/fleet-timer-convergence/`, based on their own captured
arrivals. Normal sibling worktrees were not changed. IcyDB still contains its
standalone timer-primitive probe; retire it only with the corresponding Timers
proof delivered. The earlier Canic candidate's complete check recorded 17 Memory
integration errors; this task did not rerun that sibling check.

Shared 0.24 supplies selected-bundle compact evidence verification; Timers now
uses it directly. The older host-only adapter candidate is superseded.
Timer-specific memory observations remain local: no canonical page-value API is
exported by the inspected Metrics crate, so the old extraction proposal does not
supply an adoption directive.

The last recorded delivered 0.27.1 hosted matrix passed Linux/MSRV and both macOS
hosts. Pending consumer/native-host acceptance for Shared adoption remains
separate ([Timers #67](https://github.com/dragginzgame/ic-timers/issues/67)).
Focused Linux checks do not establish native macOS or complete release acceptance.
The user runs the complete release gate; this cleanup does not activate it.

Older proposals, failed candidates, tool versions and source-bound receipts are
historical in the [archive](archive-2026-10-18.md),
[Shared evidence](../evidence/shared-tooling-history.md) and
[release evidence](../evidence/release-history.md). Review a candidate against
current providers before use; an old prepared patch is not an adoption directive.
