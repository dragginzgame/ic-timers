# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records the exact reviewed source,
entrypoint roots and all **127** payload hashes/modes. It selects [Shared PR #217](https://github.com/dragginzgame/shared-tooling/pull/217)
`61ddfdc25ad0f4342b5e324e5798897f16c43996`: committed Shared 0.15 source plus
read-only changelog preflight and verified snapshot handover. This is exact
contribution provenance, not an upstream release or hosted acceptance claim.
The [baseline](../DRAGGINZGAME.md) and [local overlay](../AGENTS.md) govern scope.

The complete [previous adoption record](evidence/shared-tooling-history.md) is
historical. Its earlier bundles, compiler selections, pending states and evidence
remain bound to their original sources. Current work belongs to the compact
[handoff](status/current.md), not a second release queue here.

## Consumer boundaries

Shared owns canonical governance/audit procedures, common tools/store admission,
formatting and release mechanics, structured Cargo/YAML checks, candidate
selection, metadata mutation/rollback and generic evidence transport. Timers owns
its runtime/provider boundary, release source/tag/impact policy, product artifact
identity, probe subjects and IC recovery/cohort qualification. The local
[release guide](releasing.md) and [hygiene overlay](audits/code-hygiene.md) identify
those callers and their authority.

Shared catalogs remain unchanged. Timers' explicitly selected
[IC exception](../ci/ic-tools-qualified.tsv) retains qualified Binaryen 132;
refreshing governance does not qualify another optimizer. Testkit owns its server
and compatibility admission through the CLI selected by the root lockfile. No
extra tool-store manager, fallback installer or implicit setup is introduced.

## Consolidated ownership

The former root/audit/helper bundles contained 146 records for 123 canonical
source paths. Canonical preview and `--takeover-snapshot` transferred the root
supplement and nested helper receipt, proving committed bytes and executable
modes before publication. The new union adds four directly used helper/fixture
roots, yielding 127 unique payloads. Eighteen nested payload copies and two
obsolete manifests are retired; their canonical owners remain. Local Cargo-helper
callers now use `scripts/ci/` directly.

Only one snapshot integrity check and one physical export layout remain. The
consumer corruption fixture still rejects governance, audit and Cargo payload
corruption and untrusted checksum helpers. Selected workflow bodies use stable
step IDs through Shared's complete-input extractor; Timers retains actual-step
execution, ordering and failure assertions. Changelog preflight is read-only;
coordinated Shared preparation is the sole metadata writer.

## Refreshing and qualification

Use the reviewed source's canonical exporter against the repository root:
preview first, then apply the same selection. Existing entrypoints are retained;
select new helpers explicitly after reviewing their callers. No nested temporary
Git export, manual ownership rewrite or generated-file patch is needed. Follow
[the canonical procedure](consuming-snapshots.md) for proof and interruption scope.

Preparation evidence and before-images are retained at
`/tmp/ic-timers-owner-consolidation.eggohebc/`. The handoff records focused native,
MSRV, Bash 5/genuine Bash 3.2 and consumer qualification. Those checks do not
replace full owner/native macOS, hosted upload or live PocketIC acceptance.
Normal Git delivery and package versions remain maintainer-owned.
