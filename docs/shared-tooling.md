# Shared Tooling adoption

## Current authority

The [snapshot](../.shared-tooling.snapshot) is the single inventory of reviewed
Shared payloads, source revision and entrypoints. It currently records **134**
payloads from Shared **0.27.3**, revision
`7ee5c83744aa822ecd070703dbf0a7d95076d5f6`. Do not patch exported files; refresh
through that owner's committed exporter and review the closure against local
callers. The [baseline](../DRAGGINZGAME.md) and [local overlay](../AGENTS.md) own
contribution, validation and maintainer-operated release authority.

Historical adoption decisions, before-images and supplier-specific observations
are preserved in [adoption evidence](evidence/shared-tooling-history.md#timers-0322-shared-0273-adoption-record).
They do not add current setup commands, outstanding delivery gates or schedules.
See [current status](status/current.md) for pending work and tested source scope.

## Compiler selection and setup

[Cargo.toml](../Cargo.toml) owns the minimum Rust version;
[rust-toolchain.toml](../rust-toolchain.toml) owns the development compiler.
Make reads the workspace floor through Shared's manifest reader. Explicit
command-line or environment `MSRV` overrides remain available for qualification.
There is no separate compiler-version catalog in Make or CI.

`make update-dev` first prepares host parsers, then reads both compiler selections,
prepares Rust with the required components/Wasm target, and installs/checks the
common and selected Cargo tools before installing the formatting hook. An
explicit `MSRV` override changes its minimum-compiler selection. CI reads the
same files after host preparation and before Rust installation. The tag lane
retains its narrower rustfmt-only preparation. All setup is explicit; ordinary
checks remain offline and never install tools.

Native/compiler/SDK bootstrap must finish before Cargo-tool setup; later changes
require explicit re-preparation. Common and selected tool receipts keep their
existing identity checks. Follow [local setup](local-setup.md) and the
[release guide](releasing.md); earlier immutable tool installations and failed
evidence remain retained.

## Consumer boundaries

Shared owns common host/Cargo tool catalogs, immutable-store admission,
formatting, fixture companion discovery, evidence transport and release metadata
transactions. Timers owns its provider and dependency exclusions, source/tag
and release-impact policy, direct-only [delivery declaration](../ci/release-delivery.txt),
qualification recipes and product evidence.

The root-lock-selected Build editor and Testkit server binaries both select
`cli`; normal workspace library edges do not. Prepare them through
`make install-release-editor install-testkit-server`, then their offline checks.
Testkit owns PocketIC server selection, admission and lifecycle. Timers selects
no IC executable bundle; optional standalone IC tooling requires an explicit
selection and matching setup/check inputs.

Common setup selects Cargo sort and sort-derives. No Candid extractor, optional
scanner, audit schedule, unattended refresh or compiler cache is activated here.

## Dependency roles and formatting

The [root manifest](../Cargo.toml) separates `Production packages` from
`Tests and tooling packages`, and `Production dependencies` from
`Tests and tooling only`. These describe use here, not registry publication.
There is still one catalog and lockfile; member dependency tables determine
actual use through `workspace = true`.

The published `ic-timers` library directly uses Metrics, Thiserror and Platform
(executor and raw IC system bindings). Metrics 0.17 supplies required Serde
transport and native-only Wire metadata transitively. The catalog's direct Serde
and Wire entries serve qualification packages; the deployed Wasm library has
neither Wire nor JSON parsing. Platform's canister runtime, Build, Host FS/Process,
Testkit and the probe protocol serve qualification packages here.
Candid remains transitive only in the native driver through PocketIC/Testkit. Local probe protocols use Wire; Platform's `cdk`
feature and both canister graphs are Candid-free. Full CDK/macros and the retired
Testkit Candid adapter are absent from the workspace graph.
The driver test binary and Testkit share Platform 0.8. The driver selects
`pocket-ic` for direct client imports, including external Principal support.
Both use the same upstream PocketIC and external Principal identities; the
older native Platform 0.7 dependency is removed.
Testkit's native provider fixtures also select Platform's Candid/management
support; those features remain excluded from deployed library and probe graphs. Its
normal coordinator excludes Platform, Testkit, Wire and the probe protocol;
they are dev-dependencies used by the maintained integration tests. The library
and canister graphs exclude native simulator features. Build, Host and Testkit
share the family selected by the root lockfile, with no older parallel Host or
Platform.
Deployed roots have one Platform, executor and IC0 provider each.
The catalog's
local `ic-timers` entry is the probes' dependency on the production library.
The coordinator uses published Host FS for baseline alias admission and
atomic report publication; its test-support feature is dev-only. Testkit's `wire`
feature is also dev-only and supplies native typed call handling. No new resolved
package identity is introduced by these explicit tooling edges.

Shared's Make defaults select `cargo sort --workspace --grouped` for formatting
and add `--check` for independent checks; the duplicate local overrides are removed. Hooks use the same `make fmt` policy
inside their exact index export. All current, commit-preparation and
committed-source release checks select `--manifest-order-grouped`. Those release
checks are read-only, operate on their selected source, and never globally
reorder the two dependency groups. Manifest write behavior is qualified in a
disposable fixture; normal checkout checks retain the authored dependency groups.

## Deployment dependency exclusions

The selected Shared direct-dependency checker `check-cargo-dependency-boundaries.sh`
runs with `--external-ic` in `actions-check`, composing the reviewed common inventory
with Timers' stricter local policy. A real-Cargo fixture proves rejection of an
optional renamed target/build Principal edge absent from the local list. The
local list remains an input to deployment graph checks, so its overlapping
entries are retained for that independent purpose. It also forbids direct Candid/CDK-family,
Platform macro-helper, retired Testkit decoder, IC0 and executor declarations in
every member. Platform owns generator/runtime paths, raw stable passthroughs and
exact upstream selections. There are no consumer exceptions. See the
[published boundary](architecture.md#candid-and-cdk-dependency-boundary).
The checker resolves aliases through Cargo metadata and
checks all normal/build/dev/target declarations. The root has one workspace;
there is no additional maintained workspace or lockfile.

`scripts/ci/check-library-dependency-boundary.sh` owns Timers' policy and calls
Shared's `check-cargo-graph.pl` for the library, both canister probes and the
protocol package, independently for Wasm and the actual Rust host. Library and
probe graphs exclude Candid, full CDK and host tooling. The library additionally
excludes probe packages, Principal, SHA-2/digest and text dependencies. Serde
belongs to Metrics' required numeric transport. Wire metadata is native-only;
the Wasm library additionally excludes Wire, Postcard and JSON parsing. Timers
retains no direct library codec dependency.
The protocol excludes Platform and timer authority. Platform may select only
executor/IC0 for the library, plus `cdk`/`runtime`/`principal` for probes.
The checker derives forbidden feature names from the selected Platform catalog
and delegates dependency-feature-edge refusal to Shared. It retains local
successful-metadata admission of one registry executor 2.0.0 and IC0 1.2.0.
Shared identity counts supplement rather than replace those source/version checks.

Real-Cargo consumer fixtures cover Principal/Candid/default feature leakage, renamed target
and build dependencies, protocol authority exclusion, permitted dev-only Wire,
Wasm JSON rejection and
same-version executor source replacement. Shared owns generic parser and failed-
producer fixtures; those are no longer repeated here. Source symlink checks run
in `actions-check`; package verification admits the prepared archive's license.

**Remaining limits:** role comments and unused catalog entries are not validated.
Feature-edge inspection is not a universal inventory of arbitrary root features;
Platform is a dependency of each selected runtime root here. Graph evidence is
not final deployment admission. Consumers must bind their own target/features,
source/lock, successful builds and final artifact digests. Native simulator Candid
transport remains intentional and outside the deployed-canister policy.

## Consumer qualification selection

Shared qualifies generic parser, formatter, tool-store and release-mechanism
matrices. Timers retains proofs at its actual callers and product boundaries:

| Contract | Timers evidence owner |
| --- | --- |
| Dependency/workflow declarations and product policy refusal | `actions-check`, `test-repository-checks.sh`, `test-library-dependency-boundary.sh` |
| Coordinated metadata, selected release commit and version preparation | Graph admission in `release-check`, `test-committed-release.sh`, `test-release-index.sh`, `test-version-preparation.sh` |
| Release source, impact, tag truth and failure ordering | Classifier, index, release-gate and tag-workflow fixtures |
| Host/Rust/bootstrap and selected tool adapters | `test-rust-preparation.sh`, `test-testkit-server.sh` |
| Hook/index isolation and retained failures | Hook, failure-evidence and fixture-completion checks |

Normal snapshot admission checks the actual export. The tag-workflow fixture
also verifies corrupted exported payloads refuse before graph admission; the
standalone duplicate corruption suite is retired. Shared owns its generic
verifier rejection matrix. Local fixtures copy explicit roots and their declared
companions through `prepare-script-fixture.sh`, retaining only their actual
substitutions and assertions.

The collector delegates selected host-bundle verification to Shared. Timers
retains source/run identity and product paths, preserving failed and unselected
stores without admitting them as active tools. Redirected product parents are
retained as links rather than traversed; incomplete collection remains failure.

## Qualification and refresh

Use the reviewed supplier exporter's impact preview and apply the same explicit
root selection. Source verification compares payload bytes and modes against the
independently selected full commit; routine local integrity checks remain offline.
Retain required companions and remove retired producer-only roots through the
exporter. Do not adopt a moving sibling checkout implicitly.

Current qualification is recorded in the [handoff](status/current.md).
[Historical adoption evidence](evidence/shared-tooling-history.md) preserves the
original commands, dependency graphs and host limits. Linux checks and Wasm
cross-compilation do not qualify native macOS or live IC. Product watchdog and
policy-cohort gates remain required for maintainer-operated release validation.
