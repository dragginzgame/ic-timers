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

IC_TIMERS_RELEASE_VERSION="${version}" \
IC_TIMERS_RELEASE_DATE="${release_date}" \
IC_TIMERS_RELEASE_PREVIOUS="${previous_version}" \
perl -0 -e '
    use strict;
    use warnings;

    my $path = shift @ARGV;
    my $text = "# Changelog\n\n";
    if (-e $path) {
        open my $input, "<", $path or die "$path: $!\n";
        local $/;
        $text = <$input>;
        close $input or die "$path: $!\n";
    }
    my $version = $ENV{IC_TIMERS_RELEASE_VERSION};
    my $date = $ENV{IC_TIMERS_RELEASE_DATE};
    my $previous = $ENV{IC_TIMERS_RELEASE_PREVIOUS};
    # Canonical components compare by length then text, never floating point.
    my $older = sub {
        my ($left, $right) = @_;
        my @left = split /\./, $left;
        my @right = split /\./, $right;
        for my $index (0 .. 2) {
            next if $left[$index] eq $right[$index];
            return length($left[$index]) < length($right[$index])
                if length($left[$index]) != length($right[$index]);
            return $left[$index] lt $right[$index];
        }
        return 0;
    };
    die "error: previous version must precede the release target\n"
        if length($previous) && !$older->($previous, $version);
    # The explicit bump owns the version. Select one current draft, whether
    # versionless or already named; history is never used as pending notes.
    die "error: changelog already finalized ## [$version]\n"
        if $text =~ /^## \[\Q$version\E\][ \t]+-[ \t]+\d{4}-\d{2}-\d{2}[ \t]*$/m;
    my $number = qr/(?:0|[1-9][0-9]*)/;
    my @drafts;
    while ($text =~ /^## \[(Draft|$number\.$number\.$number)\][ \t]*(?:\n|\z)(.*?)(?=^## |\z)/msg) {
        my $draft = [$-[0], $+[0] - $-[0], $1, $2];
        # The bump owns the previous identity. Imported undated notes at or
        # below it stay historical; only newer numbered sections are pending.
        next if length($previous) && $draft->[2] ne "Draft" &&
            !$older->($previous, $draft->[2]);
        push @drafts, $draft;
    }
    die "error: changelog has multiple undated release candidates; choose one\n"
        if @drafts > 1;
    my $notes = "";
    if (@drafts) {
        my ($start, $length, $label, $body) = @{$drafts[0]};
        die "error: named draft $label conflicts with requested release $version\n"
            if $label ne "Draft" && $label ne $version;
        $notes = $body;
        substr($text, $start, $length, "");
    }
    # Empty/missing drafts are presentation gaps, not evidence of no changes.
    # The release-impact classifier owns rejection of an unchanged subject.
    $notes =~ s/\A\s+|\s+\z//g;
    warn "warning: no release notes selected; preparing an empty $version section\n"
        unless length $notes;
    my $release = "## [$version] - $date\n\n";
    $release .= "$notes\n\n" if length $notes;
    my $position = $text =~ /^## /mg ? $-[0] : length $text;
    if ($position > 0 && substr($text, 0, $position) !~ /\n\n\z/) {
        my $separator = substr($text, 0, $position) =~ /\n\z/ ? "\n" : "\n\n";
        substr($text, $position, 0, $separator);
        $position += length $separator;
    }
    substr($text, $position, 0, $release);
    print $text or die "error: cannot write prepared changelog: $!\n";
    close STDOUT or die "error: cannot flush prepared changelog: $!\n";
' "${changelog}" > "${temporary}"

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
