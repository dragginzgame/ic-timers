#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [--check] x.y.z [YYYY-MM-DD] [CHANGELOG]" >&2
}

check_only=false
if [[ "${1:-}" == "--check" ]]; then
    check_only=true
    shift
fi

version="${1:-}"
release_date="${2:-$(date +%F)}"
changelog="${3:-CHANGELOG.md}"
previous_version="${IC_TIMERS_RELEASE_PREVIOUS:-}"

if [[ ! "${version}" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] ||
    [[ ! "${release_date}" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] ||
    [[ -n "${previous_version}" && ! "${previous_version}" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
    usage
    exit 2
fi
if [[ -L "${changelog}" || ! -f "${changelog}" ]]; then
    echo "error: changelog must be a regular file: ${changelog}" >&2
    exit 1
fi
temporary="$(mktemp "${changelog}.tmp.XXXXXX")"
cleanup() {
    rm -f -- "${temporary}"
}
trap cleanup EXIT

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# The shared selector owns candidate classification and retained bytes. The
# caller owns regular-file admission, status admission and atomic output.
# pipefail prevents a failed reader's partial output from becoming a candidate.
cat -- "${changelog}" | awk -v version="${version}" -v previous="${previous_version}" \
    -v date="${release_date}" \
    -f "${repository_root}/scripts/ci/finalize-release-changelog.awk" \
    > "${temporary}"

if [[ "${check_only}" == true ]]; then
    exit 0
fi

perl -e 'my @s = stat $ARGV[0]; @s or die "$ARGV[0]: $!\n";
    chmod($s[2] & 07777, $ARGV[1]) or die "$ARGV[1]: $!\n";' \
    "${changelog}" "${temporary}"
mv -- "${temporary}" "${changelog}"
trap - EXIT

echo "Finalized CHANGELOG.md for ${version} (${release_date})"
