![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-12

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source and incoming work

**0.19.1 is committed, tagged and pushed** at
`0c24fac2fc1d38d216f6880a581f5ff9b85020e2`. Local HEAD, the dereferenced
annotated remote tag and hosted run identities agree. Registry publication is
not independently established by this inspection.
[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38231417336)
passes Linux/MSRV and both complete native Intel/ARM gates.
[Tag CI](https://github.com/dragginzgame/ic-timers/actions/runs/38231416722)
passes. These results qualify the committed 0.19.1 graph, not incoming changes.

The committed graph selects Metrics **0.7.1**, Testkit **0.34.2**, all four
Host **0.14.5**, and PocketIC **16.1.0**. These supersede the earlier prepared
Host 0.14.4 candidate during the maintainer's release. The only incoming edit
at initial inspection is the maintainer's Testkit **0.34.3** lock selection;
preserve it. It is separate from the committed graph's hosted qualification.
During the 0.9.0-source adoption, Cargo.lock changed concurrently again to
Testkit **0.34.4**, all four Host **0.14.8** and Metrics **0.7.2**. Preserve that
incoming graph; adoption performs no lockfile write or qualification of it.
No Cargo or package version mutation is authorized by issue inspection.

The release delivers Shared **0.8.0
`045a6f22a9de497bfb3af90cb2571957384806f2`** through **59/35/11** exports,
recorded entrypoints, canonical companion closure and the cargo-sort-derives
0.14.1 pin. All prior payloads remain; automatic retirement applies only to
verified unchanged obsolete owned files outside reviewed root selection.
The [adoption owner](../shared-tooling.md) retains complete previews/apply logs
and incoming identities under `/tmp/ic-timers-shared080.IMvYFaPt/`.
Earlier preparation and qualified sources keep their original scope.

The 0.19 product API retains Metrics 0.7 wide totals/means with samples/counts/
latest/maximum unchanged. Runtime/probe Rust and the compiler-ordering fixtures
are unchanged between 0.19.0 and 0.19.1. No persistence, compatibility reader,
alternate timer registry or new platform reader is added.
The Metrics [owner](../design/callback-delivery-ownership.md#prepared-ic-metrics-07-adoption)
records the report/codec and volatile-state contract.

## Prepared 0.19.2

The requested Shared **0.9.0 source `019ce8236cc7cee14ea5540d468b939a2053ef30`**
adoption is canonical across **67/35/11** exports. Upstream committed VERSION
still says **0.8.4**, preserved in canonical annotations; producer reconciliation
is tracked in [Shared #137](https://github.com/dragginzgame/shared-tooling/issues/137).
The root closure includes installer/build companions
already selected by the audit bundle, plus the new maintenance procedure;
no files are retired and no optional local command or scheduler is activated.
The [adoption owner](../shared-tooling.md) records all three complete previews,
apply logs and preserved inputs under `/tmp/ic-timers-shared090.ucWTtqQQ/`.
Only the exact committed source is exported. Prior 0.8.4 evidence remains at
`/tmp/ic-timers-shared084.rd6lPKII/`. The prior
0.8.3 safeguards/adaptation evidence remains at `/tmp/ic-timers-shared083.2RFeaOnh/`.

Formatter admission follows the Rust catalog; local Make binds it to the
reviewed root pins, and the existing real hook fixture covers external Rust
catalog overrides. Setup retains changed-build observations and refuses
redirected diagnostic paths. Snapshot parsing and atomic payload refresh arrive
from the same committed owner. The consumer ignores retained payload scratch
files without deleting artifacts.
The existing LOC reporter now supports `bash scripts/dev/cloc.sh --json`, with
shared counted rows and complete-on-success output; its owner fixture is updated.
No fleet reporter, local output parser or historical-note rewrite is added.
The canonical verifier now owns manifest syntax admission shared by upstream
refresh and fleet reporting. It captures once, rejects NUL bytes before parsing,
and emits admitted records only after complete syntax checks. Payload integrity
and prior source ownership remain independent boundaries. No local exporter or
central fleet scanner is added; existing v1 manifests need no conversion.

The one undated pending changelog is **0.19.2**, a compatible contributor/tooling
batch; Cargo package versions remain 0.19.1 and the incoming dependency lock edits
is preserved. Integrity/syntax/local-link/diff checks pass. Actual hook/Make/
installer fixtures, full/native validation and setup remain user-owned. The
approved qualification below still selects frozen v0.19.1; it supplies no
acceptance of this newer pending snapshot.

## Issue acceptance

[#48](https://github.com/dragginzgame/ic-timers/issues/48) is closed for the
delivered compile-only permission and earlier compiler/Clippy admission.
Linux and ARM gates pass at 0.19.0. The original Intel log establishes completed
all-target compilation and both ordering/fail-fast fixture suites before its
later cancellation during PocketIC preparation. This is scoped command evidence,
not a successful full Intel gate. Its raw log and digest are in
[the evidence owner](../releasing.md#0191-delivery-and-issue-acceptance).
[#49](https://github.com/dragginzgame/ic-timers/issues/49) remains closed from
the completed released-source minimum-compiler native/Wasm proof.

[#47](https://github.com/dragginzgame/ic-timers/issues/47) is closed from complete
0.19.1 Linux/MSRV/Intel/ARM execution, including the wide-total role/order
regression, report/Candid contract, all selected recovery tests and policy
cohorts. Raw source-bound logs and digests are retained in the evidence owner.
The earlier cancelled 0.19.0 Intel job stays cancelled; no newer dirty graph or
pending Shared snapshot is attributed to these completed results.

[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains open for full
common-Make/native and corrected frozen early/late artifact acceptance. The
approved v0.17.13 early run has successful producer failures/uploads but failed
old-path download verifiers. Its late run has all three successful downloaded
verifiers. Preserve those original observations and
`/tmp/ic-timers-01713-qualification.n07_3uur/`.

The maintainer now explicitly approves **early, then late at frozen v0.19.1**,
after matching CI completes. Approval, exact tag/source readback and prerequisite
run identities are recorded at `/tmp/ic-timers-0191-qualification.IQiwIXf8/`.
Matching frozen-tag CI completes successfully, without replacing main's native
obligations. The single approved **early** dispatch is
[38234040990](https://github.com/dragginzgame/ic-timers/actions/runs/38234040990),
at the exact approved source/ref. Its receipt and original run record are saved
with the approval. Linux producer 114760214224 reaches controlled exit 22 and
successfully uploads artifact 11739996419. Its downloaded ZIP digest matches
the upload; the exact frozen verifier admits that archive locally, with the
original inputs and identity. This inspection is not hosted verifier acceptance.
ARM producer 114760214442 also reaches controlled exit 22 and uploads artifact
11741875127. Its uploaded/downloaded ZIP digest matches and the exact frozen
verifier admits it locally; the evidence owner retains its raw log and result.
Intel producer 114760214487 also reaches controlled exit 22 and uploads. All
three hosted download verifiers pass: Linux 114791741093, Intel 114791741108
and ARM 114791741301. The complete early stage is accepted at the frozen source;
all uploaded/downloaded ZIP digests, original logs and identities are retained.
Only then, one approved **late** dispatch is reconciled to
[38245658495](https://github.com/dragginzgame/ic-timers/actions/runs/38245658495)
at the same source/ref. Its intent, receipt and original run record are saved
with the approval. Late is queued at inspection; #44 stays open for its three
controlled producer/upload and three hosted download-verifier results.
Record intent before each dispatch, reconcile a missing reply without repetition,
and preserve the completed early-before-late order. This approval carries forward; do not
ask again or silently select another ref. Require each original producer to
reach the selected controlled failure/upload and all three download verifiers
to pass per stage; late also executes complete native product gates.

## Authority and next action

Inspect the already-dispatched late stage's actual controlled failures/uploads
and downloaded-verifier results, then reconcile #44 when all criteria pass.
Do not dispatch either approved stage again or change the frozen source. This pass
adds no product implementation or new release draft; evidence-only issue
acceptance does not justify another package identity.

AGENTS.md retains the standing locked/offline compile-only permission for
already prepared native/Wasm compilers. Other tests, doctests, Clippy, PocketIC,
setup and full validation remain user-owned outside the specific approved
qualification. Release execution, staging, commits, tags, pushes, publication
and cleanup stay user-owned. Preserve incoming lock edits and all old tools,
build/evidence artifacts. Siblings stay read-only; native substitutes are not
IC trap/rollback proof.
