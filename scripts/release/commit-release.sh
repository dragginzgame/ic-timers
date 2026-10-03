#!/usr/bin/env bash
set -euo pipefail

version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${version}" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
    echo "error: failed to read a release version from Cargo.toml" >&2
    exit 1
fi
bash scripts/release/check-release-truth.sh
bash scripts/release/check-lockfiles.sh
if ! git diff --quiet || [[ -n "$(git ls-files --others --exclude-standard)" ]]; then
    echo "error: commit or stage all release changes before creating v${version}" >&2
    exit 1
fi
if git diff --cached --quiet; then
    # Only a matching release commit may resume an interrupted tag phase.
    if [[ "$(git log -1 --format=%s)" != "Release ${version}" ]]; then
        echo "error: no staged release changes and HEAD is not Release ${version}" >&2
        exit 1
    fi
else
    if git rev-parse --verify --quiet "refs/tags/v${version}" >/dev/null; then
        echo "error: tag v${version} already exists; cannot commit more changes for that release" >&2
        exit 1
    fi
    git commit -m "Release ${version}"
fi
make --no-print-directory ensure-clean
if git rev-parse --verify --quiet "refs/tags/v${version}" >/dev/null; then
    bash scripts/release/check-tag-at-head.sh
else
    git tag -a "v${version}" -m "Release ${version}"
fi
