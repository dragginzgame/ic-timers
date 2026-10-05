#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

bash scripts/release/readme-version.sh --check
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
