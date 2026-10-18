# Shared Tooling adoption

## Current authority

One [snapshot](../.shared-tooling.snapshot) records **136** canonical payloads
from committed Shared 0.24.0,
`cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`. Canonical preview and refresh use
that clean committed source, excluding the sibling's pending 0.24.1 edits.
The canonical uploader remains exact 7.0.2 source
`cf430e030ddbb5b0abf93d22962f4752f3646cd9`.

Existing roots remain except the retired Make-only fixture copier and the generic
preparation fixture. The latter now requires Shared's own CI workflow; keeping
that supplier test here would cross Timers' workflow boundary. Its canonical
owner retains the test. Timers' CI workflow is unchanged. The snapshot adds the
required host-platform observer; no exported payload is patched locally.

The [baseline](../DRAGGINZGAME.md), [local overlay](../AGENTS.md) and
[0.24 adoption checklist](consuming-snapshots.md#dependency-role-adoption-in-0240)
govern this adoption. Earlier selections and qualification receipts remain
historical in the [handoff archive](status/archive-2026-10-18.md) and
[adoption evidence](evidence/shared-tooling-history.md).

## Dependency roles and formatting

The [root manifest](../Cargo.toml) separates `Production packages` from
`Tests and tooling packages`, and `Production dependencies` from
`Tests and tooling only`. These describe use here, not registry publication.
There is still one catalog and lockfile; member dependency tables determine
actual use through `workspace = true`.

The published `ic-timers` library uses Metrics, Thiserror and Platform
(executor and raw IC system bindings). Candid, the full CDK, Build, Host Process, Testkit,
its Candid adapter and the probe protocol serve only qualification packages here.
The catalog's
local `ic-timers` entry is the probes' dependency on the production library.

The local Make adapter selects `cargo sort --workspace --grouped` for formatting
and adds `--check` for independent checks. Hooks use the same `make fmt` policy
inside their exact index export. All current, commit-preparation and
committed-source release checks select `--manifest-order-grouped`. Those release
checks are read-only, operate on their selected source, and never globally
reorder the two dependency groups. No Rust source formatting or manifest write
was run against the normal checkout during this adoption; write behavior was
qualified in a disposable fixture.

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

The selected Shared 0.24 checker `check-cargo-dependency-boundaries.sh` is now
included in `actions-check`. The local policy forbids direct Candid/CDK-family
edges except Candid/CDK in the two unpublished canisters and the old decoder in
the native driver. Direct IC0 and executor declarations are forbidden in every
member; Platform 0.1.8 owns both exact selections. [Published API blockers](architecture.md#candid-and-cdk-dependency-boundary)
explain those exceptions. The checker resolves aliases through Cargo metadata and
checks all normal/build/dev/target declarations. The root has one workspace;
there are no maintained independent Cargo fixtures or manifests.

Locked all-feature normal/build graphs for the library were inspected for
`wasm32-unknown-unknown` and `x86_64-unknown-linux-gnu`. All test/tooling-only
catalog dependencies are absent, excluding the selected root package itself.
Rust 1.91 library Wasm compilation passes. This is library-graph evidence, not
qualification of an application's final linked canister.

**Missing enforcement:** pin/inheritance and direct-boundary checks do not validate
role comments, unused catalog boundary entries, unenumerated future family names,
feature reachability or transitive exclusions; the provider check validates
source ownership. Platform's default features are disabled in the catalog;
member features are inspected separately. Its unconditional Principal/Serde
support is production-transitive even though Candid remains qualification-only.
No existing automated gate binds this exclusion policy to deployed artifact
bytes. The consuming canister's build/deployment owner must enforce its own
exclusions with the exact source, lockfile, target/features and artifact digest.
A consumer may legitimately use CDK/Candid for its application even though those
are qualification-only dependencies of this workspace. See the
[release boundary](releasing.md#dependency-role-boundary).

## Qualification and refresh

Focused receipts, graph selections, canonical preview/apply and before-image
identity are retained in `target/qualification/shared024/`; the task arrival is
`/tmp/ic-timers-shared024-arrival.0djwogzx`. Current results belong in the
[handoff](status/current.md). Linux caller checks do not qualify native macOS,
hosted upload/download or the complete release gate.

Future refreshes use the reviewed source's canonical exporter: preview first,
then apply the same explicit root selection. Retain directly used helpers and
review closure changes against local callers, especially supplier-only fixtures.
No moving sibling source or temporary proposal is adopted implicitly.
