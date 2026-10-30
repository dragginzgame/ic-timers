#!/usr/bin/env bash
set -euo pipefail

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/timer-fixture-completion.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed fixture-completion checks retained"' EXIT
# shellcheck source=scripts/ci/fixture-retention-probes.sh
source "$root/scripts/ci/fixture-retention-probes.sh"

# Copy each actual initialization/EXIT boundary, including this suite's own.
# Stop before the fixture body so these probes cannot dispatch tools, builds,
# release helpers, Git mutation or another fixture. Shared copies stay upstream.
# Each entry selects source path, scratch variable and repository-root variable.
fixtures=(
    scripts/ci/test-rust-preparation.sh:fixture:root
    scripts/ci/test-failure-evidence.sh:fixture:root
    scripts/ci/test-fixture-completion.sh:fixture:root
    scripts/ci/test-git-hook.sh:temporary_root:repository_root
    scripts/ci/test-repository-checks.sh:temporary_root:repository_root
    scripts/ci/test-testkit-server.sh:fixture:root
    scripts/release/test-committed-release.sh:fixture:root
    scripts/release/test-release-gate.sh:temporary_root:repository_root
    scripts/release/test-release-impact.sh:temporary_root:repository_root
    scripts/release/test-release-index.sh:fixture:root
    scripts/release/test-tag-at-head.sh:temporary_root:repository_root
    scripts/release/test-version-preparation.sh:temporary_root:repository_root
)
mkdir "$fixture/owned scratch"
ln -s "$fixture/owned scratch" "$fixture/aliased scratch"
relative_scratch="$(perl -MFile::Spec -e 'print File::Spec->abs2rel($ARGV[0], $ARGV[1])' "$fixture/owned scratch" "$root")"
for scratch in "$fixture/owned scratch" "$relative_scratch" "$fixture/aliased scratch"; do
    for entry in "${fixtures[@]}"; do
        root_variable="${entry##*:}"
        subject="${entry%:*}"
        source_path="${subject%:*}"
        variable="${subject##*:}"
        # Local rosters and full-body completion remain consumer assertions.
        grep -Fxq 'fixture_complete=true' "$root/$source_path"
        (cd "$root" && CDPATH="$fixture" \
            FIXTURE_PROBE_ROOT_VARIABLE="$root_variable" \
            FIXTURE_PROBE_DIRECTORY_VARIABLE="$variable" \
            FIXTURE_PROBE_TEMPORARY_ROOT="$scratch" \
            fixture_probe_exits "$root/$source_path" "$fixture" "$root" "${source_path##*/}") || exit 1
    done
done
# A completion marker cannot detect an assertion that Bash 3.2 silently skips.
# Copy actual mandatory assertions into this fixture's initialization boundary,
# make their inputs contradictory, and attempt completion only after the check.
for assertion in retained-path status-match host-os host-architecture host-version; do
    # shellcheck disable=SC2016 # Match literal source expressions in the copied assertions.
    case "$assertion" in
        retained-path)
            assertion_source=scripts/ci/fixture-retention-probes.sh
            assertion_prefix='[[ -n "$retained" ]]'
            ;;
        status-match)
            assertion_source=scripts/ci/fixture-retention-probes.sh
            assertion_prefix='[[ "$status" == "$expected" &&'
            ;;
        host-os)
            assertion_source=.github/workflows/ci.yml
            assertion_prefix='[[ "$host_os" == Darwin ]]'
            ;;
        host-architecture)
            assertion_source=.github/workflows/ci.yml
            assertion_prefix='[[ "$host_arch" == "$EXPECTED_ARCHITECTURE" ]]'
            ;;
        host-version)
            assertion_source=.github/workflows/ci.yml
            assertion_prefix='[[ "$host_version" == 15.* ]]'
            ;;
    esac
    # shellcheck disable=SC2016 # Contradictory inputs belong to the child probe.
    fixture_probe_assertion "$root/scripts/ci/test-fixture-completion.sh" "$fixture" "$root" "$assertion" \
        --selected "$root/$assertion_source" "$assertion_prefix" \
        'retained=""; status=0; expected=1; host_os=Linux; host_arch=wrong; EXPECTED_ARCHITECTURE=arm64; host_version=14.0' || exit 1
done
fixture_complete=true
echo 'Local fixture completion, mandatory assertions, failure status and evidence retention checks passed'
