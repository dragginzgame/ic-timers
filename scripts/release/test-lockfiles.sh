#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
checker="${repository_root}/scripts/release/check-lockfiles.sh"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir -p "${temporary_root}"/{crates/ic-timers/src,testing/probe/src}
cd "${temporary_root}"
cat > Cargo.toml <<'EOF'
[workspace]
members = ["crates/ic-timers"]
resolver = "3"
[workspace.package]
version = "0.1.0"
EOF
cat > crates/ic-timers/Cargo.toml <<'EOF'
[package]
name = "ic-timers"
version = "0.1.0"
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > crates/ic-timers/src/lib.rs
cat > testing/Cargo.toml <<'EOF'
[workspace]
members = ["probe"]
resolver = "3"
EOF
cat > testing/probe/Cargo.toml <<'EOF'
[package]
name = "probe"
version = "0.0.0"
edition = "2024"
[dependencies]
ic-timers = { path = "../../crates/ic-timers" }
EOF
printf '%s\n' 'pub fn fixture() {}' > testing/probe/src/lib.rs
cargo generate-lockfile --offline --quiet
cargo generate-lockfile --manifest-path testing/Cargo.toml --offline --quiet
bash "${checker}"

for lockfile in Cargo.lock testing/Cargo.lock; do
    cp "${lockfile}" original.lock
    sed -i 's/^version = "0.1.0"$/version = "0.0.0"/' "${lockfile}"
    cp "${lockfile}" stale.lock
    if output="$(bash "${checker}" 2>&1)"; then
        echo "error: accepted stale path-package version in ${lockfile}" >&2
        exit 1
    fi
    if [[ "${output}" != *'--locked'* ]]; then
        echo "error: lockfile rejection did not report locked resolution: ${output}" >&2
        exit 1
    fi
    cmp "${lockfile}" stale.lock
    mv original.lock "${lockfile}"
done
bash "${checker}"

# Coherent locks are insufficient if the resolved crate differs from workspace truth.
sed -i 's/^version = "0.1.0"$/version = "0.1.1"/' crates/ic-timers/Cargo.toml
cargo generate-lockfile --offline --quiet
cargo generate-lockfile --manifest-path testing/Cargo.toml --offline --quiet
if output="$(bash "${checker}" 2>&1)"; then
    echo 'error: accepted coherent locks for the wrong workspace package version' >&2
    exit 1
fi
if [[ "${output}" != *'workspace.package version 0.1.0'* ]]; then
    echo "error: unexpected package-version rejection: ${output}" >&2
    exit 1
fi
echo 'Locked workspace metadata regression tests passed'
