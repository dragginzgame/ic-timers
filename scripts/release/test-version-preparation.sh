#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{scripts/release,docs/status,docs/changelog,docs/adoption,crates/ic-timers/src,testing/probe/src}
for script in bump-version finalize-changelog finalize-release-truth check-release-truth \
    warn-release-prose check-bump-impact check-lockfiles; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
# Classification is an isolated fixture input; preparation must not run tests.
printf '%s\n' '#!/usr/bin/env bash' 'echo crate' \
    > "${temporary_root}/scripts/release/classify-release-impact.sh"
cat > "${temporary_root}/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/ic-timers"]
resolver = "3"
[workspace.package]
version = "0.1.0"
EOF
cat > "${temporary_root}/crates/ic-timers/Cargo.toml" <<'EOF'
[package]
name = "ic-timers"
version.workspace = true
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/crates/ic-timers/src/lib.rs"
cat > "${temporary_root}/testing/Cargo.toml" <<'EOF'
[workspace]
members = ["probe"]
resolver = "3"
EOF
cat > "${temporary_root}/testing/probe/Cargo.toml" <<'EOF'
[package]
name = "probe"
version = "0.0.0"
edition = "2024"
[dependencies]
ic-timers = { path = "../../crates/ic-timers" }
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/testing/probe/src/lib.rs"
cat > "${temporary_root}/CHANGELOG.md" <<'EOF'
# Changelog

## [Unreleased]

## [0.1.1]

- Fix terminal cleanup.

## [0.1.0] - 2026-08-01

- Initial release.
EOF
cat > "${temporary_root}/docs/status/current.md" <<'EOF'
# Current status

- Workspace package version: `0.1.0`.
- Latest release line: `0.1.0`.
- Named target release: `0.1.1` (unreleased).
EOF
printf '%s\n' '# 0.1.1' '' 'Status: targeted unreleased 0.1.1.' \
    > "${temporary_root}/docs/changelog/0.1.1.md"
printf '%s\n' '# Fixture' > "${temporary_root}/README.md"
printf '%s\n' '# Adoption' '' 'Status: fixture.' > "${temporary_root}/docs/adoption/canic.md"
printf '%s\n' 'release-verify:' $'\t@touch unexpected-gate' $'\t@exit 1' \
    > "${temporary_root}/Makefile"
printf '%s\n' 'Unrelated work must survive preparation.' > "${temporary_root}/unrelated.txt"
cd "${temporary_root}"
cargo generate-lockfile --offline --quiet
cargo generate-lockfile --manifest-path testing/Cargo.toml --offline --quiet
bash scripts/release/bump-version.sh patch
test ! -f unexpected-gate
grep -Fqx 'version = "0.1.1"' Cargo.toml
bash scripts/release/check-lockfiles.sh
grep -Fqx 'Unrelated work must survive preparation.' unrelated.txt
if git rev-parse --verify HEAD >/dev/null 2>&1 || [[ -n "$(git tag --list)" ]]; then
    echo 'error: version preparation committed or tagged fixture changes' >&2
    exit 1
fi
if [[ -n "$(git diff --cached --name-only)" ]]; then
    echo 'error: version preparation staged unrelated fixture changes' >&2
    exit 1
fi
echo 'Dirty-worktree version preparation checks passed'
