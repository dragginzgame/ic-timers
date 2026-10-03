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

# Preflight validates metadata without running deployment tests or changing it.
metadata_files=(Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md \
    docs/status/current.md docs/changelog/0.1.1.md unrelated.txt)
chmod 0640 CHANGELOG.md
sha256sum "${metadata_files[@]}" > original.sha256
stat -c '%a %n' "${metadata_files[@]}" > original-modes
bash scripts/release/bump-version.sh --check patch
sha256sum --check --quiet original.sha256

# Exercise the real release recipes with an isolated failing deployment gate.
cp Makefile preparation-only.mk
cp "${repository_root}/Makefile" Makefile
printf '%s\n' 'release-verify:' $'\t@touch gate-ran' $'\t@exit 1' > overrides.mk
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')
cp CHANGELOG.md original-changelog.md
sed -i 's/^- Fix terminal cleanup\.$//' CHANGELOG.md
if "${fixture_make[@]}" release-patch >/dev/null 2>&1; then
    echo 'error: release target accepted empty notes' >&2
    exit 1
fi
test ! -f gate-ran
if "${fixture_make[@]}" release-x VERSION= >/dev/null 2>&1; then
    echo 'error: exact release accepted an empty target' >&2
    exit 1
fi
test ! -f gate-ran
mv original-changelog.md CHANGELOG.md
if "${fixture_make[@]}" release-patch >/dev/null 2>&1; then
    echo 'error: release target ignored a failed deployment gate' >&2
    exit 1
fi
test -f gate-ran
sha256sum --check --quiet original.sha256
rm gate-ran overrides.mk
mv preparation-only.mk Makefile

# Inject failures after each independently mutating lock update and check.
mkdir -p bin
IC_TIMERS_FIXTURE_CARGO="$(command -v cargo)"
export IC_TIMERS_FIXTURE_CARGO
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    'update --offline -p ic-timers') stage=root-update ;;
    'update --manifest-path testing/Cargo.toml --offline -p ic-timers') stage=testing-update ;;
    'metadata --locked --offline --format-version 1') stage=root-metadata ;;
    'metadata --manifest-path testing/Cargo.toml --locked --offline --format-version 1') stage=testing-metadata ;;
    *) stage=other ;;
esac
if [[ "${FIXTURE_FAIL_STAGE:-}" == "${stage}" ]]; then
    echo "injected ${stage} failure" >&2
    exit 1
fi
if [[ "${FIXTURE_FAIL_STAGE:-}" == interrupt && "${stage}" == testing-update ]]; then
    kill -TERM "${PPID}"
    exit 1
fi
exec "${IC_TIMERS_FIXTURE_CARGO}" "$@"
EOF
chmod +x bin/cargo
cp scripts/release/check-release-truth.sh scripts/release/original-release-truth.sh
cat > scripts/release/check-release-truth.sh <<'EOF'
#!/usr/bin/env bash
if [[ "${FIXTURE_FAIL_STAGE:-}" == release-truth ]]; then
    echo 'injected release-truth failure' >&2
    exit 1
fi
bash scripts/release/original-release-truth.sh
EOF
for stage in root-update testing-update root-metadata testing-metadata release-truth interrupt; do
    if output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE="${stage}" \
        bash scripts/release/bump-version.sh patch 2>&1)"; then
        echo "error: version preparation accepted injected ${stage} failure" >&2
        exit 1
    fi
    if [[ "${output}" != *'restoring release metadata'* ]]; then
        echo "error: version preparation did not roll back ${stage}: ${output}" >&2
        exit 1
    fi
    sha256sum --check --quiet original.sha256
    stat -c '%a %n' "${metadata_files[@]}" > restored-modes
    cmp original-modes restored-modes
done
mv scripts/release/original-release-truth.sh scripts/release/check-release-truth.sh

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
echo 'Version preflight, rollback and dirty-worktree preparation checks passed'
