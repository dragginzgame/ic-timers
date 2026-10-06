# Shared Tooling adoption

IC Timers adopts Shared Tooling 0.1.11 at reviewed committed revision
[`46c02774a8335cb3949d6f04284c4f53375353c1`](https://github.com/dragginzgame/shared-tooling/tree/46c02774a8335cb3949d6f04284c4f53375353c1).
The [baseline snapshot](../.shared-tooling.snapshot) records twenty-three exact
files, including the paired baseline/maintenance rule, release and validation
runners, checksum owner and linked guides. The supplemental
[audit snapshot](../.shared-tooling-audits.snapshot) records nineteen files:
the six unchanged audit methods, provenance/setup guidance, pin catalogs and
host-parser installer/fixtures. Its overlapping principles and integrity helpers
are identical to the baseline's records. The sixteen-file
[helper snapshot](../.shared-tooling/helpers/.shared-tooling.snapshot) records
Cargo/checksum helpers, IC setup, formatter prerequisites and annotated-tag
admission at the same revision. Its separate directory retains existing caller
paths; no alternate runtime or new helper invocation is introduced.

[AGENTS.md](../AGENTS.md) owns the product overlay and approved command-authority
exceptions. Tests, builds, lint and all release effects remain maintainer-owned.
The independent root/testing workspaces, both lockfiles and audited PocketIC
admission remain local. Optional helpers described in shared guides are reference
material until a separate caller adoption. Refresh through the upstream
[distribution workflow](consuming-snapshots.md) from a clean reviewed checkout;
CI/release validation uses these offline manifests, never a mutable sibling.
The former document-only copy under `docs/shared-tooling/` remains retired.

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
but cannot qualify these new consumer callers. #16 remains open for qualification
of the completed worktree under the maintainer's gate.

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
