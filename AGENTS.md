# AGENTS.md

This file is normative for automated contributors.

## Session handoff

- Read `docs/status/current.md` first in a new session. Continue from that
  compact handoff instead of reconstructing the scaffold from chat history.

## Scope

- Make changes only in this repository unless the maintainer explicitly names
  another exact target and authorizes mutation there.
- Inspection, review, audit, diagnosis, design, and feedback requests for other
  repositories are read-only.
- Preserve unrelated dirty worktree state.

## Status

- This is a pre-alpha timer-runtime scaffold.
- Do not describe watchdog pre-arming, lifecycle reconstruction, shared
  inventory, metrics, or failure recovery as implemented until evidence exists.
- Keep the direct `ic-cdk-timers` dependency behind `platform`.

## Pre-1.0 hard cuts

- Before 1.0, replace superseded APIs and snapshot shapes directly. Do not add
  deprecated forwarders, compatibility aliases, dual readers, or fallback
  behavior for an earlier pre-1.0 release.
- Update current tests, documentation, and downstream adapters in the same
  change. Historical changelogs may describe removed behavior but do not
  justify keeping executable compatibility code.

## API and safety hygiene

- Distinguish inert snapshots from runtime authority. Snapshot values may be
  projected by consumers but must not become an alternate mutation path into
  timer control or platform handles.
- Public validation boundaries need negative tests. Production timer paths
  should return typed errors rather than panic when input or recoverable state
  can be invalid.
- Keep safety claims aligned with `SAFETY.md`; reserve recovery claims for
  behavior backed by the required PocketIC evidence.

## Delivery

- Update `CHANGELOG.md` and the open release-line note for meaningful changes.
- When the maintainer asks to prepare a named release, run the matching version
  bump so the populated `Unreleased` section is automatically promoted to the
  dated release heading. Do not leave a named release blocked on a manual
  changelog-heading edit.
- Run targeted checks for changed behavior. Do not run broad external suites
  unless the maintainer requests them.
- Release targets are maintainer-owned: do not commit, tag, push, or publish
  unless explicitly asked.
- Use Rust edition 2024 and directory modules when adding multi-file modules.
