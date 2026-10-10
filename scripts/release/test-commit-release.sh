#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
temporary_root="$(mktemp -d)"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "${temporary_root}"
    else
        printf "Failed release-commit fixture retained: %s\n" "${temporary_root}" >&2
    fi
    exit "$status"
}
trap finish EXIT
mkdir -p "${temporary_root}"/{scripts/{ci,release},.shared-tooling/helpers/scripts/ci,bin}
for script in commit-release check-tag-at-head workspace-version; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
cp "${repository_root}/scripts/ci/ensure-clean.sh" "${temporary_root}/scripts/ci/"
cp "${repository_root}/.shared-tooling/helpers/scripts/ci/read-cargo-workspace-version.sh" \
    "${repository_root}/.shared-tooling/helpers/scripts/ci/check-release-tag.sh" \
    "${temporary_root}/.shared-tooling/helpers/scripts/ci/"
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
printf '%s\n' '# Fixture' 'ic-timers = "=0.0.7"' > README.md
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

# The early check admits only the metadata selected by release-stage. It does
# not need a release commit, coherent lockfiles, or a preparatory source commit.
initial_commit="$(git rev-parse HEAD)"
bash scripts/release/commit-release.sh --check-before-bump
expect_failure 'Usage:' bash scripts/release/commit-release.sh --check-before-bump extra
mkdir -p testing
for path in Cargo.toml Cargo.lock CHANGELOG.md; do
    tracked=false
    case "${path}" in Cargo.toml | CHANGELOG.md) tracked=true ;; esac
    printf '%s\n' '# Dirty metadata fixture.' >> "${path}"
    bash scripts/release/commit-release.sh --check-before-bump
    expect_failure 'stage all release changes' bash scripts/release/commit-release.sh
    grep -Fqx '# Dirty metadata fixture.' "${path}"
    if [[ "${tracked}" == true ]]; then git restore -- "${path}"; else rm -- "${path}"; fi
done

# README is ordinary source: stale examples are accepted, unstaged edits still
# need explicit staging and are never silently selected by release-stage.
printf '%s\n' 'Unstaged documentation.' >> README.md
expect_failure 'README.md' bash scripts/release/commit-release.sh --check-before-bump
git add README.md
bash scripts/release/commit-release.sh --check-before-bump
git restore --staged --worktree -- README.md

printf '%s\n' '# Unstaged implementation fixture.' >> scripts/ci/ensure-clean.sh
expect_failure 'scripts/ci/ensure-clean.sh' bash scripts/release/commit-release.sh --check-before-bump
grep -Fqx '# Unstaged implementation fixture.' scripts/ci/ensure-clean.sh
git add scripts/ci/ensure-clean.sh
bash scripts/release/commit-release.sh --check-before-bump
git show :scripts/ci/ensure-clean.sh > .git/expected-staged-source
printf '%s\n' '# Later unstaged implementation fixture.' >> scripts/ci/ensure-clean.sh
expect_failure 'scripts/ci/ensure-clean.sh' bash scripts/release/commit-release.sh --check-before-bump
git show :scripts/ci/ensure-clean.sh > .git/actual-staged-source
cmp .git/expected-staged-source .git/actual-staged-source
grep -Fqx '# Later unstaged implementation fixture.' scripts/ci/ensure-clean.sh
git restore --staged --worktree -- scripts/ci/ensure-clean.sh
rm -- scripts/ci/ensure-clean.sh
expect_failure 'scripts/ci/ensure-clean.sh' bash scripts/release/commit-release.sh --check-before-bump
git restore -- scripts/ci/ensure-clean.sh
for path in 'unstaged name.txt' $'unstaged\tname.txt' $'unstaged\nname.txt' 'unstaged"name.txt' $'caf\303\251.txt'; do
    printf '%s\n' 'Untracked implementation fixture.' > "${path}"
    escaped_path="$(printf '%q' "${path}")"
    expect_failure "${escaped_path}" bash scripts/release/commit-release.sh --check-before-bump
    grep -Fqx 'Untracked implementation fixture.' "${path}"
    git add -- "${path}"
    bash scripts/release/commit-release.sh --check-before-bump
    git restore --staged -- "${path}"
    rm -- "${path}"
done
current_commit="$(git rev-parse HEAD)"
test "${current_commit}" = "${initial_commit}"
release_tags="$(git tag --list v0.1.0)"
test -z "${release_tags}"
staged_paths="$(git diff --cached --name-only)"
test -z "${staged_paths}"
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
    if [[ "${FIXTURE_PARTIAL_OUTPUT:-0}" == 1 ]]; then printf '%s\0' Cargo.lock; fi
    echo 'injected untracked-file query failure' >&2
    exit 1
fi
if [[ "${FIXTURE_FAIL_UNSTAGED:-0}" == 1 && "${1:-}" == diff && "${2:-}" == --no-renames ]]; then
    if [[ "${FIXTURE_PARTIAL_OUTPUT:-0}" == 1 ]]; then printf '%s\0' Cargo.toml; fi
    echo 'injected unstaged-file query failure' >&2
    exit 128
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
for mode in commit before-bump; do
    set --
    if [[ "${mode}" == before-bump ]]; then set -- --check-before-bump; fi
    for partial_output in 0 1; do
        expect_failure 'injected untracked-file query failure' env FIXTURE_FAIL_UNTRACKED=1 \
            FIXTURE_PARTIAL_OUTPUT="${partial_output}" bash scripts/release/commit-release.sh "$@"
        expect_failure 'injected unstaged-file query failure' env FIXTURE_FAIL_UNSTAGED=1 \
            FIXTURE_PARTIAL_OUTPUT="${partial_output}" bash scripts/release/commit-release.sh "$@"
    done
done
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
expect_failure 'does not point to' bash scripts/release/commit-release.sh
git tag -d v0.1.0 >/dev/null
git tag -a v0.1.0 -m fixture
printf '%s\n' 'Unstaged change.' >> README.md
expect_failure 'stage all release changes' bash scripts/release/commit-release.sh
git add README.md
expect_failure 'cannot commit more changes' bash scripts/release/commit-release.sh
fixture_complete=true
echo 'Release commit recovery checks passed'
