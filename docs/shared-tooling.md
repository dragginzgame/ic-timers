# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records the exact reviewed source,
entrypoint roots and all **133** payload hashes/modes. It selects [Shared PR #234](https://github.com/dragginzgame/shared-tooling/pull/234)
`12eb03d28ba8814000639091a44392d08db21657`, stacked on
[#230](https://github.com/dragginzgame/shared-tooling/pull/230),
[#227](https://github.com/dragginzgame/shared-tooling/pull/227) and
[#224](https://github.com/dragginzgame/shared-tooling/pull/224) and building on merged
[#217](https://github.com/dragginzgame/shared-tooling/pull/217) and
[#218](https://github.com/dragginzgame/shared-tooling/pull/218): committed Shared
0.15 source plus changelog preflight, snapshot handover, Rust preparation and
focused verifier qualification, common hosted preparation and Make-fixture setup. This is exact
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
consumer corruption fixture still rejects corruption of actual governance, audit
and Cargo payloads. Generic untrusted-helper rejection belongs to Shared's
verifier fixture. Selected workflow bodies use stable
step IDs through Shared's complete-input extractor; Timers retains actual-step
execution, ordering and failure assertions. Changelog preflight is read-only;
coordinated Shared preparation is the sole metadata writer.

Local setup and hosted Rust lanes call `scripts/dev/prepare-rust-toolchain.sh`.
Timers' `prepare-rust.sh` owns the full development/MSRV component policy;
the tag lane retains its narrower rustfmt selection. `make update-dev` passes
its existing MSRV selection and prepares both compilers before common tools.
Validation targets never invoke setup. The one added installer root yields 128
payloads; canonical preview retains the previous 127 payloads.

Two further entrypoints select generic snapshot and downloaded-evidence owner
fixtures, yielding 130 payloads. Timers keeps actual export corruption, collector
integration, source/run identity and qualified-catalog selection. Generic archive,
receipt and directory/helper rejection cases run at their canonical owner.
Release callers use the Shared stable-version and graph readers directly; the
worktree graph selection still explicitly clears ambient `RELEASE_COMMIT`.

Three more entrypoints select the common hosted preparation action, its execution
fixture and a disposable Make-fixture bootstrap, yielding 133 payloads. Timers
keeps compiler/cache/product policy, local tool extensions and fixture assertions.
The bootstrap copies only the canonical common Make skeleton; each caller still
supplies its own Makefile, catalogs and stubs. Shared's existing Make-consumer
fixture qualifies that layout without a second bootstrap-only suite.

The existing shared evidence action also uploads caller-prepared archives and
collection logs without selecting tools again. Timers keeps product collection,
original status, identity and downloaded-content qualification; Shared owns safe
prepared-file selection, the pinned upload and completeness enforcement. The
private Timers upload action is removed. This changes no entrypoint selection.

The existing fixture exit runner owns injected exits, completion-marker selection
and checked scratch-root retention. Timers keeps its thirteen-source roster,
absolute/relative/aliased scratch contexts and collector diagnostics. Selected
mandatory assertion probes use the original owner guards, including return-based
guards. This also changes no entrypoint selection.

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
