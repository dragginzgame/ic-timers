#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

expected_source="crates/ic-timers/src/platform.rs"
if ! provider_sources="$(
    grep -RFl --include='*.rs' -- 'ic_cdk_timers' crates/ic-timers/src \
        | LC_ALL=C sort
)"; then
    echo 'error: cannot inspect direct ic-cdk-timers use' >&2
    exit 1
fi
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

if ! grep -Fqx -- '#![forbid(private_interfaces)]' crates/ic-timers/src/lib.rs >/dev/null; then
    echo "error: private platform types must not leak through public interfaces" >&2
    exit 1
fi

# Rust owns alias resolution and export visibility. Keep platform declarations
# crate-visible and forbid private-interface leaks even under local lint allows.
# shellcheck disable=SC2016
perl -0777 -ne '
    if (/\bpub\s+(?:(?:async|unsafe)\s+)*(?:struct|enum|type|fn|use|mod|trait|const|static|extern)\b/) {
        die "error: platform items must have restricted visibility in $ARGV\n";
    }
' "${expected_source}"

echo "Provider boundary checks passed"
