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

The released 0.13.3 implementation carries a removed entry in its existing
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

These are operation intervals from the 0.13.2 artifact. A bound handle previously
copied three boxed identity components to
construct its detached token. An ordinary coalesced ensure removes that temporary
allocation/deallocation and reinstallation; paired Watchdog requests remove two
such copies. The same platform operations were already avoided by coalescing.
The subsequent 0.13.3 measurements below quantify operation costs; linked Wasm
savings are not established by either run.
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

Initial preparation had source and Rust formatting/parsing review only. The
maintainer's released source `864397a7c21eec4f396fe9617dbb8d8e1f9cfc73`
subsequently passed [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342),
including Linux checks, MSRV and both native macOS complete release gates.
The [Apple Silicon job](https://github.com/dragginzgame/ic-timers/actions/runs/37430538342/job/112160114006)
records 142 native tests, doctests, all 14 maintained PocketIC runtime subjects,
and policy cohorts passing with the audited PocketIC 16.0.0 artifact. This
includes the added capture-release subjects; native substitute and actual
canister evidence retain their separate contracts.

| Policy | Initial arm, 0.13.3 | Duplicate ensure, 0.13.3 | Cancel, 0.13.2 → 0.13.3 |
| --- | ---: | ---: | ---: |
| Once | 24,934 | 6,617 | 16,800 → 16,806 |
| AfterCompletion | 28,837 | 8,344 | 25,313 → 26,600 |
| Watchdog | 25,880 | 7,878 | 21,344 → 22,492 |

Compared with the preceding source-bound probe, duplicate ensures cost about
27–30% fewer instructions, while initial arming costs about 4% more and recurring
cancellation about 5% more. These are individual probe operation intervals,
including their brackets, not total-message or application throughput results.
Snapshot/inventory subjects are unchanged; executed-work subjects are slightly
lower. The correctness fix has an observable cold/control-path cost, so it
should not be described as a universal speedup. No heap bound or linked Wasm
reduction follows from these results.

Neither hosted log emitted linked byte sizes. The 0.13.4 host-only cohort change
adds `wasm_bytes` from the exact vector installed into PocketIC to the existing
row, including the baseline cohort. The four artifacts retain the same build
profile/toolchain and can be compared without a second file read or extra build.
At preparation, that output had not been executed; it cannot retroactively
establish sizes for 0.13.2 or 0.13.3. No target Wasm or runtime instrumentation changes.
Public APIs, snapshots, recurrence, generation allocation, dependency selection
and upgrade reconstruction are unchanged; 0.13.3 is a compatible fix. No new
feature, persistence, interval provider, scheduler data structure or global
control API is justified by the reviewed consumers.

## Released 0.13.5 cohort review

The completed [Apple Silicon gate](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942/job/112198306109)
and [Intel gate](https://github.com/dragginzgame/ic-timers/actions/runs/37442170942/job/112198305932)
for `98c4b296d7461525c15a01e30adbe33b75bcfa38` record exact installed byte
sizes for the four existing cohort artifacts, built with Rust 1.88 and the
testing workspace's `opt-level = "z"`, thin LTO and stripped release profile.
The probe lock selects registry `ic-metrics 0.1.6`; the root lock separately
selects compatible 0.1.7. Neither graph qualifies subsequent worktree edits.

| Cohort | Apple Silicon Wasm bytes | Intel Wasm bytes | Bytes above each host's baseline |
| --- | ---: | ---: | ---: |
| Baseline | 263,433 | 262,094 | 0 |
| Once | 316,941 | 315,602 | 53,508 |
| AfterCompletion | 317,563 | 316,224 | 54,130 |
| Watchdog | 318,293 | 316,954 | 54,860 |

Baseline already imports IC Timers initialization, snapshot and inventory APIs,
as well as the shared Candid/CDK probe surface. These are feature-reachability
comparisons within this probe, not the total size added by linking IC Timers into
an arbitrary canister. No historical Wasm reduction can be calculated from logs
that did not record byte sizes.

Every previously emitted cohort subject matches the inspected 0.13.3 Apple
Silicon row exactly: initial arm, duplicate ensure, cancellation, snapshot,
inventory, measured scheduler/work, memory-sampling brackets and dispatch cycles.
Both Watchdog calibration rows also match. The maintained probes therefore show
no instruction regression or new instruction saving in these intervals. Runtime
and probe canister sources are unchanged between those commits; this supports
keeping the current design rather than attributing release-tooling work to a
runtime optimization.

The Intel instruction and sampling fields match Apple Silicon, including both
calibration rows. Absolute Wasm sizes differ by 1,339 bytes across every cohort,
but the baseline-relative differences match. Recorded dispatch cycles differ:
Once/AfterCompletion/Watchdog are 16,780,004/16,788,704/33,578,100 on Intel versus
15,700,004/15,708,704/31,418,100 on Apple Silicon. The calibration cycle rows
also differ. These host-specific observations are retained separately; this
review establishes neither the cause nor a cross-host cycle regression.

The zero measured memory-sampling difference is limited to its bracket, and
dispatch cycles include the driven-round interval. Neither establishes zero
measurement cost universally or total-message instruction consumption, which
the calibration explicitly reports as unavailable. Watchdog is only 730 bytes
larger than AfterCompletion in this build; deleting its distinct prearmed recovery
semantics is not justified by that size difference. No further runtime change
or performance release is supported by this review.

## IC Metrics 0.5 released graph

Released IC Timers **0.17.0** at
[`5e0d0865248f6ebfc1f98c896581f21e2ce67831`](https://github.com/dragginzgame/ic-timers/commit/5e0d0865248f6ebfc1f98c896581f21e2ce67831)
selects one registry Metrics **0.5.0**, Testkit **0.31.0**, all four Host
**0.11.0** packages and PocketIC **16.1.0**. This supersedes the 0.4/0.30
preparation below; the finalized historical release notes retain their original
description. Current consumers exchanging measurement values must select the
same Metrics package identity or use `ic_timers::MeasurementSummary` directly.
A matching struct definition in Metrics 0.4 is still a different Rust type.

All seven published Metrics 0.5 Rust files match committed
[`01549632c0c3fa6e1ff315ce4de803ddfae904ad`](https://github.com/dragginzgame/ic-metrics/commit/01549632c0c3fa6e1ff315ce4de803ddfae904ad);
its arithmetic source is unchanged from 0.4. All 40 published Testkit 0.31 Rust
files match committed
[`f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a`](https://github.com/dragginzgame/ic-testkit/commit/f1ae9e6d3b0f3f20ec1e1f1b49c8b1dea3155e0a).
Its packaged CLI lock also selects Host 0.11. No timer adapter, arithmetic reader
or direct Host dependency is needed; Host/Testkit remain outside the production
library's dependency closure. Host 0.12's availability does not change Testkit's
selected 0.11 requirement or justify a second native dependency route.

Full locked offline metadata and source comparisons are retained under
`/tmp/ic-timers-shared031.aFkkRB/`. Read-only inspection of clean IcyDB source
`736c583ebf27097826517a0010b8e6ae87a811ad` finds declarations for Timers 0.17 and
Metrics 0.5; its root lock selects one Timers 0.17.0 and one Metrics 0.5.0. This
removes the reported duplicate Metrics selection in that graph without claiming
new downstream compilation or changing sibling files. Existing released Linux library, recovery/cohort,
MSRV and tag results are recorded by the
[release owner](../releasing.md#0170-release-acceptance). Both macOS jobs remain
queued at observation; [#37](https://github.com/dragginzgame/ic-timers/issues/37)
stays open for complete source-bound qualification. No Wasm/instruction delta is
measured by these source/graph checks, and no contributor test/build ran.

## IC Metrics 0.4 adoption

Incoming maintainer catalog/lock edits after released **0.16.7** select Metrics
**0.4.0**. Its reviewed published release is
[`97ec991776bb16efbab59faeb43bc72ce9ebcefa`](https://github.com/dragginzgame/ic-metrics/commit/97ec991776bb16efbab59faeb43bc72ce9ebcefa).
All seven published Rust source files match that commit and 0.3.7 byte for byte.
The dependency-free arithmetic and consumer-owned instruction reader require no
adapter. [#37](https://github.com/dragginzgame/ic-timers/issues/37) records the
real downstream mixed 0.3/0.4 graph that motivates package convergence.

IC Timers publicly re-exports `MeasurementSummary`; its 0.3 and 0.4 identities
are different Rust types despite identical arithmetic. The then-pending
**0.17.0** covered that public hard cut and the tooling-contract change. Its final
Metrics 0.5 selection is recorded above; this paragraph records earlier preparation.
That preparation selected one Metrics package, with no
alias, second dependency route, persisted layout or timer lifecycle change.

Complete locked offline metadata at `/tmp/ic-timers-shared030.roCrPG/metadata.json`
selects Metrics 0.4.0, Testkit 0.30.0, Host 0.11.0 and PocketIC 16.1.0. The incoming
catalog and all four local lock entries report 0.16.6 although the released HEAD
is 0.16.7; these maintainer-owned bytes are preserved. Contributor preparation
does not resolve that release metadata difference or bump package versions.
The user-operated minor preparation must select 0.17.0 before release.

Source equality gives no expected algorithmic instruction or heap change, but
no Wasm/instruction delta is measured for the new package graph. Source/metadata
review is not execution qualification; the normal user-operated gate retains
library, MSRV, probe, recovery/cohort and native host checks. #37 stays open for
delivery and qualification; no product test/build/lint ran for this review.
[Alignment feedback](https://github.com/dragginzgame/ic-timers/issues/37#issuecomment-6096552294)
records the preserved incoming selections and minor boundary.

## IC Metrics 0.3 adoption

The incoming root catalog selects registry `ic-metrics 0.3`; the one root lock
selects 0.3.0. This adoption is prepared for **0.16.0**. Released 0.15.0 at
`1410415d41212385234509524480186074972764` still selects Metrics 0.2.20.
The four local packages remain at 0.15.0 until maintainer-owned version preparation.

The downloaded registry 0.3.0 package's eight `src` files are byte-identical to
the owner's tagged 0.2.20 and 0.3.0 source trees. Metrics 0.3.0's release commit is
`070c768de881a6e966b93651e242e16e4f9f1111`. The crate remains dependency-free
and `no_std`; summary arithmetic, attribution and the consumer-owned instruction
reader require no implementation changes.

IC Timers publicly re-exports `MeasurementSummary` and returns it from
`TimerPerformance::scheduler_instructions` and `work_instructions`. Rust treats
the 0.2 and 0.3 package types as distinct despite identical source. Consumers
exchanging these values with a direct Metrics dependency must select 0.3 or
use `ic_timers::MeasurementSummary`. The next minor boundary covers that public
identity change. There is no old-type alias, dual dependency route or fallback;
this change introduces no persisted layout or timer lifecycle change.

Locked offline metadata on 2026-10-09 resolves exactly one registry Metrics
0.3.0 and all four local members through the root graph. Source comparison and
metadata evidence are retained at `/tmp/ic-timers-metrics03-current`'s referenced
directory. The checks preserve the incoming manifest and lock bytes. No new
test, build, lint, PocketIC, cohort or release execution ran for this preparation.
The maintainer-operated complete release gate must qualify the selected graph;
0.15.0's hosted results do not qualify Metrics 0.3.0.

Unchanged arithmetic gives no expected algorithmic instruction or heap delta.
No measured Wasm or instruction delta is available for this graph; package
identity and build inputs prevent claiming binary identity from source equality.
The release adds no runtime mechanism or speculative Metrics API.

The maintainer subsequently released the cut as **0.16.0** at
`984c2f0a92f7e3ebde604f88895b12fb2b78cb2c`, selecting registry Metrics **0.3.1**.
Its eight `src` files match tagged 0.3.0 byte for byte, preserving the reviewed
arithmetic. Exact-source [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37923550864)
passes Linux/MSRV and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37923551348)
passes; both complete native macOS gates remain queued at inspection. No fresh
cohort measurement is claimed. The earlier 0.3.0 metadata evidence remains
preparation evidence with its original graph.

## IC Metrics 0.2 adoption

The maintainer requested registry `ic-metrics 0.2` after publication. The root
catalog and lock already selected 0.2.0 when this continuation inspected them.
The inherited library declaration stays `workspace = true`, without the removed
`ic` feature. The nested probe workspace reaches that same declaration through
its path dependency; its separate lock is updated narrowly from 0.1.6 to 0.2.0
with `cargo update --manifest-path testing/Cargo.toml -p ic-metrics@0.1.6
--precise 0.2.0 --offline`. Every unrelated testing lock record and the existing
root lock are preserved. Both select registry checksum
`e6df432373e44c1956cbaa15548625efbebdbad4070a916e9fe0c7e4359f6265`.

The downloaded registry package is dependency-free, `no_std`, edition 2024 and
MSRV 1.88. Its `src/summary/mod.rs` is byte-identical to downloaded registry
0.1.6. The previously prepared production adapter reads
`ic0::performance_counter(1)` directly; the test-only native substitute remains
consumer-owned. This is source comparison, not new execution or measurement.
The earlier 0.1 extraction checks and released 0.13.5 cohorts retain their
original dependency identities and do not qualify this selection.

IC Timers exposes the shared type through its facade and the
`TimerPerformance::scheduler_instructions` / `work_instructions` return values.
Despite identical arithmetic, `ic_metrics 0.1::MeasurementSummary` and the 0.2
type are different Rust types. For example, passing a timer performance summary
to a function accepting the old direct-dependency type can stop compiling.
The [Cargo resolver's version-incompatibility guidance](https://doc.rust-lang.org/cargo/reference/resolver.html#version-incompatibility-hazards)
explains this boundary. The complete unpublished batch moves to 0.14.0 under
the pre-1.0 compatibility rule. Consumers exchanging these values must align
their direct dependency to 0.2 or use `ic_timers::MeasurementSummary`; no old
identity alias, dual reader or fallback is retained.

Both owning locked/offline metadata checks pass and dependency-tree inspection
finds one registry 0.2.0 package per workspace. Diff/source checks pass. No new
test, build, lint, PocketIC, cohort or release command was run during this
adoption; those remain maintainer-owned. There is no claimed Wasm or instruction
saving from moving the reader or updating the dependency. The scope addresses
[ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10).

The maintainer subsequently released this adoption as 0.14.0 at
`902323a9e896ce3771044fdc23a7a2d03d49cf28`. Cargo, both local-package lock
entries and tag `v0.14.0` agree. Its matching tag CI, main Linux/MSRV and Apple
Silicon complete gate passed; Intel was unfinished when inspected. The
[host record](../releasing.md#host-support) keeps exact run links and scope.
The earlier preparation checks remain separate from that hosted execution.

The completed [0.14.0 Apple Silicon gate](https://github.com/dragginzgame/ic-timers/actions/runs/37476635415/job/112313644296)
subsequently supplied that comparison. Its four cohort and two calibration rows
match the recorded 0.13.5 Apple Silicon rows exactly, including Wasm sizes,
operation instruction fields, sampling brackets and dispatch cycles. The new
registry 0.2.0 graph therefore shows zero observed delta in these maintained
subjects after consumer ownership of the counter reader. This is a same-host,
source-bound result, not a universal size/cost guarantee or qualification for
the subsequent structured-checker adoption.

## Histogram evaluation

The maintainer requested working through
[#22](https://github.com/dragginzgame/ic-timers/issues/22) after the upstream
histogram primitive became available. Evaluation at released 0.14.6
`0c90c391dff5960a7502fc15a0718b03631f2515`, with repository-only 0.14.7 preparation,
concludes **no histogram adoption**. The issue closes as not planned, with no
runtime, public snapshot or dependency change. The current root/testing locks
select ic-metrics 0.2.3/0.2.0 independently. Upstream publication/identity
evidence for 0.2.4 is linked in the issue; primitive availability is resolved.

Actual maintained consumers in the size/runtime probes and PocketIC subjects
project sample counts and totals through the existing work summary. They are
scheduler qualification workloads, not a production distribution requirement;
they establish neither useful application bounds nor an acceptable heap and
recording-cost budget. No repeated histogram/percentile accumulator is being
removed or replaced. Additional diagnostics alone do not justify growing every
registration and copied inventory snapshot.

`TimerPerformance::record_work` remains the owning sample boundary. A future
histogram must replace that role's summary internally and project the existing
summary from it, rather than keeping two aggregates. Its field payload adds
approximately `16*N + 8` bytes per timer: eight bounds add 136 bytes, or 1.36 MB
over 10,000 retained declarations, before layout/allocation and snapshot-copy
effects. Recording adds at most N bound comparisons and a saturating bucket
update. These are source estimates, not Wasm, heap or instruction measurements.

The runtime finishes the accepted callback-envelope interval before registry
recording; unchanged reported work instructions cannot qualify bucket overhead.
The population contains retained, normally completed accepted envelopes. Traps,
missing/superseded claims and terminal remove-on-stop declarations do not produce
retained samples; these are not all attempted jobs or exclusive application cost.
Existing summary counters are sufficient for the maintained contracts.

Reopen for a named application's distribution question, caller-selected useful
bounds and a storage/recording budget. Qualification must include actual IC
recording cost and raw Wasm with identical source/lock/workload inputs, plus
native boundary/saturation and claim/reset/removal tests. Native fake timings
and callback totals alone cannot establish that cost. No such measurement or
test ran during this evaluation. Keep this research outside the 0.14.7 tooling
release; do not add arbitrary buckets, a second registry or instrumentation modes.
