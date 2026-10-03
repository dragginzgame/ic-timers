#!/usr/bin/env bash
set -euo pipefail

version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: failed to read a release version from Cargo.toml" >&2
    exit 1
fi
if git rev-parse --verify --quiet "refs/tags/v${version}" >/dev/null; then
    echo "error: tag v${version} already exists" >&2
    exit 1
fi

bash scripts/release/check-release-truth.sh
bash scripts/release/check-lockfiles.sh
if ! git diff --quiet || [[ -n "$(git ls-files --others --exclude-standard)" ]]; then
    echo "error: commit or stage all release changes before creating v${version}" >&2
    exit 1
fi
git commit -m "Release ${version}"
make --no-print-directory ensure-clean
git tag -a "v${version}" -m "Release ${version}"
