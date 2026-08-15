#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
makefile="${repository_root}/Makefile"
bump_script="${repository_root}/scripts/release/bump-version.sh"
pocketic_check="${repository_root}/scripts/ci/check-pocketic.sh"

if grep -RE --include='*.sh' \
    '(^|[;&|[:space:]])rg([[:space:]]|$)' \
    "${repository_root}/scripts/ci" "${repository_root}/scripts/release" >/dev/null; then
    echo "error: runner-executed scripts must not require ripgrep" >&2
    exit 1
fi

if downgrade_output="$(bash "${bump_script}" 0.0.0 2>&1)"; then
    echo "error: version bump accepted a downgrade" >&2
    exit 1
fi
if [[ "${downgrade_output}" != *"must be greater than"* ]]; then
    echo "error: version bump did not reject a downgrade before release work" >&2
    exit 1
fi

if invalid_output="$(bash "${bump_script}" 00.4.0 2>&1)"; then
    echo "error: version bump accepted a leading-zero SemVer component" >&2
    exit 1
fi
if [[ "${invalid_output}" != Usage:* ]]; then
    echo "error: version bump did not reject invalid SemVer before release work" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'if git rev-parse --verify --quiet "refs/tags/v${new_version}" >/dev/null; then' \
    "${bump_script}" >/dev/null; then
    echo "error: version bump does not use the exact release-tag namespace" >&2
    exit 1
fi

expected_ci_targets="CI_TARGETS := actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package"
if ! grep -Fqx -- "${expected_ci_targets}" "${makefile}" >/dev/null; then
    echo "error: normal CI does not enforce the complete required target sequence" >&2
    exit 1
fi

expected_targets="RELEASE_TARGETS := pocketic-check ci msrv testing-check pocketic-watchdog pocketic-cohorts"
if ! grep -Fqx -- "${expected_targets}" "${makefile}" >/dev/null; then
    echo "error: release gate does not contain the complete required target sequence" >&2
    exit 1
fi

if ! grep -Fqx -- 'make --no-print-directory release-verify' "${bump_script}" >/dev/null; then
    echo "error: version bump does not invoke the complete release gate" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'bash scripts/release/finalize-release-truth.sh --check "${previous_version}" "${new_version}"' \
    "${bump_script}" >/dev/null \
    || ! grep -Fqx -- \
        'bash scripts/release/check-release-truth.sh' "${bump_script}" >/dev/null; then
    echo "error: version bump does not finalize and validate release truth" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'if ! bash scripts/release/warn-release-prose.sh "${new_version}"; then' \
    "${bump_script}" >/dev/null; then
    echo "error: version bump does not run the fail-open prose advisory" >&2
    exit 1
fi

if ! grep -Fqx -- 'pocketic-watchdog: pocketic-check' "${makefile}" >/dev/null \
    || ! grep -Fqx -- 'pocketic-cohorts: pocketic-check' "${makefile}" >/dev/null; then
    echo "error: PocketIC suites do not verify the evidence binary first" >&2
    exit 1
fi

if grep -Fqx -- 'make --no-print-directory ci' "${bump_script}" >/dev/null; then
    echo "error: version bump bypasses release-verify with the narrower CI gate" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'cargo update --manifest-path testing/Cargo.toml --offline -p ic-timers' \
    "${bump_script}" >/dev/null; then
    echo "error: version bump does not update the nested testing lockfile" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'cargo metadata --locked --offline --no-deps --format-version 1 >/dev/null' \
    "${bump_script}" >/dev/null \
    || ! grep -Fq -- 'cargo metadata --manifest-path testing/Cargo.toml' \
    "${bump_script}" >/dev/null \
    || ! grep -Fqx -- \
        '    --locked --offline --no-deps --format-version 1 >/dev/null' \
        "${bump_script}" >/dev/null; then
    echo "error: version bump does not verify nested locked metadata after mutation" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'expected_version="pocket-ic-server 15.0.0"' "${pocketic_check}" >/dev/null \
    || ! grep -Fqx -- \
        'expected_sha256="29472ea4433b30a280676c4e22e369d79d5ba6ee1b4d48bab32ebe7d0ad2b4bb"' \
        "${pocketic_check}" >/dev/null \
    || ! grep -Fqx -- \
        'expected_url="https://github.com/dfinity/pocketic/releases/download/15.0.0/pocket-ic-x86_64-linux.gz"' \
        "${pocketic_check}" >/dev/null; then
    echo "error: release evidence is not pinned to the audited PocketIC binary" >&2
    exit 1
fi

if ! grep -Fqx -- \
    'POCKET_IC_BIN ?= $(CURDIR)/target/tools/pocket-ic/$(POCKET_IC_VERSION)/pocket-ic' \
    "${makefile}" >/dev/null \
    || ! grep -Fqx -- \
        $'\t\tPOCKET_IC_AUTO_INSTALL="$(POCKET_IC_AUTO_INSTALL)" \\' \
        "${makefile}" >/dev/null; then
    echo "error: default release flow does not provision the pinned PocketIC binary" >&2
    exit 1
fi

if ! grep -Fq -- \
    'git add Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md README.md' \
    "${makefile}" >/dev/null \
    || ! grep -Fq -- \
        'crates/ic-timers/Cargo.toml docs/status/current.md docs/adoption/canic.md' \
        "${makefile}" >/dev/null \
    || ! grep -Fqx -- $'\t\t\t"docs/changelog/$${version}.md"' "${makefile}" >/dev/null; then
    echo "error: release staging omits required lockfiles or release-truth documents" >&2
    exit 1
fi

echo "Release gate wiring checks passed"
