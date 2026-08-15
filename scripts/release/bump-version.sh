#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 patch|minor|major|x.y.z" >&2
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

previous_version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${previous_version}" =~ ${semver_pattern} ]]; then
    echo "error: failed to read a SemVer workspace version" >&2
    exit 1
fi

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
    bash scripts/release/classify-release-impact.sh "v${previous_version}"
)"
if [[ "${release_impact}" != "crate" ]]; then
    echo "error: changes since v${previous_version} are ${release_impact};" >&2
    echo "       do not publish a repository-only crate release" >&2
    echo "hint: use make repository-check and commit the repository update without a version tag" >&2
    exit 1
fi

release_date="$(date +%F)"
bash scripts/release/finalize-changelog.sh --check "${new_version}" "${release_date}"
bash scripts/release/finalize-release-truth.sh --check "${previous_version}" "${new_version}"
if ! bash scripts/release/warn-release-prose.sh "${new_version}"; then
    echo "warning: advisory release-prose check could not run; continuing" >&2
fi

make --no-print-directory ensure-clean
make --no-print-directory release-verify

bash scripts/release/finalize-changelog.sh "${new_version}" "${release_date}"
bash scripts/release/finalize-release-truth.sh "${previous_version}" "${new_version}"

IC_TIMERS_PREVIOUS_VERSION="${previous_version}" \
IC_TIMERS_NEW_VERSION="${new_version}" \
perl -0pi -e '
    s/^version = "\Q$ENV{IC_TIMERS_PREVIOUS_VERSION}\E"$/version = "$ENV{IC_TIMERS_NEW_VERSION}"/m
' Cargo.toml
cargo update --offline -p ic-timers
cargo update --manifest-path testing/Cargo.toml --offline -p ic-timers

# Version mutation must leave both independently locked workspaces coherent.
# The expensive behavioral evidence ran before mutation and is not repeated.
cargo metadata --locked --offline --no-deps --format-version 1 >/dev/null
cargo metadata --manifest-path testing/Cargo.toml \
    --locked --offline --no-deps --format-version 1 >/dev/null
bash scripts/release/check-release-truth.sh

echo "Bumped: ${previous_version} -> ${new_version}"
echo "Review with git diff, then use release-stage, release-commit, and release-push."
