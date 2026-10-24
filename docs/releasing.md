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
The classifier distinguishes crate, repository-only and empty subjects. Changes
under `crates/ic-timers/`, or to root `Cargo.toml`, `Cargo.lock` or `README.md`,
count as crate impact; this conservative package boundary includes license and
other packaged files. It is not an API compatibility oracle. Repository-only
work normally joins the next
code-bearing release. An explicit maintainer patch/minor command may publish it
with an advisory and the complete gate; an empty subject is rejected.

Maintain one numbered, undated pending section at the top of
[CHANGELOG.md](../CHANGELOG.md). The matching bump dates it automatically.
Missing files, versionless headings, misplaced pending sections and conflicting
version selections fail before mutation. Empty notes are advisory; exact
previous-release history can seed an empty candidate. README versions and layout do not gate delivery;
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

The consumer-owned [delivery declaration](../ci/release-delivery.txt) admits only
`direct`. Make and standalone metadata adapters share the read-only Shared
checker. Environment or command-line requests for `pr` refuse before effects;
they are not silently converted to direct delivery.

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
`install-release-editor`, `install-testkit-server` and `pocketic-check`
sequentially before validation or version preparation. Locked fetch prepares the selected graph; it cannot repair
a stale lockfile after dependency edits. No helper stages unrelated changes.

Shared 0.25 delegates candidate rendering to the published root-lock-selected
IC Build editor. Prepare it with `make install-release-editor`; standalone
metadata commands admit it offline through `make release-editor-check`. Its
positive per-document limit is maintained in `ci/release-editor.env`. Common CI
setup prepares the same executable. No local Perl rendering fallback remains.

Metadata-only `make patch`, `make minor`, `make major` and
`make bump-x VERSION=x.y.z` support preparation from a dirty worktree. They do
not run tests or deliver a release. Shared owns literal workspace version
rendering, all local lock records, changelog finalization, complete locked/offline
graph admission and rollback of those three files, including modes, on ordinary
errors and INT/TERM. SIGKILL cannot run rollback; retain its reported backup for
manual recovery. External dependency selections are preserved.

Protected commands reject Make flags that would skip execution, ignore errors
or pretend success. `publish` requires a clean checkout, the exact annotated
version tag at HEAD, the existing Clippy gate and a verified package before
publishing `ic-timers` to crates.io. These stages run sequentially even with
parallel Make; a failed admission, lint or package check stops publication.
Cleanup is separate; release commands never erase evidence.

## Deployment validation

The maintainer-operated `make release-verify` retains this fail-fast order:

1. `fetch` for the locked root workspace.
2. `install-release-editor` for the root-lock-selected metadata renderer.
3. `install-testkit-server` for explicit selected CLI/server setup.
4. `pocketic-check` for offline admission.
5. `ci`: Actions/pins, shell syntax, provider boundary, formatting, then the
   existing library Clippy gate before compilation, release fixtures, public
   docs, tests and Wasm/package checks.
6. `msrv`: native/test-target and Wasm library compilation plus doctests on 1.91.
7. `testing-check`: probe/protocol lint and each of the four exclusive cohorts.
8. `pocketic-watchdog`: the maintained real-canister runtime/recovery subjects.
9. `pocketic-cohorts`: baseline/Once/AfterCompletion/Watchdog Wasm and instruction
   measurements using the exact artifacts installed into PocketIC.

Both PocketIC targets run the native harness build driver on the selected
`MSRV` toolchain. Its recipes delegate Wasm execution and artifact discovery to
`ic-build::artifacts`, with isolated compiler state for each cohort, the root
lockfile and the `timer-probe` profile. The driver retains each exact artifact
record until the child tests finish; tests receive resolved Wasm paths rather
than reconstructing Cargo's output layout. Each generated probe cache requests
best-effort seven-day/512 MiB retention; active records remain protected, so
these are retention targets rather than hard disk bounds. Testkit's offline server admission
still precedes the driver. A build failure stops before any PocketIC test runs.

The driver supervises one build/test worker in a Host-owned process group with
an elapsed-time budget of 30 minutes, a two-second TERM grace and a five-second
reap budget. This covers build/cache waits, Cargo tests and synchronous PocketIC
instance deletion after the driver starts; the initial `cargo run` compilation
and Testkit admission remain outside it. Every separately owned Testkit server
also has a five-minute hard TTL; startup retains its 60-second timeout. These
are qualification resource limits, not timer recovery guarantees. Host preserves
inherited streaming output and typed timeout/nonzero-exit failures. OS spawn,
signal and reap syscalls can exceed elapsed-time budgets; group signalling does
not prove every descendant has exited. Native macOS timing remains unqualified.

The receipt reader, filesystem publication and injected build spawner use published Host and Build 0.68,
whose artifact engine accepts the same `OwnedChild` identity.
Update these together when exchanging Host values; metadata/tree observation's
command/byte boundary does not cover this build API. Testkit 0.68 selects the
same Host family. It and Platform/Wire/protocol are dev-dependencies of the driver;
only the child test binary needs the simulator and codecs. No aliases or local
child conversions are needed.
The child selects Testkit's `wire` feature for one-call typed transport; Timers
still selects method names/IDs, callers and directional limits. Deliberately
malformed inputs continue through raw PocketIC calls. Typed errors preserve the
original Wire or transport cause with call context rather than local string wrapping.

The root-lock-selected Build editor and Testkit server executables require the
`cli` feature. Their local setup/check adapters pass it to Shared's installer;
normal library dependencies remain CLI-free. After updating the package/feature
selection, run `make install-release-editor install-testkit-server` explicitly,
then `make tools-check`. Tool checks remain offline and never repair selections.

Before canister builds, the driver captures Cargo's terse test listing for the
selected binary/filter through Host's bounded command interface. Watchdog
selection must include every critical subject named by the driver's recovery
roster; one unrelated runtime test cannot qualify missing recovery coverage.
A second listing rejects ignored selected subjects without forcibly running them.
Cohorts must select its one fully qualified subject, run with `--exact`.
Failed discovery, invalid output and missing subjects fail even if Cargo exits
zero. Discovery executes no test bodies or simulator calls and shares the
worker's existing elapsed-time/process-group budget.

Each invocation allocates a fresh private `timer-qualification-*` directory under
the selected temporary root. Its path is printed and failures retain the directory;
successful runs clean it up. Successful discovery leaves `subjects.txt` there.
Each Testkit instance writes stdout/stderr to a unique subdirectory of this
invocation through the published caller-owned output interface. These logs survive
instance teardown and remain with failed diagnostics; existing CI temporary-root
collection includes them. They inherit the invocation retention lifecycle.
The child writes its candidate report there, never to the final output. The
parent reads at most 1 MiB, requires matching schema/context/artifact records and
all four cohorts plus both calibration subjects with their exact required metric
fields. Typed records reject missing or unknown fields in measurements, context
and artifact records; empty compiler/artifact identities also reject. Complete
schema-2 receipts require a per-cohort Build recipe identity; schema 1 rejects.
Regenerate reviewed baselines after adopting this dependency selection. Comparison uses the already
admitted report, so the compared value is also the one published.
A previous final report is never accepted in place of missing new evidence.

`pocketic-cohorts` atomically publishes `testing/target/cohorts.json` only after
subjects pass, post-measurement inputs are rechecked, candidate admission succeeds
and any requested baseline comparison passes. Earlier failures preserve an existing
final report and retain the unadmitted candidate as diagnostics. The report contains instruction brackets, dispatch cycles and Wasm bytes,
plus Build's compiler/Cargo identities, target/profile, per-cohort feature,
source/build fingerprints and artifact SHA-256. It also records the root lockfile
and admitted server SHA-256 and native host. Installed bytes are checked against
the receipt, and Build input observations, lock and server are rechecked after
measurement. Accept a receipt only with a successful driver exit; retain failed
outputs as diagnostics, not qualified baselines. No environment values or
credentials are serialized.

To compare a reviewed prior report, keep it outside the default output and run:

```sh
POCKET_IC_BIN="$(bash scripts/dev/testkit-server.sh check)" \
  RUSTUP_TOOLCHAIN=1.91.0 cargo run -p ic-timers-pocketic --locked -- \
  cohorts --baseline /absolute/path/to/reviewed-cohorts.json
```

Use `--report PATH` for a separate candidate output. Before building, the driver
uses Host's alias admission to reject the baseline's resolved name, symlinks and
device/inode aliases, including hard links. Host atomically replaces the report
directory entry with complete serialized bytes using owner-only file permissions;
it never truncates an existing inode. Interrupted staging can leave diagnostics,
and an error after publication can leave the complete new report visible. Keep
the selected parent hierarchy under caller control and exclude concurrent namespace
writers; these checks are not confinement. A successful driver exit remains
required for qualification. Comparisons require matching schema,
compiler/Cargo, target/profile, lockfile, server, host, measurement subjects and
Build recipe identities for each corresponding cohort.
Source and artifact identities may change: they attribute the candidate bytes
rather than prohibit code comparisons. Successful Linux and macOS qualification
jobs upload the final receipt with source/host/toolchain identity and 14-day
retention. Artifact upload alone does not qualify a failed or cancelled job.
Changed dependencies or tools require an explicitly reviewed new baseline; there is no automatic rewrite or cross-context
comparison. Deltas are review evidence, not an arbitrary performance pass/fail
budget. Whole-message and unaccounted instruction totals remain unavailable;
cycle costs are not converted to instructions. Build 0.68's published
`recipe_identity()` supplies the source-independent configuration admission
requested in [Build #241](https://github.com/dragginzgame/ic-tooling/issues/241).
It covers observed compiler/Cargo identities, package/example/target/profile,
workspace profiles, discovered configuration and relevant explicit build/compiler
environment settings. Features remain separately attributable by cohort. Source
bytes, lock identity, output directories and unrelated credentials are excluded;
existing source/cache fingerprints and mutation guards remain independent.
Configuration locations are compared conservatively. Wrapper executable bytes,
arbitrary build-script inputs and additional external inputs remain operator
obligations; the digest does not prove those equivalent.

Timers supplies explicit configuration limits at the Build boundary: 1 MiB per
file, 16 MiB total, 128 files and include depth 32. Build owns bounded reads,
include discovery and recipe hashing; no Cargo parser or environment dump is
implemented here. Exceeding a limit fails qualification before measurement.
Minimal and representative work calibrations remain separate from scheduler/work instruction brackets.


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
That lane prepares/checks the shared host bundle for the metadata parsers; it
does not provision common Cargo formatters, the release editor, Testkit CLI or
PocketIC. PR/main and ordinary developer setup retain the complete toolset.

## Dependency role boundary

`ic-timers` is this workspace's production library. The root catalog's role
comments describe local use and do not enforce dependency exclusions. Formatting
and metadata checks use matching grouped sorting, with release checks always
read-only. The local `ic-timers` catalog entry is used by the qualification
packages and is excluded when assessing dependencies of the selected root itself.

For the maintained Wasm library build, inspect the matching locked graph:

```bash
cargo tree --locked --offline -p ic-timers --all-features \
  --target wasm32-unknown-unknown --edges normal,build
```

`actions-check` runs the dependency policy for the library, both probes and the
protocol on Wasm and the actual native Rust host, with all features and
normal/build edges. It rejects Candid, full CDK, host tooling and unsupported Platform features in
runtime roots; the library excludes probe/Wire packages and the protocol excludes
runtime authority. A native coordinator check also rejects Platform, Testkit,
PocketIC, Candid, Wire and runtime/protocol packages on its normal/build edges;
their dev-only use remains admitted. It delegates graph mechanics to Shared,
and requires the qualified executor/IC0 identities. The separate Shared direct
checker enforces [named package ownership](../ci/cargo-dependency-boundaries.json)
across aliases and normal/build/dev/target declarations. Neither check validates
role comments or classifies every future package. See the
[enforcement scope](shared-tooling.md#deployment-dependency-exclusions).

There is no final production canister artifact deployed by this library
repository; its probe artifacts are qualification inputs. Deployment owners must
enforce their own exclusions and bind successful builds to source, lockfile,
target/features and final artifact digests. Library graph admission does not
certify deployed bytes or prohibit an application's legitimate Candid service.

## Host support

Required native hosts are Linux x86_64, Intel macOS and Apple Silicon macOS,
as selected by [CI](../.github/workflows/ci.yml). macOS checks its actual OS,
architecture/version and genuine Bash 3.2 before running the native gate.
Linux evidence does not qualify macOS. Do not weaken admission or product gates
to close a host gap.

The last recorded complete all-host acceptance here is released 0.23.1 at
`2d54724bc836ca09308d6742bf95c3f1ddecc3f4`; its exact run/jobs and graph are in
[the receipt](evidence/release-history.md#completed-0231-native-acceptance).
It does not qualify later source, newer dependency selections or the current
worktree. New hosted evidence must bind to its exact source and selections.

## Formatter prerequisites

`make fmt` / `make fmt-check` use the shared manifest/Rust formatting owner for
all five packages, with prepared tools and failure evidence. The repository-local
hook uses that same owner; focused fixtures retain actual-index preservation,
partial failure and native Bash behavior. Install tools explicitly through
[local setup](local-setup.md); checks do not provision them.

## Structured dependency checks and host parsers

`make actions-check` verifies the host bundle and delegates structured YAML/Cargo
pin and workspace-inheritance checks to the reviewed Shared helper. Every direct
dependency belongs to the root catalog and is inherited by members. Host/Cargo
setup and evidence use one shared store setting and checkout-local references;
no global tool or stale-generation fallback is admitted. See
[local setup](local-setup.md) and [adoption](shared-tooling.md) for exact owners.

## Common tools and Testkit setup

`make install-tools` prepares common host/Cargo tools, then the release editor
and Testkit. Standalone `release-verify` prepares the release editor after locked
fetch and before Testkit, matching fresh release preflight.
`make tools-check` verifies them offline in the same order. CI and Rust formatting
hooks publish common host/Cargo paths. Timers selects no IC executable bundle:
maintained probe builds use Cargo through IC Build and do not invoke Binaryen,
WABT or Quill. The former qualified IC matrix is retired. This does not qualify
an optimizer or weaken product PocketIC recovery/cohort gates.

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

The reviewed [exception list](../ci/dependency-pinning-exceptions.json) is empty.
Platform 0.6 owns the runtime and exact selections for executor `=2.0.0` and
IC0 `=1.2.0`; Timers declares none of these providers directly. The library
selects only `executor` and `ic0`, while canister probes select `cdk` and its
included `runtime` feature. The root
lockfile resolves one qualified identity for each provider. Any Platform update
changing those identities requires renewed callback/trap/cancellation/upgrade
evidence. See the [Candid/CDK boundary](architecture.md#candid-and-cdk-dependency-boundary).

An exception matches its declaring root, name and literal version. A changed
selection requires renewed source/runtime/host review; the old receipt cannot
admit a new version. This is not permission for floating Actions or images.

## CI failure evidence

The validation runner retains raw and combined logs under
`.git/release-state/validation-failures/`, including `latest.log` and
`latest-combined.log` convenience copies. Later attempts retain earlier logs;
retention failure reports its owned scratch path. Preserve these before retrying.

If `target` or `.tools` is redirected, the collector retains the parent link,
independent logs and a diagnostic without traversing its children. Collection
remains incomplete even if Shared's tool selector succeeds.

The local collector retains product/run/source identity and delegates tool
selection plus safe partial archive collection to Shared. CI captures its original
status and combined output, then passes the prepared archive and collection log to
Shared's evidence action. Available files are uploaded before missing inputs fail
completeness; original collection failure remains a failed job. The shared action
retains artifacts for fourteen days. Downloaded late qualification requires the
selected host catalog, a fresh check and compact host selection. Unselected legacy
IC sets remain full evidence; their presence is not IC-tool admission. Invalid store admission
still retains safe local evidence and reports incomplete collection; it never
traverses unadmitted external payloads. Testkit setup attempts and failed tool
builds keep their owner receipts. Hosted upload/download and native source
acceptance remain separate evidence; see [historical qualifications](evidence/release-history.md).
