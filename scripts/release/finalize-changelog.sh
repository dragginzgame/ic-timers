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
if [[ -L "${changelog}" ]] || [[ -e "${changelog}" && ! -f "${changelog}" ]]; then
    echo "error: changelog must be a regular file: ${changelog}" >&2
    exit 1
fi
temporary="$(mktemp "${changelog}.tmp.XXXXXX")"
cleanup() {
    rm -f -- "${temporary}"
}
trap cleanup EXIT

# Validate the caller's version boundary independently of draft selection.
# Compare canonical decimal components without floating-point or Bash overflow.
if [[ -n "${previous_version}" ]]; then
    perl -e '
        my ($previous, $version) = @ARGV;
        my @previous = split /\./, $previous;
        my @version = split /\./, $version;
        for my $index (0 .. 2) {
            next if $previous[$index] eq $version[$index];
            my $increasing = length($previous[$index]) != length($version[$index])
                ? length($previous[$index]) < length($version[$index])
                : $previous[$index] lt $version[$index];
            exit 0 if $increasing;
            last;
        }
        die "error: previous version must precede the release target\n";
    ' "${previous_version}" "${version}"
fi

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# The shared selector owns candidate classification and retained bytes. The
# caller owns missing-file presentation, status admission and atomic output.
# pipefail prevents a failed reader's partial output from becoming a candidate.
(
    if [[ -e "${changelog}" ]]; then
        cat -- "${changelog}"
    else
        printf '# Changelog\n\n'
    fi
) | awk -v version="${version}" -v previous="${previous_version}" \
    -v date="${release_date}" \
    -f "${repository_root}/scripts/ci/finalize-release-changelog.awk" \
    > "${temporary}"

if [[ "${check_only}" == true ]]; then
    exit 0
fi

if [[ -e "${changelog}" ]]; then
    perl -e 'my @s = stat $ARGV[0]; @s or die "$ARGV[0]: $!\n";
        chmod($s[2] & 07777, $ARGV[1]) or die "$ARGV[1]: $!\n";' \
        "${changelog}" "${temporary}"
else
    chmod 0644 "${temporary}"
fi
mv -- "${temporary}" "${changelog}"
trap - EXIT

echo "Finalized CHANGELOG.md for ${version} (${release_date})"
