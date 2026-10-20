# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records **138** canonical payloads
from reviewed Shared Tooling **0.25.2**, revision
`546d32e48be5a5d4e360299f6a94901df6acc577`. The supplier checkout is clean and
matches its published default branch. Canonical preview/apply retired the three
old Perl editor/test roots and selected the graph, source-symlink and license
checkers. Exported files remain byte-for-byte supplier-owned.

Shared owns generic graph observation, fixture matrices, tool admission, evidence
transport and metadata transactions. Timers owns package/feature exclusions,
qualified runtime identities, release/source policy and actual integration tests.
IC Build supplies the published metadata editor selected by the root lock; the
consumer's positive document budget is in `ci/release-editor.env`. Prepare it
explicitly with `make install-release-editor`; check it offline with
`make release-editor-check`. Fresh release preflight installs it only after
source admission; standalone preparation requires an already admitted executable.

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
(executor and raw IC system bindings). Serde is already production-transitive
through Platform's Principal; only probes directly inherit its catalog entry.
Wire, Platform's canister runtime, Build, Host Process, Testkit and the probe protocol serve
qualification packages here.
Candid remains transitive only in the native driver through PocketIC/Testkit. Local probe protocols use Wire; Platform 0.2's `cdk`
feature and both canister graphs are Candid-free. Full CDK/macros and the retired
Testkit Candid adapter are absent from the workspace graph.
The catalog's
local `ic-timers` entry is the probes' dependency on the production library.

The local Make adapter selects `cargo sort --workspace --grouped` for formatting
and adds `--check` for independent checks. Hooks use the same `make fmt` policy
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

## Deployment dependency exclusions

The selected Shared direct-dependency checker `check-cargo-dependency-boundaries.sh` is now
included in `actions-check`. The local policy forbids direct Candid/CDK-family,
Platform macro-helper, retired Testkit decoder, IC0 and executor declarations in
every member. Platform 0.2 owns generator/runtime paths, stable helpers and
exact upstream selections. There are no consumer exceptions. See the
[published boundary](architecture.md#candid-and-cdk-dependency-boundary).
The checker resolves aliases through Cargo metadata and
checks all normal/build/dev/target declarations. The root has one workspace;
there are no maintained independent Cargo fixtures or manifests.

`scripts/ci/check-library-dependency-boundary.sh` owns Timers' policy and calls
Shared's `check-cargo-graph.pl` for the library, both canister probes and the
protocol package, independently for Wasm and the actual Rust host. Library and
probe graphs exclude Candid, full CDK and host tooling. The library additionally
excludes Wire/probe packages. The protocol excludes Platform and timer authority.
Platform may select only executor/IC0 for the library, plus `cdk` for probes.
The checker derives forbidden feature names from the selected Platform catalog
and delegates dependency-feature-edge refusal to Shared. It retains local
successful-metadata admission of one registry executor 2.0.0 and IC0 1.2.0.
Shared identity counts supplement rather than replace those source/version checks.

Real-Cargo consumer fixtures cover Candid/default feature leakage, renamed target
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

Current preview/apply, before-images and focused qualification receipts are in
`/tmp/ic-timers-three-improvements.7qqc7zen/`. The
[handoff](status/current.md) records the tested current scope. Linux evidence does
not qualify native macOS or the complete maintainer-operated release gate.

Future refreshes use the reviewed source's canonical exporter: preview first,
then apply the same explicit root selection. Retain directly used helpers and
review closure changes against local callers, especially supplier-only fixtures.
No moving sibling source or temporary proposal is adopted implicitly.
