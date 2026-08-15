# Documentation

- [Architecture](architecture.md): current boundary, intended runtime, and
  consumer integration.
- [Observability contract](design/observability.md): implemented canonical
  timer snapshot, counter semantics, and validated Canic adapter mapping.
- [0.3 production runtime design](design/0.3-production-timer-runtime.md):
  bounded registry, two-message watchdog protocol, lifecycle seam, and
  completed recovery evidence gate.
- [0.3 Patch 1 contract](design/0.3-patch-1-contract.md): frozen capacity,
  public API, policy/state, counters, provider evidence, MSRV, and measurement
  decisions before runtime implementation.
- [Safety boundary](../SAFETY.md): implemented guarantees, missing recovery
  behavior, and required evidence.
- [Code-hygiene audit](audits/code-hygiene.md): recurring mechanical and API
  review checklist.
- [Latest hygiene report](audits/code-hygiene-2026-08-15.md): module hierarchy,
  public-surface, duplication, and comment audit.
- [Prior hygiene report](audits/code-hygiene-2026-08-14.md): callback-authority
  lifetime and release monotonicity findings, fixes, and evidence.
- [Initial hygiene report](audits/code-hygiene-2026-08-13.md): adopted peer
  practices, 0.3 closeout, and the 0.3.2 audit addendum.
- [0.3 runtime evidence](audits/0.3-runtime-evidence-2026-08-13.md): PocketIC
  matrix, Rust 1.88 verdict, Wasm/instruction/cycle cohorts, complexity, and
  downstream adoption sketch.
- [IcyDB adoption record](adoption/icydb.md): accepted shared-registry hard cut,
  downstream recovery evidence, and measured costs.
- [Canic adapter contract](adoption/canic.md): required pre-1.0 hard cut,
  identity/policy mapping, custody boundary, and parity gate.
- [Releasing](releasing.md): crate-impact classification, pre-1.0 SemVer, and
  version/release commands.
- [Changelog lines](changelog/README.md): release-line working notes.
- [Current status](status/current.md): compact handoff for the next session.
