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

## Delivery

- Update `CHANGELOG.md` and the open release-line note for meaningful changes.
- Run targeted checks for changed behavior. Do not run broad external suites
  unless the maintainer requests them.
- Release targets are maintainer-owned: do not commit, tag, push, or publish
  unless explicitly asked.
- Use Rust edition 2024 and directory modules when adding multi-file modules.
