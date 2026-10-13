![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-13

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Delivered source

The maintainer reports **0.21.3 live** at
`18ca1de856643f316b79c4a22774f5b0c34c423b`. The completed standard direct receipt
binds owner validation to source `51ff94b4a2ff7f78f5280dc93a4ca961dd160417` and
`435d71194f56b4513104d796c34e1dd9ee59c0cf`. Source-to-release changes are only
workspace versions and changelog finalization. The
[delivery owner](../releasing.md#0213-delivery-and-prepared-0214-cleanup)
records acceptance separately from earlier failures and the new cleanup; no
successful raw full-gate log or agent replay is fabricated.

Delivered graph: Metrics **0.8.2**, Testkit **0.35.1**, Host artifacts/fs/process
**0.15.2**, PocketIC **16.1.0**, Shared **0.9.13** across **68/36/11** files.
The canonical compiler-isolated, commit-free fixture remains directly invoked.
Original failed fixture/adapter evidence and prior frozen qualifications stay
with their existing owners. The 0.21 lifecycle semantics still publish new
registration slots atomically and preserve existing callbacks on later failures.

Hosted [run 38373566875](https://github.com/dragginzgame/ic-timers/actions/runs/38373566875)
passes; [run 38373566372](https://github.com/dragginzgame/ic-timers/actions/runs/38373566372)
is queued at inspection. No complete new hosted/macOS acceptance is claimed.

## Prepared 0.21.4 complexity cleanup

Internal callbacks now store erased closures directly inside `Rc<RefCell<...>>`,
removing the nested callback Box. Shared ownership, mutable borrowing and capture
cleanup timing remain; ordinary returned futures stay separately boxed/pinned.
This is an ownership-layer reduction, not a measured aggregate heap/Wasm saving.

Lifecycle reconciliation selects the existing/empty owner slot once. Remove the
unreachable conflict branch after an already-proven occupied slot. Fresh-slot
publication, exact identity/policy/lifetime checks, creation-error cleanup and
unused-factory destruction ordering remain. Public APIs and timer semantics are
unchanged; select the next patch, **0.21.4**, without Cargo version mutation.
Retain separate ordinary/watchdog failure paths, detached handle restoration,
confirmed-delivery tracking and boxed removed entries; their distinct obligations
make forced consolidation inappropriate.

[Cleanup evidence](../design/callback-delivery-ownership.md#prepared-0214-callback-ownership-simplification)
remains at `/tmp/ic-timers-complexity0214.CsNLubA0/`: original source/Cargo/index,
prepared patch and compiler logs. Compile-only library/test-target checks pass on
Rust **1.99.0** and **1.88.0**, with the Wasm library check passing on **1.88.0**;
all locked/offline with prepared inputs and `RUSTUP_AUTO_INSTALL=0`. No test
executes during these checks. Cargo inputs/index remain unchanged.

The maintainer approves focused native library tests and library Clippy. On Rust
1.99.0 all **154 native subjects pass** and library/all-target/all-feature Clippy
passes with warnings denied, with locked/offline prepared inputs. Existing
negative/capture, suspended-work, recurrence and reconciliation assertions remain.
Setup, PocketIC and release commands are excluded. No new Wasm, instruction or
heap measurement; the normal user-operated gate still owns cohort qualification.

## Issues and authority

The current issue inspection finds no open Timers issues.
[#50](https://github.com/dragginzgame/ic-timers/issues/50#issuecomment-6129282698)
remains closed from delivered Metrics adoption at 0.21.0.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) remains closed from
frozen v0.19.1 acceptance; original failures, artifact identities and restored/
missing evidence stay with the release owner. Shared fleet
[#98](https://github.com/dragginzgame/shared-tooling/issues/98) retains other
consumer obligations. No qualification dispatch is pending.

Siblings remain read-only. Tests, builds, lint, PocketIC and setup remain
user-owned outside explicit scoped approvals; prepared locked/offline library
compile-only checks retain their AGENTS.md exception. Cargo versions, staging,
commits, tags, pushes, publication and cleanup remain user-owned. Preserve
incoming edits and retained evidence.
