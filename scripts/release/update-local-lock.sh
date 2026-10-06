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
perl "$script_root/.shared-tooling/helpers/scripts/ci/rewrite-local-lock-versions.pl" \
    "$lockfile" "$2" "$3" ic-timers > "$output"
cat "$output" > "$lockfile"
