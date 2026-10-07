#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/release/check-lockfiles.sh"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-lockfile-test.XXXXXX")"
trap 'status=$?; if [[ "$status" == 0 ]]; then rm -rf -- "${temporary_root}";
    else printf "Failed lockfile fixture retained: %s\n" "${temporary_root}" >&2;
    fi; exit "$status"' EXIT
mkdir -p "${temporary_root}"/{crates/ic-timers/src,testing/probe/src}
cd "${temporary_root}"
cat > Cargo.toml <<'EOF'
[workspace]
members = ["crates/ic-timers", "testing/probe"]
resolver = "3"
[workspace.dependencies]
ic-timers = { path = "crates/ic-timers" }
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
cat > testing/probe/Cargo.toml <<'EOF'
[package]
name = "probe"
version = "0.0.0"
edition = "2024"
[dependencies]
ic-timers.workspace = true
EOF
printf '%s\n' 'pub fn fixture() {}' > testing/probe/src/lib.rs
cargo generate-lockfile --offline --quiet
bash "${checker}"

# Preserve Cargo's failure status before parsing empty or plausible JSON output.
mkdir -p bin
IC_TIMERS_FIXTURE_CARGO="$(command -v cargo)"
export IC_TIMERS_FIXTURE_CARGO
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == metadata && "${3:-}" == "${FIXTURE_FAIL_METADATA:-}" ]]; then
    if [[ "${FIXTURE_METADATA_OUTPUT:-}" == matching ]]; then
        printf '%s\n' '{"packages":[{"name":"ic-timers","version":"0.1.0"}]}'
    fi
    echo 'injected offline metadata failure' >&2
    exit 101
fi
exec "${IC_TIMERS_FIXTURE_CARGO}" "$@"
EOF
chmod +x bin/cargo
cp Cargo.lock original-root.lock
for failed_manifest in Cargo.toml; do
    for produced_output in empty matching; do
        failure_status=0
        output="$(PATH="${temporary_root}/bin:${PATH}" \
            FIXTURE_FAIL_METADATA="${failed_manifest}" FIXTURE_METADATA_OUTPUT="${produced_output}" \
            bash "${checker}" 2>&1)" || failure_status=$?
        if [[ "${failure_status}" != 101 || "${output}" != *'injected offline metadata failure'* ]]; then
            echo "error: metadata check lost Cargo failure for ${failed_manifest}: ${output}" >&2
            exit 1
        fi
        cmp Cargo.lock original-root.lock
    done
done
rm bin/cargo original-root.lock

# A stale unpublished member is as invalid as a stale library record. Locked
# metadata must inspect the complete graph even though the library is default.
for package in ic-timers probe; do
    cp Cargo.lock original.lock
    IC_TIMERS_FIXTURE_PACKAGE="${package}" perl -0pi -e '
        my $name = $ENV{IC_TIMERS_FIXTURE_PACKAGE};
        s/(\[\[package\]\]\nname = "\Q$name\E"\nversion = ")[^"]+("\n)/${1}999.0.0$2/
            or die "fixture package missing: $name\n";
    ' Cargo.lock
    cp Cargo.lock stale.lock
    if output="$(bash "${checker}" 2>&1)"; then
        echo "error: accepted stale ${package} version in the root lockfile" >&2
        exit 1
    fi
    if [[ "${output}" != *'--locked'* ]]; then
        echo "error: lockfile rejection did not report locked resolution: ${output}" >&2
        exit 1
    fi
    cmp Cargo.lock stale.lock
    mv original.lock Cargo.lock
done
bash "${checker}"

# A coherent lock is insufficient if the resolved crate differs from workspace truth.
perl -pi -e 's/^version = "0\.1\.0"$/version = "0.1.1"/' crates/ic-timers/Cargo.toml
cargo generate-lockfile --offline --quiet
if output="$(bash "${checker}" 2>&1)"; then
    echo 'error: accepted a coherent lock for the wrong workspace package version' >&2
    exit 1
fi
if [[ "${output}" != *'workspace.package version 0.1.0'* ]]; then
    echo "error: unexpected package-version rejection: ${output}" >&2
    exit 1
fi
echo 'Locked workspace metadata regression tests passed'
