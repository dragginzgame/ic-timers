#!/usr/bin/env bash
set -euo pipefail

# Dependency resolution validates both locks without building either workspace.
# --no-deps would skip resolution and accept stale path-package versions.
version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
for manifest in Cargo.toml testing/Cargo.toml; do
    cargo metadata --manifest-path "${manifest}" --locked --offline --format-version 1 \
        | IC_TIMERS_EXPECTED_VERSION="${version}" perl -MJSON::PP -0777 -e '
            my $metadata = decode_json(<>);
            my @timers = grep { $_->{name} eq "ic-timers" } @{$metadata->{packages}};
            die "error: locked workspace must resolve one ic-timers at workspace.package version $ENV{IC_TIMERS_EXPECTED_VERSION}\n"
                unless @timers == 1 && $timers[0]->{version} eq $ENV{IC_TIMERS_EXPECTED_VERSION};
        '
done
