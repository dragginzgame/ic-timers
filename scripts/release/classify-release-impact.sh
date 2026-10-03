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

# Command substitution propagates producer failures; process substitution would
# turn a failed Git command into an empty, apparently unchanged release subject.
tracked_paths="$(git diff --no-renames --name-only --diff-filter=ACDMRTUXB "${base_ref}" --)"
untracked_paths="$(git ls-files --others --exclude-standard)"
changed_paths="$(printf '%s\n' "${tracked_paths}" "${untracked_paths}" | sort -u)"

impact="none"
while IFS= read -r path; do
    [[ -z "${path}" ]] && continue
    impact="repository"
    case "${path}" in
        Cargo.toml | crates/ic-timers/Cargo.toml | crates/ic-timers/build.rs | crates/ic-timers/src/*)
            impact="crate"
            break
            ;;
    esac
done <<< "${changed_paths}"

printf '%s\n' "${impact}"
