#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ( "$1" == setup || "$1" == check ) ]] || {
    echo 'usage: release-editor.sh setup|check' >&2; exit 2;
}
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
arguments=(--consumer "$root" --package ic-build --lockfile "$root/Cargo.lock"
    --bin ic-build-release-metadata --profile release)
[[ "$1" != check ]] || arguments+=(--check)
exec bash "$root/scripts/dev/install-rust-tools.sh" "${arguments[@]}"
