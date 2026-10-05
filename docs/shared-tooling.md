# Shared Tooling adoption

IC Timers adopts reviewed revision [`f52c0e2476aee094359ed21de91c468540d3969f`](https://github.com/dragginzgame/shared-tooling/tree/f52c0e2476aee094359ed21de91c468540d3969f).
The [snapshot manifest](../.shared-tooling.snapshot) records twenty exact files,
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
The same normal target restarts preflight/validation failures on current source
and automatically reconciles saved preparation intent before choosing a new
increment. The local retry wrapper and its duplicate fixtures are removed.
The shared formatting hook and installer are vendored unchanged. Local formatting
targets sort both workspace catalogs and format Rust; explicit setup and hosted
CI use `cargo-sort` 2.1.4 from [tool-versions.env](../tool-versions.env).
The existing two-workspace release boundary and independently centralized
dependency catalogs remain explicit in [AGENTS.md](../AGENTS.md).
The numbered pending changelog follows the new shared version-selection rules;
package mutation and validation authority remain maintainer-owned.
The [host matrix](releasing.md#host-support) owns native qualification: Linux
syntax and command-stub passes do not qualify macOS or live IC execution.

This adoption was prepared as working-tree changes, without release execution or
upstream mutation. The exported 20-file snapshot verifies, shell and embedded
fixture syntax parses, and manifest sorting preserves effective workspace
metadata plus both lockfiles byte-for-byte. The GitHub description was inspected
and matches the current purpose. New recovery/hook fixtures and hosted setup
remain unexecuted; hook activation is a separate maintainer setup action.

The 0.13.0 release uses published registry `ic-metrics 0.1.3` in the root
dependency catalog and both lockfiles, with one resolved registry package per
workspace. The temporary sibling path and local 0.1.1 selection belong to the
earlier extraction, superseded by
[registry adoption](https://github.com/dragginzgame/ic-timers/issues/9). Its
initial focused library check reported an unused-assignment warning in separate
delivery-retirement work; that historical result does not qualify the current
runtime. Later focused evidence is scoped in the [handoff](status/current.md).

Focused measurement tests passed for role-specific accounting and registration
identity during extraction. Subsequent unrelated delivery-retirement changes
retain their own validation owner; these results do not qualify those changes.
