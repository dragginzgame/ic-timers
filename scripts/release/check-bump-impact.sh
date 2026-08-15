#!/usr/bin/env bash
set -euo pipefail

impact="${1:-}"
previous_version="${2:-unknown}"

case "${impact}" in
    crate)
        ;;
    repository)
        echo "warning: changes since v${previous_version} are repository-only" >&2
        echo "warning: continuing because the maintainer invoked an explicit version bump" >&2
        ;;
    none)
        echo "error: no changes exist since v${previous_version}" >&2
        exit 1
        ;;
    *)
        echo "error: unknown release-impact classification: ${impact:-<empty>}" >&2
        exit 2
        ;;
esac
