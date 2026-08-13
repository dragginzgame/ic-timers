#!/usr/bin/env bash
set -euo pipefail

if ! git rev-parse --verify HEAD >/dev/null 2>&1; then
    echo "error: repository has no commit yet" >&2
    exit 1
fi

if ! git diff-index --quiet HEAD -- || [[ -n "$(git ls-files --others --exclude-standard)" ]]; then
    echo "error: working directory is not clean; commit or stash changes first" >&2
    exit 1
fi
