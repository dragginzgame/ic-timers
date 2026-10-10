# Shared Tooling adoption

## Shared Tooling 0.3.6 hook and lock selection

After released **0.17.4** `d7ae76c8b23ea4b0a868eb86330531ef8bd70116`, pending
**0.17.5** refreshes all three canonical exports to committed Shared **0.3.6**
`0604bfd730ec7ec288cd2cfdad217a0d42bf256b`, confirmed against remote main.
Selections remain **56 baseline / 30 audit-setup / 11 helpers**. Clean detached
source, canonical export logs and preserved incoming catalog/lock/pins/index are
at `/tmp/ic-timers-shared036.u0iglx_p/`. The isolated helper export uses an owned
temporary Git consumer; no sibling file is modified or uncommitted byte copied.

[Timers #40](https://github.com/dragginzgame/ic-timers/issues/40) adopts
[Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106)'s hook
repair. Initial, post-snapshot and post-format tree observations admit command
status before comparing values. The consumer hook fixture injects status 23
with empty or matching tree output at each boundary, checking the diagnostic,
exact status, stopped later Git commands, staged entries and working-file preservation,
and whether formatting ran. Existing successful formatting, mode rejection,
partial staging and formatter-failure cases remain. Cases are written, not run;
source-bound current/native qualification remains outstanding, so #40 stays open.

The selected Cargo installer gains `--lockfile`; the Testkit adapter now passes
the absolute root lock instead of parsing package/version/source locally.
[Shared #96](https://github.com/dragginzgame/shared-tooling/issues/96)'s owner admits
one exact stable crates.io selection, propagates parser failures and rechecks the
selection before activation and returning a path. Shared fixtures own malformed,
ambiguous, unsupported-source and changing-lock cases. The consumer fixture checks
the exact forwarded lock, unchanged lock bytes, explicit offline policy and failed
admission with plausible stdout in both setup/check modes before Testkit dispatch.
Its duplicate parser cases are removed; ordered Make/jobserver and server failure
propagation remain. Server provisioning/admission stays entirely Testkit-owned.

Other selected changes are host/IC setup guidance and the upstream Rust installer
fixture. The optional producer optimizer smoke and hook-adoption checker remain
unselected. Consumer-owned Binaryen **132** pins, tool installations and retained
evidence are preserved; no optimizer, Node, task scheduler or second server route
is enabled. The incoming maintainer-owned catalog selects Testkit `0.33`; locked
offline metadata resolves **0.33.0**, all four Host packages at **0.12.4** and
Metrics **0.5.4**. Its dependency update is separate from this refresh and must
retain its own qualification.
No timer/public API or measured Wasm/instruction change follows from this batch.
Tests/builds/lint/setup and release effects remain maintainer-owned.

Permitted checks pass: integrity for all 56/30/11 exports, syntax for 41 shell
scripts and 27 embedded Bash bodies, and locked offline metadata. Incoming
Cargo, lock, pin catalogs and raw index compare exactly with preserved copies.
The [exact-source producer CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38061078001)
is queued at observation; producer delivery supplies no consumer execution or
native-host proof. The maintainer's existing full gate retains qualification.

The maintainer's later release validation passed the Testkit adapter fixture,
then stopped in the first empty-output hook case because its raw-index assertion
rejected a harmless `TREE` cache refresh. Retained fixture
`/tmp/timer-hook-test.lcNSLP/` contains identical 263 entry records and staged
maps before/after, with the cache extension growing from 1357 to 1644 bytes.
The local fixture now compares NUL-delimited `git ls-files --stage` output, so
paths, object IDs, modes and stages remain checked independently of cache bytes.
Failure status, stopped commands, working files and full staged diff checks
remain. Original fixture/logs are preserved; diagnosis and incoming inputs are
at `/tmp/ic-timers-hook-index-cache.nbhv05iq/`. This repair changes no shared hook
or snapshot bytes. Syntax and diff inspection are preparation evidence; the
contributor does not rerun the fixture or gate under the validation exception.

## Shared Tooling 0.3.5 review and pending hook repair

Committed upstream **0.3.5** `a744d7f1990b9e1451ef45cd6d495de00a141cd3` matches
remote main. Since the reviewed 0.3.3, selected executable payloads remain
unchanged; 0.3.4 changes selected host/IC guidance and adds an optional producer
optimizer qualification helper outside our export. The 0.3.5 repair clears
inherited log/summary selections in the producer's validation-runner fixture,
which Timers does not select. Its analogous consumer-owned release-gate fixture
gap is repaired locally, with evidence at the
[release owner](releasing.md#0173-delivery-and-fixture-log-isolation).

[Timers #40](https://github.com/dragginzgame/ic-timers/issues/40) tracks adoption
of the hook observation repair in
[Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106).
The sibling's uncommitted 0.3.6 work is not canonical delivery. Retain reviewed
0.3.3 snapshots and the byte-exact shared hook until its producer revision is
committed, then refresh through canonical exports and qualify actual Git failure
status, untouched index/files and stopped later operations on supported hosts.
This review changes no optimizer pins, optional helper selection or setup route.

## Shared Tooling 0.3.4 review and retained optimizer pins

Committed upstream **0.3.4** `169d77b8440568c5200eede971625126181f7bb2` matches
remote main. Its change selects Binaryen 133 at the upstream pin owner, adds
a producer-owned Node/Wasm optimization smoke and updates setup/host guidance.
The engineering baseline, selected Make/setup/helper scripts and runtime runner
are unchanged from our reviewed 0.3.3. Exact-source
[producer CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38054347275)
is queued at observation; [Shared #102](https://github.com/dragginzgame/shared-tooling/issues/102)
owns qualification.

Timers retains its three canonical **0.3.3** snapshots and consumer-owned
`ci/ic-tools.tsv` selecting **132**. No maintained build/probe target invokes
`wasm-opt`, so this inspection establishes no product Wasm/instruction benefit
from changing that installed tool. Upstream requires consumer Wasm/native
qualification before a new optimizer selection and permits retaining a reviewed
local matrix while that remains outstanding. No pin edit, setup, optimization
smoke, Node dependency or additional CI gate is added. Review the completed
producer evidence and a concrete consumer qualification subject before moving
the pin; do not count existing unoptimized probe acceptance as Binaryen evidence.

## Shared Tooling 0.3.3 validation completion

Released **0.17.2** advances all three canonical exports from 0.3.2 to committed
Shared **0.3.3** `d63f0cfaba8ab2961d6012064adbf051c1898bc1`, confirmed against remote
main. Selections remain **56 baseline / 30 audit-setup / 11 helpers**. The only
selected payload change is `scripts/ci/run-validation-targets.sh`; governance,
setup and helper bytes are unchanged. Clean detached source, canonical export
logs and preserved incoming inputs are retained at
`/tmp/ic-timers-shared033.0vrxoaq1/`. This continues the 0.3.2 adoption below,
including the advisory README task and local jobserver/fixture changes.

[Shared #104](https://github.com/dragginzgame/shared-tooling/issues/104)'s committed
repair admits a canonical non-negative decimal nesting depth of at most 18
digits before arithmetic, log creation or target dispatch. Empty/unset depth
starts at zero; leading zeroes, expressions, signs, whitespace and oversized
values are rejected. The runner's source wrapper and log cleanup require explicit
completion, preserve nonzero failure status and retain available evidence on
premature exits, including Bash 3.2 nounset exits which can report zero. Completed
target failures keep the existing evidence/status contract. There is no local
runner patch or new execution path. Upstream's runner fixture adds malformed-depth
and actual-source early-exit cases; it remains producer-owned rather than adding
a second consumer fixture. Existing consumer fixture completion work stays with
[#38](https://github.com/dragginzgame/ic-timers/issues/38).

Exact-source [Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38052409053)
is queued at observation. This refresh supplies no new executed fixture, native
host, timer or PocketIC qualification. The maintainer-owned gate retains those
obligations. The incoming root catalog/lock now select Testkit 0.32.0, all four
Host 0.12.2 packages and Metrics 0.5.1; the
[graph record](releasing.md#0172-preparation-inputs) owns that separate selection.
This inspection preserves those inputs and adds no dependency/version mutation.

Permitted checks pass: integrity for all 56/30/11 snapshot files, syntax for 38
shell scripts and 24 embedded Bash bodies, added local documentation links,
diff whitespace and full locked offline metadata. Cargo, incoming lock, both
pin catalogs and the real index compare exactly with preserved input copies.
These checks execute no setup or fixture and supply no new host qualification.

## Shared Tooling 0.3.2 jobserver and fixture admission

After pushed **0.17.1** `2a8710834c9283ccc46daa4e4c93b1cc4629c6bc`, pending
**0.17.2** selects committed Shared **0.3.2**
`c16444bf006f17c5bb4dda5ad070a0f345da9623` through all three canonical exports.
The baseline explicitly adds `tasks/readme-freshness.md` to close its updated
catalog/prompt links; selections are **56 baseline / 30 audit-setup / 11 helpers**.
Clean source, export logs and preserved incoming inputs are retained at
`/tmp/ic-timers-shared032.rXciNI/`. The isolated helper slice uses a verified
temporary Git consumer as before. No fleet helper, production override or new
installation route is added; both pin catalogs and real index are preserved.

Shared formatting, Rust setup/check and LOC recipes now preserve Cargo's
jobserver descriptors. Standalone `make/tools.mk` includes the existing execution
guard, so recursive recipe markings cannot admit dry-run, touch, question or
ignore-errors execution. Local Cargo build/lint/test/package/publication recipes
and Testkit setup/check receive the same descriptor handoff. Targets, arguments,
ordering, graph selection and user-owned execution authority are unchanged.
The existing Testkit fixture checks readable/writable pipe jobserver descriptors
through the actual root Make product extension, requesting pipe mode only when
Make supports that option. Shared fixtures own the generic Make guard checks.

Selected shared fixtures require explicit completion before successful cleanup.
The local Testkit adapter adopts that boundary too and injects nounset, command,
nonzero, premature-zero and completed exits into disposable copies of itself
before any installer/owner dispatch. These cases are written, not run. Remaining
consumer-owned fixture admission is tracked by
[#38](https://github.com/dragginzgame/ic-timers/issues/38); the canonical production
validation-depth gap remains with
[Shared #104](https://github.com/dragginzgame/shared-tooling/issues/104), whose fix
is not included in this reviewed commit. Do not patch the shared runner locally.

The requested [README task](../tasks/readme-freshness.md) is advisory: assess
maintained commands, examples and claims against their actual owners, allow
valid older examples and ranges, and report concrete findings without prose
rewrites or release/CI blockers. It joins the seven routine maintenance tasks;
this refresh enables no schedule and adds no release prerequisite.

Exact-source [Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38051446835)
passes lint/security, with Linux regression running and both native jobs queued
at observation. Existing released consumer results do not qualify this new
snapshot or its local fixture changes. No contributor test/build/lint/setup or
release command runs. Current offline metadata inspection stops at the uncached
maintainer-selected Metrics 0.5.1 archive; that selection is preserved rather than
fetched or reverted. Host/Testkit graph review has its
[release owner](releasing.md#host-0122-and-testkit-032-review).

Permitted preparation checks pass: integrity for all 56/30/11 snapshot files,
syntax for 38 shell scripts and 24 embedded Bash bodies, seven new local
documentation targets/anchors, and diff whitespace. Cargo, incoming lock, pin
catalogs and real index compare exactly with the preserved input copies. Full
locked offline metadata remains blocked by the uncached Metrics archive; these
checks execute no fixtures and provide no new native qualification.

## Shared Tooling 0.3.1 setup preflight

After released IC Timers **0.17.0**
`5e0d0865248f6ebfc1f98c896581f21e2ce67831`, pending **0.17.1** refreshes all three
canonical exports to committed Shared **0.3.1**
`fa452afaa5012866eb1c20820dfa8038c106e7ec`, retaining **55/30/11** selections.
The clean detached source, export logs, unchanged input copies and graph/source
inspection are retained at `/tmp/ic-timers-shared031.aFkkRB/`. Baseline and audit
slices refresh directly; the isolated helper slice is exported to an owned
temporary Git consumer and copied with its verified paths and modes. All slices
use the same reviewed revision. No optional fleet helper or task is added.

`install-tools` first calls the existing IC and Rust installers with
`--preflight`: admit the complete-set platform and IC catalog, then probe the
consumer-selected `rustc` and `cargo` with Rustup auto-installation disabled.
These calls exit before tool/build directory creation and downloads. Installation
then retains host → IC → Cargo → selected Testkit CLI/server ordering; setup still
requires explicit Rust bootstrap. Offline `tools-check`, narrow setup commands,
tool receipts/authentication and retained failures preserve their prior roles.
The improved host check names the exact tool, expected version, path, reason and
repair command; untrusted payloads are authenticated before diagnostic execution.

The existing Testkit adapter fixture admits the two preflight calls separately
from setup/check, requires their order under `-j4`, and injects failure at every
preflight/common/CLI/server boundary. Its expected command prefix prevents a
preflight failure from reaching installation or product setup. These new cases
are written, not executed; upstream fixtures retain generic installer ownership.
Cargo, lock, both tool pin catalogs and the real index remain unchanged. The
released graph is Metrics 0.5/Testkit 0.31/Host 0.11/PocketIC 16.1; this tooling
refresh adds no timer code, production dependency or measured optimization.

[Shared #101](https://github.com/dragginzgame/shared-tooling/issues/101) owns producer
acceptance. Its exact-source
[CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38049622600) passes
Linux regression and lint/security, with both native macOS jobs queued at
observation; no earlier result qualifies this revision. Released
0.17.0's Linux/MSRV/tag proof has its
[release owner](releasing.md#0170-release-acceptance), with both native macOS jobs
still queued. [#36](https://github.com/dragginzgame/ic-timers/issues/36) remains open
for complete released 0.3.0 adoption qualification. New 0.3.1 consumer execution
belongs to the normal user-operated gate; no contributor test/build/lint/setup
or release command is run.

Permitted preparation checks pass: all three snapshot integrity checks, syntax
for 38 shell scripts and 24 embedded Bash bodies, eight new local documentation
targets/anchors, diff whitespace and full locked offline metadata. Preserved
Cargo, lock, pin and index copies compare exactly. These checks execute no setup
or fixtures and provide no new native host qualification.

## Shared Tooling 0.3.0 complete toolset

Released **0.17.0** adopts committed Shared **0.3.0**
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d` after released 0.16.7
`999d9b5c3a84ec5abd729ca72b8f259abbb060e1`. All three canonical exports retain
**55 baseline / 30 audit-setup / 11 helper files**. Clean detached source,
canonical export logs and preserved inputs are retained under
`/tmp/ic-timers-shared030.roCrPG/`. The dirty sibling dashboard is excluded;
no fleet caller or additional snapshot file is selected. All pin values remain
unchanged; the consumer-owned IC matrix is not overwritten.

The host installer always authenticates/adopts jq, yq, ripgrep with PCRE2 and
cloc. Its removed optional flags have no aliases. Both common aggregates now
run host → five IC executables → cargo-sort/cargo-sort-derives/candid-extractor,
stopping on failure. This consumer registers its existing `install-testkit-server`
and `pocketic-check` as the matching ordered product extensions. Those targets
do not call the aggregate; Testkit retains CLI/server selection, receipts and
offline admission. The admitted release preflight and complete release roster
retain their existing fetch/setup/check boundaries.

`update-dev` prepares the pinned Rust toolchain before aggregate setup and offline
check, then installs the formatting hook. All four hosted setup sites likewise
prepare their declared toolchain first. Separate global cargo-sort installation
and repeated Testkit setup are removed. CI uses explicit Bash pipefail while
teeing complete aggregate logs into RUNNER_TEMP; the existing local collector
retains `tools-*.log` with formatter failures and failed builds/bundles. The
canonical selector uses complete host admission; selected shared fixtures and
the retained-evidence action refresh with their owners.

The existing local Testkit adapter fixture gains actual root-Make aggregate
coverage under `-j4`: common steps must precede CLI/server setup and check, and
failure at each boundary must stop later effects while preserving the fixture
lock. The collector fixture compares retained aggregate log bytes/modes. Shared
fixtures retain ownership of common downloads, receipts, reuse and generic
ordering; this consumer does not implement another installer or pin catalog.
These new behavioral assertions are written but unexecuted under the approved
local validation exception. Native macOS execution is still required separately.

The changed setup/host-installer contract required a minor boundary. Preparation
initially preserved incoming Metrics 0.4 and Testkit 0.30 selections; the released
Metrics 0.5/Testkit 0.31
[identity and graph](design/callback-delivery-ownership.md#ic-metrics-05-released-graph)
supersedes that preparation and is separate from Shared setup. Cargo/pin/index
inputs were preserved during contributor preparation; no package
version or release effect is performed. Existing installations, receipts and
failure artifacts remain intact. No timer runtime optimization is claimed.

[#36](https://github.com/dragginzgame/ic-timers/issues/36) owns consumer delivery
and qualification; [Shared #98](https://github.com/dragginzgame/shared-tooling/issues/98)
owns producer acceptance. At inspection,
[matching Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38044218125)
passes lint/security, with Linux running and both native jobs queued. Committed
source, producer results and local integrity checks do not qualify consumer
setup or its new graph. Tests/builds/lint/setup and release execution remain
maintainer-owned; the full normal gate is retained.

Permitted preparation checks pass: exact source/hash/mode/companion closure,
all three actual snapshot checks, syntax for 42 selected/changed shell scripts,
six embedded fixture stubs and all 28 workflow shell steps, parsed workflow
pipeline/PATH review, 279 documentation targets/anchors, whitespace and complete
locked offline metadata. Incoming Cargo bytes, both pin files and real index
are unchanged. These checks do not execute the new aggregate or its fixtures.
[Consumer feedback](https://github.com/dragginzgame/ic-timers/issues/36#issuecomment-6096551930)
records this preparation. Subsequent downloaded 0.17.0 Intel/ARM logs complete
consumer acceptance alongside Linux/MSRV/tag evidence at the
[release owner](releasing.md#0170-release-acceptance); this resolves #36 without
qualifying later dependency graphs, shared revisions or fixture repairs.

## Shared Tooling 0.2.14 CI inspection

After released IC Timers **0.16.6** `0b929539686a5c428a6a3af96c2a88139cc5553d`,
subsequently released **0.16.7** refreshed all three canonical snapshots to committed
Shared **0.2.14** `fd11692f31e7dfd44dcc2ca56634eaeab3569825`, retaining
**55 baseline / 30 audit-setup / 11 helper files**. Clean detached source and
canonical export records are retained at `/tmp/ic-timers-shared0214.axid05wo/`.
No new optional helper, caller, test roster or local parser is introduced.
Engineering baseline, audit methods, Make includes and installation behavior
are unchanged from 0.2.13; the maintenance rule and selected `gh-ci.sh` refresh.

`gh-ci.sh --run ID --logs` no longer silently succeeds when a failed, cancelled
or unfinished run has no returned failure logs. It retains observation files,
checks status/conclusion only for an empty log body and reports unavailable
failure evidence. Completed successful/neutral/skipped runs can legitimately
have no failed-step logs. A failed fetch retains partial stdout/stderr and its
original nonzero status. This is read-only inspection, not CI rerun or gate
acceptance. [Shared #97](https://github.com/dragginzgame/shared-tooling/issues/97)
owns the helper's regression coverage; consumers do not duplicate its test suite.

The snapshot refresh preserves its released Cargo/pin inputs and the real index.
Subsequent maintainer catalog/lock edits select Testkit 0.29 and Host 0.11;
those incoming bytes are preserved, with the separate
[graph owner](releasing.md#testkit-029-and-host-011-preparation) recording scope.
No contributor tests, builds, lint, tool setup
or release effects run under the existing local command-authority exceptions.
The complete user-operated gate remains unchanged. Exact-source
[Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38041453237)
passes lint/security and Linux portable regression, with Intel running and ARM
queued at inspection;
this does not qualify the consumer's uncommitted helper selection.

Permitted preparation checks pass: exact source/hash/mode and companion closure
for all 55/30/11 records, all three actual snapshot integrity checks, syntax of
38 selected shell scripts, documentation targets/anchors and diff whitespace.
The unchanged engineering baseline and all audit methods match 0.2.13 byte for
byte. Complete locked offline metadata records the incoming graph separately;
it does not execute or qualify the new dependencies.
[Consumer feedback](https://github.com/dragginzgame/shared-tooling/issues/97#issuecomment-6096202572)
records this preparation at the existing upstream owner.

## Shared Tooling 0.2.13 selected CLI preparation

Released IC Timers **0.16.6** selected committed Shared
**0.2.13** `5864f468d39f8f9d1bd26fca1afe0e20f25f1b5e` across all three canonical
snapshots, retaining **55 baseline / 30 audit-setup / 11 helper files**.
Clean detached source and exports are retained under
`/tmp/ic-timers-shared0213.7y9utif1/`. No moving sibling bytes or optional tools
are adopted; Cargo catalogs/lock, both pin files and the real Git index preserve
their input bytes. The earlier 0.2.12 directory admission and fixture cases below
remain part of this batch.

The shared selected-Cargo installer now names package, exact version, target
kind/name, profile and destination when an installation is missing or invalid.
Our Testkit adapter adds `run make install-testkit-server`, preserves the original
failure status and never treats a failed command's partial stdout as authority
to execute the CLI. Successful admission still returns the owner-selected path.
Receipt checks, immutable installation reuse and failed-build evidence stay with
the canonical installer; server setup and admission stay with Testkit.

The shared release/dependency contract additionally requires existing selected
CLI setup at admitted preflight. Our ordered complete release roster already
prepared Testkit before validation. The local preflight adapter now also calls
`fetch`, `install-testkit-server`, then `pocketic-check`, after source/version/notes
admission. No entrypoint prerequisite or parallel sibling setup is added.
Saved prepared/committed phases retain the runner's existing recovery routing;
this does not replay preparation against partly written metadata. Standalone
watchdog/cohort commands retain their offline-admission dependency before builds.
The [release owner](releasing.md#pinned-ic-tool-setup) records these commands.

Consumer fixtures add selected-version forwarding, explicit offline-policy
inheritance, failed/partial CLI output isolation, preflight setup/check failure
ordering and metadata preservation, and actual parallel Make refusal before
probe builds. The real locked-fetch recipe remains exercised; setup/check effects
are substituted. Installation reuse, byte receipts, old-installation preservation
and locks remain tested at their canonical shared owner rather than reproduced
in another consumer installer. New behavioral cases are written but unexecuted.

The baseline also clarifies virtual workspace layout, final-response-only cleanup
inventories and full-suite-before-delivery validation. The existing four-member
root catalog already has the required layout. Our explicit maintainer-approved
AGENTS validation and release exceptions remain authoritative; this refresh
does not authorize contributor tests, builds, lint, setup or release execution.
No timer source/API changes or measured Wasm/instruction/heap changes are claimed.

Permitted preparation checks pass: exact committed source/hash/mode and companion
closure for 55/30/11 records, actual snapshot integrity, shell syntax (45
selected/local scripts and ten embedded shell stubs), 250 local documentation
targets/anchors, whitespace and complete locked offline metadata. The graph is
retained at `/tmp/ic-timers-0166-shared0213-metadata.json`, with four local 0.16.5
members and the expected unique incoming package selections. Repository purpose
and its GitHub description remain aligned. No tests or tool setup were executed.

[Exact-source Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38039035514)
passes Linux portable and lint/security, with ARM running and Intel queued at
inspection. Source/integrity/syntax/documentation/locked-metadata checks remain
preparation evidence. Released 0.16.6 qualification is recorded separately in the
[source-bound owner](releasing.md#0166-release-acceptance).
[#96](https://github.com/dragginzgame/shared-tooling/issues/96) owns shared delivery
and remaining consumer coordination, independently of directory admission #95.
[Consumer feedback](https://github.com/dragginzgame/shared-tooling/issues/96#issuecomment-6095895802)
records the local preparation and its unexecuted behavioral scope.

## Shared Tooling 0.2.12 directory admission

Earlier preparation of IC Timers **0.16.6** refreshed all three canonical snapshots to committed
Shared **0.2.12** `a8ba9b461b831846eacf64452e6ddcd2acd000f1`:
**55 baseline / 30 audit-setup / 11 helper files**, with no new selection.
The root baseline and both verifier copies come from a clean detached source
through its canonical exporter. The audit methods are byte-identical at the new
reviewed revision. Moving sibling changes after that commit are excluded.

The new baseline prohibits LF/CR in operational directory names, including
ancestors and resolved symlink destinations. Deliberate negative fixtures are
allowed; preserve previously retained artifacts and evidence rather than renaming
or deleting them. The exporter and trusted verifier validate supplied and physical
paths before command-substitution trimming can select another checkout. Snapshot
records additionally reject CR alongside LF/tab. Ordinary spaces, relative paths
and physical aliases remain supported; no snapshot shape or compatibility reader
is added. The [upstream issue](https://github.com/dragginzgame/shared-tooling/issues/95)
retains the wrong-target export and false-verification evidence.

The consumer fixture exercises all three selections. A newline-ending empty
checkout must refuse even when its trimmed neighbor has a valid snapshot. CR and
CRLF directories contain valid payloads, so a failure cannot be attributed merely
to a missing manifest. Direct and aliased forbidden roots must return failure and
leave both manifests unchanged. Normal relative/aliased roots still verify with
CDPATH set. These are intentionally negative directory fixtures, not supported
operational paths. Existing corruption/refusal and untrusted-helper isolation
checks remain. The new cases are written, not executed by the contributor.

Canonical preparation records are retained under
`/tmp/ic-timers-shared0212.3kq1myy5/`. Source bytes/modes, companion closure,
all snapshot integrity records, shell syntax, documentation references, whitespace
and locked offline metadata are permitted preparation checks. Cargo manifests,
incoming Cargo.lock and both consumer pin files retain their exact input bytes.
Tests/builds/lint/setup and all version/release effects remain maintainer-owned;
no functions, methods or types are removed, and no timer runtime change is made.

[Exact-source Shared CI](https://github.com/dragginzgame/shared-tooling/actions/runs/38037750017)
passes Linux portable regression and lint/security; Intel is in progress and ARM
remains queued at inspection. This and canonical export do not qualify the pending consumer
source. The released 0.16.5 Linux/ARM/MSRV/tag results remain bound to its 0.2.11
selection, with Intel still in progress. New-source acceptance is separate.

[Consumer feedback](https://github.com/dragginzgame/shared-tooling/issues/95#issuecomment-6095815906)
records this preparation upstream; [#35's follow-up](https://github.com/dragginzgame/ic-timers/issues/35#issuecomment-6095816387)
records the released ARM evidence and pending consumer scope. Both issues remain
open for their respective remaining acceptance, not for a new timer feature.


## Shared Tooling 0.2.11 formatting and Make admission

Released IC Timers **0.16.5** selects exact committed Shared
`83efac446348dea024798a331d77933b24b429dc` across all three clean canonical
exports: **55 baseline / 30 audit-setup / 11 helper files**. Its committed
VERSION is **0.2.11**, despite the commit subject saying 0.2.10; source identity
comes from the exact revision and committed bytes, not that subject. This
supersedes the pending 0.2.9 selection below and retains its snapshot diagnostics.
The baseline and audit-method bytes remain unchanged. Add only the explicitly
selected `scripts/ci/run-formatting.sh` companion, not optional fleet or registry
observation helpers.

The canonical formatter prints `Formatting... ok` or `Checking formatting... ok`
on success. Failure prints the command's exit status and an escaped path to the
complete retained stdout/stderr log. Make may add its own error line and returns
its normal recipe-failure status. Sorter-first ordering, all workspace members,
check-only flags, offline tool admission and selected consumer root/pins stay at
their existing owners. Each of the five disposable actual-Make callers receives
the new companion; the index-hook fixture stages it and the shared release smoke
caller explicitly includes it. The shared immutable evidence action refreshes
with the audit snapshot. Our consumer collector includes `formatting.*` from
CI's required RUNNER_TEMP; the wrapper inherits that selection. Local invocations
without RUNNER_TEMP retain failures under TMPDIR or /tmp instead.

This revision also contains Shared #30's hidden-mode correction. The selected
include passes MAKEFLAGS and MFLAGS independently through GNU Make's parser and
refuses assignments that erase generated MFLAGS. No consumer parser or vendored
patch is introduced. The actual consumer release and formatting fixtures now
cover all four unsafe modes with preserved, cleared and replaced MAKEFLAGS and
both variables cleared, requiring status 2 before effects. Existing parallel,
argument-bearing recursive Make, selected-root and direct-policy cases remain.
The formatter fixture checks exact success output, sorter failure status 23,
complete retained output streams, skipped Rustfmt and unchanged index/worktree.
The collector fixture archives an actual reporter failure and compares its bytes
and mode. These new behavioral cases are written, not executed by the contributor.

Clean source, canonical export and preparation records are retained under
`/tmp/ic-timers-shared0211.706a0ieg/`. Snapshot source bytes/modes and companion
closure, integrity, shell syntax, local documentation references, whitespace and
complete locked offline metadata are the permitted preparation checks. No tests,
builds, lint, setup, Make target, version mutation, staging, commit or release
runs. Cargo.toml, the incoming Cargo.lock and both pin catalogs are byte-preserved.
No function, method or type is removed; the formatter/admission change has
expected zero production timer Wasm/instruction/heap impact because it changes
no product code. The preserved incoming graph separately selects Metrics 0.3.6,
Testkit 0.28.0, Host 0.10.1 and PocketIC 16.1.0. All seven published Metrics Rust
source files match 0.3.5 exactly; no arithmetic adapter change is needed. The
new package identity and cc/smallvec/syn refresh still require their own execution
qualification; no measured Wasm or instruction delta is claimed for that graph.

This adoption was prepared as repository-only maintenance in the 0.16.5 batch.
The exact Shared source now passes
[all three hosted profiles](https://github.com/dragginzgame/shared-tooling/actions/runs/38034912323);
Shared #30/#91/#92 are closed. Released consumer **0.16.5**
`0d7b85ec6658f91421bd13c44fa19592d8cc029e` passes matching Linux/MSRV/tag checks.
[The graph owner](releasing.md#0165-linux-and-tag-acceptance) scopes actual hook,
release, product and collector evidence. Both native consumer jobs now pass.
[#35](https://github.com/dragginzgame/ic-timers/issues/35) retains the pending .6
explicit working/index lock-preservation proof; new snapshots/graph/fixtures do
not inherit released acceptance.

The maintainer's 2026-10-10 release verification of preparation commit
`71f24df` passes fetch, Testkit server setup and offline admission, then fails
the hook fixture's exact-output comparison. Retained
`/tmp/timer-hook-test.EouiJS/format.log` contains the correct formatter success
line followed by the hook's existing selected-file refresh confirmation. The
fixture incorrectly applied the formatter's one-line contract to the whole hook.
Its expected output now includes both lines; direct `fmt-check` still requires
exactly its one formatter line. The canonical hook and formatter are unchanged.
Full failure log:
`.git/release-state/validation-failures/20261010T075030Z-2112417-3-ci.log`.
The repaired fixture remains unexecuted by the contributor; user verification
and new-source host acceptance are still required.

Subsequent successful tag job **114168872998** and Linux job **114168872952**
execute the repaired hook, complete retained-log/Make admission fixtures and
release/version preparation at 0.16.5. Logs are retained as
`/tmp/ic-timers-0165-tag-job.log` and `/tmp/ic-timers-0165-linux-job.log`.
That supersedes the failed preparation observation above without rerunning it.

The pending **0.16.6** follow-up adds the explicit lock-preservation obligation
from #35: distinct index and unstaged Cargo.lock bytes must survive successful
selected-file hook formatting and direct checks. Existing rejection/failure
comparisons now include working lock bytes. Current formatter prerequisites in
[the release guide](releasing.md#formatter-prerequisites) name the new companion
and independent flag admission, retiring the stale claim that the old bypass
remains current. No canonical payload or product timer behavior changes. The new
fixture assertions are unexecuted; prior host results cannot qualify them.

Earlier adoption records retain their source and qualification scope.

## Shared Tooling 0.2.9 snapshot diagnostics

After pushed IC Timers **0.16.4** `da921fc899d2c7c99ed0a6cacac6e2111307ac71`,
the authorized continuation refreshes all three selections together to committed
Shared **0.2.9** `f8a70ba348e9975a6eb5b337860b00bc8a0b36d1`: still
**54 baseline / 30 audit-setup / 11 helper files**. Baseline/audit method bytes,
Make admission, formatter and release behavior are unchanged. The local audit
overlay now identifies the current reviewed method revision rather than an older
byte-identical adoption.

The clean canonical exporter records the source's committed VERSION as a
`# version` annotation. Verification reports that display version and full
revision with the existing file count and rejects malformed/duplicate annotations.
The commit and declared hashes/modes remain the integrity identity; the version
annotation neither proves publication/provenance nor selects Cargo or tooling.
Keep the new version field with refresh rather than editing it independently.
No fleet-dashboard, LOC report or optional helper is added. The shared hosted
artifact readback repair belongs to its own CI; consumer transport is unchanged.

Exports and source-bound preparation evidence remain at
`/tmp/ic-timers-shared029.00umv319/`. Committed bytes/modes, annotation identity, declared companion closure, all
snapshot integrity records, shell syntax, documentation references, whitespace
and complete locked offline metadata checks pass.
Cargo.toml/Cargo.lock and both pin catalogs retain their incoming bytes; the
released graph already selects Testkit 0.28.0, Metrics 0.3.5, Host 0.10.1 and
PocketIC 16.1.0 with four local members 0.16.4. No contributor test, build, lint,
setup, release, version mutation, staging or commit runs. No function, method or
type is removed, and expected production Wasm/instruction/heap delta is zero.

The one undated **0.16.5** draft is repository-only maintenance. Prefer bundling
it with code-bearing work unless the maintainer explicitly selects publication.
[Shared's exact 0.2.9 run](https://github.com/dragginzgame/shared-tooling/actions/runs/37967008620)
passes Linux portable regression and lint/security; both native macOS jobs
remain queued. Those upstream results and source export do not establish
consumer/native acceptance.
Shared #30's command-line MAKEFLAGS replacement gap remains unchanged. #34/#35
stay open for source-bound product and Make/hook native acceptance; the
[0.2.8 owner](#shared-tooling-028-make-admission) retains the earlier failed and
prepared fixture evidence.

A subsequent latest-source check confirms Shared HEAD still equals this exact
0.2.9 revision with no dirty source to adopt. The pushed 0.16.4 Linux job has now
passed, including the version-preparation correction and actual product/hook
subjects; [the qualification owner](releasing.md#testkit-028-and-host-010-released-graph)
binds those results to the released 0.2.8 consumer and selected library graph.
It does not qualify this pending snapshot refresh or close native acceptance.
No further code change is justified by this review; Shared #30 still owns the
unchanged flag-replacement gap. No new release note is added for routine evidence.

On 2026-10-10, exact Shared 0.2.9 CI also passes both native macOS jobs.
The released **0.16.4** consumer passes all three hosts, including actual hook,
release and product subjects; #34 is closed with source-bound Testkit acceptance.
Those consumer results select Shared 0.2.8, so this pending 0.2.9 export does not
inherit execution qualification. New cc/smallvec/syn lock edits remain separate.
[The graph owner](releasing.md#testkit-028-and-host-010-released-graph) records
native raw logs, distinct canister toolchains and incoming metadata scope.

The original #35 Make/formatter adoption is qualified, but its new
[concise-formatting follow-up](https://github.com/dragginzgame/ic-timers/issues/35#issuecomment-6086711136)
waits for committed Shared 0.2.10. The sibling checkout now has provisional
Make/wrapper/collector changes; no such bytes are copied. When committed, adopt
`run-formatting.sh` with the canonical include, every selected scratch/index
fixture and the local failure collector's `formatting.*` retention together.
Preserve the existing prepared tools, sorter-first ordering, check-only mode,
failing status and complete diagnostics. Shared #30's flag-replacement gap stays
separate. No new implementation or release note is added for routine evidence.

Earlier adoption records retain their source and qualification scope.

## Shared Tooling 0.2.8 Make admission

The authorized 0.16.4 continuation adopts clean committed Shared **0.2.8**
`b2646cde9abbc8861857a4379c683a0c19eba43e` across all three snapshots:
**54 baseline / 30 audit-setup / 11 helper files**. The baseline policy is
unchanged. Add only `make/execution.mk` to the baseline selection; its existing
execution-probe companion is already selected. Never copy the sibling's dirty
post-release exporter or fleet-report changes.

The canonical includes now admit Make before recipes, so ordinary ignore-errors
and non-executing selections cannot hide a prerequisite refusal. Admission
selects the probe beside the actual include and the running Make executable (`MAKE_COMMAND`),
independently of ambient runtime snapshot routing or argument-bearing recursive
`MAKE`. This resolves the two consumer integration cases reported against 0.2.7
on [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30#issuecomment-6083697521).
Keep the consumer's target-specific root and formatter pin bindings. Direct
delivery remains authoritative through a separate global override assignment
and export: GNU Make 3.81 rejects the released combined target-specific
`override export` syntax. The existing environment/command-line delivery fixture
is written to qualify the policy without freezing its syntax. The isolated
release-command smoke checker also binds its disposable root and preserves trailing-newline input paths rather than inheriting a runner
from another snapshot.

All five disposable root-Make callers copy the fourth include and its probe
before their first Make parse. The index-hook fixture stages both; the shared
smoke caller explicitly supplies both. Existing composite recursive Make,
parallel logger, aliased/spaced checkout, external-root and failure-ordering
coverage stays. Added consumer cases require parse-time refusal for unsupported
modes with no release-runner or formatter effects; an external admission sentinel
must never execute. No duplicate local Make flag parser is introduced.

Canonical exports come from a clean detached source clone; isolated helpers
are exported to a disposable consumer before installation. Preparation evidence
is retained at `/tmp/ic-timers-shared028.zqdya2y6/`. Source bytes/modes, declared
companions, snapshot integrity, shell syntax, local documentation links,
whitespace and full locked offline metadata are checked. Cargo manifest/lock and
both pin catalogs retain their incoming bytes. No function, method or type is
removed; timer API/source is unchanged. Expected production
Wasm/instruction/heap impact is zero, without new measurements.

[Exact Shared 0.2.8 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37955525946)
passes Linux portable regression and lint/security; both native macOS jobs are
queued at inspection. Those upstream results and source preparation do not
qualify actual consumer execution. The maintainer operates the new fixtures and
complete gate; no contributor test, build, lint, setup, Make or release ran.
#34/#35 remain open for their source-bound native acceptance. This is
repository-only maintenance in the existing undated 0.16.4 batch; prefer bundling
it with code-bearing work unless the maintainer explicitly selects publication.

Shared #30 now also records an independently executed **remaining** gap:
[clearing command-line MAKEFLAGS](https://github.com/dragginzgame/shared-tooling/issues/30#issuecomment-6084901420)
can hide the actual outer invocation mode from the child probe. The upstream
Metrics reproduction reaches its inert release runner and returns false success
for `-i release-patch MAKEFLAGS=`. Our added consumer negatives preserve invocation
flags and do not prove the cleared-flag case. No canonical payload is patched or
duplicate local parser added; retain this owning upstream limitation and keep
acceptance open. Do not describe the adopted probe as complete unsupported-mode
protection. Consumer fixture execution, portable direct-policy parsing and native
macOS gates also remain pending.

The maintainer's subsequent retained validation passes release-gate execution
and standard adapter checks, then stops in the version-preparation fixture.
The repair base inspected here is `33d902012e71f28c3d6d55a116b638e7d43eed3b`
with incoming Cargo release edits; those edits remain untouched. Retained
`/tmp/timer-version-test.hPIL7K/MAKEFLAGS-i.log` proves the shared include rejected
the first unsafe invocation while parsing. The consumer still required the older
helper's exact refusal phrase and exited silently on that prose mismatch. The
local repair removes that assertion and the obsolete allowance for ignored
recipe failures: unsafe modes now require Make status 2, no release-phase events
and unchanged metadata bytes/modes. A failed status/effect check prints its
retained output. Version-only invocations retain their no-effect contract.
Failure evidence remains at
`.git/release-state/validation-failures/20261009T170629Z-3470085-3-ci.log`.
This is a consumer fixture repair, not a canonical guard change or a fix for the
separate MAKEFLAGS override gap. Shell syntax and whitespace checks pass; no
contributor fixture/test/build/lint/release rerun qualifies the repair.

Earlier adoption records retain their source and qualification scope.

## Shared Tooling 0.2.6 Make adoption

After released IC Timers 0.16.2, the authorized 0.16.3 continuation adopts clean
committed Shared **0.2.6** `ce13a5314916891fd239d9b199b4a91b04775054`. All three
snapshots identify that source: **53 baseline / 30 audit-setup / 11 helper files**.
The four new baseline selections are `make/release.mk`, `make/rust-format.mk`,
`scripts/ci/check-format-tools.sh` and `scripts/ci/test-format-tools.sh`.
The latter two move out of the isolated helper selection, removing their old
payloads and records together. The baseline policy itself is unchanged.

The root Makefile selects the canonical includes. Standard release entrypoints
delegate to the shared runner; a local target-specific override exports direct
delivery even when the environment or Make command line selects PR delivery.
Remove duplicate `.PHONY` declarations for targets now owned by those includes.
Exact-version `release-x`, phase adapters, metadata ownership and complete
validation gates stay local. The release-gate fixture now owns standard delivery
policy, exact argument forwarding, runner failure and conflicting-goal refusal;
its former narrower duplicate is removed from the committed-metadata fixture.
Formatting and standard release entrypoints also override `SHARED_TOOLING_ROOT`
with their current root, preserving their former repository-local routing. The
release fixture supplies an external sentinel through environment and Make
variables; the hook/check fixture supplies an unselected root. Neither may
redirect dispatch. This protects this consumer from the inherited-root leak in
[Shared #7](https://github.com/dragginzgame/shared-tooling/issues/7#issuecomment-6083158846)
without patching the immutable checker; its reusable owner repair remains upstream.

Formatting keeps the same workspace commands, offline prerequisite checks and
explicit cargo-sort setup. Delete the duplicate root `tool-versions.env`;
Make, all four hosted setup sites, update-dev and the index-hook fixture select
the existing `SHARED_TOOLING_CARGO_SORT_VERSION` in `ci/tool-versions.env`.
Formatting binds that exact local pin input, preserving its former consumer-owned
selection even if ambient host setup selects a different versions file. The
hook/check fixture includes both external snapshot and pin selections.
Every disposable copy of the root Makefile gains its three includes. The hook
fixture stages the guard and sole pin catalog along with those includes; the
canonical hook still formats an isolated index and supplies no implicit setup.
This does not activate hooks or add a full Rust-tool installation to setup.

Canonical exports come from a clean detached clone at the exact source revision.
The isolated helper selection is exported to a disposable consumer before copy.
Preparation evidence is retained at `/tmp/ic-timers-shared026.0kili4bv/`.
Cargo.toml, Cargo.lock, `ci/tool-versions.env` and the five-tool IC matrix retain
their pre-adoption bytes. No named function, method or type is deleted: the
formatter guard's `usage` function moves with its unchanged file. Public Make
target names remain. The change removes independently maintained recipes and a
pin owner; it does not claim total physical LOC savings after vendoring includes.

[Exact Shared 0.2.6 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37944389294)
passes Linux portable regression (job 113866777007) and lint/security (113866777285).
Both native macOS jobs (113866776748/113866777469) remain queued at inspection.
Source/export/mode, companion closure, snapshot integrity, shell syntax, document
references, whitespace and locked offline metadata checks are preparation evidence.
They do not qualify consumer formatter execution, setup or native behavior.
The maintainer operates the changed fixtures and normal complete gate; no
contributor test, build, lint, installation, staging, commit or release ran.

The one undated 0.16.3 draft is repository-only. Timer source/API is unchanged;
expected production Wasm/instruction/heap impact is zero, without new measurements.
Prefer bundling maintenance with the next code-bearing release unless the
maintainer explicitly selects a repository-only patch. #34's Testkit acceptance
remains separate from this Make adoption and from the frozen #30 transport results.
The new upstream [outer-Make ignore-errors finding](https://github.com/dragginzgame/shared-tooling/issues/30#issuecomment-6083156831)
also remains open. This adoption does not claim that `make -i` can propagate a
helper refusal: that requires admission at the shared Make boundary, beyond the
existing directly callable runner/hook guards. No copied flag parser or local
replacement framework is introduced.

Earlier adoption records retain their source and qualification scope.

The maintainer subsequently released this adoption in **0.16.3** at
`25957e206fbd351656870e9f87a23c47eed0c015`.
[Matching Linux checks](https://github.com/dragginzgame/ic-timers/actions/runs/37948400741/job/113880561620)
pass all three snapshot checks, standard release smoke/metadata/gate fixtures,
Testkit adapter checks and actual consumer staged-hook formatting/failure isolation.
MSRV and [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37948400870)
also pass. Both native macOS gates remain queued; #35 stays open for those results.
The later incoming Host 0.9.4 lock is outside this exact source. No contributor
test/build/lint or new dispatch ran during this evidence review.

## Shared Tooling 0.2.5 refresh

After released IC Timers 0.16.1, the authorized continuation refreshes the
baseline, audit/setup and isolated helper snapshots together to clean committed
Shared **0.2.5** `04e07b4bf54e7aeb03eb7804a845cee27b7305df`. Selections remain
**49/30/13** files. The reviewed baseline itself is unchanged; AGENTS.md and all
three provenance records identify the same source.

The hook and hook installer preserve trailing-newline checkout paths and literal
`core.hooksPath` values, while failed Git reads still stop setup. Seven selected
test scripts gain explicit required-companion declarations; all companions are
already selected in their owning snapshot, so no fixture roster is expanded.
The cumulative documentation update also clarifies optional installer suites,
upstream-only export integration and bounded CI queue diagnosis. This consumer
does not select the standalone CI installer, its broad suite or upstream
snapshot-distribution fixture. No new tool or CI mode is introduced.

Canonical exports come from a clean detached temporary clone at that exact
revision. The isolated helpers are exported to a disposable consumer before
installation; root snapshots use their existing selections. Source bytes/modes,
all three snapshot integrity records and declared companions are checked.
The incoming Cargo manifest/lock and both consumer pin files are byte-preserved.
No hook is activated and no Git hook configuration is changed. No function,
method or type is removed; hook installer and formatter ownership remain shared.
Preparation evidence is retained at `/tmp/ic-timers-shared025.3v_vh3a0/`.
After export and the first metadata check, a concurrent update selected Host
0.9.3 in the root lock. That incoming graph was preserved and checked separately;
it does not relabel the earlier 0.9.2 metadata evidence.

[Exact Shared 0.2.5 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37937705371)
passes Linux portable regression and lint/security, with both native macOS jobs
queued at inspection. This source/integrity/syntax preparation does not qualify
actual consumer hook setup, staged formatting or native platform behavior.
The maintainer-operated gate retains those obligations; no contributor test,
build, lint, installation, commit or release execution ran for this refresh.
The undated 0.16.2 notes contain repository-only maintenance, with no timer API
or production Wasm/instruction/heap change expected. Prefer bundling it into the
next code-bearing release unless the maintainer explicitly selects publication.

Earlier adoption records retain their source and qualification scope.

## Shared Tooling 0.2.0 hard cut

The maintainer explicitly authorized the latest Shared update and ownership hard
cut on 2026-10-09. All three snapshots now select committed
`06b2e22f6bd213f1a590eb2a8797aee34c42dd69`, 0.2.1 (VERSION 0.2.1): **49 baseline / 30 audit-setup / 13 helper files**. Canonical exports
come from a clean isolated checkout; three retired PocketIC records and payloads
are removed from the audit selection. Consumer IC pins are not an export: their
three PocketIC rows are removed locally. The source also fixes IC pin processing without a final newline, while retaining
exact caller bytes; the canonical fixture covers all three host selections. It
also supplies the 0.1.38
single-document exception fix and 0.1.37 checkout-local hook PATH repair.

The pending batch becomes **0.15.0**, superseding the undated 0.14.24 draft,
because the documented server provisioning/override contract is removed. Cargo
versions and dependencies are not changed by this work. The timer API, private
provider and fresh-server/instance topology are unchanged; expected production
Wasm/instruction/heap impact is zero.

The local `scripts/dev/testkit-server.sh` reads exactly one registry Testkit
selection from the root lockfile, then selects only its published owner CLI via
the shared Cargo installer, release profile. Explicit setup prepares the CLI and
owner server; check verifies the CLI receipt/bytes and owner server offline.
No global-tool fallback, local server catalog/hash policy or implicit check-time
setup remains. Make/CI/update-dev use this adapter. Release preparation runs
setup before offline admission; watchdog/cohort commands recheck after builds
and pass the returned path. Failed CLI builds and provisioning attempts remain
collectable without selecting admitted server payloads.

Deleted files contain **621 physical lines**: local downloader (95), local
verification fixture (288), shared alignment (63), shared binary checker (19)
and shared dedicated fixture (156). Removed named shell functions are `cleanup`
from the downloader; `usage` from alignment; `expect_failure` from the shared
fixture; and `expect_events`, `assert_cache_preserved`, `reject_without_execution`
from the local fixture. Their former pin/provisioning policies belong to Testkit;
the new adapter tests only lock selection, setup/check separation and propagation.
No Rust function, method or type is removed. The release orchestration fixture
retains its actual gate/fail-fast checks and replaces old override cases with
explicit setup/offline check. The download verifier's synthetic IC matrix is
updated to the five-tool shape; frozen #30 runs retain their old source verifier.

Source-bound qualification remains pending. Testkit's published 0.25.4 owner
setup/check/managed-launch contract passed [all native hosts](https://github.com/dragginzgame/ic-testkit/actions/runs/37901828971).
An incoming maintainer update now selects 0.26.0; consumer CLI compilation/startup and native
acceptance at that selection are required. [Earlier Shared 0.2.0 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37916384666)
passes Linux portable regression and lint/security; both macOS jobs are queued.
The selected installer has [Linux production qualification](https://github.com/dragginzgame/shared-tooling/actions/runs/37915522504)
at 0.1.38, while both macOS hosts remain pending. [Shared #76](https://github.com/dragginzgame/shared-tooling/issues/76)
retains owner/consumer coordination. No all-host acceptance is claimed. Latest exact-source
[0.2.1 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37918240103)
is queued; the 0.2.0 pass does not qualify the final-row fix. Consumer preparation
is recorded on [Shared #76](https://github.com/dragginzgame/shared-tooling/issues/76#issuecomment-6079190668).

Preparation checks pass: source/export/integrity, shell/Python/workflow syntax,
311 local document references/anchors, whitespace and full locked offline
metadata. No contributor manifest/lock mutation ran. During preparation, incoming
maintainer edits advanced the root Testkit requirement to 0.26 and selected
Testkit 0.26.0 / Host 0.9.0 / Metrics 0.2.20. Those changes are retained; the
Timers-used `pic` startup APIs are unchanged. The streaming server check and
cache-process cleanup belong to their Testkit/Host owners. A fresh full locked/offline metadata read resolves one package identity for
each selected dependency and leaves both current inputs byte-identical. It is
graph evidence only; the new native selection still needs the product gate. The new adapter and changed collector/gate
fixtures, real explicit five-tool/Testkit setup, offline refusal and complete
native recovery/cohort gates remain maintainer-owned. Evidence is retained under
`/tmp/ic-timers-shared020.81DJke/`. No installation, tests/builds/lint, dependency
update, staging, commit, version mutation or release execution ran for this cut.

The follow-up cleanup removes only old split-workspace fixture deletion and
retired-directory assertions plus the single-manifest loops in the root checker
and its fixture: **10 net physical lines** in four files. Current failure/status,
full graph and lock-preservation checks remain. No named function, method or type
is removed. The actual cheap root metadata check and syntax/diff checks pass;
Cargo bytes are preserved, and changed fixtures remain unexecuted. Retained
follow-up evidence: `/tmp/ic-timers-cleanup.pPMGOg/`.

The separate selected cargo-sort consolidation stays pending: the canonical
index hook still supplies no original tool-root input for checking a nested
selected installation receipt. Do not patch the shared hook or ship a partial
formatter route. Existing formatter commands are retained, independently of the
complete PocketIC ownership cut above. Earlier reviews below keep their original
source and pending-evidence scope.

## Shared Tooling 0.1.37 review

Reviewed clean committed `dc4fdf0f78928d75b69bbf43b37c690c53a04d1e`
(`VERSION` 0.1.37). It includes the committed 0.1.36 installer repairs and adds
the original checkout's fixed host/IC/Rust bin directories to PATH for staged
formatting and the formatting-adoption checker. Staged source/configuration
isolation is preserved. This addresses Shared #85's fixed-bundle lookup.

The authorized selected cargo-sort adoption remains pending: the registry mode
uses a nested installation slot, and the hook still exposes no original tool-root
input for its offline receipt checks. The already-posted
[consumer requirement](https://github.com/dragginzgame/shared-tooling/issues/65#issuecomment-6077815113)
therefore remains relevant. Both 0.1.36 and 0.1.37 normal CI are queued at review;
no separate native production-installer qualification is listed for either.
Keep the qualified snapshots and active formatter callers unchanged. No tests,
installer, dependency update, release draft or workflow dispatch runs here.

Subsequent issue review adds [Shared #86](https://github.com/dragginzgame/shared-tooling/issues/86)
to refresh prerequisites: its canonical dependency checker validates the last
exception JSON document but consumes the first, allowing malformed concatenated
input to suppress a finding. Our nested helper has the same parser; the actual
consumer exception catalog is a single document. Repair and qualify this at its
shared owner, then adopt exact bytes rather than patching the consumer copy.

## Shared Tooling 0.1.35 review

Reviewed clean committed `be550afa57fe9e16872e5110b5cd69c24b4fa9e8`
(`VERSION` 0.1.35). Its network/cache policy distinguishes authorized locked
preparation from offline validation and respects explicit caller offline settings.
Timers already uses a Bash release adapter and locked fetch before offline
metadata checks; no compiled-adapter launcher or preparation change is needed.
Local user-owned validation, dependency metadata and release exceptions remain.

Defer the three-snapshot refresh. The new selected Cargo binary/example branch
does not recheck shared ancestors after Cargo returns, unlike its fixed-bundle
branch. `selected_paths "$stage"` checks the final directory and children, not
the shared build ancestor or selection slot. The independently reproduced gap
and smallest owner correction are already recorded in
[Shared #65](https://github.com/dragginzgame/shared-tooling/issues/65#issuecomment-6077295387).
Source inspection confirms that omission; no reproduction/test ran in Timers.
Keep the current qualified 49/33/13 snapshots unchanged rather than patching
immutable payloads or introducing a caller for the new mode.

[Exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37904190217)
has passed Linux portable regression and lint/security; both macOS jobs are still
running at review. Passing existing fixtures alone cannot close the admission
gap. Review a committed owner correction and its native evidence before adoption.
This review creates no release draft, installer execution or dependency update.

### Authorized 0.14.24 formatter consolidation

The maintainer authorized consolidating cargo-sort setup on 2026-10-09. Prepare
one local entry point selecting only `cargo-sort 2.1.4`, binary `cargo-sort`,
release profile, through the shared installer. Developer setup and all four
hosted installation sites must call it after host prerequisites are prepared.
Retire the five direct `cargo install` recipes together. Do not install the
unused cargo-sort-derives/candid-extractor bundle or change the formatter pin.

Offline formatter admission must check the selected installation receipt/bytes
before the existing exact version and rustfmt checks. Trace Make formatting,
release metadata checks and the shared index-snapshot hook together: that hook
formats an exported tree with no ignored `.tools` directory, so the prepared
tool root must remain explicitly bound to the invoking checkout. A per-CURDIR
lookup alone would break staged formatting. No global-tool fallback should hide
a missing or changed managed installation.

Shared **0.1.36** is now committed at
`1af63d31942448a46ef42553285176b98dce9274`. Source inspection confirms the three
owner corrections: post-Cargo ancestor admission, single-document receipt
admission and propagation of Cargo's original failed status. See the
[owner's repair and qualification record](https://github.com/dragginzgame/shared-tooling/issues/65#issuecomment-6077706862).
The corrected production installer still needs its separate native registry-install
qualification; normal 0.1.35 portable CI does not supply it. The index-export tool
root requirement is reported in [consumer feedback](https://github.com/dragginzgame/shared-tooling/issues/65#issuecomment-6077815113).
The pending shared hook's fixed-bundle PATH enhancement is not an input for
receipt-checking the new nested registry slot. Keep active Timers commands and
snapshots unchanged until those prerequisites are met. Then implement
the callers and their focused fixture together, prepare the undated 0.14.24
changelog, and leave consumer validation/release execution maintainer-owned.

## Shared Tooling committed 0.1.34 follow-up

Released **0.14.23** selects committed
`3d33cd250fcae7dbe5cabe44b2abd6b2c91a1822` through all three snapshots:
**49/33/13** files. Its commit is labelled 0.1.34; committed `VERSION` is 0.1.33.
[Exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37900620129)
passes Linux, Intel macOS, Apple Silicon and lint/security. Canonical exports
come from a clean detached temporary clone; dirty sibling changes are excluded.
The local command-authority exceptions and consumer-owned pins remain.

Consumer acceptance is now complete at released
`10a392f98d42701959d0c1d2deddfbef5c96144a`:
[main CI](https://github.com/dragginzgame/ic-timers/actions/runs/37904586955) and
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37904587019)
pass. Inspected Linux, Intel and ARM host logs confirm snapshot export checks,
shared Make tool commands and local cloc fixtures. #33 is closed with this source-bound
acceptance; the earlier pending qualification below describes preparation.

For [#33](https://github.com/dragginzgame/ic-timers/issues/33), caller inspection
finds no product owner for the fleet tooling inventory. Remove only
`scripts/dev/cloc-tooling.pl`, `scripts/ci/test-cloc-tooling.sh`, their two baseline
records, the local CI invocation and README/Make help advertisements. The removed
files total **405 code LOC** (279 Perl, 126 shell; cloc 2.10), or 434 physical
lines. Local `make cloc`, the pinned cloc executable, setup/check commands,
checksums and snapshot verification remain. The immutable shared include retains
its optional `cloc-tooling` target: without an explicitly selected reporter it
explains that fleet reports belong in Shared Tooling and exits before invocation.
Its existing command fixture now covers both omitted and selected reporters.
Shared procedure/host guides can still describe optional upstream commands;
they do not activate a fleet scan in this consumer.

The refreshed alignment helper fixes
[Shared #82](https://github.com/dragginzgame/shared-tooling/issues/82) at its owner:
relative operands are anchored before directory resolution, a sentinel preserves
newline-ending directory names, and a lost directory stops before Cargo starts.
The existing upstream fixture covers CDPATH, spaces, a leading dash, final
newlines and the disappearing-directory refusal. Keep the local absolute-path
Make caller and its space/symlink release-gate fixture. No snapshot payload is
patched, no gate is weakened, and no extra parser or runtime path is introduced.

Host review is at released **0.8.8**
`ccfd7724dd31c14cfbb8ae434f683babfeabf906`;
[CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37901415314)
passes native Linux, both macOS hosts and MSRV. Its new direct-child constructor
and bounded no-follow hashing belong to callers that own foreground processes or
artifact hashing. Timers still delegates managed server startup to Testkit and
has neither caller. No direct Host dependency or adapter is added. Incoming lock
edits select Testkit 0.25.4, Host 0.8.8 and Metrics 0.2.17; they are preserved
separately from this tooling cleanup and require their own graph qualification.

Preparation passes all three snapshot integrity checks, exact selected source
bytes/modes, shell syntax, 282 documentation references/anchors, whitespace and
full locked/offline metadata for the incoming graph. Evidence is retained under
`/tmp/ic-timers-01423.KPKxjw/`, including the clean source, original
pin/manifest/lock bytes and removed-file LOC inputs. Cargo manifest, incoming
lock and caller pin files remain byte-identical to their preparation inputs.
Local fixtures/builds/lint and changed caller native qualification remain
maintainer-owned. #33 stays open until those
callers are qualified. #30's compact hosted upload/download measurements remain
separate. There is no timer API, Wasm, instruction or heap impact from this tooling
selection. No Cargo/version/release mutation runs during contributor preparation.

## Shared Tooling committed 0.1.32 follow-up

The compatible **0.14.22** draft now selects reviewed committed Shared
`635a39a9dd5f8d021fa9c9196b591e00521a7e02` through all three existing
**51/33/13** exports. The commit is labelled 0.1.32; its `VERSION` remains 0.1.31.
Remote main matches, and [exact-source CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37893234402)
passes Linux, Intel, Apple Silicon and lint/security. A clean detached temporary
clone supplies the canonical exports; dirty sibling dashboard changes are excluded.
No selected files are added. Consumer IC/host pins, local command exceptions and
the complete release gate remain; no Cargo-install qualification, dashboard,
fleet report or scheduled agent gains a local caller.

The installer and its canonical AWK admission now reuse the complete validated
IC selection across comments and row order, preserving exact original receipt
bytes and all checksum/version checks ([Shared #79](https://github.com/dragginzgame/shared-tooling/issues/79)).
The already-pending compact verifier therefore delegates receipt record admission
to that same AWK owner instead of requiring byte-identical installed/caller pins.
Caller-pin evidence must still match the selected source exactly. Collector and
download fixtures cover distinct equivalent receipt provenance plus invalid,
changed and duplicate selections. This replaces the pending byte comparison;
there is no second pin schema or receipt rewrite.

Local CI adopts the pushed-source retention from
[Shared #80](https://github.com/dragginzgame/shared-tooling/issues/80): push groups
include the SHA, and only superseded PR revisions cancel. Explicit manual
observations additionally use their run identity, preserving frozen attempts and
early/late separation. Job contents, permissions, pinned Actions and gates stay
the same. The refreshed fleet policy keeps inventories in Shared Tooling and
measurement arithmetic in IC Metrics. The documented eventual PocketIC handoff
is a prerequisite, not an implemented replacement; current audited admission and
provisioning remain. Shared #82's alignment path defect is not fixed by this
revision. The local Make caller now passes explicit absolute manifest/pin paths,
preventing inherited CDPATH output from corrupting normal-checkout resolution.
The helper's general relative/newline path repair remains upstream; this caller
change does not qualify arbitrary newline-ending checkouts or patch the snapshot.

Host review is at released **0.8.5** `1cad3253096b6eb67be5187209e7fb606593c501`;
[its CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37893479726)
passes. Its Rust implementation is unchanged from 0.8.4. The incoming root lock
already selects all four 0.8.5 packages through Testkit 0.25.3, so no direct Host
dependency or private process/lock adapter is added. Managed server ownership
stays with Testkit; live output observation and artifact lock policy belong there.
The two trusted local probe-file reads do not establish a new Host requirement;
an arbitrary byte limit or loader wrapper is deferred without a demonstrated need.
No sibling files, Cargo manifest or lock are edited by the contributor.

The released 0.14.21 lock actually selects Host 0.8.4, not the stale 0.8.2 handoff
label; current source-bound host records are corrected. Incoming Metrics/TOML
and Host selections need their own complete graph qualification. This batch has
no timer runtime/API change or expected timer Wasm/instruction/heap delta from
the tooling; source/syntax/metadata inspection does not prove the dirty native
gate. #30 retains compact consumer acceptance. No named function, method or type
is deleted. Actual preparation checks pass: all three independent integrity
checks, exact source bytes/modes/overlaps, selected shell and existing Python
syntax, workflow YAML parsing, 264 local references/anchors, full locked/offline
metadata and whitespace checks. Root manifest, incoming lock, caller pins and
timer source are preserved. Evidence is retained under
`/tmp/ic-timers-shared032.lQrZvb/`; tests/builds/lint, installer, release and
workflow execution were not run. Earlier preparation records below keep their
original scopes.

## Compact consumer evidence preparation

The compatible **0.14.22** draft completes the remaining implementation of
[#30](https://github.com/dragginzgame/ic-timers/issues/30), using the selector
already in reviewed Shared 0.1.29. All three snapshots remain at
`1a54fb625d6e47efa64c4384808ecbc87be84e7e`; their payloads are unchanged.
The selected `select-tool-evidence.sh` is byte-identical to the owner at
`db039347d2372b877c1c46dcdd2b5c3aa9412009`, whose
[complete CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37787910279)
passes full/compact native upload/download and retained-byte/receipt verification
on Linux, Intel and Apple Silicon. Shared #66 is closed. This permits consumer
implementation; it does not qualify the new dirty collector.

The local collector replaces its inline host/IC wildcard selection with a direct
`select-tool-evidence.sh compact` call and the consumer's two pin files. Selection
output is captured before reading NUL-delimited root/path pairs, so a producer
failure aborts collection and retains its partial output and metadata. Fresh
verification admits omission of that exact active bundle; failed, changed,
unselected and unmanaged bundles stay full. Pins, check logs and IC receipts are
archived under `tool-evidence/`. Product identity, fixture/release/validation log
roots, output refusal, original outcome and upload/download naming remain local.
The shared composite uploader is still not selected; no snapshot refresh, new
retention mode or private verifier is added.

The actual collector fixture now creates tiny authenticated host and IC payloads
for both successful omission and corrupt active-set retention. It checks forwarded
pins, IC receipts, exact selection identity, original logs and job identity, then
injects a failed selector with partial output and requires no archive. Existing
empty/early, newline/CDPATH, mode/symlink, partial-tar and occupied-output cases
remain. Copied consumer scripts include the selector's installer/checker
companions. The existing downloaded-archive verifier also requires late
qualification's compact selections, exact caller pins/IC receipts and absence of
the verified payloads. Its rejection fixture covers missing/changed evidence and
retained active payloads; earlier identity/status/mode checks remain. No new Python
tool or prerequisite is added. These are synthetic fixture inputs, not qualification of the real
release assets or a compression/time benchmark. No named function, method or type
is removed; only inline root selection is replaced.

The earlier release obligations are now accepted separately:
[0.14.20 CI](https://github.com/dragginzgame/ic-timers/actions/runs/37807464532)
and [0.14.21 CI](https://github.com/dragginzgame/ic-timers/actions/runs/37819730920)
pass Linux/MSRV and both complete native macOS gates. All three 0.14.21 host logs
report passing actual release-index, runner and installer/evidence fixtures;
matching [tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37819731178)
passes. #31/#32 are closed. Earlier cancelled upstream and uncached offline
observations remain historical. All six frozen 0.14.17 round trips are also
complete at their [evidence owner](releasing.md#evidence-path-repair-and-01417-qualification).

Consumer compact qualification remains open on #30: user-operated fixture and
full native gates, then source-bound early/late hosted transport at the eventual
committed collector and measured actual archive bytes/time. No new dispatch or
local test/build/lint ran. Production timer source/API, Cargo manifest and IC pins
are unchanged. A later external lock edit selects Metrics 0.2.16, TOML 1.1.8 and
toml_parser 1.1.5 (and their checksums); it is preserved separately and requires
qualification of that graph. No contributor dependency update or lock edit ran.
The selected graph's cheap full locked/offline metadata inspection completes;
the JSON is retained at `/tmp/ic-timers-01422-metadata.json`. Preparation syntax,
documentation references, exact qualified-selector bytes and diff checks remain
separate from behavior qualification.
The collector work alone is repository-only, with zero expected timer
Wasm/instruction/heap delta, and can remain untagged until a code-bearing release.

## Shared Tooling 0.1.29 adoption

After pushed 0.14.20 (`40611eff87b3165e58528c597debaa95427cdf5a`), the compatible
**0.14.21** draft selects committed
[`1a54fb625d6e47efa64c4384808ecbc87be84e7e`](https://github.com/dragginzgame/shared-tooling/tree/1a54fb625d6e47efa64c4384808ecbc87be84e7e)
(0.1.29) through **51 baseline / 33 audit-setup / 13 helper** records.
[#32](https://github.com/dragginzgame/ic-timers/issues/32) owns consumer adoption.
The three canonical exports come from one clean detached scratch clone; the
nested bundle uses the existing isolated temporary consumer method. The one
baseline addition is `scripts/ci/check-release-source.sh`, required by the
refreshed release guide and now directly selected by the local adapter. Later
dirty 0.1.30 Cargo-install qualification is excluded. Shared files are unchanged
committed bytes and executable modes; no shared payload is patched locally.

`scripts/release/adapter.sh` deletes its private **`admit_release_paths`** function
and delegates both admission calls to the shared owner. Its four literal
exceptions remain `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md` and `README.md`.
The helper reports all observed rejected paths with staged/unstaged/untracked
context, preserves unusual names and uses a read-only Git status observation
with optional index locks disabled. Git observation failure remains distinct
from dirty source. The runner's new message describes only initial preflight
for that attempt; later preparation/reconciliation does not claim that no prior
validation ran ([Shared #74](https://github.com/dragginzgame/shared-tooling/issues/74)).
Metadata checks, exact committed-source checks, Git effects, full validation and
cache preparation remain in their current owners. No wrapper, alternate path
parser or compatibility checker is retained. This is the only removed named
function, method or type in this batch.

The existing actual-adapter `test-release-index.sh` fixture now checks combined
hidden staged, ordinary working and newline-untracked refusal, exact index/file
byte preservation and refusal before fetch. Fault stubs cover the helper's
checkout/status observations, including partial output, without relabelling errors
as dirty source. Existing metadata allowance, prepared-index and cache preparation
cases remain. These regression changes were inspected and syntax checked; no
fixture or test execution was performed by the contributor.

The coordinated baseline, audit and maintenance prompt update limits standing
issue writes to verified `dragginzgame` repositories. Existing contributor test,
commit and release exceptions remain. Optional npm declarations gain no caller
or Node prerequisite here; the current Cargo/Action gate keeps its selection.
The optional terminal issue dashboard and scheduling are not selected. The
consumer-owned PocketIC 16.1.0 matrix, host pins, full evidence retention and
complete native release gate are preserved. Production timer source/API and
Cargo versions are unchanged. The diagnostic cleanup alone has no expected
timer Wasm/instruction/heap delta and can remain untagged until later code-bearing
work rather than requiring another release. A later external root-lock edit
advances only ic-metrics to 0.2.15 and its registry checksum; that incoming
selection is retained separately and needs fresh dependency qualification.

Exact-source [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37806453080)
passes Linux portable regression and lint/security, but both macOS jobs were
cancelled and the complete run is cancelled. This does not qualify native macOS.
Pushed Timers 0.14.20 has Linux checks/MSRV and
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37807464542)
passes; both native gates remain queued in its
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37807464532).
Those results do not qualify this dirty adapter. All three snapshot integrity
checks, exact source/export bytes/modes/overlaps, selected Bash syntax, 396 local
references across 49 document selections, locked/offline metadata and diff checks
pass. The initial Cargo/lock and IC matrix preservation check passed. The later
external Metrics update is the sole lock difference; the contributor performed
no dependency update or lock edit. A current-graph locked/offline metadata recheck
fails because ic-metrics 0.2.15 is not cached; the exact failed output is retained
in `/tmp/ic-timers-shared029.z2UkaB/metadata-after-metrics.stderr`. No online retry
or dependency fetch ran. The root manifest, IC matrix, production timer source
and published changelog history remain unchanged. The adapter's 4 added/17 removed lines yield 13 fewer
local lines; the shared helper is a 53-line owner, so this is ownership convergence,
not an overall repository LOC reduction. No local test/build/lint,
installation, scheduler, version mutation, stage, commit, tag or push ran.
[#31](https://github.com/dragginzgame/ic-timers/issues/31) retains 0.1.28 native acceptance;
[#30](https://github.com/dragginzgame/ic-timers/issues/30)
retains frozen failure transport/compact scope. Neither obligation is relabelled
by the new draft.

## Shared Tooling 0.1.28 adoption

The compatible **0.14.20** draft selects committed
[`1872ed2c20f6c70689bb2249050b1d673c60bfa0`](https://github.com/dragginzgame/shared-tooling/tree/1872ed2c20f6c70689bb2249050b1d673c60bfa0)
(0.1.28) through all three exports: **50 baseline / 33 audit-setup / 13 helper**
files. [#31](https://github.com/dragginzgame/ic-timers/issues/31) owns consumer
adoption. An isolated clean detached clone supplies exact committed bytes and
modes. The root exports use the canonical refresh helper; the nested bundle
uses its established isolated temporary consumer because it is not a checkout
root. Its 13 payloads are unchanged; only its source record advances. An initial
direct nested refresh refused that boundary without changing the helper bundle.
The sibling's later dirty policy, release-source and dependency-checker edits
are excluded. No shared payload is patched locally.

The host/IC installers now preserve literal active-link bytes before managed
name admission. Existing fixtures cover one/two trailing newlines, refusal before
execution/downloads and preservation of the original link
([Shared #75](https://github.com/dragginzgame/shared-tooling/issues/75)). The
consumer's existing `release-check` still invokes the canonical runner fixture,
which now simulates Git effects and permits only inert native hashing. Actual
scratch-Git tracking/race cases remain in the separate Shared owner suite, which
is neither copied nor selected here
([Shared #70](https://github.com/dragginzgame/shared-tooling/issues/70)). Those
scenarios moved upstream; no named function, method or type is removed here.

The refreshed baseline/audit catalog references the new `tasks/` procedures,
prompt and optional scheduler guidance, so their complete linked closure is
included. Copying the coordinator and sample units starts no agent and installs
no service or timer. The local validation, contribution and release exceptions
apply to every task. New MSRV guidance retains the current 1.88.0 floor: the
selected normal `ic-cdk` dependency also declares 1.88.0, and our minimum lane
already selects that compiler explicitly. The production Wasm memory intrinsic
uses its equivalent `std::arch` path under the refreshed Rust hygiene rule;
there is no timer semantic or public API change.

The maintainer reports released **0.14.19** live at `e637224`; its selected graph
is Testkit 0.25.1 / Host 0.8.2 / Metrics 0.2.14 / PocketIC 16.1.0. The refresh
preserves the root manifest, caller host pins and the consumer-owned IC matrix.
During preparation an external lock edit advanced only Testkit to 0.25.2, with
its new registry checksum. That edit is retained; this contributor changed no
Cargo or lockfile bytes. The initial cheap locked/offline metadata inspection
completed for the released graph. A current-graph recheck failed because registry
Testkit 0.25.2 is not cached; no online retry or dependency preparation ran.
The strict PocketIC admission, full release targets, original failure collector
and full tool-bundle retention remain. No measured or expected production
Wasm/instruction/heap improvement is claimed for this batch.

Exact-source [upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37799837183)
passes Linux and Apple Silicon portable regression plus lint/security; Intel is
queued at inspection. This is incomplete native qualification and does not prove
consumer behavior. Source/export/mode/overlap inspection, all three snapshot
integrity checks, Bash syntax, 395 local references across 49 document selections,
manifest/IC pin preservation, published changelog preservation and diff checks
pass. The later metadata cache failure remains a separate limitation. No local
tests, builds, lint, installation, scheduler,
version mutation or Git delivery ran. Native consumer qualification remains
with the complete user-operated gate. [#30](https://github.com/dragginzgame/ic-timers/issues/30)
retains the separate archive/compact acceptance scope.

## Consumer-owned PocketIC 16.1.0 matrix

The 0.14.19 admission repair transfers the existing `ci/ic-tools.tsv` out of the
immutable audit/setup export. A fresh scratch consumer selects the same reviewed
`db039347d2372b877c1c46dcdd2b5c3aa9412009` with the previous roster minus that one
file through the canonical distribution helper; all remaining payloads compare
identically before the exported manifest is installed. Counts are now **36/33/13**,
with unchanged revision and matching overlaps. No manifest digest is hand-edited,
shared implementation patched, sibling mutated or second matrix introduced.

The local matrix now owns the product-selected PocketIC 16.1.0 archives; all
other tool records are preserved. Shared setup/admission helpers still receive
that same explicit matrix. The upstream default guide/fixture may describe a
16.0.0 synthetic/default set; those immutable bytes do not qualify or select the
consumer's runtime pair. Binary hashes and the independent host/URL fixture stay
local. See [artifact provenance and qualification limits](releasing.md#pocketic-artifact-pins).

This ownership change prevents a later snapshot refresh from silently replacing
product-reviewed pins. The strict PocketIC version/hash and complete native gate
remain; fresh runtime qualification is pending and no production timer change is
claimed. No named function, method or type is removed.

## Shared Tooling 0.1.27 committed follow-up

The undated compatible **0.14.19** draft selects reviewed committed follow-up
[`db039347d2372b877c1c46dcdd2b5c3aa9412009`](https://github.com/dragginzgame/shared-tooling/tree/db039347d2372b877c1c46dcdd2b5c3aa9412009)
(VERSION remains 0.1.27). All three exports retain **36/34/13** files and matching
overlaps. A clean detached scratch clone supplies committed bytes; the sibling's
uncommitted 0.1.28 draft is excluded. Baseline rules, local command exceptions,
caller pins, exact PocketIC alignment and production timer code are unchanged.

The host/IC/Rust installers now preserve literal physical consumer operands;
the IC installer also anchors and preserves caller-selected pin paths. Existing
host/IC fixtures check relative consumer operands under inherited CDPATH. Their
second-line companion declarations now make incomplete fixture exports refuse
before replacement; the already-complete consumer roster needs no additions.
The reviewed helper guide now documents the compact selector's pin/check/race
contract. No named function, method or type is removed.

The upstream workflow oracle now compares all four actual IC/Rust producer logs.
Exact-source [run 37787910279](https://github.com/dragginzgame/shared-tooling/actions/runs/37787910279)
passes Linux portable fixtures, native full/compact upload/download and final
pin/receipt/candidate/log comparisons, plus lint/security. Intel is running and
Apple Silicon is queued at inspection. This supersedes b866's stale-log failure
only for db039; it does not supply complete native acceptance. Ordinary consumer
CI retains full bundles and its existing collector. No archive savings are claimed.
[Shared #75](https://github.com/dragginzgame/shared-tooling/issues/75) separately
owns literal active-link admission; these committed installers still contain the
reported newline-stripping capture. The selector's literal-byte guard is distinct
and the finding is not a claim of archive loss. No immutable payload is patched.

Source/export bytes/modes/overlaps, all three integrity checks, syntax and diff
checks are preparation evidence. No local tests/builds/lint, installation or
release effects ran. Existing explicitly authorized hosted observations remain
source-bound in the [evidence owner](releasing.md#evidence-path-repair-and-01417-qualification).
The snapshot follow-up alone has no expected production Wasm, heap or timer
instruction delta and does not justify a runtime feature or hard cut.

## Shared Tooling 0.1.27 preparation

The compatible undated **0.14.18** draft selects reviewed committed source
[`b866d41041a1986eeec95bde9af4c6ba0853d2e3`](https://github.com/dragginzgame/shared-tooling/tree/b866d41041a1986eeec95bde9af4c6ba0853d2e3)
(VERSION 0.1.27). A clean detached scratch clone supplies all three exports:
**36 baseline / 34 audit-setup / 13 helper** files. Overlapping payloads agree.
The baseline rules and local command-authority exceptions are unchanged. Host/IC
pins, strict PocketIC admission and the complete user-operated gate remain.

The refresh carries anchored physical shell bootstrap, the dependency checker's
checkout-root admission, release/validation entrypoint repair and dotted
`.shared-tooling*.snapshot` LOC recognition. Existing host/IC fixtures now call
`test-tool-evidence.sh`, which selects the shared selector, neutral archiver and
actual composite-action collection step. Those four exact files are added to the
audit export; the archiver already selected in the baseline is recorded identically.
This is required fixture closure, not adoption of the action in consumer workflows.
Neither the composite uploader nor compact selection is selected in ordinary
consumer CI. The local collector retains every selected/unselected tool bundle.

The consumer evidence-path fixture replaces its temporary installer/logger
substitutes with the repaired real owners and their companions, checking both
controlled failure statuses and retained download/log bytes. No named function,
method or type is removed. Public timer behavior and production Wasm, instruction
and heap costs are unchanged. The incoming Testkit graph remains separately
owned in the [dependency record](releasing.md#testkit-024-selection).

All three exported integrity checks, source byte/mode/overlap inspection, syntax
and diff checks are preparation evidence. No tests, builds, lint, installation,
version mutation or Git delivery ran. Exact-source
[upstream 0.1.27 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37778118837)
passes lint/security and the Linux portable regression set, including the actual
synthetic selector/action fixtures and path-bootstrap cases. Its Linux native
compact round trip fails at a stale `portable-regression.log` comparison after
selection/pin/receipt/candidate assertions: the producer now writes separate
IC/Rust install/check logs. Both native macOS jobs are queued. This is not complete
compact-policy acceptance. [Feedback is recorded on Shared #66](https://github.com/dragginzgame/shared-tooling/issues/66#issuecomment-6060214674); compact retention
remains deferred, with no size or collection-time saving claimed. The
[consumer evidence owner](releasing.md#evidence-path-repair-and-01417-qualification)
keeps the already-dispatched frozen 0.14.17 observations separate from this draft.
Local [#30](https://github.com/dragginzgame/ic-timers/issues/30) remains open through
adapter acceptance and the compact-selection obligation.

## Shared Tooling 0.1.26 preparation

The incoming **0.14.17** draft selects reviewed committed source
[`75a8a60f49cec11d3f6aecab5c977029c42cc549`](https://github.com/dragginzgame/shared-tooling/tree/75a8a60f49cec11d3f6aecab5c977029c42cc549)
(VERSION 0.1.26). A clean detached scratch clone supplies all three exports.
The baseline/audit/helper manifests now select **36/30/13** files at that one
revision; overlapping payloads agree. The single explicit file-set addition is
`scripts/ci/archive-evidence.sh`. The nested helper export uses an isolated
temporary consumer. The refresh helper's uncommitted-export admission is used
only for those exact previously recorded bytes; no sibling files are changed.

The release runner refreshes the configured upstream after confirmed direct
delivery or completed resume, checking ref type under Git's update lock. The
committed correction preserves a concurrently installed symbolic ref even if it
resolves to the captured old OID ([shared #62](https://github.com/dragginzgame/shared-tooling/issues/62)).
LOC reporting includes `bin/` and explicitly identifies unborn repositories
([shared #61](https://github.com/dragginzgame/shared-tooling/issues/61)). Companion
declarations add no implicit file selections. The approved maintainer-owned
validation/commit/release exceptions, explicit direct delivery, native gates,
local pins and strict PocketIC admission remain.

The local collector delegates generic tar mechanics to the new shared helper.
The [consumer contract and qualification](releasing.md#shared-failure-archiver-adoption)
remain locally owned. No named function, method or type is removed; only the
collector's inline tar option assembly is replaced. Existing Python verification
is adjusted in place; no new Python tool or prerequisite is introduced.

All three exported integrity checks pass during preparation. Shell/Perl/Python
syntax, diff and cheap full locked offline Cargo metadata are the only other
local checks; no tests, builds, lint, installation, release or Git delivery ran.
Source-bound upstream archive acceptance at
[`eeb72e7`](https://github.com/dragginzgame/shared-tooling/actions/runs/37762726615)
passes all three native hosts. The subsequent 0.1.25 source `672ab4b` passes
Linux, Apple Silicon and lint/security, but its Intel job was cancelled.
The exact selected [0.1.26 run](https://github.com/dragginzgame/shared-tooling/actions/runs/37770856593)
is queued at inspection. Neither earlier archive acceptance nor this source
review establishes native qualification of the new runner/exporter or this
consumer adapter. Keep #30 open through consumer acceptance and preserve the
closed #23 observations at their original source. The upstream/local acceptance
gaps are explicit; no production timer Wasm/instruction change is expected.

## Released 0.1.23 adoption

Released compatible repository-only **0.14.14** adopts reviewed committed
Shared Tooling 0.1.23
[`0ba0ad00ed94848e54ecc82629b6b7873b7284c0`](https://github.com/dragginzgame/shared-tooling/tree/0ba0ad00ed94848e54ecc82629b6b7873b7284c0).
The [baseline snapshot](../.shared-tooling.snapshot),
[audit/setup snapshot](../.shared-tooling-audits.snapshot) and
[helper snapshot](../.shared-tooling/helpers/.shared-tooling.snapshot) select
35/30/13 exact files at that same revision. A clean detached scratch clone
supplies every export; moving sibling work is excluded. The
nested bundle is exported through its own isolated temporary consumer.
During contributor preparation, no sibling, staged path, Cargo version or
incoming dependency lock was changed by this refresh. Released 0.14.12 adopted the preceding 0.1.19 snapshot;
its historical evidence remains scoped below.

[AGENTS.md](../AGENTS.md) retains the product overlay and approved command
exceptions, including maintainer-owned contribution commits. Standard Make
release commands select direct delivery explicitly; the reviewed PR helper is
included without selecting that workflow. Timer runtime, the root catalog, library-only default builds,
private provider and audited PocketIC artifact remain. Optional Rust tooling
is still separate from aggregate setup; the disk-space helper is not adopted
without a consumer capacity requirement.

The maintainer released the batch at
`6fc76e9ffaabad575fe5f044029e6fdd063e4e32`. Its
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37750074305)
passed Linux, MSRV and Apple Silicon; Intel was cancelled before complete
acceptance.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37750074677)
passed. Subsequent released **0.14.15**
`ae26b854a1a473c5d2d153705c2d13570f378d38` preserves every selected snapshot,
Make adapter and fixture. Its
[main run](https://github.com/dragginzgame/ic-timers/actions/runs/37752855158)
passed Linux, MSRV and both complete native macOS gates; matching
[tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/37752855388)
passed. This supplies actual consumer acceptance and closes
[#29](https://github.com/dragginzgame/ic-timers/issues/29#issuecomment-6056613195).
That graph selects Metrics 0.2.11, Testkit 0.21.3 and Host 0.4.6; the incoming
Testkit 0.22 / Host 0.5.1 adoption requires its own consumer gate. Separate
explicitly authorized early/late runs now qualify all six hosted failure
artifacts at released 0.14.15 and close
[#23](https://github.com/dragginzgame/ic-timers/issues/23#issuecomment-6057617574);
see the [evidence owner](releasing.md#hosted-qualification-at-01415). The active
installed-tool overcollection finding remains separate on #30.

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
Consumer acceptance is now recorded on
[#29](https://github.com/dragginzgame/ic-timers/issues/29), separately from the
subsequently completed hosted failure-artifact observations in #23. No function,
method or type was
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
[the former `check-pocketic.sh`](https://github.com/dragginzgame/ic-timers/blob/10a392f98d42701959d0c1d2deddfbef5c96144a/scripts/ci/check-pocketic.sh), replaced by the canonical
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
CI use `cargo-sort` 2.1.4 from the then-selected root `tool-versions.env`
(retired by the [0.2.6 adoption](#shared-tooling-026-make-adoption)).
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
