#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/release/check-tag-at-head.sh"
cleaner="${repository_root}/scripts/ci/ensure-clean.sh"
temporary_root="$(mktemp -d)"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$temporary_root" "Failed tag-at-head fixture retained"' EXIT
git init -q "${temporary_root}"
cd "${temporary_root}"
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
printf '%s\n' '[workspace.dependencies.fixture]' 'version = "0.2.0"' \
    '[workspace.package]' 'version = "0.1.0"' > Cargo.toml
git add Cargo.toml
git commit -qm fixture

expect_rejection() {
    local expected="${1}" output
    shift
    if output="$(bash "${checker}" "$@" 2>&1)"; then
        echo "error: tag guard accepted ${expected}: ${output}" >&2
        exit 1
    fi
}

git branch v0.1.0
expect_rejection 'missing tag despite a same-named branch'
git tag v0.1.0
expect_rejection 'lightweight tag'
git tag -d v0.1.0 >/dev/null
git tag -a v0.1.0 -m fixture
bash "${checker}"
release_commit="$(git rev-parse HEAD)"
git commit --allow-empty -qm 'advance head'
expect_rejection 'tag at older commit in HEAD mode'
bash "${checker}" "$release_commit" 0.1.0
expect_rejection 'wrong selected commit' "$(git rev-parse HEAD)" 0.1.0
expect_rejection 'symbolic commit identity' HEAD 0.1.0
expect_rejection 'abbreviated commit identity' "${release_commit:0:12}" 0.1.0
expect_rejection 'malformed version' "$release_commit" 0.1.01
expect_rejection 'incomplete invocation' "$release_commit"

# Each Git observation must fail closed even when it emits exactly the value
# that would otherwise authorize the selected release. Effects remain real only
# in this disposable fixture; the guard and substitutes never mutate refs.
mkdir bin
IC_TIMERS_TAG_REAL_GIT="$(command -v git)"
export IC_TIMERS_TAG_REAL_GIT
cat > bin/git <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FIXTURE_FAIL_TAG_QUERY:-}" == "$*" ]]; then
    printf '%s' "${FIXTURE_TAG_QUERY_OUTPUT:-}"
    echo 'injected tag query failure' >&2
    exit 37
fi
exec "$IC_TIMERS_TAG_REAL_GIT" "$@"
STUB
chmod +x bin/git
git for-each-ref --format='%(refname) %(objectname)' > .git/refs-before
for query in "rev-parse --verify ${release_commit}^{commit}" \
    'cat-file -t refs/tags/v0.1.0' 'rev-parse --verify refs/tags/v0.1.0^{commit}'; do
    expected_output="$release_commit"
    if [[ "$query" == cat-file* ]]; then expected_output=tag; fi
    for observation in '' "$expected_output"; do
        if output="$(PATH="$temporary_root/bin:$PATH" FIXTURE_FAIL_TAG_QUERY="$query" \
            FIXTURE_TAG_QUERY_OUTPUT="$observation" \
            bash "$checker" "$release_commit" 0.1.0 2>&1)"; then
            echo "error: tag guard admitted failed query: $query" >&2
            exit 1
        fi
        if [[ "$output" != *'injected tag query failure'* ]]; then
            echo "error: tag query fixture failed before its intended boundary: $output" >&2
            exit 1
        fi
    done
done
git for-each-ref --format='%(refname) %(objectname)' > .git/refs-after
cmp .git/refs-before .git/refs-after

# Publication still requires a clean worktree. Retain these admission checks
# here after removing the independent local commit/tag delivery owner.
git add bin/git
git commit -qm 'publication guard fixture'
bash "$cleaner"
git for-each-ref --format='%(refname) %(objectname)' > .git/clean-refs-before
cp -p .git/index .git/clean-index-before
for query in 'rev-parse --verify HEAD' 'rev-parse --show-prefix' \
    'status --porcelain=v1 -z --untracked-files=all'; do
    for observation in '' 'plausible but failed observation'; do
        if output="$(PATH="$temporary_root/bin:$PATH" FIXTURE_FAIL_TAG_QUERY="$query" \
            FIXTURE_TAG_QUERY_OUTPUT="$observation" bash "$cleaner" 2>&1)"; then
            echo "error: clean publication guard admitted failed query: $query" >&2
            exit 1
        fi
        if [[ "$query" != 'rev-parse --verify HEAD' &&
            "$output" != *'injected tag query failure'* ]]; then
            echo "error: clean guard fixture missed its intended boundary: $output" >&2
            exit 1
        fi
    done
done
cmp .git/index .git/clean-index-before
git for-each-ref --format='%(refname) %(objectname)' > .git/clean-refs-after
cmp .git/clean-refs-before .git/clean-refs-after
printf '\n# Unstaged publication change\n' >> Cargo.toml
if bash "$cleaner" >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted unstaged work' >&2
    exit 1
fi
git add Cargo.toml
if bash "$cleaner" >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted staged work' >&2
    exit 1
fi
git restore --staged --worktree Cargo.toml
printf 'untracked' > 'untracked publication change'
if bash "$cleaner" >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted untracked work' >&2
    exit 1
fi
rm 'untracked publication change'
bash "$cleaner"
# A source-only checker can admit an empty unborn checkout; publication cannot.
git init -q "$temporary_root/unborn"
if (cd "$temporary_root/unborn" && bash "$cleaner" >/dev/null 2>&1); then
    echo 'error: clean publication guard admitted a checkout without a commit' >&2
    exit 1
fi
# Qualify the actual tag workflow after the independent tag/publication cases.
workflow_root="$temporary_root/workflow"
mkdir "$workflow_root"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
# Only this disposable clone owns Git effects. All provisioning is substituted.
git clone -q --depth 1 --no-local --no-hardlinks "$repository_root" "${workflow_root}/repository"
mkdir "${workflow_root}/bin" "${workflow_root}/runner"
export GITHUB_WORKSPACE="${workflow_root}/repository" RUNNER_TEMP="${workflow_root}/runner"
export TAG_MAKE_EVENTS="${workflow_root}/make-events" TAG_SCRIPT_EVENTS="${workflow_root}/script-events" TAG_REAL_BASH="$BASH"
TAG_VERSION="$(cd "$GITHUB_WORKSPACE" && bash scripts/release/workspace-version.sh)"
export TAG_VERSION GITHUB_REF_NAME="v$TAG_VERSION" TAG_WORKFLOW_SCOPE=tag
git -C "$GITHUB_WORKSPACE" config user.name 'Tag workflow fixture'
git -C "$GITHUB_WORKSPACE" config user.email fixture@example.invalid
git -C "$GITHUB_WORKSPACE" tag -fa "$GITHUB_REF_NAME" -m fixture > /dev/null
git -C "$GITHUB_WORKSPACE" update-ref refs/remotes/origin/main HEAD
# Execute the actual workflow body, including its real tag, snapshot and graph guards.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -er '.jobs["tag-truth"].steps[] | select(.name == "Verify release tag") | .run' > "${workflow_root}/workflow.sh"
printf '#!%s\n' "$BASH" > "${workflow_root}/bin/make"
cat >> "${workflow_root}/bin/make" <<'STUB'
set -euo pipefail
printf '%s\n' "$*" >> "$TAG_MAKE_EVENTS"
case "$*" in
    install-tools|tools-check) ;;
    fetch) [[ "${TAG_FAIL_FETCH:-false}" != true ]] || exit 37 ;;
    ci) [[ "$TAG_WORKFLOW_SCOPE" == checks ]] || exit 97 ;;
    '--no-print-directory version')
        printf '%s\n' "$TAG_VERSION"
        [[ "${TAG_FAIL_VERSION:-false}" != true ]] || exit 37 ;;
    release-tag-check) exec "$TAG_REAL_BASH" "$GITHUB_WORKSPACE/scripts/release/check-tag-at-head.sh" ;;
    *) echo 'tag workflow dispatched a forbidden target' >&2; exit 97 ;;
esac
STUB
printf '#!%s\n' "$BASH" > "${workflow_root}/bin/bash"
cat >> "${workflow_root}/bin/bash" <<'STUB'
set -euo pipefail
printf '%s\n' "$*" >> "$TAG_SCRIPT_EVENTS"
exec "$TAG_REAL_BASH" "$@"
STUB
chmod +x "${workflow_root}/bin/make" "${workflow_root}/bin/bash"
run_workflow() {
    : > "$TAG_MAKE_EVENTS"
    : > "$TAG_SCRIPT_EVENTS"
    (cd "$GITHUB_WORKSPACE" && PATH="${workflow_root}/bin:$PATH" "$BASH" -e -o pipefail "${workflow_root}/workflow.sh") \
        > "${workflow_root}/workflow.log" 2>&1
}
run_workflow
printf 'install-tools\ntools-check\n--no-print-directory version\nrelease-tag-check\nfetch\n' > "${workflow_root}/expected-make-events"
cmp "${workflow_root}/expected-make-events" "$TAG_MAKE_EVENTS"
for expected in 'scripts/ci/verify-shared-tooling-snapshot.sh' \
    'scripts/ci/verify-shared-tooling-snapshot.sh --manifest .shared-tooling-audits.snapshot' \
    '.shared-tooling/helpers/scripts/ci/verify-shared-tooling-snapshot.sh --consumer .shared-tooling/helpers' \
    'scripts/release/check-lockfiles.sh'; do
    grep -Fxq "$expected" "$TAG_SCRIPT_EVENTS"
done
reject_workflow() {
    local status=0
    run_workflow || status=$?
    [[ "$status" != 0 ]] || { echo 'tag workflow accepted contradicted admission' >&2; exit 1; }
}
GITHUB_REF_NAME=v0.0.0 reject_workflow
TAG_FAIL_VERSION=true reject_workflow
# Actual failed snapshot admission must stop before graph validation.
for payload in DRAGGINZGAME.md audits/README.md .shared-tooling/helpers/scripts/ci/dependency-pins.jq; do
    cp -p "$GITHUB_WORKSPACE/$payload" "${workflow_root}/original"
    printf '\nchanged\n' >> "$GITHUB_WORKSPACE/$payload"
    reject_workflow
    if grep -Fxq 'scripts/release/check-lockfiles.sh' "$TAG_SCRIPT_EVENTS"; then exit 1; fi
    cp -p "${workflow_root}/original" "$GITHUB_WORKSPACE/$payload"
done
TAG_FAIL_FETCH=true reject_workflow
grep -Fxq fetch "$TAG_MAKE_EVENTS"
if grep -Fxq 'scripts/release/check-lockfiles.sh' "$TAG_SCRIPT_EVENTS"; then exit 1; fi
# An otherwise admitted event still needs its actual coordinated lock graph.
cp -p "$GITHUB_WORKSPACE/Cargo.lock" "${workflow_root}/original-lock"
printf 'version = 4\n' > "$GITHUB_WORKSPACE/Cargo.lock"
reject_workflow
grep -Fxq 'scripts/release/check-lockfiles.sh' "$TAG_SCRIPT_EVENTS"
cp -p "${workflow_root}/original-lock" "$GITHUB_WORKSPACE/Cargo.lock"
# Do not require parent history: CI checks out a shallow source. An independent
# commit with the same tree is a deterministic unrelated-main counterexample.
unrelated_main="$(printf 'unrelated main\n' | git -C "$GITHUB_WORKSPACE" commit-tree "HEAD^{tree}")"
git -C "$GITHUB_WORKSPACE" update-ref refs/remotes/origin/main "$unrelated_main"
reject_workflow
git -C "$GITHUB_WORKSPACE" update-ref refs/remotes/origin/main HEAD
git -C "$GITHUB_WORKSPACE" tag -d "$GITHUB_REF_NAME" > /dev/null
git -C "$GITHUB_WORKSPACE" tag "$GITHUB_REF_NAME"
reject_workflow
# The new fixture is in the full owner, never dispatched by a tag-only job.
awk '/^release-check:/ { selected=1; next } selected && /^[^[:space:]#]/ { exit } selected { print }' "$repository_root/Makefile" |
    grep -Fq 'bash scripts/release/test-tag-at-head.sh'
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -e '.jobs["tag-truth"].steps | any(.name == "Archive failure evidence") and any(.name == "Upload failure evidence")
        and all(.[]; ((.run // "") | test("make[^\\n]* release-check([ \\t\\n]|$)") | not))' > /dev/null
# The normal CI lane must prepare all root-lock sources before offline graph
# fixtures; its default-member build does not fetch unpublished probe dependencies.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -er '.jobs.checks.steps[] | select(.name == "Run CI gate") | .run' \
    > "${workflow_root}/normal-ci.sh"
for failed_fetch in false true; do
    : > "$TAG_MAKE_EVENTS"
    status=0
    (cd "$GITHUB_WORKSPACE" && TAG_WORKFLOW_SCOPE=checks TAG_FAIL_FETCH="$failed_fetch" \
        PATH="${workflow_root}/bin:$PATH" "$BASH" -e -o pipefail "${workflow_root}/normal-ci.sh") \
        > "${workflow_root}/normal-ci.log" 2>&1 || status=$?
    if [[ "$failed_fetch" == true ]]; then
        [[ "$status" == 37 ]] || exit 1
        printf 'fetch\n' > "${workflow_root}/expected-ci-events"
    else
        [[ "$status" == 0 ]] || exit 1
        printf 'fetch\nci\n' > "${workflow_root}/expected-ci-events"
    fi
    cmp "${workflow_root}/expected-ci-events" "$TAG_MAKE_EVENTS"
done
fixture_complete=true
echo 'Exact annotated release-tag and clean publication checks passed'
