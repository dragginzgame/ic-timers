# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records the exact reviewed source,
entrypoint roots and all **134** payload hashes/modes. It selects committed
Shared 0.20.0 source `3f87ddc1a0c1616775b2114c42a4adee7376c9b1` through canonical
preview and refresh. Existing roots are retained; the release include adds its
required delivery checker. The unused IC installer fixture entrypoint is retired
through the canonical exporter. Uncommitted supplier work is not part of this adoption.

Common setup and PATH publication cover host/Cargo tools. Timers adds only
Testkit preparation/checking through its ordered Make extensions. No maintained
command selects an IC executable bundle or publishes its PATH. Product Wasm
builds use Cargo through IC Build; recovery and cohort gates remain required.

The consumer-owned [delivery declaration](../ci/release-delivery.txt) admits only
direct release delivery. Common Make admission and the standalone adapter use
the same read-only checker; unsupported PR selections refuse before effects.
Focused consumer fixtures qualify release routing, refusal, hook formatting,
tool setup and failure-evidence integration. They do not establish native macOS,
hosted or complete release acceptance.
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

Shared catalogs remain unchanged. Timers' unused qualified IC catalog is retired;
old IC sets and failed setup diagnostics remain eligible for evidence retention
without selecting or admitting them. Testkit owns its server and compatibility
admission through the CLI selected by the root lockfile. No extra tool-store
manager, fallback installer or implicit setup is introduced.

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

Timers' format and release fixtures use the per-target Make admission helper.
Shared owns exhaustive flag aliases/replacement cases; Timers keeps actual target
rosters, substituted effects, snapshot routing and unchanged worktree/index checks.

Generic nested-workflow discovery and failed Git-inventory cases now run at
Shared's dependency-checker owner. Timers retains a positive/negative caller smoke
check, its own dependency exceptions and actual Make/provider integration checks.
This adoption changes no entrypoint or tool selection.

The Cargo reader owns missing/duplicate tables and invalid version syntax.
Timers exercises reader rejection at its actual bump boundary, preserving status
and metadata. The release-gate fixture owns the complete CI roster and failure
order; repository checks keep their own admission and provider boundaries.

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
