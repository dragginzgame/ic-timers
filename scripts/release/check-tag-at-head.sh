#!/usr/bin/env bash
set -euo pipefail

version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

tag_ref="refs/tags/v${version}"
if ! tag_type="$(git cat-file -t "${tag_ref}" 2>/dev/null)"; then
    echo "error: release tag v${version} does not exist" >&2
    exit 1
fi
if [[ "${tag_type}" != tag ]]; then
    echo "error: release tag v${version} must be annotated" >&2
    exit 1
fi
tag_commit="$(git rev-parse --verify "${tag_ref}^{commit}")"
head_commit="$(git rev-parse HEAD)"
if [[ "${tag_commit}" != "${head_commit}" ]]; then
    echo "error: release tag v${version} does not point to HEAD" >&2
    exit 1
fi
