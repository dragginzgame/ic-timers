#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{scripts/release,docs/status,docs/changelog,crates/ic-timers/src,testing/probe/src}
for script in bump-version finalize-changelog finalize-release-truth check-release-truth \
    warn-release-prose check-bump-impact check-lockfiles; do
    cp "${repository_root}/scripts/release/${script}.sh" "${temporary_root}/scripts/release/"
done
# Classification is an isolated fixture input; preparation must not run tests.
cat > "${temporary_root}/scripts/release/classify-release-impact.sh" <<'EOF'
#!/usr/bin/env bash
if [[ "${FIXTURE_RELEASE_IMPACT:-crate}" == error ]]; then
    echo 'fixture classification failure' >&2
    exit 1
fi
echo "${FIXTURE_RELEASE_IMPACT:-crate}"
EOF
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
EOF
printf '%s\n' '# 0.1.1' '' 'Status: prepared for 0.1.1; delivery is user-owned.' \
    > "${temporary_root}/docs/changelog/0.1.1.md"
printf '%s\n' '# Fixture' > "${temporary_root}/README.md"
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

# Classification must reach the bump boundary, including failure and advisory.
for impact in none unexpected error; do
    if FIXTURE_RELEASE_IMPACT="${impact}" bash scripts/release/bump-version.sh --check patch >/dev/null 2>&1; then
        echo "error: preflight accepted ${impact} release impact" >&2
        exit 1
    fi
    sha256sum --check --quiet original.sha256
done
output="$(FIXTURE_RELEASE_IMPACT=repository bash scripts/release/bump-version.sh --check patch 2>&1)"
if [[ "${output}" != *'continuing because the maintainer invoked an explicit version bump'* ]]; then
    echo 'error: repository-only preflight omitted its advisory' >&2
    exit 1
fi

# Exercise every real release recipe. Only the preflight is real: the remaining
# phase targets record calls, so the fixture cannot commit, push or run suites.
cp Makefile preparation-only.mk
cp "${repository_root}/Makefile" Makefile
mv scripts/release/bump-version.sh scripts/release/preparation-bump-version.sh
cat > scripts/release/bump-version.sh <<'EOF'
#!/usr/bin/env bash
printf 'preflight %s\n' "$*" >> release-events
bash scripts/release/preparation-bump-version.sh "$@"
EOF
cat > overrides.mk <<'EOF'
release-verify:
	@printf '%s\n' gate >> release-events
	@if [ '$(FAIL_GATE)' = 1 ]; then exit 1; fi
patch minor major bump-x:
	@printf 'bump %s\n' '$(if $(filter bump-x,$@),$(VERSION),$@)' >> release-events
release-stage release-commit release-push:
	@printf '%s\n' '$@' >> release-events
ensure-clean release-tag-check:
	@:
EOF
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')
cp CHANGELOG.md original-changelog.md
for target in release-patch release-minor release-major release-x; do
    case "${target}" in
        release-patch) requested=patch; candidate=0.1.1 ;;
        release-minor) requested=minor; candidate=0.2.0 ;;
        release-major) requested=major; candidate=1.0.0 ;;
        release-x) requested=0.4.2; candidate=0.4.2 ;;
    esac
    sed "s/0.1.1/${candidate}/g" original-changelog.md > candidate-changelog.md
    if [[ "${candidate}" != 0.1.1 ]]; then
        printf '# %s\n\nStatus: prepared for %s; delivery is user-owned.\n' \
            "${candidate}" "${candidate}" > "docs/changelog/${candidate}.md"
    fi
    for scenario in empty-notes failed-gate success repeat-success; do
        cp candidate-changelog.md CHANGELOG.md
        case "${scenario}" in
            empty-notes)
                sed -i 's/^- Fix terminal cleanup\.$//' CHANGELOG.md
                expected=("preflight --check ${requested}") ;;
            failed-gate) expected=("preflight --check ${requested}" gate) ;;
            *) expected=("preflight --check ${requested}" gate "bump ${requested}"
                release-stage release-commit release-push) ;;
        esac
        fail_gate=0
        if [[ "${scenario}" == failed-gate ]]; then fail_gate=1; fi
        if "${fixture_make[@]}" "${target}" "VERSION=${candidate}" "FAIL_GATE=${fail_gate}" >/dev/null 2>&1; then
            if [[ "${scenario}" == empty-notes || "${scenario}" == failed-gate ]]; then
                echo "error: ${target} accepted ${scenario}" >&2
                exit 1
            fi
        elif [[ "${scenario}" == success || "${scenario}" == repeat-success ]]; then
            echo "error: ${target} rejected valid preflight" >&2
            exit 1
        fi
        mapfile -t actual < release-events
        if [[ "${actual[*]}" != "${expected[*]}" ]]; then
            echo "error: ${target} ${scenario} ran unexpected phases: ${actual[*]}" >&2
            exit 1
        fi
        rm release-events
        cp original-changelog.md CHANGELOG.md
        sha256sum --check --quiet original.sha256
    done
    if [[ "${candidate}" != 0.1.1 ]]; then rm "docs/changelog/${candidate}.md"; fi
done
if "${fixture_make[@]}" release-x VERSION= >/dev/null 2>&1; then
    echo 'error: exact release accepted an empty target' >&2
    exit 1
fi
test ! -f release-events
rm original-changelog.md candidate-changelog.md overrides.mk
mv scripts/release/preparation-bump-version.sh scripts/release/bump-version.sh
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

# A failed prose advisory still runs before mutation and cannot block the bump.
mv scripts/release/warn-release-prose.sh scripts/release/original-warn-release-prose.sh
cat > scripts/release/warn-release-prose.sh <<'EOF'
#!/usr/bin/env bash
sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml > advisory-version
exit 1
EOF
output="$(bash scripts/release/bump-version.sh patch 2>&1)"
if [[ "${output}" != *'advisory release-prose check could not run; continuing'* ]]; then
    echo 'error: version preparation did not report its failed advisory' >&2
    exit 1
fi
grep -Fqx 0.1.0 advisory-version
mv scripts/release/original-warn-release-prose.sh scripts/release/warn-release-prose.sh
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

# Staging selects current release metadata and excludes unrelated adoption work.
cp "${repository_root}/Makefile" Makefile
mkdir -p docs/adoption
printf '%s\n' '# Unrelated adoption edits' > docs/adoption/canic.md
make --no-print-directory release-stage >/dev/null
expected_staged=(CHANGELOG.md Cargo.lock Cargo.toml README.md crates/ic-timers/Cargo.toml
    docs/changelog/0.1.1.md docs/status/current.md testing/Cargo.lock)
mapfile -t staged < <(git diff --cached --name-only)
if [[ "${staged[*]}" != "${expected_staged[*]}" ]]; then
    echo "error: release staging selected unexpected paths: ${staged[*]}" >&2
    exit 1
fi
grep -Fqx '# Unrelated adoption edits' docs/adoption/canic.md

# A same-named branch is harmless; only an existing exact release tag blocks.
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
git commit -qm fixture
git branch v0.1.2
sed -i '/^## \[Unreleased\]$/a\
\n## [0.1.2]\n\n- Next fixture release.' CHANGELOG.md
printf '%s\n' '# 0.1.2' '' 'Status: prepared for 0.1.2; delivery is user-owned.' > docs/changelog/0.1.2.md
bash scripts/release/bump-version.sh --check patch >/dev/null
git tag v0.1.2
sha256sum Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md \
    docs/status/current.md docs/changelog/0.1.2.md > tagged.sha256
if output="$(bash scripts/release/bump-version.sh --check patch 2>&1)"; then
    echo 'error: version preflight accepted an existing release tag' >&2
    exit 1
fi
if [[ "${output}" != *'tag v0.1.2 already exists'* ]]; then
    echo "error: unexpected tag rejection: ${output}" >&2
    exit 1
fi
sha256sum --check --quiet tagged.sha256
echo 'Version preflight, rollback and dirty-worktree preparation checks passed'
