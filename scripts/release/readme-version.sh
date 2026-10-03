#!/usr/bin/env bash
set -euo pipefail

case "${1:-}" in
    --check | --update) ;;
    *) echo "Usage: $0 --check|--update" >&2; exit 2 ;;
esac
if [[ "$#" != 1 ]]; then
    echo "Usage: $0 --check|--update" >&2
    exit 2
fi

# README fields are projections of workspace truth, never version selectors.
version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
IC_TIMERS_README_VERSION="${version}" perl -0 -e '
    use strict;
    use warnings;
    my ($mode, $path) = @ARGV;
    open my $input, "<", $path or die "$path: $!\n";
    my $text = do { local $/; <$input> };
    close $input or die "$path: $!\n";
    my $version = $ENV{IC_TIMERS_README_VERSION};
    my ($major, $minor) = split /\./, $version;
    my $api = "$major.$minor";
    my $number = qr/(?:0|[1-9][0-9]*)/;
    my $api_field = qr/^(\|[^|\n]*API line[ \t]*\|[ \t]*`)$number\.$number(`[ \t]*\|[ \t]*)$/m;
    my $pin_field = qr/^([ \t]*ic-timers[ \t]*=[ \t]*"=)$number\.$number\.$number("[ \t]*(?:\#[^\n]*)?)$/m;
    my @api_lines = $text =~ /^(\|[^|\n]*API line[ \t]*\|[^\n]*)$/mg;
    my @pin_lines = $text =~ /^([ \t]*ic-timers[ \t]*=[^\n]*)$/mg;
    die "error: README.md needs one canonical API line field\n"
        if @api_lines != 1 || $api_lines[0] !~ $api_field;
    die "error: README.md needs one canonical exact ic-timers dependency\n"
        if @pin_lines != 1 || $pin_lines[0] !~ $pin_field;
    my $updated = $text;
    $updated =~ s/$api_field/$1 . $api . $2/e;
    $updated =~ s/$pin_field/$1 . $version . $2/e;
    if ($mode eq "--check") {
        die "error: README.md version fields do not match workspace $version\n"
            if $updated ne $text;
    } elsif ($updated ne $text) {
        open my $output, ">", $path or die "$path: $!\n";
        print {$output} $updated or die "$path: $!\n";
        close $output or die "$path: $!\n";
    }
' -- "${1}" README.md
