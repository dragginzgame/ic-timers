# IcyDB adoption record

Status: adopted by the accepted IcyDB 0.226.1 candidate at commit
`4b6e6f7e5ebf164e25709626290e56cb8811f619`; that downstream candidate was
ahead of its published release when inspected on 2026-08-14.

## Dependency and ownership

IcyDB pins `ic-timers = "=0.3.4"` and resolves one `ic-timers` package. Its
generated database actor owns one retained `WatchdogRegistration` with the
fixed identity `icydb/startup/recovery` and a one-second cadence. The lifecycle
owner initializes and reconciles that registration before application
post-upgrade work.

The hard cut removed IcyDB's startup `TimerId`, active flag, cadence guard,
serial interval, and zero-delay cleanup path. The inspected workspace and
generated probe subjects contained no direct production `ic-cdk-timers` use;
the provider remained a private transitive dependency below `ic-timers`.
IcyDB persists readiness and terminal database failure, not timer handles,
generations, snapshots, or library policy.

The result mapping is:

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

The accepted measurements report 1,163 instructions for the watchdog sample,
1,986 instructions across two application callbacks, and a final optimized raw
Wasm size of 4,164,071 bytes. The exact 0.3.4 substitution added 3 raw bytes
over the 0.3.3 subject; the broader direct-provider-to-shared-runtime candidate
delta was 38,576 raw bytes. The Candid surface remained unchanged. The public
IcyDB facade and its `ic-timers` surface compiled to Wasm on Rust 1.88.

These are maintained downstream results inspected read-only; this repository
did not rerun or modify IcyDB's suite.

## Follow-up boundary

The 0.226.1 candidate currently derives its operator-facing scheduled bit from
`TimerSnapshot::next_deadline_ns()`. Starting with the 0.3.5 candidate, a
retained registration can instead call `has_armed_wakeup()` to observe exact
future provider-handle ownership. That is a downstream hard cut after 0.3.5 is
released; it is not required for the established watchdog safety protocol.

Canic has not adopted `ic-timers`. A combined IcyDB/Canic application must
still prove dependency unification and a canister-wide provider inventory.
