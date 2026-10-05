#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir "${temporary_root}/repo"
git init -q "${temporary_root}/repo"
cd "${temporary_root}/repo"
git config user.name 'ic-timers hook test'
git config user.email 'hook-test@example.invalid'
# An isolated formatter stand-in checks only the files copied from the index.
printf '%s\n' 'fmt-check:' $'\t@cmp -s staged.rs expected.rs' $'\t@cmp -s unrelated.rs original.rs' > Makefile
printf '%s\n' 'formatted' > expected.rs
cp expected.rs original.rs
cp expected.rs staged.rs
cp expected.rs unrelated.rs
git add Makefile expected.rs original.rs staged.rs unrelated.rs
git commit -qm fixture

working_files=(staged.rs unrelated.rs)
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

# A partially staged file and unrelated dirty file must remain byte-for-byte
# unchanged; formatting is validated against their staged versions.
printf '%s\n' 'formatted staged edit' > expected.rs
cp expected.rs staged.rs
git add expected.rs staged.rs
printf '%s\n' 'unformatted working edit' > staged.rs
printf '%s\n' 'unrelated dirty edit' > unrelated.rs
capture_before_hook
bash "${repository_root}/.githooks/pre-commit"
assert_unchanged

# A formatted working copy cannot conceal an unformatted staged version.
printf '%s\n' 'unformatted staged edit' > staged.rs
git add staged.rs
cp expected.rs staged.rs
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/output" 2>&1; then
    echo 'error: commit hook accepted unformatted staged content' >&2
    exit 1
fi
assert_unchanged

# The production formatting target must also reject an unformatted nested
# workspace in the index while preserving formatted or dirty working copies.
mkdir -p "${temporary_root}/workspaces"/{src,testing/src}
cd "${temporary_root}/workspaces"
git init -q
git config user.name 'ic-timers hook test'
git config user.email 'hook-test@example.invalid'
cp "${repository_root}/Makefile" Makefile
cat > Cargo.toml <<'EOF'
[workspace]

[package]
name = "hook-root-fixture"
version = "0.0.0"
edition = "2024"
EOF
cat > testing/Cargo.toml <<'EOF'
[workspace]

[package]
name = "hook-nested-fixture"
version = "0.0.0"
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > src/lib.rs
cp src/lib.rs testing/src/lib.rs
git add Makefile Cargo.toml src/lib.rs testing/Cargo.toml testing/src/lib.rs
git -c core.hooksPath=/dev/null commit -qm 'two-workspace fixture'

printf '%s\n' 'pub fn fixture( ){}' > testing/src/lib.rs
git add testing/src/lib.rs
printf '%s\n' 'pub fn fixture() {}' > testing/src/lib.rs
printf '%s\n' 'pub fn fixture( ) { }' > src/lib.rs
working_files=(src/lib.rs testing/src/lib.rs)
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/output" 2>&1; then
    echo 'error: commit hook accepted unformatted staged nested Rust' >&2
    exit 1
fi
if [[ "$(cat "${temporary_root}/output")" != *'testing/src/lib.rs'* ]]; then
    cat "${temporary_root}/output" >&2
    echo 'error: commit hook did not reach the nested formatting check' >&2
    exit 1
fi
assert_unchanged

git add testing/src/lib.rs
capture_before_hook
bash "${repository_root}/.githooks/pre-commit"
assert_unchanged
echo 'Commit hook snapshot checks passed'
