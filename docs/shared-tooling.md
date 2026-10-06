# Shared Tooling adoption

IC Timers adopts reviewed revision [`cb86188c5956866564de4fb6ec6be67b27981ab9`](https://github.com/dragginzgame/shared-tooling/tree/cb86188c5956866564de4fb6ec6be67b27981ab9).
The [baseline snapshot manifest](../.shared-tooling.snapshot) records twenty-one exact files,
including [the baseline](../DRAGGINZGAME.md), linked governance, release and
validation runners and verification fixtures. [AGENTS.md](../AGENTS.md) owns the product overlay and the
maintainer-approved validation and release-authority exceptions.
The separate [audit snapshot](../.shared-tooling-audits.snapshot) records seventeen
files from [`a37771f1b6b5fc9a88ed6ab3b705bdda35cd8fa3`](https://github.com/dragginzgame/shared-tooling/tree/a37771f1b6b5fc9a88ed6ab3b705bdda35cd8fa3),
including the six unchanged methods, their adoption/provenance guidance and
linked reference documents/catalogs. Its two principle documents and both
integrity helpers are byte-identical to the baseline's copies and recorded in
both manifests. The adoption guide's ownership moved from the baseline manifest
to the audit manifest; each changed document has one source revision.

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
The newer commits also introduce declaration pinning, executable setup and
validation-runner changes. Those are outside these narrow adoptions and need
a separate complete baseline review, tool preparation and consumer qualification.
The inspected [upstream CI for `a7efade`](https://github.com/dragginzgame/shared-tooling/actions/runs/37443591873)
passed Linux regression and lint/security; both macOS jobs were queued. No mutable
sibling bytes are authority, and no new checker or download is implicitly introduced here.

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
AGENTS.md scopes their reference-only status. No parser/tool installer, pinning
checker, setup Make target or changed release/validation runner is adopted.
Full `a37771f` baseline adoption remains a separate scope.

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

Refresh the baseline and audit manifests separately from their reviewed clean
source revisions; the custom audit export uses
`--manifest .shared-tooling-audits.snapshot`. Do not refresh either source to a
different revision implicitly. Verify both offline:

```bash
bash scripts/ci/verify-shared-tooling-snapshot.sh
bash scripts/ci/verify-shared-tooling-snapshot.sh --manifest .shared-tooling-audits.snapshot
```

## Release-tooling adoption

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
