#!/usr/bin/env bash
set -euo pipefail

# Real Make and metadata owners; Git/Cargo effects are command stubs. This
# proves consumer selection, not live release execution or native qualification.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export PATH="$root/.tools/host/bin:$PATH"
export YQ="$root/.tools/host/bin/yq"
COMMITTED_RELEASE_REAL_CARGO="$(command -v cargo)"
export COMMITTED_RELEASE_REAL_CARGO
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/committed-release-check.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    if [[ "$fixture_complete" != true || "$status" != 0 ]]; then
        fixture_replay_logs "$fixture/output"
    fi
    fixture_finish "$status" "$fixture" 'Failed committed-release fixture retained'
}
trap finish EXIT
mkdir -p "$fixture/current/scripts" "$fixture/current/scripts/ci" \
    "$fixture/selected" "$fixture/bin" "$fixture/tmp"
cp "$root/Makefile" "$fixture/current/"
bash "$root/scripts/ci/prepare-make-fixture.sh" "$root" "$fixture/current"
cp -R "$root/scripts/release" "$fixture/current/scripts/"
cp "$root/scripts/ci/read-cargo-workspace-version.sh" \
    "$root/scripts/ci/check-release-tag.sh" \
    "$root/scripts/ci/cargo-release-metadata.sh" \
    "$root/scripts/ci/cargo-workspace-members.jq" \
    "$root/scripts/ci/compare-stable-versions.pl" \
    "$root/scripts/ci/StableVersion.pm" \
    "$fixture/current/scripts/ci/"
mkdir -p "$fixture/current/ci"
cp "$root/ci/tool-versions.env" "$fixture/current/ci/"
printf '%s\n' '[workspace.package]' 'version = "0.1.0"' > "$fixture/selected/Cargo.toml"
printf '%s\n' 'ic-timers = "=0.0.7"' > "$fixture/selected/README.md"
printf '%s\n' '## [0.1.0] - 2026-10-06' > "$fixture/selected/CHANGELOG.md"
printf '%s\n' 'version = 4' '[[package]]' 'name = "ic-timers"' 'version = "0.1.0"' \
    > "$fixture/selected/Cargo.lock"
cp "$fixture/selected/Cargo.toml" "$fixture/current/"
printf '%s\n' '## [0.1.1]' > "$fixture/current/CHANGELOG.md"
printf '%s\n' 'newer HEAD metadata must not qualify an older release' > "$fixture/current/README.md"
export SELECTED_TREE="$fixture/selected" EVENTS="$fixture/events"
export SELECTED_COMMIT=1111111111111111111111111111111111111111
export CURRENT_COMMIT=2222222222222222222222222222222222222222
real_bash="$(command -v bash)"
printf '#!%s\n' "$real_bash" > "$fixture/bin/git"
cat >> "$fixture/bin/git" <<'STUB'
set -euo pipefail
printf 'git %s\n' "$*" >> "$EVENTS"
case "$*" in
    "rev-parse --verify $SELECTED_COMMIT^{commit}") printf '%s\n' "$SELECTED_COMMIT" ;;
    "cat-file -t $SELECTED_COMMIT") printf 'commit\n' ;;
    'rev-parse HEAD') printf '%s\n' "$CURRENT_COMMIT" ;;
    "archive --format=tar $SELECTED_COMMIT")
        [[ "${FAIL_ARCHIVE:-0}" != before ]] || exit 31
        tar -cf - -C "$SELECTED_TREE" .
        [[ "${FAIL_ARCHIVE:-0}" != after ]] || exit 32
        ;;
    'cat-file -t refs/tags/v0.1.0')
        [[ "${TAG_TYPE:-tag}" != missing ]] || exit 33
        printf '%s\n' "${TAG_TYPE:-tag}"
        ;;
    'rev-parse --verify refs/tags/v0.1.0^{commit}') printf '%s\n' "${TAG_COMMIT:-$SELECTED_COMMIT}" ;;
    *) echo "unexpected Git command: $*" >&2; exit 34 ;;
esac
STUB
printf '#!%s\n' "$real_bash" > "$fixture/bin/cargo"
cat >> "$fixture/bin/cargo" <<'STUB'
set -euo pipefail
printf 'cargo %s\n' "$*" >> "$EVENTS"
case "$*" in
    'locate-project --workspace --message-format plain --manifest-path '*)
        exec "$COMMITTED_RELEASE_REAL_CARGO" "$@" ;;
    'sort --workspace --check')
        cmp "$SELECTED_TREE/Cargo.toml" Cargo.toml
        if [[ "${FAIL_SORT:-}" == "$*" ]]; then
            echo 'fixture manifest ordering failure' >&2
            exit 35
        fi
        ;;
    'metadata --locked --offline --format-version 1') lock=Cargo.lock ;;
    *) echo "unexpected Cargo command: $*" >&2; exit 35 ;;
esac
if [[ "$1" == metadata ]]; then
    [[ "${FAIL_METADATA:-}" != "$lock" ]] || { echo 'fixture locked metadata failure' >&2; exit 36; }
    version="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' "$lock" | head -n 1)"
    if [[ "${EXTRA_WORKSPACE_MEMBER:-0}" == 1 ]]; then
        printf '{"workspace_members":["timers","probe"],"packages":[{"id":"timers","name":"ic-timers","source":null,"version":"%s"},{"id":"probe","name":"probe","source":null,"version":"%s"}]}\n' "$version" "$version"
    else
        printf '{"workspace_members":["timers"],"packages":[{"id":"timers","name":"ic-timers","source":null,"version":"%s"}]}\n' "$version"
    fi
fi
STUB
chmod +x "$fixture/bin/git" "$fixture/bin/cargo"
export PATH="$fixture/bin:$PATH" TMPDIR="$fixture/tmp"
cd "$fixture/current"
real_make="$(command -v make)"

check() {
    "$real_make" --no-print-directory "$1" RELEASE_COMMIT="$SELECTED_COMMIT" \
        RELEASE_VERSION=0.1.0 RELEASE_DATE=2026-10-06 > "$fixture/output" 2>&1
}
reject() {
    if check "$1"; then echo "error: committed check accepted $2" >&2; exit 1; fi
}

for target in release-committed-check release-tagged-check release-push-check; do
    : > "$EVENTS"
    check "$target"
    grep -Fqx "cargo metadata --locked --offline --format-version 1" "$EVENTS"
    [[ "$(grep -Fxc "git archive --format=tar $SELECTED_COMMIT" "$EVENTS")" == 1 ]] || exit 1
    [[ "$(grep -Fxc 'cargo sort --workspace --check' "$EVENTS")" == 1 ]] || exit 1
    [[ -z "$(ls -A "$fixture/tmp")" ]] || exit 1
done
# Documentation need not contain canonical fields or exist in a release tree.
printf '%s\n' 'An ordinary README without version fields.' > "$SELECTED_TREE/README.md"
check release-push-check
rm "$SELECTED_TREE/README.md"
check release-push-check
[[ -z "$(ls -A "$fixture/tmp")" ]] || exit 1
# All three metadata outputs must be checked in the selected tree, not HEAD.
for path in Cargo.toml CHANGELOG.md Cargo.lock; do
    cp "$SELECTED_TREE/$path" "$fixture/saved"
    printf '%s\n' 'invalid selected release metadata' > "$SELECTED_TREE/$path"
    reject release-push-check "$path corruption"
    # Shared retains the selected export with failed metadata evidence.
    grep -Fq 'failed Cargo metadata check retained:' "$fixture/output"
    [[ -n "$(ls -A "$fixture/tmp")" ]] || exit 1
    cp "$fixture/saved" "$SELECTED_TREE/$path"
done
# Neither an omitted workspace row nor an extra local lock row can qualify.
export EXTRA_WORKSPACE_MEMBER=1
: > "$EVENTS"
reject release-push-check 'workspace member absent from the lock roster'
grep -Fq 'workspace members and admitted lock identities must agree' "$fixture/output"
if grep -Eq '^git cat-file -t refs/tags/' "$EVENTS"; then exit 1; fi
unset EXTRA_WORKSPACE_MEMBER
cp "$SELECTED_TREE/Cargo.lock" "$fixture/saved-lock"
printf '\n[[package]]\nname = "outside"\nversion = "0.1.0"\n' >> "$SELECTED_TREE/Cargo.lock"
: > "$EVENTS"
reject release-push-check 'local lock row absent from the workspace'
grep -Fq 'workspace members and admitted lock identities must agree' "$fixture/output"
if grep -Eq '^git cat-file -t refs/tags/' "$EVENTS"; then exit 1; fi
cp "$fixture/saved-lock" "$SELECTED_TREE/Cargo.lock"

for kind in missing commit; do
    export TAG_TYPE="$kind"
    reject release-tagged-check "$kind tag"
done
unset TAG_TYPE
export TAG_COMMIT="$CURRENT_COMMIT"
reject release-push-check 'tag pointing to newer HEAD'
unset TAG_COMMIT
for phase in before after; do
    export FAIL_ARCHIVE="$phase"
    reject release-committed-check "$phase archive failure"
done
unset FAIL_ARCHIVE
# Fail root graph admission. A later successful check must never mask
# the original failure, including on Apple's Bash 3.2.
lock=Cargo.lock
: > "$EVENTS"
export FAIL_METADATA="$lock"
reject release-push-check "failed $lock resolution"
grep -Fq 'fixture locked metadata failure' "$fixture/output"
if grep -Eq '^git cat-file -t refs/tags/' "$EVENTS"; then
    echo 'error: push check inspected tags after failed locked resolution' >&2
    exit 1
fi
unset FAIL_METADATA
command='sort --workspace --check'
: > "$EVENTS"
export FAIL_SORT="$command"
reject release-push-check "$command failure"
grep -Fq 'fixture manifest ordering failure' "$fixture/output"
# Sorting failure stops before locked resolution or tag lookup.
if grep -Eq '^cargo metadata |^git cat-file -t refs/tags/' "$EVENTS"; then
    echo 'error: committed check continued after manifest ordering failure' >&2
    exit 1
fi
unset FAIL_SORT
if "$real_make" --no-print-directory release-committed-check RELEASE_COMMIT= \
    RELEASE_VERSION=0.1.0 RELEASE_DATE=2026-10-06 > "$fixture/output" 2>&1; then exit 1; fi
# Prepared metadata checks must still inspect the current worktree.
reject release-prepared-check 'invalid current metadata'

# Standard entrypoint policy and forwarding belong to test-release-gate.sh.
fixture_complete=true
echo 'Selected-commit Make metadata checks passed (Git/Cargo stubs; no release effects)'
