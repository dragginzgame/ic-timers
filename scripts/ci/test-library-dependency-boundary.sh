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
    scripts/ci/check-library-dependency-boundary.sh
cp "$root/Cargo.toml" "$root/Cargo.lock" "$fixture/"
for package in crates/ic-timers testing/crates/ic-timers-{pocketic,runtime-probe,size-probe,test-protocol}; do
    mkdir -p "$fixture/$package/src"
    cp "$root/$package/Cargo.toml" "$fixture/$package/"
    touch "$fixture/$package/src/lib.rs" "$fixture/$package/src/main.rs"
done
printf 'fn main() {}\n' > "$fixture/crates/ic-timers/build.rs"
git init -q "$fixture"
cd "$fixture"
checker=scripts/ci/check-library-dependency-boundary.sh
bash "$checker"
cp crates/ic-timers/Cargo.toml library-original.toml
perl -pi -e 's/"executor", "ic0"/"executor", "ic0", "candid"/' crates/ic-timers/Cargo.toml
# An independently optional codec adds a dependency edge to Platform 0.2 even
# when another package already resolves Candid. Prepare only the fixture lock.
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
if bash "$checker" > feature-failure.log 2>&1; then
    echo 'error: library admitted accidental Platform Candid feature' >&2
    exit 1
fi
if ! grep -q 'forbidden Platform feature: candid' feature-failure.log; then
    cat feature-failure.log >&2
    exit 1
fi
cp library-original.toml crates/ic-timers/Cargo.toml
cp "$root/Cargo.lock" Cargo.lock

# A renamed target-specific build dependency must be checked by package name;
# the same package used only in dev-dependencies is outside this library graph.
# Lock preparation is confined to this disposable fixture, never the checkout.
cp Cargo.toml catalog-original.toml
perl -pi -e 's/^ic-wire = /leaked-wire = { package = "ic-wire", version = "0.4.4" }\nic-wire = /' Cargo.toml
for table in "target.'cfg(target_arch = \"wasm32\")'.dependencies" "target.'cfg(unix)'.build-dependencies"; do
    cp library-original.toml crates/ic-timers/Cargo.toml
    # Build-dependency cfgs describe the host; normal dependencies describe the
    # selected target. Exercise both with real Cargo rather than invented trees.
    printf '\n[%s]\nleaked-wire.workspace = true\n' "$table" >> crates/ic-timers/Cargo.toml
    RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
    if bash "$checker" > build-failure.log 2>&1; then
        echo "error: library admitted a renamed target-specific Wire dependency: $table" >&2
        exit 1
    fi
    grep -q 'forbidden production dependency: ic-wire' build-failure.log
done
cp library-original.toml crates/ic-timers/Cargo.toml
cp catalog-original.toml Cargo.toml
cat >> crates/ic-timers/Cargo.toml <<'EOF'

[dev-dependencies]
ic-wire.workspace = true
EOF
RUSTUP_AUTO_INSTALL=0 cargo tree --offline -p ic-timers --target wasm32-unknown-unknown > /dev/null
bash "$checker"
cp library-original.toml crates/ic-timers/Cargo.toml
cp "$root/Cargo.lock" Cargo.lock

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
grep -q 'expected one ic-cdk-executor 2.0.0' identity-failure.log
cp catalog-original.toml Cargo.toml
cp "$root/Cargo.lock" Cargo.lock

# A producer may emit an otherwise acceptable complete graph and then fail.
# Never treat those bytes as successful dependency evidence.
mkdir bin
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
"$BOUNDARY_REAL_CARGO" "$@" || exit
exit 37
EOF
chmod +x bin/cargo
BOUNDARY_REAL_CARGO="$(command -v cargo)"
export BOUNDARY_REAL_CARGO
if PATH="$fixture/bin:$PATH" bash "$checker" > producer-failure.log 2>&1; then
    echo 'error: library admitted failed Cargo evidence' >&2
    exit 1
fi
grep -q 'library Cargo tree failed' producer-failure.log
cmp "$root/Cargo.lock" Cargo.lock
fixture_complete=true
echo 'Library graph admission, accidental Candid and failed-producer checks passed'
