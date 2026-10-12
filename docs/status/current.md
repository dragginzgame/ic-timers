![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.20.0 is committed and locally tagged** at
`01031c912058089756aed87fafa4918476c12bd4`; the maintainer reports it live.
Exact-source hosted [main run 38272698611](https://github.com/dragginzgame/ic-timers/actions/runs/38272698611)
is queued and [tag run 38272698734](https://github.com/dragginzgame/ic-timers/actions/runs/38272698734)
passes at inspection. No complete hosted acceptance is claimed yet.
The earlier 0.19.3 main run is cancelled, while its tag run passes; these are
separate source-bound observations, not failures of the new runtime.

The committed graph selects Metrics **0.7.3**, Testkit **0.34.7**, all four
Host **0.14.10** and PocketIC **16.1.0**. Shared **0.9.2**
`c118062a070a2e238f1eb3989b6fd700f369ce1e` is delivered across **67/35/11**
payloads. Its [adoption owner](../shared-tooling.md) retains installer cleanup,
checksum/path admission and source-bound preparation evidence.

The [typed registration cut](../design/0.20-typed-registration.md) is delivered:
`TimerStart` combines declaration retention with the initial schedule, making
inactive remove-on-stop creation unrepresentable. Lifecycle reconciliation,
callback policies, cancellation and recovery retain their existing contracts.
The earlier approved native tests, doctests, Clippy and probe compile checks
belong to their retained preparation source/graph, not automatically to the
later release graph. Canic has the [argument mapping](https://github.com/dragginzgame/canic/issues/33#issuecomment-6127168880)
and [focused qualification follow-up](https://github.com/dragginzgame/canic/issues/33#issuecomment-6127448281).
Sibling source/pins remain separately owned.

## Prepared Metrics 0.8 adoption

Incoming maintainer Cargo.toml/Cargo.lock changes select published registry
Metrics **0.8.0**. Its official cached archive matches the selected checksum and
released source `71a29d8a4d7ecf6774d9715895ce9abd13b9e5d7`.
Locked/offline metadata resolves one Metrics package and one Timers package;
all four members still inherit direct dependencies from the root catalog.

Timers already records through tracked summaries and has no obsolete raw
recording helper or raw-only error match to replace. Metrics adds checked
fractional `scaled_mean(scale)`, available directly through the summary.
The public summary/error type identity changes, requiring the next minor
**0.21.0** draft and coordinated direct Metrics consumers. Timer execution,
callback attribution, volatile ownership and probe Candid fields stay unchanged.
No compatibility wrapper, new report API or persistence dependency is added.

The [adoption owner](../design/callback-delivery-ownership.md#prepared-ic-metrics-08-adoption)
records the precise source/graph and consumer implications. Library/test-target
compile-only checking passes on Rust 1.99.0; production library Wasm checking
passes on Rust 1.88.0 with locked/offline prepared inputs. Metadata, commands,
logs and incoming inputs remain at `target/development/metrics080.ehT3o5JK/`.
The separately approved focused checks below now also exercise the Metrics 0.8
graph with the atomic lifecycle source. Cargo files and real index entries are
preserved. No PocketIC, setup, release effects or footprint measurements run.
The prior 0.20 focused-check approval remains completed at its original inputs.

## Prepared atomic first lifecycle reconciliation

The approved additional 0.21 hard cut publishes a fresh owner slot only after
its initial requested state succeeds. A first-call error removes its exact new
declaration and handles before capture destruction, leaving the slot empty so
a retry installs its supplied callback. Existing-slot failures retain their
authority and callback; unused replacement factories still drop before control
is applied, outside registry borrows. Public signatures stay unchanged.
The [lifecycle owner](../design/0.20-typed-registration.md#prepared-021-lifecycle-creation)
records error precedence, monotonic identities, destructor effects and IC limits.

Prepared native fixtures cover deadline/duration errors, binding/confirmation
faults and retry selection for all policies, later failure retention and
unused-factory destruction ordering. The existing canister probe and PocketIC
caller gain the initial-reconciliation rejection subject. Library/test-target
compile-only checking passes on Rust 1.99.0 and library Wasm compilation passes
on Rust 1.88.0 with locked/offline prepared inputs. Source, commands and logs are
retained at `/tmp/ic-timers-021-reconciliation.sFVjKHjS/`. With explicit focused
approval, all **154 native tests**, nine doctests on each of Rust 1.99.0 and
1.88.0, library Clippy on Rust 1.99.0, runtime/PocketIC probe Clippy on Rust
1.88.0, all four size-probe Clippy selections and runtime/four-cohort Wasm
compile-only checks on Rust 1.88.0 pass. The checked source patch and Cargo inputs
match their saved records; index entries are preserved. No setup, PocketIC or
release command runs. Canic has the [semantic cut notice](https://github.com/dragginzgame/canic/issues/33#issuecomment-6128606209).

The maintainer's subsequent release attempt at prepared commit
`48299c0c23765ce89445a57727364228388988fc` passes 15 PocketIC watchdog subjects
and fails the new reconciliation subject before any timer call: its absolute
clock reset moves time backward, rejected as `SettingTimeIntoPast`.
The host fixture now advances its existing clock by one second, retaining all
policy/inventory/capture assertions. Runtime/probe source and the built Wasm are
unchanged; Rust 1.88.0 probe Clippy passes. Original failure, repaired host-test
patch, probe hash and input/check records remain at
`/tmp/ic-timers-021-clock.l0OCwT2E/`. Focused PocketIC replay permission is requested
separately; no replay or complete release result is claimed yet. The
[release owner](../releasing.md#pocketic-fixture-clock-repair) records the diagnosis.

## Issues and frozen qualification

The latest issue inspection finds one open Timers issue,
[#50](https://github.com/dragginzgame/ic-timers/issues/50). Publication is now
established for Metrics 0.8; focused native/report and compiler/probe checks pass.
The complete release and IC acceptance remain outstanding. Keep it open until
the selected graph is qualified.

[#44](https://github.com/dragginzgame/ic-timers/issues/44) is closed from the
completed, sequential early/late qualification at frozen **v0.19.1**
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`. All three producers reach their
controlled failure/upload, and all three downloaded-artifact verifiers pass
in each stage. The [evidence owner](../releasing.md) retains exact run/job/artifact
identities, hashes, original failures and missing/restored local-record limits.
No repeated dispatch is authorized. Shared fleet [#98](https://github.com/dragginzgame/shared-tooling/issues/98)
has Timers' completed observation; other consumers retain their own obligations.
#47/#48/#49 are also closed from their source-bound acceptance.

## Authority and next action

Finish #50's qualification against the current selected graph before delivery;
do not use frozen 0.19.1 or upstream arithmetic experiments as that proof.
Shared 0.9.5 and Host 0.15 are available at inspected committed sibling heads;
neither is adopted by this Metrics review. The selected indirect Host graph
remains Testkit-owned, with no new direct Host dependency.

AGENTS.md permits prepared, locked/offline library compile-only checks.
Tests, doctests, Clippy, probe builds, PocketIC, setup and complete gates remain
user-owned outside explicit scoped approvals. Cargo version mutation, staging,
commits, tags, pushes, publication and cleanup stay user-owned. Preserve incoming
Cargo changes and retained evidence; siblings remain read-only. Native
substitutes and compiler checks are not IC trap/rollback proof.
