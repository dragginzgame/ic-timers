![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-30

## Source and accepted work

Local main is released **0.32.2**, `4337fbe46b7f31007c959696465c1dd6ce0138aa`.
The six runtime/fixture reductions are delivered. The maintainer then accepted
compiler-policy consolidation and a general current-documentation review.
That work is prepared locally with one undated **0.32.3** changelog section;
Cargo package versions remain 0.32.2. No staging, commits, pushes, release
commands, dependency updates or sibling source edits ran.

The incoming Cargo.lock selects pending Platform/macros **0.8.3**, versus the
released lock's 0.8.2. It remains byte-for-byte unchanged. This tooling/docs batch
does not qualify that supplier update. The other selected IC families remain
Tooling **0.70.2**, Wire/derive **0.7.23** and Metrics **0.17.5**.

## Compiler policy and setup

- Root `workspace.package.rust-version` owns the minimum compiler; the
  `rust-toolchain.toml` channel owns development selection. The current files
  select 1.91.0 and 1.99.1, respectively. Make and CI no longer carry their own
  version literals. `MSRV` command-line/environment overrides remain supported.
- `read-rust-policy.sh` projects both files into CI environment assignments,
  using Shared's workspace-floor reader and prepared jq/yq. Both reads must
  succeed before any assignment is published; malformed and failed producers
  refuse without selecting a fallback compiler.
- Local `make update-dev` and all four CI lanes prepare host parsers first,
  then select/prepare Rust before building Cargo tools. Explicit invalid updater
  arguments refuse before provisioning. The tag lane keeps rustfmt-only Rust
  preparation; normal checks remain read-only/offline and never install tools.
- Shared **0.27.3**, revision `7ee5c83744aa822ecd070703dbf0a7d95076d5f6`, is unchanged.
  Exported payloads were not edited. Existing native/bootstrap, tool admission,
  Testkit server ownership and product recovery gates remain required.

## Documentation

README now describes the released 0.32 API, uses 0.32.2 in its dependency example,
and names all four unpublished qualification packages. Setup/release guidance
names the canonical compiler files and actual preparation order. The hygiene
overlay now describes selected-handle cleanup, not the deleted restoration path.
The current Shared adoption guide retains ownership, deployment exclusions and
actual consumer checks; its former long narrative is preserved in the existing
adoption-evidence archive. Callback-delivery history is explicitly separated from
current architecture/safety authority. No historical measurements or downstream
qualification claims were rewritten as current evidence.

## Qualification

Current tooling/docs evidence: `/tmp/ic-timers-docs-rust.6yXhDp/`.
Passes: actual Make/updater and all CI preparation bodies with changed canonical
files; default/command/environment selections; host/Rust/tool failure ordering;
malformed policy and plausible-output parser failure rejection; strict ShellCheck
and actionlint; structured dependency/workflow declarations; actual release-gate
and annotated-tag/workflow fixtures; fixture completion; Shared snapshot integrity;
changelog admission, local document links and diff checks. All provisioning and
Rust execution in the preparation fixtures are substituted, not real installs.
Runtime code and manifests are unchanged; Rust/PocketIC suites were not rerun.
No fresh native macOS, full release/cohort gate or live IC qualification ran.

Released cleanup evidence remains at `/tmp/ic-timers-cleanup-yod5gscl/`: 86 runtime
and 40 registry tests, strict MSRV Clippy/Wasm and ten exact installed PocketIC
subjects. Its final artifact/evidence is under `installed-final/`, with Wasm SHA
`f1ba492e0a31eddb51252cd7a93bb2857cf65dd06598b1be1c8826cdc782b5b1`.
That evidence uses Platform 0.8.2, not the incoming pending 0.8.3 selection.

## Previously recorded issues

No GitHub status was queried or changed in this batch. Previously Timers #79
was open and #77/#78/#80 were delivered/closed; the #79 Timers predicate and
installed proof are delivered, while Canic delivery is independent. Scheduler
normal return remains the protocol assumption: arbitrary scheduler traps,
permanently live futures and uncertain-effect replay remain unqualified.

Earlier prepared Canic work remains at
`/tmp/ic-timers-backlog-03114.GlZdEf/canic`, with its parent `canic-code.patch`.
The original sibling checkout is untouched. Canic #695/#700, Platform #75 and
Tooling #267 retain their owners' delivery/qualification decisions. Canic's public
call-builder retirement remains subject to its accepted Human Minor Closeout
Gate. Earlier handoff observations are historical, not current downstream status.
