# Safety boundary

`ic-timers` is a higher-level wrapper around `ic-cdk-timers`. The CDK remains
the platform timer provider; this crate owns only the coordination it can
actually enforce and test.

## Current guarantees

The current pre-alpha crate provides:

- a pure generation-based control state machine;
- rejection of stale callback starts and completions;
- deterministic arbitration among scheduling, reconciliation, cancellation,
  and callback completion;
- checked conversion of relative delays to absolute nanosecond deadlines;
- an opaque platform timer handle with the direct `ic-cdk-timers` dependency
  isolated in `platform`; and
- inert provider-neutral snapshot values with bounded identities and
  saturating observation counters.

Snapshot values describe runtime observations. They are not authority to arm,
clear, restore, or mutate a timer and must not become an alternate control
path.

## Not yet guaranteed

The crate does not yet provide a live shared registry, measured callback
execution, lifecycle reconstruction, or a recovery watchdog. In particular:

- after-completion recurrence cannot recover when the callback traps or
  exhausts its instruction limit before arming a successor;
- a watchdog policy and pre-armed-successor snapshot type do not mean watchdog
  scheduling is implemented;
- interruption counters are representational until a recovery or lifecycle
  owner can establish that a started generation will not complete;
- epoch reset types do not restore scheduling state across upgrade; and
- no PocketIC evidence yet covers trap recovery, instruction exhaustion,
  upgrade reconstruction, ingress isolation, independent timers, or
  insufficient cycles.

Recovery-critical consumers should retain their proven timer runtime until the
corresponding behavior and evidence exist here.

## Failure and measurement semantics

Callback starts and completions are deliberately separate. A trap or
instruction exhaustion can prevent all post-run code, so the runtime must not
invent a completion, zero instruction cost, zero duration, or zero work count.
An interruption becomes observable only when a later watchdog or lifecycle
step establishes it.

All hot-path observation counters and aggregates saturate rather than trap.
Saturation protects timer execution; it does not make a saturated metric exact.
The runtime epoch identifies the reset scope so operators can distinguish a
reset from a genuine lifetime zero.

## Evidence required before recovery claims

A recovery implementation must add focused PocketIC cases for:

1. callback trap after a successor becomes authoritative;
2. instruction exhaustion at each scheduling boundary;
3. upgrade while scheduled and while logically running;
4. stale predecessor callbacks after replacement or reconstruction;
5. isolation between independent logical timers;
6. rejection of external ingress to internal callback paths; and
7. insufficient-cycle behavior without silent loss of the recovery schedule.

Only behavior covered by those tests should be described as recovery-capable.
