#!/usr/bin/env bash
set -euo pipefail

# Real Make and metadata owners; Git/Cargo effects are command stubs. This
# proves consumer selection, not live release execution or native qualification.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/committed-release-check.XXXXXX")"
trap 'rm -rf -- "$fixture"' EXIT
mkdir -p "$fixture/current/scripts" "$fixture/selected/testing" "$fixture/bin" "$fixture/tmp"
cp "$root/Makefile" "$fixture/current/"
cp -R "$root/scripts/release" "$fixture/current/scripts/"
cp "$root/tool-versions.env" "$fixture/current/"
printf '%s\n' '[workspace.package]' 'version = "0.1.0"' > "$fixture/selected/Cargo.toml"
printf '%s\n' '| API line | `0.1` |' 'ic-timers = "=0.1.0"' > "$fixture/selected/README.md"
printf '%s\n' '## [0.1.0] - 2026-10-06' > "$fixture/selected/CHANGELOG.md"
for path in Cargo.lock testing/Cargo.lock; do
    printf '%s\n' 'version = "0.1.0"' > "$fixture/selected/$path"
done
printf '%s\n' '[workspace]' > "$fixture/selected/testing/Cargo.toml"
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
    'sort --workspace --check' | 'sort --workspace --check testing') [[ "${FAIL_SORT:-0}" == 0 ]] ;;
    'metadata --manifest-path Cargo.toml --locked --offline --format-version 1') lock=Cargo.lock ;;
    'metadata --manifest-path testing/Cargo.toml --locked --offline --format-version 1') lock=testing/Cargo.lock ;;
    *) echo "unexpected Cargo command: $*" >&2; exit 35 ;;
esac
if [[ "$1" == metadata ]]; then
    [[ "${FAIL_METADATA:-0}" == 0 ]] || { echo 'fixture locked metadata failure' >&2; exit 36; }
    version="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' "$lock")"
    printf '{"packages":[{"name":"ic-timers","version":"%s"}]}\n' "$version"
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
    [[ -z "$(ls -A "$fixture/tmp")" ]]
}

for target in release-committed-check release-tagged-check release-push-check; do
    : > "$EVENTS"
    check "$target"
    for manifest in Cargo.toml testing/Cargo.toml; do
        grep -Fqx "cargo metadata --manifest-path $manifest --locked --offline --format-version 1" "$EVENTS"
    done
    [[ -z "$(ls -A "$fixture/tmp")" ]]
done
# All five metadata outputs must be checked in the selected tree, not HEAD.
for path in Cargo.toml README.md CHANGELOG.md Cargo.lock testing/Cargo.lock; do
    cp "$SELECTED_TREE/$path" "$fixture/saved"
    printf '%s\n' 'invalid selected release metadata' > "$SELECTED_TREE/$path"
    reject release-push-check "$path corruption"
    cp "$fixture/saved" "$SELECTED_TREE/$path"
done
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
export FAIL_METADATA=1
reject release-committed-check 'failed locked resolution'
grep -Fq 'fixture locked metadata failure' "$fixture/output"
unset FAIL_METADATA
export FAIL_SORT=1
reject release-committed-check 'manifest ordering failure'
unset FAIL_SORT
if "$real_make" --no-print-directory release-committed-check RELEASE_COMMIT= \
    RELEASE_VERSION=0.1.0 RELEASE_DATE=2026-10-06 > "$fixture/output" 2>&1; then exit 1; fi
# Prepared metadata checks must still inspect the current worktree.
reject release-prepared-check 'invalid current metadata'
echo 'Selected-commit Make metadata checks passed (Git/Cargo stubs; no release effects)'
