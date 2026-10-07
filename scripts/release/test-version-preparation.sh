#!/usr/bin/env bash
set -euo pipefail

unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-version-test.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf -- "${temporary_root}"; else printf "Failed version-preparation fixture retained: %s\n" "${temporary_root}" >&2; fi' EXIT
git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{scripts/release,scripts/ci,.shared-tooling/helpers/scripts/ci,docs/status,docs/changelog,crates/ic-timers/src,testing/probe/src}
for script in bump-version finalize-changelog \
    warn-release-prose check-bump-impact check-lockfiles workspace-version readme-version update-local-lock; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
cp "${repository_root}/scripts/ci/next-release-version.sh" \
    "${repository_root}/scripts/ci/check-make-execution.sh" "${temporary_root}/scripts/ci/"
cp "${repository_root}/.shared-tooling/helpers/scripts/ci/read-cargo-workspace-version.sh" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/rewrite-local-lock-versions.pl" \
    "${temporary_root}/.shared-tooling/helpers/scripts/ci/"
# Classification is an isolated fixture input; preparation must not run tests.
cat > "${temporary_root}/scripts/release/classify-release-impact.sh" <<'EOF'
#!/usr/bin/env bash
if [[ "${FIXTURE_RELEASE_IMPACT:-crate}" == error ]]; then
    echo 'fixture classification failure' >&2
    exit 1
fi
echo "${FIXTURE_RELEASE_IMPACT:-crate}"
EOF
cat > "${temporary_root}/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/ic-timers", "testing/probe"]
resolver = "3"
[workspace.dependencies]
ic-timers = { path = "crates/ic-timers" }
[workspace.dependencies.fixture]
version = "0.1.0"
[workspace.package]
  version = "0.1.0" # Workspace truth; preserve spacing and this comment.
EOF
cat > "${temporary_root}/crates/ic-timers/Cargo.toml" <<'EOF'
[package]
name = "ic-timers"
version.workspace = true
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/crates/ic-timers/src/lib.rs"
cat > "${temporary_root}/testing/probe/Cargo.toml" <<'EOF'
[package]
name = "probe"
version.workspace = true
edition = "2024"
[dependencies]
ic-timers.workspace = true
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/testing/probe/src/lib.rs"
cat > "${temporary_root}/CHANGELOG.md" <<'EOF'
# Changelog

## [Draft]

- Fix terminal cleanup.

## [0.1.0]

- Initial release.
EOF
cat > "${temporary_root}/docs/status/current.md" <<'EOF'
# Current status

Read Cargo for package identity. This handoff has no release marker.
EOF
printf '%s\n' '# Fixture' '| API line | `0.1` |' 'ic-timers = "=0.1.0"' \
    > "${temporary_root}/README.md"
printf '%s\n' 'release-verify:' $'\t@touch unexpected-gate' $'\t@exit 1' \
    > "${temporary_root}/Makefile"
printf '%s\n' 'Unrelated work must survive preparation.' > "${temporary_root}/unrelated.txt"
cd "${temporary_root}"
cargo generate-lockfile --offline --quiet

# Preflight validates metadata without running deployment tests or changing it.
metadata_files=(Cargo.toml Cargo.lock CHANGELOG.md README.md \
    docs/status/current.md unrelated.txt)
chmod 0640 CHANGELOG.md

# Keep actual bytes and modes rather than separate checksum and mode manifests.
capture_metadata() {
    local snapshot="${1}"
    local path
    for path in "${metadata_files[@]}"; do
        mkdir -p "${snapshot}/$(dirname "${path}")"
        cp -p "${path}" "${snapshot}/${path}"
    done
}

assert_metadata_unchanged() {
    perl -MFile::Compare=compare -e '
        my $snapshot = shift @ARGV;
        for my $path (@ARGV) {
            my $original = "$snapshot/$path";
            compare($original, $path) == 0
                or die "error: fixture file contents changed: $path\n";
            my @before = stat $original;
            my @after = stat $path;
            @before && @after or die "error: cannot read fixture mode: $path\n";
            ($before[2] & 07777) == ($after[2] & 07777)
                or die "error: fixture file mode changed: $path\n";
        }
    ' "${1}" "${metadata_files[@]}"
}

capture_metadata original-files
# Reject malformed invocation before metadata reads, gates or mutation. A
# misplaced --check must never be silently ignored by a real bump.
for invocation in no-arguments missing-version extra-argument extra-version misplaced-check duplicate-check; do
    case "${invocation}" in
        no-arguments) set -- ;;
        missing-version) set -- --check ;;
        extra-argument) set -- --check patch extra ;;
        extra-version) set -- 0.1.1 extra ;;
        misplaced-check) set -- patch --check ;;
        duplicate-check) set -- --check --check patch ;;
    esac
    if output="$(bash scripts/release/bump-version.sh "$@" 2>&1)"; then
        echo "error: version preparation accepted ${invocation}" >&2
        exit 1
    else
        rejection_status=$?
    fi
    if [[ "${rejection_status}" != 2 || "${output}" != Usage:* ]]; then
        echo "error: invalid ${invocation} reached preparation: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged original-files
done
# Version ownership is table-scoped and rejects ambiguity before mutation.
cp Cargo.toml original-manifest.toml
for invalid in missing-table missing-version duplicate-table duplicate-version leading-zero; do
    cp original-manifest.toml Cargo.toml
    case "${invalid}" in
        missing-table) perl -pi -e 's/\[workspace\.package\]/[workspace.metadata]/' Cargo.toml ;;
        missing-version)
            perl -ni -e '$package ||= /^\[workspace\.package\]/;
                print unless $package && /^\s*version =/' Cargo.toml ;;
        duplicate-table) printf '%s\n' '[workspace.package]' 'version = "0.1.0"' >> Cargo.toml ;;
        duplicate-version) printf '%s\n' 'version = "0.1.0"' >> Cargo.toml ;;
        leading-zero)
            perl -pi -e '$package ||= /^\[workspace\.package\]/;
                s/"0\.1\.0"/"00.1.0"/ if $package' Cargo.toml ;;
    esac
    cp Cargo.toml rejected-manifest.toml
    for operation in read set; do
        set --
        if [[ "${operation}" == set ]]; then set -- set 0.1.0 0.1.1; fi
        if bash scripts/release/workspace-version.sh "$@" >/dev/null 2>&1; then
            echo "error: workspace version ${operation} accepted ${invalid}" >&2
            exit 1
        fi
        cmp rejected-manifest.toml Cargo.toml
    done
done
cp original-manifest.toml Cargo.toml
for arguments in '0.1.7 0.1.1' '0.1.0 00.1.1'; do
    read -r previous candidate <<< "${arguments}"
    if bash scripts/release/workspace-version.sh set "${previous}" "${candidate}" >/dev/null 2>&1; then
        echo "error: workspace version accepted ${arguments}" >&2
        exit 1
    fi
    cmp original-manifest.toml Cargo.toml
done
rm original-manifest.toml rejected-manifest.toml

# Tag-query errors must stop both preflight and the bump before any mutation.
mkdir -p bin
IC_TIMERS_FIXTURE_GIT="$(command -v git)"
export IC_TIMERS_FIXTURE_GIT
cat > bin/git <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == tag && "${2:-}" == --list ]]; then
    printf '%s' "${FIXTURE_TAG_LOOKUP_OUTPUT:-}"
    echo 'injected release-tag lookup failure' >&2
    exit 128
fi
exec "${IC_TIMERS_FIXTURE_GIT}" "$@"
EOF
chmod +x bin/git
for operation in preflight bump; do
    arguments=(patch)
    if [[ "${operation}" == preflight ]]; then arguments=(--check patch); fi
    for lookup_output in '' v0.1.1; do
        if output="$(PATH="${temporary_root}/bin:${PATH}" \
            FIXTURE_TAG_LOOKUP_OUTPUT="${lookup_output}" \
            bash scripts/release/bump-version.sh "${arguments[@]}" 2>&1)"; then
            echo "error: ${operation} accepted a failed release-tag lookup" >&2
            exit 1
        fi
        if [[ "${output}" != *'injected release-tag lookup failure'* ]]; then
            echo "error: unexpected tag-query rejection: ${output}" >&2
            exit 1
        fi
        assert_metadata_unchanged original-files
        staged_paths="$(git diff --cached --name-only)"
        test -z "${staged_paths}"
    done
done
rm bin/git

for kind in patch minor major; do
    case "${kind}" in
        patch) candidate=0.1.1 ;;
        minor) candidate=0.2.0 ;;
        major) candidate=1.0.0 ;;
    esac
    output="$(bash scripts/release/bump-version.sh --check "${kind}" 2>&1)"
    if [[ "${output}" != *"Release preflight passed: 0.1.0 -> ${candidate}"* ]]; then
        echo "error: ${kind} preflight selected an unexpected version: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged original-files
done

# Preparation must not replace a consumer-owned metadata symlink.
mv README.md owned-readme.md
ln -s owned-readme.md README.md
if bash scripts/release/bump-version.sh patch >/dev/null 2>&1; then
    echo 'error: version preparation accepted a symlinked metadata output' >&2
    exit 1
fi
test -L README.md
assert_metadata_unchanged original-files
rm README.md
mv owned-readme.md README.md

# Classification must reach the bump boundary, including failure and advisory.
for impact in none unexpected error; do
    if FIXTURE_RELEASE_IMPACT="${impact}" bash scripts/release/bump-version.sh --check patch >/dev/null 2>&1; then
        echo "error: preflight accepted ${impact} release impact" >&2
        exit 1
    fi
    assert_metadata_unchanged original-files
done
output="$(FIXTURE_RELEASE_IMPACT=repository bash scripts/release/bump-version.sh --check patch 2>&1)"
if [[ "${output}" != *'continuing because the maintainer invoked an explicit version bump'* ]]; then
    echo 'error: repository-only preflight omitted its advisory' >&2
    exit 1
fi

# Standard Make delegation and shared phase ordering are checked by the shared
# release-command checker and test-release-runner in release-check.
# test-commit-release exercises the actual local worktree admission.
cp Makefile preparation-only.mk
cp "${repository_root}/Makefile" Makefile
mkdir -p make
cp "${repository_root}/make/tools.mk" make/
mv scripts/release/bump-version.sh scripts/release/preparation-bump-version.sh
cat > scripts/release/bump-version.sh <<'EOF'
#!/usr/bin/env bash
printf 'preflight %s\n' "$*" >> release-events
bash scripts/release/preparation-bump-version.sh "$@"
EOF
cat > scripts/release/commit-release.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "$#:$*" == '1:--check-before-bump' ]]
printf '%s\n' worktree >> release-events
if [[ "${FIXTURE_FAIL_WORKTREE:-0}" == 1 ]]; then exit 1; fi
EOF
cat > overrides.mk <<'EOF'
release-verify:
	@printf '%s\n' gate >> release-events
	@if [ '$(FAIL_GATE)' = 1 ]; then exit 1; fi
patch minor major bump-x:
	@printf 'bump %s\n' '$(if $(filter bump-x,$@),$(VERSION),$@)' >> release-events
	@if [ '$(FAIL_PHASE)' = '$@' ]; then exit 9; fi
release-stage release-commit release-push:
	@printf '%s\n' '$@' >> release-events
	@if [ '$(FAIL_PHASE)' = '$@' ]; then exit 9; fi
ensure-clean release-tag-check:
	@:
EOF
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')
# The local exact-version recipe still owns this sequence. Exercise its real
# preflight with recording leaf phases, without commits, pushes or test execution.
cp CHANGELOG.md original-changelog.md
for target in release-x; do
    case "${target}" in
        release-x) requested=0.4.2; candidate=0.4.2 ;;
    esac
    cp original-changelog.md candidate-changelog.md
    for scenario in empty-notes failed-worktree failed-gate failed-bump failed-stage \
        failed-commit failed-push success repeat-success; do
        cp candidate-changelog.md CHANGELOG.md
        fail_phase=''
        case "${scenario}" in
            empty-notes)
                perl -pi -e 's/^- Fix terminal cleanup\.$//' CHANGELOG.md
                expected=("preflight --check ${requested}" worktree gate "bump ${requested}"
                    release-stage release-commit release-push) ;;
            failed-worktree) expected=("preflight --check ${requested}" worktree) ;;
            failed-gate) expected=("preflight --check ${requested}" worktree gate) ;;
            failed-bump)
                fail_phase=bump-x
                expected=("preflight --check ${requested}" worktree gate "bump ${requested}") ;;
            failed-stage)
                fail_phase=release-stage
                expected=("preflight --check ${requested}" worktree gate "bump ${requested}"
                    release-stage) ;;
            failed-commit)
                fail_phase=release-commit
                expected=("preflight --check ${requested}" worktree gate "bump ${requested}"
                    release-stage release-commit) ;;
            failed-push)
                fail_phase=release-push
                expected=("preflight --check ${requested}" worktree gate "bump ${requested}"
                    release-stage release-commit release-push) ;;
            *) expected=("preflight --check ${requested}" worktree gate "bump ${requested}"
                release-stage release-commit release-push) ;;
        esac
        fail_gate=0
        if [[ "${scenario}" == failed-gate ]]; then fail_gate=1; fi
        fail_worktree=0
        if [[ "${scenario}" == failed-worktree ]]; then fail_worktree=1; fi
        if FIXTURE_FAIL_WORKTREE="${fail_worktree}" "${fixture_make[@]}" "${target}" \
            "VERSION=${candidate}" "FAIL_GATE=${fail_gate}" "FAIL_PHASE=${fail_phase}" \
            > "${target}-${scenario}.log" 2>&1; then
            if [[ "${scenario}" == failed-* ]]; then
                cat "${target}-${scenario}.log" >&2
                echo "error: ${target} accepted ${scenario}" >&2
                exit 1
            fi
        elif [[ "${scenario}" != failed-* ]]; then
            cat "${target}-${scenario}.log" >&2
            echo "error: ${target} rejected valid preflight" >&2
            exit 1
        fi
        printf '%s\n' "${expected[@]}" > expected-release-events
        if ! cmp -s expected-release-events release-events; then
            cat release-events >&2
            echo "error: ${target} ${scenario} ran unexpected phases" >&2
            exit 1
        fi
        rm release-events
        cp original-changelog.md CHANGELOG.md
        assert_metadata_unchanged original-files
    done
done
if "${fixture_make[@]}" release-x VERSION= > release-x-empty-version.log 2>&1; then
    echo 'error: exact release accepted an empty target' >&2
    exit 1
fi
test ! -f release-events
# Actual Make dispatch must stop before every leaf phase and metadata mutation.
# Outer Make -i may itself return success after ignoring the guard's error;
# version-only Make never dispatches the recipe. Neither may produce effects.
# GNU Make 3.81 on macOS ignores GNUMAKEFLAGS; 4.0 introduced it.
mode_variables=(MAKEFLAGS)
make_version="$(make --version)"
case "$make_version" in 'GNU Make 3.'*) ;; *) mode_variables+=(GNUMAKEFLAGS) ;; esac
for variable in "${mode_variables[@]}"; do
    for flags in i n q t v --ignore-errors --dry-run --question --touch --version; do
        mode_status=0
        mode_output="${variable}-${flags}.log"
        env "$variable=$flags" "${fixture_make[@]}" release-x VERSION=0.4.2 \
            > "${mode_output}" 2>&1 || mode_status=$?
        case "$flags" in
            v|--version) ;;
            *) grep -Fq 'requires recipe execution and failure propagation' "${mode_output}" ;;
        esac
        case "$flags" in
            i|--ignore-errors|v|--version) ;;
            *) test "$mode_status" -ne 0 ;;
        esac
        test ! -f release-events
        assert_metadata_unchanged original-files
    done
done
rm original-changelog.md candidate-changelog.md overrides.mk
mv scripts/release/preparation-bump-version.sh scripts/release/bump-version.sh
mv preparation-only.mk Makefile

# Inject failures after the mutating lock update and metadata check.
mkdir -p bin
IC_TIMERS_FIXTURE_CARGO="$(command -v cargo)"
export IC_TIMERS_FIXTURE_CARGO
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    'metadata --manifest-path Cargo.toml --locked --offline --format-version 1') stage=root-metadata ;;
    *) stage=other ;;
esac
if [[ "${FIXTURE_FAIL_STAGE:-}" == "${stage}" ]]; then
    echo "injected ${stage} failure" >&2
    exit 1
fi
exec "${IC_TIMERS_FIXTURE_CARGO}" "$@"
EOF
chmod +x bin/cargo
cp scripts/release/update-local-lock.sh scripts/release/original-update-local-lock.sh
cat > scripts/release/update-local-lock.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in Cargo.lock) stage=root-update ;; *) exit 2 ;; esac
if [[ "${FIXTURE_FAIL_STAGE:-}" == "$stage" ]]; then
    echo "injected ${stage} failure" >&2
    exit 1
fi
if [[ "${FIXTURE_FAIL_STAGE:-}" == interrupt && "$stage" == root-update ]]; then
    kill -TERM "$PPID"; exit 1
fi
bash scripts/release/original-update-local-lock.sh "$@"
EOF
cp scripts/release/readme-version.sh scripts/release/original-readme-version.sh
cat > scripts/release/readme-version.sh <<'EOF'
#!/usr/bin/env bash
if [[ "${FIXTURE_FAIL_STAGE:-}" == readme-update && "${1:-}" == --update ]]; then
    echo 'injected README update failure' >&2
    exit 1
fi
bash scripts/release/original-readme-version.sh "$@"
EOF
for stage in readme-update root-update root-metadata interrupt; do
    if output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE="${stage}" \
        bash scripts/release/bump-version.sh patch 2>&1)"; then
        echo "error: version preparation accepted injected ${stage} failure" >&2
        exit 1
    fi
    if [[ "${output}" != *'restoring release metadata'* ]]; then
        echo "error: version preparation did not roll back ${stage}: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged original-files
done
mv scripts/release/original-readme-version.sh scripts/release/readme-version.sh

# An absent changelog is created during preparation and removed by rollback.
# Keep the lock-update injector installed until this final rollback scenario.
mv CHANGELOG.md existing-changelog.md
if output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE=root-update \
    bash scripts/release/bump-version.sh patch 2>&1)"; then
    echo 'error: version preparation accepted a failed update with no changelog' >&2
    exit 1
fi
if [[ "${output}" != *'injected root-update failure'* ||
    "${output}" != *'restoring release metadata'* ]]; then
    echo "error: missing-changelog rollback did not reach the injected update failure: ${output}" >&2
    exit 1
fi
test ! -e CHANGELOG.md
mv existing-changelog.md CHANGELOG.md
assert_metadata_unchanged original-files
mv scripts/release/original-update-local-lock.sh scripts/release/update-local-lock.sh

# A failed prose advisory still runs before mutation and cannot block the bump.
mv scripts/release/warn-release-prose.sh scripts/release/original-warn-release-prose.sh
cat > scripts/release/warn-release-prose.sh <<'EOF'
#!/usr/bin/env bash
bash scripts/release/workspace-version.sh > advisory-version
exit 1
EOF
output="$(bash scripts/release/bump-version.sh patch 2>&1)"
if [[ "${output}" != *'advisory release-prose check could not run; continuing'* ]]; then
    echo 'error: version preparation did not report its failed advisory' >&2
    exit 1
fi
grep -Fqx 0.1.0 advisory-version
mv scripts/release/original-warn-release-prose.sh scripts/release/warn-release-prose.sh
test ! -f unexpected-gate
grep -Fqx '  version = "0.1.1" # Workspace truth; preserve spacing and this comment.' Cargo.toml
test "$(bash scripts/release/workspace-version.sh)" = 0.1.1
grep -Fqx '## [0.1.0]' CHANGELOG.md
grep -Fqx 'ic-timers = "=0.1.1"' README.md
grep -Fqx '| API line | `0.1` |' README.md
# The earlier dependency version must survive the bump unchanged.
grep -Fqx 'version = "0.1.0"' Cargo.toml
bash scripts/release/check-lockfiles.sh
grep -Fqx 'Unrelated work must survive preparation.' unrelated.txt
fixture_tags="$(git tag --list)"
if git rev-parse --verify HEAD >/dev/null 2>&1 || [[ -n "${fixture_tags}" ]]; then
    echo 'error: version preparation committed or tagged fixture changes' >&2
    exit 1
fi
staged_paths="$(git diff --cached --name-only)"
if [[ -n "${staged_paths}" ]]; then
    echo 'error: version preparation staged unrelated fixture changes' >&2
    exit 1
fi

# Staging selects current release metadata and excludes unrelated adoption work.
cp "${repository_root}/Makefile" Makefile
printf '\n[features]\nmaintainer_fixture = []\n' >> crates/ic-timers/Cargo.toml
mkdir -p docs/adoption
printf '%s\n' '# Unrelated adoption edits' > docs/adoption/canic.md
cp Cargo.toml valid-stage-manifest.toml
perl -pi -e 's/\[workspace\.package\]/[workspace.metadata]/' Cargo.toml
for target in version release-stage; do
    if make --no-print-directory "${target}" >/dev/null 2>&1; then
        echo "error: ${target} accepted missing workspace version ownership" >&2
        exit 1
    fi
    staged_paths="$(git diff --cached --name-only)"
    test -z "${staged_paths}"
done
mv valid-stage-manifest.toml Cargo.toml
make --no-print-directory release-stage >/dev/null
expected_staged=(CHANGELOG.md Cargo.lock Cargo.toml README.md)
git diff --cached --name-only > staged-paths
printf '%s\n' "${expected_staged[@]}" > expected-staged-paths
if ! cmp -s expected-staged-paths staged-paths; then
    cat staged-paths >&2
    echo "error: release staging selected unexpected paths" >&2
    exit 1
fi
grep -Fqx '# Unrelated adoption edits' docs/adoption/canic.md
grep -Fqx 'maintainer_fixture = []' crates/ic-timers/Cargo.toml

# A same-named branch is harmless; only an existing exact release tag blocks.
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
git commit -qm fixture
git branch v0.1.2
perl -0pi -e 's/^(## \[)/## [Draft]\n\n- Next fixture release.\n\n$1/m' CHANGELOG.md
bash scripts/release/bump-version.sh --check patch >/dev/null
git tag v0.1.2
capture_metadata tagged-files
if output="$(bash scripts/release/bump-version.sh --check patch 2>&1)"; then
    echo 'error: version preflight accepted an existing release tag' >&2
    exit 1
fi
if [[ "${output}" != *'tag v0.1.2 already exists'* ]]; then
    echo "error: unexpected tag rejection: ${output}" >&2
    exit 1
fi
assert_metadata_unchanged tagged-files
echo 'Version preflight, rollback and dirty-worktree preparation checks passed'
