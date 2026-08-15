#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
classifier="${repository_root}/scripts/release/classify-release-impact.sh"
temporary_root="$(mktemp -d)"
cleanup() {
    rm -rf -- "${temporary_root}"
}
trap cleanup EXIT

git init -q "${temporary_root}"
mkdir -p \
    "${temporary_root}/crates/ic-timers/src" \
    "${temporary_root}/docs"
printf '%s\n' \
    '[workspace]' \
    'members = ["crates/ic-timers"]' \
    '' \
    '[workspace.package]' \
    'version = "0.3.8"' \
    > "${temporary_root}/Cargo.toml"
printf '%s\n' \
    '[package]' \
    'name = "ic-timers"' \
    'version.workspace = true' \
    > "${temporary_root}/crates/ic-timers/Cargo.toml"
printf '%s\n' 'pub fn fixture() {}' \
    > "${temporary_root}/crates/ic-timers/src/lib.rs"
git -C "${temporary_root}" add .
git -C "${temporary_root}" \
    -c user.name='ic-timers release test' \
    -c user.email='release-test@example.invalid' \
    commit -qm 'fixture'
git -C "${temporary_root}" tag v0.3.8

classify() {
    (
        cd "${temporary_root}"
        bash "${classifier}" v0.3.8
    )
}

if [[ "$(classify)" != "none" ]]; then
    echo "error: unchanged release subject was not classified as none" >&2
    exit 1
fi

printf '%s\n' 'Repository evidence.' > "${temporary_root}/docs/evidence.md"
if [[ "$(classify)" != "repository" ]]; then
    echo "error: documentation-only work was not classified as repository" >&2
    exit 1
fi

printf '%s\n' 'pub fn added() {}' \
    > "${temporary_root}/crates/ic-timers/src/added.rs"
if [[ "$(classify)" != "crate" ]]; then
    echo "error: untracked crate source was not classified as crate" >&2
    exit 1
fi
rm -f -- "${temporary_root}/crates/ic-timers/src/added.rs"

printf '%s\n' '# package metadata changed' \
    >> "${temporary_root}/crates/ic-timers/Cargo.toml"
if [[ "$(classify)" != "crate" ]]; then
    echo "error: crate-manifest work was not classified as crate" >&2
    exit 1
fi

git -C "${temporary_root}" restore crates/ic-timers/Cargo.toml
printf '%s\n' '# workspace package metadata changed' \
    >> "${temporary_root}/Cargo.toml"
if [[ "$(classify)" != "crate" ]]; then
    echo "error: workspace-manifest work was not classified as crate" >&2
    exit 1
fi

echo "Release-impact classification checks passed"
