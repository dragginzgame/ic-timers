#!/usr/bin/env bash
set -euo pipefail

[[ $# == 1 && -n "$1" && "$1" != -* && "$1" != *[[:space:]]* ]] || {
    echo 'usage: update-dev.sh MINIMUM_TOOLCHAIN (use make update-dev)' >&2
    exit 2
}
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TOOLCHAIN="$(sed -n 's/^channel = "\(.*\)"/\1/p' "${ROOT_DIR}/rust-toolchain.toml")"

if [[ -z "${TOOLCHAIN}" ]]; then
    echo "error: failed to read the pinned Rust toolchain" >&2
    exit 1
fi

"$BASH" "$ROOT_DIR/scripts/dev/prepare-rust.sh" "$TOOLCHAIN"
"$BASH" "$ROOT_DIR/scripts/dev/prepare-rust.sh" "$1"
make --no-print-directory -C "${ROOT_DIR}" install-tools
make --no-print-directory -C "${ROOT_DIR}" tools-check
make --no-print-directory -C "${ROOT_DIR}" install-hooks

echo "ic-timers development toolchain is ready: Rust ${TOOLCHAIN}, minimum $1"
