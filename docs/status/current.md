![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and pending work

Released/default-branch base: **0.30.10**, `7a186003`. Incoming claim/qualification
cleanup is preserved. Cargo package versions remain 0.30.10; dependency catalog
and lock selections now adopt Build/Host/Testkit 0.67.0. The undated **0.30.11**
draft combines that adoption and compatible internal cleanup. Public timer APIs,
CLI flags, timer policy and recovery semantics are unchanged. Staging, commits,
pushes and release execution remain maintainer-owned.

Private reconciliation now receives a direct claim extractor instead of a trait
implemented only to expose three private fields. Factory destruction, callback
retention, conflict validation and fresh-registration rollback retain their order.
The native reconciliation fixture uses the same direct field access.

Qualification now selects Watchdog or Cohorts through a private enum; Cohorts
carries a required output path and optional baseline. Clap arguments normalize
once and the parent forwards that complete selection to the worker. Subject
admission, ignored-test rejection, deadlines, source/artifact guards, baseline
alias rejection and durable receipt publication retain their existing contracts.
Snapshot tests keep policy/cadence assertions; the existing callback conversion
table now also owns the millisecond case through both public decision types.

## Delivered ownership and dependencies

One private platform module owns one-shot dispatch and executor integration.
Provider effects preserve linear handle restoration/cleanup and defer capture
destruction until registry borrows and provider cleanup end. Native substitutes
remain test evidence; installed-IC rollback is not simulated by those tests.

Typed cohort context/artifact/metric records now require **schema 2**, including
a nonempty per-cohort Build recipe identity. Schema 1 rejects; regenerate reviewed
baselines. Comparison consumes the admitted report and refuses differing recipes. The mixed probe no longer maintains
parallel instruction totals; its host and canister records changed together.
Native sequences retain 768 distinct suspended traces with pre-cleanup outcome
assertions. Generic codec arithmetic belongs to Wire/Metrics; Timers retains
record, event-routing, callback-isolation and malformed-ingress integration proof.

Published Build/Host/Testkit **0.67.0** is selected atomically. Platform/macros
**0.6.13**, Wire/derive **0.6.12** and Metrics **0.15.8** were already current.
Official registry metadata and archive checksums verify these selections.
Host's three used crate source trees are unchanged from 0.66.3; Build requires
explicit configuration budgets. Timers supplies them at its one Wasm build call.
Both canonical lock-selected CLIs are refreshed and admitted offline; Testkit
retains its admitted PocketIC 16.1.0 server. No installer or server pin is copied.

Shared Tooling is reviewed commit `4c0f198e56d4c6d27b12a5c2d515e0e01070151e`,
with 137 payloads. Its committed VERSION still says 0.26.10; 0.26.11 notes contain
the setup phase/duration diagnostics. The supplier's dirty VERSION was excluded;
canonical impact/apply used a clean committed clone. Path-only stdout, quiet
offline checks and tool identities remain. CI tool caching remains disabled.

The published API from [Build #241 — Recipe identity](https://github.com/dragginzgame/ic-tooling/issues/241)
now supplies cohort comparison admission. Its digest covers relevant configuration,
profiles and explicit build/compiler settings without serializing values. Feature
identity remains cohort-owned; source/cache guards and lock/compiler/server checks
remain independent. Wrapper executable bytes and arbitrary build-script/external
inputs still require operator equivalence; no local configuration parser is added.

## Issue review

[Timers #76 — Correct README tool selections](https://github.com/dragginzgame/ic-timers/issues/76)
is closed after verifying the delivered README against `main` at 7a186003 and its
Make selections. This was advisory documentation work, not a CI/release gate.
[Timers #75 — Keep Wire alias rejection on the current graph](https://github.com/dragginzgame/ic-timers/issues/75)
has its source fix delivered. The latest delivered-source [CI run 40082577312](https://github.com/dragginzgame/ic-timers/actions/runs/40082577312)
has successful Linux, MSRV and macOS ARM64 results, but the macOS Intel complete
release gate was still running at review. A factual issue comment records that
remaining acceptance gate; close after the complete required matrix succeeds.
The repository has no open PRs at the review. Source changes in this batch remain
local and are independent of those already delivered issue fixes.

## Verification and limits

Current adoption evidence and incoming before-images:
`/tmp/ic-timers-upstream-refresh/`. Official package checksums/source review, locked metadata, dependency boundaries,
Rust 1.91 library/probe Clippy and Wasm checks pass. Report and driver regressions,
84 runtime tests, the actual release-editor fixture and Testkit adapter fixture
pass. Linux PocketIC passes 33 watchdog selections and one cohort subject; the
fresh schema-2 receipt has all four recipe identities and matches the current
lock. Both selected CLIs and Shared's 137 payloads verify offline.
The earlier cleanup validation remains at `/tmp/ic-timers-cleanup-0311/`.
No sibling source, Git index, own package version or release state was changed.
The prior issue review's external writes remain factual comments/closures.

Earlier evidence remains at `/tmp/ic-timers-shared-02610/`,
`/tmp/timers-cleanup/`, `/tmp/ic-timers-upstream.JrCXUA/` and
`/tmp/ic-timers-improvements.zG1CA1/`. Prior retained failures include
`/tmp/timer-qualification-971360-0/`; do not remove retained build/failure artifacts.
Hosted results qualify their delivered source, not this pending working tree.
Native macOS execution of this batch, installed-IC behavior and the complete
maintainer-operated release gate require their own qualification.
