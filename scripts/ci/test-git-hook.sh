#!/usr/bin/env bash
set -euo pipefail

# The fixture owns its Make selections; negative cases set them explicitly.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
source "${repository_root}/ci/tool-versions.env"
bash "${repository_root}/scripts/ci/check-format-tools.sh" \
    "${SHARED_TOOLING_CARGO_SORT_VERSION}"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-hook-test.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf -- "${temporary_root}"; else printf "Failed hook fixture retained: %s\n" "${temporary_root}" >&2; fi' EXIT
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
mkdir -p make
cp "${repository_root}/make/tools.mk" "${repository_root}/make/rust-format.mk" \
    "${repository_root}/make/release.mk" "${repository_root}/make/execution.mk" make/
mkdir -p ci
cp "${repository_root}/ci/tool-versions.env" ci/
# The current fmt prerequisite must also exist in the fixture's exact index.
cp -p "${repository_root}/scripts/ci/check-format-tools.sh" \
    scripts/ci/
cp -p "${repository_root}/scripts/ci/check-make-execution.sh" scripts/ci/
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
git add Makefile make/tools.mk make/rust-format.mk make/release.mk make/execution.mk ci/tool-versions.env Cargo.toml \
    crates/hook-fixture testing/crates/hook-probe \
    scripts/ci/check-format-tools.sh scripts/ci/check-make-execution.sh
printf 'unrelated working edit\n' >> README.md
cp README.md "${temporary_root}/unrelated-readme"

working_files=(Cargo.toml crates/hook-fixture/Cargo.toml testing/crates/hook-probe/Cargo.toml
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

# A rejected mode must not format working files or refresh the real index.
capture_before_hook
# GNU Make 3.81 on macOS ignores GNUMAKEFLAGS; 4.0 introduced it.
mode_variables=(MAKEFLAGS)
make_version="$(make --version)"
case "$make_version" in 'GNU Make 3.'*) ;; *) mode_variables+=(GNUMAKEFLAGS) ;; esac
for variable in "${mode_variables[@]}"; do
    for flags in i n q t v --ignore-errors --dry-run --question --touch --version; do
        mode_output="${temporary_root}/${variable}-${flags}.log"
        if env "$variable=$flags" bash "${repository_root}/.githooks/pre-commit" \
            > "${mode_output}" 2>&1; then
            cat "${mode_output}" >&2
            echo "error: commit hook accepted $variable=$flags" >&2
            exit 1
        fi
        grep -Fq 'requires recipe execution and failure propagation' "${mode_output}"
        assert_unchanged
    done
done

# The actual consumer Makefile must refuse before its formatter prerequisites,
# even if Make would ignore a recipe failure or skip execution.
for mode in --ignore-errors --dry-run --touch --question; do
    status=0
    SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
        make --no-print-directory "$mode" fmt-check \
        "SHARED_TOOLING_ROOT=${temporary_root}/unselected-snapshot" \
        > "${temporary_root}/make-${mode}.log" 2>&1 || status=$?
    if [[ "$status" != 2 ]]; then
        cat "${temporary_root}/make-${mode}.log" >&2
        echo "error: consumer formatting did not refuse $mode before recipes" >&2
        exit 1
    fi
    assert_unchanged
done

# The actual consumer fmt target formats and refreshes both root workspace members,
# preserving unrelated edits and requiring no dependencies, builds or network.
if ! CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 \
    SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" \
    bash "${repository_root}/.githooks/pre-commit" \
    > "${temporary_root}/format.log" 2>&1; then
    cat "${temporary_root}/format.log" >&2
    exit 1
fi
cat "${temporary_root}/format.log"
for path in crates/hook-fixture/src/lib.rs testing/crates/hook-probe/src/lib.rs; do
    formatted="$(git show ":${path}")"
    test "${formatted}" = 'pub fn fixture() {}'
done
cmp "${temporary_root}/unrelated-readme" README.md
if ! SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" make --no-print-directory \
    fmt-check "SHARED_TOOLING_ROOT=${temporary_root}/unselected-snapshot" \
    "HOST_TOOL_VERSIONS=${temporary_root}/unselected-pins.env" \
    > "${temporary_root}/fmt-check.log" 2>&1; then
    cat "${temporary_root}/fmt-check.log" >&2
    exit 1
fi
cat "${temporary_root}/fmt-check.log"

# Partial staging must reject before formatting or refreshing the real index.
printf 'pub fn fixture() {}\n// unstaged edit\n' > testing/crates/hook-probe/src/lib.rs
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/partial-staging.log" 2>&1; then
    echo 'error: commit hook accepted partially staged probe Rust' >&2
    exit 1
fi
assert_unchanged
git checkout-index -f -- testing/crates/hook-probe/src/lib.rs

# A failed consumer formatter must not copy or stage even its earlier changes.
printf 'pub fn fixture( ){}\n' > testing/crates/hook-probe/src/lib.rs
git add testing/crates/hook-probe/src/lib.rs
printf 'fmt:\n\t@printf "pub fn fixture() {}\\n" > testing/crates/hook-probe/src/lib.rs\n\t@exit 1\n' > Makefile
git add Makefile
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/failed-formatter.log" 2>&1; then
    echo 'error: commit hook accepted failed snapshot formatting' >&2
    exit 1
fi
assert_unchanged
echo 'Consumer hook auto-formatting, nested selection and failure-isolation checks passed'
