#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

bash scripts/release/readme-version.sh --check
bash scripts/release/check-lockfiles.sh
untracked_paths="$(git ls-files --others --exclude-standard)"
if ! git diff --quiet || [[ -n "${untracked_paths}" ]]; then
    echo "error: commit or stage all release changes before creating v${version}" >&2
    exit 1
fi
staged_status=0
git diff --cached --quiet || staged_status=$?
if [[ "${staged_status}" -gt 1 ]]; then
    echo 'error: cannot inspect staged release changes' >&2
    exit "${staged_status}"
fi
if [[ "${staged_status}" == 0 ]]; then
    # Only a matching release commit may resume an interrupted tag phase.
    head_subject="$(git log -1 --format=%s)"
    if [[ "${head_subject}" != "Release ${version}" ]]; then
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
