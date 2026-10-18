#!/usr/bin/env bash
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
metadata_checker="$script_dir/../../scripts/ci/cargo-release-metadata.sh"
bash "$script_dir/../ci/check-release-delivery.sh" \
    "$script_dir/../../ci/release-delivery.txt" "${RELEASE_DELIVERY-direct}"

case "${1:-}" in
    preflight)
        previous_version="$(bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)" || exit $?
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
        RELEASE_COMMIT='' bash "$metadata_checker" check ic-timers --manifest-order-grouped
        ;;
    commit-check)
        bash "$script_dir/../ci/check-release-source.sh" \
            --allow Cargo.toml --allow Cargo.lock --allow CHANGELOG.md
        git ls-files --error-unmatch -- Cargo.toml Cargo.lock CHANGELOG.md > /dev/null
        git diff --quiet --
        RELEASE_COMMIT='' bash "$metadata_checker" check ic-timers --manifest-order-grouped
        ;;
    check-committed)
        : "${RELEASE_COMMIT:?selected release commit is required}"
        bash "$metadata_checker" check ic-timers --manifest-order-grouped
        ;;
    *) echo 'usage: adapter.sh preflight|check|commit-check|check-committed' >&2; exit 2 ;;
esac
