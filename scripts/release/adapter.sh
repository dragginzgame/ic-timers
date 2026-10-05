#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
    preflight)
        [[ "$(bash scripts/release/workspace-version.sh)" == "${RELEASE_PREVIOUS:?}" ]]
        # Capture all producer output before admitting paths, including staged work.
        paths="$(mktemp "${TMPDIR:-/tmp}/timers-release-paths.XXXXXX")"
        trap 'rm -f "$paths"' EXIT
        git diff --name-only -z HEAD -- > "$paths"
        git ls-files --others --exclude-standard -z >> "$paths"
        while IFS= read -r -d '' path; do
            case "$path" in Cargo.toml|Cargo.lock|testing/Cargo.lock|CHANGELOG.md|README.md) ;;
                *) printf 'uncommitted non-release path: %q\n' "$path" >&2; exit 1 ;;
            esac
        done < "$paths"
        IC_TIMERS_RELEASE_DATE="${RELEASE_DATE:?}" bash scripts/release/bump-version.sh --check "${RELEASE_VERSION:?}"
        cargo fetch --manifest-path Cargo.toml --locked --offline
        cargo fetch --manifest-path testing/Cargo.toml --locked --offline
        ;;
    check)
        [[ "$(bash scripts/release/workspace-version.sh)" == "${RELEASE_VERSION:?}" ]]
        cargo sort --workspace --check
        cargo sort --workspace --check testing
        bash scripts/release/readme-version.sh --check
        bash scripts/release/check-lockfiles.sh
        awk -v heading="## [$RELEASE_VERSION] - ${RELEASE_DATE:?}" \
            '$0 == heading { n++ } END { if (n != 1) exit 1 }' CHANGELOG.md
        ;;
    *) echo 'usage: adapter.sh preflight|check' >&2; exit 2 ;;
esac
