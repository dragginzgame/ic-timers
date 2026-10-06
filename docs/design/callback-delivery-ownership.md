# Callback delivery ownership

This records the implementation and evidence scope of the callback delivery
change. It does not select a release or establish deployment validation.

## Consume the delivered handle through the selected entry

Runtime work delivery previously called registry-level handle consumption and
then work acceptance. Each operation independently selected the same entry.
`begin_ordinary` and `begin_watchdog_work` now consume the delivered handle through
their selected entry before accepting work. Runtime dispatch calls that one
operation and still releases the registry borrow before invoking consumer work.

The shared private `Entry::consume_provider_handle` owner verifies claim, policy
role and owned slot generation. It consumes a fired handle independently of
whether control state accepts work, preserving the previous delivery ordering.
An older generation or superseded claim cannot consume the replacement's handle.
Acceptance still independently checks work role, claim, generation and state,
records rejection or start, and returns only the matching callback. The old
registry-level consumption method and single-caller slot accessor are deleted.

## Detach Watchdog scheduler capabilities before the transition

Watchdog scheduler delivery must detach the remaining handles before its registry
transition can remove a transient declaration. This remains a separate stage from
work acceptance. `take_watchdog_scheduler_handles` now consumes the delivered
scheduler handle and detaches those remaining capabilities through one selected
entry, replacing the runtime's two registry operations and stale-claim match.

Missing and superseded claims still yield empty handles as normal stale delivery;
other registry errors retain their typed propagation. The scheduler transition
still follows detachment under the same runtime borrow, with unchanged time
sampling, allocation order, unacknowledged accounting and terminal cleanup.
Only runtime invokes provider clearing or arming after releasing the borrow.
Public control restoration, effect confirmation and Watchdog callback rollback
remain distinct and unchanged.

The change removes repeated entry selection and a separately callable consumption
phase, without new state, token types or a callback framework. No timing, size or
performance result is claimed. It is private and behavior-preserving: public APIs,
snapshots, dependency versions, generation sequences and persisted formats are
unchanged. No generated artifact or downstream adapter needs updating, and
downstream work remains deferred.

## Focused verification

Existing fixtures were updated rather than adding a test suite:

- `provider_roles_keep_paired_handles_distinct_and_reject_policy_mismatches`
  exercises the entry's consumption owner for all three provider roles. It now
  rejects a mismatched claim with the same role and callback generation, alongside
  its existing stale-generation and cross-policy role cases. It retains paired
  slot identity, rejected-binding cleanup and mock provider-count assertions.
  The expanded ownership matrix has a justified local line-limit expectation.
- `stale_reused_identity_callback_cannot_change_handles_or_measurements` queues
  the old delivery through the mock provider and real runtime dispatch. It
  requires a stale event without a work start, replacement handle consumption or
  measurement changes, then executes the replacement normally. Its independent
  late-measurement boundary check remains.
- `suspended_after_completion_uses_completion_time_and_exact_reconciliation`
  now checks that the registration owns no wakeup after work suspends. Existing
  suspended Once coverage already checks this ownership boundary.

Existing Watchdog prearming, continuation, cancellation, dispatched recovery,
terminal failure/lifetime and provider failure fixtures remain applicable. Native
mock behavior does not establish IC rollback or provider-heap results.

Tests, builds, lint gates and deployment validation remain maintainer-owned and
have not been run. Before the Shared Tooling adoption, changed Rust formatting,
diff whitespace, the then-current prepared release metadata and release-truth
checks, and locked offline metadata checks passed. Those checks do not validate
runtime behavior or the subsequently changed release tooling; its verification
is scoped in the [adoption record](../shared-tooling.md). Cargo versions and lockfiles
remain unchanged. The root changelog owns the maintainer's release selection;
this document does not select a version or establish delivery.
All version bumps and Git/release execution remain maintainer-owned.


## Capture removal and coalesced requests

The 2026-10-06 source review at release commit
`134899f1620b29f51711ff479c37c681db3fb9ec` found that ordinary abandonment
returned removed callbacks after its registry borrow, but normal cancellation,
unregistration, transient control failure and rejected registration could drop
consumer captures inside `RUNTIME`'s mutable borrow. A nontrapping destructor
calling `timer_inventory` observed `RuntimeBusy`; cleanup that consumed another
claim could lose its control capability after that rejection.

The pending 0.13.3 implementation carries a removed entry in its existing
`RegistryTransition`. Runtime applies provider effects before dropping that
transition, outside registry access. Rejected public registration retains a
local `Rc` until registration access ends. This is temporary destructor custody,
not a retained timer, retry queue or new mutation authority. Ordinary abandonment
keeps its separate provider-call-free cleanup contract. Capture destructors
must still be bounded and nontrapping, and must not schedule provider work from
CDK cleanup.

Public control now validates its claim before applying the pure transition.
Rejected requests leave owned handles intact. A no-effect transition retains
those handles in the selected entry; a removal transfers the entry and detaches
its handles from that returned owner; other effects detach handles after the
transition. No user callback runs in between. Watchdog callback completion keeps
its existing explicit detachment and trap/rollback rule.
The obsolete detached-error restoration finalizer is removed: input rejection
no longer creates detached capabilities needing restoration. Binding/restoration
errors after an actual provider effect retain their existing cleanup owners.

The release baseline's successful Apple Silicon gate reports these instruction
subjects from the maintained size probe:

| Policy | Initial arm | Duplicate ensure |
| --- | ---: | ---: |
| Once | 24,006 | 9,512 |
| AfterCompletion | 27,695 | 11,496 |
| Watchdog | 24,924 | 10,872 |

These are operation intervals from the 0.13.2 artifact, not results for the pending
change. A bound handle previously copied three boxed identity components to
construct its detached token. An ordinary coalesced ensure removes that temporary
allocation/deallocation and reinstallation; paired Watchdog requests remove two
such copies. The same platform operations were already avoided by coalescing.
No percentage instruction reduction or linked Wasm saving has been measured.
A removal now makes one temporary boxed-entry allocation so ordinary transition
values stay small; the allocation lasts only until provider effects finish.
Successful registration adds one short-lived `Rc` increment/decrement, with no
additional callback allocation. There is no persistent heap-growth claim.

Native fixtures cover removed captures inspecting the absent entry, reusing its
identity, and mutating timers after provider cleanup across all three policies;
rejected factories; transient binding/scheduler failures; and rejected/coalesced
requests retaining installation-fault injections until a real arm. PocketIC adds
same-message armed removal for all three policies and checks registry access
from capture Drop in existing ordinary abandonment subjects. No provider work is
scheduled by those PocketIC destructors.

This pending batch has source and Rust formatting/parsing review only. Native,
Clippy/MSRV, PocketIC and cohort execution remain maintainer-owned. Acceptance
requires the complete release gate, the new capture and handle-preservation
fixtures, unchanged trap/await/cancellation evidence, and comparison with the
baseline operation intervals and linked cohort Wasms. Reject the optimization
if duplicate ensures do not improve or actual-effect paths regress materially.
Public APIs, snapshots, recurrence, generation allocation, dependency selection
and upgrade reconstruction are unchanged; 0.13.3 is a compatible fix. No new
feature, persistence, interval provider, scheduler data structure or global
control API is justified by the reviewed consumers.
