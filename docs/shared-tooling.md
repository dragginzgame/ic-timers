# Shared Tooling adoption

IC Timers adopts the engineering baseline at reviewed Shared Tooling revision
[`f4bd8657938836521493d3fda3e8387586d44504`](https://github.com/dragginzgame/shared-tooling/tree/f4bd8657938836521493d3fda3e8387586d44504).
The [local copy](shared-tooling/AGENTS.md) and its linked guidance contain the
exact committed documents at that revision. [Root AGENTS.md](../AGENTS.md)
identifies the baseline and owns the product overlay and maintainer-approved
validation exception. No Shared Tooling executable is adopted by this batch.

The copy is available offline. Do not edit its documents locally or inherit
changes from a sibling checkout. Refresh by reviewing another exact upstream
commit, copying these same documents from that commit, and updating this record
and the root reference together. Uncommitted upstream changes are separate from
the reviewed baseline. CI and releases do not fetch or refresh these documents.

## Local decisions

- The maintainer retains tests, builds, lint gates and all release execution.
  Automated contributors can update fixtures and prepare documentation, inspect
  source, check shell syntax and perform cheap metadata checks.
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
- Current hosted validation uses Linux x86_64. The reviewed shared host matrix
  describes Shared Tooling's scripts, not IC Timers qualification. This adoption
  adds no macOS execution evidence or portability claim. Existing GNU-dependent
  scripts and the Linux-only audited PocketIC artifact retain their current
  limitations; no dependency, toolchain or host-support change is made here.

## Verification scope

Document provenance is checked against the named commit, rather than a dirty
working copy. Release fixtures cover draft finalization, version-field projection,
user-owned validation, requested bumps and metadata rollback. Execution of those
fixtures remains maintainer-owned. Syntax and source inspection are weaker
evidence and must be reported separately from executed behavior.

On 2026-10-05, comparison of all copied documents against the pinned commit,
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
