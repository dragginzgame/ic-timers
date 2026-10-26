![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-26

## Source and pending work

Released base: **0.31.2**, `50ee4ca`. The undated **0.31.3** section covers
compatible dependency qualification and Shared Tooling adoption. Cargo package
versions remain 0.31.2. No timer API, runtime semantic, probe protocol or MSRV
change requires a minor release. This is tooling/dependency qualification;
publication can wait for the next code-bearing release. Commits, staging, pushes
and release execution remain maintainer-owned.

The incoming lockfile already selected published Wire/derive **0.7.15**,
Build/Host/Testkit **0.68.6** and Platform/macros **0.6.17**. It is preserved byte
for byte. Registry metadata and checksum-verified published archives confirm
these selections and Rust 1.91 support. Wire 0.7.16 in the sibling checkout was
unpublished when checked and is not adopted. Metrics **0.16.2** and Thiserror
**2.0.18** remain selected; PocketIC requires that exact Thiserror version.

## Ownership and adoption

The library still uses only Metrics, Platform and Thiserror directly. Platform
retains executor **2.0.0** and IC0 **1.2.0** identities. Library and probe deployment
graphs remain Candid-free; native Testkit/PocketIC transport intentionally retains
Candid. The normal host coordinator excludes that transport. The protocol package
owns Wire/Serde records; timer policy, authority and callback recovery stay local.

Wire runtime codecs are unchanged. Its host generator API changes are outside
Timers' selected features; probe schemas and budgets remain intact. Platform and
macro Rust sources match 0.6.16. Tooling changes build resolution, metadata editing,
filesystem and startup internals; the existing APIs need no local adapters.
Both lock-selected 0.68.6 CLIs were explicitly prepared and admitted offline;
Testkit still selects PocketIC 16.1.0. CLI build inputs use the setup compiler;
probe/build qualification selects Rust 1.91 separately after server admission.

Shared Tooling **0.26.19**, revision
`c830c33447f0a84e015bda7fa14b6c096a58368e`, supplies **135** payloads through its
canonical exporter. The source VERSION/changelog declare 0.26.19 despite the
0.27.19 commit subject. Roots and retired producer fixtures are unchanged.
Normal release commands now start fresh; exact recovery requires
`release-resume VERSION=x.y.z`. Local instructions and release guidance match.
Build/validation evidence and current-source/ref checks remain; no real release
operation ran. CI caching stays off and all product/native-host gates remain.

## Verification and limits

Evidence, published archives, exporter impact/apply and the incoming lockfile are
in `/tmp/ic-timers-adoption-0313/`. Rust 1.91 Clippy passes for all five affected
packages. Library Wasm checking and optimized runtime-probe compilation pass.
Protocol, build-driver and report tests pass (three each). Three Linux PocketIC
subjects pass: typed Wire rejection, trapped-work successor/executor privacy, and
shared-watchdog trap/upgrade recovery. The actual cohort driver passes all four
policies and both calibration subjects, retaining a separate schema-2 receipt at
`/tmp/ic-timers-adoption-0313/cohorts.json`; existing baselines were not replaced.

Formatting, pin/direct/deployed dependency boundaries, provider ownership,
snapshot integrity/corruption, local fixture completion, Testkit adapter,
changelog and metadata-preparation/release-gate fixtures pass. The reviewed
supplier release-runner command-stub fixture passes from the temporary clone,
without adding producer matrices to the snapshot or routine gate.

The first integration invocation selected Rust 1.91 before CLI admission, which
correctly refused an unprepared CLI build identity and left the tests without an
admitted server. Its failure log remains retained. The passing retry admitted the
prepared CLI first, then selected Rust 1.91 for probes; no admission was weakened.
No full release suite, baseline performance comparison, native macOS or live
deployment qualification ran. Prior release evidence remains at its original
owners and is not relabelled as qualification of this selection.
