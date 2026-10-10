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
    bash "$script_dir/check-lockfiles.sh" || return
    awk -v heading="## [$RELEASE_VERSION] - ${RELEASE_DATE:?}" \
        '$0 == heading { n++ } END { if (n != 1) exit 1 }' CHANGELOG.md
}

case "${1:-}" in
    preflight)
        previous_version="$(bash scripts/release/workspace-version.sh)" || exit $?
        [[ "$previous_version" == "${RELEASE_PREVIOUS:?}" ]] || exit 1
        bash "$script_dir/../ci/check-release-source.sh" \
            --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
        IC_TIMERS_RELEASE_DATE="${RELEASE_DATE:?}" bash scripts/release/bump-version.sh --check "${RELEASE_VERSION:?}"
        # Prepare only the admitted graph's existing owners, in order. Fetching
        # sources alone does not install the root-lock-selected executable.
        make --no-print-directory fetch
        make --no-print-directory install-testkit-server
        make --no-print-directory pocketic-check
        ;;
    check)
        check_metadata
        ;;
    commit-check)
        bash "$script_dir/../ci/check-release-source.sh" \
            --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
        git ls-files --error-unmatch -- Cargo.toml Cargo.lock CHANGELOG.md > /dev/null
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
