#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 3 ]] || { echo 'usage: update-local-lock.sh LOCKFILE PREVIOUS CANDIDATE' >&2; exit 2; }
semver='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
[[ "$2" =~ $semver && "$3" =~ $semver ]] || exit 2
lockfile="$1"
[[ -f "$lockfile" && ! -L "$lockfile" ]]
output="$(mktemp "${TMPDIR:-/tmp}/timers-local-lock.XXXXXX")"
trap 'rm -f "$output"' EXIT
perl -0777 -e '
    use strict;
    use warnings;
    my ($path, $previous, $candidate) = @ARGV;
    open my $input, "<", $path or die "$path: $!\n";
    my $text = do { local $/; <$input> };
    my $count = 0;
    $text =~ s{(\[\[package\]\]\n.*?)(?=\n\[\[package\]\]|\z)}{
        my $block = $1;
        if ($block =~ /^name = "ic-timers"$/m && $block !~ /^source = /m) {
            die "unexpected local ic-timers version in $path\n"
                unless $block =~ s/^version = "\Q$previous\E"$/version = "$candidate"/m;
            $count++;
        }
        $block =~ s/"\Qic-timers $previous\E"/"ic-timers $candidate"/g;
        $block
    }gse;
    die "expected one local ic-timers package in $path\n" unless $count == 1;
    print $text or die "lock output: $!\n";
' "$lockfile" "$2" "$3" > "$output"
cat "$output" > "$lockfile"
