![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-18

## Source and ownership

HEAD is released 0.27.1 (`194204db30444f0355729deda32f1de739160734`). The
working tree prepares the undated 0.28.0 batch in `CHANGELOG.md`. Metrics 0.14
changes the publicly reexported `MeasurementSummary` identity, so this batch
requires a minor release even though its timer scheduling API is unchanged.
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
Testkit/Candid 0.53 and Host Process 0.27. Platform 0.1.8 is selected;
the incoming Build/Host/Metrics selections are preserved.
Shared 0.24.0 (`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`) now supplies 136
canonical payloads. The Make-only copier is retired; the supplier preparation
fixture stays at its owner because its new closure includes Shared's CI workflow.
Timers' actual CI workflow is preserved. Common Cargo setup selects two sorters;
Candid extraction is not selected. Host/Cargo tools and Testkit remain the only
selected setup; there is no IC executable bundle.

## Published Platform adoption

Cargo registry refresh on 2026-10-18 selected published Platform 0.1.8
(checksum `4a3adb6d9785133587b854b66b37dfdd8f7945555bdc59b6d0be26995187484d`,
VCS `57fcf8d4f55914dedf0a25e70cec2e8affb3fb82`, MSRV 1.88). The root
catalog/lock select this registry archive. All other package identities and
incoming work are preserved. Default features stay disabled: production selects
`executor` and `ic0`, protocol DTOs/native encoding select `candid`. No parser,
Agent, certification, management or ledger feature is selected.

The release adds snapshot/status management DTO exports behind an unselected
feature. All executor, IC0 and Candid module bytes used here match 0.1.7; exact
executor 2.0.0/IC0 1.2.0 identities remain shared with CDK. Production calls stay
on Platform with no direct executor/IC0 declarations or local pin exceptions.
No Rust implementation, decoder policy, callback/trap context, upgrade tuple or
wire shape needs to change for this update. Parent bounded Candid helpers retain
explicit limits; no unbounded convenience function replaces them.

The archive still documents direct probe CDK/Candid for unsupported typed
entrypoint/call/spawn/lifecycle APIs
([Platform #1](https://github.com/dragginzgame/ic-platform/issues/1)); published
Testkit 0.53 still requires its old limits/error identity
([Platform #2](https://github.com/dragginzgame/ic-platform/issues/2)). The actual
limits-import E0308 trial and full 0.1.7 runtime evidence remain retained under
`target/qualification/platform017/`; unchanged API bytes and constraints preserve
that explanation. No duplicate issue or local facade is justified by this release.

Current receipts are in `target/qualification/platform018/`, compared with
`/tmp/ic-timers-platform018-arrival.17psrb19`. Rust 1.91 library Wasm, 160 library
and two protocol tests, eight doctests, Rustdoc, strict library/runtime/driver
and all four cohort lint selections pass. The graph audit confirms only
Platform's identity changed; all-feature Linux/Wasm library normal/build graphs
exclude Candid/full CDK, parser, Agent, management/ledger DTOs and test tooling.
One executor/IC0/CDK identity is resolved. All 24 runtime PocketIC subjects
and the four-artifact cohort subject pass on admitted server 16.1.0, covering
real await, callback traps, cancellation, queue pressure, instruction exhaustion
and upgrades. Eight DTO wire descriptions and both encoded vectors match the
pre-Platform output. This is fresh 0.1.8 Linux simulator evidence.
Grouped formatting, pin/inheritance/direct/provider checks, snapshot integrity,
release metadata and local documentation links pass. No new consumer regression
or previously untracked API gap was found. Existing #1/#2 already cover the
remaining exceptions, so no duplicate issue or repeated unchanged comment was
posted. Prior evidence retains its own source/graph scope.

Shared's direct policy forbids all member IC0/executor declarations, including
aliases and normal/build/dev/target tables, with only the documented consumer
exceptions. Unused catalog entries, future unenumerated names, transitive feature
reachability and deployed-artifact admission remain separate obligations; see
[architecture](../architecture.md#candid-and-cdk-dependency-boundary) and
[adoption boundaries](../shared-tooling.md#deployment-dependency-exclusions).
Native macOS, live IC, hosted and complete release gates remain independent.

## Pending implementation and preserved evidence

The incoming registry completion change chooses checked schedule, directive
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
baseline feature. The unused-dependency scan flags only size-probe `candid`:
selected CDK macros emit direct `::candid` references, so that edge is retained
and documented beside its declaration. No full gate, live PocketIC, native macOS
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

The registry still ends at Testkit 0.53.0. An isolated consumer of captured pending
Testkit source removes both old-decoder declarations, uses Platform's limits
identity and forbids the retired dependency without changing the five ceilings.
Actual Rust 1.91 strict all-target driver Clippy and all six producer call/decoder
subjects pass. [Platform #2](https://github.com/dragginzgame/ic-platform/issues/2#issuecomment-6249021600)
records readiness and the publication prerequisite. This is unpublished-source
evidence only: the normal catalog, lockfile and decoder exception remain intact.
No compatibility wrapper or alternate transport is introduced.

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
