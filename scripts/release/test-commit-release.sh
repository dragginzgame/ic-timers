#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir -p "${temporary_root}"/{scripts/{ci,release},docs/{status,changelog},bin}
for script in commit-release check-release-truth check-tag-at-head workspace-version readme-version; do
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
printf '%s\n' '# Changelog' '' '## [Unreleased]' '' '## [0.1.0] - 2026-10-03' '' '- Fixture release.' > CHANGELOG.md
printf '%s\n' '# 0.1.0' '' 'Status: released 0.1.0.' > docs/changelog/0.1.0.md
printf '%s\n' '- Workspace package version: `0.1.0`.' > docs/status/current.md
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
# Fail tagging after the real commit. Retrying must finish this same commit.
IC_TIMERS_FIXTURE_GIT="$(command -v git)"
export IC_TIMERS_FIXTURE_GIT
cat > bin/git <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FIXTURE_FAIL_TAG:-0}" == 1 && "${1:-}" == tag ]]; then
    echo 'injected tag failure' >&2
    exit 1
fi
exec "${IC_TIMERS_FIXTURE_GIT}" "$@"
EOF
chmod +x bin/git
# The wrapper is an isolated fixture input, never an untracked release output.
git add bin/git
export PATH="${temporary_root}/bin:${PATH}"
expect_failure 'injected tag failure' env FIXTURE_FAIL_TAG=1 bash scripts/release/commit-release.sh
release_commit="$(git rev-parse HEAD)"
test "$(git log -1 --format=%s)" = 'Release 0.1.0'
bash scripts/release/commit-release.sh
test "$(git rev-parse HEAD)" = "${release_commit}"
bash scripts/release/check-tag-at-head.sh
bash scripts/release/commit-release.sh
test "$(git rev-parse HEAD)" = "${release_commit}"

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
