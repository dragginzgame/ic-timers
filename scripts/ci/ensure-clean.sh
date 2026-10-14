#!/usr/bin/env bash
set -euo pipefail
script_root="$(cd "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"

if ! git rev-parse --verify HEAD >/dev/null 2>&1; then
    echo "error: repository has no commit yet" >&2
    exit 1
fi

exec bash "$script_root/check-release-source.sh"
