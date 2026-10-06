#!/usr/bin/env bash
set -euo pipefail

case "$#" in
    0)
        version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"
        selected_commit="$(git rev-parse HEAD)"
        selected_name=HEAD
        ;;
    2)
        selected_commit="$1"
        version="$2"
        if [[ ! "$version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
            echo 'error: invalid release version' >&2
            exit 2
        fi
        resolved="$(git rev-parse --verify "${selected_commit}^{commit}")"
        if [[ "$selected_commit" != "$resolved" ]]; then
            echo 'error: selected release must be an exact commit SHA' >&2
            exit 1
        fi
        selected_name="$selected_commit"
        ;;
    *) echo 'usage: check-tag-at-head.sh [COMMIT_SHA VERSION]' >&2; exit 2 ;;
esac

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
if [[ "${tag_commit}" != "${selected_commit}" ]]; then
    echo "error: release tag v${version} does not point to ${selected_name}" >&2
    exit 1
fi
