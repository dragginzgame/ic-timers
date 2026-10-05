# Shared Tooling adoption

IC Timers adopts the engineering baseline at reviewed Shared Tooling revision
[`ca319ba05c5a8016f3cdbf5af073fca2e6279268`](https://github.com/dragginzgame/shared-tooling/tree/ca319ba05c5a8016f3cdbf5af073fca2e6279268).
The [local copy](shared-tooling/DRAGGINZGAME.md) and its linked guidance contain the
exact committed documents at that revision. [Root AGENTS.md](../AGENTS.md)
identifies the baseline and owns the product overlay and maintainer-approved
validation and command-authority exceptions. This adoption includes documents
only; no Shared Tooling executable, snapshot manifest or verifier is installed.

The copy is available offline. Do not edit its documents locally or inherit
changes from a sibling checkout. Refresh by reviewing another exact upstream
commit, copying these same documents from that commit, and updating this record
and the root reference together. Uncommitted upstream changes are separate from
the reviewed baseline. CI and releases do not fetch or refresh these documents.

This uses the baseline's revision-bound reference option. Eight unmodified
documents are copied under `docs/shared-tooling/`, retaining their relative paths:
`DRAGGINZGAME.md`, `docs/consuming-snapshots.md`, `docs/supported-hosts.md` and
the five files in `docs/principles/`. Export their bytes from the exact source
commit with `git cat-file blob <revision>:<path>`, rather than copying a dirty
working tree. Keep [root AGENTS.md](../AGENTS.md) local. The former copied
`AGENTS.md` was replaced by `DRAGGINZGAME.md`; no parallel baseline is retained.

## Local decisions

- The maintainer retains tests, builds, lint gates and all release execution.
  Automated contributors can update fixtures and prepare documentation, inspect
  source, check shell syntax and perform cheap metadata checks. These explicit
  validation and command-authority exceptions preserve the maintainer's existing
  deployment workflow; the refreshed baseline does not revoke them.
- One top changelog draft records the accepted batch. It is versionless until
  the maintainer selects a target; a continuation does not select another release.
  The user's bump command resolves the label and date. Historical release notes
  remain evidence, without a required new per-version note or status marker.
- Package identity, lock resolution, annotated tags, main reachability and the
  exact PocketIC artifact remain real validation boundaries. Changelog layout
  and explanatory release prose are not deployment gates.
- Release staging selects the bump's five metadata outputs. Inherited member
  manifests and supporting documentation retain separate maintainer ownership;
  the preparation fixture checks that they are not included by release staging.
- The native platform substitute does not establish IC rollback or provider
  heap behavior. Those claims require the maintained PocketIC evidence.
- macOS support is required by the refreshed baseline. The
  [local host matrix](releasing.md#host-support) distinguishes that requirement
  from qualification evidence and records known workflow gaps. Current hosted
  validation uses Linux x86_64; this refresh supplies no native macOS evidence,
  support exception or host-tooling change. The shared host matrix describes
  Shared Tooling's scripts, not IC Timers qualification.

## Verification scope

Document provenance is checked against the named commit, rather than a dirty
working copy. Release fixtures cover draft finalization, version-field projection,
user-owned validation, requested bumps and metadata rollback. Execution of those
fixtures remains maintainer-owned. Syntax and source inspection are weaker
evidence and must be reported separately from executed behavior.

For the original `f4bd8657938836521493d3fda3e8387586d44504` adoption on
2026-10-05, comparison of all copied documents against that commit,
changed shell and embedded Perl syntax checks, local document link targets,
diff whitespace, the current README version projection and read-only draft
finalization checks passed. Cargo manifests and lockfiles were unchanged.
No fixture suite, build, lint gate, version bump or release command was run.
These checks do not establish runtime behavior or exercised rollback.

The maintainer subsequently selected 0.11.5 for this batch. Its named undated
changelog preflight, current README projection, both locked offline metadata
checks, changed fixture syntax and diff whitespace checks passed. Release staging
was inspected and its existing fixture extended with an unrelated member-manifest
edit; that fixture was not executed. Version mutation and all Git effects remain
maintainer-owned.

The maintainer supplied passing draft-finalization fixture output before its
diagnostic cleanup. That fixture now captures successful child output, including
the warnings deliberately produced by empty-note scenarios, and prints the
captured diagnostic on unexpected failure. Actual release preparation retains
its empty-note warning. The changed fixture has only been checked for shell syntax
and reviewed; it has not been re-executed by an automated contributor.

For the refresh to `ca319ba05c5a8016f3cdbf5af073fca2e6279268`, all eight copied
documents match their committed source bytes and non-executable modes. The source
checkout's dirty `DRAGGINZGAME.md` and changelog were excluded. Local document links,
baseline references and diff whitespace were checked. This is documentation-only
work: no tests, builds, lint gates, host qualification, version changes or Git
release execution were performed. Runtime and deployment behavior is unchanged.
