# Shared Tooling adoption

The compatible repository-only **0.14.14** draft prepares reviewed committed
Shared Tooling 0.1.23
[`0ba0ad00ed94848e54ecc82629b6b7873b7284c0`](https://github.com/dragginzgame/shared-tooling/tree/0ba0ad00ed94848e54ecc82629b6b7873b7284c0).
The [baseline snapshot](../.shared-tooling.snapshot),
[audit/setup snapshot](../.shared-tooling-audits.snapshot) and
[helper snapshot](../.shared-tooling/helpers/.shared-tooling.snapshot) select
35/30/13 exact files at that same revision. A clean detached scratch clone
supplies every export; moving sibling work is excluded. The
nested bundle is exported through its own isolated temporary consumer.
No sibling, staged path, Cargo version or incoming dependency lock was changed
by this refresh. Released 0.14.12 adopted the preceding 0.1.19 snapshot;
its historical evidence remains scoped below.

[AGENTS.md](../AGENTS.md) retains the product overlay and approved command
exceptions, including maintainer-owned contribution commits. Standard Make
release commands select direct delivery explicitly; the reviewed PR helper is
included without selecting that workflow. Timer runtime, the root catalog, library-only default builds,
private provider and audited PocketIC artifact remain. Optional Rust tooling
is still separate from aggregate setup; the disk-space helper is not adopted
without a consumer capacity requirement.

## Shared Tooling 0.1.23 preparation

The maintainer authorized continuation of the reviewed refresh. All three
manifests now bind 0.1.23; overlapping entries are refreshed together. The
baseline adds `rules/contributions.md` and `scripts/ci/release-pr.sh`; the
supplemental file set remains 30 and the nested helper set remains 13 with
unchanged helper payloads. This is a coherent worktree preparation, not an atomic
filesystem transaction or a released consumer qualification.

The canonical runner rechecks committed payload, independent index, worktree
and the exact tag object after the final hook. Completed direct resume verifies
local annotated-tag identity, the exact remote tag object and release ancestry
in the observed destination branch without replaying completed release effects.
Existing adapter/receipt checks and full native release gates remain selected.
The consumer's committed-release fixture additionally checks that ambient and
Make-supplied `RELEASE_DELIVERY=pr` cannot redirect any standard release or resume
command. It substitutes the runner and observes no Git/Cargo effects; it was not
executed during preparation. The helper does not add PR delivery or merge authority.

Source/export byte and executable-mode inspection, all three snapshot integrity
checks, shell syntax and diff checks are preparation evidence. No tests, builds,
lint, version bump, staging, commit, tag, push or publication ran. Upstream
0.1.23 Linux, lint/security and both native macOS gates now pass in
[the exact-source run](https://github.com/dragginzgame/shared-tooling/actions/runs/37746567888).
Consumer acceptance remains tracked by
[#29](https://github.com/dragginzgame/ic-timers/issues/29), separately from the
hosted failure-artifact observations in #23. No function, method or type was
removed by this refresh; timer Wasm/instruction changes from the tooling refresh
are not expected.

## Shared Tooling 0.1.19 consolidation

The committed Rust-install repair admits directories, executable leaves and
Cargo receipts before probes or installation, and rechecks after Cargo returns.
This prepares [#28](https://github.com/dragginzgame/ic-timers/issues/28) without
running an installation or adding a local guard. The refreshed validation owner
retains unique combined failed-batch logs and `latest-combined.log`; its existing
per-target logs remain. The failure collector already archives their owning
validation directory, so it needs no second concatenation path.

The local changelog wrapper delegates selection to the canonical AWK owner.
It still validates the requested/previous version boundary, supplies missing-file
presentation, rejects unsafe output types, admits successful reader/selector
status, preserves file modes and replaces output atomically. The pending body's
formatting follows the shared selector rather than a second local normalizer;
historical notes retain exact bytes. The version-preparation fixture explicitly
copies this new dependency. Failed-reader and failed-selector cases exercise
original-file preservation, alongside the released 0.14.11 EOF/history cases.
This prepares [#24](https://github.com/dragginzgame/ic-timers/issues/24).

The IC installer and consumer PocketIC provisioner share matrix admission through
`ic-tool-pins.awk`. The provisioner projects the version/archive digest from that
matrix, delegates checksum and executable admission to shared owners, and retains
its reviewed extracted-binary hashes, host selection, adjacent temporary download,
archive-before-decompression ordering and atomic single-artifact cache replacement.
The Make gate checks full locked offline client/server alignment before admission
or provisioning. Explicit overrides are never replaced. Its isolated fixture
proves that alignment failure prevents provisioning; the artifact fixture still
checks all three hosts' pins, ordering and cache preservation through controlled
hash/download substitutes. The shared PocketIC fixture is explicitly selected in
`release-check`. This prepares [#25](https://github.com/dragginzgame/ic-timers/issues/25).

Removed named functions are `sha256` and `verify_binary` from
[`check-pocketic.sh`](../scripts/ci/check-pocketic.sh), replaced by the canonical
checksum and PocketIC binary owners. The anonymous Perl draft-selection engine
is also removed; mode and scalar version-boundary checks remain in the wrapper.
No timer API or runtime source changes. Testkit 0.21.1 is an incoming compatible
lock selection and is preserved; ic-metrics remains 0.2.8 and the split Host
packages remain 0.4.2. Their unpublished sibling work is excluded.

Preparation is exact source/mode/snapshot, shell/embedded-Perl syntax, documentation,
diff and full locked offline metadata inspection only. No fixtures, tests, builds,
lint, formatter, installation or release commands were run. At preparation GitHub
main still identified 0.1.18 with no exact 0.1.19 run. Upstream main now includes
that committed source through 0.1.20 `3ecc48e`;
[its native CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37641211708)
now passes all three declared hosts. That supplies upstream qualification;
later governance changes are not adopted in this snapshot, and the consumer's
Intel evidence remains separate.
The maintainer released the consumer batch at `72e8f5d` (0.14.12).
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37639154601)
passed; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37639154602)
passed Linux/MSRV and Apple Silicon. Intel macOS failed without executing steps
and supplied no artifact or native qualification. The three adoption issues
remained open at that review; their subsequent acceptance is recorded below.
#23 remains open for controlled hosted-failure artifacts.
The 0.14.13 manual driver is documented with the
[existing evidence owner](releasing.md#ci-failure-evidence). Earlier evidence below
retains its original source scope.

The .13 follow-up reuses the download action pin from committed 0.1.20 for a
manual-only consumer verification matrix, while keeping the existing collector
and uploader. It verifies actual downloaded tar entries against known scenario
bytes, modes, status and exact original job/run/source identity. Upstream upload
proof cannot substitute for these six consumer observations. No shared snapshot
bytes or baseline authority change is needed for this local qualification repair.

The 2026-10-07 continuation reviewed committed Shared Tooling 0.1.21
`45e34e92b43edb9543d5b7212774f87f8334079f` and
[its pending native run](https://github.com/dragginzgame/shared-tooling/actions/runs/37652236506).
It adds explicit PR-gated delivery for protected release branches; the common
direct flow remains the default. IC Timers' main branch is unprotected and no
local issue requires that mode. Keep the current snapshot rather than copying
an unqualified merged-source release flow or selecting PR delivery implicitly.
An eventual reviewed refresh must reconcile contribution policy with the approved
local command exceptions and keep all three snapshot revisions aligned; refreshing
the runner alone does not adopt PR delivery. No new common-helper repair is needed
for #24/#25/#28; their subsequent Intel acceptance is recorded below.

## Consumer adoption qualification

Released 0.14.13 `0b12c8a6dbe5f359f5499df67313ffa5c02af446` now supplies the
missing [complete Intel native gate](https://github.com/dragginzgame/ic-timers/actions/runs/37655294064/job/112908747235).
Linux/MSRV and tag truth also pass at that source. Source comparison proves 82
adoption files unchanged from 0.14.12: all three snapshot manifests and payloads,
the Make gate, changelog wrapper/bump/preservation fixtures and PocketIC
provisioner/fixture. Apple Silicon's complete gate passed that unchanged adoption
at 0.14.12. #24/#25/#28 are closed with these source-bound native observations.

This does not relabel the earlier dependency graph: 0.14.12 selected Testkit
0.21.1, Host 0.4.2 and Metrics 0.2.8; 0.14.13 selects Testkit 0.21.2, Host 0.4.6
and Metrics 0.2.9. Both use PocketIC 16.0.0 and the unchanged timer runtime.
Apple Silicon's own 0.14.13 gate subsequently passed, so the same
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37655294064)
now qualifies the complete released dependency graph on both native hosts.
#23's manual artifact verification is also separate and has not run at this
source. No contributor test, build, lint, rerun, dispatch or release executed.

Committed Shared Tooling 0.1.22
`2687f26317952c43c685f7f799ed09288dc10a67` follows the cancelled 0.1.21 run.
[Its Linux and both native macOS jobs pass](https://github.com/dragginzgame/shared-tooling/actions/runs/37659875012).
Its #57 correction affects `test-cloc-siblings.sh`
and its context fixture, neither of which this consumer selects. The adopted
`test-cloc-tooling.sh` is unchanged. No snapshot refresh or new release-mode
adoption is needed to repair a local issue. Keep the 0.1.19 reviewed baseline
and approved command exceptions.

## Shared Tooling 0.1.23 source review

Review on 2026-10-08 compared the selected 0.1.19 payloads with clean committed
upstream 0.1.23 `0ba0ad00ed94848e54ecc82629b6b7873b7284c0`. Ten of the 33
baseline payloads and one supplemental guide differ; all 13 nested-helper
payloads are unchanged and no selected file disappeared. This is a source
comparison, not adoption or execution of the new consumer graph.

The [release-integrity repair](https://github.com/dragginzgame/shared-tooling/issues/58)
applies to the selected direct runner. It now rechecks the committed payload,
independent index, worktree and exact tag object after the final consumer hook.
Completed direct resume also verifies the local annotated tag, exact remote tag
object and release ancestry in the observed destination branch before reporting
success. The current consumer hooks inspect immutable release metadata and tag
identity; no incorrect live release was observed. Those hooks do not replace the
missing completed-resume checks. Consumer adoption and its required proof are
tracked by [#29](https://github.com/dragginzgame/ic-timers/issues/29).

At review, [the exact-source upstream run](https://github.com/dragginzgame/shared-tooling/actions/runs/37746567888)
passes Linux portable regression and lint/security; both native macOS jobs are
running. The proposed refresh keeps all three snapshot identities aligned and
includes the newly required contribution rule and release helper, with explicit
maintainer-owned command exceptions. Direct delivery remains the selected policy;
including the helper does not select PR delivery. No snapshot file, Cargo version
or release effect was changed during this inspection.

That paragraph records the initial inspection, before the separately authorized
[preparation](#shared-tooling-0123-preparation) above. It does not describe the
subsequently refreshed worktree or qualify its new bytes.

The current `test-git-hook.sh` already exercises the actual consumer formatter
and checks real Rust output, partial staging, unrelated edits and failure
isolation. The new frontend reference supplies no reason to introduce npm or
duplicate that Rust qualification. Recent sibling LOC repairs affect unselected
scripts. Neither improvement is added to the consumer refresh scope. All tests,
builds, lint and native consumer qualification remain maintainer-owned.

## Shared Tooling 0.1.18 refresh

The logger admits the entire target list before dispatch: options, assignments
and control characters are rejected. Optional `VALIDATION_LOG_DIR` retains
complete successful/failed logs and timings in unique invocation directories;
ordinary release failure retention still uses the Git-owned directory. Existing
release callers and fixture owners remain unchanged.

LOC fixtures now resolve their own manifests/target directories, even with
scratch inside an enclosing workspace; tooling fixtures qualify adopted
working-tree exports without requiring a consumer commit or distribution helper.
Reports handle configured build-output aliases and snapshot root/provenance
selection. The root report already covers all four maintained packages after
0.14.9; no independent testing manifest is recreated for a LOC report.

The audit catalog now links the canister application addendum. The local
[hygiene overlay](audits/code-hygiene.md) scopes it to probe/service and Wasm
questions without claiming a new audit, recovery proof or performance result.

The shared Make include now declares optional Rust-tool commands, so the setup
snapshot includes their installer and fixture dependencies. Existing explicit
cargo-sort preparation and host/IC aggregate setup are retained. Its fixture is
explicitly selected in the maintainer-operated release checks to
qualify these commands with substitute Cargo, without real tool compilation.
The Rust set has not been installed or added to aggregate setup; the upstream
redirected-path
repair in [Shared Tooling #54](https://github.com/dragginzgame/shared-tooling/issues/54)
is still unpublished. Its adoption requires reviewed committed source and the
consumer's normal native qualification, not a patched installer or local guard.

[Upstream 0.1.18 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37604299590)
passed Linux portable/lint and both native macOS gates. Consumer preparation
checks hashes/modes, syntax, links, metadata and diffs only; no tests, builds,
lint, formatter, installation or release commands ran. Consumer qualification
remains separate and user-owned. The maintainer subsequently released this
batch at `479c4b8c8b6412babf7c98ea17c948eacdaeadc3` (0.14.10).
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37617070319)
passed; [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37617070245)
passed Linux/MSRV with both native macOS jobs running at inspection. Those results
do not qualify the subsequently edited dependency lock or future shared repairs.

At the 0.1.18 refresh, the private changelog selector was deliberately retained. Source inspection found
remaining dated-heading and historical EOF preservation gaps in the shared
helper, reported in [Shared Tooling #55](https://github.com/dragginzgame/shared-tooling/issues/55).
[IC Timers #24](https://github.com/dragginzgame/ic-timers/issues/24) stays open;
the trailing-heading-whitespace fix is accepted, but replacing the engine now
would weaken maintained refusal/preservation behavior. This refresh alone does
not close the selector, PocketIC pin or hosted-failure artifact obligations.

Released repository-only 0.14.11 prepared the consumer-side history
contract with byte-exact comparisons in
[`test-finalize-changelog.sh`](../scripts/release/test-finalize-changelog.sh).
Dated and imported undated historical sections retain their final bytes with
no terminal newline, one newline or trailing blank/whitespace lines. Cases
with and without pending notes check both read-only admission and full output;
no history extraction normalizes away the EOF defect. Bash syntax and diff
inspection are preparation evidence only; fixtures were not executed. The
canonical cut still waits for committed, reviewed Shared Tooling source.

## Root dependency catalog consolidation

On 2026-10-07 the maintainer requested every dependency in the main Cargo
catalog, superseding the prior two-workspace exception. All four maintained
packages now inherit root dependency selections and share one lockfile.
The root default member and explicit library commands keep host Testkit outside
canister builds; probe optimization settings move to the root `timer-probe`
profile. Release mutation/staging, metadata/index/fetch and hook fixtures now
select the sole graph. No shared snapshot payload is patched by this change.

The pre-refresh inspection found committed Shared Tooling 0.1.18 at
`a3430b34b32a60f3b245a2b4f7e2f5321556fe56`, matching remote main at inspection.
Its LOC fixture and logger argument corrections address the reviewed upstream
issues; the standard dependency-inheritance rules are unchanged. This task
inspected the new source without adopting it. The three local snapshots remain
at 0.1.15 at that inspection. Exact-source [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37604299590)
passed Linux, lint/security and both native macOS jobs. The subsequent adoption
is scoped above. Canonical selector cleanup remains in
[#24](https://github.com/dragginzgame/ic-timers/issues/24).

Current preparation evidence is complete locked offline Cargo metadata and
source/syntax/inheritance inspection. Tests, builds, lint, formatting and the
complete native/PocketIC release gate remain maintainer-owned and unexecuted
for this worktree. No runtime API, provider pin or measurement guarantee changed.

## Shared Tooling 0.1.15 common tool commands

Prepared undated 0.14.8 adopts clean detached exports of committed
`bfb50bd0884b5e6c5ee9592056531c6108f96d73`, with 33/23/13 files in the baseline,
audit/setup and helper snapshots. Exact shared bytes and executable modes are
retained. No sibling files change and no snapshot payload is patched.

The root Makefile includes [make/tools.mk](../make/tools.mk) once. It replaces
six copied setup/check recipes and supplies the checkout-local PATH, `cloc` and
`cloc-tooling`. Both setup and offline checks select jq 1.8.2, yq 4.47.2,
ripgrep 15.2.0 with PCRE2, and cloc 2.10 using the reviewed
[pin catalog](../ci/tool-versions.env). Only cloc is a new pin. IC pins and the
separate exact audited PocketIC admission remain unchanged. Existing two-parser
bundles need explicit `make install-host-tools`; ordinary checks remain offline
and fail closed. No tool installation ran during preparation.

`actions-check` depends on the common `host-tools-check`; `update-dev` and all
hosted jobs select `install-tools`/`tools-check`. Duplicate apt/Homebrew ripgrep
installation is removed. Standard-release checking exports the common include,
as do isolated repository, hook, exact-release, committed-release and gate
fixtures. Hook copyback uses an index containing the actual include. The release
gate selects unchanged upstream common-command, host/IC and LOC fixtures; these
are substitute/native-tool qualification, not release or IC recovery execution.

The common LOC report covers the publishable root Cargo workspace. The separate
`testing/` workspace is not included in its counts; passing `CLOC_ROOT=testing`
still resolves the containing Git root in the shared reporter.
[Shared Tooling #41](https://github.com/dragginzgame/shared-tooling/issues/41) owns
explicit independent-workspace selection; this is not a release blocker. No composed
runtime/test total is claimed. The sibling tooling inventory reads working files
and snapshot hashes/modes without executing consumer scripts, Make or Cargo.
Shared/local counts are discovery evidence, not justification to remove distinct
product owners.

The maintainer's local validation of committed `.8` preparation at `fb86e29`
first stopped on the old parser-only host bundle. Explicit pinned host setup
then completed, preserving the prior bundle. The retry passed host verification
and dependency admission but failed `shell-check`: the local Makefile still
selected the now-removed nested `scripts/dev/*.sh` glob. The follow-up removes
that obsolete inventory entry. The repository fixture keeps that directory
absent and still injects syntax failures into every current script directory,
so fixture-only files cannot mask the retired path again. Script syntax and
inventory inspection are preparation evidence; the full gate and updated
fixture remain maintainer-owned. Original failure logs are retained.

The next maintainer attempt at `f2bdaaf` passed the selected snapshot, shared
tool/LOC, collector-retention, metadata/index and lockfile fixtures, then stopped
in `test-release-gate.sh`. Retained scratch `/tmp/tmp.y2Sr9sdTBI` contains the
actual Makefile/include and leaf overrides, but no host installer or recorded
leaf events. The override of `actions-check` kept its new `host-tools-check`
prerequisite, causing real setup verification inside the incomplete fixture.
The follow-up records/substitutes that prerequisite and puts it before
`actions-check` in the independent expected order. Existing failure-prefix cases
therefore include host-admission failure for CI and complete-release dispatch.
Passing and rejected per-gate Make output is now retained instead of discarded.
Source/syntax inspection is preparation only; the updated gate fixture has not
been rerun by the contributor. This does not change production admission or
qualify the complete `.8` release gate.

At inspection, exact-source
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37593142226)
passed Linux, lint/security and both native macOS jobs.
The continued .8 batch retains failed local committed-release, index/staging,
lockfile and repository-check scratch for #23's existing collector, rather than
removing diagnostic inputs unconditionally. EXIT handlers preserve original
status, print retained paths and still remove successful scratch. The collector
fixture now injects early failures through those actual scripts and compares
retained bytes after archive/extraction. No builds or real Git writes occur on
those substituted paths. These new cases have not run during preparation.

Consumer qualification for this new source remains pending. Preparation performs
integrity, source/mode comparison, shell/Perl/YAML syntax, reference and diff
inspection only. No tests, builds, lint, formatter or release execution ran.
Runtime/probe source, manifests and both lockfiles are preserved. This is
repository-only work with no measured Wasm, instruction or heap delta.

[#24](https://github.com/dragginzgame/ic-timers/issues/24) remains blocked by
[Shared Tooling #38](https://github.com/dragginzgame/shared-tooling/issues/38).
The committed shared selector still misses draft headings with trailing spaces
or tabs. Keep the existing consumer finalizer until a reviewed committed owner
fix supports consolidation; its preservation/whitespace cases are already
maintained. Do not patch the shared AWK or add another selector.

## Shared Tooling 0.1.14 Make admission

The maintainer authorized [#20](https://github.com/dragginzgame/ic-timers/issues/20)
after released 0.14.6. Clean detached exports of committed
`25e7ce83149e081e4dcc52c55c33724e44153f2a` refresh the baseline, audit/setup and
helper manifests together: 27/19/17 files. Shared source bytes and executable
modes are retained; dirty sibling work is excluded. Exact-source
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37586649650)
passed Linux, lint/security and both native macOS jobs.

The baseline explicitly adds the canonical
[Make execution guard](../scripts/ci/check-make-execution.sh). The release runner,
validation logger and selected-index formatting hook invoke it before their
protected effects. It exercises an isolated recipe's execution and failure
propagation through actual GNU Make, clearing only that probe's `MAKEFILES`;
it does not parse version-dependent flag spelling or change caller variables.
Ignore-errors, dry-run, question, touch and version-only modes cannot qualify
those entrypoints. Ordinary release selections and jobserver controls remain
inherited. The logger copies its guard into its temporary source snapshot.

The local `release-x` recipe uses the same guard before preflight or mutation.
Its complete sequence runs in one recursive shell recipe with `set -e`, so a
failed guard cannot fall through to later phases when outer Make ignores errors.
The recursive recipe is entered under dry-run, question and touch modes solely
to perform admission. Outer `make -i` can still report success after ignoring
that shell's error; rejection prevents effects, not GNU Make's own exit policy.
Version-only outer Make executes no recipe. Exact-version release ordering and
its existing manual interruption recovery remain unchanged.

Consumer logger fixtures export the guard beside both root and child loggers;
the restricted PATH supplies real Make plus external `printf` and `echo`.
Hook fixtures include the guard in their isolated index. Negative cases cover
compact and long modes inherited through `MAKEFLAGS` and, on Make 4+, `GNUMAKEFLAGS`, checking
no leaf release phases, callback-target dispatch, formatting or index refresh.
Normal nested logger coverage retains release identity and exercises `-j2`.
GNU Make 3.81 on macOS ignores `GNUMAKEFLAGS`; that variable was introduced in
[GNU Make 4.0](https://sourceware.org/pipermail/cygwin-announce/2013-October/005192.html).
Those additional cases are selected only for versions that support it.
Shared runner fixtures retain their canonical real-Make admission and substituted
release effects. No parallel local flag parser or patched snapshot is introduced.

The local exact-release fixture also injects bump, stage, commit and push failures
after validation. It compares the executed phase prefix and requires failure
propagation, with recording leaves that perform no real release effects.
Version-preparation and hook fixtures retain failed scratch directories and
print their paths, while successful runs remove them. Each Make-mode scenario
has its own log; exact-release phase cases retain their Make output, and normal
hook formatting/check failures print and retain theirs. This supplies local
failure diagnostics, not durable hosted artifact storage. These added cases
have not been executed during contributor preparation.

The workspace rule now also permits application-owned packages under `apps/`;
our existing independent `crates/` roots need no further move. IC Host Tooling
package attribution is corrected in shared guidance without changing tool pins.
The upstream sibling-cloc changes are outside our adopted helper set and add no
local command. Product validation, both workspace graphs and the exact audited
PocketIC gate remain unchanged.

Compatible draft 0.14.7 contains this repository-only batch. Preparation uses
snapshot integrity, exact export/mode comparison, shell syntax, documentation
and diff inspection. Runtime, probe source, Cargo manifests and both lockfiles
are preserved. Tests, builds, lint, formatter and release execution remain
maintainer-owned and have not run for this draft. #20 remains open until complete
consumer qualification, including native macOS, binds to the prepared source.
No Wasm, instruction or heap delta is measured or claimed.

The maintainer subsequently authorized the consumer-owned
[#23 CI evidence collection](https://github.com/dragginzgame/ic-timers/issues/23).
Final failure-only steps in each job archive job-owned scratch, raw validation
failures and rejected installer candidates through a small local collector.
The reviewed official uploader 7.0.1 is pinned in the workflow; tar preserves
modes and records symlinks without following them, excluding fixture Git metadata.
The complete release gate selects its native archive fixtures. This does not
patch the shared logger, installer or snapshots. Shell/YAML syntax and source
inspection are preparation only; fixtures and controlled hosted failures remain
maintainer-owned. #20/#23 stay open for their respective qualification evidence.
The [release owner](releasing.md#ci-failure-evidence) records exact scope and limits.

Read-only metrics review identifies committed 0.2.4 at
[`21e980b3ed4f1a8d9b6203079ef1e457a3fea588`](https://github.com/dragginzgame/ic-metrics/tree/21e980b3ed4f1a8d9b6203079ef1e457a3fea588),
with passing [upstream CI](https://github.com/dragginzgame/ic-metrics/actions/runs/37587072330).
It adds fixed caller-bound histograms; the summary implementation consumed here
is unchanged. The new dirty 0.2.5 draft is Make tooling work and is not adopted
as dependency source. Neither current dependency graph is updated. Histogram
evaluation [#22](https://github.com/dragginzgame/ic-timers/issues/22) concludes
that no histogram belongs in this release: no production workload, useful bounds
or acceptable recording-cost budget is established. The upstream publication
evidence is recorded in that issue; availability is no longer the reason for
deferral. The [measurement owner](design/callback-delivery-ownership.md#histogram-evaluation)
records the decision and criteria for reopening. No runtime measurement or
dependency update is made.

The maintainer reports the batch live at released
`a30bfe01d9ce82ba691f5ddd1980f9a4b7c0454a` (0.14.7).
Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893540)
passed; its Linux log records shared runner, local gate, hook and collector
fixtures passing. [Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37592893527)
passed Linux/MSRV and both complete native macOS gates. Logs from all three hosts
record shared runner, local gate, hook and collector fixtures passing. #20 closes
with this complete consumer qualification. Failure-only
archive/upload steps were skipped in passing jobs, so #23's actual hosted
failure-artifact obligation remains unqualified. These results do not qualify
the subsequent 0.1.15 adoption worktree. No contributor validation ran.

## Shared Tooling 0.1.13 workspace adoption

The maintainer authorized [#19](https://github.com/dragginzgame/ic-timers/issues/19)
after the 0.14.5 evidence review. Exact-source
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37581058940)
at `e378671` passed Linux portable regression, lint/security and both native
macOS jobs. The reviewed diff adds the
[workspace layout rule](../rules/rust-workspaces.md) and links it from the
baseline, dependency and hook rules and the flat governance export list.
It changes no shared executable and does not fix the Make-mode boundary in #20.

A clean detached source export supplied all three snapshots at that revision.
The baseline explicitly adds the new rule, becoming 26 files; the audit/setup
and nested helper exports retain their 19/17 file sets and unchanged bytes.
No dirty sibling files were adopted or modified. AGENTS identifies the new
revision, preserves the maintainer's validation/release exceptions and records
both independent virtual roots and their package scope.

The publishable library already lives at `crates/ic-timers`. The maintained
testing packages move as follows, retaining their Cargo package names:

| Previous path | Current path |
| --- | --- |
| `testing/pocketic` | `testing/crates/ic-timers-pocketic` |
| `testing/runtime-probe` | `testing/crates/ic-timers-runtime-probe` |
| `testing/size-probe` | `testing/crates/ic-timers-size-probe` |

Only testing workspace membership and current owner documentation need path
changes. Each moved package manifest and Rust file retains its bytes and mode;
no functions, methods or types are removed. Inherited dependencies still resolve
from `testing/Cargo.toml`, whose library path is relative to that root and remains
unchanged. Make and CI select the same workspace roots and Cargo package names.
The runtime/cohort harnesses load caller-supplied artifact paths, so their
`testing/target` output locations remain. No build artifacts, frozen reports or
historical release notes were moved, deleted or rewritten. No alias or duplicate
package remains at the old maintained source paths.

The compatible 0.14.6 batch records this repository-only adoption.
Both locked offline metadata reads pass and preserve their selected lock graphs;
all moved files are compared to their previous hashes and source paths are
inspected through metadata. Three integrity checks and diff checks pass.
Root Cargo, both lockfiles and runtime source retain their pre-adoption bytes,
including the unrelated root-lock ic-metrics 0.2.3 edit. This is preparation
evidence, not native formatting/build/test or IC qualification. At preparation,
#19 stayed open pending complete native-host qualification. No new tests,
builds, lint, formatter, installation or release effects ran. No Wasm or
instruction delta was measured.

The maintainer subsequently released 0.14.6 at
`0c90c391dff5960a7502fc15a0718b03631f2515` and reports publication live.
[Exact-source main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584151377)
passed Linux checks, MSRV and both complete native macOS gates. Matching [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37584150869)
passed exact identity/main ancestry admission. #19 was closed with this
source-bound complete qualification. These are hosted results, not new contributor tests;
registry publication was not independently checked. The released root/testing
locks select ic-metrics 0.2.3/0.2.0 respectively; the external root-lock edit was
preserved throughout preparation, separately from the layout implementation.

The initial read-only inspection identified the then-dirty Make guard dependency
and recorded the required logger, exact-release and hook fixture propagation in
[#20](https://github.com/dragginzgame/ic-timers/issues/20). That release adopted
no dirty helper or entrypoint. The subsequent committed adoption is recorded
[above](#shared-tooling-0114-make-admission).

## Shared Tooling 0.1.12 adoption

The maintainer authorized the next tooling cleanup after released 0.14.4.
Shared Tooling 0.1.12 became clean and committed during that preparation; exact
source [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37511845192)
at `33c2a6f` passed Linux portable regression, lint/security and native macOS 15
Intel/Apple Silicon. All three consumer manifests were reviewed and exported from
a clean detached checkout through the upstream distribution helper. Overlapping
root verifier records are refreshed together; the nested verifier has the same
new bytes. No sibling source was modified or dirty byte adopted.

The paired baseline/maintenance rule and audit authority changes are included.
They allow relevant issue work while retaining separate authorization for sibling
file edits and all release effects. Redundant local issue-scope wording was
retired; the maintainer-owned tests/builds/lint/release exceptions and independent
workspace/PocketIC contracts remain. The baseline adds only its referenced
read-only `gh-ci.sh` and flat governance file list. The list is documentation
selection for new consumers, not an executable instruction to widen these
manifests. Existing linked guides are present and checked in the exported tree.
Standalone yq installers, shared portable-suite prerequisites and upstream CI
artifact-upload recipes are outside our caller adoption. Current host/IC pins,
no-flag parser setup and system-ripgrep bootstrap remain locally owned.

### Standard-release command checker

[#17](https://github.com/dragginzgame/ic-timers/issues/17) removes
`scripts/release/test-standard-release.sh` completely. It contained only the
copied generic smoke block and no named functions/types. `release-check` now calls
`.shared-tooling/helpers/scripts/ci/check-release-commands.sh` with the actual
consumer root and explicit `tool-versions.env` input. The helper itself is
byte-identical to the already-qualified 0.1.11 source. No compatibility adapter,
local generic duplicate or second fixture dispatcher remains.

The checker exports the actual Makefile and selected input into its own scratch
checkout, puts the recording runner at the Makefile's existing relative path,
and never invokes the real release runner. It verifies exact NUL-separated
patch/minor/major/resume arguments, success/failure propagation and all ordered
pairs of conflicting goals before dispatch. It clears inherited Make/logger
identities, retains failed logs and deletes only successful owned scratch. The
remaining local fixtures still own both-lockfile metadata, index admission,
PocketIC ordering, prepared payloads, exact tags and interruption recovery.
The version-preparation fixture's cross-reference moves to the shared owner.

### Recorded release destination and integrity bootstrap

[#18](https://github.com/dragginzgame/ic-timers/issues/18) adopts owner corrections
for [Shared Tooling #25](https://github.com/dragginzgame/shared-tooling/issues/25)
and [#27](https://github.com/dragginzgame/shared-tooling/issues/27). Standard release
intent still records the selected sole push URL. The runner rechecks that URL
after validation, after local push admission and immediately before dispatch.
The atomic branch/tag push uses the captured URL with an option boundary, matching
remote observation; later remote-name changes cannot redirect that dispatch.
Selected commit/tag refs, no-follow-tags, retained intent and lost-response
reconciliation remain. Local late callbacks retain `RELEASE_COMMIT` selection;
standalone HEAD/tag/publication guards remain local.

The adopted runner fixture expects captured-URL argv and exercises URL replacement
or addition during validation, final admission and remote observation, followed
by exact-destination retry without duplicate commit/tag. Existing uncertain-effect
and older-release recovery cases remain. No local command substitute parses the
runner's old push shape, so no parallel consumer Git substitute needs rewriting.

Snapshot verification now hashes inspected files directly with a trusted host
SHA-256 backend and executes no inspected helper. It retains file/mode/path and
manifest admission. This is independent integrity checking, not signed provenance
or protection against an untrusted verifier/manifest. `test-shared-snapshots.sh`
exports each actual consumer manifest with exact file modes into owned temporary
directories, proves the unchanged export admissible, then rejects payload-only,
helper-only and combined corruption. A marker detects any execution of the
changed no-op helper. Only copies are altered; failed fixture logs are retained.
The existing complete gate selects these checks after real snapshot admission.

The governance file list and already-present linked guides satisfy the local
export scope of [Shared Tooling #28](https://github.com/dragginzgame/shared-tooling/issues/28).
Source-tree link checks alone are insufficient; documentation references are also
inspected against the assembled baseline/audit export. No automatic widening,
new dependency, runtime API or persistence mechanism is introduced.

### Qualification boundary

Released consumer `49e4e8a25025c6a6329c993ea85255341474682b` (0.14.4) passed
[main Linux/MSRV and both complete native macOS gates](https://github.com/dragginzgame/ic-timers/actions/runs/37505847432)
and [tag identity/main ancestry CI](https://github.com/dragginzgame/ic-timers/actions/runs/37505847427).
This qualifies the previous tag-checker, logger and restricted-PATH fixture repair.
With the maintainer's explicit authorization, #16 was closed with that evidence.
That evidence qualifies only 0.14.4; registry publication was not independently
checked.

The 0.14.5 batch is compatible repository-only work. No named
function/method/type was removed; the retired fixture had only top-level commands.
Tooling preparation left timer source, Cargo versions, dependencies and both
lockfiles unchanged, so the implementation has no downstream Wasm/instruction/heap
change. Preparation evidence is exact
export bytes/modes, three integrity manifests, shell syntax, local/exported
references and diff inspection. No tests, build, lint gate, installation, release
command, staging, commit, tag, push or publication ran locally during preparation.

The maintainer subsequently released 0.14.5 at
`c84d4e4d26f968a9d7f2d37f30f7fed447692c0f`.
[Exact-source main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526800896)
passed Linux checks, MSRV and both complete native macOS gates; matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37526801007)
passed tag identity and main ancestry admission. Linux and both macOS logs verify
the three 25/19/17-file snapshots, pass the actual consumer export/corruption
fixture, the directly selected shared command checker and the captured-destination
runner fixtures. Both macOS gates also pass 142 native tests, 14 PocketIC runtime
subjects, doctests and policy cohorts. #17/#18 were closed with this source-bound
consumer evidence on 2026-10-07. No contributor checks or release effects were
executed during that review. Registry publication was not independently checked.
The records below retain their historical source scope.

### Pending upstream corrections

At the initial inspection the consumer adopted `33c2a6f`. During the 2026-10-07 review, upstream
committed 0.1.13 at `e378671d90afa237ff63a4b0e3b9551eb2c222b6`; its changes are
workspace governance only and do not correct the Make-mode finding below.
[Shared Tooling #30](https://github.com/dragginzgame/shared-tooling/issues/30)
demonstrates inherited Make modes accepting failed or unexecuted validation and
formatting. Source inspection confirms the consumer uses the affected release
runner, validation logger and formatting hook; normal-environment hosted success
does not qualify these modes. [Consumer #20](https://github.com/dragginzgame/ic-timers/issues/20)
owns adoption after a reviewed committed upstream correction, including assessment
of the local `release-x` path. Until corrected, ignore-errors, dry-run, touch and
query invocations must not be treated as validation or release qualification.
This is a tooling trust-boundary finding, not a demonstrated timer runtime defect.
No local reproduction or vendored patch was made.

[Shared Tooling #34](https://github.com/dragginzgame/shared-tooling/issues/34)
adds a virtual-root and `crates/<package-name>/` layout rule in that new revision.
Its CI was in progress at the initial inspection. The subsequent all-host pass
and authorized consumer preparation belong in the
[0.1.13 adoption record](#shared-tooling-0113-workspace-adoption) above.
GitHub remains the active follow-up owner for both changes.

## Shared Tooling 0.1.11 refresh

The maintainer authorized the selected refresh after inspection. A clean detached
source checkout at `46c0277` exported each manifest through the upstream
refresh helper into owned temporary consumers. The verified exports were installed
only after checking that changed destination files had no unrelated local edits.
Both root manifests were refreshed together because they share the checksum owner.
The baseline adds only the linked verification-helper and tag-maintenance guides;
no tag-deletion executable or unrelated helper is adopted. The nested helper set
remains sixteen files. Its only byte change since `b32d303` is success-only cleanup
in the dependency fixture, preserving failed scratch evidence; production helpers,
including the prepared #16 tag checker, are unchanged.

### Logger and release callers

The canonical logger fixes
[Shared Tooling #22](https://github.com/dragginzgame/shared-tooling/issues/22):
passing/ignored Rust paths containing `error::` remain ordinary output. Actual
`error:`, `error[E...]:` and failed-test diagnostics still receive highlighting.
Retained summaries preserve nearby ordinary context without tagging it as an
error; raw failed-attempt logs remain undecorated and survive retries.
The logger also clears its own temporary checkout/snapshot identity before target
dispatch, allowing an independently located child logger to choose its checkout.
It preserves Make release selections, failure-log policy and nesting depth.

The existing `release-verify` recipe still selects the complete target list in
its existing fail-fast order and binds logs to the Git release-state directory.
`run-release.sh`, version arithmetic, hooks and hook installation are byte-identical
to the prior baseline. No release plan, commit, tag, push or publication behavior
is replaced. `test-release-gate.sh` supplies independent Make/logger identities
and retains failures. Its new source-reviewed cases use the actual consumer recipe
and prepared/stock search paths to check passing/ignored names, typed/bare/no-space
errors, failed namespaced tests, neutral retained context, raw logs and child
checkout routing with exact inherited release version/commit and nesting depth.
These cases remain inside the already-selected complete gate.

The shared AWK finalizer now includes the
[#23 precision correction](https://github.com/dragginzgame/shared-tooling/issues/23)
and its upstream release-runner fixture cases. This fixes the previously reported
large-component boundary in the existing vendored fixture subject. It does not
replace the local Perl finalizer: its accepted whitespace, identity admission,
file ownership and transactional rollback remain with the
[existing consumer owner](#changelog-finalizer-review-and-history-fix).
No second active release selector or new compatibility path is introduced.

### Policy and setup scope

The baseline and `rules/agent-maintenance.md` now jointly adopt
[#24](https://github.com/dragginzgame/shared-tooling/issues/24): authorized local
repairs are applied in the worktree; inspection remains inspection; owning-repo
issue reports follow a duplicate search and distinguish preparation from native
qualification. Reporting authority does not authorize sibling source changes,
assignment/closure or release effects. The prior authorized #16 worktree is the
local-repair walkthrough; the explicitly requested #23 report and upstream
correction provide the issue-owner walkthrough. Neither proves this refreshed
consumer's qualification. The local filesystem scope now makes that issue-report
boundary explicit, and redundant dirty-work wording was removed after checking
that the baseline retains it. Product validation/release exceptions are preserved.

The separate `a7efade` dependency-preparation reference is retired because the
same obligations are now in the adopted Cargo rule: trace both independent graphs,
prepare affected locks together, preserve selections and check owning locked
metadata during authorized dependency changes. Workspace boundaries and command
authority are unchanged; this tooling refresh changes no dependency or lockfile.
The GitHub description was inspected and remains consistent with the README.

The host installer now rejects a failed version producer even if it emits the
expected version. Existing jq/yq pins and no-flag Make/CI setup/check calls are
unchanged. Its new ripgrep option is not enabled here; IC Timers retains its
explicit system-ripgrep bootstrap. The shared guide describes optional upstream
setup; [local release setup](releasing.md#structured-dependency-checks-and-host-parsers)
owns our callers.
The all-host retention-fixture repair from
[#21](https://github.com/dragginzgame/shared-tooling/issues/21) is qualified upstream;
that standalone fixture and its unrelated executable subjects are not imported.

### Evidence and acceptance

Exact-source [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37500153922)
passed Linux portable regression, lint/security and both native macOS 15 jobs
(Intel and Apple Silicon). This qualifies Shared Tooling's subjects at `46c0277`;
it does not qualify this consumer worktree. Preparation checks are exact exported
bytes/modes, all three snapshot manifests, shell syntax, local documentation
references and diff inspection. No test, build, lint gate, formatter, tool
installation or release effect ran locally.

Acceptance remains the maintainer's normal complete gate and matching consumer
Linux/MSRV and native macOS CI, including the expanded logger and existing tag,
metadata, installer and recovery fixtures. #16 stays open pending that evidence.
The single undated 0.14.4 draft contains this repository-only batch. Runtime source,
Cargo versions, dependency selections and both lockfiles are unchanged; there is
no downstream Wasm, instruction or heap delta from these tooling edits.
Historical adoption/qualification records below retain their original source scope.

### Consumer restricted-PATH fixture repair

The maintainer's local validation of preparatory commit `8f03721` stopped in
`test-release-gate.sh`. Its retained `stock-passing-output` and raw
`20261006T172609Z-80780-0-logging-pass.log` both report
`make[4]: echo: No such file or directory` (status 127). The prepared-path cases
completed; the restricted-path pass case failed before its diagnostic assertions,
and the later restricted failure/nested-checkout cases were not reached.
The parent gate log remains at
`.git/release-state/validation-failures/20261006T172520Z-44432-2-ci.log`, and the
original fixture remains at `/tmp/tmp.ywUjFrGE9t`. Those local paths describe this
attempt, not portable documentation links or evidence retained in Git.

GNU Make can execute a simple `echo` recipe directly instead of invoking a shell
builtin. The consumer fixture omitted external `echo` from its restricted PATH.
The local repair adds it and resolves all selected binaries with Bash `type -P`,
so a builtin name cannot produce a broken relative symlink. The restricted PATH
still excludes ripgrep and exercises the logger's stock grep branch. No shared
snapshot, production logger or gate membership is changed. Shell syntax and diff
inspection are preparation checks; the repaired fixture has not been rerun by
the contributor. Fresh maintainer execution and native-host qualification remain
required. No named function or type was removed.

## Audit-method adoption review

[Consumer adoption #11](https://github.com/dragginzgame/ic-timers/issues/11)
requires a committed source for the six shared audit documents. The earlier
review found no `audits/` files in committed `a7efade`; dirty proposals were
excluded. After the maintainer committed `a37771f`, its clean source and all six
methods were reviewed and exported from a detached temporary checkout using
the upstream distribution helper. No sibling file was modified or dirty byte
attributed to an older revision.

The [local overlay](audits/code-hygiene.md) now selects shared code hygiene.
The following map covers every question in the retired checklist. Generic
questions and automatic repair/broad-gate instructions were replaced in the
same batch as installation; product authorities and dated reports remain local.

| Retired checklist obligation | Current destination and retained local authority |
| --- | --- |
| Typed failures for invalid input and recoverable state; constructor validation; negative tests | Shared code hygiene boundary review; public constructors and control errors in `crates/ic-timers/src`. |
| Inert values versus validated configuration and runtime authority | Shared API inventory plus the local snapshot/claim/callback distinctions in [AGENTS.md](../AGENTS.md) and [architecture](architecture.md). |
| Intended facade, private implementation modules and imports from defining owners | Shared API review, Rust ownership guidance and module surface hardening; retain the crate root's private provider/registry/control/dispatch boundary. |
| Large atomic registry/runtime transitions | Shared responsibility review and flow convergence; retain independent effect confirmation, suspension and rollback boundaries from [architecture](architecture.md) and [SAFETY.md](../SAFETY.md). |
| Delegated callback capability expires with its exact work attempt | Local [callback authority contract](design/0.5-policy-specific-callback-authority.md), including cancellation, abandonment and stale-delivery ownership. |
| Separate scheduler starts, dispatches, work starts, completions, unacknowledged attempts and measurements | Local architecture and [measurement/delivery owner](design/callback-delivery-ownership.md); generic structural findings cannot merge these events. |
| Checked counters, totals, deadlines and generations | Local safety boundary and their owning source modules; preserve independent overflow and stale-generation rejection evidence. |
| Only `platform` directly uses `ic-cdk-timers` | Local AGENTS boundary and the existing `provider-check` owner, including the native substitute's test-only scope. |
| README, architecture, status, changelog and safety claims agree | Shared documentation review plus the maintained native/PocketIC evidence required by SAFETY; hygiene alone proves no recovery guarantee. |
| Pinned external Actions, necessary dependencies and intended package contents | Shared declaration/artifact review plus pinned governance, both independent dependency graphs and the package/repository check owners. |
| Forward-only SemVer selection, exact tag identity and pre-1.0 minor cuts | [Release guide](releasing.md) and the baseline's release/compatibility rules. |
| Repository-only work avoids an unnecessary package identity | Local delivery rule, including its explicit maintainer-owned repository-only release exception; `repository-check` retains its actual lint/fixture effects and is not an automatic inspection command. |

The overlay separates cheap inspection (`rg`, `git diff --check`,
snapshot integrity and applicable locked metadata) from focused test/lint/build
commands and broad gates. Every local test, build and lint command still needs
the maintainer's explicit request under AGENTS.md, including focused commands.
`make ci`, `make msrv`, `make repository-check` and `make release-verify` are not
automatic audit steps. Audit findings do not authorize mechanical or behavioral
repairs, dependency changes or release effects. This adoption introduces no
new audit schedule, executable helper or performance metric.

The audit README links the local `cb86188` baseline. Its required authority,
hard-cut and evidence contracts remain compatible with the adopted methods.
The linked Rust/simplicity principles and integrity helpers have identical Git
blob identities at both revisions. The newer pinning/setup documents and catalogs
are present to preserve the unchanged adoption/provenance documents' offline
links; they describe upstream capabilities, not installed consumer commands.
At the audit-method adoption, AGENTS.md scoped their reference-only status; no
parser/tool installer, pinning checker or setup target was activated. The later
checker adoption below activates only dependency pinning and host-parser setup.
No changed release/validation runner or whole `a37771f` baseline is adopted.

A representative walk of the preserved
[0.4.1 report](audits/code-hygiene-0.4.1-2026-08-15.md) supports the installed
coverage: transient cancellation and exact handle consumption map to local
lifetime/generation obligations; private DRY cleanup maps to flow convergence;
provider-effect cohesion maps to independent validation and rollback boundaries;
comment truth maps to documentation review. Its recorded CI/native evidence
remains historical and does not establish current behavior or IC rollback. Its
then-current PocketIC disposition cannot waive today's SAFETY obligations.
Comparison of aggregate verdicts is `N/A (method change)`; individual historical
assertions retain their original source and proof scope. No product audit or
new validation was executed for this adoption review.

The local catalog and AGENTS entrypoint now select the installed methods and
overlay. No prior method fingerprint/catalog machinery exists to rotate.
Historical checklist identity remains available at released `98c4b29`; it is
ineligible for new runs. Each new report records its consumer source/dirty scope
and the current overlay's content identity, rather than relabeling old reports.
The existing `release-check` now verifies both manifests before its unchanged
fixtures. Direct integrity checks and document-link inspection are adoption
evidence; the updated Make gate itself has not been executed locally. The
maintainer retains subsequent gate execution and native qualification.

Adoption evidence binds to consumer base `98c4b296d7461525c15a01e30adbe33b75bcfa38`
plus this working-tree documentation/manifest/Make change. The installed overlay's
SHA-256 is `2634884413884ad0e21a4a8f9b56fee39fa0eced47679b9cf3a6cc26943820d9`.
Both integrity checks pass (21 baseline and 17 audit records); inspection verifies
154 local document links/anchors. At that adoption review, Git diffs confirmed
dated reports and runtime/probe source were unchanged. Separate Cargo edits were
preserved and not qualified there; subsequent metrics changes have their
[own evidence record](design/callback-delivery-ownership.md#ic-metrics-02-adoption).
The matching [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37450707625)
passed Linux regression, lint/security and both native macOS regressions at
`a37771f`. This qualifies upstream's maintained subjects, not this consumer's
new Make invocation, product behavior or full release gate.

Refresh baseline/audit slices directly with their manifest arguments from their
reviewed clean revisions. For the nested helper slice, export its declared paths
to an owned temporary Git consumer through the distribution helper, then copy
that verified tree and manifest under `.shared-tooling/helpers/`. Keep committed
source paths and modes intact inside that root. Do not implicitly advance another
slice or copy uncommitted source. Verify all three offline:

```bash
bash scripts/ci/verify-shared-tooling-snapshot.sh
bash scripts/ci/verify-shared-tooling-snapshot.sh --manifest .shared-tooling-audits.snapshot
bash .shared-tooling/helpers/scripts/ci/verify-shared-tooling-snapshot.sh --consumer "$PWD/.shared-tooling/helpers"
```

## Structured checker adoption

[Issue #13](https://github.com/dragginzgame/ic-timers/issues/13) records two
admission defects in the retired line regex: rejection of quoted SHA references
and acceptance of Docker tags without digests. The shared owner at committed
`a37771f` already parses these declarations and has maintained YAML/TOML fixtures.
This adoption adds five unchanged files to that existing revision's supplemental
snapshot: the checker, jq filter, checker fixtures, host-parser installer and
installer fixtures. The clean detached source/export helper produces the exact
22-file manifest; the older 21-file baseline remains untouched. Dirty upstream
work and the failed newer whole-baseline CI do not supply copied bytes.

The established `actions-check` command remains the local gate adapter. It
verifies the installed jq/yq pair offline, then runs the one shared checker over
Actions and Cargo declarations. Existing CI/repository target lists continue
to select that adapter. The weaker `check-github-actions-pinned.sh` is deleted
and every maintained caller moves. Repository fixtures now inject failed Git
inventory with empty/partial records; nested paths, spaces and scratch cleanup
remain covered. Generic declaration cases have the shared fixture owner, while
provider confinement and repository-only gate ordering remain local.

Explicit `install-host-tools` / offline `host-tools-check` adopt only the parser
prerequisite of [#12](https://github.com/dragginzgame/ic-timers/issues/12).
Explicit `update-dev` and all applicable CI jobs provision it before gates. Ordinary validation
does not download parsers; the IC toolset installer, Quill and PocketIC provisioning
remain outside this batch. The original PocketIC gate and pin owner are unchanged.
There is one parser pin owner and no copied installer implementation.

The [four exact IC exceptions](releasing.md#dependency-pin-exceptions) preserve
existing selections at their qualified runtime/harness boundaries. No dependency,
lockfile, Cargo package version or timer source changes to admit this gate.
At preparation, the compatible repository-only batch selected an undated
0.14.1 changelog draft. The maintainer subsequently chose and released that
repository-only package identity; its qualification record follows.

Preparation evidence is source review, exact export/snapshot integrity, shell
syntax and diff inspection. No installer, fixture, lint, build or complete gate
has been executed locally. The `a37771f` upstream all-host result recorded above
qualifies its subjects only; this consumer's new Linux/macOS setup, fixtures and
gate invocation remain unqualified until maintained native execution. Evidence
for published 0.14.0 cannot qualify these subsequent tooling edits. Issue closure
and release execution remain maintainer-owned.

## Structured checker CI repair

The maintainer released the adoption at `fbd319de7a60d6475232439e398fff2263a9d666`
(0.14.1). Its [main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380254)
passed MSRV but failed Linux and both complete macOS gates. The matching
[tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37483380756)
failed for the same reason: shared `test-dependency-pins.sh` uses `rg` in its
rejection helper, but consumer CI provisioned parsers without this documented
system prerequisite. The preceding host-tool fixtures passed; this failure
cannot qualify subsequent fixtures or runtime gates.

The [#13](https://github.com/dragginzgame/ic-timers/issues/13) repair provisions
ripgrep explicitly through apt in all Linux jobs and Homebrew in the native
macOS job, then checks command availability before any fixtures. It preserves
the shared sources/manifests, offline ordinary checks and complete release gate.
No fixture is skipped or vendored helper patched. The one compatible 0.14.2
draft is repository-only; Cargo, locks, dependencies and runtime are unchanged.
Source/YAML projection, extracted shell syntax and diff checks are preparation
evidence. No local test, build, lint or release gate ran; fresh hosted/native
qualification remains pending for the worktree repair.

A read-only review found the lock rewrite and Cargo reader/inheritance helpers
for [#14](https://github.com/dragginzgame/ic-timers/issues/14) and
[#15](https://github.com/dragginzgame/ic-timers/issues/15) committed at
`d957d1f8801885c5b69e4a9ef900155f5f2a8a9d`. Its
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37484175750)
passed Linux regression and lint/security but failed both macOS regressions
after IC installation fixtures, before reporting host-tool success. The available
failed logs do not establish the underlying host-fixture error or native
qualification of later metadata helpers. Their subsequent authorized consumer
adoption is recorded below; upstream results do not qualify the new callers.

## Cargo and IC helper adoption

The maintainer authorized the remaining adoption after the CI repair review.
A clean detached temporary clone exported the committed helpers from
[`d957d1f`](https://github.com/dragginzgame/shared-tooling/tree/d957d1f8801885c5b69e4a9ef900155f5f2a8a9d)
through the distribution helper into a separately rooted, verified manifest. The sibling
had newer dirty changes at export time; none were copied. The release/hook/
validation runners, six audit methods and parser installer remain unchanged.
This adopts selected helpers, not the whole upstream baseline.

For [#14](https://github.com/dragginzgame/ic-timers/issues/14), the existing
`update-local-lock.sh` selects `ic-timers` and delegates transformation to the
shared Perl helper. It captures a complete successful candidate before replacing
the selected lockfile. Both root and testing adapters, regular-file ownership,
rollback backups and the two final locked/offline checks remain local. Local
identities and unqualified exact dependency references change; registry/Git
identities, source-qualified references and unrelated bytes do not.

For [#15](https://github.com/dragginzgame/ic-timers/issues/15), `actions-check`
enables the shared `--cargo-inheritance` option. Cargo discovers each manifest's
own root offline; member versions and ordinary/dev/build/target dependencies
inherit that root's catalog by alias. The approved independent `testing/` root
remains separate. The no-argument workspace-version adapter reads its selected
working/exported manifest through the shared stable TOML reader. Its guarded
`set PREVIOUS NEW` operation remains a local byte-preserving mutator. Current
adapter code still checks the selected immutable release tree, never executes
its older scripts, and retains targets for Cargo manifest validity. Parser setup
now precedes tag-version inspection. Fixture copies include new dependencies;
release Cargo stubs delegate manifest-only discovery to real offline Cargo
while keeping resolution/fetch effects substituted.

For [#12](https://github.com/dragginzgame/ic-timers/issues/12), explicit Make
setup and `update-dev` adopt the shared six-tool IC installer and unchanged
`ci/ic-tools.tsv`. Make selects both bundle bin directories; CI explicitly
prepares and verifies them. Native macOS qualification selects the installed
PocketIC binary as an explicit override and still checks the independent
consumer-audited raw binary hash and exact version. Automatic single-artifact
provisioning into the existing evidence cache remains required when no override
is supplied. This local admission/provisioning owner is retained deliberately:
replacing it with the full installer would implicitly download six tools during
validation and change the override contract. The generic bundle's receipt is
not a substitute for the consumer's PocketIC evidence pins. Installing ICP CLI
or Quill supplies no deployment, credential or network-target authority.

`release-check` includes shared Cargo, lockfile, IC-installation and checksum
fixtures alongside every existing consumer gate. They cover malformed/ambiguous
versions, parser failure with plausible output, dependency overrides/missing
aliases, duplicate local identities, source-qualified references, bundle
interruption and preservation. Wiring is not execution evidence. Preparation
is committed-source review, export/integrity, script/YAML syntax and cheap
locked metadata inspection only. That check stopped at missing offline
`ic-metrics 0.2.1` sources after a concurrent root lock change; no dependency
selection or cache was changed here. The independent testing graph passed cheap
locked/offline metadata inspection with `ic-metrics 0.2.0` and local `ic-timers
0.14.1`; the shared version reader reports 0.14.1. No tests, builds, lint gates, tool
installation, release execution or upstream mutation ran for this adoption.
This adoption leaves runtime, manifests and lock selections unchanged.
Fresh native Linux, Intel and Apple Silicon
qualification remains maintainer-owned; upstream macOS failures above remain
visible. No issue closure is implied. Helper contracts also have an exact
[upstream guide](https://github.com/dragginzgame/shared-tooling/blob/d957d1f8801885c5b69e4a9ef900155f5f2a8a9d/docs/verification-helpers.md);
the local release guide owns consumer commands and prerequisites.

## Formatter prerequisite adoption

The compatible 0.14.3 batch adopted the shared formatter guard and its
fixture from clean committed
[`b32d303`](https://github.com/dragginzgame/shared-tooling/tree/b32d3038c850a7c53470c326b0f7f11263b31669)
(Shared Tooling 0.1.9). The reviewed changes since `d957d1f` also add host-fixture
diagnostics, restore authenticated archive bytes rather than repacking on macOS,
and document shared changelog finalization. No release runner, finalizer, hook
or host-installer implementation is changed in this consumer adoption. Existing
host setup already provisions the required ripgrep; switching its installer is
separate from the formatter admission defect.

`fmt` and `fmt-check` share `format-tools-check`, which passes the existing local
`IC_TIMERS_CARGO_SORT_VERSION=2.1.4` pin to the canonical guard. Successful exact
`cargo sort --version` and successful `cargo fmt --version` are required before
any files are formatted or checked. The probes force offline Cargo and disable
rustup automatic installation. Toolchain selection remains the caller's existing
`RUSTUP_TOOLCHAIN`/Rustup contract, and both independent workspace rosters are
unchanged. The hook fixture retires its duplicated version comparison, overlays
the current guard into its isolated index, and still exercises actual consumer
formatting and preservation. Orchestration fixtures substitute this prerequisite
alongside their other leaf commands; the shared negative fixture owns tool
admission rather than duplicating it locally.
The exact [formatter contract](https://github.com/dragginzgame/shared-tooling/blob/b32d3038c850a7c53470c326b0f7f11263b31669/docs/verification-helpers.md#formatter-prerequisites)
is a revision-bound reference; the complete newer baseline is not adopted.

The source was clean and exported through the distribution helper to an owned
temporary Git consumer, then installed as the fifteen-file nested slice. All
thirteen prior hashes/modes are unchanged. The snapshot retains one source
revision for every byte, and the older baseline/audit manifests remain intact.
Subsequent dirty installer/cache and fixture proposals in the sibling checkout
are excluded from the committed source identity and this adoption.
Native upstream [CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37489483879)
passed Linux regression/lint-security and Apple Silicon portable regression
plus real IC tool installation at the inspected source; Intel was still running.
Those subjects do not qualify the new consumer Make prerequisite or hook fixture.
Local preparation uses source/export/integrity, script syntax and diff inspection
only; no formatter execution, tests, builds, lint gate, installation or release
ran. Fresh consumer native qualification remains maintainer-owned.

The pushed 0.14.2 consumer at `88aedf0a5353d176037062ae262dd67bae11beae`
has successful [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37489635619)
and [main Linux/MSRV jobs](https://github.com/dragginzgame/ic-timers/actions/runs/37489635451).
Both complete macOS jobs were queued at inspection. This supersedes 0.14.1's
missing-ripgrep failures on the inspected Linux jobs and qualifies the earlier
Cargo/IC helper fixture wiring there. It does not supply complete all-host
evidence or qualify the subsequent 0.14.3 worktree.

## Release-tooling adoption

### Annotated-tag checker adoption

The compatible undated 0.14.4 draft addresses
[#16](https://github.com/dragginzgame/ic-timers/issues/16) using the existing shared
`check-release-tag.sh` at reviewed committed `b32d303`. It was exported from a
clean detached source through the distribution helper, extending the nested slice
from fifteen to sixteen files with all prior hashes/modes unchanged. The baseline
and audit/parser manifests remain unchanged. Dirty sibling finalizer/logger work
and 0.1.10's unrelated macOS failures do not supply this helper's source or evidence.

The existing `check-tag-at-head.sh` now owns only selection: no arguments read
the workspace version and HEAD; the two-argument late callbacks retain the exact
saved release commit/version. It delegates stable-version/full-commit validation,
commit resolution, annotated-tag type and exact tag target to the shared owner.
All standalone commit/tag/push/publication and late reconciliation callers keep
their selection and ordering. The duplicate validation body is deleted, with no
forwarder or second admission path. Error wording now comes from the shared owner;
wrong-target diagnostics identify the selected SHA. No named function/type was
removed. Timer source, dependencies, Cargo versions and locks are unchanged.

The two isolated adapter fixtures include the new helper in their exported inputs.
The existing tag fixture covers accepted annotations, missing/lightweight/wrong
tags, invalid or abbreviated identities and an older selected commit despite
newer HEAD. Its new command substitutes reject failed commit resolution, tag-type
and tag-target producers with empty or exactly matching output, and check refs
remain unchanged. These fixtures are still selected by the complete release gate.
They were not executed during preparation. Source/export/snapshot integrity,
shell syntax and diff checks are preparation evidence; no commits, tags, pushes,
formatter, tests, builds, lint, installation or release ran locally. The upstream shared
verification fixture includes tag admission at the all-host-green `b32d303`,
but could not qualify the new consumer callers during preparation. Released
0.14.4 subsequently passed all native consumer gates and #16 was closed; the
[current qualification boundary](#qualification-boundary) records that evidence.

Released consumer `e001ab98195d3c8541430a934fd76f756c0717d2` (0.14.3) passed
[main Linux/MSRV and both complete native macOS gates](https://github.com/dragginzgame/ic-timers/actions/runs/37493326312)
and [tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/37493325538).
That source qualifies the completed audit/tool/parser/lock/Cargo adoptions, not
the later tag-checker worktree. At the maintainer's explicit request, consumer
issues #11–#15 were closed with that source-bound evidence. The release
[host record](releasing.md#host-support) retains the historical failure boundaries.

### Changelog finalizer review and history fix

The compatible 0.14.3 batch fixes the existing consumer-owned finalizer's
classification of undated history. Previously every undated numbered heading was
pending, so an imported section at the current package version competed with the
next draft or conflicted with the requested release. `bump-version.sh` now passes
its validated, strictly older `previous_version` to both calls through
`IC_TIMERS_RELEASE_PREVIOUS`. Sections at or below it remain history. The CLI is
unchanged; standalone use without that identity conservatively treats every
undated numbered section as pending. Supplied previous identities must be
canonical and strictly below the target. The local comparator uses component
length and explicit Perl string equality/ordering; no numeric conversion occurs.
Path/mode ownership, completed candidate status, whitespace handling, empty-note
advisories and the five-file rollback transaction remain unchanged. No named
function or type was removed.

Review of the unchanged shared finalizer at `b32d303` and newly committed
`21f3ec3dd97f2968c9f0b08924451bb2f71770d1` found a rare source-level precision
hazard. Its `historical()` first compares split component
values with `a[n] != b[n]`, before using string-prefixed ordering. Numeric strings
from `split()` can compare numerically, so equal-length values above the exact
floating-point range may collapse at that first comparison. See the
[GNU AWK comparison contract](https://www.gnu.org/software/gawk/manual/html_node/Variable-Typing.html)
and the [exact upstream source](https://github.com/dragginzgame/shared-tooling/blob/21f3ec3dd97f2968c9f0b08924451bb2f71770d1/scripts/ci/finalize-release-changelog.awk).
For example, adjacent components `9007199254740992` and `9007199254740993` are
both valid u64 values. The upstream correction should force string comparison
for equality too and qualify adjacent large components on all hosts. At the
maintainer's explicit request, this source-review finding was filed as
[Shared Tooling #23](https://github.com/dragginzgame/shared-tooling/issues/23),
including the proposed correction and unexecuted qualification case. No upstream
source was modified and no local reproduction was run. The local parser remains
the sole active consumer selector;
no vendored file is patched and no new selector is added to the snapshot.

Maintained fixtures now cover imported undated history with original spacing,
strict previous identities, those adjacent large components, spaced dated-target
refusal and failed candidate production after plausible output. The existing
version-preparation fixture carries undated previous history through preflight,
rollback and real preparation. Existing competing-draft, symlink/directory,
mode, absent-file and rollback subjects remain. Shell syntax, snapshot integrity
and diff inspection are preparation evidence; no fixture or release command ran.
Upstream [0.1.9 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37489483879)
has now passed all native hosts, but does not prove this additional boundary or
qualify the local fix. Native consumer gates remain maintainer-owned. The new
0.1.10 installer/cache and fixture-retention changes were inspected but not adopted.
Shared Tooling #23 is a P3 boundary case requiring manually enormous version
components; it does not block adoption or justify a release by itself. The local
undated-history fix is independent of that issue. No dirty upstream correction
is eligible for snapshot adoption.

### Established runner and recovery

The shared runner owns the three standard SemVer entry points, ordering, Git
operations and exact-version recovery. Consumer adapters retain the five release
metadata outputs, package identity, README projection, both lockfiles and the
complete PocketIC release gate. Publishing remains a separate command.
Normal targets restart preflight/validation failures on current source and
reconcile saved preparation intent before choosing a new increment. Once a
release is committed, newer descendant fixes or a different requested increment
cause the runner to finish that release, then validate the requested next one.
Late local checks read metadata from `RELEASE_COMMIT`, using the current adapter
and existing metadata owners; they never execute scripts from the selected old
tree. Standalone tag and publication guards still require the tag at HEAD.
Local preflight inspects the index and working files independently; commit
admission requires only the five release outputs in the index, matching the
prepared working files. The canonical validation runner retains raw release-gate
failures under the Git directory across retries, including temporary-log fallback
when the retention destination fails. Full gate membership and ordering remain.
The local retry wrapper and its duplicate fixtures are removed.
The shared formatting hook and installer are vendored unchanged. Local formatting
targets sort both workspace catalogs and format Rust; explicit setup and hosted
CI use `cargo-sort` 2.1.4 from [tool-versions.env](../tool-versions.env).
The existing two-workspace release boundary and independently centralized
dependency catalogs remain explicit in [AGENTS.md](../AGENTS.md).
The numbered pending changelog follows the new shared version-selection rules;
package mutation and validation authority remain maintainer-owned.
The [host matrix](releasing.md#host-support) owns native qualification: Linux
syntax and command-stub passes do not qualify macOS or live IC execution.

The previous `f52c0e2` adoption's 20-file snapshot, shell syntax and
metadata-preserving manifest formatting checks retain their historical scope.
Both refreshes export committed bytes from clean temporary checkouts. At initial
inspection, later sibling edits were dirty and excluded. Once those changes
were committed at `cb86188`, their source was reviewed and exported separately;
no dirty bytes were copied. The expanded file set was exported to an owned
temporary consumer, then installed only after verifying that the prior snapshot
still matched its manifest. The linked maintenance rule, formatting-hook failure
fix and validation logger are vendored unchanged. The GitHub description was
inspected and matches the current purpose.

The matching upstream [CI run](https://github.com/dragginzgame/shared-tooling/actions/runs/37428740374)
passed on Linux and both macOS 15 architectures at `9437bab`. That qualifies
upstream's fixtures, not this consumer's adapters or live release effects.
The later [upstream run](https://github.com/dragginzgame/shared-tooling/actions/runs/37431805988)
for `cb86188` passed Linux regression and lint/security, but both macOS jobs
failed in snapshot-distribution fixtures with `source is not a Git checkout`.
Their logs show the validation runner, installer and formatting-hook tests
passing first; the later metadata fixture was not reached. Source review
indicates a logical `/var` versus physical `/private/var` mismatch between the
fixture's fake source identity and the refresh helper's canonical path. That is
a diagnosis, not an executed reproduction. The distribution helper/fixture are
not part of this consumer's offline snapshot, and its verification still passes.
This result qualifies the shared logger fixture on both native hosts but does
not qualify the complete upstream batch or this consumer's changed adapters.
An upstream fixture correction and matching rerun remain necessary; no upstream
source or GitHub issue was changed during this consumer investigation.
Consumer scope, pending fixture/native qualification and artifact preservation
belong in the [release guide](releasing.md#standard-release-runner), addressing
[#10](https://github.com/dragginzgame/ic-timers/issues/10). No tests, release
execution or upstream mutation were performed during this refresh. Hook
activation remains a separate maintainer setup action.

The 0.13.0 release uses published registry `ic-metrics 0.1.3` in the root
dependency catalog and both lockfiles, with one resolved registry package per
workspace. The temporary sibling path and local 0.1.1 selection belong to the
earlier extraction, superseded by
[registry adoption](https://github.com/dragginzgame/ic-timers/issues/9). Its
initial focused library check reported an unused-assignment warning in separate
delivery-retirement work; that historical result does not qualify the current
runtime. Later focused evidence is scoped in the [handoff](status/current.md).

Focused measurement tests passed for role-specific accounting and registration
identity during extraction. Subsequent unrelated delivery-retirement changes
retain their own validation owner; these results do not qualify those changes.
