#!/usr/bin/env bash
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

check_metadata() {
    # Bash 3.2 does not reliably apply errexit to functions in subshells.
    # Preserve every failed check explicitly before proceeding to the next one.
    local version
    version="$(bash "$script_dir/workspace-version.sh")" || return
    [[ "$version" == "${RELEASE_VERSION:?}" ]] || return
    cargo sort --workspace --check || return
    bash "$script_dir/readme-version.sh" --check || return
    bash "$script_dir/check-lockfiles.sh" || return
    awk -v heading="## [$RELEASE_VERSION] - ${RELEASE_DATE:?}" \
        '$0 == heading { n++ } END { if (n != 1) exit 1 }' CHANGELOG.md
}

admit_release_paths() {
    paths="$(mktemp "${TMPDIR:-/tmp}/timers-release-paths.XXXXXX")" || return
    trap 'rm -f "$paths"' EXIT
    # Inspect index and worktree independently; a restored working file can
    # otherwise conceal unrelated staged content from HEAD-to-worktree diff.
    git diff --cached --no-renames --name-only -z HEAD -- > "$paths" || return
    git diff --no-renames --name-only -z -- >> "$paths" || return
    git ls-files --others --exclude-standard -z >> "$paths" || return
    while IFS= read -r -d '' path; do
        case "$path" in Cargo.toml|Cargo.lock|CHANGELOG.md|README.md) ;;
            *) printf 'uncommitted non-release path: %q\n' "$path" >&2; exit 1 ;;
        esac
    done < "$paths"
}

case "${1:-}" in
    preflight)
        [[ "$(bash scripts/release/workspace-version.sh)" == "${RELEASE_PREVIOUS:?}" ]]
        admit_release_paths
        IC_TIMERS_RELEASE_DATE="${RELEASE_DATE:?}" bash scripts/release/bump-version.sh --check "${RELEASE_VERSION:?}"
        # Populate the selected caches before the offline checks. Requiring them
        # here would prevent the full gate's fetch target from repairing a cold cache.
        make --no-print-directory fetch
        ;;
    check)
        check_metadata
        ;;
    commit-check)
        admit_release_paths
        git ls-files --error-unmatch -- Cargo.toml Cargo.lock CHANGELOG.md README.md > /dev/null
        git diff --quiet --
        check_metadata
        ;;
    check-committed)
        # The current checks inspect the selected immutable tree, even when HEAD
        # contains newer fixes. Never execute adapter scripts from that tree.
        commit="${RELEASE_COMMIT:?selected release commit is required}"
        resolved="$(git rev-parse --verify "${commit}^{commit}")"
        if [[ "$commit" != "$resolved" ]]; then
            echo 'error: RELEASE_COMMIT must be an exact commit SHA' >&2
            exit 1
        fi
        snapshot="$(mktemp -d "${TMPDIR:-/tmp}/timers-release-metadata.XXXXXX")"
        trap 'rm -rf -- "$snapshot"' EXIT
        git archive --format=tar "$commit" | tar -xf - -C "$snapshot"
        (cd "$snapshot" && check_metadata)
        ;;
    *) echo 'usage: adapter.sh preflight|check|commit-check|check-committed' >&2; exit 2 ;;
esac
