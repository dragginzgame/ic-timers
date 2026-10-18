#!/usr/bin/env bash
# Shared companions: scripts/ci/qualify-hosted-failure.sh
set -euo pipefail

# Timers selects evidence paths; Shared admits the explicit manual request
# before creating the parent or starting a controlled failure scenario.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
exec bash "$root/scripts/ci/qualify-hosted-failure.sh" "$@" "$root" \
    "${RUNNER_TEMP:-}/ic-timers-fixtures" "$root/ci/tool-versions.env" \
    "$root/target/validation-failures"
