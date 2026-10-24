![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and scope

Released base: **0.30.9**, `2ddb278`. Cargo package versions remain 0.30.9.
The undated **0.30.10** draft contains the current test/qualification cleanup.
Published APIs, production timer transitions and recovery semantics are unchanged;
this is compatible contributor/test work, not a reason to publish independently.
Staging, commits, pushes and release execution remain maintainer-owned.

The cleanup removes the local IcyDB readiness model and its self-testing mapping,
retaining the lifecycle scenario with explicit callback results. Ordinary provider
failure tests share a two-policy matrix. Counter tests retain event forwarding,
role isolation and completion aggregation while delegating arithmetic to Metrics.
Suspended sequences now run 768 traces, omitting ineffective abandonment/fault
combinations and asserting deadlines, lifetime, failure state and counters before
cleanup. Existing focused arbitration and IC recovery subjects remain maintained.

Protocol tests retain actual record/checkpoint round trips and wide values.
Generic codec rejection coverage was checked at Wire 0.6.12's published source
revision `b17c0779b1db1b6805ad0135a4396ebd09ee72b9` before removal locally.
The PocketIC boundary still tests malformed input before timer mutation.
The unpublished mixed-policy probe drops its parallel instruction bookkeeping;
its host and canister wire record changed together. Rebuild them together.
Mixed-policy progress, timing, isolation and upgrade evidence remain intact.

Fresh-registration and existing-claim reconciliation retain separate dispatch:
creation rollback/publication and later-error retention have different duties.
A further generic selector was rejected because its indirection did not justify
changing those lifecycle paths. Private platform handles and provider effects
retain their established ownership and cleanup ordering.

## Qualification receipts

Typed records now own cohort metrics, context and artifact fields. Complete
schema-1 receipts preserve their JSON field names. Missing/unknown fields,
empty compiler/artifact identities, wrong cohort identities and absent measurements
reject before publication or comparison. Comparison uses the admitted object,
not a second read of the child's candidate path. Full-width counters round-trip;
unrepresentable signed comparison deltas still reject.

Qualification retains named recovery-subject admission, ignored-test rejection,
fresh private diagnostics, caller-owned Testkit logs and atomic Host FS publication.
Old output is never fallback evidence. Successful host CI uploads remain evidence
receipts, not proof of qualification by upload alone. See the release guide for
operator-controlled baseline paths and matching comparison context.

[Build #241](https://github.com/dragginzgame/ic-tooling/issues/241) remains the
recorded source-independent build-recipe identity gap. Cross-source comparison
still requires identical operator-controlled ambient Cargo/Rust configuration;
this cleanup adds no local configuration resolver.

## Dependencies and preserved work

Build/Host/Testkit remain 0.66.3, Wire/derive 0.6.12 and Metrics 0.15.7.
Platform/macros changed independently from 0.6.12 to **0.6.13** during validation.
That lockfile edit is preserved, not attributed to this cleanup. Its missing
published sources were downloaded with `cargo fetch --locked`; no dependency or
manifest selection was edited here.

Shared Tooling is now reviewed **0.26.10**,
`a8141e775983c7fe504c4dd911e978152e657fdf`: 137 payloads refreshed through the
canonical impact/apply interface. Formatting uses literal catalog admission;
`actions-check` composes the common external-IC inventory with local policy.
Optional CI Cargo-tool caching remains disabled. See `docs/shared-tooling.md`.

The alias fixture and README setup fixes for Timers #75/#76 are in the local
0.30.9 release history. Remote issue closure was not assessed in this pass.
No sibling checkout, Git index, release state or package version was modified.

## Evidence and remaining limits

Shared adoption evidence and incoming before-images: `/tmp/ic-timers-shared-02610/`.
Snapshot integrity, actions/host-and-Wasm dependency policy, formatting, the real
Cargo boundary fixture, hook, repository, workflow, release-gate and failure-
evidence fixtures pass. Offline `tools-report` and read-only selection reports
pass; tools needed no reinstallation. Native macOS/hosted cache are unqualified.
Cleanup logs and published-source inspection: `/tmp/timers-cleanup/`.
Focused runtime tests (84), Metrics tests (6), protocol tests (3), report tests (3),
Rust 1.91 library Clippy/Wasm and every probe lint selection pass. Formatting
and provider-boundary checks pass. Library checks were repeated against the
independently updated Platform 0.6.13 lock selection.

The initial watchdog run passed 33 subjects; cohorts then refused offline
metadata because the independently selected Platform 0.6.13 was not cached.
That failed invocation remains at `/tmp/timer-qualification-971360-0/`.
After locked dependency preparation, qualification was rerun against the current
lock: all 33 watchdog selections (including host regressions) and the policy
cohort subject pass. The fresh complete receipt is `testing/target/cohorts.json`;
successful invocation diagnostics were cleaned by their owning driver.

Earlier evidence remains at `/tmp/ic-timers-upstream.JrCXUA/` and
`/tmp/ic-timers-improvements.zG1CA1/`; their committed historical handoff preserves
older evidence references. Do not remove retained failure/build artifacts.
Native macOS, installed-IC execution, hosted artifact upload and the complete
maintainer-operated release gate were not qualified by this local Linux pass.
