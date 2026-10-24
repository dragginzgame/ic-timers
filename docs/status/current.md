![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-24

## Source and accepted work

Released base: **0.30.8**, `f732b62`; Cargo versions remain unchanged. The
undated **0.30.9** draft includes the pending qualification/tooling batch,
registration-drop documentation and a private runtime effect-module extraction.
The extraction preserves transition bodies and cleanup ordering. No public API,
wire format, timer policy or recovery semantic changes are intended. Unlike the
earlier tooling-only batch, this adds crate source/documentation changes.
Staging, commits, pushes and release execution remain maintainer-owned.

Incoming lock selections are preserved: Platform/macros **0.6.12**, Wire/derive
**0.6.11**, Smallvec **1.16.4**, Metrics **0.15.7** and Build/Host/Testkit **0.66.3**.
The platform/wire refresh arrived independently during the preceding task; its
missing sources were prepared with `cargo fetch --locked`. The current registry check confirms these are the latest published direct IC
selections. The 0.66.3 Host family was already in the incoming lock; this adoption
preserves that lock rather than attributing an earlier selection to this task.
Clap 4.6.7, Serde 1.0.229 and Serde JSON 1.0.154 are also latest. Thiserror 2.0.21
cannot resolve with PocketIC 16.1.0's exact `=2.0.18` requirement; the precise
attempt failed without changing the lock, so 2.0.18 remains. No manifest selection,
Cargo version or lockfile mutation was made by this task.

## Shared Tooling and pending corrections

Shared **0.26.9**, `3657c044c0e91321effe8835989491a4cd0b5cba`, was the latest
reviewed upstream and is still published main at the current remote check. Canonical preview reported KEEP;
134 payloads verified unchanged. No optional audit or schedule was activated.

[#75](https://github.com/dragginzgame/ic-timers/issues/75): the alias-rejection
fixture now obtains Wire's prepared identity from successful locked offline
metadata instead of hard-coding Wire 0.5. A restricted cache reproduced the old
failure and passed the corrected real-Cargo matrix. No registry fallback or
weakened alias assertion was added. Default-branch delivery remains outstanding.

[#76](https://github.com/dragginzgame/ic-timers/issues/76): README setup now names
explicit optional IC tool selections; normal setup selects no IC executable
bundle. Testkit retains PocketIC admission/provisioning. The README API example
uses 0.30.8; two snippets previously compiled on Rust 1.91. Guidance is advisory,
not a new documentation gate. Neither issue was closed by these local changes.

## Qualification and ownership

The driver uses Host FS for inode-aware baseline alias admission and atomic
publication. Earlier failures preserve the previous report. Parents remain
caller-controlled; successful driver exit is required even if new bytes became
visible before a later filesystem error.

Qualification discovers the selected binary before canister builds. Watchdog
requires the driver's named critical recovery subjects; cohorts requires its
exact subject. A second listing rejects ignored selected tests. Host preserves
producer failures and bounds discovery output within the existing worker budget.

Every invocation owns a fresh private diagnostics directory. Failed runs retain
subject listings, candidate reports and per-instance Testkit stdout/stderr logs;
successful runs clean it. CI's existing temporary-root collection covers these
files. Parent admission requires matching build context/artifacts and exact
metric fields for four cohorts and two calibrations, then optional comparison,
then durable publication. Old final reports are never candidate evidence.
Successful hosted Linux/macOS jobs now upload cohort receipts for 14 days, named
by source, host and toolchain. Upload alone does not establish qualification.

[Build #241](https://github.com/dragginzgame/ic-tooling/issues/241) requests a
source-independent build-recipe identity. Published Build 0.66.3's fingerprints
combine source and configuration, so cross-source comparison still requires
operator-controlled identical ambient Cargo/Rust configuration. No local Cargo
configuration resolver or environment serialization was introduced.

Provider imports outside `platform` reject, including ordinary aliases. Private
`runtime/effects.rs` owns applying effects and restoring/clearing linear handles;
registry policy transitions remain atomic and together. New bounded native
sequences cover suspended commands, completion/abandonment and admission failure.
Registration-drop tests preserve the existing non-RAII contract: dropping a claim
does not unregister, cancel or free retained registry capacity.

Native Wire transport delegates to Testkit's typed call helpers; local adapters
retain Method identities, caller selection and directional limits. Raw malformed
frames remain necessary tests. Protocol records remain Wire/Serde-only. Tag-only
CI prepares host tools without formatters/editor/Testkit provisioning. Adoption
pages explicitly preserve historical evidence rather than assert current status.

## Latest dependency adoption

Official registry metadata and published archive inspection select Build/Host/
Testkit 0.66.3. Host and Testkit Rust APIs are unchanged from 0.66.2; Build changes
nested-workspace source/input observation, not the timer executor. Its generated
`target` exclusion retains declared inputs and remains Build-owned. No recipe-
identity API was added; #241 remains open. Both lock-selected CLIs are refreshed and admitted offline; Testkit retains the
same admitted PocketIC 16.1.0 server. Rust 1.91 library and probe Clippy, every
size-probe feature selection, six host regressions, Wasm compilation, complete
selected dependency-boundary checks and the actual release-editor fixture pass.
Linux PocketIC passes 33 recovery subjects and one policy cohort subject, then
publishes the fresh complete report. The reviewed Shared snapshot verifies and
remote main remains 0.26.9; the dirty supplier checkout is not adopted. Official
registry metadata, archive checksums, source comparisons and validation logs are
at `/tmp/ic-timers-upstream.JrCXUA/`; its `before.tar` preserves incoming work.
No Rust source or manifest change was necessary for this adoption.

## Evidence and limits

Prior focused evidence includes native/Wasm library checks, README compilation,
Shared integrity, tag/setup/boundary fixtures, six host regressions, 33 Watchdog
subjects and one cohort subject on Linux. The previous intentionally incompatible
baseline preserved the final file and retained a complete fresh diagnostic report.

Before the dependency adoption, the implementation batch passed Rust 1.91 Clippy
for library/driver, 86 focused runtime tests
(including 1,024 new suspended traces), six host regressions, Wasm cross-compilation,
warning-denied API docs, actions/provider/format checks and the boundary fixture.
PocketIC passes 33 Watchdog subjects and one cohort subject; exact-field admission
publishes the final receipt. The initial sandbox run failed before canister
execution at localhost bind; all 27 server stderr logs survived teardown. The
network-enabled retry passed and cleaned its diagnostics. Successful CI upload
wiring is inspected, not hosted-executed.

Current evidence: `/tmp/ic-timers-improvements.zG1CA1/`; before-images are in
`before.tar`. Expected sandbox-failure logs remain at
`/tmp/timer-qualification-3258-0/`.
Preceding evidence remains at `/tmp/ic-timers-subject-admission-s5wud59m/`,
`/tmp/ic-timers-followup-btrst1nt/`, `/tmp/ic-timers-shared-review-ooytw4fb/`
and `/tmp/ic-timers-new-deps-thq4p0u8/`. These are local evidence paths, not
hosted acceptance or permanent release receipts.

Native macOS, installed-IC behavior, artifact upload and the full maintainer-run
release gate require their own qualification. Linux and cross-compilation do not
close those gaps. No sibling source, Git index, release state or package version
was modified; the sole upstream write is Build issue #241.
