![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-29

## Source and accepted work

Local main is **0.32.0**, `8f2fdfd4fe0838c3e2c32b9e1e225c55ae238a8a`.
The maintainer requested latest IC Tooling and Wire on top of the pending
Platform/Metrics batch. Existing dirty work is preserved. The root catalog now
selects Tooling **0.70.0** and Wire **0.7.22**. The incoming lock already had Wire
0.7.22; bounded resolution replaces the incoming Tooling 0.69.14 family and drops
its old Platform 0.7.6 dependency. Incoming Platform 0.8.1, Metrics 0.17.3 and
unrelated versions remain unchanged. There is one undated **0.32.1** draft.
Tooling's upstream minor affects unused ledger/Registry types, not Timers' public
API; this remains a compatible patch. Cargo package versions stay 0.32.0.
No staging, commits, pushes, release commands, package version mutation or sibling
source edits ran.

## Published dependencies and ownership

Official crates.io records verify the latest non-yanked stable selections at
adoption. Checksum-verified archives, exact-revision notes and source comparisons
are retained at `/tmp/ic-timers-tooling-wire.r_zgg1ky/`. Rust 1.91 is unchanged.

| Owner | Selected release | Published source revision |
| --- | --- | --- |
| Build, Host Artifacts/FS/Process, Testkit | **0.70.0** | `ecc8d61d19f0b711788f7e10e5dd728d0db6ff60` |
| Wire and derive | **0.7.22** | `ac08fdc1757b7d3f1483caecadd7ac1c817a30d8` |

Tooling 0.70 adopts published Platform 0.8.1. Its ledger and Registry types have
new Platform-owned Rust identities, but Timers uses neither capability. The used
Tooling Rust source trees are unchanged from 0.69.14, and Wire's runtime and
macro source trees are byte-identical to 0.7.21. No caller, codec limit, frame,
error contract, callback, trap, lifecycle or internal eight-byte dispatch change
was needed. No compatibility alias, local wrapper or policy relaxation was added.

The complete locked workspace now resolves **one Platform 0.8.1**, including the
native test worker. The old Platform 0.7.6 package is removed. Executor **2.0.0**,
ic0 **1.2.0**, PocketIC **16.1.0** and external Principal **0.1.5** remain singular
identities. The library selects only Platform `executor`/`ic0`; probes use `cdk`
with owned Principal. Deployed probes exclude external Principal, SHA-2/digest,
Candid and JSON parsing. The Wasm library also excludes Wire/Postcard and host
tooling. Native Testkit explicitly selects external conversions for its existing
client API; it introduces no deployed timer authority.

[IC Tooling #267 — Adopt Platform 0.8 in Testkit](https://github.com/dragginzgame/ic-tooling/issues/267)
has a published supplier correction and fresh Timers consumer proof posted.
It was still open at the last read; no issue state was changed. Supplier closure
uses its own qualification, not a consumer publication/acknowledgement gate.
There is no remaining local duplicate-Platform adoption blocker.

Canonical editor/Testkit setup and offline admission select **0.70.0**. Testkit
continues to own server 16.1.0 admission and fresh-instance lifecycle. Metrics'
catalog minimum remains **0.17.2**, resolving incoming **0.17.3**; its Rust source
is unchanged from 0.17.2. Required Serde and native Wire metadata remain, with
recording unselected and no Logging package. Thiserror **2.0.18** and Shared
**0.27.3** are unchanged. Candid **0.10.39** remains in the native PocketIC control
graph only; the normal coordinator excludes codecs and simulator capabilities.
Storage/memory remain out of scope.

## Issue delivery and next work

The previous live triage found only Timers **#79** open and no open PRs; this
adoption did not change issue state. Its Timers predicate and installed proof
are delivered. Canic's prepared inspector integration is still a separate
consumer delivery, not grounds for another Timers implementation.

- **#77/#78/#80 are closed**: queued-scheduler repair evidence, global-timer
  conflict detection and application-call ownership are delivered. Scheduler-body
  normal return remains the protocol assumption; arbitrary scheduler traps or
  permanently live futures remain unqualified. No uncertain-effect replay is
  authorized. Causal stop/restart evidence remains at
  `/tmp/ic-timers-scheduler-pressure.YxSpot/`.
- **#79 / Canic #695**: prepared ingress/cancellation work remains at
  `/tmp/ic-timers-backlog-03114.GlZdEf/canic`, with `canic-code.patch` in the parent.
  The original sibling working checkout remains untouched. The last live Canic
  main inspection still lacked these changes; do not close before delivery.
- **Canic #700**: published `ic-call` 0.7.2 resolves its supply blocker, with an
  existing Rust 1.91 native/Wasm witness. The public Call/CallBuilder/CallResult
  retirement remains a Canic breaking minor, subject to its maintainer-requested
  and accepted 0.112 Human Minor Closeout Gate. The blocker and proof are recorded
  on #700. Platform #75 owns its remaining migration-document follow-up.

## Qualification

Current evidence: `/tmp/ic-timers-tooling-wire.r_zgg1ky/`.
Passes: locked/offline metadata and explicit one-Platform assertion; strict Rust
1.91 library/runtime-probe/protocol/coordinator Clippy; three native build-owner
tests; library-only MSRV Wasm; canonical editor/Testkit setup/admission; actual
version preflight/rollback/dirty-worktree preparation; formatting; pin and direct
IC checks; all native/Wasm library/probe/protocol/coordinator dependency, feature
and identity exclusions; real-Cargo positive/negative boundary fixtures;
changelog admission and diff checks. Unrelated incoming versions and the root
package version are preserved. No product runtime implementation changed.

A disposable registry-only Build 0.70.0 consumer built the actual optimized probe
and held its evidence through four installed PocketIC subjects: all six
queued-scheduler stop/restart trajectories, malformed Wire admission/typed errors,
ingress/foreign-call authentication and response-trap cleanup. Each subject ran
and passed, and source/build evidence remained current throughout. Retained Wasm
SHA-256: `9cbebccf720df2bf50286b9775732df8068dad03f2b0ea80152482a97054c310`.

Released 0.32.0's [CI](https://github.com/dragginzgame/ic-timers/actions/runs/40839164860)
passed Linux, MSRV and both native macOS jobs on its older supplier graph. Fresh
native macOS, full local release/cohort gates and live IC were not run for this
adoption. Preserve normal maintainer-operated release and host requirements.
Earlier evidence at `/tmp/ic-timers-platform.kwzduv0m/` retains its original
Tooling 0.69.14 / two-Platform graph scope.
