# Current status

Last updated: 2026-08-15

## Purpose

This is the compact handoff for a new session. The repository is a higher-level
wrapper around `ic-cdk-timers`; the released 0.3 bounded runtime and its
IcyDB-shaped watchdog evidence are complete.

## Current foundation

- Workspace package version: `0.4.0`.
- Direct timer provider: exact `ic-cdk-timers` 1.0.0.
- `control` is the private Canic-derived ordinary generation/registration
  state machine for checked request sequences, stale callbacks, immediate
  cancellation, scheduling, and reconciliation. It owns no pending command.
- `registry` is the provider-call-free 64-entry canonical state/effect engine.
  It owns unique identity claims, deterministic ordering, policy-specific
  ordinary and watchdog states, the sole pending ordinary command, nested
  request arbitration, and coherent snapshots.
- `platform` is the private direct provider/system-fact boundary. Its handle is
  linear, all effects are bound to exact handles owned by registry entries,
  and instruction measurement uses IC call-context counter type 1.
- `runtime` is the one volatile canister-local owner. Its synchronous,
  idempotent initialization seam derives the epoch from IC time and canister
  version without exporting lifecycle hooks.
- Public `Once` and `AfterCompletion` registrations own erased async callbacks
  and expose exact-claim ensure, cancel, and consuming unregister operations.
  Consumer futures run without a registry borrow and nested ensure/cancel
  arbitration is defined by the canonical registry. Delegated `TimerContext`
  mutation is checked against the exact running callback token and expires at
  completion; a stored context cannot control a successor generation.
- All three registration capabilities expose claim-scoped
  `has_armed_wakeup()` observation. It reports exact ownership of the canonical
  armed provider wake-up handle, not snapshot-derived or durable scheduling
  authority; watchdog work handles do not count.
- Ordinary registrations also expose authoritative optional schedule
  reconciliation, which may move a deadline earlier or later. The public
  `reconcile_once` helper now complements the existing after-completion and
  watchdog lifecycle helpers. All lifecycle helpers hard-code retained
  declarations; transient remove-on-stop callbacks use direct registration.
- Public `Watchdog` registrations own synchronous callbacks. Their bounded
  scheduler arms a successor from current dispatch time, queues separate
  zero-delay work, and returns before consumer code. Canonical entries own at
  most one successor and one work provider handle.
- `schedule` owns validated positive cadence, typed post-run directives, and
  checked deadlines. After-completion recurrence has one configured cadence.
- `snapshot` contains closed inert identity, policy, state, outcome, split
  counter, instruction, epoch, and canonical snapshot values. The registry is
  their only constructor.
- Basic GitHub CI, one formatting-only pre-commit hook, SemVer release helpers,
  development-tool setup, README, changelog, and architecture docs exist.
- The 0.3 notes are finalized under the dated `0.3.0` heading. Future version
  bumps automatically date a staged named section, while still supporting
  promotion from `Unreleased` when no target was known earlier. The guarded
  publish target requires a clean tagged `HEAD`. Post-release hardening makes
  every future bump run `release-verify`: normal CI, Rust 1.88, nested probe
  lint, the watchdog PocketIC matrix, and all policy cohorts. It updates,
  verifies, and stages both root and nested testing lockfiles.
- Post-0.3 hardening was released as 0.3.1, the Canic adoption corrections as
  0.3.2, and the callback-authority/provider-binding fixes as 0.3.3 on
  2026-08-14.
- Latest release line: `0.4.0`.
- The named 0.4 line starts with the completed repository-only release-policy
  and adoption evidence maintenance, then performs the code-bearing cleanup
  and hard cuts requested for the next compatibility line.
- The current 0.4 worktree makes the crate root the only public import path,
  collapses `TimerLabel` into `TimerIdentity`, removes snapshot-to-command and
  redundant observation projections, prevents default construction of partial
  observations, validates lifecycle declarations by exact claim, shares
  earliest/exact deadline mutation, and consolidates provider-handle ownership
  paths. A focused fault test closes an early-return path that could skip the
  second of two detached watchdog handles after the first restoration failed.
  Provider selection is now lazy when an exact detached handle is already
  available. Unexpected synchronous transition errors after detachment now
  restore all exact handles or retire the claim. Watchdog terminal failures
  share one pending-clearing path with atomic paired generation allocation,
  and illegal missing-cadence recurrence is private rather than an unreachable
  public schedule error. The package version is intentionally unchanged until
  the maintainer-owned release flow.
- Normal CI, all 78 native tests, Rust 1.88 workspace and nested-probe checks,
  warning-denied rustdoc/Clippy, Wasm compilation, offline packaging, provider
  and release checks, dependency-duplicate inspection, and diff validation
  pass for the current 0.4 candidate. PocketIC was not rerun because no
  canonical registry transition, normal provider binding, or watchdog
  protocol changed; native fault injection owns the restoration-path change.
- CI also checks rustdoc, shell syntax, and full-SHA GitHub Actions pins;
  Dependabot covers Cargo and Actions dependencies. A structural gate keeps
  every direct `ic-cdk-timers` reference inside private `platform` code and
  forbids re-exporting the provider boundary.
- `SAFETY.md` is the canonical boundary for implemented guarantees and consumer
  obligations; a recurring code-hygiene checklist and initial
  audit report exist under `docs/audits`.
- The 0.2 observability contract and a local Canic-shaped projection test
  require the canonical snapshot to preserve Canic's timer status, counters,
  scheduling, and performance information without importing Canic-specific
  DTOs. A validated uncommitted Canic worktree now supplies the real adapter
  and focused downstream evidence.
- The 0.3 production-runtime design freezes IcyDB's watchdog
  invariants around a one-shot two-message protocol: a bounded scheduler
  arms the next successor and a separate immediate work callback, then returns
  to commit both so fallible consumer work runs only in the later message.
- The completed 0.3 Patch 1 contract fixes the registry capacity at 64,
  separates async ordinary work from synchronous watchdog work, makes provider
  handles private and linear, closes the policy/state/counter vocabulary, and
  defines comparable raw-Wasm and instruction measurement subjects.
- Patch 2 implements that coherent value model and pure bounded registry.
  Focused tests cover all ordinary directives and watchdog decisions,
  duplicates/capacity, claim generations and unregistration, idempotent
  ensure/cancel, stale callbacks, nesting, checked arithmetic, deterministic
  snapshots, and truthful unacknowledged attempts.
- Rust 1.88.0 successfully compiles the current source and resolved dependency
  graph for native and Wasm. The declared MSRV is now Rust 1.88.0.
- Normal development is pinned to Rust 1.97.1; Rust 1.88.0 remains the
  separately checked minimum supported Rust version.

Patch 3 binds live ordinary execution. Replacement and cancellation clear the
actual provider handle, after-completion successors are scheduled from real
completion time, and live bounded snapshot access projects canonical state.
Actual-arm counters advance only after provider-handle ownership commits.

Patch 4 binds the two-message watchdog. Native tests cover pre-arming,
scheduler/work separation, stale/unacknowledged takeover, terminal clearing,
nested requests, declaration removal, and overdue coalescing. A focused
PocketIC 15 test proves successor survival and later progress after one
explicit work trap, plus external rejection of the provider executor route.

Patch 5 adds scheduled/inactive reconciliation helpers for consumer-owned
volatile registration slots, live normally-completed instruction aggregates,
and an IcyDB-shaped Ready/Recovering/terminal fixture with synchronous
commit-guard re-enablement. The PocketIC probe now also reconstructs after
upgrade from probe-owned durable desired state before its downstream-hook
observation and makes later progress. The observed probe samples were 41,900
scheduler instructions across two callbacks and 19,998 instructions for one
normal work callback; trapped work committed no sample.

Patch 6 completes the PocketIC 15 matrix: explicit trap, actual
40-billion-instruction exhaustion, external executor rejection, upgrade
reconstruction before downstream observation, stop/resume, insufficient
cycles and top-up, terminal and scheduler/work-gap cancellation, duplicate
demand, two-timer trap isolation, 300-second overdue coalescing,
commit-window ensure, and one-attempt-per-generation observations. A full
64-entry inventory is ordered and measured.

Comparable Rust 1.88 cohorts report final `wasm-opt -Oz` raw sizes of 212,248
bytes for baseline, 253,315 for Once, 253,695 for after-completion, and 253,746
for Watchdog. The Watchdog no-op scheduler/work samples are 19,530/19,523
instructions; its observed dispatch charge is 15,364,624 PocketIC cycles over
after-completion. Exact hashes and method are in
`docs/audits/0.3-runtime-evidence-2026-08-13.md`.

The released 0.3 evidence passed formatting, strict Clippy, rustdoc, and 56
native tests on Rust 1.97.1 and Rust 1.88.0, plus native and Wasm compilation
on Rust 1.88.0, offline package verification, six recovery/inventory PocketIC
tests, and the four-cohort comparison.

IcyDB's design review found no correctness blocker. Its feedback is now folded
into the contract: recovery claims explicitly cover consumer work rather than
the scheduler message, and the IcyDB-shaped fixture freezes progress,
retryable-failure, Ready, and durable-terminal result mappings.

The post-release hardening tree passed `release-verify` end to end with 58
native tests, Rust 1.88 checks, every supported probe lint configuration, six
watchdog/recovery PocketIC tests, and the four-cohort comparison. Unexpected
internal callback completion, provider cleanup, and accounting failures are no
longer discarded; callback-only failures trap the message, preserving
watchdog rollback semantics. Fault tests cover completion and cleanup paths,
and the IcyDB-shaped live fixture now executes retryable continuation and
terminal failure through scheduler/work callbacks. All registration
capabilities are `must_use`. The release flow keeps the root and nested testing
lockfiles on the same local package version.

PocketIC release evidence is now pinned to `pocket-ic-server 15.0.0` with
SHA-256
`29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb`;
the gate verifies both before expensive work. When `POCKET_IC_BIN` is unset,
the release flow installs that exact Linux x86_64 artifact into the ignored
`target/tools` cache automatically. Explicit overrides remain strict.

Released 0.3.1 added the maintained Canic adapter contract, exact ordinary
reconciliation, one pending-command authority, automatic pinned PocketIC
provisioning, and release hardening. Its guarded release flow passed the full
release gate before tagging and publishing.

Released 0.3.2 accepts one bounded Canic-only custody collection of
opaque claims so authority-snapshot quiescence can enumerate application
timers without duplicating timer state. It corrects all five dynamic built-ins
to retained `Once`, identifies root canister-pool maintenance as retained
`AfterCompletion`, adds lifecycle deferrals to the inventory, and freezes the
exact legacy metrics projection. Fresh inactive reconciliation now installs
all three policy declarations so fixed owners appear and reserve capacity
before application hooks. No Canic files were changed.

The 0.3.2 guarded release passed normal CI, Rust 1.88, all supported nested
probe lint configurations, the PocketIC watchdog suite, and policy cohorts
before tagging. Its hygiene audit removed blanket dead-code suppression, made
pure fixture helpers test-only, confirmed no duplicate dependency versions or
RustSec findings across 13 locked dependencies, and verified the packaged
source inventory and MIT SPDX metadata.

Released 0.3.3 fixed an audit finding where a consumer could retain
`TimerContext` after a callback and reuse its longer-lived registration claim
generation. The context now carries and validates the exact callback
generation, work role, and running state. Ordinary and watchdog regression
tests prove expired contexts cannot clear or replace an already-scheduled
successor. A second
finding closes the public provider-effect failure path: failed installation or
confirmation now clears owned handles and retires a retained declaration as
`ProviderBindingFailed`, rather than leaving a false scheduled state. Release
tooling also rejects non-increasing explicit versions and non-canonical
leading-zero numeric components before expensive gates or mutation. The
maintained contract's stale Canic schedule-count sentence now correctly uses
committed `wakeups_armed`. These changes do not alter the provider protocol.
The local release gate passed with 65 native tests, strict
Clippy/rustdoc, Wasm compilation, offline packaging, and provider/release
checks; Rust 1.88 and every supported nested-probe lint configuration also
pass.

The 0.3.4 changes remove runner dependence on `rg`, hard-cut lifecycle
reconciliation to retained declarations, and add provider-binding fault
coverage for initial and replacement arms, after-completion, partial watchdog
binding, effect confirmation, and remove-on-stop cleanup. Release truth is now
mechanically finalized and checked across Cargo, the changelog, compact status,
and dedicated release note; README and the Canic contract no longer duplicate
the current release number. Normal CI passes with 70 native tests,
warning-denied Clippy, rustdoc, Wasm compilation, offline packaging, and the
portable release/provider checks. Rust 1.88 and every supported nested-probe
configuration pass. PocketIC was not rerun because this candidate does not
change the scheduler/work protocol; the version bump retains that complete
release gate.

Released 0.3.5 adds claim-scoped armed-wakeup observation for all three
registration capabilities. Focused native tests cover inactive, armed,
cancelled, pre-armed watchdog, expired transient claims, and identity reuse.
No provider binding or scheduler/work transition changed.

Tagged IcyDB 0.226.1 adopted exact `ic-timers` 0.3.4. Its validated post-tag
integration worktree upgrades to exact 0.3.5 and uses the retained watchdog
claim's `has_armed_wakeup()` result for reporting while continuing to ensure
unconditionally from durable recovery demand. Focused downstream
real-canister evidence passes with unchanged instruction samples, byte-identical
Candid, and a 374-byte raw-Wasm increase over its 0.3.4 subject. That downstream
worktree was still uncommitted when inspected.

Released 0.3.6 corrects that downstream record and keeps release-truth
validation structural. Exact Cargo, changelog, release-note, and compact-status
version markers remain enforced; free-form narrative is not interpreted after
the finalizer mutates the version. This is documentation and release tooling
only.

Released 0.3.7 keeps the public facade and module ownership
explicit, removes the unused public `TimerFuture` erasure alias, consolidates
ordinary callback erasure and duration validation, and corrects comments about
declaration lifetime, scheduling mode, and armed provider ownership. It does
not change registry transitions, provider binding, the watchdog message
protocol, snapshot shape, counters, or persisted state. It also stops
duplicating full hosted Rust/MSRV validation on a main push and its same-SHA tag
push: tags now verify exact version identity, main ancestry, and release truth.
A pre-bump compact-status prose advisory warns about likely stale release
wording but always continues, so free-form prose cannot strand a release.
Removing the public alias in a patch was a SemVer mistake because compatible
0.3 requirements may select 0.3.7 automatically. The hard cut remains; future
public removals advance the minor compatibility line.

Released 0.3.8 records the read-only Canic adoption
inspection. On tagged v0.102.1 baseline commit
`86763c5f16478e2e548e2059e5efaa963bf9a966`, an uncommitted Canic worktree
resolves exact `ic-timers` 0.3.6, removes its direct provider, parallel
registry/control/counters and timer-specific performance storage, makes public
timer control fallible, advances runtime introspection to schema version 2,
and derives status plus timer/performance metrics from one shared inventory
scan. Canic's maintained status reports targeted lifecycle, inventory,
protocol, host-adapter, and PocketIC timer evidence passing; this repository
did not rerun those downstream suites or collect a numerical performance
comparison.

After that release, the same Canic worktree advanced its exact dependency from
0.3.6 to 0.3.8 without adapter changes. Its maintained status reports affected
package checks, strict targeted Clippy, inventory/lifecycle guards, three timer
adapter unit tests, and PocketIC cancellation, recurrence, and reconstruction
passing. The current IcyDB post-tag worktree also exact-pins 0.3.8 and retains
its focused real-canister recovery evidence. Both development graphs now align
to one package version; tagged combined qualification remains downstream work.

The repository policy update separates implementation hard cuts from
SemVer: an incompatible public pre-1.0 change requires a minor bump. A release
impact classifier distinguishes crate source/manifest changes from repository
documentation, evidence, external-test, CI, and tooling changes. Version bumps
reject a repository-only subject before expensive validation or mutation; such
work is validated with `repository-check`, committed without a package tag,
and bundled into the next code-bearing release.

## Remaining downstream work

- Land and release Canic's validated exact-0.3.8 adoption worktree.
- Land IcyDB's validated exact-0.3.8 claim-scoped integration worktree.
- Prove a tagged combined Canic, IcyDB, and application canister resolves one
  exact `ic-timers` package ID and inventory any remaining direct provider
  users.
  The provider's 250-call semaphore is canister-wide and is not reserved by
  the 128-handle library bound.
- Optionally collect a numerical Canic before/after metrics-request benchmark;
  the adapter already scans the registry only once per request, so this is not
  an adoption correctness gate.

Tagged IcyDB 0.226.1 at
`cd388cad96383f7c4c56054a8f27de608e9371e3` hard-cuts to exact `ic-timers`
0.3.4 and supplies maintained shared-registry evidence. Its validated post-tag
worktree completes the exact-0.3.8 claim-scoped observation integration but
had not landed when inspected. Canic's validated uncommitted adoption also
uses exact 0.3.8. See `docs/adoption/icydb.md` and `docs/adoption/canic.md`.

## Next action

Review and freeze the 0.4 candidate, then use the maintainer-owned minor
release flow. After publication, ask Canic and IcyDB to requalify one exact
0.4 package. Do not mutate downstream repositories unless the maintainer
explicitly authorizes an exact target.

The maintainer owns release tags and all package-publication actions.
