![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-10

## Purpose

This is the compact session handoff. Historical implementation, delivery
references and validation belong in [release notes](../changelog/README.md),
[audits](../audits/code-hygiene.md) and the [safety boundary](../../SAFETY.md).

## Current release and remaining acceptance

Pushed **0.16.4** is `da921fc899d2c7c99ed0a6cacac6e2111307ac71`.
The maintainer reports it pushed. Its
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37967730569)
passes Linux checks, explicit MSRV and both complete native macOS gates. [Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37967730677)
passes. The required Testkit handoff acceptance is complete; do not dispatch
duplicate observations.

The actual released root graph selects Testkit **0.28.0**, all four Host crates
**0.10.1**, Metrics **0.3.5** and PocketIC **16.1.0**, four local members 0.16.4.
The earlier 0.16.4 preparation notes describe older selections, not this final
tagged graph. The [qualification owner](../releasing.md#testkit-028-and-host-010-released-graph)
records the Host durable-publication cut and why this private startup harness
needs no writer adapter, public timer change or retained-data reset. The released graph remains its own execution identity. An incoming lock now
updates Metrics 0.3.5 -> 0.3.6, cc 1.6.0 -> 1.7.0, smallvec 1.16.2 -> 1.16.3
and syn 3.0.6 -> 3.0.7. All seven published Metrics Rust source files are identical;
no adapter change follows. Its private inspector/tooling changes do not affect
our arithmetic callers.
Preserve those bytes; cheap locked offline metadata passes with four local 0.16.4
members, but the refreshed transitive graph has no new execution qualification.

Released 0.16.4 adopts Shared **0.2.8** and includes both the portable direct-policy
repair and the version-preparation fixture correction. The
[adoption owner](../shared-tooling.md#shared-tooling-028-make-admission) preserves
the earlier retained failure evidence. Both 0.16.3 macOS jobs have now failed
before the native gate at the combined target-specific override/export parse.
0.16.4 removes that exact syntax; its new native jobs now pass.
The [host owner](../releasing.md#host-support) retains this distinction. Released
0.16.3 Linux product evidence remains bound to its own source/graph/toolchain.

The completed 0.16.4 Linux job **113946212101** passes the repaired version
preparation, release gate, actual staged-hook fixture, 142 native library tests,
14 PocketIC recovery subjects and all four policy cohorts at Rust 1.99.0. Its
[qualification owner](../releasing.md#testkit-028-and-host-010-released-graph)
retains actual source/graph and raw log scope. This qualifies that pushed Linux
graph, not the uncommitted Shared 0.2.9 selection or native macOS behavior.

The later completed Intel **113946212038** and ARM **113946212124** jobs each
prove Testkit host-specific setup/admission, complete release-gate/Make/hook
checks, 14 maintained PocketIC recovery subjects and four policy cohorts.
Native Wasm subjects use explicit Rust **1.88.0**; Linux uses **1.99.0**. Do not
compare those cross-toolchain cohort values as regressions or improvements.
[The graph owner](../releasing.md#testkit-028-and-host-010-released-graph) retains
raw native logs and host/source identities. **#34 is closed** with
[completed acceptance](https://github.com/dragginzgame/ic-timers/issues/34#issuecomment-6095003127).
The original #35 Make/formatter adoption has the same all-host evidence.

The authorized continuation prepares one undated **0.16.5** repository-only draft
and adopts committed Shared **0.2.11**
`83efac446348dea024798a331d77933b24b429dc`, **55/30/11** selections.
The [adoption owner](../shared-tooling.md#shared-tooling-0211-formatting-and-make-admission)
records the new concise formatter and independent MAKEFLAGS/MFLAGS admission,
canonical companion propagation to all five actual-Make fixtures and staged
index, and CI retention of complete `formatting.*` diagnostics. This source's
VERSION says 0.2.11 although its commit subject says 0.2.10. No optional fleet or
registry observer, Cargo selection, tool pin or product timer change is added.

**#35 remains open** for the new consumer source's actual fixture and Linux/native
macOS qualification. Shared #30's correction is now committed and adopted; its
upstream acceptance remains with that issue. New success/failure/log-transport
and hidden-mode consumer cases are written but unexecuted. Permitted
source/export/integrity/syntax/link/metadata checks are preparation only.
Tests/builds/lint/setup and all Cargo/release effects remain user-owned. The
incoming Metrics/cc/smallvec/syn graph likewise does not inherit released qualification.
No new timer defect or feature is evidenced by these tooling issues.

**#30 is closed with completed compact transport qualification.** All six frozen v0.14.23
producers reach the intended failure and archive/upload successfully; all six
hosted verifiers and actual downloaded archives pass. The [evidence owner](../releasing.md#compact-hosted-qualification-at-01423)
records all six artifact IDs, sizes, exact source/attempt and retained logs.
The new native macOS results qualify that frozen compact caller; they do not
qualify the later handoff or incoming lock. #34's separate three-host acceptance
is now complete at 0.16.4, with its own source-bound evidence above.

Earlier source-bound records follow.

Released **0.16.0** is `984c2f0a92f7e3ebde604f88895b12fb2b78cb2c`.
The maintainer reports it live. Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37923550864)
passes Linux/MSRV, with both complete macOS gates queued; [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37923551348)
passes. The released lock selects Metrics **0.3.1**, Testkit **0.26.0**, Host
**0.9.1** and PocketIC **16.1.0**. Registry Metrics 0.3.1's eight source files
match 0.3.0 byte for byte. The public summary identity cut is delivered in the
0.16 minor line; the earlier 0.3.0 preparation below retains its original scope.

Incoming maintainer catalog/lock edits now select Testkit **0.27.0**. Preserve
those bytes. The undated **0.16.1** notes prepare this repository-only harness/CLI
update, with [source and qualification scope](../releasing.md#testkit-027-preparation).
Locked offline metadata passes with one Testkit, Metrics and PocketIC package
and four local members still at 0.16.0. No harness rewrite is needed; no Cargo
mutation, test/build/lint, setup or release execution ran during this preparation.
Testkit's released 0.27 Linux portable/MSRV/concurrency jobs pass, Linux complete
checks are in progress, and both native macOS matrices remain queued. Adoption
acceptance is pending; do not infer it from registry availability or metadata.

The existing #30 observations still have early Intel and both late macOS
producers queued. The existing #34 consumer native acceptance is also pending.
Do not redispatch observations or close either issue before its required evidence.

Earlier source-bound records follow.

Released **0.15.0** is `1410415d41212385234509524480186074972764`.
The maintainer reports it live. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37920576526)
passes Linux/MSRV with both macOS gates queued; [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37920576620)
passes. [#34](https://github.com/dragginzgame/ic-timers/issues/34) records the
released Testkit handoff; complete three-host acceptance remains pending. The
released graph selects Testkit 0.26.0, Host 0.9.1, Metrics 0.2.20 and PocketIC 16.1.0.
Linux logs prove actual selected CLI/server setup and adapter/release-gate fixtures;
the Linux CI job does not run the product startup/recovery/cohort subjects. Do not
count setup alone as Linux product startup acceptance. Both complete macOS jobs
still await runners.

A later incoming catalog/lock update selects Metrics **0.3.0**. Arithmetic source
is unchanged from 0.2.20, but the public `MeasurementSummary` has a different Rust
package identity. Its adoption requires **0.16.0**, not a 0.15 patch. Preserve the
incoming edits; do not describe them as released or create executable compatibility
aliases. The undated **0.16.0** changelog now prepares that adoption; its
[owner](../design/callback-delivery-ownership.md#ic-metrics-03-adoption) records
registry source equality and the single resolved package identity. Locked offline
metadata passes with all four local versions still 0.15.0. No Cargo mutation,
test, build, lint or release execution ran during this preparation.

Shared 0.2.2 `ee48bb37c98c771e77b92fd891f0757d8c1c8b99` changes the standalone CI
tool installer and its test, which this consumer does not select. There is no
urgent runtime/helper adoption to justify another tooling-only release. The
independent cargo-sort consolidation still awaits the original hook tool-root input.

The earlier preparation and 0.14.23 qualification follow with their source scope.

Released **0.14.23** is `10a392f98d42701959d0c1d2deddfbef5c96144a`, tag
`v0.14.23`, observed in the local release commit and hosted runs.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37904587019)
passes; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37904586955)
now passes Linux, MSRV and both complete native macOS gates, qualifying the
released cleanup callers. The released lock
selects Testkit 0.25.4, all four Host crates at 0.8.8, Metrics 0.2.18 and
PocketIC 16.1.0. Prior 0.14.22 passed all normal native gates. Registry publication
was not independently checked.

Released **0.14.23** adopts committed Shared follow-up
`3d33cd250fcae7dbe5cabe44b2abd6b2c91a1822` through **49/33/13** snapshots
(labelled 0.1.34, committed VERSION 0.1.33), with complete upstream native CI.
For [#33](https://github.com/dragginzgame/ic-timers/issues/33), removed the unused
fleet reporter and dedicated suite, their manifest records and local CI/help
references: **405 code LOC** in two files. Workspace LOC, all tool setup/check
commands and required companions remain. The optional shared target explains its
central owner when unselected; no sibling is invoked implicitly.

The snapshot also fixes PocketIC alignment's general CDPATH/unusual-directory
handling and refuses a directory lost before Cargo starts. Keep the absolute-path
Make caller and its space/symlink fixture. Existing selected command/alignment
fixtures cover these changes; execution and consumer native qualification remain
user-owned. The [adoption owner](../shared-tooling.md#shared-tooling-committed-0134-follow-up)
records exact scope and evidence at `/tmp/ic-timers-01423.KPKxjw/`.

Host 0.8.8's direct-child ownership and no-follow streaming hashing do not justify
new Timers callers: Testkit remains the server owner. The dependency edits are
now in the released lock and qualified by matching native main CI. Contributors
did not mutate that graph. Timer source/API and consumer IC/host
pins were unchanged by the tooling cleanup.

The maintainer authorized the latest Shared ownership hard cut. Released **0.15.0**
supersedes the 0.14.24 draft; Cargo versions are now 0.15.0. All three
snapshots now select committed Shared **0.2.1**
`06b2e22f6bd213f1a590eb2a8797aee34c42dd69` (VERSION 0.2.1) through **49/30/13**
exports. Remove shared PocketIC helpers/fixture and matrix rows together; retain
the other five tools. The [adoption owner](../shared-tooling.md#shared-tooling-020-hard-cut)
records exact scope, deleted functions and pending native qualification.

`make install-testkit-server` explicitly prepares the root-lock selected Testkit
CLI and its owner server. `pocketic-check` is offline. Release preparation and
CI/update-dev select the adapter; watchdog/cohorts pass only the rechecked admitted
path and retain their existing fresh-server topology. Remove the local downloader,
raw-hash/version catalog, alignment rule, Make override and their tests. Failed
CLI builds/provisioning attempts remain retained; admitted server payloads are
not archived. The existing #30 size/time output and its tests stay in the one
draft. Timer API, production Wasm and instructions are unchanged.

Incoming maintainer edits now select Testkit 0.26 in the root catalog and
Testkit 0.26.0, all four Host 0.9.1 and Metrics 0.3.0 in the current lock.
The Metrics update is after the 0.15.0 release and still awaits adoption. Contributors
did not mutate Cargo or update dependencies. The adapter
reads that sole lock instead of introducing a second CLI version catalog. The
Shared exception parser fix is adopted exactly; our actual exceptions still
have one document. Earlier Shared 0.2.0 Linux/lint and production-installer Linux qualification pass;
both native macOS assessments remain pending. Source/syntax/snapshot/document checks are preparation evidence,
not consumer setup or real startup qualification. New fixtures/builds/lint and
release execution remain user-owned; no installation ran for this cut.

The authorized cargo-sort consolidation remains separate and pending. The
canonical index hook still exposes no invoking tool-root for nested selected
receipt admission. Keep its current formatter route until that owner input and
native production evidence arrive; do not patch immutable shared payloads.

[#30](https://github.com/dragginzgame/ic-timers/issues/30) is actively qualifying
frozen v0.14.23 in the authorized [early run](https://github.com/dragginzgame/ic-timers/actions/runs/37912633994)
and [late run](https://github.com/dragginzgame/ic-timers/actions/runs/37912637422),
both attempt 1. Linux reaches the intended status 22/2, archives/uploads, and
its actual downloads pass the frozen verifier. Early/late outer ZIPs are
1,341/5,118 bytes; late inner tar.gz is 4,649 bytes. Early ARM also reaches status 22, uploads artifact 11610964142
(10,711-byte ZIP), and its frozen-source download passes. Early Intel and both
late macOS producers plus hosted verifiers remain pending: keep the issue open, do not redispatch or count an
unrelated setup failure. The [evidence owner](../releasing.md#compact-hosted-qualification-at-01423)
records identity, comparison/time scope and retained files at
`/tmp/ic-timers-issue30.vdZlUd/`. These runs qualify released compact transport,
not the dirty 0.15.0 measurement output or incoming lock graph. The six frozen
v0.14.17 observations remain complete with their original scope.

#33 is closed with all three native logs. Shared #86's committed parser fix is
now adopted; native acceptance remains upstream-owned. Current #30 work ran its
authorized focused fixtures and two hosted dispatches before this hard cut.
Those results do not qualify the subsequent changed fixtures or five-tool
shape. No dependency update, staging, commit, version mutation, tag, push or
release execution ran.

## Released 0.14.5 tooling

Released compatible 0.14.5 at `c84d4e4` implements
[#17](https://github.com/dragginzgame/ic-timers/issues/17) and
[#18](https://github.com/dragginzgame/ic-timers/issues/18). That release's three exact snapshots
identify clean committed Shared Tooling 0.1.12
`33c2a6f0018a94915f819ff219e270500ed5b73b`: 25 baseline files, 19 audit/setup files
and 17 nested helpers. Overlapping root integrity records were refreshed together.
The paired maintenance policy is adopted while tests/builds/lint and all release
execution remain maintainer-owned. Sibling file edits still require their own
authorization. The tooling batch leaves timer source and dependency selections
unchanged.

`release-check` directly selects the shared release-command checker with the
actual root and `tool-versions.env`. The copied `test-standard-release.sh` is
removed with no wrapper; local metadata/index/lock/recovery fixtures remain.
No named function/type was deleted. Standard release pushes now recheck and use
the captured sole destination URL. Snapshot verification hashes files without
executing inspected code. A new isolated fixture checks all three actual exports
and rejects payload-only, helper-only and combined corruption without executing
a changed helper. Failed scratch/logs remain retained.

The referenced read-only CI helper and flat governance file list are adopted;
existing guides are checked in the export. The list does not automatically widen
consumer manifests. Optional shared suite prerequisites, standalone yq setup and
upstream artifact-upload workflows are not introduced. Local host/IC pins, system
ripgrep preparation, independent workspaces and audited PocketIC gate remain.
See the [adoption owner](../shared-tooling.md#shared-tooling-0112-adoption).

Exact-source [upstream 0.1.12 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37511845192)
passed Linux portable regression, lint/security and both native macOS jobs.
Consumer source/export/modes, snapshot integrity, shell syntax, local/exported
documentation references and diff checks are preparation evidence. No contributor
tests, builds, lint, installation or release effects ran during preparation.
The maintainer subsequently released the batch. Exact-source
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526800896)
passed Linux, MSRV and both complete native macOS gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526801007) passed.
The consumer snapshot corruption, shared command checker and release runner
fixtures pass on all three hosts. #17/#18 were closed with this evidence;
#11–#16 remain closed. No contributor validation ran during this review.

Released 0.14.4 at `49e4e8a` passed main Linux/MSRV and both complete native macOS
release gates plus matching tag CI. It qualifies the previous tag-checker, logger
and restricted-PATH fixture repair. #16 was closed with the maintainer's explicit
authorization and exact hosted evidence; #11–#15 remain closed. The earlier local
fixture failure and its repair retain their scoped
[evidence](../shared-tooling.md#consumer-restricted-path-fixture-repair).
The maintainer reports publication live; registry publication was not independently
checked. No upstream files were changed.

## Consumer-owned instruction reader

Released 0.14.0 includes the consumer-owned reader and registry ic-metrics
0.2.0 in both independent lock graphs. Production uses the already-owned ic0
counter-1 binding; test-only native counter handling is unchanged. The retired
`ic` feature is absent. Summary arithmetic is unchanged, but the exposed
`MeasurementSummary` has a new Rust package identity: consumers passing it to
or from a direct ic-metrics dependency must align that dependency to 0.2 or use
IC Timers' re-export. This public type cut requires the next minor line and
supersedes the entire unpublished 0.13.6 draft; it introduces no compatibility
alias. Scheduler/work attribution, sample admission and registration identity
remain unchanged. See the [adoption owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption)
and [ic-metrics #10](https://github.com/dragginzgame/ic-metrics/issues/10).

Earlier extraction qualification used 0.1 arithmetic. With the maintainer's
then-existing explicit extraction permission, strict native
library/tests and Wasm library Clippy passed. The two named role-specific
saturation and memory-projection tests passed. Both independent locked/offline
graphs resolved without changing package selections; testing's metrics lock edge
to ic0 was removed. Source/manifest/lock hashes stayed unchanged through checks.
Native tests use the consumer-owned fake and provide no IC execution evidence.
Logs and identities remain in ic-metrics' `target/evidence/arithmetic-cut-020/`.
Those checks do not qualify the subsequent 0.2 dependency selection. The current
adoption ran only a targeted offline testing-lock update, both cheap locked
metadata checks, dependency-tree inspection and diff/source checks. Root already
selected 0.2.0 and was preserved; every non-metrics testing lock record is
unchanged. No new tests, builds, lint gates, commit, release or publication ran;
that adoption's preparation evidence remains separate from hosted qualification.
The maintainer subsequently released 0.14.0 at `902323a`; matching tag CI and
main Linux/MSRV and both complete macOS gates passed. See
the [source-bound host record](../releasing.md#host-support).

## Release state

- Read `[workspace.package].version` in [Cargo.toml](../../Cargo.toml) for
  package identity. The top [changelog section](../../CHANGELOG.md) records the
  released batch or, when present, its automatically selected, undated next version.
  The maintainer's bump finalizes and dates it; a dated section alone does not prove
  tagging, publication or deployment.
- Released Cargo/lock, finalized changelog and tag identify
  `5c6b7f45d72f9051b4e155410ec47339885dc4f2` (0.14.21). Main Linux/MSRV and both
  complete native macOS gates plus tag truth pass. The compatible 0.14.22 draft
  changes consumer evidence selection and retains incoming dependency selections;
  package identity remains 0.14.21.
  Closed #23's observations remain at 0.14.15; complete frozen 0.14.17 round trips
  retain their own exact runs. Evidence belongs in the
  [host record](../releasing.md#host-support).
- During the 2026-10-07 evidence review, an external root-lock edit selected
  ic-metrics 0.2.3 while the independent testing lock still selected 0.2.0.
  That edit was preserved during preparation and is now committed in 0.14.6;
  its complete hosted qualification remains separate from the subsequent
  tooling worktree. Those independently selected graphs were distinct at that source; current
  consolidation supersedes their boundary without rewriting that evidence.
- Released 0.13.3 covers a compatible callback-capture destruction fix
  and removal of handle detachment/reinstallation for rejected/coalesced public
  requests. API, snapshots, recurrence, generations and dependencies are unchanged.
  Removed captures remain in their returned transition until provider cleanup
  finishes; rejected factories retain a local Rc across registry access. Native
  and PocketIC capture-access fixtures passed in matching hosted qualification.
  Duplicate ensure costs fell about 27–30%; initial arm rose about 4%, and
  recurring cancellation about 5% in the maintained operation probes. These are
  operation intervals, not a universal speedup or total-message cost.
  Tests, lint/build, MSRV, PocketIC, cohort measurement
  and release execution remain maintainer-owned. The
  [delivery ownership contract](../design/callback-delivery-ownership.md#capture-removal-and-coalesced-requests)
  records implementation, baseline cost, temporary allocation and acceptance.
- Released 0.13.4 covers compatible repository-only release recovery
  for [#10](https://github.com/dragginzgame/ic-timers/issues/10), using reviewed
  Shared Tooling `cb86188`. Late adapters inspect exact `RELEASE_COMMIT` metadata
  and tags separately from HEAD; normal commands finish an older committed
  release before fresh validation for newer fixes or another requested increment.
  Current adapter code owns checks, with no execution of old snapshot scripts.
  The 22-file snapshot includes the maintenance rule, hook failure fix and
  canonical validation logger. Preflight rejects hidden staged implementation
  edits, commit admission checks the exact prepared index, and real release-gate
  failures retain unique raw logs across retries outside tracked release inputs.
  Preflight now delegates selected dependency-cache preparation to `make fetch`
  before validation, avoiding an offline cache check that blocked cold checkouts.
  The real-index fixture covers failed Git producers/partial output and both
  locked fetches, preserving metadata and failing before release intent. No
  dependency fetching was performed during contributor preparation.
  The host-only cohort now appends exact loaded `wasm_bytes` to its existing
  baseline/policy rows. Old hosted logs omitted byte sizes; no historical Wasm
  delta is inferred. This changes evidence output only, with no target Wasm
  instrumentation or runtime behavior change.
  Contributor source/snapshot/syntax/diff inspection was preparation evidence;
  the subsequent hosted attempt failed as recorded above. Timer runtime and
  dependency selections are unchanged. A repository-only patch retains the
  complete gate rather than using tag success as complete qualification.
- Released 0.13.5 repairs those compatible repository-only failures.
  Metadata and Git admission functions explicitly propagate failed commands
  instead of relying on Bash 3.2 subshell errexit. Sort and lock failures are
  injected independently for both workspaces. The index fixture deliberately
  uses a shallow clone with its own controlled baseline tag, satisfying impact
  classification without remote history or production changes. Both fixtures
  report captured adapter output before failure cleanup. Contributor source,
  shell syntax and diff inspection was preparation evidence; subsequent Linux
  and both native macOS hosted gates passed both repaired fixtures. The release-check
  repair leaves runtime sources unchanged.
  The separate dependency update requires `ic-metrics 0.1.6`; the maintainer's
  release preflight stopped at the stale testing lock before version mutation.
  With explicit authorization, a targeted offline Cargo update aligned that lock
  to 0.1.6 and preserved every other record. Both existing locked metadata checks
  pass; each independent graph selects one registry 0.1.6 package. This is metadata
  preparation, separate from subsequent hosted qualification. Tagged 0.13.5's
  root lock selects compatible registry 0.1.7 and its testing lock selects 0.1.6;
  both satisfy the tagged 0.1.6 requirement. Independent graphs need not select
  identical compatible packages. The current manifest requires 0.2; a later
  concurrent root lock change selects 0.2.1, while testing retains 0.2.0.
  The original 0.2.0 adoption lies outside 0.13.5 qualification and its matching
  0.14.0 hosted qualification passed as recorded above. The concurrent 0.2.1
  lock selection has no qualification in this adoption. The requested
  prevention feedback is filed as
  [Shared Tooling #6](https://github.com/dragginzgame/shared-tooling/issues/6).
  No upstream filesystem change or contributor release effect was performed.
  Both macOS gates subsequently passed; the authorized closure of
  [#10](https://github.com/dragginzgame/ic-timers/issues/10#issuecomment-6014163192)
  records the exact source and fixture/PocketIC evidence limits.
- The requested dependency-preparation guidance is adopted through the explicit
  revision-bound rule in [AGENTS.md](../../AGENTS.md) from Shared Tooling
  `a7efade1a68e43f148252a1a73908a46c4cbe9e9`. The full snapshot remains reviewed
  `cb86188` (21 files), with the adoption guide transferred into the separate
  original 17-file audit slice at `a37771f`, subsequently extended to 22 files
  for the structured checker/parser adoption. The audit adoption preserves baseline
  executables and product code; the subsequent metrics adapter/dependency change
  has its separate evidence owner above.
  That batch added only checker/host-parser setup. The current draft adopts the
  isolated Cargo/IC helpers above; newer logger and host-installer changes remain
  unadopted. Consumer qualification of the new slice remains pending.
  Scope belongs in the
  [adoption record](../shared-tooling.md).
  Its repository-only audit-method adoption and additional snapshot verification
  shipped in the same 0.14.0 batch, selected for the public measurement
  type's ic-metrics 0.2 identity cut. The maintainer performed version mutation
  and release execution.
- The 0.13.5 Apple Silicon cohort review records 263,433 baseline Wasm bytes and
  316,941/317,563/318,293 bytes for Once/AfterCompletion/Watchdog. Every previously
  emitted instruction/cycle subject matches the 0.13.3 row. Baseline already
  includes runtime inventory/snapshot and shared probe code; these are cohort
  comparisons, not total library size. No further runtime change or performance
  release is supported. Details belong in the
  [measurement owner](../design/callback-delivery-ownership.md#released-0135-cohort-review).
  Intel records the same instruction fields and baseline-relative byte costs,
  with different absolute byte sizes and dispatch cycles retained separately.
- Apply the pinned [Shared Tooling baseline and local overlay](../../AGENTS.md).
  Its [adoption record](../shared-tooling.md) scopes provenance and exceptions.
  Historical release notes retain evidence; no new versioned note or mutable
  handoff release marker is required for version preparation.
  During the 0.13.0 adoption, shared measurement arithmetic selected published
  registry `ic-metrics 0.1.3`; attribution and registration identity remained local.
  The root dependency and both lockfiles selected one registry package, preserving
  every other lock record and consumer package version. Locked offline metadata and manifest
  sorting pass for both workspaces. Warning-denied library Clippy, four measurement
  tests and focused registration/reset identity, stale delivery, discarded delivery,
  binding-failure and normal-completion checks pass on Linux. The initial Clippy
  gate reported three guard diagnostics; the complete guard is now captured until
  normal completion, and fallible borrowing uses `let ... else`. A completion
  fixture initially aborted at native thread-local teardown with a timer left
  armed; explicit unregistration corrects fixture cleanup. Two earlier filters
  matched no tests; exact names supplied the recorded evidence. This is focused
  native-substitute evidence, with no broad suite or PocketIC qualification.
  [Registry adoption](https://github.com/dragginzgame/ic-timers/issues/9) was
  committed by the maintainer at `685b4ff` during these checks, with the compatible
  `0.1.3` requirement and both locks still selecting the verified package. That
  adoption is included in the tagged 0.13.0 source; the focused evidence remains
  separate from broad native, release and CI qualification.
  The maintainer's subsequent test-target lint reported an empty-slice assertion,
  missing unit-expression semicolon and redundant identity clone in delivery
  fixtures. Those are corrected without changing fixture semantics; source
  formatting and diff checks pass, while the lint rerun remains user-owned.
  The subsequent broad native run aborted at TLS destruction in a liveness
  fixture with its replacement timer still armed. All runtime setup fixtures now
  retain scoped cleanup for queued and suspended fake tasks before TLS teardown,
  dropping outside the task-map borrow while runtime TLS remains available.
  A focused cleanup fixture is added; production behavior is unchanged. Native
  execution remains user-owned and pending, with scope recorded in the
  [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
  Standard SemVer releases use the refreshed shared runner: preflight/validation
  failures restart on current source, while normal targets automatically
  reconcile prepared intent at its saved version before a new increment. The
  local retry wrapper and its duplicate fixtures are removed; no release was
  executed during adoption. The shared hook formats only fully staged selected
  files, and the local formatting/gate owners cover both workspaces with pinned
  cargo-sort 2.1.4. Local hook activation remains separate from snapshot adoption.
  The 0.13.0 minor boundary covers the public Abandoned variant and ordinary
  delivery-retirement semantic cut; consumers adopting it must handle both.
  Snapshot integrity, shell syntax and metadata-preserving manifest sorting
  passed during snapshot adoption; both lockfiles were then byte-identical. The GitHub description matches
  current scope. New recovery/hook fixture execution remains user-owned; source/snapshot scope
  belongs in the [release guide](../releasing.md).
  The reviewed baseline is now `cb86188c5956866564de4fb6ec6be67b27981ab9`, with
  shared rules in `DRAGGINZGAME.md`. Maintainer-owned validation and release
  exceptions remain explicit; the [local host matrix](../releasing.md#host-support)
  records required macOS workflows and their unresolved qualification.
  Repository and release-gate fixtures now compare ordered records directly,
  removing their `mapfile` dependency; scoped verification remains in that matrix.
  Preservation fixtures retain file copies and compare bytes and permission bits,
  without checksum manifests, separate mode tables or GNU in-place sed. Preparation
  phase/staging comparisons use direct records and retain Git failure propagation.
  Clean-worktree and release-commit guards reject failed Git queries; their new
  rejection fixtures remain unexecuted. PocketIC fixture events use exact record
  comparisons, with scoped source/syntax review recorded in the host matrix.
  Version preparation and release commits now capture exact tag listings before
  checking absence, rejecting failed lookups before mutation and after a release
  commit. Fixtures cover empty/matching failure output and retrying the same
  untagged commit; source/syntax review is recorded in the release guide, and
  those scenarios remain unexecuted.
  Repository checks also reject failed workflow discovery and provider-source
  searches/orderings before accepting records. The existing repository fixture
  covers producer failures and discovery cleanup; its pending execution and
  syntax/source-review scope are recorded in the release guide.
  The user-operated release gate now prepares both locked dependency caches
  before validation, after the reported offline `js-sys` metadata failure during
  version preparation. Metadata parsing preserves Cargo's original failure.
  Fetch-order and metadata-failure fixtures remain unexecuted; source/syntax
  scope and the failed attempt are recorded in the release guide.
  Hosted tag CI now validates the tagged checkout's actual root/testing lockfiles
  using those existing fetch and metadata owners after tag/main checks and before
  fixtures. This repository-only continuation is in the current changelog section.
  Workflow syntax/source-review scope and pending hosted execution are recorded
  in the release guide.
  Release-impact classification now reads NUL-delimited Git paths after both
  queries succeed, preserving crate classification for display-quoted filenames
  without joining or sorting. Filename, partial-failure and cleanup fixtures
  remain unexecuted; their source/syntax scope is recorded in the release guide.
  PocketIC provisioning now verifies pinned archives before decompression and
  retains exact host-specific binary/version checks for Linux x86_64 and macOS
  Intel/Apple Silicon. macOS 15 PR/main jobs run the complete release gate under
  Apple's Bash 3.2 with both pinned Rust toolchains. Artifact-pin provenance and
  the declared host matrix belong in the release guide; fixtures and native
  qualification remain unexecuted. Empty-argument fixtures use positional
  arguments to avoid Bash 3.2 nounset behavior. No runtime API, Cargo version,
  lockfile or release execution changed.
  Combined release commands now reuse the commit guard's read-only worktree
  admission before validation and bumping. Only the five bump-owned metadata
  outputs may remain unstaged; other paths are reported without auto-staging.
  Standalone version preparation keeps its dirty-worktree contract. Source and
  syntax scope and pending fixture execution belong in the release guide.
  The bump helper now rejects missing or extra arguments and misplaced check
  flags before reading metadata. PocketIC requires regular executable candidates
  and rejects invalid provisioning destinations before downloading, preserving
  verified file symlinks as read-only input. Updated rejection/preservation
  fixtures remain unexecuted; source/syntax scope belongs in the release guide.
- The current ordinary callback API is policy-specific: Once and
  AfterCompletion entry points require separate result/decision types.
  Shared public results/directives are removed; the private erasure and canonical
  arbitration remain. Current probes and fixtures use the typed API, and both
  test/MSRV owners include API doctests. The implementation and validation
  scope belong in the [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-callback-results).
  Release identity and Git release execution remain maintainer-owned. The return-type cut
  uses a minor release with coordinated downstream adoption, without shims.
  Borrow-rejection fixtures now cover both ordinary policies/lifetimes, and
  public recurrence fixtures cover each completion classification. The README
  describes requested recurrence after normal return. Automated review did not
  execute these fixtures; their evidence scope belongs to the same callback
  contract.
- Public removals or incompatible semantic changes require the next minor line;
  private behavior-preserving simplifications may use a patch. See the
  [ordinary arbitration hard cut](../changelog/0.9.0.md).
- Version mutation, staging, commits, tags, pushes, publication, release commands,
  tests and build/lint gates are user-owned. Automated contributors implement
  requested changes and prepare changelogs/notes without executing those gates.
- Direct provider: exact `ic-cdk-timers` 1.0.0; exact `ic0` 1.2.0. Probe canisters
  use exact `ic-cdk` 0.20.3. MSRV is Rust 1.88.0; development/hosted CI uses 1.99.0.
- The earlier host-only harness adoption used exact `ic-testkit` 0.17.3 and
  one PocketIC 16.0.0 client, with caller-owned servers and bounded startup
  from the verified binary. Current root selection is described above.
  Official Linux/macOS archive hashes and executable headers were inspected
  without executing binaries; compilation, lint, host recovery and cohort
  qualification remain pending in the [release guide](../releasing.md#testkit-harness-qualification).
- The isolated version-preparation fixture now includes the shared increment
  helper and tests current Makefile delegation instead of the removed standard
  recipe sequence. The shared runner fixture is included in `release-check`;
  corrected fixture execution remains maintainer-owned, with scope recorded in
  the [release guide](../releasing.md).
  The maintainer's later gate passed the preceding release fixtures but failed
  the missing-changelog rollback assertion because its lock-update injector had
  already been restored. The injector now remains active through that case;
  the case verifies the injected failure, rollback and metadata preservation.
  Source/syntax checks pass; corrected fixture execution remains pending. The
  production bump helper and workspace versions are unchanged by this repair.

## Canonical runtime

- The 0.13 runtime implements retirement of confirmed ordinary deliveries dropped
  before normal completion. A queued-token guard is created before the first poll;
  exact ownership checks exclude stale, cancelled, replaced and unconfirmed work.
  Retained state becomes Abandoned/Failed with Unacknowledged accounting; transients
  and pending unregister are removed, without provider calls or fabricated completion.
  Normal live-await control and Watchdog prearming remain separate. Native drop and
  PocketIC pre-await/continuation trap fixtures are unexecuted; cleanup qualification
  and cost deltas remain pending with the [callback contract](../design/0.5-policy-specific-callback-authority.md#ordinary-delivery-abandonment).
  The finalized 0.13.0 section records the minor semantic cut; finalized 0.13.1
  records the shared instruction reader and documentation. Package versions and
  lockfiles remain maintainer-owned.

- One volatile canister-local registry owns at most 64 structured identities,
  declaration claims, callback generations, policy states, pending commands,
  callbacks, observations and provider handles. Entries own matching control,
  callback and cadence in one typed payload; policy is derived.
- Runtime initialization creates that registry and samples its epoch only when
  the slot is empty. Repeated initialization returns the existing epoch; borrow
  conflicts remain typed errors. See the [initialization note](../changelog/0.10.17.md).
- Once and AfterCompletion accept async work. Watchdog accepts one synchronous
  bounded unit after its scheduler commits a cadence successor.
- Only private `platform` calls the provider and IC system facts. Rust visibility
  and repository checks enforce provider confinement and restricted declarations.
- Snapshots and armed-wakeup observations are inert. Contexts expire with their
  exact work token; retained claims own longer-lived control. Claims and generations
  do not wrap. Consumer durable authority reconstructs volatile retained declarations
  synchronously before downstream hooks; shared-registry adoption is atomic.
  Policy-specific contexts store their token directly and use the shared
  claim-transition validation boundary before applying control; see the
  [control simplification note](../changelog/0.11.3.md).
- Ordinary inactive state owns its reason; running state owns its pending command.
  Watchdog inactive state owns its reason; awaiting-work state owns its pending
  command through dispatch and execution. Leaving either running or awaiting-work
  state discards its command. Public snapshots project these states without
  mutation authority.
- Each control owns one callback-generation allocation history; active states
  do not repeat it. Watchdog dispatch stamps successor and work with one fresh
  generation, distinguished by role and separate provider slots. Requests while
  awaiting work retain the pair until completion or recovery expires that attempt.
  Awaiting-work snapshots project the shared generation and direct attempt status
  without an attempt wrapper; tokens and handles retain independent
  delivery stamps. The 0.11.0 contract and evidence scope are recorded in its
  [release note](../changelog/0.11.0.md).
- Private callback tokens carry their registration claim. Context control and
  callback cleanup borrow it without claim reconstruction; running-work and
  provider ownership checks remain distinct. See the
  [callback claim ownership note](../changelog/0.10.14.md).
  Provider installation and effect confirmation use the canonical mutable claim
  lookup with callback-specific stale-error translation; see the
  [claim lookup note](../changelog/0.10.21.md).
  Late measurements use that lookup while retaining no-op behavior for missing or
  superseded claims; see the
  [lookup and detachment note](../changelog/0.10.22.md). Watchdog failure cleanup
  obtains paired handles through the entry's existing detachment owner, with
  provider clearing after the registry borrow is released. Ordinary terminal
  request failures stop their validated control directly in the request owner.
  The shared claim-transition operation validates context authority before
  detachment; lifecycle reconciliation directly verifies retained declarations.
  See the [runtime validation ownership note](../changelog/0.10.20.md).
  Ordinary reconciliation owns claim-policy validation directly, before
  cancellation or schedule resolution; see the
  [validation and scheduler failure note](../changelog/0.10.23.md). Watchdog
  scheduler allocation and deadline errors share terminal finalization while
  retaining generation-first validation and checking the deadline before allocation.
- Registry transitions borrow identity from their claim or token for local
  lookup and removal. Queued effects, snapshots and detached capabilities retain
  owned identities; see the [identity borrowing note](../changelog/0.10.18.md).
- Classified completion counters own completed-work accounting. The public total
  projects their saturating sum; starts and unacknowledged attempts remain
  separate events. See the [completion counter note](../changelog/0.10.15.md).
  Latest outcome and work count project from one recorded event; historical
  timestamps and the failure streak remain independent. See the
  [outcome ownership note](../changelog/0.10.20.md).
- Registry arbitration owns ordinary pending-command order and exact running-work
  authorization. Exact reconciliation replaces a discarded callback scheduling
  proposal before validation. Invariant failure remains terminal; unregister is
  sticky; ensure selects earliest demand. The effective exact directive is observed.
  Pending scheduling commands select precedence in one match and retain the
  existing schedule metadata at equal deadlines; see the
  [ordinary command precedence note](../changelog/0.10.19.md).
- Ordinary requests and authorized completion successors share checked arming;
  allocation is checked directly in that operation. Stopping selects its reason
  and allocates no generation. Cancellation applies lifetime removal once from
  the final policy state. See the [state ownership note](../changelog/0.11.2.md).
- Cancellation allocates no callback generation. Scheduled/dispatched cancellation
  selects inactive state and clears the existing handles; stale delivery is rejected
  by state, claim and role. Rearming and dispatch still require fresh non-wrapping
  generations.
  Running cancellation remains a pending command applied on normal completion;
  confirmed ordinary abandonment retires the delivery and discards that command
  under the 0.13 contract above.
  The 0.11.0 semantic hard cut and its verification scope are recorded in the
  [release note](../changelog/0.11.0.md).
- Ordinary directive resolution returns canonical control failures directly.
  Explicit scheduling requests retain schedule errors at their input boundary;
  the registry owns terminal state and completion accounting. Directive and
  generation failures share one completion finalization branch and removal exit,
  preserving validation before allocation. See the
  [completion failure ownership note](../changelog/0.11.3.md).
- Watchdog initial and replacement requests share one scheduling-mode update after
  successful arming. Coalesced, pending and failed requests preserve it; requested
  delays still record accepted demand, and cadence deadlines are checked only for
  inactive control. See the [request observation note](../changelog/0.11.3.md).
  Coalescing is recorded once for a no-effect transition without failure. Ordinary
  request mode updates have one owner for successful arms or exact reconciliation;
  coalesced ensures preserve the existing mode. See the
  [request accounting note](../changelog/0.11.4.md).
- Ordinary dispatch shares completion finalization for callback results and
  callback-borrow failures. Work measurements are recorded only after executed
  work; see the [ordinary callback finalization note](../changelog/0.10.9.md).
  Ordinary and Watchdog work acceptance returns the typed callback directly from
  the validated entry. Dispatch releases that borrow before consumer work and
  has no second callback lookup. Context and completion authorization remain
  independent; see the [callback acceptance note](../changelog/0.11.2.md).
  Work delivery consumes the matching fired handle through that same selected
  entry. Watchdog scheduler consumption and remaining-handle detachment share
  one lookup before the transition; see the
  [callback delivery evidence](../design/callback-delivery-ownership.md).
- Watchdog completion decides lifetime removal once from final inactive state
  and pending unregister, after selecting its successor or terminal transition;
  see the [Watchdog completion removal note](../changelog/0.10.10.md).
  Terminal request and scheduler failures decide lifetime removal from their
  already selected entry, without another lookup; see the
  [ownership and verification scope](../architecture.md#terminal-removal-verification).
- Registry and owned-handle bounds do not bound the provider heap. Cancelled future
  deadline records remain queued. Page extents do not establish allocator bounds.
- Public control failures retire false scheduled state. One detached-claim finalizer
  restores handles after registry errors and retires claims after unexpected
  restoration or provider failures. Unexpected Watchdog work completion failures
  trap for IC rollback. Effect confirmation uses one validated wakeup-generation
  marker. These distinct failure rules must remain separate.
- Effect application binds wakeups and Watchdog dispatch in its validated match
  branches; confirmation retains its independent state and generation checks.
  Successor tokens share claim-based construction after authorization. Watchdog
  cancellation selects cleanup and immediate accounting together from its active
  state in the registry command owner; see the
  [cancellation decision note](../changelog/0.11.1.md).
- Platform page reads and callback measurements share inert `MemoryPageExtent`
  values. The registry pairs start/end extents without a second representation.

## Unresolved scope

Downstream work is deferred at the maintainer's request. Historical adoption
records remain scoped to their recorded subjects; do not treat them as current
composed qualification. Ordinary and Watchdog command machines, effect confirmation
and handle-restoration stages retain their distinct suspension and recovery roles.
The native dispatch-failure fixture and its verification scope are documented in
the [0.10.11 note](../changelog/0.10.11.md). Native injection does not simulate IC
rollback; deployment validation remains maintainer-owned.

Generation ownership, provider binding, workspace formatting and audit follow-up
are recorded in the [0.10.12 note](../changelog/0.10.12.md).

## Evidence

The complete normal-gate record includes 0.14.17 at `031e6c67`, as
recorded in the host owner; 0.14.20 still needs complete native acceptance.
The following earlier inspections retain their historical source scope.

At the earlier inspection, release 0.14.2 at `88aedf0` had tag CI and main Linux/MSRV passes,
while both complete macOS gates were queued. Exact source and scope are recorded
in the [adoption owner](../shared-tooling.md#formatter-prerequisite-adoption).
The 0.14.1 missing-`rg` failure remains historical evidence; it does not describe
the successful 0.14.2 Linux rerun or qualify later formatter changes.
The then-latest complete all-host qualification was 0.14.0 at `902323a`: tag CI, main
Linux/MSRV and both native macOS gates passed. Apple Silicon
records 142 native tests, doctests, 14 PocketIC subjects and cohorts; its six
measurement rows match 0.13.5 exactly, including Wasm bytes. Exact source and
comparison scope belong in the [measurement owner](../design/callback-delivery-ownership.md#ic-metrics-02-adoption).
Earlier 0.13.5 at `98c4b29` passed tag CI, main Linux/MSRV and both
complete native macOS gates passed. The repaired metadata/index fixtures pass
on Linux and both macOS hosts. Each macOS gate records 142 native tests,
14 PocketIC runtime subjects, doctests and cohorts. The preceding
0.13.2 qualification at `134899f` retains its own 138 native/13 PocketIC scope.
Exact
links and qualification scope belong in the [host matrix](../releasing.md#host-support).
The maintainer authorized closing [#9](https://github.com/dragginzgame/ic-timers/issues/9)
with that integration/CI evidence; it is closed. The authorized closure of #10
now records complete 0.13.5 qualification. Open adoption obligations remain in
[GitHub](https://github.com/dragginzgame/ic-timers/issues); no open PR was listed.
The requested [#11](https://github.com/dragginzgame/ic-timers/issues/11) adoption
is released from committed `a37771f`: six
unchanged shared methods, a product-only hygiene overlay, preserved reports and
a separate verified audit manifest. The obligation map and representative
historical-report walk belong in the
[adoption owner](../shared-tooling.md#audit-method-adoption-review). This is
documentation adoption, not a fresh product audit; no local test/build/lint or
complete Make gate was executed. GitHub closure is not authorized by the repair
request alone. The audit slice is repository-only within released 0.14.0;
the metrics dependency makes the complete batch crate-impacting.
The earlier authorized ic-metrics comments on #1, #3 and #4
record IC Timers 0.13.3 consumer qualification; they do not qualify 0.13.4.
The committed Shared Tooling head and remote main matched adopted `cb86188`;
its upstream Linux regression and lint/security passed, but both macOS jobs
failed at snapshot-distribution fixture source-path admission after passing the
shared logger tests. The diagnosis and remaining qualification boundary belong
in the [adoption record](../shared-tooling.md). The preceding `9437bab` Linux and
both macOS upstream gates passed, qualifying that recovery source alone.
Later sibling changes were initially dirty and excluded; after they were
committed at `cb86188`, a separate clean reviewed export adopted them.
No tests were executed locally during this continuation. Shared Tooling committed
its newer batch at `a7efade`; upstream Linux regression and lint/security passed,
while both macOS jobs remained pending at inspection. Only its
reviewed dependency-preparation section is adopted through an explicit reference;
the baseline executables and runtime are unchanged. The later clean `a37771f`
export supplies the audit snapshot only, with linked setup/pinning material
scoped as reference rather than full policy/tool adoption. Its upstream Linux
regression, lint/security and both native macOS jobs passed. Both consumer
snapshots, 154 local links/anchors and diff checks pass as adoption evidence.
Those documentation/Make and dependency changes are now included in released
`902323a`; its matching hosted run remains incomplete at inspection. See the
[release runner evidence owner](../releasing.md#standard-release-runner).

Earlier inspection records follow; their pending/failure language describes
those earlier sources and times, not the current released baseline.


On 2026-10-06 the committed Shared Tooling head and remote main both remained
`f52c0e2476aee094359ed21de91c468540d3969f`; the 20-file snapshot verified. The
dirty upstream maintenance rules remain outside the reviewed snapshot. Both
0.13.1 macOS jobs failed the release fixture's logical-versus-physical default
cache comparison, while Linux checks passed. MSRV and tag jobs did not acquire
hosted runners. The 0.13.2 fixture repair retains all gate and override assertions;
its source/syntax scope and pending native rerun are recorded in the
[host matrix](../releasing.md#host-support). No tests, builds, lint gates, version
mutation or release execution were performed during this repair.

The extraction workflow also ran the previously authorized focused qualification
for the shared-reader wiring: native and Wasm library Clippy with warnings denied,
Rust 1.88 Wasm compilation, and four named native tests for role-specific
measurement saturation, completion totals, registration identity and abandoned
work. They passed while Cargo still identified the package as 0.13.0. The
maintainer subsequently committed that exact platform source and selected
0.13.1; its release commit changed metadata only. Both locked workspace graphs
resolve one registry ic-metrics 0.1.5 with `ic`, preserving every external
selection except metrics. These checks use the consumer/test-owned native
substitute; they do not establish timer PocketIC or full release qualification.


A 2026-10-05 source review traced the 0.13.1 reader through the downloaded registry
source to `ic0::performance_counter(1)`, and reviewed ordinary delivery ownership
and scoped native fixture cleanup. This is source evidence, with no new test,
build, lint, PocketIC or cost measurements. GitHub's main and tag CI runs for
`54bbcfc4985d4657578150cbe7112795297115fd` were queued when inspected:
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535919) and
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37370535920).
At that earlier inspection, [issue #9](https://github.com/dragginzgame/ic-timers/issues/9)
was open for owning native release/CI qualification; the later 0.13.2 gates
supplied it and the maintainer authorized closure.
No open PR was listed at that earlier inspection. Shared Tooling's committed
head then remained the adopted revision; its maintenance-rule draft was outside
that earlier reviewed snapshot.

Inspected hosted validation for 0.9.4 is scoped in its
[delivery note](../changelog/0.9.4.md). Release reports and Git references alone do
not establish new hosted or PocketIC results. Each change's verification record
belongs with its design/evidence owner rather than a repeated handoff claim.
The native mock does not simulate IC rollback or provider heap allocation;
maintained PocketIC subjects remain required for those claims. Tests, builds,
lint gates and deployment validation remain user-owned.
Measurement ownership across identity reuse and policy-specific completion
failure assertions are scoped in the [0.10.16 note](../changelog/0.10.16.md).
Public lifecycle rejection fixtures cover identity, lifetime, cadence, expired
claims and occupied-identity reconstruction from an empty slot; their pending
verification is scoped in the [0.10.21 note](../changelog/0.10.21.md).

## Next action

The 0.16.4 main/tag/MSRV and both native gates pass; #34 is closed for completed
Testkit setup/admission/product acceptance. The pending 0.16.5 batch now adopts
committed Shared 0.2.11's concise formatter and hidden-Make-mode correction.
#35 stays open for actual consumer fixtures and the new source's Linux/native
acceptance. Do not reuse the released 0.2.8 consumer evidence for this selection.

Retain the one undated 0.16.5 draft and incoming Metrics/cc/smallvec/syn lock refresh.
Tests/builds/lint/setup, Cargo selection and release effects remain user-owned.
No timer feature or extra release is justified merely to fill this tooling batch.
