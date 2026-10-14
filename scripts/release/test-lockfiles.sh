#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/release/check-lockfiles.sh"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-lockfile-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$temporary_root" "Failed lockfile fixture retained"' EXIT
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
version.workspace = true
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > crates/ic-timers/src/lib.rs
cat > testing/probe/Cargo.toml <<'EOF'
[package]
name = "probe"
version.workspace = true
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
if [[ "$*" == "${FIXTURE_FAIL_METADATA:-}" ]]; then
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
for produced_output in empty matching; do
    failure_status=0
    output="$(PATH="${temporary_root}/bin:${PATH}" \
        FIXTURE_FAIL_METADATA='metadata --locked --offline --format-version 1' FIXTURE_METADATA_OUTPUT="${produced_output}" \
        bash "${checker}" 2>&1)" || failure_status=$?
    if [[ "${failure_status}" != 101 || "${output}" != *'injected offline metadata failure'* ]]; then
        echo "error: metadata check lost Cargo failure for Cargo.toml: ${output}" >&2
        exit 1
    fi
    cmp Cargo.lock original-root.lock
done
rm bin/cargo original-root.lock

# A resolved mixed-version member must fail the coordinated graph contract.
cp testing/probe/Cargo.toml original-probe.toml
cp Cargo.lock original-root.lock
perl -pi -e 's/version.workspace = true/version = "0.0.0"/' testing/probe/Cargo.toml
cargo generate-lockfile --offline --quiet
if output="$(bash "${checker}" 2>&1)"; then
    echo 'error: graph admission accepted a resolved mixed-version probe' >&2; exit 1
fi
[[ "$output" == *'invalid coordinated workspace lock identities'* ]] || exit 1
mv original-probe.toml testing/probe/Cargo.toml
mv original-root.lock Cargo.lock

# A stale unpublished member is as invalid as a stale library record. Locked
# admission must inspect the complete graph even though the library is default.
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
    if [[ "${output}" != *'invalid coordinated workspace lock identities'* ]]; then
        echo "error: lockfile rejection did not report stale local identities: ${output}" >&2
        exit 1
    fi
    cmp Cargo.lock stale.lock
    mv original.lock Cargo.lock
done
bash "${checker}"

# A coherent lock is insufficient if the resolved crate differs from workspace truth.
perl -0pi -e 's/^version\.workspace = true$/version = "0.1.1"/m
    or die "fixture inherited library version missing\n";' crates/ic-timers/Cargo.toml
cargo generate-lockfile --offline --quiet
if output="$(bash "${checker}" 2>&1)"; then
    echo 'error: accepted a coherent lock for the wrong workspace package version' >&2
    exit 1
fi
if [[ "${output}" != *'invalid coordinated workspace lock identities'* ]]; then
    echo "error: unexpected package-version rejection: ${output}" >&2
    exit 1
fi
fixture_complete=true
echo 'Locked workspace metadata regression tests passed'
