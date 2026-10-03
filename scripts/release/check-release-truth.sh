#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
bash "$(dirname -- "${BASH_SOURCE[0]}")/readme-version.sh" --check

release_note="docs/changelog/${version}.md"
if [[ ! -f "${release_note}" ]]; then
    echo "error: release note is missing: ${release_note}" >&2
    exit 1
fi
IC_TIMERS_RELEASE_VERSION="${version}" perl -0 -e '
    use strict;
    use warnings;
    my ($changelog_file, $note_file, $status_file) = @ARGV;
    my $version = $ENV{IC_TIMERS_RELEASE_VERSION};
    sub read_text {
        my ($path) = @_;
        open my $fh, "<", $path or die "$path: $!\n";
        local $/;
        return <$fh>;
    }
    my $changelog = read_text($changelog_file);
    my $note = read_text($note_file);
    my $status = read_text($status_file);
    my @headers = $changelog =~ /^(## \[\Q$version\E\].*)$/mg;
    die "error: CHANGELOG.md needs exactly one dated $version release heading\n"
        if @headers != 1 || $headers[0] !~ /^## \[\Q$version\E\] - \d{4}-\d{2}-\d{2}$/;
    my @headings = $note =~ /^(# .*)$/mg;
    die "error: $note_file needs exactly one matching release heading\n"
        if @headings != 1 || $headings[0] !~ /^# \Q$version\E(?:[ \t]|$)/;
    my @statuses = $note =~ /^(Status:.*)$/mg;
    die "error: $note_file needs exactly one released $version status\n"
        if @statuses != 1 || $statuses[0] ne "Status: released $version.";
    for my $field ("Workspace package version", "Latest release line") {
        my @markers = $status =~ /^(- \Q$field\E:.*)$/mg;
        die "error: current status needs exactly one matching $field marker\n"
            if @markers != 1 || $markers[0] ne "- $field: `$version`.";
    }
' CHANGELOG.md "${release_note}" docs/status/current.md
if grep -Eq -- '^Version [0-9]+\.[0-9]+\.[0-9]+ is the current published release\.' README.md; then
    echo "error: README.md duplicates mutable release-version truth" >&2
    exit 1
fi
echo "Release truth checks passed for ${version}"
