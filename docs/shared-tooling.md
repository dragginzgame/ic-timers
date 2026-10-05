# Shared Tooling adoption

IC Timers adopts reviewed revision [`b8537873ac124ad17b30e32aa23e9006a3e6ec21](https://github.com/dragginzgame/shared-tooling/tree/b8537873ac124ad17b30e32aa23e9006a3e6ec21).
The [snapshot manifest](../.shared-tooling.snapshot) records sixteen exact files,
including [the baseline](../DRAGGINZGAME.md), linked governance, release runner and
verification fixtures. [AGENTS.md](../AGENTS.md) owns the product overlay and the
maintainer-approved validation and release-authority exceptions.

Refresh through the upstream distribution helper from a clean reviewed checkout,
then verify `bash scripts/ci/verify-shared-tooling-snapshot.sh`. CI and releases
use the offline snapshot; a mutable sibling checkout supplies no authority.
The former document-only copy under `docs/shared-tooling/` is retired.

The shared runner owns the three standard SemVer entry points, ordering, Git
operations and exact-version recovery. Consumer adapters retain the five release
metadata outputs, package identity, README projection, both lockfiles and the
complete PocketIC release gate. Publishing remains a separate command.
The local standard entry point archives only preflight/validation attempts
before dispatching a fresh run through the unchanged pinned runner. Its release
lock protects retry admission; preparation and later phases keep exact-version
recovery. The [local procedure](releasing.md) owns this narrower retry behavior.
The [host matrix](releasing.md#host-support) owns native qualification: Linux
syntax and command-stub passes do not qualify macOS or live IC execution.

The exact local ic-metrics pin and both lockfiles now select the maintainer-tagged
0.1.1 package. Locked offline metadata resolves one canonical package in both
workspaces. The focused library check completes with an unused-assignment warning
in separate delivery-retirement work (`runtime/mod.rs:1090`); it is not a
warning-free qualification of that work. The temporary path remains pending
[registry adoption](https://github.com/dragginzgame/ic-timers/issues/9).

Focused measurement tests passed for role-specific accounting and registration
identity during extraction. Subsequent unrelated delivery-retirement changes
retain their own validation owner; these results do not qualify those changes.
