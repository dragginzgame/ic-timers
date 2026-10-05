![IC Timers — Internet Computer helper library](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Documentation

If IC Timers is new to you, begin with the [project overview](../README.md).
It explains what the library does, when it is useful, and how its three
scheduling modes differ.

## Start here

- [Safety boundary](../SAFETY.md): a plain-English summary followed by the
  exact guarantees, limits, and application responsibilities.
- [Architecture](architecture.md): an overview of who owns durable and
  temporary state, followed by the implementation-level module design.
- [Observability contract](design/observability.md): the precise meaning of
  timer status, counters, instruction measurements, and memory observations.
- [Continuity and deadlines](design/0.8-registration-continuity-and-deadlines.md):
  registration identity, measurement limits, exact Watchdog scheduling, and
  downstream adoption requirements.

## Adoption records

These documents record specific integrations and the evidence available for
them. They are not general getting-started guides or proof that later versions
and dependency combinations have been qualified.

- [IcyDB adoption](adoption/icydb.md): shared-registry adoption, recovery
  evidence, and measured costs.
- [Canic adapter](adoption/canic.md): identity and policy mapping, custody
  boundaries, and the required atomic migration.
- [Toko Miner adoption](adoption/toko-miner.md): historical combined-system
  evidence, the later dependency-graph blocker, and remaining work.

## Design records

These are detailed engineering records. They preserve the decisions and
constraints for the release in which they were written.

- [Production timer runtime](design/0.3-production-timer-runtime.md): bounded
  registry, two-message Watchdog protocol, lifecycle seam, and recovery gate.
- [0.3 Patch 1 contract](design/0.3-patch-1-contract.md): frozen capacity,
  public API, policy state, counters, provider evidence, and measurements.
- [Policy-specific callback authority](design/0.5-policy-specific-callback-authority.md):
  typed work capabilities and request ordering.
- [Immediate Watchdog continuation](design/immediate-watchdog-continuation.md):
  immediate continuation, arbitration, rollback, and observation.

## Evidence and audits

- [Runtime evidence](audits/0.3-runtime-evidence-2026-08-13.md): PocketIC
  recovery matrix, Rust support, resource cohorts, and complexity results.
- [Immediate Watchdog evidence](audits/immediate-watchdog-continuation-2026-08-28.md):
  historical pre-release evidence for immediate continuation and retained
  recovery behavior.
- [Recurring code-hygiene audit](audits/code-hygiene.md): the current review
  checklist.
- [Initial hygiene report](audits/code-hygiene-2026-08-13.md),
  [follow-up report](audits/code-hygiene-2026-08-14.md),
  [0.3.7 report](audits/code-hygiene-2026-08-15.md),
  [0.4 report](audits/code-hygiene-0.4-2026-08-15.md), and
  [0.4.1 report](audits/code-hygiene-0.4.1-2026-08-15.md): historical review
  findings and fixes.

## Maintainer documentation

- [Releasing](releasing.md): version selection, validation, and publication.
- [Historical release evidence](changelog/README.md): recorded release subjects.
- [Shared Tooling adoption](shared-tooling.md): pinned baseline and local overlay.
- [Host support](releasing.md#host-support): required macOS workflows and current
  qualification gaps.
- [Current status](status/current.md): compact handoff for the next maintainer
  session.
