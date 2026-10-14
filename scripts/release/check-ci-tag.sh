#!/usr/bin/env bash
set -euo pipefail

# Inspect the actual tagged checkout only. No fixture, setup or build dispatch.
[[ $# == 0 ]] || { echo 'usage: check-ci-tag.sh' >&2; exit 2; }
root="$(cd "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ "$(pwd -P)" == "$root" ]] || { echo 'tag admission requires the checkout root' >&2; exit 1; }
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
: "${GITHUB_REF_NAME:?event tag required}" "${GITHUB_SHA:?event commit required}"
[[ "${GITHUB_EVENT_NAME:-}" == push && "${GITHUB_REF:-}" == "refs/tags/$GITHUB_REF_NAME" &&
   "$GITHUB_SHA" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || { echo 'exact tag push identity required' >&2; exit 1; }
checkout="$(git rev-parse --verify HEAD)"
[[ "$checkout" == "$GITHUB_SHA" ]] || { echo 'event commit differs from checkout' >&2; exit 1; }
bash "$root/scripts/ci/check-release-source.sh"
version="$(bash "$root/scripts/release/workspace-version.sh")"
[[ "$GITHUB_REF_NAME" == "v$version" ]] || { echo 'event tag differs from workspace version' >&2; exit 1; }
# Reachability does not claim completion of main's native/full validation.
git merge-base --is-ancestor "$checkout" refs/remotes/origin/main
bash "$root/scripts/release/check-tag-at-head.sh" "$checkout" "$version"
# The actual release must lead its changelog with one finalized matching section.
awk -v version="$version" '
    /^## / {
        if (!seen++) {
            if ($0 !~ /^## \[[0-9]+\.[0-9]+\.[0-9]+\] - [0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]$/ ||
                $2 != "[" version "]") exit 1
        }
        if ($2 == "[" version "]") count++
    }
    END { if (seen == 0 || count != 1) exit 1 }
' CHANGELOG.md
bash "$root/scripts/ci/verify-shared-tooling-snapshot.sh" --consumer "$root"
bash "$root/scripts/ci/verify-shared-tooling-snapshot.sh" --consumer "$root" --manifest .shared-tooling-audits.snapshot
bash "$root/.shared-tooling/helpers/scripts/ci/verify-shared-tooling-snapshot.sh" \
    --consumer "$root/.shared-tooling/helpers"
bash "$root/scripts/release/check-lockfiles.sh"
printf 'Tag %s admits exact checkout %s; full qualification remains with PR/main.\n' "$GITHUB_REF_NAME" "$checkout"
