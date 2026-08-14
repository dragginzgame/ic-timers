#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

semver_pattern='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${version}" =~ ${semver_pattern} ]]; then
    echo "error: failed to read a canonical workspace version" >&2
    exit 1
fi

release_note="docs/changelog/${version}.md"
if [[ ! -f "${release_note}" ]]; then
    echo "error: release note is missing: ${release_note}" >&2
    exit 1
fi
if ! grep -Eq -- "^## \[${version//./\.}\] - [0-9]{4}-[0-9]{2}-[0-9]{2}$" CHANGELOG.md; then
    echo "error: CHANGELOG.md has no dated ${version} release heading" >&2
    exit 1
fi
if ! grep -Fqx -- "Status: released ${version}." "${release_note}"; then
    echo "error: ${release_note} does not describe released ${version}" >&2
    exit 1
fi
if ! grep -Fqx -- "- Workspace package version: \`${version}\`." docs/status/current.md; then
    echo "error: current status does not match workspace version ${version}" >&2
    exit 1
fi
if IC_TIMERS_CURRENT_VERSION="${version}" perl -0 -e '
    use strict;
    use warnings;

    my $version = $ENV{IC_TIMERS_CURRENT_VERSION};
    my $status = do { local $/; <> };
    for my $paragraph (split /\n[[:space:]]*\n/, $status) {
        my $mentions_current = $paragraph =~ /\Q$version\E/;
        next if !$mentions_current;
        exit 0 if $paragraph =~ /\bcandidate\b/i;
        exit 0 if $paragraph =~ /\brelease[ -]next\b/i;
        exit 0 if $paragraph =~ /\bnext release\b/i;
        exit 0 if $paragraph =~ /\b(?:expose|publish|tag)\b.{0,80}\brelease flow\b/is;
        exit 0 if $paragraph =~ /\brelease flow\b.{0,80}\b(?:later|next|pending)\b/is;
        exit 0 if $paragraph =~ /\b(?:after|before|once|when)\b.{0,80}\breleased\b/is;
    }
    exit 1;
' docs/status/current.md; then
    echo "error: current status describes released ${version} as a candidate or future release" >&2
    exit 1
fi
if grep -Eq -- '^Version [0-9]+\.[0-9]+\.[0-9]+ is the current published release\.' README.md; then
    echo "error: README.md duplicates mutable release-version truth" >&2
    exit 1
fi
if ! grep -Fqx -- 'Status: downstream contract; Canic has not adopted `ic-timers`.' \
    docs/adoption/canic.md; then
    echo "error: Canic contract status must remain release-version neutral" >&2
    exit 1
fi

echo "Release truth checks passed for ${version}"
