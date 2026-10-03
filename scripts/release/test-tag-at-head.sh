#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
checker="${repository_root}/scripts/release/check-tag-at-head.sh"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
git init -q "${temporary_root}"
cd "${temporary_root}"
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
printf '%s\n' '[workspace.dependencies.fixture]' 'version = "0.2.0"' \
    '[workspace.package]' 'version = "0.1.0"' > Cargo.toml
git add Cargo.toml
git commit -qm fixture

expect_rejection() {
    local expected="${1}" output
    if output="$(bash "${checker}" 2>&1)"; then
        echo "error: tag guard accepted ${expected}" >&2
        exit 1
    fi
    if [[ "${output}" != *"${expected}"* ]]; then
        echo "error: unexpected tag guard rejection: ${output}" >&2
        exit 1
    fi
}

git branch v0.1.0
expect_rejection 'does not exist'
git tag v0.1.0
expect_rejection 'must be annotated'
git tag -d v0.1.0 >/dev/null
git tag -a v0.1.0 -m fixture
bash "${checker}"
git commit --allow-empty -qm 'advance head'
expect_rejection 'does not point to HEAD'
echo 'Exact annotated release-tag checks passed'
