#!/usr/bin/env bash
set -euo pipefail
[[ $# == 0 ]] || { echo 'Usage: workspace-version.sh' >&2; exit 2; }
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
exec bash "$script_root/scripts/ci/read-cargo-workspace-version.sh" \
    --stable "$PWD/Cargo.toml"
