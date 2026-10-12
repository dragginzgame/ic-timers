![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.19.3 is committed, tagged and pushed** at
`1e451ba35613f2efde601073636c63ed97a91c06`. Local HEAD and the remote annotated
`v0.19.3` tag agree. Registry publication is not independently established.
Exact-source hosted [main run 38249542866](https://github.com/dragginzgame/ic-timers/actions/runs/38249542866)
is queued, and [tag run 38249543691](https://github.com/dragginzgame/ic-timers/actions/runs/38249543691)
passes at inspection; no complete hosted
acceptance is claimed for this release yet.

The committed graph selects Metrics **0.7.2**, Testkit **0.34.5**, all four
Host **0.14.9** and PocketIC **16.1.0**. Incoming Cargo.lock selects Metrics
**0.7.3**; preserve it separately from committed-source qualification. Latest
Host/Testkit releases change contributor tooling rather than consumed library
implementation. No dependency rewrite is performed by this preparation.

The release adopts corrected Shared 0.9.0 source
`aec7a149f581a42b9531fc747b46024fc0314fcf` across **67/35/11** payloads.
Canonical annotations now match VERSION. The [adoption owner](../shared-tooling.md)
preserves the preceding source's mismatched-version observation separately.

The delivered timer API/runtime matches 0.19.1. Public instruction totals and
means remain `u128`; samples/counts/latest/maximum remain `u64`. The Metrics
[owner](../design/callback-delivery-ownership.md#prepared-ic-metrics-07-adoption)
records the report/codec and volatile-state contract. No persistence, alternate
registry, compatibility reader or new platform reader is added.

## Prepared Shared 0.9.2 admission and cleanup

All three canonical exports select
**0.9.2 `c118062a070a2e238f1eb3989b6fd700f369ce1e`**. Clean-source previews
precede apply, retaining the existing roots and **67/35/11** files without
additions or retirements. Only the exact committed source is selected.
Host/IC installation now shares lifecycle handling: checkout cleanup failures
do not prevent shared-lock release or hide an original installation error;
cleanup-only failure also returns nonzero and failed candidates stay retained.
The selected fixtures and local-setup guidance follow the same source.
Evidence checksum manifests now reject NUL bytes before hashing referenced
files, and installers reject supplied/resolved CR/LF consumer paths before
configuration or setup. No new local script-path suite or command is activated.

Inputs, before manifests and preview/apply logs remain at
`/tmp/ic-timers-shared092.W81svtKr/`. Prior 0.9.1 inputs/logs remain at
`/tmp/ic-timers-shared091.LNqFhTsF/`. Integrity, Bash syntax, owner-document links
and diff checks pass; Cargo files, host/IC pins and real index entries match the
saved inputs. Installer fixtures, setup and release effects are not run; the
separately approved product checks are recorded below.
These compatible contributor changes are now bundled into the one undated
**0.20.0** draft for the approved constructor hard cut below. Cargo versions
remain 0.19.3. No product correctness defect is claimed by the tooling review.
The audit overlay now reads its method identity from the reviewed audit snapshot
and distinguishes approved library compile-only development checks from tests,
lint and other builds. The focused source inspection of scheduling, cancellation,
callback finalization, abandonment and reconciliation finds no new runtime defect;
this is source review, not new test execution or a complete runtime audit.

## Prepared 0.20 typed registration

The approved hard cut replaces the independent lifetime/optional-schedule
arguments of all three direct registration functions with `TimerStart`.
Retained starts allow an optional initial schedule; remove-on-stop starts require
one structurally. The shared initial-registration owner decodes it once and
retains existing validation-before-allocation, provider binding/confirmation
and capture cleanup. `DeclarationLifetime` stays an inert snapshot observation.
`TimerError::InitialScheduleRequired` and the probe's missing-schedule path are
removed; no old-signature forwarder or compatibility reader remains.

Current native fixtures, runtime/size probes, PocketIC endpoint/caller and API
examples use typed starts. The public compile-fail example covers the invalid
configuration at its type boundary. Lifecycle reconciliation signatures, async
ordinary callbacks, after-completion cadence and pre-armed synchronous Watchdog
semantics remain intact by source inspection. The [design owner](../design/0.20-typed-registration.md)
records the exact scope, consumer mapping and remaining qualification.

The approved offline/locked `make check` passes on Rust 1.99.0 with incoming
Metrics 0.7.3; a compile-only library Wasm check passes on Rust 1.88.0.
Separate explicit approval permits focused native tests, doctests, library/probe
Clippy and probe compile checks. All 149 native tests and nine doctests pass;
the nine doctests also pass on Rust 1.88.0. Library Clippy passes on Rust 1.99.0.
Runtime/PocketIC probe Clippy, all four size-probe Clippy selections and the
runtime/four-cohort Wasm compile-only checks pass on Rust 1.88.0. Commands, logs
and the source patch are retained in `target/development/typed-start.rFyIOguo/`.
Cargo inputs and real index entries match their pre-validation records.
No setup, PocketIC execution or release command is run; the complete release
gate and footprint measurements remain outstanding. No Wasm/instruction savings
are claimed.
Canic's direct callers need this argument cut at adoption; Jobs uses unchanged
lifecycle helpers. Sibling source/pins are not edited.
Canic's existing [adoption owner](https://github.com/dragginzgame/canic/issues/33#issuecomment-6127168880)
has the exact argument mapping and preparation/qualification limits.

## Completed frozen qualification

[#44](https://github.com/dragginzgame/ic-timers/issues/44) is closed from the
completed frozen early/late acceptance. Shared fleet owner
[#98](https://github.com/dragginzgame/shared-tooling/issues/98) has been updated
for Timers; other consumers keep their own requirements.
The latest GitHub inspection finds one open Timers issue:
[#50](https://github.com/dragginzgame/ic-timers/issues/50), the Metrics 0.8 tracked
recording/type-identity adoption. The producer still prepares 0.8.0 on top of
released 0.7.3. Timers already uses tracked summaries and needs no replacement
for the removed raw recording helpers. Leave the selected graph unchanged
until publication, then qualify the public dependency identity and report/probe
owners; current 0.7.3 checks do not qualify 0.8.
#47/#48/#49 are closed from their source-bound evidence; their prior cancelled
or failed attempts remain separately recorded in the [evidence owner](../releasing.md).

The approved qualification stays frozen at **v0.19.1**,
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`, using its committed Metrics 0.7.1,
Testkit 0.34.2, Host 0.14.5, PocketIC 16.1.0 and Shared 0.8.0 graph.
That source's main gates pass Linux/MSRV/Intel/ARM. The approved
[early run 38234040990](https://github.com/dragginzgame/ic-timers/actions/runs/38234040990)
is accepted: all three producers reach controlled exit 22/upload and all three
hosted downloaded-artifact verifiers pass. Run/job/artifact JSON, original raw
logs, exact archives, uploaded/downloaded hashes and dispatch identities
were recorded at `/tmp/ic-timers-0191-qualification.IQiwIXf8/`. Original failures
stay failures. That earlier local directory was absent at the preceding inspection;
available GitHub observations/archives are restored under its new
`restored-readback-20261012/` subdirectory. Original local dispatch intent/reply
files are unavailable and are not fabricated. Existing approval and issue/history
establish the already-submitted pair; no repeated dispatch is authorized.

Only after early completes, one approved
[late run 38245658495](https://github.com/dragginzgame/ic-timers/actions/runs/38245658495)
is dispatched at the same source/ref. Linux producer **114798745212** completes
CI, probe lint and PocketIC recovery/policy gates, then reaches the intended
late exit 2 and uploads artifact **11746077825**. Its raw log and exact downloaded
archive are retained; the downloaded ZIP hash matches the uploaded digest.
Local frozen-verifier replay passes, separately from hosted acceptance.
ARM producer **114798745283** also passes its complete native gate and reaches
controlled late exit 2/upload, artifact **11751795650**. Its original raw log
and exact downloaded archive are restored from GitHub with matching upload hash;
local frozen-verifier replay passes. Intel producer **114798745371** now also
passes its complete native gate, reaches controlled exit 2 and uploads artifact
**11752557455**. Its original log and exact ZIP are retained with matching upload
hash. All three hosted late verifiers pass: Intel **114854856791**, Linux
**114854856857**, ARM **114854857036**. The sequential pair is accepted at the
exact frozen source/ref/attempt, with original intentional overall failures
preserved. These results do not qualify the newer release graph or pending
0.9.2 snapshot. Final records and hashes are in the evidence owner.

## Authority and next action

The explicitly approved early/late pair is finished; no additional dispatch,
retry or source switch is authorized or needed. Continue bounded CI/issue
inspection on the next requested pass. Do not invent product work or repeat
completed validation for unchanged inputs.
Original v0.17.13 archive and earlier adoption directories under /tmp are also
absent at inspection. Retain historical evidence links/digests; no prior outcome
is relabelled and no new run is submitted to replace missing local records.

AGENTS.md permits locked/offline compile-only development checks after Rust
changes with prepared tools and no owner validation active. Other tests,
doctests, Clippy, PocketIC, setup and full gates remain user-owned outside the
specific approvals recorded above. The focused 0.20 checks are finished;
their approval excludes setup, PocketIC execution and release commands.
Staging, commits, Cargo version mutation,
release commands, tags, pushes, publication and cleanup stay user-owned.
Preserve all old tools/build/evidence artifacts; siblings stay read-only and
native substitutes are not IC trap/rollback proof.
