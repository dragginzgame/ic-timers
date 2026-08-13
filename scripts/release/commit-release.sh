#!/usr/bin/env bash
set -euo pipefail

version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: failed to read a release version from Cargo.toml" >&2
    exit 1
fi
if git rev-parse "v${version}" >/dev/null 2>&1; then
    echo "error: tag v${version} already exists" >&2
    exit 1
fi

git commit -m "Release ${version}"
git tag -a "v${version}" -m "Release ${version}"
