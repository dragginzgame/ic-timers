#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
cd "${repository_root}"

if [[ -n "${1:-}" ]]; then
    base_ref="${1}"
else
    version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
    base_ref="v${version}"
fi

if ! git rev-parse --verify --quiet "${base_ref}^{commit}" >/dev/null; then
    echo "error: release-impact base does not resolve to a commit: ${base_ref}" >&2
    exit 1
fi

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
done < <(
    {
        git diff --no-renames --name-only --diff-filter=ACDMRTUXB "${base_ref}" --
        git ls-files --others --exclude-standard
    } | sort -u
)

printf '%s\n' "${impact}"
