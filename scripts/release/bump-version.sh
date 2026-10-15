#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [--check] patch|minor|major|x.y.z" >&2
}

semver_pattern='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'

check_only=false
if [[ "${1:-}" == --check ]]; then
    check_only=true
    shift
fi
if [[ "$#" != 1 ]]; then
    usage
    exit 2
fi
requested="${1:-}"
case "${requested}" in
    patch | minor | major) ;;
    *)
        if [[ ! "${requested}" =~ ${semver_pattern} ]]; then
            usage
            exit 2
        fi
        ;;
esac

previous_version="$(bash scripts/ci/read-cargo-workspace-version.sh --stable "$PWD/Cargo.toml")"

if [[ "${requested}" =~ ^[0-9] ]]; then
    new_version="${requested}"
else
    new_version="$(bash scripts/ci/next-release-version.sh "$previous_version" "$requested")"
fi

order="$(perl "$(dirname "${BASH_SOURCE[0]}")/../../scripts/ci/compare-stable-versions.pl" \
    "${new_version}" "${previous_version}")"
if [[ "$order" != 1 ]]; then
    echo "error: target version ${new_version} must be greater than ${previous_version}" >&2
    exit 1
fi
release_tags="$(git tag --list "v${new_version}")"
if [[ -n "${release_tags}" ]]; then
    echo "error: tag v${new_version} already exists" >&2
    exit 1
fi

release_impact="$(
    bash scripts/release/classify-release-impact.sh
)"
bash scripts/release/check-bump-impact.sh "${release_impact}" "${previous_version}"

release_date="${IC_TIMERS_RELEASE_DATE:-$(date -u +%F)}"
bash scripts/ci/check-release-changelog.sh "$new_version" "$previous_version" "$release_date" CHANGELOG.md
if ! bash scripts/ci/warn-release-prose.sh "${new_version}" docs/status/current.md; then
    echo "warning: advisory release-prose check could not run; continuing" >&2
fi

if [[ "${check_only}" == true ]]; then
    echo "Release preflight passed: ${previous_version} -> ${new_version}"
    exit 0
fi

# Shared owns the byte-preserving transaction and coordinated graph checks.
# Local impact, tag, changelog preflight and advisory policy remain above.
RELEASE_PREVIOUS="$previous_version" RELEASE_VERSION="$new_version" RELEASE_DATE="$release_date" \
    bash "$(dirname "${BASH_SOURCE[0]}")/../../scripts/ci/cargo-release-metadata.sh" \
    prepare ic-timers

echo "Bumped: ${previous_version} -> ${new_version}"
echo "Review with git diff; deployment validation, commits, tags, and pushes are user-owned."
