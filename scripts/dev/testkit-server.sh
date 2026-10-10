#!/usr/bin/env bash
set -euo pipefail

[[ $# == 1 && ( "$1" == setup || "$1" == check ) ]] || {
    echo 'usage: testkit-server.sh setup|check' >&2
    exit 2
}
mode="$1"
root="$0"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
export PATH="$root/.tools/host/bin:$PATH"
export RUSTUP_AUTO_INSTALL=0
[[ "$mode" != check ]] || export CARGO_NET_OFFLINE=true

# The root lock selects both the test dependency and its published owner CLI.
# Admission and receipts belong to the shared installer, server pins to Testkit.
version="$(yq -p toml -o json '.' "$root/Cargo.lock" | jq -er '
    [.package[] | select(.name == "ic-testkit")] |
    if length == 1 and .[0].source == "registry+https://github.com/rust-lang/crates.io-index"
    then .[0].version else error("require one registry ic-testkit selection") end
')"
arguments=(--consumer "$root" --package ic-testkit --version "$version"
    --bin ic-testkit-server --profile release)
[[ "$mode" != check ]] || arguments+=(--check)
if executable="$(bash "$root/scripts/dev/install-rust-tools.sh" "${arguments[@]}")"; then
    :
else
    status=$?
    echo 'Testkit CLI preparation/admission failed; run make install-testkit-server' >&2
    exit "$status"
fi
"$executable" "$mode" --directory "$root/.tools/testkit-server"
