#!/usr/bin/env bash
# Shared companions: scripts/ci/check-library-dependency-boundary.sh scripts/ci/fixture-lifecycle.sh scripts/ci/prepare-script-fixture.sh
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/timer-library-boundary.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed library-boundary fixture retained"' EXIT

# Real Cargo resolution against the maintained catalog and lock. Empty fixture
# source is never built; no sibling source or network dependency is introduced.
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" \
    scripts/ci/check-library-dependency-boundary.sh scripts/ci/check-cargo-dependency-boundaries.sh
cp "$root/Cargo.toml" "$root/Cargo.lock" "$fixture/"
for package in crates/ic-timers testing/crates/ic-timers-{pocketic,runtime-probe,size-probe,test-protocol}; do
    mkdir -p "$fixture/$package/src"
    cp "$root/$package/Cargo.toml" "$fixture/$package/"
    touch "$fixture/$package/src/lib.rs" "$fixture/$package/src/main.rs"
done
printf 'fn main() {}\n' > "$fixture/crates/ic-timers/build.rs"
git init -q "$fixture"
cd "$fixture"
reset_manifests() {
    for manifest in Cargo.toml Cargo.lock crates/ic-timers/Cargo.toml \
        testing/crates/ic-timers-{pocketic,runtime-probe,size-probe,test-protocol}/Cargo.toml; do
        cp "$root/$manifest" "$manifest"
    done
}

checker=scripts/ci/check-library-dependency-boundary.sh
bash "$checker"
# The test-only simulator graph is admitted, but promoting it into the native
# build coordinator must fail even when the lock already contains every package.
perl -pi -e 's/^\[dependencies\]/[dependencies]\nic-testkit.workspace = true/' testing/crates/ic-timers-pocketic/Cargo.toml
if bash "$checker" > worker-failure.log 2>&1; then
    echo 'error: build coordinator admitted its test-only simulator dependency' >&2
    exit 1
fi
grep -q 'forbidden ic-testkit' worker-failure.log
reset_manifests
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "candid"/' crates/ic-timers/Cargo.toml
# An independently optional codec adds a dependency edge to Platform even
# when another package already resolves Candid. Prepare only the fixture lock.
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > feature-failure.log 2>&1; then
    echo 'error: library admitted accidental Platform Candid feature' >&2
    exit 1
fi
if ! grep -q 'forbidden candid' feature-failure.log; then
    cat feature-failure.log >&2
    exit 1
fi
reset_manifests

# Principal support belongs to application/probe profiles, never the raw timer
# provider. Refuse its transitive dependencies even when already in the lock.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "principal"/' crates/ic-timers/Cargo.toml
if bash "$checker" > principal-failure.log 2>&1; then
    echo 'error: library admitted Platform Principal support' >&2; exit 1
fi
grep -Eq 'forbidden (ic_principal|serde|serde_core|serde_derive|sha2|digest|crc32fast|data-encoding)' principal-failure.log
reset_manifests

# Runtime APIs belong only to entrypoint-owning probes; the published library
# must keep its executor/IC0-only boundary even when runtime adds no new macro.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "runtime"/' crates/ic-timers/Cargo.toml
if bash "$checker" > runtime-failure.log 2>&1; then
    echo 'error: library admitted Platform runtime APIs' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform/runtime' runtime-failure.log
reset_manifests

# Empty Platform defaults still violate the library feature allowlist, even
# without introducing another package. Probe and protocol policies are separate.
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "default"/' crates/ic-timers/Cargo.toml
if bash "$checker" > default-failure.log 2>&1; then
    echo 'error: library admitted Platform default features' >&2; exit 1
fi
grep -q 'forbidden dependency feature edge: ic-platform/default' default-failure.log
reset_manifests
perl -pi -e 's/\["cdk"\]/["cdk", "candid"]/' testing/crates/ic-timers-runtime-probe/Cargo.toml
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers-runtime-probe --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > probe-failure.log 2>&1; then
    echo 'error: canister probe admitted Candid' >&2; exit 1
fi
grep -q 'forbidden candid' probe-failure.log
reset_manifests
cat >> testing/crates/ic-timers-test-protocol/Cargo.toml <<'EOF'

[build-dependencies]
ic-platform = { workspace = true, features = ["executor"] }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers-test-protocol --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > protocol-failure.log 2>&1; then
    echo 'error: protocol admitted Platform authority' >&2; exit 1
fi
grep -q 'forbidden ic-platform' protocol-failure.log
reset_manifests

# A renamed target-specific build dependency must be checked by package name;
# the same package used only in dev-dependencies is outside this library graph.
# Lock preparation is confined to this disposable fixture, never the checkout.
# Pin the deliberate alias to the already prepared package, rather than resolving
# an independent old Wire graph that a clean offline CI cache cannot provide.
RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --format-version 1 > wire-alias-metadata.json
perl -MJSON::PP -0777 -i -pe '
    BEGIN {
        open my $input, "<", "wire-alias-metadata.json" or die "$!\n";
        my @wire = grep { $_->{name} eq "ic-wire" } @{decode_json(<$input>)->{packages}};
        close $input or die "$!\n";
        @wire == 1 or die "expected one prepared Wire package\n";
        $version = $wire[0]{version};
    }
    s/^ic-wire = /leaked-wire = { package = "ic-wire", version = "=$version" }\nic-wire = /m
        or die "missing workspace Wire declaration\n";
' Cargo.toml
for table in "target.'cfg(target_arch = \"wasm32\")'.dependencies" "target.'cfg(unix)'.build-dependencies"; do
    cp "$root/crates/ic-timers/Cargo.toml" crates/ic-timers/Cargo.toml
    # Build-dependency cfgs describe the host; normal dependencies describe the
    # selected target. Exercise both with real Cargo rather than invented trees.
    printf '\n[%s]\nleaked-wire.workspace = true\n' "$table" >> crates/ic-timers/Cargo.toml
    RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
    if bash "$checker" > build-failure.log 2>&1; then
        echo "error: library admitted a renamed target-specific Wire dependency: $table" >&2
        exit 1
    fi
    grep -q 'forbidden pattern matches ic-wire' build-failure.log
done
reset_manifests
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[dev-dependencies]
ic-wire.workspace = true
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
bash "$checker"
reset_manifests

# The common direct guard composes with Timers' stricter deployment policy.
# Principal is intentionally absent from the local declaration list: it must
# still reject an optional renamed target/build edge through the common owner.
direct_checker=scripts/ci/check-cargo-dependency-boundaries.sh
bash "$direct_checker" --consumer "$fixture" --external-ic
perl -pi -e 's/^ic-platform = /external-principal = { package = "ic_principal", version = "0.1" }\nic-platform = /' Cargo.toml
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[target.'cfg(target_arch = "wasm32")'.build-dependencies]
external-principal = { workspace = true, optional = true }
EOF
RUSTUP_AUTO_INSTALL=0 cargo metadata --offline --format-version 1 > /dev/null
bash "$direct_checker" --consumer "$fixture"
if bash "$direct_checker" --consumer "$fixture" --external-ic > direct-failure.log 2>&1; then
    echo 'error: common IC boundary admitted a renamed optional target/build dependency' >&2
    exit 1
fi
grep -q 'ic-timers -> ic_principal' direct-failure.log
reset_manifests

# A source override at the same version is not the qualified executor identity.
mkdir -p executor-substitute/src
cat > executor-substitute/Cargo.toml <<'EOF'
[package]
name = "ic-cdk-executor"
version = "2.0.0"
edition = "2024"

[workspace]
EOF
touch executor-substitute/src/lib.rs
cat >> Cargo.toml <<'EOF'

[patch.crates-io]
ic-cdk-executor = { path = "executor-substitute" }
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > identity-failure.log 2>&1; then
    echo 'error: library admitted an unqualified executor source at the same version' >&2
    exit 1
fi
grep -q 'expected one qualified registry ic-cdk-executor 2.0.0' identity-failure.log
reset_manifests

cmp "$root/Cargo.lock" Cargo.lock
fixture_complete=true
echo 'Library, probe, protocol and build coordinator dependency policy checks passed'
