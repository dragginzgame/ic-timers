#!/usr/bin/env bash
set -euo pipefail

before_bump=false
case "$#:${1:-}" in
    0:) ;;
    1:--check-before-bump) before_bump=true ;;
    *) echo "Usage: $0 [--check-before-bump]" >&2; exit 2 ;;
esac

version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

# Capture both queries before reading paths. Failed or partial Git output must
# not establish a clean worktree; NUL records preserve all filename bytes.
worktree_paths="$(mktemp "${TMPDIR:-/tmp}/ic-timers-release-paths.XXXXXX")"
trap 'rm -f -- "${worktree_paths}"' EXIT
git diff --no-renames --name-only -z -- > "${worktree_paths}"
git ls-files --others --exclude-standard -z >> "${worktree_paths}"
unstaged=false
while IFS= read -r -d '' path; do
    if [[ "${before_bump}" == true ]]; then
        # These are exactly the three outputs selected by make release-stage.
        case "${path}" in
            Cargo.toml | Cargo.lock | CHANGELOG.md) continue ;;
        esac
    fi
    if [[ "${unstaged}" == false ]]; then
        if [[ "${before_bump}" == true ]]; then
            echo 'error: stage or commit implementation changes before starting the release gate' >&2
            echo 'release-stage selects only Cargo.toml, Cargo.lock and CHANGELOG.md' >&2
        else
            echo "error: commit or stage all release changes before creating v${version}" >&2
        fi
        echo 'Unstaged paths (Bash-escaped):' >&2
    fi
    printf '  %q\n' "${path}" >&2
    unstaged=true
done < "${worktree_paths}"
if [[ "${unstaged}" == true ]]; then
    exit 1
fi
if [[ "${before_bump}" == true ]]; then
    echo 'Release worktree preflight passed.'
    exit 0
fi

bash scripts/release/check-lockfiles.sh
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
    release_tags="$(git tag --list "v${version}")"
    if [[ -n "${release_tags}" ]]; then
        echo "error: tag v${version} already exists; cannot commit more changes for that release" >&2
        exit 1
    fi
    git commit -m "Release ${version}"
fi
make --no-print-directory ensure-clean
release_tags="$(git tag --list "v${version}")"
if [[ -n "${release_tags}" ]]; then
    bash scripts/release/check-tag-at-head.sh
else
    git tag -a "v${version}" -m "Release ${version}"
fi
