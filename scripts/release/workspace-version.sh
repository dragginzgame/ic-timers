#!/usr/bin/env bash
set -euo pipefail

# Own the repository's workspace.package version, never a dependency version.
# Deliberately require one literal, canonical SemVer field in that table.
if (( $# != 0 )) && [[ $# != 3 || "$1" != set ]]; then
    echo "Usage: $0 [set PREVIOUS_VERSION NEW_VERSION]" >&2
    exit 2
fi

perl -e '
    use strict;
    use warnings;
    my ($operation, $previous, $new) = @ARGV;
    my $semver = qr/(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)/;
    open my $input, "<", "Cargo.toml" or die "Cargo.toml: $!\n";
    my $text = do { local $/; <$input> };
    close $input or die "Cargo.toml: $!\n";
    my ($in_package, $tables, $fields, $offset) = (0, 0, 0, 0);
    my ($version, $start);
    for my $line (split /(?<=\n)/, $text) {
        if ($line =~ /^[ \t]*\[/) {
            $in_package = $line =~ /^[ \t]*\[[ \t]*workspace\.package[ \t]*\][ \t]*(?:#[^\r\n]*)?(?:\r?\n|\z)/;
            $tables++ if $in_package;
        } elsif ($in_package && $line =~ /^[ \t]*version[ \t]*=/) {
            $fields++;
            die "error: workspace.package.version must be a literal canonical SemVer\n"
                unless $line =~ /^([ \t]*version[ \t]*=[ \t]*")($semver)("[ \t]*(?:#[^\r\n]*)?(?:\r?\n|\z))/;
            ($version, $start) = ($2, $offset + length($1));
        }
        $offset += length($line);
    }
    die "error: Cargo.toml needs exactly one workspace.package table and version\n"
        unless $tables == 1 && $fields == 1;
    if (defined $operation) {
        die "error: expected and new workspace versions must be canonical SemVer\n"
            unless $previous =~ /^$semver\z/ && $new =~ /^$semver\z/;
        die "error: workspace version changed: expected $previous, found $version\n"
            unless $version eq $previous;
        substr($text, $start, length($version), $new);
        open my $output, ">", "Cargo.toml" or die "Cargo.toml: $!\n";
        print {$output} $text or die "Cargo.toml: $!\n";
        close $output or die "Cargo.toml: $!\n";
    } else {
        print "$version\n";
    }
' "$@"
