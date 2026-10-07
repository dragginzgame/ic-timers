#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
pins="${repository_root}/ci/ic-tools.tsv"
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
        host_platform=linux-x86_64
        expected_sha256="69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792"
        ;;
    Darwin/x86_64)
        asset="pocket-ic-x86_64-darwin.gz"
        host_platform=darwin-x86_64
        expected_sha256="b8233ebee53452db7465b43e7b2ff80f2e1445dc148eb2b4b237493d8d15ec66"
        ;;
    Darwin/arm64)
        asset="pocket-ic-arm64-darwin.gz"
        host_platform=darwin-arm64
        expected_sha256="781f643d4b16105e7544ca810a972f99c0ef1919016c680faa93f10909a14496"
        ;;
    *)
        echo "error: no audited PocketIC artifact for ${host_os}/${host_arch}" >&2
        exit 1
        ;;
esac
# The shared matrix owns archive identities; the consumer still owns audited
# extracted-binary digests and single-artifact provisioning for release evidence.
server_version="$(awk -v tool=pocket-ic \
    -f "${repository_root}/scripts/ci/ic-tool-pins.awk" "${pins}")"
expected_version="pocket-ic-server ${server_version}"
expected_archive_sha256="$(awk -F '\t' -v host="${host_platform}" \
    '$1 == "pocket-ic" && $3 == host { print $4 }' "${pins}")"
expected_url="https://github.com/dfinity/pocketic/releases/download/${server_version}/${asset}"
checker="${repository_root}/scripts/ci/check-pocketic-binary.sh"
checksum="${repository_root}/scripts/ci/verify-file-checksum.sh"

if verification_error="$(bash "${checker}" "${server_version}" "${expected_sha256}" "${pocket_ic_bin}" 2>&1)"; then
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

echo "Installing audited PocketIC ${server_version} into ${pocket_ic_bin}"
curl --fail --location --silent --show-error --output "${archive}" "${expected_url}"
bash "${checksum}" sha256 "${expected_archive_sha256}" "${archive}"
gzip --decompress --stdout "${archive}" > "${binary}"
chmod 0755 "${binary}"

if ! bash "${checker}" "${server_version}" "${expected_sha256}" "${binary}"; then
    echo "error: downloaded PocketIC artifact failed verification" >&2
    exit 1
fi

mv -- "${binary}" "${pocket_ic_bin}"
rm -f -- "${archive}"
rmdir -- "${install_directory}"
trap - EXIT

echo "PocketIC evidence binary verified: ${expected_version} (${expected_sha256})"
