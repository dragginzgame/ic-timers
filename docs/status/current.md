![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links the [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md) and [safety boundary](../../SAFETY.md).
The earlier accumulated handoff remains in [its historical archive](archive-2026-10-10.md).

## Released source and qualification

Released **0.17.8** is `afb773f20ed7bf4e11d86f8d2e5db916fe9f1654`.
Its committed graph selects Metrics **0.6.1**, Testkit **0.33.3**, all four Host
packages at **0.13.0**, PocketIC **16.1.0** and Shared **0.3.11**
`db143ae9ae140eb66826c413360723d23c078f66` through 56/30/11 exports.

[Main CI](https://github.com/dragginzgame/ic-timers/actions/runs/38125888063)
now passes Linux, MSRV and both native macOS gates.
[Tag truth](https://github.com/dragginzgame/ic-timers/actions/runs/38125888250)
passes at the same SHA. Downloaded plaintext logs, run/job metadata and hashes
are retained at `/tmp/ic-timers-0178-acceptance.ncHCWdZo/` with matched references
at the [0.17.8 evidence owner](../releasing.md#0178-release-evidence).
All three hosts actually execute the diagnostic fixture, 144 library tests,
14 recovery subjects and four policy cohorts. This establishes acceptance for
[#42](https://github.com/dragginzgame/ic-timers/issues/42) and
[#43](https://github.com/dragginzgame/ic-timers/issues/43), now closed. It does not qualify
new tool/dependency bytes or an intentional live-failure artifact round trip.

The [Metrics identity owner](../design/callback-delivery-ownership.md#ic-metrics-06-released-graph)
records the public boundary delivered in the 0.17.7 patch in error. Future public
hard cuts require a minor release; do not restore a shim. Native reset evidence
is not IC upgrade/rollback qualification; existing PocketIC contracts retain that role.

## Pending 0.17.9 tooling adoption

All three canonical exports select reviewed Shared **0.5.0**
`a306180ce534227fcb98b882fba9a526d5d747e0`, with **56/32/11** files. The audit-setup
export adds the two required store/fixture companions. Exact-source producer CI
was queued at inspection; no native acceptance is inferred. The earlier 0.4.2
policy-only pass was interrupted, and its preserved inputs/logs remain separate.
[The adoption record](../shared-tooling.md#shared-tooling-050-storage-adoption)
owns committed source, previews and the current refresh boundary.

Host/IC setup now reuses verified shared bundles with each checkout's own pins.
Command paths remain `.tools/host/bin` and `.tools/ic/bin`. Run
`make install-tools` then `make tools-check`; previous physical local host/IC
bundles remain retained but no longer pass active admission. Use the same
`SHARED_TOOLING_STORE` setting for setup, checks and evidence if overriding the
per-user default. Do not relocate/copy executables, create another store manager,
use sibling PATHs or delete retained installations/evidence. Cargo tool slots
remain local and common commands use `.tools/rust/common/bin`.

The shared selector admits only referenced store bundles for the local collector.
Prepared local cases now use actual canonical installers with isolated fake
archives; check compact provenance, damaged shared binaries/libraries and
unrelated-entry exclusion. Intentional early-failure qualification has a private
scenario store so a cached success cannot bypass the injected failure, and that
physical scratch remains archived. Copied entrypoints carry the new helper.
These cases are written, not executed; full native acceptance remains pending at
the [release owner](../releasing.md#shared-050-storage-adoption).

Incoming maintainer selections **Metrics 0.6.2 / Testkit 0.33.5 / all four Host
0.13.1** remain preserved byte-for-byte. PocketIC remains 16.1.0 and Cargo 0.17.8.
Testkit remains the sole Host consumer and owns its installed CLI's packaged lock
and PocketIC admission. Binaryen 132 and other consumer pins, raw index and
staging are preserved.

[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains open for actual
setup/reuse, corruption/interruption and local evidence fixtures, formatting and
complete Linux/Intel/ARM qualification of new tools and graph. The single undated
**0.17.9** changelog remains a patch under package-impact rules: the published
library contract and consumer build requirements are unchanged. No timer API
change or measured Wasm/instruction saving is claimed. Tests/builds/lint/setup,
Cargo mutation, staging/commits and release remain user-owned; preparation uses
only allowed source, syntax, integrity, documentation/diff and metadata checks.
Those checks pass, including all three snapshots and full locked offline metadata;
catalog/lock/pins and staging compare unchanged with preserved incoming inputs.
