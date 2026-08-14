#!/usr/bin/env bash
set -euo pipefail

expected_version="pocket-ic-server 15.0.0"
expected_sha256="29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb"
expected_url="https://github.com/dfinity/pocketic/releases/download/15.0.0/pocket-ic-x86_64-linux.gz"
pocket_ic_bin="${POCKET_IC_BIN:-}"
auto_install="${POCKET_IC_AUTO_INSTALL:-0}"

if [[ -z "${pocket_ic_bin}" ]]; then
    echo "error: POCKET_IC_BIN resolved to an empty path" >&2
    exit 2
fi

binary_is_audited() {
    local actual_sha256 actual_version
    [[ -x "${pocket_ic_bin}" ]] || return 1
    actual_version="$("${pocket_ic_bin}" --version 2>/dev/null)" || return 1
    [[ "${actual_version}" == "${expected_version}" ]] || return 1
    read -r actual_sha256 _ < <(sha256sum -- "${pocket_ic_bin}")
    [[ "${actual_sha256}" == "${expected_sha256}" ]]
}

report_invalid_binary() {
    if [[ ! -x "${pocket_ic_bin}" ]]; then
        echo "error: POCKET_IC_BIN is not executable: ${pocket_ic_bin}" >&2
        return
    fi
    local actual_sha256 actual_version
    actual_version="$("${pocket_ic_bin}" --version 2>/dev/null || true)"
    read -r actual_sha256 _ < <(sha256sum -- "${pocket_ic_bin}")
    echo "error: PocketIC evidence binary does not match the audited artifact" >&2
    echo "expected version: ${expected_version}" >&2
    echo "actual version:   ${actual_version:-<unavailable>}" >&2
    echo "expected SHA-256: ${expected_sha256}" >&2
    echo "actual SHA-256:   ${actual_sha256}" >&2
}

if binary_is_audited; then
    echo "PocketIC evidence binary verified: ${expected_version} (${expected_sha256})"
    exit 0
fi

if [[ "${auto_install}" != "1" ]]; then
    report_invalid_binary
    exit 1
fi

if [[ "$(uname -s)" != "Linux" || "$(uname -m)" != "x86_64" ]]; then
    echo "error: automatic PocketIC installation is supported only on Linux x86_64" >&2
    echo "set POCKET_IC_BIN to a separately audited binary for this platform" >&2
    exit 1
fi

mkdir -p -- "$(dirname -- "${pocket_ic_bin}")"
install_directory="$(mktemp -d "$(dirname -- "${pocket_ic_bin}")/.pocket-ic-install.XXXXXX")"
archive="${install_directory}/pocket-ic.gz"
binary="${install_directory}/pocket-ic"
cleanup() {
    rm -f -- "${archive}" "${binary}"
    rmdir -- "${install_directory}" 2>/dev/null || true
}
trap cleanup EXIT

echo "Installing audited PocketIC 15.0.0 into ${pocket_ic_bin}"
curl --fail --location --silent --show-error --output "${archive}" "${expected_url}"
gzip --decompress --stdout "${archive}" > "${binary}"
chmod 0755 "${binary}"

downloaded_version="$("${binary}" --version)"
read -r downloaded_sha256 _ < <(sha256sum -- "${binary}")
if [[ "${downloaded_version}" != "${expected_version}" ]] \
    || [[ "${downloaded_sha256}" != "${expected_sha256}" ]]; then
    echo "error: downloaded PocketIC artifact failed version/hash verification" >&2
    echo "expected version: ${expected_version}" >&2
    echo "actual version:   ${downloaded_version}" >&2
    echo "expected SHA-256: ${expected_sha256}" >&2
    echo "actual SHA-256:   ${downloaded_sha256}" >&2
    exit 1
fi

mv -- "${binary}" "${pocket_ic_bin}"
rm -f -- "${archive}"
rmdir -- "${install_directory}"
trap - EXIT

echo "PocketIC evidence binary verified: ${downloaded_version} (${downloaded_sha256})"
