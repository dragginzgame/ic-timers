#!/usr/bin/env bash
set -euo pipefail

unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-version-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed version-preparation fixture retained"' EXIT
git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{scripts/release,scripts/ci,.shared-tooling/helpers/scripts/ci,docs/status,docs/changelog,crates/ic-timers/src,testing/probe/src}
for script in bump-version finalize-changelog \
    warn-release-prose check-bump-impact check-lockfiles workspace-version update-local-lock; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
cp "${repository_root}/scripts/ci/next-release-version.sh" \
    "${repository_root}/scripts/ci/validate-release-inputs.pl" \
    "${repository_root}/scripts/ci/warn-release-prose.sh" \
    "${repository_root}/scripts/ci/check-make-execution.sh" \
    "${repository_root}/scripts/ci/run-formatting.sh" \
    "${repository_root}/scripts/ci/finalize-release-changelog.awk" "${temporary_root}/scripts/ci/"
cp "${repository_root}/.shared-tooling/helpers/scripts/ci/read-cargo-workspace-version.sh" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/render-cargo-workspace-version.pl" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/cargo-release-metadata.sh" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/cargo-workspace-members.jq" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/compare-stable-versions.pl" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/StableVersion.pm" \
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

## [0.1.1]

- Fix terminal cleanup.

## [0.1.0]

- Initial release.
EOF
cat > "${temporary_root}/docs/status/current.md" <<'EOF'
# Current status

Read Cargo for package identity. This handoff has no release marker.
EOF
# shellcheck disable=SC2016 # Markdown fixture keeps literal backticks.
printf '%s\n' '# Fixture' '| API line | `0.0` |' 'ic-timers = "=0.0.7"' \
    'ic-timers = "0.0" # Another legitimate documentation example.' \
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
# The actual lock updater must reject invalid outputs before Cargo, temporary
# output creation or writes, even when Bash 3.2 ignores a bare [[ ... ]] failure.
mkdir -p lock-admission-bin lock-admission-tmp rejected-lock-directory
printf '%s\n' 'Preserve the symlink target.' > rejected-lock-target
ln -s rejected-lock-target rejected-lock-alias
cat > lock-admission-bin/cargo <<'EOF'
#!/usr/bin/env bash
printf '%s\n' invoked >> "$LOCK_ADMISSION_EVENTS"
exit 91
EOF
chmod +x lock-admission-bin/cargo
for invalid_lock in missing-lock rejected-lock-directory rejected-lock-alias; do
    status=0
    LOCK_ADMISSION_EVENTS="$temporary_root/lock-admission-events" \
        PATH="$temporary_root/lock-admission-bin:$PATH" \
        TMPDIR="$temporary_root/lock-admission-tmp" \
        "$BASH" scripts/release/update-local-lock.sh "$invalid_lock" 0.1.0 0.1.1 \
        > "lock-admission-$invalid_lock.log" 2>&1 || status=$?
    [[ "$status" == 1 && ! -e lock-admission-events ]] || exit 1
    [[ -z "$(ls -A lock-admission-tmp)" ]] || exit 1
    [[ ! -e missing-lock && -d rejected-lock-directory && -L rejected-lock-alias ]] || exit 1
    printf '%s\n' 'Preserve the symlink target.' > expected-lock-target
    cmp expected-lock-target rejected-lock-target
    assert_metadata_unchanged original-files
done
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
    # Each requested increment has its own explicit numbered pending release.
    CANDIDATE="${candidate}" perl -pi -e 's/^## \[0\.1\.1\]$/## [$ENV{CANDIDATE}]/' CHANGELOG.md
    capture_metadata preflight-files
    output="$(bash scripts/release/bump-version.sh --check "${kind}" 2>&1)"
    if [[ "${output}" != *"Release preflight passed: 0.1.0 -> ${candidate}"* ]]; then
        echo "error: ${kind} preflight selected an unexpected version: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged preflight-files
    cp -p original-files/CHANGELOG.md CHANGELOG.md
done

# README shape, freshness and presence are not release preconditions.
mv README.md owned-readme.md
bash scripts/release/bump-version.sh --check patch >/dev/null
test ! -e README.md
ln -s owned-readme.md README.md
bash scripts/release/bump-version.sh --check patch >/dev/null
test -L README.md
rm README.md
mv owned-readme.md README.md
assert_metadata_unchanged original-files

# Preparation must not replace a consumer-owned metadata symlink.
mv Cargo.lock owned-lock
ln -s owned-lock Cargo.lock
if bash scripts/release/bump-version.sh patch >/dev/null 2>&1; then
    echo 'error: version preparation accepted a symlinked metadata output' >&2
    exit 1
fi
test -L Cargo.lock
assert_metadata_unchanged original-files
rm Cargo.lock
mv owned-lock Cargo.lock

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

# Standard delivery ordering and Make execution admission belong to the
# shared release runner and its adopted fixtures, not version preparation.

# Inject failures after the mutating lock update and metadata check.
mkdir -p bin
IC_TIMERS_FIXTURE_CARGO="$(command -v cargo)"
export IC_TIMERS_FIXTURE_CARGO
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    'metadata --locked --offline --format-version 1') stage=root-metadata ;;
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
for stage in root-update root-metadata interrupt; do
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

# A missing changelog is refused before mutation or the lock-update injector.
mv CHANGELOG.md existing-changelog.md
for operation in check prepare; do
    arguments=(patch)
    if [[ "${operation}" == check ]]; then arguments=(--check patch); fi
    if output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE=root-update \
        bash scripts/release/bump-version.sh "${arguments[@]}" 2>&1)"; then
        echo "error: ${operation} accepted a missing changelog" >&2
        exit 1
    fi
    if [[ "${output}" == *'injected root-update failure'* ||
        "${output}" == *'restoring release metadata'* ]]; then
        echo "error: missing-changelog refusal occurred after mutation: ${output}" >&2
        exit 1
    fi
    test ! -e CHANGELOG.md
    cmp original-files/Cargo.toml Cargo.toml
    cmp original-files/Cargo.lock Cargo.lock
done
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
cmp original-files/README.md README.md
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

# A same-named branch is harmless; only an existing exact release tag blocks.
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -qm fixture
git branch v0.1.2
perl -0pi -e 's/^(## \[)/## [0.1.2]\n\n- Next fixture release.\n\n$1/m' CHANGELOG.md
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
fixture_complete=true
echo 'Version preflight, rollback and dirty-worktree preparation checks passed'
