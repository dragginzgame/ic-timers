#!/usr/bin/env bash
set -euo pipefail

# Reuse the current commit without creating fixture commits. Deliberately use
# a shallow checkout like CI; only the isolated clone receives a baseline tag.
# The adapter reads a real Git index; Cargo calls are stubs, not compilation.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export PATH="$root/.tools/host/bin:$PATH"
export YQ="$root/.tools/host/bin/yq"
export RELEASE_INDEX_REAL_CARGO="$(command -v cargo)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/release-index-check.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "${fixture}"
    else
        if [[ -f "$fixture/output" ]]; then cat "$fixture/output" >&2 || :; fi
        printf "Failed release-index fixture retained: %s\n" "${fixture}" >&2
    fi
    exit "$status"
}
trap finish EXIT
git clone -q --no-local --depth 1 "$root" "$fixture/repo"
mkdir "$fixture/bin" "$fixture/tmp"
export FETCH_EVENTS="$fixture/fetch-events"
export PREPARATION_EVENTS="$fixture/preparation-events"
: > "$PREPARATION_EVENTS"
export RELEASE_INDEX_REAL_MAKE="$(command -v make)"
real_bash="$(command -v bash)"
printf '#!%s\n' "$real_bash" > "$fixture/bin/cargo"
cat >> "$fixture/bin/cargo" <<'STUB'
set -euo pipefail
case "$1" in
    locate-project) exec "$RELEASE_INDEX_REAL_CARGO" "$@" ;;
    sort) ;;
    metadata) printf '{"packages":[{"name":"ic-timers","version":"%s"}]}\n' "$RELEASE_VERSION" ;;
    fetch)
        printf '%s\n' "$*" >> "$FETCH_EVENTS"
        printf 'fetch\n' >> "$PREPARATION_EVENTS"
        printf '%s\n' "${CARGO_NET_OFFLINE:-unset}" > "$FETCH_EVENTS.offline"
        [[ "$*" == 'fetch --manifest-path Cargo.toml --locked' ]] || exit 1
        if [[ "${FAIL_FETCH_MANIFEST:-}" == "$3" ]]; then
            echo 'fixture locked fetch failure' >&2
            exit 37
        fi
        ;;
    *) echo "unexpected Cargo command: $*" >&2; exit 1 ;;
esac
STUB
chmod +x "$fixture/bin/cargo"
printf '#!%s\n' "$real_bash" > "$fixture/bin/make"
cat >> "$fixture/bin/make" <<'STUB'
set -euo pipefail
case "$*" in
    '--no-print-directory install-testkit-server') phase=setup ;;
    '--no-print-directory pocketic-check') phase=check ;;
    *) exec "$RELEASE_INDEX_REAL_MAKE" "$@" ;;
esac
printf '%s\n' "$phase" >> "$PREPARATION_EVENTS"
printf '%s\n' "${CARGO_NET_OFFLINE:-unset}" > "$PREPARATION_EVENTS.offline"
if [[ "${FAIL_TOOL_PHASE:-}" == "$phase" ]]; then
    echo "fixture selected tool $phase failure" >&2
    exit 38
fi
STUB
chmod +x "$fixture/bin/make"
export PATH="$fixture/bin:$PATH" TMPDIR="$fixture/tmp"
cd "$fixture/repo"
original_head="$(git rev-parse HEAD)"
export RELEASE_VERSION="$(bash "$root/scripts/release/workspace-version.sh")"
export RELEASE_PREVIOUS="$RELEASE_VERSION"
export RELEASE_DATE="$(sed -n "s/^## \[$RELEASE_VERSION\] - //p" CHANGELOG.md)"
# CI's main checkout need not contain release tags. Supply the impact owner's
# prerequisite locally, without fetching history or changing the source repo.
git tag -f "v$RELEASE_PREVIOUS" "$original_head" > /dev/null
[[ "$(git rev-parse --is-shallow-repository)" == true ]] || exit 1

reject() {
    if bash "$root/scripts/release/adapter.sh" "$1" > "$fixture/output" 2>&1; then
        echo "error: release adapter accepted $2" >&2
        exit 1
    fi
}

bash "$root/scripts/release/adapter.sh" commit-check
source_path=crates/ic-timers/src/lib.rs
printf '\n// staged implementation fixture\n' >> "$source_path"
git add "$source_path"
git show "HEAD:$source_path" > "$source_path"
# HEAD-to-worktree is clean, while the index still contains an implementation edit.
git diff --quiet HEAD -- "$source_path"
staged_tree="$(git write-tree)"
working_path=crates/ic-timers/src/platform.rs
printf '\n// working implementation fixture\n' >> "$working_path"
untracked_path=$'unexpected\nsource.rs'
printf '%s\n' 'untracked implementation' > "$untracked_path"
cp .git/index "$fixture/admission-index"
cp "$source_path" "$fixture/admission-source"
cp "$working_path" "$fixture/admission-working"
reject preflight 'hidden staged implementation'
for expected in "staged: $source_path" "unstaged: $source_path" "unstaged: $working_path"; do
    grep -Fq "$expected" "$fixture/output"
done
printf '  untracked: %q\n' "$untracked_path" > "$fixture/expected-untracked"
grep -Fx -f "$fixture/expected-untracked" "$fixture/output" > /dev/null
[[ ! -s "$FETCH_EVENTS" ]] || exit 1
[[ ! -s "$PREPARATION_EVENTS" ]] || exit 1
cmp .git/index "$fixture/admission-index"
cmp "$source_path" "$fixture/admission-source"
cmp "$working_path" "$fixture/admission-working"
reject commit-check 'implementation in release index'
cmp .git/index "$fixture/admission-index"
[[ "$(git write-tree)" == "$staged_tree" ]] || exit 1
git reset -q HEAD -- "$source_path"
git show "HEAD:$working_path" > "$working_path"
rm "$untracked_path"

printf '\nPrepared metadata fixture.\n' >> CHANGELOG.md
git add CHANGELOG.md
staged_tree="$(git write-tree)"
git show HEAD:CHANGELOG.md > CHANGELOG.md
reject commit-check 'index differing from prepared working metadata'
[[ "$(git write-tree)" == "$staged_tree" ]] || exit 1
git reset -q HEAD -- CHANGELOG.md

printf '\nPrepared metadata fixture.\n' >> CHANGELOG.md
reject commit-check 'unstaged metadata'
git add CHANGELOG.md
bash "$root/scripts/release/adapter.sh" commit-check
git reset -q HEAD -- CHANGELOG.md
git show HEAD:CHANGELOG.md > CHANGELOG.md

git rm -q --cached CHANGELOG.md
reject commit-check 'untracked release metadata'
git reset -q HEAD -- CHANGELOG.md
# Ordinary documentation edits remain subject to source admission, independently
# of version freshness. Neither unstaged nor staged README edits are metadata.
printf '\nDocumentation fixture.\n' >> README.md
reject preflight 'unstaged ordinary documentation'
git add README.md
reject commit-check 'ordinary documentation in release index'
git reset -q HEAD -- README.md
git show HEAD:README.md > README.md
printf '%s\n' 'untracked implementation' > unexpected-source.rs
reject commit-check 'untracked implementation'
rm unexpected-source.rs

# Real Git guards must reject failed producers, even when they emit plausible
# metadata paths first. No failure may become successful empty-path admission.
export RELEASE_INDEX_REAL_GIT="$(command -v git)"
printf '#!%s\n' "$real_bash" > "$fixture/bin/git"
cat >> "$fixture/bin/git" <<'STUB'
set -euo pipefail
query=''
case "${1:-}:${2:-}" in
    rev-parse:--show-prefix) query=checkout ;;
    status:--porcelain=v1) query=status ;;
    diff:--quiet) query=index ;;
esac
if [[ -n "$query" && "${FAIL_ADMISSION_QUERY:-}" == "$query" ]]; then
    if [[ "${PARTIAL_ADMISSION_OUTPUT:-0}" == 1 ]]; then
        if [[ "$query" == status ]]; then printf ' M CHANGELOG.md\0';
        else printf '%s\n' 'partial-checkout/'; fi
    fi
    echo "fixture failed $query query" >&2
    exit 128
fi
exec "$RELEASE_INDEX_REAL_GIT" "$@"
STUB
chmod +x "$fixture/bin/git"
for operation in preflight commit-check; do
    for query in checkout status; do
        for partial in 0 1; do
            export FAIL_ADMISSION_QUERY="$query" PARTIAL_ADMISSION_OUTPUT="$partial"
            reject "$operation" "failed $query query with partial=$partial"
            grep -Fq "fixture failed $query query" "$fixture/output"
            grep -Fq 'cannot inspect' "$fixture/output"
            if grep -Fq 'release source refused' "$fixture/output"; then
                echo 'error: failed Git observation was reported as dirty source' >&2
                exit 1
            fi
            [[ -z "$(ls -A "$fixture/tmp")" ]] || exit 1
        done
    done
done
export FAIL_ADMISSION_QUERY=index PARTIAL_ADMISSION_OUTPUT=0
reject commit-check 'failed index/worktree comparison'
grep -Fq 'fixture failed index query' "$fixture/output"
unset FAIL_ADMISSION_QUERY PARTIAL_ADMISSION_OUTPUT

# A failed version reader may emit the expected version. Never compare that
# output as successful metadata or advance to source admission/fetch/setup.
export RELEASE_INDEX_REAL_BASH="$real_bash"
printf '#!%s\n' "$real_bash" > "$fixture/bin/bash"
cat >> "$fixture/bin/bash" <<'STUB'
set -euo pipefail
if [[ "${1:-}" == scripts/release/workspace-version.sh && -n "${FAIL_VERSION_READ:-}" ]]; then
    case "$FAIL_VERSION_READ" in
        matching) printf '%s\n' "$RELEASE_PREVIOUS" ;;
        empty) ;;
        mismatch) printf '%s\n' '0.0.0'; exit 0 ;;
    esac
    echo 'fixture failed workspace version read' >&2
    exit 23
fi
exec "$RELEASE_INDEX_REAL_BASH" "$@"
STUB
chmod +x "$fixture/bin/bash"
cp .git/index "$fixture/version-read-index"
for path in Cargo.toml Cargo.lock CHANGELOG.md README.md; do
    cp "$path" "$fixture/version-read-${path}"
done
for output in empty matching mismatch; do
    : > "$FETCH_EVENTS"
    : > "$PREPARATION_EVENTS"
    status=0
    FAIL_VERSION_READ="$output" "$real_bash" "$root/scripts/release/adapter.sh" preflight \
        > "$fixture/output" 2>&1 || status=$?
    expected=23; [[ "$output" != mismatch ]] || expected=1
    [[ "$status" == "$expected" ]] || exit 1
    if [[ "$output" != mismatch ]]; then
        grep -Fq 'fixture failed workspace version read' "$fixture/output"
    fi
    if grep -Fq 'release source refused' "$fixture/output"; then exit 1; fi
    [[ ! -s "$FETCH_EVENTS" && ! -s "$PREPARATION_EVENTS" && ! -e .git/release-state ]] || exit 1
    cmp .git/index "$fixture/version-read-index"
    for path in Cargo.toml Cargo.lock CHANGELOG.md README.md; do
        cmp "$path" "$fixture/version-read-${path}"
    done
done

# A clean release preflight fetches, sets up and admits the selected tool, in
# order. Every failure stops before later effects or release metadata mutation.
cp CHANGELOG.md "$fixture/original-changelog"
export RELEASE_VERSION="$(bash "$root/scripts/ci/next-release-version.sh" "$RELEASE_PREVIOUS" patch)"
# HEAD may already contain an undated draft when CI checks a preparation commit.
# Give this isolated scenario one candidate rather than adding a second draft.
printf '# Changelog\n\n## [%s]\n\n- Cache preparation fixture.\n\n## [%s] - %s\n\n- Prior fixture.\n' \
    "$RELEASE_VERSION" "$RELEASE_PREVIOUS" "$RELEASE_DATE" > CHANGELOG.md
cp CHANGELOG.md "$fixture/pending-changelog"
for failed_phase in '' fetch setup check; do
    : > "$FETCH_EVENTS"
    : > "$PREPARATION_EVENTS"
    export FAIL_FETCH_MANIFEST='' FAIL_TOOL_PHASE=''
    if [[ "$failed_phase" == fetch ]]; then export FAIL_FETCH_MANIFEST=Cargo.toml;
    else export FAIL_TOOL_PHASE="$failed_phase"; fi
    if [[ -z "$failed_phase" ]]; then
        bash "$root/scripts/release/adapter.sh" preflight > "$fixture/output" 2>&1
    else
        reject preflight "failed $failed_phase preparation"
        if [[ "$failed_phase" == fetch ]]; then
            grep -Fq 'fixture locked fetch failure' "$fixture/output"
        else
            grep -Fq "fixture selected tool $failed_phase failure" "$fixture/output"
        fi
    fi
    printf '%s\n' 'fetch --manifest-path Cargo.toml --locked' > "$fixture/expected-fetch"
    cmp "$fixture/expected-fetch" "$FETCH_EVENTS"
    : > "$fixture/expected-preparation"
    for phase in fetch setup check; do
        printf '%s\n' "$phase" >> "$fixture/expected-preparation"
        [[ "$phase" != "$failed_phase" ]] || break
    done
    cmp "$fixture/expected-preparation" "$PREPARATION_EVENTS"
    cmp "$fixture/pending-changelog" CHANGELOG.md
    [[ ! -e .git/release-state && -z "$(ls -A "$fixture/tmp")" ]] || exit 1
    for path in Cargo.toml Cargo.lock README.md; do
        git show "HEAD:$path" > "$fixture/original-metadata"
        cmp "$fixture/original-metadata" "$path"
    done
done
unset FAIL_FETCH_MANIFEST FAIL_TOOL_PHASE
# Neither fetch nor the selected setup may override explicit offline policy.
: > "$PREPARATION_EVENTS"
CARGO_NET_OFFLINE=true bash "$root/scripts/release/adapter.sh" preflight > "$fixture/output" 2>&1
printf 'fetch\nsetup\ncheck\n' > "$fixture/expected-preparation"
cmp "$fixture/expected-preparation" "$PREPARATION_EVENTS"
[[ "$(cat "$PREPARATION_EVENTS.offline")" == true ]] || exit 1
[[ "$(cat "$FETCH_EVENTS.offline")" == true ]] || exit 1
cmp "$fixture/pending-changelog" CHANGELOG.md
cp "$fixture/original-changelog" CHANGELOG.md
[[ "$(git rev-parse HEAD)" == "$original_head" ]] || exit 1
git diff --quiet HEAD --
git diff --cached --quiet
fixture_complete=true
echo 'Real release-index admission checks passed (reused history; Cargo/tool stubs)'
