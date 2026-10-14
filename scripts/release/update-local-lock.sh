#!/usr/bin/env bash
set -euo pipefail
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ $# -eq 3 ]] || { echo 'usage: update-local-lock.sh LOCKFILE PREVIOUS CANDIDATE' >&2; exit 2; }
semver='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'
[[ "$2" =~ $semver && "$3" =~ $semver ]] || exit 2
lockfile="$1"
[[ -f "$lockfile" && ! -L "$lockfile" ]] || exit 1
output="$(mktemp "${TMPDIR:-/tmp}/timers-local-lock.XXXXXX")"
trap 'rm -f "$output"' EXIT
# Cargo supplies the actual member inventory after the manifest bump. The
# no-deps projection does not resolve or rewrite the still-previous lockfile.
metadata_json="$(cargo metadata --manifest-path "$script_root/Cargo.toml" \
    --no-deps --locked --offline --format-version 1)"
package_names="$(printf '%s\n' "$metadata_json" | jq -L "$script_root/.shared-tooling/helpers/scripts/ci" \
    -er --arg primary ic-timers --arg version "$3" '
        include "cargo-workspace-members";
        coordinated_members($primary; $version) | .[].name')"
packages=()
while IFS= read -r package; do packages+=("$package"); done <<< "$package_names"
perl "$script_root/.shared-tooling/helpers/scripts/ci/rewrite-local-lock-versions.pl" \
    "$lockfile" "$2" "$3" "${packages[@]}" > "$output"
cat "$output" > "$lockfile"
