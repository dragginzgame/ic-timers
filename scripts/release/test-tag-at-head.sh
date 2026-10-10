#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/release/check-tag-at-head.sh"
temporary_root="$(mktemp -d)"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "${temporary_root}"
    else
        printf "Failed tag-at-head fixture retained: %s\n" "${temporary_root}" >&2
    fi
    exit "$status"
}
trap finish EXIT
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
fixture_complete=true
echo 'Exact annotated release-tag checks passed'
