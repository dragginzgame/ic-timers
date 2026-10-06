#!/usr/bin/env bash
set -euo pipefail
script_root="$(cd "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd -P)"

case "$#" in
    0)
        version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
        selected_commit="$(git rev-parse HEAD)"
        ;;
    2)
        selected_commit="$1"
        version="$2"
        ;;
    *) echo 'usage: check-tag-at-head.sh [COMMIT_SHA VERSION]' >&2; exit 2 ;;
esac

exec bash "$script_root/.shared-tooling/helpers/scripts/ci/check-release-tag.sh" \
    "$selected_commit" "$version"
