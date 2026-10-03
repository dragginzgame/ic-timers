#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

expected_source="crates/ic-timers/src/platform.rs"
provider_sources="$(
    grep -RFl --include='*.rs' -- 'ic_cdk_timers' crates/ic-timers/src \
        | LC_ALL=C sort \
        || true
)"
if [[ "${provider_sources}" != "${expected_source}" ]]; then
    echo "error: direct ic-cdk-timers use must remain solely in ${expected_source}" >&2
    if [[ -n "${provider_sources}" ]]; then
        echo "found:" >&2
        echo "${provider_sources}" >&2
    fi
    exit 1
fi

if ! grep -Fqx -- 'mod platform;' crates/ic-timers/src/lib.rs >/dev/null; then
    echo "error: the provider boundary must remain a private module" >&2
    exit 1
fi

# Follow local import aliases and inspect complete export/type-alias statements.
# Crate visibility also lets Rust reject indirect platform re-exports.
# Perl, rather than the shell, expands its variables.
# shellcheck disable=SC2016
find crates/ic-timers/src -name '*.rs' -print0 | xargs -0 perl -0777 -ne '
    my %boundary = map { $_ => 1 } qw(ic_cdk_timers platform);
    my @imports = /\buse\s+([^;]+);/sg;
    my $previous = 0;
    while ($previous != scalar keys %boundary) {
        $previous = scalar keys %boundary;
        my $names = join "|", map { quotemeta $_ } keys %boundary;
        for my $import (@imports) {
            next unless $import =~ /\b(?:$names)\b/;
            $boundary{$1} = 1 while $import =~ /\bas\s+(\w+)/g;
            $boundary{$1} = 1 while $import =~ /\b(?:$names)\s*::\s*(\w+)/g;
            while ($import =~ /\b(?:$names)\s*::\s*\{([^{}]+)\}/g) {
                my $group = $1;
                $boundary{$1} = 1 while $group =~ /\b(\w+)\b/g;
            }
        }
    }
    my $names = join "|", map { quotemeta $_ } keys %boundary;
    if (/\bpub\s+use\s+[^;]*\b(?:$names)\b/s
        || /\bpub\s+type\s+\w+[^;]*=\s*[^;]*\b(?:$names)\b/s
        || /\bpub\s+(?:extern\s+crate|mod)\s+(?:ic_cdk_timers|platform)\b/) {
        die "error: provider/platform public export in $ARGV\n";
    }
    if ($ARGV eq "crates/ic-timers/src/platform.rs"
        && /\bpub\s+(?:struct|enum|type|fn|use)\b/) {
        die "error: platform items must have restricted visibility in $ARGV\n";
    }
'

echo "Provider boundary checks passed"
