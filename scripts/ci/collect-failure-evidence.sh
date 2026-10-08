#!/usr/bin/env bash
set -euo pipefail

# CI owns the checkout and scratch directory. Archive only the maintained
# evidence roots; the shared archiver owns modes, link handling and Git exclusion.
[[ $# == 0 ]] || { echo 'usage: collect-failure-evidence.sh' >&2; exit 2; }
: "${GITHUB_WORKSPACE:?CI collection requires GITHUB_WORKSPACE}"
: "${RUNNER_TEMP:?CI collection requires RUNNER_TEMP}"
: "${GITHUB_SHA:?CI collection requires GITHUB_SHA}"
root="$(cd "$GITHUB_WORKSPACE" && pwd -P)"
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
metadata="$(mktemp -d "$RUNNER_TEMP/ic-timers-evidence.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf "$metadata"; else printf "Failed evidence collection retained: %s\n" "$metadata" >&2; fi' EXIT
checkout_sha="$(git -C "$root" rev-parse HEAD)"
printf 'checkout_sha=%s\nevent_sha=%s\njob=%s\nhost=%s/%s\nrun=%s\nattempt=%s\n' \
    "$checkout_sha" "$GITHUB_SHA" "${GITHUB_JOB:-unknown}" \
    "${RUNNER_OS:-unknown}" "${RUNNER_ARCH:-unknown}" \
    "${GITHUB_RUN_ID:-unknown}" "${GITHUB_RUN_ATTEMPT:-unknown}" > "$metadata/identity.txt"
archive="$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz"
selections=("$metadata" identity.txt)

if [[ -d "$RUNNER_TEMP/ic-timers-fixtures" ]]; then
    selections+=("$RUNNER_TEMP" ic-timers-fixtures)
fi
# Select the raw release logs without admitting the surrounding Git metadata.
if [[ -d "$root/.git/release-state/validation-failures" ]]; then
    selections+=("$root/.git/release-state" validation-failures)
fi
shopt -s nullglob
for path in "$root/target/validation-failures" \
    "$root"/.tools/host-set.* "$root"/.tools/ic-set.*; do
    [[ -d "$path" ]] || continue
    selections+=("$root" "${path#"$root/"}")
done
# Full installer retention remains deliberate until compact selection can prove
# successful verification of each exact active bundle (Shared Tooling #66).
bash "$script_root/scripts/ci/archive-evidence.sh" "$archive" "${selections[@]}"
printf 'CI failure evidence archived at: %s\n' "$archive"
