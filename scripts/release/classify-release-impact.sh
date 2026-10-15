#!/usr/bin/env bash
set -euo pipefail
script="${BASH_SOURCE[0]}"
[[ "$script" == /* ]] || script="$PWD/$script"
root="$(cd -P "${script%/*}/../.." && pwd)"
# Drain the owner pipe before publishing policy, so failed observations cannot
# become a release classification. No caller scratch file enters the subject.
exec perl -e '
    use strict; use warnings;
    my $helper = shift @ARGV;
    open my $paths, "-|", "bash", $helper, @ARGV or die "cannot start release observation: $!\n";
    my $impact = "none";
    { local $/ = "\0";
      while (my $path = <$paths>) {
        chomp $path;
        $impact = "repository" if $impact eq "none";
        $impact = "crate" if $path eq "Cargo.toml" || $path eq "crates/ic-timers/Cargo.toml"
            || $path eq "crates/ic-timers/build.rs" || index($path, "crates/ic-timers/src/") == 0;
      }
    }
    close $paths or exit(($? & 127) ? 128 + ($? & 127) : (($? >> 8) || 1));
    print "$impact\n";
' "$root/scripts/ci/collect-release-changes.sh" "$@"
