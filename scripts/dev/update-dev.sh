#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
source "${ROOT_DIR}/tool-versions.env"
TOOLCHAIN="$(sed -n 's/^channel = "\(.*\)"/\1/p' "${ROOT_DIR}/rust-toolchain.toml")"

if [[ -z "${TOOLCHAIN}" ]]; then
    echo "error: failed to read the pinned Rust toolchain" >&2
    exit 1
fi

rustup toolchain install "${TOOLCHAIN}" --profile minimal \
    --component clippy --component rustfmt \
    --target wasm32-unknown-unknown
cargo +"${TOOLCHAIN}" install cargo-sort --version "${IC_TIMERS_CARGO_SORT_VERSION}" --locked
bash "${ROOT_DIR}/scripts/dev/install-host-tools.sh"
bash "${ROOT_DIR}/.shared-tooling/helpers/scripts/dev/install-ic-tools.sh" --consumer "${ROOT_DIR}"
bash "${ROOT_DIR}/scripts/dev/install-git-hooks.sh"

echo "ic-timers development toolchain is ready: Rust ${TOOLCHAIN}"
