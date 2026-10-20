![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-20

## Source and accepted work

Released base: **0.29.5**, `ccebb61`; package versions remain unchanged.
The undated **0.29.6** draft contains the qualification improvements below.
These change repository qualification and native tests, not the published timer
API or production behavior. Release execution remains maintainer-owned.
Preserve the incoming catalog/lock update to Testkit **0.59.0**. The selected
Platform **0.4.1**, Build **0.22.1**, Host **0.31.1**, Wire **0.5.0** and unrelated
resolved versions are unchanged from the incoming worktree. Shared Tooling remains
**0.25.4**, `db555ab6cf4a93af9a183f18d921d3b0ee97c725`, with 138 payloads.

## Qualification improvements

The native driver supervises its build/test worker through Host process-group
ownership: 30-minute elapsed-time budget, two-second TERM grace, five-second reap
budget. This covers cache/build waits, tests and synchronous PocketIC deletion
once the driver starts. Initial Cargo compilation and Testkit admission remain
outside it; OS calls can overrun budgets. Each separately grouped Testkit server
has a five-minute hard TTL and retains its 60-second startup timeout. Host owns
process mechanics; Testkit still owns server admission and lifecycle. No timer
recovery guarantees or required release subjects were weakened.

A bounded exhaustive native test covers 7,776 four-command words across all three
policies and both declaration lifetimes. Commands include ensure, reconcile,
cancel, replacement, delivery and discard. Assertions check stale authority,
independent-timer isolation, bounded handles and final cleanup. Stale-wakeup
accounting can legitimately change on rejected delivery. Native fake evidence
still cannot establish IC rollback, traps or recovery.

Cohorts retain structured costs in `testing/target/cohorts.json`, bound to Build
compiler/Cargo observations, target/profile, cohort features, source/build
fingerprints, exact installed artifact hashes, root lock, server hash and host.
Inputs and lock/server identities are rechecked after measurement. The explicit
`cohorts --baseline PATH [--report PATH]` comparison rejects incompatible
contexts/subjects and prints cost deltas; it never silently rewrites a baseline
or invents a performance budget. Accept reports only after successful driver
exit. Whole-message/unaccounted instruction totals remain unavailable. See
[release qualification](../releasing.md#deployment-validation) for commands.
Host artifact hashing and JSON serialization are existing resolved capabilities
now directly selected by the unpublished host driver; canister/protocol graphs
retain their deployment boundaries.

## Ownership and evidence

Timers owns volatile scheduling, claims, observations and private IC dispatch.
Applications own durable intent and lifecycle/health policy. Platform owns IC
bindings, Wire application codecs, Testkit simulator admission/lifecycle, and
Build/Host artifact and process mechanics. Testkit's native simulator subtree
still includes Candid; published library and probe-canister graphs remain
Candid/full-CDK-free. The protocol retains only Wire/Serde. See
[architecture](../architecture.md#capability-ownership) and [SAFETY.md](../../SAFETY.md).

Focused Linux Rust 1.91 checks cover the generated traces, deadline/nonzero-exit
and baseline rejection fixtures, strict library/probe lint, Wasm library build,
all 27 maintained real-Wasm watchdog subjects and four-policy cohorts with both
calibrations. The comparison CLI reproduces all 54 cost fields with zero deltas under the
same recorded context. Shared declaration/inheritance and native/Wasm graph
checks pass.
The admitted Testkit 0.59 CLI/server is prepared through the existing owner.
Evidence and incoming before-images: `/tmp/ic-timers-qualification.q8nqf412/`.
Native macOS, installed-IC evidence and the complete user-operated release gate
remain separate; none is qualified by these Linux runs.

Earlier adoption evidence remains in
`/tmp/ic-timers-platform-host-shared.kc_mnr7r/`. Historical design and release
notes remain evidence, not authority to restore retired dependencies or paths.
Version bumps, staging, commits, pushes, tags and publication remain user-owned.
