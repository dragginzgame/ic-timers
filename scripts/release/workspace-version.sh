#!/usr/bin/env bash
set -euo pipefail

# Shared helpers own TOML admission and pure byte-preserving rendering. This
# adapter alone installs a validated candidate; the release caller owns rollback.
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
helpers="$script_root/.shared-tooling/helpers/scripts/ci"
if (( $# != 0 )) && [[ $# != 3 || "$1" != set ]]; then
    echo "Usage: $0 [set PREVIOUS_VERSION NEW_VERSION]" >&2
    exit 2
fi
if (( $# == 0 )); then
    exec bash "$helpers/read-cargo-workspace-version.sh" \
        --stable "$PWD/Cargo.toml"
fi

bash "$helpers/read-cargo-workspace-version.sh" --stable "$PWD/Cargo.toml" >/dev/null
candidate="$(mktemp "${TMPDIR:-/tmp}/timers-workspace-version.XXXXXX")"
trap 'rm -f -- "$candidate"' EXIT
perl "$helpers/render-cargo-workspace-version.pl" "$PWD/Cargo.toml" "$2" "$3" > "$candidate"
# Parse the rendered bytes independently before installation. Cargo admission
# above used the real workspace path; a scratch file cannot resolve its members.
"${YQ:-yq}" -p toml -o json '.' "$candidate" |
    jq -e --arg version "$3" '.workspace.package.version == $version' >/dev/null
cat "$candidate" > Cargo.toml
