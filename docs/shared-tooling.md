# Shared Tooling adoption

IC Timers adopts reviewed revision [`cb86188c5956866564de4fb6ec6be67b27981ab9`](https://github.com/dragginzgame/shared-tooling/tree/cb86188c5956866564de4fb6ec6be67b27981ab9).
The [baseline snapshot manifest](../.shared-tooling.snapshot) records twenty-one exact files,
including [the baseline](../DRAGGINZGAME.md), linked governance, release and
validation runners and verification fixtures. [AGENTS.md](../AGENTS.md) owns the product overlay and the
maintainer-approved validation and release-authority exceptions.
The supplemental [audit snapshot](../.shared-tooling-audits.snapshot) records nineteen
files from [`a37771f1b6b5fc9a88ed6ab3b705bdda35cd8fa3`](https://github.com/dragginzgame/shared-tooling/tree/a37771f1b6b5fc9a88ed6ab3b705bdda35cd8fa3),
including the six unchanged methods, their adoption/provenance guidance and
linked documents/catalogs and the host-parser installer/fixtures. Its two
principle documents and integrity helpers are byte-identical to the baseline's
copies and recorded in both manifests. The fifteen-file
[helper snapshot](../.shared-tooling/helpers/.shared-tooling.snapshot) records reviewed committed
`b32d3038c850a7c53470c326b0f7f11263b31669`: Cargo/checksum helpers, IC setup
and formatter prerequisite checks. The thirteen earlier helper files are
byte-identical to their original `d957d1f` adoption.
The helper slice lives under `.shared-tooling/helpers/` so its newer checksum
owner cannot replace the checksum required by either older manifest. The
checker/filter and unchanged checker fixture moved out of the audit manifest
into this slice; their former executable paths are removed. Each changed file
has one source revision. The shared snapshot verifier remains byte-identical
across all three sources.

The maintainer-authorized adoption of
[Shared Tooling #6](https://github.com/dragginzgame/shared-tooling/issues/6)
uses an explicit revision-bound reference in [AGENTS.md](../AGENTS.md) to the
[dependency-preparation section at `a7efade1a68e43f148252a1a73908a46c4cbe9e9`](https://github.com/dragginzgame/shared-tooling/blob/a7efade1a68e43f148252a1a73908a46c4cbe9e9/rules/cargo-dependencies.md#preparing-authorized-dependency-changes).
The committed section requires tracing affected independent graphs, preparing
their applicable lockfiles together, preserving unrelated selections and
verifying owning locked metadata before declaring the change complete. It keeps
cache fetching locked and retains local command authority. Source review covers
that committed section and the corresponding release-cache cross-reference;
this is accepted guidance, not a new consumer runtime or host qualification.

The baseline's release/hook/validation executables remain unchanged at `cb86188`.
The later Cargo/IC helper adoption is scoped below. Newer host-installer and
validation-runner changes remain outside it. The structured checker and parser
setup originally adopted at `a37771f` retains its historical record below.
The inspected [upstream CI for `a7efade`](https://github.com/dragginzgame/shared-tooling/actions/runs/37443591873)
passed Linux regression and lint/security; both macOS jobs were queued. No mutable
sibling bytes are authority, and no new checker or download is implicitly introduced here.

A subsequent read-only review on 2026-10-06 found committed sibling HEAD
`47cd2ccaf0e8b428f06e6db0262df76cfc1581de` (0.1.7), with additional dirty work
excluded from that identity. Its
[upstream CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37458968809)
passed lint/security but failed all three portable regression jobs. Linux
stopped in the RustSec preparation fixture after retaining its scratch path;
the available log does not establish the underlying error. Both macOS jobs
reported `release metadata test failed: partial version read passed preflight`
inside the nested validation-runner fixture. No whole-baseline refresh or
qualification is inferred. The consumer snapshots remain at their recorded
revisions, and uncommitted helpers are ineligible for revision-bound adoption.

Refresh through the upstream distribution helper from a clean reviewed checkout,
then verify `bash scripts/ci/verify-shared-tooling-snapshot.sh`. CI and releases
use the offline snapshot; a mutable sibling checkout supplies no authority.
The former document-only copy under `docs/shared-tooling/` is retired.

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

The next compatible 0.14.3 draft adopts the shared formatter guard and its
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
`21f3ec3dd97f2968c9f0b08924451bb2f71770d1` found a source-level precision hazard
that prevents adopting it yet. Its `historical()` first compares split component
values with `a[n] != b[n]`, before using string-prefixed ordering. Numeric strings
from `split()` can compare numerically, so equal-length values above the exact
floating-point range may collapse at that first comparison. See the
[GNU AWK comparison contract](https://www.gnu.org/software/gawk/manual/html_node/Variable-Typing.html)
and the [exact upstream source](https://github.com/dragginzgame/shared-tooling/blob/21f3ec3dd97f2968c9f0b08924451bb2f71770d1/scripts/ci/finalize-release-changelog.awk).
For example, adjacent components `9007199254740992` and `9007199254740993` are
both valid u64 values. The upstream correction should force string comparison
for equality too and qualify adjacent large components on all hosts. That is
read-only review feedback; no upstream file or issue was changed and no local
reproduction was run. The local parser remains the sole active consumer selector;
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
