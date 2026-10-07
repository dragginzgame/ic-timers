#!/usr/bin/env bash
set -euo pipefail
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ $# -eq 3 ]] || { echo 'usage: update-local-lock.sh LOCKFILE PREVIOUS CANDIDATE' >&2; exit 2; }
semver='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
[[ "$2" =~ $semver && "$3" =~ $semver ]] || exit 2
lockfile="$1"
[[ -f "$lockfile" && ! -L "$lockfile" ]]
output="$(mktemp "${TMPDIR:-/tmp}/timers-local-lock.XXXXXX")"
trap 'rm -f "$output"' EXIT
# Cargo supplies the actual member inventory after the manifest bump. The
# no-deps projection does not resolve or rewrite the still-previous lockfile.
metadata_json="$(cargo metadata --manifest-path "$script_root/Cargo.toml" \
    --no-deps --locked --offline --format-version 1)"
package_names="$(printf '%s\n' "$metadata_json" | IC_TIMERS_CANDIDATE="$3" \
    perl -MJSON::PP -0777 -e '
        my $metadata = decode_json(<>);
        my %members = map { $_ => 1 } @{$metadata->{workspace_members}};
        my @packages = grep { $members{$_->{id}} } @{$metadata->{packages}};
        @packages or die "error: workspace has no packages\n";
        for my $package (@packages) {
            $package->{version} eq $ENV{IC_TIMERS_CANDIDATE}
                or die "error: workspace member must inherit the release version\n";
            print "$package->{name}\n";
        }
    ')"
packages=()
while IFS= read -r package; do packages+=("$package"); done <<< "$package_names"
perl "$script_root/.shared-tooling/helpers/scripts/ci/rewrite-local-lock-versions.pl" \
    "$lockfile" "$2" "$3" "${packages[@]}" > "$output"
cat "$output" > "$lockfile"
