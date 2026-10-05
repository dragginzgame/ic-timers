#!/usr/bin/env bash
set -euo pipefail

expected_version="pocket-ic-server 15.0.0"
pocket_ic_bin="${POCKET_IC_BIN:-}"
auto_install="${POCKET_IC_AUTO_INSTALL:-0}"

if [[ -z "${pocket_ic_bin}" ]]; then
    echo "error: POCKET_IC_BIN resolved to an empty path" >&2
    exit 2
fi

# Digests and provenance belong to docs/releasing.md#pocketic-artifact-pins.
# Resolve each query independently so a failed OS query cannot be hidden by a
# successful architecture query. Overrides admit only this host's exact artifact.
host_os="$(uname -s)"
host_arch="$(uname -m)"
case "${host_os}/${host_arch}" in
    Linux/x86_64)
        asset="pocket-ic-x86_64-linux.gz"
        expected_archive_sha256="972d592975bdd0f046b05b5414ed6f9a044c676ef912b8ac81d359dd4982b038"
        expected_sha256="29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb"
        ;;
    Darwin/x86_64)
        asset="pocket-ic-x86_64-darwin.gz"
        expected_archive_sha256="1d133a07c08c8e8ce25a2d08e6e7a590c27a5621735a0ab95c19f111d73f9f72"
        expected_sha256="e0a93fdd0b11345096797807002fcccecf7004c9ebfcdb1c0a3f4bcab2344964"
        ;;
    Darwin/arm64)
        asset="pocket-ic-arm64-darwin.gz"
        expected_archive_sha256="e6a96df4559949091411877f562512e132b75916bb704c03494bc7877fcfea0e"
        expected_sha256="3635e41075cded4c0fcfe4bb80b55d03324f8fa85a1a99a0f9eed2a097c50eb0"
        ;;
    *)
        echo "error: no audited PocketIC artifact for ${host_os}/${host_arch}" >&2
        exit 1
        ;;
esac
expected_url="https://github.com/dfinity/pocketic/releases/download/15.0.0/${asset}"

sha256() {
    # Core Perl Digest::SHA avoids a GNU sha256sum dependency on macOS.
    perl -MDigest::SHA -e '
        open my $input, "<:raw", $ARGV[0] or die "$ARGV[0]: $!\n";
        print Digest::SHA->new(256)->addfile($input)->hexdigest, "\n";
    ' "${1}"
}

verify_binary() {
    local candidate="${1}"
    if [[ ! -f "${candidate}" ]]; then
        echo "error: PocketIC binary is not a regular file: ${candidate}" >&2
        return 1
    fi
    if [[ ! -x "${candidate}" ]]; then
        echo "error: PocketIC binary is not executable: ${candidate}" >&2
        return 1
    fi
    local actual_sha256='<unavailable>' actual_version='<not executed: hash mismatch>'
    if actual_sha256="$(sha256 "${candidate}")"; then
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

# mv treats a directory (including a directory symlink) as a container rather
# than replacing the selected cache path. Do not provision through a link or
# replace another file type. Verified file symlinks remain valid read-only input.
if [[ -L "${pocket_ic_bin}" || ( -e "${pocket_ic_bin}" && ! -f "${pocket_ic_bin}" ) ]]; then
    echo "error: automatic PocketIC installation needs a regular cache file or absent path: ${pocket_ic_bin}" >&2
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
actual_archive_sha256="$(sha256 "${archive}")"
if [[ "${actual_archive_sha256}" != "${expected_archive_sha256}" ]]; then
    echo "error: downloaded PocketIC archive does not match the audited artifact" >&2
    echo "expected SHA-256: ${expected_archive_sha256}" >&2
    echo "actual SHA-256:   ${actual_archive_sha256}" >&2
    exit 1
fi
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
