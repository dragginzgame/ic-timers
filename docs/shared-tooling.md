# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records **134** canonical payloads
from reviewed Shared Tooling **0.26.7**, revision
`3e8d7cd3e7e9b7b79a5e800c30daf0be2710dab0`. The clean supplier checkout matches
published `main`; VERSION and its committed changelog identify 0.26.7. Canonical
preview/apply retains the existing selection and changes only the selected-tool
installer, its store helper and generic setup guide. Exported files remain
byte-for-byte supplier-owned; retired generic suites stay upstream.

Explicit selected Cargo-tool setup now waits up to 60 seconds for an admitted
slot owner, then rechecks build, source, lock and receipt inputs before reuse.
`SHARED_TOOLING_SETUP_WAIT_SECONDS` selects 0–300 seconds; zero refuses immediately.
Checks/preflight, checkout and source-cache locks, and Host/IC setup remain
immediate. Waiting neither removes foreign locks nor admits partial output. The
build identity policy is unchanged; this refresh alone invalidates no binaries.

This adoption includes explicit IC executable selection, corrected Cargo feature
annotations and stronger retained validation/checksum evidence. Timers selects no
IC executable bundle: its compact collector passes an empty IC selection and
retains legacy IC installations in full. Testkit still owns PocketIC admission.
No optional audit, schedule or generic producer test matrix is activated.

The existing library Clippy gate now precedes compilation, tests and packaging
in `ci`, including the `release-verify` path. The direct `publish` recipe runs
clean-source admission, exact-tag admission, that same lint gate, packaging and
publication as sequential recursive Make calls. Failure stops later stages under
parallel Make. Package/target/feature selection and warning denial are unchanged;
probe lint and real-canister recovery/cohort gates remain required. The local
release fixture substitutes all leaves and external effects, checks the complete
CI/release/publication sequence with `-j4`, and rejects every failed stage.

Failed `check`/`check-graph` observations now retain Cargo stdout (even empty),
streamed stderr, independent Cargo/logger statuses, the selected physical root
and root manifest/lock copies. Cargo failure takes precedence over logger
failure; neither is retried or repaired. Committed-source failures also retain
the exported source tree. Timers' unchanged collector selects CI's existing
`ic-timers-fixtures` root; a local integration fixture verifies the new diagnostic
files reach that archive. Shared owns the generic failure permutations and
fixture dependency discovery.

Shared owns generic graph observation, fixture matrices, tool admission, evidence
transport and metadata transactions. Timers owns package/feature exclusions,
qualified runtime identities, release/source policy and actual integration tests.
IC Build supplies the published metadata editor selected by the root lock; the
consumer's positive document budget is in `ci/release-editor.env`. Prepare it
explicitly with `make install-release-editor`; check it offline with
`make release-editor-check`. Fresh release preflight installs it only after
source admission; standalone preparation requires an already admitted executable.

Selected-tool installation supports explicit Cargo features and binds their
canonical set to immutable selections and receipts. IC Tooling 0.65 requires
`cli` for the editor and Testkit server binaries. Both local setup/check adapters
pass `--features cli`; the workspace library dependencies do not enable it.
Prepare these selections explicitly with `make install-release-editor
install-testkit-server`. Shared 0.26.7 retains selected-tool feature admission
and the generic Testkit setup example passes `--features cli`. Use the local
adapters for both
binaries; a selected package/feature change requires a new receipt.
Package-selection changes still require explicit setup for that selected tool;
checks remain offline and never install or repair it. Old installations and
failed attempts remain retained.

The [baseline](../DRAGGINZGAME.md) and [local overlay](../AGENTS.md) retain
maintainer-owned commits/releases, direct delivery, Testkit server ownership and
separate macOS qualification. This adoption does not activate optional audits or
schedules. Historical selections and receipts remain in the
[handoff archive](status/archive-2026-10-20.md) and
[adoption evidence](evidence/shared-tooling-history.md).

## Dependency roles and formatting

The [root manifest](../Cargo.toml) separates `Production packages` from
`Tests and tooling packages`, and `Production dependencies` from
`Tests and tooling only`. These describe use here, not registry publication.
There is still one catalog and lockfile; member dependency tables determine
actual use through `workspace = true`.

The published `ic-timers` library uses Metrics, Thiserror and Platform
(executor and raw IC system bindings). Serde is qualification-only and belongs
in the tests/tooling catalog group; the library's lean Platform graph excludes it.
Wire, Platform's canister runtime, Build, Host Process, Testkit and the probe protocol serve
qualification packages here.
Candid remains transitive only in the native driver through PocketIC/Testkit. Local probe protocols use Wire; Platform 0.6's `cdk`
feature and both canister graphs are Candid-free. Full CDK/macros and the retired
Testkit Candid adapter are absent from the workspace graph.
The driver test binary and Testkit 0.65 share Platform 0.6; the driver selects
`pocket-ic` explicitly for direct client imports, including Principal support.
Testkit's native provider fixtures also select Platform's Candid/management
support; those features remain excluded from deployed library and probe graphs. Its
normal coordinator excludes Platform, Testkit, Wire and the probe protocol;
they are dev-dependencies used by the maintained integration tests. The library
and canister graphs exclude native simulator features. Build, Host and Testkit
share the 0.65 family, with no older parallel Host or Platform provider.
The catalog's
local `ic-timers` entry is the probes' dependency on the production library.

Shared's Make defaults select `cargo sort --workspace --grouped` for formatting
and add `--check` for independent checks; the duplicate local overrides are removed. Hooks use the same `make fmt` policy
inside their exact index export. All current, commit-preparation and
committed-source release checks select `--manifest-order-grouped`. Those release
checks are read-only, operate on their selected source, and never globally
reorder the two dependency groups. Manifest write behavior is qualified in a
disposable fixture; normal checkout checks retain the authored dependency groups.

## Consumer boundaries

Shared owns common tools/store admission, fixture dependency closure, evidence
transport, formatting mechanics and Cargo release metadata custody. Timers owns
runtime/provider boundaries, direct-only delivery in
[the declaration](../ci/release-delivery.txt), product source/tag/impact policy,
probe subjects and complete recovery/cohort gates. See [releasing](releasing.md)
and the [hygiene overlay](audits/code-hygiene.md).

Common Rust setup now selects only Cargo sort and sort-derives. Timers has no
Candid-extraction caller; existing retained extractor installations are not a
new product dependency or selected common tool. The ordered Make extensions
prepare/check Testkit through the root-lock-selected CLI. Testkit owns server
selection and admission. Timers selects no IC executable bundle or PATH.

Local script/Make fixtures select explicit roots through the shared
`prepare-script-fixture.sh`; Shared admits and copies the declared closure.
Timers retains actual Make inputs, substitutions and product assertions.
Downloaded-evidence qualification now delegates host-only compact verification
to Shared's selected-bundle interface. Timers retains source/run identity and
product paths; it no longer implements a parallel host receipt verifier.
Unselected IC stores and failed diagnostics remain eligible for retention.
Redirected `target`/`.tools` parents are retained as links with local diagnostics;
children are not selected. Independent logs remain eligible and the collector
reports incomplete product collection even when tool selection succeeds.

## Consumer qualification selection

Shared owns the generic tool-command, formatter prerequisite, Make-interface and
LOC regression matrices. Timers no longer dispatches them in `release-check` or
selects them as snapshot roots. This implements the local change prepared in
[PR #73](https://github.com/dragginzgame/ic-timers/pull/73) for
[#74](https://github.com/dragginzgame/ic-timers/issues/74); default-branch delivery
remains required before issue closure.

Timers retains actual tool setup/admission, hook/index preservation, repository
orchestration, snapshot corruption, release recovery and failure-evidence checks.
Product MSRV, Wasm, watchdog and cohort qualification is unchanged. Review relevant
upstream fixtures when a contract changes; do not replay the producer's whole
implementation matrix as a routine consumer prerequisite.

## Deployment dependency exclusions

The selected Shared direct-dependency checker `check-cargo-dependency-boundaries.sh` is now
included in `actions-check`. The local policy forbids direct Candid/CDK-family,
Platform macro-helper, retired Testkit decoder, IC0 and executor declarations in
every member. Platform 0.6 owns generator/runtime paths, raw stable passthroughs and
exact upstream selections. There are no consumer exceptions. See the
[published boundary](architecture.md#candid-and-cdk-dependency-boundary).
The checker resolves aliases through Cargo metadata and
checks all normal/build/dev/target declarations. The root has one workspace;
there are no maintained independent Cargo fixtures or manifests.

`scripts/ci/check-library-dependency-boundary.sh` owns Timers' policy and calls
Shared's `check-cargo-graph.pl` for the library, both canister probes and the
protocol package, independently for Wasm and the actual Rust host. Library and
probe graphs exclude Candid, full CDK and host tooling. The library additionally
excludes Wire/probe packages, Principal, Serde, SHA-2/digest and text dependencies.
The protocol excludes Platform and timer authority. Platform may select only
executor/IC0 for the library, plus `cdk`/`runtime`/`principal` for probes.
The checker derives forbidden feature names from the selected Platform catalog
and delegates dependency-feature-edge refusal to Shared. It retains local
successful-metadata admission of one registry executor 2.0.0 and IC0 1.2.0.
Shared identity counts supplement rather than replace those source/version checks.

Real-Cargo consumer fixtures cover Principal/Candid/default feature leakage, renamed target
and build dependencies, protocol authority exclusion, permitted dev-only Wire and
same-version executor source replacement. Shared owns generic parser and failed-
producer fixtures; those are no longer repeated here. Source symlink checks run
in `actions-check`; package verification admits the prepared archive's license.

**Remaining limits:** role comments and unused catalog entries are not validated.
Feature-edge inspection is not a universal inventory of arbitrary root features;
Platform is a dependency of each selected runtime root here. Graph evidence is
not final deployment admission. Consumers must bind their own target/features,
source/lock, successful builds and final artifact digests. Native simulator Candid
transport remains intentional and outside the deployed-canister policy.

## Qualification and refresh

Current preview/apply, incoming before-images, verified published archives and
focused qualification logs are in `/tmp/ic-timers-upstream-refresh-zs_qs4gq/`.
The [handoff](status/current.md) records tested scope and separate macOS/release
qualification limits. A one-time isolated owner fixture checks the changed wait
contract; no producer matrix is restored to routine Timers validation.

Future refreshes use the reviewed source's canonical exporter: preview first,
then apply the same explicit root selection. Retain directly used helpers and
review closure changes against local callers, especially supplier-only fixtures.
No moving sibling source or temporary proposal is adopted implicitly.
