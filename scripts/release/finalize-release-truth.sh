#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [--check] PREVIOUS_VERSION NEW_VERSION" >&2
}

check_only=false
if [[ "${1:-}" == "--check" ]]; then
    check_only=true
    shift
fi

previous_version="${1:-}"
new_version="${2:-}"
semver_pattern='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
if [[ ! "${previous_version}" =~ ${semver_pattern} ]] \
    || [[ ! "${new_version}" =~ ${semver_pattern} ]]; then
    usage
    exit 2
fi

status_file="docs/status/current.md"
release_note="docs/changelog/${new_version}.md"
if [[ ! -f "${status_file}" || ! -f "${release_note}" ]]; then
    echo "error: release status or note is missing for ${new_version}" >&2
    exit 1
fi

IC_TIMERS_PREVIOUS_VERSION="${previous_version}" \
IC_TIMERS_NEW_VERSION="${new_version}" \
perl -0 -e '
    use strict;
    use warnings;

    my ($status_file, $release_note) = @ARGV;
    my $previous = $ENV{IC_TIMERS_PREVIOUS_VERSION};
    my $new = $ENV{IC_TIMERS_NEW_VERSION};
    my $status = do { local $/; open my $fh, "<", $status_file or die "$status_file: $!\n"; <$fh> };
    my $note = do { local $/; open my $fh, "<", $release_note or die "$release_note: $!\n"; <$fh> };
    my $workspace_marker = "- Workspace package version: `$previous`.";
    my $latest_marker = "- Latest release line: `$previous`.";

    my @workspace_markers = $status =~ /^(- Workspace package version:.*)$/mg;
    my @latest_markers = $status =~ /^(- Latest release line:.*)$/mg;
    die "error: current status workspace marker is missing or duplicated\n"
        if @workspace_markers != 1;
    die "error: current status workspace version does not match $previous\n"
        if $workspace_markers[0] ne $workspace_marker;
    die "error: current status latest-release marker is missing or duplicated\n"
        if @latest_markers != 1;
    die "error: current status latest release does not match $previous\n"
        if $latest_markers[0] ne $latest_marker;

    # The requested bump and changelog own target selection. Handoff prose
    # must not duplicate that authority or require particular English words.
    my @note_headings = $note =~ /^(# .*)$/mg;
    die "error: release-note heading is missing or duplicated for $new\n"
        if @note_headings != 1 || $note_headings[0] !~ /^# \Q$new\E(?:[ \t]|$)/;
    my @note_statuses = $note =~ /^Status:(.*)$/mg;
    die "error: release-note status is missing or duplicated\n"
        if @note_statuses != 1;
    die "error: release-note status is empty\n"
        if $note_statuses[0] !~ /\S/;
    die "error: release note is already finalized for $new\n"
        if $note_statuses[0] =~ /^\s*released \Q$new\E\.\s*$/;
' "${status_file}" "${release_note}"

if [[ "${check_only}" == true ]]; then
    exit 0
fi

IC_TIMERS_PREVIOUS_VERSION="${previous_version}" \
IC_TIMERS_NEW_VERSION="${new_version}" \
perl -0pi -e '
    my $previous = $ENV{IC_TIMERS_PREVIOUS_VERSION};
    my $new = $ENV{IC_TIMERS_NEW_VERSION};
    if ($ARGV eq "docs/status/current.md") {
        s/^\Q- Workspace package version: `$previous`.\E$/- Workspace package version: `$new`./m;
        s/^\Q- Latest release line: `$previous`.\E$/- Latest release line: `$new`./m;
    } elsif ($ARGV eq "docs/changelog/$new.md") {
        s/^Status:.*$/Status: released $new./m;
    }
' "${status_file}" "${release_note}"

echo "Finalized release truth for ${new_version}"
