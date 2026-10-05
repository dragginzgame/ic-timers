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
