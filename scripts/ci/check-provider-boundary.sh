#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

expected_source="crates/ic-timers/src/platform.rs"
provider_sources="$(
    grep -RFl --include='*.rs' -- 'ic_cdk_timers' crates/ic-timers/src \
        | LC_ALL=C sort \
        || true
)"
if [[ "${provider_sources}" != "${expected_source}" ]]; then
    echo "error: direct ic-cdk-timers use must remain solely in ${expected_source}" >&2
    if [[ -n "${provider_sources}" ]]; then
        echo "found:" >&2
        echo "${provider_sources}" >&2
    fi
    exit 1
fi

if ! grep -Fqx -- 'mod platform;' crates/ic-timers/src/lib.rs >/dev/null; then
    echo "error: the provider boundary must remain a private module" >&2
    exit 1
fi

if grep -RE --include='*.rs' \
    'pub[[:space:]]+(use|extern[[:space:]]+crate|mod)[[:space:]]+.*ic_cdk_timers|pub[[:space:]]+use[[:space:]]+platform' \
    crates/ic-timers/src >/dev/null; then
    echo "error: ic-cdk-timers or the platform boundary must not be re-exported" >&2
    exit 1
fi

echo "Provider boundary checks passed"
