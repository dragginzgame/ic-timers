#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
source "${repository_root}/tool-versions.env"
bash "${repository_root}/.shared-tooling/helpers/scripts/ci/check-format-tools.sh" \
    "${IC_TIMERS_CARGO_SORT_VERSION}"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
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
cp "${repository_root}/tool-versions.env" tool-versions.env
# The current fmt prerequisite must also exist in the fixture's exact index.
cp -p "${repository_root}/.shared-tooling/helpers/scripts/ci/check-format-tools.sh" \
    .shared-tooling/helpers/scripts/ci/
mkdir -p src testing/src
for workspace in . testing; do
    cat > "${workspace}/Cargo.toml" <<'EOF'
[workspace]
members = []

[package]
name = "hook-fixture"
version = "0.0.0"
edition = "2024"
EOF
    printf 'pub fn fixture( ){}\n' > "${workspace}/src/lib.rs"
done
git add Makefile tool-versions.env Cargo.toml src/lib.rs testing/Cargo.toml testing/src/lib.rs \
    .shared-tooling/helpers/scripts/ci/check-format-tools.sh
printf 'unrelated working edit\n' >> README.md
cp README.md "${temporary_root}/unrelated-readme"

working_files=(Cargo.toml testing/Cargo.toml src/lib.rs testing/src/lib.rs README.md)
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

# The actual consumer fmt target formats and refreshes both workspace selections,
# preserving unrelated edits and requiring no dependencies, builds or network.
CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 bash "${repository_root}/.githooks/pre-commit"
for path in src/lib.rs testing/src/lib.rs; do
    formatted="$(git show ":${path}")"
    test "${formatted}" = 'pub fn fixture() {}'
done
cmp "${temporary_root}/unrelated-readme" README.md
make --no-print-directory fmt-check

# Partial staging must reject before formatting or refreshing the real index.
printf 'pub fn fixture() {}\n// unstaged edit\n' > testing/src/lib.rs
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/output" 2>&1; then
    echo 'error: commit hook accepted partially staged nested Rust' >&2
    exit 1
fi
assert_unchanged
git checkout-index -f -- testing/src/lib.rs

# A failed consumer formatter must not copy or stage even its earlier changes.
printf 'pub fn fixture( ){}\n' > testing/src/lib.rs
git add testing/src/lib.rs
printf 'fmt:\n\t@printf "pub fn fixture() {}\\n" > testing/src/lib.rs\n\t@exit 1\n' > Makefile
git add Makefile
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/output" 2>&1; then
    echo 'error: commit hook accepted failed snapshot formatting' >&2
    exit 1
fi
assert_unchanged
echo 'Consumer hook auto-formatting, nested selection and failure-isolation checks passed'
