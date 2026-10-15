![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Releasing

This is the current maintainer procedure. Source-bound preparation receipts and
past commands are preserved in [release history](evidence/release-history.md);
the compact [handoff](status/current.md) records work awaiting delivery.
Release execution, normal staging, commits, tags and publication are user-owned.

## Root workspace and dependencies

The root [Cargo.toml](../Cargo.toml) owns all five packages, their direct dependency
catalog and one [Cargo.lock](../Cargo.lock). Members inherit versions and
`workspace = true` dependencies. `ic-timers` is the default published member;
the four packages under `testing/crates/` are unpublished. There is no separate
probe workspace or lockfile. Library commands select `-p ic-timers` explicitly.
Probe builds retain the root `timer-probe` profile and ignored `testing/target`
cohort directories. Optimization policy belongs to application qualification;
a small probe alone does not establish a fleet-wide LTO choice.

## Pre-1.0 compatibility

Every superseding API or semantic change is a hard cut: remove the replaced
contract and update current callers, tests and documentation together. A public
break requires the next minor line; a patch cannot remove public items merely
because the crate is pre-1.0. Do not retain aliases, deprecated forwarders,
migrations or fallback readers without an explicit compatibility exception.

## Repository updates versus crate releases

Use `make release-impact` to classify changes since the current version tag.
The classifier distinguishes crate, repository-only and empty subjects; it is
not an API compatibility oracle. Repository-only work normally joins the next
code-bearing release. An explicit maintainer patch/minor command may publish it
with an advisory and the complete gate; an empty subject is rejected.

Maintain one numbered, undated pending section at the top of
[CHANGELOG.md](../CHANGELOG.md). The matching bump dates it automatically.
Missing files, versionless headings, misplaced pending sections and conflicting
version selections fail before mutation. Empty notes are advisory; exact
previous-release history can seed an empty candidate. Release prose checks are
advisory before mutation. README versions and layout do not gate delivery;
periodic documentation maintenance owns staleness checks.

## Standard release runner

After reviewing and committing implementation changes, the maintainer runs the
matching standard command, for example:

```sh
make release-minor
make publish
```

`release-patch`, `release-minor` and `release-major` use the reviewed
[Shared release contract](releases.md), explicitly selecting direct delivery to
`origin/main`. They validate, prepare metadata, stage admitted outputs, commit,
create an annotated version tag and atomically push the branch and tag.
Publication stays separate. PR release delivery is not adopted.

Normal retry reconciles saved unfinished intent at its exact version before
considering another increment. `make release-resume VERSION=x.y.z` explicitly
resumes that saved release. A saved plan alone does not require manual deletion
or block normal retry. New descendant fixes or a different increment receive
fresh validation after the old release is completed. Late checks bind to
`RELEASE_COMMIT`, independently of HEAD; they inspect its immutable metadata
with current helpers and never execute scripts from the exported tree.

Preflight admits only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md` as mutable
release outputs. Other staged, unstaged or untracked changes must be resolved by
the maintainer. It checks version/tag/impact/notes, then runs `fetch`,
`install-testkit-server` and `pocketic-check` sequentially before validation or
version preparation. Locked fetch prepares the selected graph; it cannot repair
a stale lockfile after dependency edits. No helper stages unrelated changes.

Metadata-only `make patch`, `make minor`, `make major` and
`make bump-x VERSION=x.y.z` support preparation from a dirty worktree. They do
not run tests or deliver a release. Shared owns literal workspace version
rendering, all local lock records, changelog finalization, complete locked/offline
graph admission and rollback of those three files, including modes, on ordinary
errors and INT/TERM. SIGKILL cannot run rollback; retain its reported backup for
manual recovery. External dependency selections are preserved.

Protected commands reject Make flags that would skip execution, ignore errors
or pretend success. `publish` requires a clean checkout, the exact annotated
version tag at HEAD and a verified package before publishing `ic-timers` to
crates.io. Cleanup is separate; release commands never erase evidence.

## Deployment validation

The maintainer-operated `make release-verify` retains this fail-fast order:

1. `fetch` for the locked root workspace.
2. `install-testkit-server` for explicit selected CLI/server setup.
3. `pocketic-check` for offline admission.
4. `ci`: Actions/pins, shell syntax, provider boundary, formatting, library
   compilation/lint, release fixtures, public docs, tests and Wasm/package checks.
5. `msrv`: native/test-target and Wasm library compilation plus doctests on 1.91.
6. `testing-check`: probe/protocol lint and each of the four exclusive cohorts.
7. `pocketic-watchdog`: the maintained real-canister runtime/recovery subjects.
8. `pocketic-cohorts`: baseline/Once/AfterCompletion/Watchdog Wasm and instruction
   measurements using the exact artifacts installed into PocketIC.

The native protocol tests belong to `make test`. Native runtime tests use a
substitute platform and cannot establish IC rollback, trap recovery or upgrade
behavior. Real PocketIC gates retain their own assertions and admitted server;
focused source checks do not replace them. Compare Wasm/instruction results only
with compiler, graph, features, build profile and measurement subjects recorded.

Ordinary focused checks follow the adopted [cadence](../DRAGGINZGAME.md#validation-and-evidence)
and [local command overlay](audits/code-hygiene.md#inspection-and-validation-commands).
Broad aggregate gates, setup, live PocketIC, hosted dispatch and release execution
retain their separate authority. Missing offline inputs are reported rather
than silently fetched or installed.

Full Rust/probe/MSRV validation runs on PRs and `main`. The smaller tag lane
checks the actual annotated tag/version, main reachability, the unified snapshot
payload and the locked graph without repeating broad release fixtures or full
Rust builds for the same commit. A successful tag lane alone is not product
qualification.

## Host support

Required native hosts are Linux x86_64, Intel macOS and Apple Silicon macOS,
as selected by [CI](../.github/workflows/ci.yml). macOS checks its actual OS,
architecture/version and genuine Bash 3.2 before running the native gate.
Linux evidence does not qualify macOS. Do not weaken admission or product gates
to close a host gap.

The last recorded complete all-host acceptance here is released 0.23.1 at
`2d54724bc836ca09308d6742bf95c3f1ddecc3f4`; its exact run/jobs and graph are in
[the receipt](evidence/release-history.md#completed-0231-native-acceptance).
It does not qualify later source, the incoming Testkit 0.45 graph or this pending
0.24 cut. New hosted evidence must bind to its exact source and selections.

## Formatter prerequisites

`make fmt` / `make fmt-check` use the shared manifest/Rust formatting owner for
all five packages, with prepared tools and failure evidence. The repository-local
hook uses that same owner; focused fixtures retain actual-index preservation,
partial failure and native Bash behavior. Install tools explicitly through
[local setup](local-setup.md); checks do not provision them.

## Structured dependency checks and host parsers

`make actions-check` verifies the host bundle and delegates structured YAML/Cargo
pin and workspace-inheritance checks to the reviewed Shared helper. Every direct
dependency belongs to the root catalog and is inherited by members. Host/IC/Cargo
setup and evidence use one shared store setting and checkout-local references;
no global tool or stale-generation fallback is admitted. See
[local setup](local-setup.md) and [adoption](shared-tooling.md) for exact owners.

## Pinned IC tool setup

Shared catalogs select default host/IC/Cargo tools. The explicit
[qualified IC matrix](../ci/ic-tools-qualified.tsv) retains Binaryen 132 pending
application optimizer qualification; refreshing a snapshot does not qualify a
new optimizer. `make install-tools` explicitly prepares host, IC and Cargo tools,
then Testkit. `make tools-check` verifies them offline in the same order.

The [Testkit adapter](../scripts/dev/testkit-server.sh) selects the CLI from the
single root lock. Shared's Cargo installer admits that exact executable; Testkit
owns its server assets, checksums, setup and compatibility admission under
`.tools/testkit-server`. `make install-testkit-server` is explicit setup;
`make pocketic-check` is offline verification and prints the admitted absolute
server path. There is no local PocketIC pin, manual binary override, global CLI
or alternate installer. Failed checks never trigger setup.

Watchdog and cohort commands admit the server before building and recheck it
before execution, passing only the admitted path. Each product fixture retains
its own fresh server/instance and existing startup budget; this does not add
shared-server reuse or recovery guarantees.

## Dependency pin exceptions

The exact reviewed [exceptions](../ci/dependency-pinning-exceptions.json) are:

| Dependency | Selection | Qualification boundary |
| --- | --- | --- |
| `ic-cdk-timers` | `=1.0.0` | Provider dispatch, cancellation and recovery behavior |
| `ic0` | `=1.2.0` | Production platform bindings and instruction counter 1 |
| `ic-cdk` | `=0.20.3` | Probe callback execution and suspension semantics |

An exception matches its declaring root, name and literal version. A changed
selection requires renewed source/runtime/host review; the old receipt cannot
admit a new version. This is not permission for floating Actions or images.

## CI failure evidence

The validation runner retains raw and combined logs under
`.git/release-state/validation-failures/`, including `latest.log` and
`latest-combined.log` convenience copies. Later attempts retain earlier logs;
retention failure reports its owned scratch path. Preserve these before retrying.

The local failure action retains product/run/source identity and delegates tool
selection plus safe partial archive collection to Shared. Invalid store admission
still retains safe local evidence and reports incomplete collection; it never
traverses unadmitted external payloads. Testkit setup attempts and failed tool
builds keep their owner receipts. Hosted upload/download and native source
acceptance remain separate evidence; see [historical qualifications](evidence/release-history.md).
