#!/usr/bin/env bash
# Shared companions: scripts/dev/prepare-rust.sh scripts/ci/read-rust-policy.sh
set -euo pipefail

[[ $# == 0 || ( $# == 1 && -n "$1" && "$1" != -* && "$1" != *[[:space:]]* ) ]] || {
    echo 'usage: update-dev.sh [MINIMUM_TOOLCHAIN] (use make update-dev)' >&2
    exit 2
}
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# Host parsers do not require Rust. Prepare them before reading either compiler
# policy, then finish Rust/native bootstrap before building any Cargo tools.
make --no-print-directory -C "${ROOT_DIR}" install-host-tools
policy="$(bash "$ROOT_DIR/scripts/ci/read-rust-policy.sh")"
TOOLCHAIN="${policy##*IC_TIMERS_INTERNAL_TOOLCHAIN=}"
minimum="${policy#IC_TIMERS_MSRV=}"
minimum="${minimum%%$'\n'*}"
[[ $# == 0 ]] || minimum="$1"

"$BASH" "$ROOT_DIR/scripts/dev/prepare-rust.sh" "$TOOLCHAIN"
"$BASH" "$ROOT_DIR/scripts/dev/prepare-rust.sh" "$minimum"
make --no-print-directory -C "${ROOT_DIR}" install-tools
make --no-print-directory -C "${ROOT_DIR}" tools-check
make --no-print-directory -C "${ROOT_DIR}" install-hooks

echo "ic-timers development toolchain is ready: Rust ${TOOLCHAIN}, minimum $minimum"
