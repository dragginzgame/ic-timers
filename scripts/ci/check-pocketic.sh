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

verify_binary() {
    local candidate="${1}"
    if [[ ! -x "${candidate}" ]]; then
        echo "error: PocketIC binary is not executable: ${candidate}" >&2
        return 1
    fi
    local actual_sha256='<unavailable>' actual_version='<not executed: hash mismatch>'
    if actual_sha256="$(sha256sum < "${candidate}")"; then
        actual_sha256="${actual_sha256%% *}"
        if [[ "${actual_sha256}" == "${expected_sha256}" ]]; then
            if actual_version="$("${candidate}" --version 2>/dev/null)"; then
                if [[ "${actual_version}" == "${expected_version}" ]]; then
                    return 0
                fi
            else
                actual_version='<unavailable>'
            fi
        fi
    else
        actual_sha256='<unavailable>'
    fi
    echo "error: PocketIC evidence binary does not match the audited artifact" >&2
    echo "expected version: ${expected_version}" >&2
    echo "actual version:   ${actual_version:-<unavailable>}" >&2
    echo "expected SHA-256: ${expected_sha256}" >&2
    echo "actual SHA-256:   ${actual_sha256}" >&2
    return 1
}

if verification_error="$(verify_binary "${pocket_ic_bin}" 2>&1)"; then
    echo "PocketIC evidence binary verified: ${expected_version} (${expected_sha256})"
    exit 0
fi

if [[ "${auto_install}" != "1" ]]; then
    printf '%s\n' "${verification_error}" >&2
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

if ! verify_binary "${binary}"; then
    echo "error: downloaded PocketIC artifact failed verification" >&2
    exit 1
fi

mv -- "${binary}" "${pocket_ic_bin}"
rm -f -- "${archive}"
rmdir -- "${install_directory}"
trap - EXIT

echo "PocketIC evidence binary verified: ${expected_version} (${expected_sha256})"
