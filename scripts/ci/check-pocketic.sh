#!/usr/bin/env bash
set -euo pipefail

expected_version="pocket-ic-server 15.0.0"
expected_sha256="29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb"
pocket_ic_bin="${POCKET_IC_BIN:-}"

if [[ -z "${pocket_ic_bin}" || ! -x "${pocket_ic_bin}" ]]; then
    echo "error: POCKET_IC_BIN must name the executable PocketIC 15.0.0 evidence binary" >&2
    exit 2
fi

actual_version="$("${pocket_ic_bin}" --version)"
if [[ "${actual_version}" != "${expected_version}" ]]; then
    echo "error: expected '${expected_version}', found '${actual_version}'" >&2
    exit 1
fi

read -r actual_sha256 _ < <(sha256sum -- "${pocket_ic_bin}")
if [[ "${actual_sha256}" != "${expected_sha256}" ]]; then
    echo "error: PocketIC 15.0.0 SHA-256 does not match the audited evidence binary" >&2
    echo "expected: ${expected_sha256}" >&2
    echo "actual:   ${actual_sha256}" >&2
    exit 1
fi

echo "PocketIC evidence binary verified: ${actual_version} (${actual_sha256})"
