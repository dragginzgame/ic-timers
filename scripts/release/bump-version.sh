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

previous_version="$(bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh")"

if [[ "${requested}" =~ ^[0-9] ]]; then
    new_version="${requested}"
else
    new_version="$(bash scripts/ci/next-release-version.sh "$previous_version" "$requested")"
fi

if ! semver_greater_than "${new_version}" "${previous_version}"; then
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
IC_TIMERS_RELEASE_PREVIOUS="${previous_version}" \
    bash scripts/release/finalize-changelog.sh --check "${new_version}" "${release_date}"
bash scripts/release/readme-version.sh --check
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
metadata_files=(Cargo.toml Cargo.lock CHANGELOG.md README.md)
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
            if [[ -f "${backup_directory}/${path}" ]]; then
                if ! cp -p -- "${backup_directory}/${path}" "${path}"; then
                    rollback_failed=true
                fi
            elif ! rm -f -- "${path}"; then
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
    if [[ -L "${path}" ]] || [[ -e "${path}" && ! -f "${path}" ]]; then
        echo "error: release metadata must be a regular file: ${path}" >&2
        exit 1
    fi
    mkdir -p -- "${backup_directory}/$(dirname -- "${path}")"
    if [[ -e "${path}" ]]; then
        cp -p -- "${path}" "${backup_directory}/${path}"
    fi
done
mutation_started=true

IC_TIMERS_RELEASE_PREVIOUS="${previous_version}" \
    bash scripts/release/finalize-changelog.sh "${new_version}" "${release_date}"

bash "$(dirname -- "${BASH_SOURCE[0]}")/workspace-version.sh" set "${previous_version}" "${new_version}"
bash scripts/release/readme-version.sh --update
# Preserve the tested dependency selection; update only this local package.
bash scripts/release/update-local-lock.sh Cargo.lock "$previous_version" "$new_version"

# Version mutation must leave the complete root workspace graph coherent.
# Behavioral evidence belongs to the user-operated deployment release gate.
bash scripts/release/check-lockfiles.sh
bash scripts/release/readme-version.sh --check
bump_completed=true

echo "Bumped: ${previous_version} -> ${new_version}"
echo "Review with git diff; deployment validation, commits, tags, and pushes are user-owned."
