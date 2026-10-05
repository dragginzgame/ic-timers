#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

if [[ -n "${1:-}" ]]; then
    base_ref="${1}"
else
    # Package metadata can advance before a release is tagged. Compare the
    # entire pending subject with the most recent reachable release tag.
    base_ref="$(git describe --tags --abbrev=0 --match 'v[0-9]*' HEAD)"
    if [[ ! "${base_ref}" =~ ^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]]; then
        echo "error: release-impact base is not a canonical release tag: ${base_ref}" >&2
        exit 1
    fi
fi

if ! git rev-parse --verify --quiet "${base_ref}^{commit}" >/dev/null; then
    echo "error: release-impact base does not resolve to a commit: ${base_ref}" >&2
    exit 1
fi

# Complete both Git queries before reading their NUL-delimited paths. Display
# quoting and line breaks must not hide crate changes, nor may producer failures
# appear to be an unchanged subject. Duplicate paths do not affect classification.
changed_paths="$(mktemp "${TMPDIR:-/tmp}/ic-timers-impact.XXXXXX")"
trap 'rm -f -- "${changed_paths}"' EXIT
git diff --no-renames --name-only --diff-filter=ACDMRTUXB -z "${base_ref}" -- > "${changed_paths}"
git ls-files --others --exclude-standard -z >> "${changed_paths}"

impact="none"
while IFS= read -r -d '' path; do
    impact="repository"
    case "${path}" in
        Cargo.toml | crates/ic-timers/Cargo.toml | crates/ic-timers/build.rs | crates/ic-timers/src/*)
            impact="crate"
            break
            ;;
    esac
done < "${changed_paths}"

printf '%s\n' "${impact}"
