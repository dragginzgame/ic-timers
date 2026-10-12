![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

**0.19.2 is committed, tagged and pushed** at
`8b3e09fe82ee6a7e9cebda904536b1717bfc1fd3`. Local HEAD and the remote annotated
`v0.19.2` tag agree. Registry publication is not independently established.
Exact-source hosted [main run 38247833419](https://github.com/dragginzgame/ic-timers/actions/runs/38247833419)
and [tag run 38247833714](https://github.com/dragginzgame/ic-timers/actions/runs/38247833714)
are incomplete at inspection; no complete hosted
acceptance is claimed for this release yet.

The committed graph selects Metrics **0.7.2**, Testkit **0.34.4**, all four
Host **0.14.9** and PocketIC **16.1.0**. There are no incoming dependency edits
at initial inspection. Preserve Cargo versions, lock selection and root catalog.
Host's newer releases and Testkit 0.34.5 change contributor tooling rather than
consumed library implementation. During this pass Cargo.lock changes
concurrently to Testkit **0.34.5**; preserve it separately from committed-source
qualification. No local dependency rewrite or build is performed.

The release adopts Shared's requested 0.9.0 source
`019ce8236cc7cee14ea5540d468b939a2053ef30` across **67/35/11** payloads.
Its committed VERSION was still 0.8.4, so canonical annotations retained that
value. The [adoption owner](../shared-tooling.md) preserves this original
observation and earlier clean-source previews/apply logs.

Timer API/runtime Rust is unchanged from 0.19.1. Public instruction totals and
means remain `u128`; samples/counts/latest/maximum remain `u64`. The Metrics
[owner](../design/callback-delivery-ownership.md#prepared-ic-metrics-07-adoption)
records the report/codec and volatile-state contract. No persistence, alternate
registry, compatibility reader or new platform reader is added.

## Prepared provenance correction

Shared corrects only VERSION at
**0.9.0 `aec7a149f581a42b9531fc747b46024fc0314fcf`**;
[Shared #137](https://github.com/dragginzgame/shared-tooling/issues/137) is closed.
All three canonical exports now record 0.9.0 and that exact revision. Complete
previews retain every existing entrypoint and **67/35/11** file as KEEP;
selected payloads are byte-identical, without additions or retirements.
Uncommitted sibling installer edits remain unselected.

Preserved inputs, before manifests and preview/apply logs are at
`/tmp/ic-timers-shared090-corrected.L7AXQHLo/`. Integrity, owner-document links
and diff checks pass; the root manifest, host/IC pins and real index entries
match the saved inputs. The original and concurrently changed lockfiles are
retained separately. No tests, builds, lint, setup or release effects run.

The one undated **0.19.3** note describes repository-only provenance work.
Cargo package versions stay 0.19.2. Normally bundle this with future code-bearing
work instead of publishing a new package identity solely for annotations.
No runtime defect or new feature requirement is confirmed by this pass.

## Remaining issue and approved qualification

Only [#44](https://github.com/dragginzgame/ic-timers/issues/44) remains open.
#47/#48/#49 are closed from their source-bound evidence; their prior cancelled
or failed attempts remain separately recorded in the [evidence owner](../releasing.md).

The approved qualification stays frozen at **v0.19.1**,
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`, using its committed Metrics 0.7.1,
Testkit 0.34.2, Host 0.14.5, PocketIC 16.1.0 and Shared 0.8.0 graph.
That source's main gates pass Linux/MSRV/Intel/ARM. The approved
[early run 38234040990](https://github.com/dragginzgame/ic-timers/actions/runs/38234040990)
is accepted: all three producers reach controlled exit 22/upload and all three
hosted downloaded-artifact verifiers pass. Run/job/artifact JSON, original raw
logs, exact archives, uploaded/downloaded hashes and dispatch identities remain
at `/tmp/ic-timers-0191-qualification.IQiwIXf8/`. Original failures stay failures.

Only after early completes, one approved
[late run 38245658495](https://github.com/dragginzgame/ic-timers/actions/runs/38245658495)
is dispatched at the same source/ref. Linux producer **114798745212** completes
CI, probe lint and PocketIC recovery/policy gates, then reaches the intended
late exit 2 and uploads artifact **11746077825**. Its raw log and exact downloaded
archive are retained; the downloaded ZIP hash matches the uploaded digest.
Local frozen-verifier replay passes, separately from hosted acceptance.
Both macOS producers are queued at inspection;
late remains incomplete and #44 stays open. Overall intentional run failure is
expected; unrelated gate failure is not acceptance. These frozen results do not
qualify the newer release graph or pending provenance correction.

## Authority and next action

Inspect the already-dispatched late stage on the next requested maintenance
pass. Close #44 only after every original producer reaches the complete gate,
selected controlled failure/upload and all three hosted download verifiers pass.
No new dispatch, retry or source switch is authorized. Approval for the existing
early/late pair carries forward; do not ask again or poll unchanged queues.
Preserve original v0.17.13 archives and failed verifiers in
`/tmp/ic-timers-01713-qualification.n07_3uur/`.

AGENTS.md permits locked/offline compile-only development checks after Rust
changes with prepared tools and no owner validation active. Other tests,
doctests, Clippy, PocketIC, setup and full gates remain user-owned outside the
specific approved qualification. Staging, commits, Cargo version mutation,
release commands, tags, pushes, publication and cleanup stay user-owned.
Preserve all old tools/build/evidence artifacts; siblings stay read-only and
native substitutes are not IC trap/rollback proof.
