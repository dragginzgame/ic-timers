#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/ci/check-release-tag.sh"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-tag-at-head.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed tag-at-head fixture retained"' EXIT
git init -q "${temporary_root}"
cd "${temporary_root}"
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
printf '%s\n' '[workspace.dependencies.fixture]' 'version = "0.2.0"' \
    '[workspace.package]' 'version = "0.1.0"' > Cargo.toml
bash "$repository_root/scripts/ci/prepare-script-fixture.sh" "$repository_root" "$temporary_root" \
    Makefile make/consumer.mk scripts/ci/read-cargo-workspace-version.sh \
    scripts/ci/check-release-source.sh scripts/ci/check-release-tag.sh
git add Cargo.toml Makefile ci make scripts
git commit -qm fixture

expect_rejection() {
    local expected="${1}" output
    shift
    if [[ $# == 0 ]]; then set -- make --no-print-directory release-tag-check
    else set -- bash "$checker" "$@"; fi
    if output="$("$@" 2>&1)"; then
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
make --no-print-directory release-tag-check
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
# The implicit Make path must preserve failure even with plausible stdout.
for observation in '' "$release_commit"; do
    status=0
    PATH="$temporary_root/bin:$PATH" FIXTURE_FAIL_TAG_QUERY='rev-parse HEAD' \
        FIXTURE_TAG_QUERY_OUTPUT="$observation" \
        make --no-print-directory release-tag-check > "$temporary_root/implicit-output" 2>&1 || status=$?
    [[ "$status" != 0 ]] || exit 1
    grep -Fq 'injected tag query failure' "$temporary_root/implicit-output"
done
rm "$temporary_root/implicit-output"
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
make --no-print-directory ensure-clean
git for-each-ref --format='%(refname) %(objectname)' > .git/clean-refs-before
cp -p .git/index .git/clean-index-before
for query in 'rev-parse --verify HEAD' 'rev-parse --show-prefix' \
    'status --porcelain=v1 -z --untracked-files=all'; do
    for observation in '' 'plausible but failed observation'; do
        if output="$(PATH="$temporary_root/bin:$PATH" FIXTURE_FAIL_TAG_QUERY="$query" \
            FIXTURE_TAG_QUERY_OUTPUT="$observation" make --no-print-directory ensure-clean 2>&1)"; then
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
if make --no-print-directory ensure-clean >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted unstaged work' >&2
    exit 1
fi
git add Cargo.toml
if make --no-print-directory ensure-clean >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted staged work' >&2
    exit 1
fi
git restore --staged --worktree Cargo.toml
printf 'untracked' > 'untracked publication change'
if make --no-print-directory ensure-clean >/dev/null 2>&1; then
    echo 'error: clean publication guard admitted untracked work' >&2
    exit 1
fi
rm 'untracked publication change'
make --no-print-directory ensure-clean
# A source-only checker can admit an empty unborn checkout; publication cannot.
git init -q "$temporary_root/unborn"
bash "$repository_root/scripts/ci/prepare-script-fixture.sh" "$repository_root" "$temporary_root/unborn" \
    Makefile make/consumer.mk
if output="$(cd "$temporary_root/unborn" && make --no-print-directory ensure-clean 2>&1)"; then
    echo 'error: clean publication guard admitted a checkout without a commit' >&2
    exit 1
fi
[[ "$output" == *'repository has no commit yet'* ]] || exit 1
# Qualify the actual tag workflow after the independent tag/publication cases.
workflow_root="$temporary_root/workflow"
mkdir "$workflow_root"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
# Only this disposable clone owns Git effects. All provisioning is substituted.
git clone -q --depth 1 --no-local --no-hardlinks "$repository_root" "${workflow_root}/repository"
# Exercise the current caller/export layout even before these edits are committed.
# The disposable shallow clone supplies real history; only its worktree is overlaid.
while IFS=$'\t' read -r record _digest _mode path; do
    [[ "$record" == file ]] || continue
    mkdir -p "${workflow_root}/repository/$(dirname "$path")"
    cp -p "$repository_root/$path" "${workflow_root}/repository/$path"
done < "$repository_root/.shared-tooling.snapshot"
cp "$repository_root/.shared-tooling.snapshot" "${workflow_root}/repository/"
cp "$repository_root/Makefile" "$repository_root/Cargo.toml" "$repository_root/Cargo.lock" "${workflow_root}/repository/"
# The current lock graph includes every member's dependency declarations. Keep
# that one graph coherent rather than combining it with committed manifests.
members="$(yq -p toml -r '.workspace.members[]' "$repository_root/Cargo.toml")"
while IFS= read -r member; do
    mkdir -p "${workflow_root}/repository/$member"
    cp "$repository_root/$member/Cargo.toml" "${workflow_root}/repository/$member/Cargo.toml"
done <<< "$members"
mkdir "${workflow_root}/bin" "${workflow_root}/runner"
export GITHUB_WORKSPACE="${workflow_root}/repository" RUNNER_TEMP="${workflow_root}/runner"
TAG_REAL_MAKE="$(command -v make)"
export TAG_REAL_MAKE
export TAG_MAKE_EVENTS="${workflow_root}/make-events" TAG_SCRIPT_EVENTS="${workflow_root}/script-events" TAG_REAL_BASH="$BASH"
TAG_VERSION="$(cd "$GITHUB_WORKSPACE" && bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)"
export TAG_VERSION GITHUB_REF_NAME="v$TAG_VERSION" TAG_WORKFLOW_SCOPE=tag
# The actual tag graph check must select the current worktree despite ambient intent.
export RELEASE_COMMIT=unrelated-saved-release
git -C "$GITHUB_WORKSPACE" config user.name 'Tag workflow fixture'
git -C "$GITHUB_WORKSPACE" config user.email fixture@example.invalid
git -C "$GITHUB_WORKSPACE" tag -fa "$GITHUB_REF_NAME" -m fixture > /dev/null
git -C "$GITHUB_WORKSPACE" update-ref refs/remotes/origin/main HEAD
# Execute the actual workflow body, including its real tag, snapshot and graph guards.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -e '.jobs["tag-truth"].steps' > "${workflow_root}/tag-steps.json"
bash "$repository_root/scripts/ci/extract-workflow-step.sh" "${workflow_root}/tag-steps.json" release-tag > "${workflow_root}/workflow.sh"
printf '#!%s\n' "$BASH" > "${workflow_root}/bin/make"
cat >> "${workflow_root}/bin/make" <<'STUB'
set -euo pipefail
printf '%s\n' "$*" >> "$TAG_MAKE_EVENTS"
case "$*" in
    install-host-tools|host-tools-check)
        [[ "${TAG_FAIL_HOST_TARGET:-}" != "$*" ]] || exit 37 ;;
    fetch) [[ "${TAG_FAIL_FETCH:-false}" != true ]] || exit 37 ;;
    ci) [[ "$TAG_WORKFLOW_SCOPE" == checks ]] || exit 97 ;;
    '--no-print-directory version')
        printf '%s\n' "$TAG_VERSION"
        [[ "${TAG_FAIL_VERSION:-false}" != true ]] || exit 37 ;;
    release-tag-check) exec "$TAG_REAL_MAKE" --no-print-directory release-tag-check ;;
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
printf 'install-host-tools\nhost-tools-check\n--no-print-directory version\nrelease-tag-check\nfetch\n' > "${workflow_root}/expected-make-events"
cmp "${workflow_root}/expected-make-events" "$TAG_MAKE_EVENTS"
for expected in 'scripts/ci/verify-shared-tooling-snapshot.sh' \
    'scripts/ci/cargo-release-metadata.sh check-graph ic-timers'; do
    grep -Fxq "$expected" "$TAG_SCRIPT_EVENTS"
done
reject_workflow() {
    local status=0
    run_workflow || status=$?
    [[ "$status" != 0 ]] || { echo 'tag workflow accepted contradicted admission' >&2; exit 1; }
}
GITHUB_REF_NAME=v0.0.0 reject_workflow
TAG_FAIL_VERSION=true reject_workflow
for target in install-host-tools host-tools-check; do
    TAG_FAIL_HOST_TARGET="$target" reject_workflow
    if grep -Fxq -- '--no-print-directory version' "$TAG_MAKE_EVENTS"; then exit 1; fi
    if [[ "$target" == install-host-tools ]] && grep -Fxq host-tools-check "$TAG_MAKE_EVENTS"; then exit 1; fi
done
# Actual failed snapshot admission must stop before graph validation.
for payload in DRAGGINZGAME.md audits/README.md scripts/ci/dependency-pins.jq; do
    cp -p "$GITHUB_WORKSPACE/$payload" "${workflow_root}/original"
    printf '\nchanged\n' >> "$GITHUB_WORKSPACE/$payload"
    reject_workflow
    if grep -Fxq 'scripts/ci/cargo-release-metadata.sh check-graph ic-timers' "$TAG_SCRIPT_EVENTS"; then exit 1; fi
    cp -p "${workflow_root}/original" "$GITHUB_WORKSPACE/$payload"
done
TAG_FAIL_FETCH=true reject_workflow
grep -Fxq fetch "$TAG_MAKE_EVENTS"
if grep -Fxq 'scripts/ci/cargo-release-metadata.sh check-graph ic-timers' "$TAG_SCRIPT_EVENTS"; then exit 1; fi
# An otherwise admitted event still needs its actual coordinated lock graph.
cp -p "$GITHUB_WORKSPACE/Cargo.lock" "${workflow_root}/original-lock"
printf 'version = 4\n' > "$GITHUB_WORKSPACE/Cargo.lock"
reject_workflow
grep -Fxq 'scripts/ci/cargo-release-metadata.sh check-graph ic-timers' "$TAG_SCRIPT_EVENTS"
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
    jq -e '.jobs["tag-truth"].steps | any(.uses == "./.github/actions/retain-failure-evidence" and .if == "${{ failure() }}")
        and all(.[]; ((.run // "") | test("make[^\\n]* release-check([ \\t\\n]|$)") | not))' > /dev/null
# The normal CI lane must prepare all root-lock sources before offline graph
# fixtures; its default-member build does not fetch unpublished probe dependencies.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -e '.jobs.checks.steps' > "${workflow_root}/ci-steps.json"
bash "$repository_root/scripts/ci/extract-workflow-step.sh" "${workflow_root}/ci-steps.json" ci-gate > "${workflow_root}/normal-ci.sh"
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
