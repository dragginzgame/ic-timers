# IcyDB adoption record

Status: tagged IcyDB 0.226.1 at commit
`cd388cad96383f7c4c56054a8f27de608e9371e3` adopted exact `ic-timers` 0.3.4.
A validated post-tag IcyDB worktree upgrades to exact 0.3.5 and completes
claim-scoped armed-wakeup observation; that downstream worktree was uncommitted
when inspected on 2026-08-14.

## Dependency and ownership

The tagged release resolves one `ic-timers` 0.3.4 package. The validated
post-tag integration pins `ic-timers = "=0.3.5"` and still resolves exactly one
package. In both subjects, the generated database actor owns one retained
`WatchdogRegistration` with the fixed identity `icydb/startup/recovery` and a
one-second cadence. The lifecycle owner initializes and reconciles that
registration before application post-upgrade work.

The hard cut removed IcyDB's startup `TimerId`, active flag, cadence guard,
serial interval, and zero-delay cleanup path. The inspected workspace and
generated probe subjects contain no direct production `ic-cdk-timers` use;
the provider remains a private transitive dependency below `ic-timers`. IcyDB
persists readiness and terminal database failure, not timer handles,
generations, snapshots, or library policy.

The result mapping remains:

| IcyDB result | Timer completion | Watchdog decision |
| --- | --- | --- |
| recovery page made progress | success | continue |
| returned retryable recovery failure | retryable failure | continue |
| database is ready | no work | stop |
| durable terminal database failure | invariant failure | stop |

A normally returned mutation error that retains recovery controls invokes the
synchronous idempotent ensure seam before returning. Application timer probes
also use the same `ic-timers` registry, so this evidence is not based on an
IcyDB-only fallback runtime.

## Accepted downstream evidence

IcyDB's maintained 0.225 status and integration suite record:

- successor survival and later progress after an explicit work trap and
  40-billion-instruction exhaustion;
- rejection of external timer-executor ingress;
- synchronous upgrade reconstruction before downstream hooks;
- terminal watchdog unregistration;
- one coalesced attempt after a 300-second overdue jump; and
- independent application-timer progress while the recovery watchdog traps.

The tagged 0.3.4 subject reports 4,164,071 optimized raw Wasm bytes. The
validated 0.3.5 integration reports 4,164,445 bytes, an increase of 374 bytes.
Its compiler-emitted artifact is 4,767,744 bytes and deterministic gzip is
1,606,653 bytes. Relative to the 4,125,495-byte direct-provider subject, the
shared runtime adds 38,950 raw bytes and remains within IcyDB's 65,536-byte
owner budget.

The normally completed watchdog sample remains 1,163 instructions and two
application callbacks remain 1,986 instructions total. Candid is byte-identical
at 60,348 bytes with SHA-256
`a3a396639a0b809cf8865fc838ec9f69ada7ee291b3fdbdedd4c3f55525e97e5`.
The public IcyDB facade and `ic-timers` compile to Wasm on Rust 1.88. The
validated graph contains exactly one `ic-timers` 0.3.5 package, with
`ic-cdk-timers` private and transitive.

These are maintained downstream results inspected read-only; this repository
did not modify IcyDB. IcyDB reran the focused real-canister recovery evidence
for its 0.3.5 integration. Full downstream repository validation remains
IcyDB-owner work.

## Claim-scoped integration

The tagged 0.226.1 source derives its reporting bit from
`TimerSnapshot::next_deadline_ns()`. The validated post-tag integration instead
calls `WatchdogRegistration::has_armed_wakeup()`, correctly distinguishing an
unarmed live claim from an expired or invalid claim. The observation counts
the pre-armed watchdog successor and excludes the separately queued work
callback.

Durable recovery demand still invokes `ensure_scheduled()` unconditionally.
The observation is therefore reporting only and does not introduce a
check-then-arm race. No public IcyDB API, Candid surface, persisted format, or
compatibility path changes. The remaining downstream step is to land that
already-validated post-tag worktree.

Canic has not adopted `ic-timers`. A combined IcyDB/Canic application must
still prove dependency unification and a canister-wide provider inventory.
