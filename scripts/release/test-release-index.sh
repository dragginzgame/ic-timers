#!/usr/bin/env bash
set -euo pipefail

# Reuse committed history without creating fixture commits. The current adapter
# reads a real isolated Git index; Cargo calls are stubs, not compilation.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/release-index-check.XXXXXX")"
trap 'rm -rf -- "$fixture"' EXIT
git clone -q --local --no-hardlinks "$root" "$fixture/repo"
mkdir "$fixture/bin" "$fixture/tmp"
export FETCH_EVENTS="$fixture/fetch-events"
real_bash="$(command -v bash)"
printf '#!%s\n' "$real_bash" > "$fixture/bin/cargo"
cat >> "$fixture/bin/cargo" <<'STUB'
set -euo pipefail
case "$1" in
    sort) ;;
    metadata) printf '{"packages":[{"name":"ic-timers","version":"%s"}]}\n' "$RELEASE_VERSION" ;;
    fetch)
        printf '%s\n' "$*" >> "$FETCH_EVENTS"
        [[ "$*" == 'fetch --manifest-path Cargo.toml --locked' ||
            "$*" == 'fetch --manifest-path testing/Cargo.toml --locked' ]]
        if [[ "${FAIL_FETCH_MANIFEST:-}" == "$3" ]]; then
            echo 'fixture locked fetch failure' >&2
            exit 37
        fi
        ;;
    *) echo "unexpected Cargo command: $*" >&2; exit 1 ;;
esac
STUB
chmod +x "$fixture/bin/cargo"
export PATH="$fixture/bin:$PATH" TMPDIR="$fixture/tmp"
cd "$fixture/repo"
original_head="$(git rev-parse HEAD)"
export RELEASE_VERSION="$(bash "$root/scripts/release/workspace-version.sh")"
export RELEASE_PREVIOUS="$RELEASE_VERSION"
export RELEASE_DATE="$(sed -n "s/^## \[$RELEASE_VERSION\] - //p" CHANGELOG.md)"

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
reject preflight 'hidden staged implementation'
grep -Fq "$source_path" "$fixture/output"
reject commit-check 'implementation in release index'
[[ "$(git write-tree)" == "$staged_tree" ]]
git reset -q HEAD -- "$source_path"

printf '\nPrepared metadata fixture.\n' >> README.md
git add README.md
staged_tree="$(git write-tree)"
git show HEAD:README.md > README.md
reject commit-check 'index differing from prepared working metadata'
[[ "$(git write-tree)" == "$staged_tree" ]]
git reset -q HEAD -- README.md

printf '\nPrepared metadata fixture.\n' >> README.md
reject commit-check 'unstaged metadata'
git add README.md
bash "$root/scripts/release/adapter.sh" commit-check
git reset -q HEAD -- README.md
git show HEAD:README.md > README.md

git rm -q --cached README.md
reject commit-check 'untracked release metadata'
git reset -q HEAD -- README.md
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
    diff:--cached) query=staged ;;
    diff:--no-renames) query=unstaged ;;
    ls-files:--others) query=untracked ;;
    diff:--quiet) query=index ;;
esac
if [[ -n "$query" && "${FAIL_ADMISSION_QUERY:-}" == "$query" ]]; then
    if [[ "${PARTIAL_ADMISSION_OUTPUT:-0}" == 1 ]]; then printf '%s\0' CHANGELOG.md; fi
    echo "fixture failed $query query" >&2
    exit 128
fi
exec "$RELEASE_INDEX_REAL_GIT" "$@"
STUB
chmod +x "$fixture/bin/git"
for operation in preflight commit-check; do
    for query in staged unstaged untracked; do
        for partial in 0 1; do
            export FAIL_ADMISSION_QUERY="$query" PARTIAL_ADMISSION_OUTPUT="$partial"
            reject "$operation" "failed $query query with partial=$partial"
            grep -Fq "fixture failed $query query" "$fixture/output"
            [[ -z "$(ls -A "$fixture/tmp")" ]]
        done
    done
done
export FAIL_ADMISSION_QUERY=index PARTIAL_ADMISSION_OUTPUT=0
reject commit-check 'failed index/worktree comparison'
grep -Fq 'fixture failed index query' "$fixture/output"
unset FAIL_ADMISSION_QUERY PARTIAL_ADMISSION_OUTPUT

# A clean release preflight prepares both selected locks instead of demanding
# a warm cache. Fetch failures stop immediately and never mutate release metadata.
cp CHANGELOG.md "$fixture/original-changelog"
export RELEASE_VERSION="$(bash "$root/scripts/ci/next-release-version.sh" "$RELEASE_PREVIOUS" patch)"
# HEAD may already contain an undated draft when CI checks a preparation commit.
# Give this isolated scenario one candidate rather than adding a second draft.
printf '# Changelog\n\n## [%s]\n\n- Cache preparation fixture.\n\n## [%s] - %s\n\n- Prior fixture.\n' \
    "$RELEASE_VERSION" "$RELEASE_PREVIOUS" "$RELEASE_DATE" > CHANGELOG.md
cp CHANGELOG.md "$fixture/pending-changelog"
for failed_manifest in '' Cargo.toml testing/Cargo.toml; do
    : > "$FETCH_EVENTS"
    export FAIL_FETCH_MANIFEST="$failed_manifest"
    if [[ -z "$failed_manifest" ]]; then
        bash "$root/scripts/release/adapter.sh" preflight > "$fixture/output" 2>&1
    else
        reject preflight "failed $failed_manifest cache preparation"
        grep -Fq 'fixture locked fetch failure' "$fixture/output"
    fi
    printf '%s\n' 'fetch --manifest-path Cargo.toml --locked' > "$fixture/expected-fetch"
    if [[ "$failed_manifest" != Cargo.toml ]]; then
        printf '%s\n' 'fetch --manifest-path testing/Cargo.toml --locked' >> "$fixture/expected-fetch"
    fi
    cmp "$fixture/expected-fetch" "$FETCH_EVENTS"
    cmp "$fixture/pending-changelog" CHANGELOG.md
    [[ ! -e .git/release-state && -z "$(ls -A "$fixture/tmp")" ]]
    for path in Cargo.toml Cargo.lock testing/Cargo.lock README.md; do
        git show "HEAD:$path" > "$fixture/original-metadata"
        cmp "$fixture/original-metadata" "$path"
    done
done
unset FAIL_FETCH_MANIFEST
cp "$fixture/original-changelog" CHANGELOG.md
[[ "$(git rev-parse HEAD)" == "$original_head" ]]
git diff --quiet HEAD --
git diff --cached --quiet
echo 'Real release-index admission checks passed (reused history; Cargo stubs)'
