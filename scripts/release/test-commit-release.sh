#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir -p "${temporary_root}"/{scripts/{ci,release},bin}
for script in commit-release check-tag-at-head workspace-version readme-version; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
cp "${repository_root}/scripts/ci/ensure-clean.sh" "${temporary_root}/scripts/ci/"
# Cargo coherence is tested by test-lockfiles; this fixture isolates Git phases.
printf '%s\n' '#!/usr/bin/env bash' 'exit 0' > "${temporary_root}/scripts/release/check-lockfiles.sh"
cd "${temporary_root}"
git init -q
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
printf '%s\n' 'ensure-clean:' $'\t@bash scripts/ci/ensure-clean.sh' > Makefile
printf '%s\n' '[workspace.dependencies.fixture]' 'version = "0.2.0"' \
    '[workspace.package]' 'version = "0.1.0"' > Cargo.toml
printf '%s\n' '# Changelog' '' '## [0.1.0] - 2026-10-03' '' '- Fixture release.' > CHANGELOG.md
printf '%s\n' '# Fixture' '| API line | `0.1` |' 'ic-timers = "=0.1.0"' > README.md
git add .
git commit -qm fixture

expect_failure() {
    local expected="${1}" output
    shift
    if output="$("$@" 2>&1)"; then
        echo "error: release commit accepted ${expected}" >&2
        exit 1
    fi
    if [[ "${output}" != *"${expected}"* ]]; then
        echo "error: unexpected release rejection: ${output}" >&2
        exit 1
    fi
}

expect_failure 'HEAD is not Release' bash scripts/release/commit-release.sh
printf '%s\n' 'Prepared release.' >> README.md
git add README.md
# Fail tag lookup and creation after the real commit. Retries keep that commit.
IC_TIMERS_FIXTURE_GIT="$(command -v git)"
export IC_TIMERS_FIXTURE_GIT
cat > bin/git <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FIXTURE_FAIL_UNTRACKED:-0}" == 1 && "${1:-}" == ls-files ]]; then
    echo 'injected untracked-file query failure' >&2
    exit 1
fi
if [[ "${FIXTURE_FAIL_STAGED:-0}" == 1 && $# == 3 && "${1:-}" == diff \
    && "${2:-}" == --cached && "${3:-}" == --quiet ]]; then
    echo 'injected staged-diff query failure' >&2
    exit 128
fi
if [[ "${FIXTURE_FAIL_SUBJECT:-0}" == 1 && "${1:-}" == log ]]; then
    # Even plausible output must not conceal the failed producer.
    printf '%s\n' 'Release 0.1.0'
    echo 'injected release-subject query failure' >&2
    exit 1
fi
if [[ "${1:-}" == tag && "${2:-}" == --list ]]; then
    fail_lookup="${FIXTURE_FAIL_TAG_LOOKUP:-0}"
    if [[ "${fail_lookup}" == after-commit ]]; then
        subject="$("${IC_TIMERS_FIXTURE_GIT}" log -1 --format=%s)"
        if [[ "${subject}" == 'Release 0.1.0' ]]; then fail_lookup=1; fi
    fi
    if [[ "${fail_lookup}" == 1 ]]; then
        printf '%s' "${FIXTURE_TAG_LOOKUP_OUTPUT:-}"
        echo 'injected release-tag lookup failure' >&2
        exit 128
    fi
fi
if [[ "${FIXTURE_FAIL_TAG:-0}" == 1 && "${1:-}" == tag && "${2:-}" == -a ]]; then
    echo 'injected tag failure' >&2
    exit 1
fi
exec "${IC_TIMERS_FIXTURE_GIT}" "$@"
EOF
chmod +x bin/git
# The wrapper is an isolated fixture input, never an untracked release output.
git add bin/git
export PATH="${temporary_root}/bin:${PATH}"
initial_commit="$(git rev-parse HEAD)"
expect_failure 'injected untracked-file query failure' env FIXTURE_FAIL_UNTRACKED=1 bash scripts/release/commit-release.sh
expect_failure 'injected staged-diff query failure' env FIXTURE_FAIL_STAGED=1 bash scripts/release/commit-release.sh
for lookup_output in '' v0.1.0; do
    expect_failure 'injected release-tag lookup failure' env FIXTURE_FAIL_TAG_LOOKUP=1 \
        FIXTURE_TAG_LOOKUP_OUTPUT="${lookup_output}" bash scripts/release/commit-release.sh
done
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${initial_commit}"
expect_failure 'injected release-tag lookup failure' env FIXTURE_FAIL_TAG_LOOKUP=after-commit \
    bash scripts/release/commit-release.sh
release_commit="$(git rev-parse HEAD)"
release_subject="$(git log -1 --format=%s)"
test "${release_subject}" = 'Release 0.1.0'
release_tags="$(git tag --list v0.1.0)"
test -z "${release_tags}"
expect_failure 'injected tag failure' env FIXTURE_FAIL_TAG=1 bash scripts/release/commit-release.sh
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${release_commit}"
release_tags="$(git tag --list v0.1.0)"
test -z "${release_tags}"
bash scripts/release/commit-release.sh
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${release_commit}"
bash scripts/release/check-tag-at-head.sh
bash scripts/release/commit-release.sh
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${release_commit}"

# Query failures reject clean worktrees and interrupted-release retries too.
expect_failure 'injected untracked-file query failure' env FIXTURE_FAIL_UNTRACKED=1 bash scripts/ci/ensure-clean.sh
expect_failure 'injected untracked-file query failure' env FIXTURE_FAIL_UNTRACKED=1 bash scripts/release/commit-release.sh
expect_failure 'injected staged-diff query failure' env FIXTURE_FAIL_STAGED=1 bash scripts/release/commit-release.sh
expect_failure 'injected release-subject query failure' env FIXTURE_FAIL_SUBJECT=1 bash scripts/release/commit-release.sh
for lookup_output in '' v0.1.0; do
    expect_failure 'injected release-tag lookup failure' env FIXTURE_FAIL_TAG_LOOKUP=1 \
        FIXTURE_TAG_LOOKUP_OUTPUT="${lookup_output}" bash scripts/release/commit-release.sh
done
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${release_commit}"
bash scripts/release/check-tag-at-head.sh

git tag -d v0.1.0 >/dev/null
git tag v0.1.0
expect_failure 'must be annotated' bash scripts/release/commit-release.sh
git tag -d v0.1.0 >/dev/null
git tag -a v0.1.0 HEAD~1 -m fixture
expect_failure 'does not point to HEAD' bash scripts/release/commit-release.sh
git tag -d v0.1.0 >/dev/null
git tag -a v0.1.0 -m fixture
printf '%s\n' 'Unstaged change.' >> README.md
expect_failure 'stage all release changes' bash scripts/release/commit-release.sh
git add README.md
expect_failure 'cannot commit more changes' bash scripts/release/commit-release.sh
echo 'Release commit recovery checks passed'
