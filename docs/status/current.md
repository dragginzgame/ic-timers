![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-27

## Source and pending work

Released base: **0.31.10**, `2d06f7e`. The undated **0.31.11** draft now records
Shared Tooling 0.26.24, retaining the preceding uncommitted 0.26.23 adoption and
its consumer-hook fixture changes. The new refresh began with that pending work;
no reset, staging or commit was performed. Cargo manifests, lockfile and package
versions remain byte-identical to the incoming source.

This is compatible repository-only work: no timer API, implementation, protocol
or deployed dependency changes. Bundle it with the next code-bearing release by
default; an explicit maintainer-owned bump may select a repository-only patch
under the existing exception. All release execution remains maintainer-owned,
with the complete product and native-host gates intact.

## Shared adoption and ownership

Shared Tooling **0.26.24**, revision
`bd4d5541e388ca75e12509781711f1c6668db302`, is verified against upstream main
and adopted from a clean clone through canonical impact/apply with the existing
roots. All 133 canonical payloads verify. Relative to the pending 0.26.23 export,
one missing Node guide is added through declared governance companions; none
retire. The earlier formatter companion and local fixture edits are preserved.

The staged-file hook now keeps tracked submodules opaque. The standard Rust
formatter adds an explicitly selected standalone-file/edition interface, which
runs uncached. Timers has neither submodules nor standalone formatting inputs,
so its root grouped workspace selection needs no adapter or additional option.
The optional runtime-scanner source is not selected or compiled here; no scanner
package or Clap dependency change is needed. The Node guide is documentation,
not activation of Node setup, tooling or deployment gates.

Existing standard formatting retains visible pass timings, strict prepared-tool
admission and reuse only for unchanged successful inputs. Isolated hook trees
always execute both passes. The consumer fixture stages the formatter helper,
forces substituted failure passes with `FORMAT_RUST_ARGS=--no-cache`, and checks
actual Timers routing, partial-staging refusal, selected-file/index isolation,
lock-byte preservation, failure status/logs and stopping before Rustfmt.
Generic submodule, standalone-input and invalidation matrices remain upstream.
The [adoption guide](../shared-tooling.md) records these boundaries.

## Dependency boundary retained

Build/Host/Testkit remain **0.69.0**, with one Platform **0.7.0** package;
Metrics 0.16.3 and Wire 0.7.18 are unchanged. The library's existing executor/ic0
selection and qualified identities remain intact. Candid, Wire and host tooling
remain excluded from that deployed graph; native simulator tests retain required
transitive Candid through Platform/PocketIC. This refresh changes no dependency
input. Prior dependency/tool admission evidence remains at
`/tmp/ic-timers-tooling-03110.BwVqaF/`, with its original scope.

## Verification and limits

Current before-images, impact/apply output and focused logs are retained at
`/tmp/ic-timers-shared-03111-update.yvn2FT/`. Snapshot integrity verifies 133
payloads. The actual consumer hook fixture passes with the updated hook/driver,
including injected sorter failures and worktree/index isolation. Root formatting
runs both passes and records stable inputs; a second unchanged check reuses the
result. Cargo manifest/lock comparisons, Bash/Perl syntax, changelog and diff
checks pass. Release Make routing passes with an isolated substitute runner and
no release effects. Earlier 0.26.23 logs remain at
`/tmp/ic-timers-shared-03111.1gQlCY/` for their original inputs.

These are focused Linux tooling checks. No fresh Rust build/test, full release
gate, PocketIC measurement or native macOS run was performed. No claim of generic
submodule/standalone qualification is made for Timers' absent use cases; Linux
execution does not qualify native macOS.
