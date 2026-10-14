#!/usr/bin/env bash
set -euo pipefail

script="${BASH_SOURCE[0]}"
[[ "$script" == /* ]] || script="$PWD/$script"
root="$(cd -P "${script%/*}/../.." && pwd)"
exec bash "$root/scripts/ci/warn-release-prose.sh" "${1:-}" "${2:-docs/status/current.md}"
