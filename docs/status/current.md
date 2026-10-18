![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-18

## Source and ownership

The source base is released 0.28.1 (`c9cc95c259cb46637aacccf6f1fbed6a8280aa39`).
The working tree prepares the undated 0.28.2 batch in `CHANGELOG.md`. The released
0.28.0 batch includes Metrics 0.14's public `MeasurementSummary` identity change.
Cargo versions, staging, commits, pushes and releases remain user-owned.

Timers owns volatile registration, scheduling, observation and private one-shot
IC dispatch. Consumers retain durable intent, lifecycle order and operational
health policy. [Architecture](../architecture.md#capability-ownership) maps
implementation, dependencies and inspected consumers; [SAFETY.md](../../SAFETY.md)
owns guarantees. Runtime and product qualification must not be inferred from a
historical version or a native platform substitute.

One root workspace/catalog and lockfile select all five maintained packages.
The root manifest uses the canonical production and test/tooling role groups;
matching grouped sorting is active in Make, hooks and all release metadata
checks. The current catalog still selects Metrics 0.14, Build 0.17,
Testkit 0.54 and Host Process 0.27. Platform 0.1.12 is selected;
the incoming Build/Host/Metrics selections are preserved.
Shared 0.24.0 (`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`) now supplies 132
canonical payloads. Generic installer/release-runner matrices now stay at Shared;
their three entrypoint roots and unused cache-key companion are canonically retired.
Timers retains actual tool admission, release integration and all runtime gates.
The Make-only copier is retired; the supplier preparation
fixture stays at its owner because its new closure includes Shared's CI workflow.
Timers' actual CI workflow is preserved. Common Cargo setup selects two sorters;
Candid extraction is not selected. Host/Cargo tools and Testkit remain the only
selected setup; there is no IC executable bundle.

## Published Platform adoption

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
