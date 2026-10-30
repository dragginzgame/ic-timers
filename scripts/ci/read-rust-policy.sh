#!/usr/bin/env bash
# Shared companions: scripts/ci/read-cargo-workspace-rust-version.sh
set -euo pipefail

[[ $# == 0 ]] || { echo 'usage: read-rust-policy.sh' >&2; exit 2; }
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
export PATH="$root/.tools/host/bin:$PATH"

# Shared reads the workspace floor. The toolchain file owns the development
# channel; jq/yq are explicitly prepared before this read-only CI projection.
minimum="$(bash "$root/scripts/ci/read-cargo-workspace-rust-version.sh" "$root/Cargo.toml")"
document="$(yq -p toml -o json '.' "$root/rust-toolchain.toml")"
development="$(printf '%s' "$document" | jq -er '
    .toolchain.channel | select(type == "string" and test("^[A-Za-z0-9][A-Za-z0-9_.+-]*$"))
')"
# Admit both observations before publishing either environment assignment.
printf 'IC_TIMERS_MSRV=%s\nIC_TIMERS_INTERNAL_TOOLCHAIN=%s\n' "$minimum" "$development"
