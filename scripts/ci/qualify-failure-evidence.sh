#!/usr/bin/env bash
set -euo pipefail

# Deliberately fail a manually selected hosted job through maintained owners.
# Do not select this driver in ordinary CI or in release commands.
[[ $# == 1 && ( "$1" == early || "$1" == late ) ]] || {
    echo 'usage: qualify-failure-evidence.sh early|late' >&2; exit 2;
}
[[ "${GITHUB_ACTIONS:-}" == true && "${GITHUB_EVENT_NAME:-}" == workflow_dispatch ]] || {
    echo 'failure qualification requires an explicit manual CI dispatch' >&2; exit 2;
}
: "${GITHUB_WORKSPACE:?qualification requires GITHUB_WORKSPACE}"
: "${RUNNER_TEMP:?qualification requires RUNNER_TEMP}"
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
workspace="$GITHUB_WORKSPACE"
[[ "$workspace" == /* ]] || workspace="$PWD/$workspace"
workspace="$(cd -P "$workspace" && printf '%s/.' "$PWD")"
workspace="${workspace%/.}"
[[ "$workspace" == "$root" ]] || {
    echo 'qualification must use the selected CI checkout' >&2; exit 2;
}
stage="$1"
temporary="$RUNNER_TEMP"
[[ "$temporary" == /* ]] || temporary="$PWD/$temporary"
temporary="$(cd -P "$temporary" && printf '%s/.' "$PWD")"
temporary="${temporary%/.}"
mkdir -p "$temporary/ic-timers-fixtures"
exec bash "$root/scripts/ci/qualify-hosted-failure.sh" "$1" "$root" \
    "$temporary/ic-timers-fixtures" "$root/ci/tool-versions.env" \
    "$root/target/validation-failures"
