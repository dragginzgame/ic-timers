#!/usr/bin/env bash
set -euo pipefail

[[ $# == 1 ]] || { echo 'usage: prepare-rust.sh TOOLCHAIN' >&2; exit 2; }
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# Timers owns this component policy; Shared owns explicit installation.
exec "$BASH" "$root/scripts/dev/prepare-rust-toolchain.sh" "$1" \
    --component clippy --component rustfmt --target wasm32-unknown-unknown
