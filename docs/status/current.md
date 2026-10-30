![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-30

## Source and accepted work

Local main is released **0.32.1**, `a1dc2856a515ae5bbb8d48bf8e6e49fbc6d881a9`.
The maintainer accepted the six complexity reductions from the read-only review.
They are implemented locally, with one undated **0.32.2** changelog section.
The public API and timer semantics are preserved; Cargo package versions remain
0.32.1. No staging, commits, pushes, release commands or sibling source edits ran.

The incoming pending Cargo.lock already selected Tooling **0.70.2**, Platform
**0.8.2**, Wire/derive **0.7.23** and Metrics **0.17.4**, plus unrelated updates.
During validation Metrics changed externally to **0.17.5**. Both observations are
retained; the final observed lock is preserved byte-for-byte. This cleanup made
no catalog, lockfile or dependency-version edits. Dependency adoption/provenance
review of that incoming batch is separate from these focused consumer checks.

## Cleanup and ownership

- The registry now selects only handles that an effect will clear after its
  transition. Unaffected work and successors remain bound; removed entries
  transfer every handle before callback captures are released. Runtime clears
  detached capabilities before arms and capture release outside registry access.
  The handle-restoration path, token-bearing cleanup wrapper and claim-detachment
  helper are deleted. Scheduler
  delivery still consumes its exact fired handle before the transition; Watchdog
  work completion retains its separate trap/IC rollback contract.
- Watchdog completion resolves its winning stop/schedule action and observation
  metadata together before applying it. Pending precedence, invariant failure,
  exact relative-origin metadata and successor retention are unchanged.
- Native registration/reconciliation fixtures and installed probe rejection/removal
  subjects share their immediate stopping callback constructors. Real-await,
  trap, authority and compile-fail subjects keep their distinct contracts.
- The scheduling trace discards unused registration capabilities and keeps only
  an explicit started flag. Registry-owned callback lifetime and context-driven
  self-cancellation are unchanged.
- The standalone snapshot-corruption suite and its Make/completion-roster callers
  are deleted. Normal snapshot admission remains; the actual tag-workflow fixture
  still checks copied-export corruption refuses before Cargo graph admission.
  Shared owns the generic verifier matrix at the reviewed snapshot revision.

Timers still owns volatile scheduling, callback authority, observations and its
private one-shot backend. Platform owns executor/system interfaces, Metrics owns
measurement arithmetic, Wire owns probe codecs, Build owns Wasm artifacts,
Testkit owns PocketIC admission/lifecycle, and Shared owns common tooling.
The normal coordinator remains free of simulator/codecs; deployed roots keep
one Platform/executor/IC0 owner and their existing dependency exclusions.
Shared **0.27.3**, revision `7ee5c83744aa822ecd070703dbf0a7d95076d5f6`, is unchanged.

## Qualification

Evidence: `/tmp/ic-timers-cleanup-yod5gscl/`.

Passes on the final observed graph: **86 runtime tests**, including the new
unaffected-handle binding-fault regression and bounded command sequences;
**40 registry tests**; strict Rust 1.91 library/runtime-probe Clippy; library-only
MSRV Wasm; formatting; provider and native/Wasm dependency boundaries; Shared
snapshot integrity; local fixture-completion checks; actual annotated-tag/clean
publication/workflow fixture; changelog admission and diff checks.
The first lint pass caught ignored must-use trace capabilities; explicit
intentional discard fixes those sites. MSRV rustfmt is not installed; formatting
uses the existing development toolchain and the maintained formatter gate.

Explicit canonical setup prepared the root-lock Testkit 0.70.2 CLI; its offline
check admitted the owner-selected PocketIC 16.1.0 binary. A disposable registry
Build/Host 0.70.2 witness built the actual optimized runtime probe on Rust 1.91
and held its artifact evidence across **ten exact installed PocketIC subjects**:
scheduling trace; armed callback capture release; rejected initial registration
and reconciliation; scheduler/work-gap cancellation; immediate continuation;
exact deadline/registration replacement; trapped work with committed successor;
all six round-pressure scheduler-abandonment modes; dispatch-response trap cleanup.
Every subject executed once and passed; Build inputs remained current throughout.
Retained Wasm SHA-256:
`f1ba492e0a31eddb51252cd7a93bb2857cf65dd06598b1be1c8826cdc782b5b1`.
The final installed logs and Wasm are under the evidence root's `installed-final/`;
the earlier wrapper-removal intermediate run is retained separately.

These are Linux native/MSRV/Wasm and installed PocketIC checks. No fresh native
macOS, full release/cohort gate or live IC qualification ran. Preserve the normal
maintainer-operated release and host requirements. Earlier dependency evidence
at `/tmp/ic-timers-tooling-wire.r_zgg1ky/` retains its original 0.70.0/Platform
0.8.1/Wire 0.7.22 graph scope.

## Previously recorded issues

This cleanup did not query or change GitHub issue state. The previous triage
recorded only Timers #79 open, with #77/#78/#80 delivered and closed. Timers'
#79 predicate and installed proof are delivered; Canic delivery is independent.
No arbitrary scheduler-trap, permanently live future or uncertain-effect replay
claim is added by this cleanup.

Earlier prepared Canic work remains at
`/tmp/ic-timers-backlog-03114.GlZdEf/canic` with its parent `canic-code.patch`.
The original sibling checkout stays untouched. The old handoff's Canic main
observations are historical, not a current delivery check. Canic #695/#700 and
Platform #75 retain their own delivery/qualification decisions. The public Canic
call-builder retirement remains subject to its accepted Human Minor Closeout
Gate; no next-minor work was started here.

[IC Tooling #267 — Adopt Platform 0.8 in Testkit](https://github.com/dragginzgame/ic-tooling/issues/267)
had a published supplier correction and Timers consumer proof at the last read.
It was still open then; supplier closure uses its own qualification rather than
a consumer publication/acknowledgement gate.
