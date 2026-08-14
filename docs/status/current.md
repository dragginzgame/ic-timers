# Current status

Last updated: 2026-08-14

## Purpose

This is the compact handoff for a new session. The repository is a higher-level
wrapper around `ic-cdk-timers`; the released 0.3 bounded runtime and its
IcyDB-shaped watchdog evidence are complete.

## Current foundation

- Workspace package version: `0.3.1`, tagged, pushed, and published.
- Direct timer provider: exact `ic-cdk-timers` 1.0.0.
- `control` is the private Canic-derived ordinary generation/registration
  state machine for checked request sequences, stale callbacks, immediate
  cancellation, scheduling, and reconciliation. It owns no pending command.
- `registry` is the pure 64-entry canonical state/effect engine. It owns
  unique identity claims, deterministic ordering, policy-specific ordinary and
  watchdog states, the sole pending ordinary command, nested request
  arbitration, and coherent snapshots.
- `platform` is the private direct provider/system-fact boundary. Its handle is
  linear, all effects are bound to exact handles owned by registry entries,
  and instruction measurement uses IC call-context counter type 1.
- `runtime` is the one volatile canister-local owner. Its synchronous,
  idempotent initialization seam derives the epoch from IC time and canister
  version without exporting lifecycle hooks.
- Public `Once` and `AfterCompletion` registrations own erased async callbacks
  and expose exact-claim ensure, cancel, and consuming unregister operations.
  Consumer futures run without a registry borrow and nested ensure/cancel
  arbitration is defined by the pure registry.
- Ordinary registrations also expose authoritative optional schedule
  reconciliation, which may move a deadline earlier or later. The public
  `reconcile_once` helper now complements the existing after-completion and
  watchdog lifecycle helpers.
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
- Post-0.3 hardening was released as 0.3.1 on 2026-08-14. The next corrections
  are staged under the undated `0.3.2` changelog heading and the dedicated
  `docs/changelog/0.3.2.md` release-line note; the package remains 0.3.1 until
  the maintainer requests the patch bump.
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
  DTOs. A real downstream Canic adapter test remains required.
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
capabilities are `must_use`. The nested testing lockfile resolves the local
package as 0.3.1.

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

The open 0.3.2 correction accepts one bounded Canic-only custody collection of
opaque claims so authority-snapshot quiescence can enumerate application
timers without duplicating timer state. It corrects all five dynamic built-ins
to retained `Once`, identifies root canister-pool maintenance as retained
`AfterCompletion`, adds lifecycle deferrals to the inventory, and freezes the
exact legacy metrics projection. Fresh inactive reconciliation now installs
all three policy declarations so fixed owners appear and reserve capacity
before application hooks. No Canic files were changed.

The open 0.3.2 tree passes the normal CI gate with 62 native tests, strict
Clippy, rustdoc, Wasm compilation, packaging, provider-boundary checks, and
release-script checks. Rust 1.88 workspace compilation and all supported
nested-probe lint configurations pass. PocketIC and cohort evidence remains a
mandatory automatic step in the maintainer-owned patch release flow; it was
not rerun for this provider-neutral reconciliation and documentation patch.
The follow-up hygiene audit removed blanket dead-code suppression, made pure
fixture helpers test-only, confirmed no duplicate dependency versions or
RustSec findings across 13 locked dependencies, and verified the packaged
source inventory and MIT SPDX metadata.

## Remaining downstream work

- Canic's real semantic snapshot/metrics projection test and replacement of
  its parallel timer instrumentation.
- IcyDB's generated watchdog adapter and maintained startup/schema/commit
  integration suites.
- Dependency-unification checks proving IcyDB, Canic, and applications resolve
  one `ic-timers` package ID.
- A final-canister inventory that removes production direct `ic-cdk-timers`
  users or names fixture-only exceptions; the provider's 250-call semaphore is
  canister-wide and is not reserved by the 128-handle library bound.
- The real Canic adapter hard cut described in `docs/adoption/canic.md`,
  including its fallible public facade, removal of the parallel provider/state
  machine, owner-local lifecycle composition, and focused parity tests.

No downstream adoption has occurred. Version 0.3.1 is published.

## Next action

Validate the staged 0.3.2 inactive-inventory correction and corrected Canic
contract, then use the maintainer-owned patch release flow when approved. Do
not mutate downstream repositories unless the maintainer explicitly
authorizes an exact target.

The maintainer owns release tags and all package-publication actions.
