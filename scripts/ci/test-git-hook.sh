#!/usr/bin/env bash
set -euo pipefail

# The fixture owns its Make selections; negative cases set them explicitly.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
source "${repository_root}/ci/tool-versions.env"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
# shellcheck source=scripts/ci/make-adapter-probes.sh
source "$repository_root/scripts/ci/make-adapter-probes.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-hook-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed hook fixture retained"' EXIT
bash "${repository_root}/scripts/ci/check-format-tools.sh" \
    "${SHARED_TOOLING_CARGO_SORT_VERSION}"
# Reuse committed objects read-only; the fixture never creates a commit.
source_commit="$(git rev-parse HEAD)"
source_objects="$(git rev-parse --git-path objects)"
case "${source_objects}" in /*) ;; *) source_objects="${repository_root}/${source_objects}" ;; esac
mkdir "${temporary_root}/repo"
git init -q "${temporary_root}/repo"
cd "${temporary_root}/repo"
mkdir -p .git/objects/info
printf '%s\n' "${source_objects}" > .git/objects/info/alternates
git update-ref HEAD "${source_commit}"
git read-tree HEAD
git checkout-index --all
cp "${repository_root}/Makefile" Makefile
bash "$repository_root/scripts/ci/prepare-make-fixture.sh" "$repository_root" "$temporary_root/repo"
mkdir -p ci
cp "${repository_root}/ci/tool-versions.env" ci/
# The current fmt prerequisite must also exist in the fixture's exact index.
cp -p "${repository_root}/scripts/ci/check-format-tools.sh" \
    scripts/ci/
# The fixture overlays both members into the one root-owned workspace.
mkdir -p crates/hook-fixture/src testing/crates/hook-probe/src
cat > Cargo.toml <<'EOF'
[workspace]
members = ["crates/hook-fixture", "testing/crates/hook-probe"]
resolver = "3"
[workspace.package]
version = "0.0.0"
edition = "2024"
EOF
for member in crates/hook-fixture testing/crates/hook-probe; do
    cat > "${member}/Cargo.toml" <<EOF
[package]
name = "$(basename "$member")"
version.workspace = true
edition.workspace = true
EOF
    printf 'pub fn fixture( ){}\n' > "${member}/src/lib.rs"
done
git add Makefile make/ ci/tool-versions.env Cargo.toml \
    crates/hook-fixture testing/crates/hook-probe \
    scripts/ci/check-format-tools.sh scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh
printf 'unrelated working edit\n' >> README.md
cp README.md "${temporary_root}/unrelated-readme"
# Preserve distinct index and working lock bytes across selected-file formatting.
git show :Cargo.lock > "${temporary_root}/index-lock"
printf '\n# unrelated working lock edit\n' >> Cargo.lock
cp Cargo.lock "${temporary_root}/working-lock"

working_files=(Makefile Cargo.toml Cargo.lock crates/hook-fixture/Cargo.toml testing/crates/hook-probe/Cargo.toml
    crates/hook-fixture/src/lib.rs testing/crates/hook-probe/src/lib.rs README.md)
capture_before_hook() {
    local path
    for path in "${working_files[@]}"; do
        mkdir -p "${temporary_root}/before-files/$(dirname "${path}")"
        cp "${path}" "${temporary_root}/before-files/${path}"
    done
    git diff --cached --binary > "${temporary_root}/before-index"
}
assert_unchanged() {
    local path
    for path in "${working_files[@]}"; do
        cmp "${path}" "${temporary_root}/before-files/${path}"
    done
    git diff --cached --binary > "${temporary_root}/after-index"
    cmp "${temporary_root}/before-index" "${temporary_root}/after-index"
}

# One rejected mode proves the actual hook delegates before formatting or staging.
# Shared owns flag aliases and environment-variable permutations.
capture_before_hook
if MAKEFLAGS=n bash "${repository_root}/.githooks/pre-commit" \
    > "${temporary_root}/hook-admission.log" 2>&1; then
    cat "${temporary_root}/hook-admission.log" >&2
    echo 'error: commit hook accepted MAKEFLAGS=n' >&2
    exit 1
fi
grep -Fq 'requires recipe execution and failure propagation' "${temporary_root}/hook-admission.log"
assert_unchanged

# Shared owns the flag matrix; qualify our actual formatting targets and refusal
# before formatter dispatch, even when ambient routing names another snapshot.
mkdir "$temporary_root/format-admission"
export TIMERS_FORMAT_ADMISSION_EVENTS="$temporary_root/format-admission/events"
cat > "$temporary_root/format-admission.mk" <<'MAKE'
include Makefile
format-tools-check:
	@printf 'format-tools-check\n' >> "$$TIMERS_FORMAT_ADMISSION_EVENTS"
MAKE
SHARED_TOOLING_ROOT="$temporary_root/unselected-snapshot" \
    make_probe_target_admission "$(command -v make)" "$temporary_root/format-admission.mk" \
        "$temporary_root/format-admission" "$TIMERS_FORMAT_ADMISSION_EVENTS" fmt fmt-check || exit 1
assert_unchanged

# Shared qualifies generic tree-read failures and snapshot formatter rollback.
# This fixture owns the actual Timers Make integration and selected workspace.
# The actual consumer fmt target formats and refreshes both root workspace members,
# preserving unrelated edits and requiring no dependencies, builds or network.
if ! CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 \
    SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" \
    RUST_TOOL_VERSIONS="${temporary_root}/unselected-rust-pins.env" \
    bash "${repository_root}/.githooks/pre-commit" \
    > "${temporary_root}/format.log" 2>&1; then
    cat "${temporary_root}/format.log" >&2
    exit 1
fi
# The formatter owns one success line; the hook separately confirms index refresh.
printf '%s\n' 'Formatting... ok' \
    'Pre-commit formatting passed; refreshed only the selected files.' \
    > "${temporary_root}/expected-format-output"
cmp "${temporary_root}/expected-format-output" "${temporary_root}/format.log"
for path in crates/hook-fixture/src/lib.rs testing/crates/hook-probe/src/lib.rs; do
    formatted="$(git show ":${path}")"
    test "${formatted}" = 'pub fn fixture() {}'
done
cmp "${temporary_root}/unrelated-readme" README.md
cmp "${temporary_root}/working-lock" Cargo.lock
git show :Cargo.lock > "${temporary_root}/after-hook-index-lock"
cmp "${temporary_root}/index-lock" "${temporary_root}/after-hook-index-lock"
if ! SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" \
    RUST_TOOL_VERSIONS="${temporary_root}/unselected-rust-pins.env" make --no-print-directory \
    fmt-check "SHARED_TOOLING_ROOT=${temporary_root}/unselected-snapshot" \
    "HOST_TOOL_VERSIONS=${temporary_root}/unselected-pins.env" \
    "RUST_TOOL_VERSIONS=${temporary_root}/unselected-rust-pins.env" \
    > "${temporary_root}/fmt-check.log" 2>&1; then
    cat "${temporary_root}/fmt-check.log" >&2
    exit 1
fi
printf 'Checking formatting... ok\n' > "${temporary_root}/expected-check-output"
cmp "${temporary_root}/expected-check-output" "${temporary_root}/fmt-check.log"
cmp "${temporary_root}/working-lock" Cargo.lock
git show :Cargo.lock > "${temporary_root}/after-check-index-lock"
cmp "${temporary_root}/index-lock" "${temporary_root}/after-check-index-lock"

# Use the actual consumer recipe and reporter with a sorter that fails after
# both output streams. Rustfmt must not run, and no working/index input changes.
mkdir "${temporary_root}/format-logs"
cat > "${temporary_root}/failing-cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    'sort --version') echo "cargo-sort $SHARED_TOOLING_CARGO_SORT_VERSION"; exit 0 ;;
    'fmt --version') exit 0 ;;
    sort*) echo 'sorter stdout details'; echo 'sorter stderr details' >&2; exit 23 ;;
    *) echo 'unexpected rustfmt execution' > "$FORMAT_TEST_EVENTS"; exit 99 ;;
esac
CARGO
chmod +x "${temporary_root}/failing-cargo"
printf 'sorter stdout details\nsorter stderr details\n' > "${temporary_root}/expected-diagnostics"
capture_before_hook
for target in fmt fmt-check; do
    status=0
    RUNNER_TEMP="${temporary_root}/format-logs" \
        SHARED_TOOLING_CARGO_SORT_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION" \
        FORMAT_TEST_EVENTS="${temporary_root}/unexpected-format-events" \
        make --no-print-directory "$target" "FORMAT_CARGO=${temporary_root}/failing-cargo" \
        > "${temporary_root}/$target-failure.log" 2>&1 || status=$?
    test "$status" -eq 2
    label=Formatting
    [[ "$target" != fmt-check ]] || label='Checking formatting'
    grep -Fxq "$label... FAILED (exit 23)" "${temporary_root}/$target-failure.log"
    logs=("${temporary_root}"/format-logs/formatting.*)
    test "${#logs[@]}" -eq 1
    printf 'Details: %q\n' "${logs[0]}" > "${temporary_root}/expected-log-path"
    grep -Fxf "${temporary_root}/expected-log-path" "${temporary_root}/$target-failure.log"
    cmp "${temporary_root}/expected-diagnostics" "${logs[0]}"
    test ! -e "${temporary_root}/unexpected-format-events"
    assert_unchanged
    mv "${logs[0]}" "${temporary_root}/$target-diagnostics.log"
done

# Partial staging must reject before formatting or refreshing the real index.
printf 'pub fn fixture() {}\n// unstaged edit\n' > testing/crates/hook-probe/src/lib.rs
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/partial-staging.log" 2>&1; then
    echo 'error: commit hook accepted partially staged probe Rust' >&2
    exit 1
fi
assert_unchanged
git checkout-index -f -- testing/crates/hook-probe/src/lib.rs

fixture_complete=true
echo 'Consumer hook auto-formatting, nested selection and failure-isolation checks passed'
