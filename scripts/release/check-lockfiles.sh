#!/usr/bin/env bash
set -euo pipefail

# Dependency resolution validates the complete root graph without building packages.
# --no-deps would skip resolution and accept stale path-package versions.
version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
for manifest in Cargo.toml; do
    # Parse only successful Cargo output; preserve download/resolution failures
    # rather than adding a JSON error for empty or partial producer output.
    metadata_json="$(cargo metadata --manifest-path "${manifest}" --locked --offline --format-version 1)"
    printf '%s\n' "${metadata_json}" \
        | IC_TIMERS_EXPECTED_VERSION="${version}" perl -MJSON::PP -0777 -e '
            my $metadata = decode_json(<>);
            my @timers = grep { $_->{name} eq "ic-timers" } @{$metadata->{packages}};
            die "error: locked workspace must resolve one ic-timers at workspace.package version $ENV{IC_TIMERS_EXPECTED_VERSION}\n"
                unless @timers == 1 && $timers[0]->{version} eq $ENV{IC_TIMERS_EXPECTED_VERSION};
        '
done
