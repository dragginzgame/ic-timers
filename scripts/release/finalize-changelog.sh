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

if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] ||
    [[ ! "${release_date}" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]; then
    usage
    exit 2
fi
if [[ ! -f "${changelog}" ]]; then
    echo "error: changelog not found: ${changelog}" >&2
    exit 1
fi

temporary="$(mktemp "${changelog}.tmp.XXXXXX")"
cleanup() {
    rm -f -- "${temporary}"
}
trap cleanup EXIT

IC_TIMERS_RELEASE_VERSION="${version}" \
IC_TIMERS_RELEASE_DATE="${release_date}" \
perl -0 -e '
    use strict;
    use warnings;

    my $text = <>;
    my $version = $ENV{IC_TIMERS_RELEASE_VERSION};
    my $date = $ENV{IC_TIMERS_RELEASE_DATE};
    my $unreleased_count = () = $text =~ /^## \[Unreleased\]$/mg;

    die "error: changelog needs exactly one ## [Unreleased] heading\n"
        if $unreleased_count != 1;
    die "error: could not read the Unreleased changelog section\n"
        if $text !~ /^## \[Unreleased\]\n(.*?)(?=^## \[|\z)/ms;

    my $notes = $1;
    my $target_count = () =
        $text =~ /^## \[\Q$version\E\](?:\s+-\s+\d{4}-\d{2}-\d{2})?$/mg;
    die "error: changelog contains multiple ## [$version] headings\n"
        if $target_count > 1;
    die "error: changelog already finalized ## [$version]\n"
        if $text =~ /^## \[\Q$version\E\]\s+-\s+\d{4}-\d{2}-\d{2}$/m;

    if ($text =~ /^## \[\Q$version\E\]\n(.*?)(?=^## \[|\z)/ms) {
        my $staged_notes = $1;
        die "error: Unreleased must be empty while ## [$version] is staged\n"
            if $notes =~ /\S/;
        die "error: the staged ## [$version] section is empty\n"
            if $staged_notes !~ /\S/;

        $text =~ s/^## \[\Q$version\E\]$/## [$version] - $date/m;
    } else {
        die "error: the Unreleased changelog section is empty\n"
            if $notes !~ /\S/;

        my $replacement = "## [Unreleased]\n\n## [$version] - $date\n" . $notes;
        $text =~ s/^## \[Unreleased\]\n.*?(?=^## \[|\z)/$replacement/ms;
    }
    print $text;
' "${changelog}" > "${temporary}"

if [[ "${check_only}" == true ]]; then
    exit 0
fi

chmod --reference="${changelog}" "${temporary}"
mv -- "${temporary}" "${changelog}"
trap - EXIT

echo "Finalized CHANGELOG.md for ${version} (${release_date})"
