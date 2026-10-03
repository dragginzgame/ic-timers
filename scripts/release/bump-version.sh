#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 [--check] patch|minor|major|x.y.z" >&2
}

semver_pattern='^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'

semver_greater_than() {
    local candidate="${1}"
    local baseline="${2}"
    local candidate_parts
    local baseline_parts
    local index
    local candidate_part
    local baseline_part

    IFS=. read -r -a candidate_parts <<< "${candidate}"
    IFS=. read -r -a baseline_parts <<< "${baseline}"
    for index in 0 1 2; do
        candidate_part="${candidate_parts[${index}]}"
        baseline_part="${baseline_parts[${index}]}"
        if [[ "${candidate_part}" == "${baseline_part}" ]]; then
            continue
        fi
        if (( ${#candidate_part} != ${#baseline_part} )); then
            (( ${#candidate_part} > ${#baseline_part} ))
            return
        fi
        [[ "${candidate_part}" > "${baseline_part}" ]]
        return
    done
    return 1
}

check_only=false
if [[ "${1:-}" == --check ]]; then
    check_only=true
    shift
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

previous_version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

if [[ "${requested}" =~ ^[0-9] ]]; then
    new_version="${requested}"
else
    IFS=. read -r major minor patch <<< "${previous_version}"
    case "${requested}" in
        patch) patch=$((patch + 1)) ;;
        minor) minor=$((minor + 1)); patch=0 ;;
        major) major=$((major + 1)); minor=0; patch=0 ;;
    esac
    new_version="${major}.${minor}.${patch}"
fi

if ! semver_greater_than "${new_version}" "${previous_version}"; then
    echo "error: target version ${new_version} must be greater than ${previous_version}" >&2
    exit 1
fi
if git rev-parse --verify --quiet "refs/tags/v${new_version}" >/dev/null; then
    echo "error: tag v${new_version} already exists" >&2
    exit 1
fi

release_impact="$(
    bash scripts/release/classify-release-impact.sh
)"
bash scripts/release/check-bump-impact.sh "${release_impact}" "${previous_version}"

release_date="$(date +%F)"
bash scripts/release/finalize-changelog.sh --check "${new_version}" "${release_date}"
bash scripts/release/finalize-release-truth.sh --check "${previous_version}" "${new_version}"
if ! bash scripts/release/warn-release-prose.sh "${new_version}"; then
    echo "warning: advisory release-prose check could not run; continuing" >&2
fi

if [[ "${check_only}" == true ]]; then
    echo "Release preflight passed: ${previous_version} -> ${new_version}"
    exit 0
fi

# Capture only files this bump mutates, including any existing user edits.
# Failed updates/checks and catchable interruptions restore that exact state.
backup_directory="$(mktemp -d "${TMPDIR:-/tmp}/ic-timers-bump.XXXXXX")"
metadata_files=(Cargo.toml Cargo.lock testing/Cargo.lock CHANGELOG.md \
    docs/status/current.md "docs/changelog/${new_version}.md")
mutation_started=false
bump_completed=false
cleanup() {
    local exit_status=$?
    local rollback_failed=false
    local path
    trap - EXIT
    if [[ "${mutation_started}" == true && "${bump_completed}" != true ]]; then
        echo 'error: version preparation failed; restoring release metadata' >&2
        for path in "${metadata_files[@]}"; do
            if ! cp -p -- "${backup_directory}/${path}" "${path}"; then
                rollback_failed=true
            fi
        done
        if [[ "${rollback_failed}" == true ]]; then
            echo "error: rollback incomplete; originals preserved in ${backup_directory}" >&2
            exit 1
        fi
        if [[ "${exit_status}" == 0 ]]; then exit_status=1; fi
    fi
    rm -rf -- "${backup_directory}"
    exit "${exit_status}"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
for path in "${metadata_files[@]}"; do
    mkdir -p -- "${backup_directory}/$(dirname -- "${path}")"
    cp -p -- "${path}" "${backup_directory}/${path}"
done
mutation_started=true

bash scripts/release/finalize-changelog.sh "${new_version}" "${release_date}"
bash scripts/release/finalize-release-truth.sh "${previous_version}" "${new_version}"

bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh" set "${previous_version}" "${new_version}"
cargo update --offline -p ic-timers
cargo update --manifest-path testing/Cargo.toml --offline -p ic-timers

# Version mutation must leave both independently locked workspaces coherent.
# Behavioral evidence belongs to the user-operated deployment release gate.
bash scripts/release/check-lockfiles.sh
bash scripts/release/check-release-truth.sh
bump_completed=true

echo "Bumped: ${previous_version} -> ${new_version}"
echo "Review with git diff; deployment validation, commits, tags, and pushes are user-owned."
