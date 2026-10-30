![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-30

## Source and accepted work

Local main is released **0.32.3**, `6ea2058d34a9162c98b0719d1db8b47605d76def`.
The accepted batch is prepared locally under one undated **0.32.4** changelog:
exact-subject PocketIC selection, retained discovery/runtime evidence, Watchdog
freeze-gap coverage and ordinary async frozen-await coverage. Package versions
remain 0.32.3. Production timer implementation and public contracts are unchanged;
this tooling/test/docs batch normally belongs with the next code-bearing release.
No staging, commits, pushes, releases or sibling source edits ran.

The incoming Cargo.lock selects Build/Host/Testkit **0.70.4**, versus main's
0.70.3. It remains byte-for-byte unchanged. Current focused evidence uses that
graph, Platform/macros **0.8.3**, Wire/derive **0.7.23** and Metrics **0.17.6**;
it is not a complete supplier-adoption or release qualification. Shared **0.27.3**,
revision `7ee5c83744aa822ecd070703dbf0a7d95076d5f6`, is unchanged.

## Driver and evidence

The existing driver accepts `focused tests::fully_qualified_subject [...]`, using
exact matching for discovery and execution. Empty, duplicate, missing, ignored
and non-runtime selections refuse before canister builds. Focused selection
retains the existing Build custody, source checks, Testkit admission and deadlines.
The full Watchdog recovery roster now requires all four freeze subjects; focused
success does not replace the full recovery/cohort release gates.

Each private invocation directory survives success/failure. Bounded raw discovery
and ignored-listing stdout/stderr plus status/truncation metadata remain there;
producer failures print the stderr path. Saving errors never replace an existing
typed producer failure. Successful runtime runs publish `runtime.json` after tests
and Build/context verification, reusing existing build identities. Receipts identify
actual subjects and distinguish focused from full Watchdog selection. They are
observations, not authority to skip gates. No parallel runner or cohort shape was
introduced. See [the release guide](../releasing.md).

## Freeze coverage and limits

All four cases share one real positive-balance five-minute freeze operation and
typed out-of-cycles update rejection. The two Watchdog cases cover armed and
committed scheduler/work windows: funding alone restores delivery and cadence
under the same registration. Queued work and an overdue successor may each
complete once; missed intervals are not replayed.

The new `Once`/`AfterCompletion` cases observe one callback awaiting real self-calls.
A response executes while frozen, but the next call has no liquid cycles. The
probe maps that error to `InvariantFailure`; runtime ends the failed attempt,
expires its callback context and leaves the retained registration inactive.
This is a normal failed completion, not abandonment: one start, one completion,
zero successful application completions and zero unacknowledged attempts.
Funding and more time/rounds cannot replay it. Explicit scheduling starts fresh
work with unchanged registration identity. `Once` finishes once; `AfterCompletion`
recurs from the new successful completion timestamp. Module hash and freezing
threshold remain unchanged. Probe observations now expose registration identity,
start count and invariant-failure state; the shared unpublished Wire record owns
that shape. No new endpoint or production API was added.

Applications own call-error/retry policy. These fixtures do not qualify arbitrary
pending futures, every frozen interleaving or recovery after uninstall. README,
architecture, SAFETY and release guidance describe that distinction.

## Qualification

Current evidence: `/tmp/ic-timers-freeze-await.wMbdUW/`.
Passes: five exact installed subjects on Rust **1.91.0** / admitted Linux PocketIC
**16.1.0**: both ordinary frozen-await cases, both Watchdog freeze cases and the
existing accepted-work dispatch-timeout case. Build held and verified the exact
optimized Wasm. Receipt and server logs:
`/tmp/timer-qualification-1856382-0/runtime.json`.
Also passed: the mandatory-roster admission regression; strict MSRV Clippy for
the native driver/protocol and the Wasm runtime probe; formatting, changelog,
documentation link/consistency and diff checks. The original failed expectation
of automatic async resumption is retained with its actual observations and logs.
No full recovery/cohort/release gate, native macOS or live IC qualification ran.
No setup/install or dependency mutation ran in this latest slice.

Earlier driver evidence remains at `/tmp/ic-timers-driver-evidence.PLJBv2/`
(nine native checks, diagnostics failure injection, two Watchdog freeze subjects),
with receipt `/tmp/timer-qualification-1157270-0/runtime.json`. Earlier freeze
work remains at `/tmp/ic-timers-focused-freeze.ZiI29h/`. Historical runtime cleanup
and compiler-policy evidence remain at `/tmp/ic-timers-cleanup-yod5gscl/` and
`/tmp/ic-timers-docs-rust.6yXhDp/`; do not relabel them as current-graph full recovery.

## Outstanding delivery

No GitHub writes or fresh status queries ran in this batch. The most recent prior
check found Timers [#79](https://github.com/dragginzgame/ic-timers/issues/79) open:
Timers' predicate/proof were delivered, while Canic's inspector still needed
consumer integration. Keep it open until that delivery is verified. Canic #695
is unrelated pool cancellation, superseded into #710. Earlier isolated Canic
work remains at `/tmp/ic-timers-backlog-03114.GlZdEf/canic` and its parent
`canic-code.patch`; sibling checkouts remain untouched. Its public call-builder
retirement retains the Human Minor Closeout Gate. Scheduler normal return remains
the protocol assumption; arbitrary scheduler traps, permanently live futures and
uncertain-effect replay remain unqualified.
