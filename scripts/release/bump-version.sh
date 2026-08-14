#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 patch|minor|major|x.y.z" >&2
}

requested="${1:-}"
case "${requested}" in
    patch | minor | major) ;;
    *)
        if [[ ! "${requested}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            usage
            exit 2
        fi
        ;;
esac

previous_version="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)"
if [[ ! "${previous_version}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
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

if [[ "${new_version}" == "${previous_version}" ]]; then
    echo "error: target version is already ${new_version}" >&2
    exit 1
fi
if git rev-parse "v${new_version}" >/dev/null 2>&1; then
    echo "error: tag v${new_version} already exists" >&2
    exit 1
fi

bash scripts/release/finalize-changelog.sh --check "${new_version}"

make --no-print-directory ensure-clean
make --no-print-directory release-verify

bash scripts/release/finalize-changelog.sh "${new_version}"

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

echo "Bumped: ${previous_version} -> ${new_version}"
echo "Review with git diff, then use release-stage, release-commit, and release-push."
