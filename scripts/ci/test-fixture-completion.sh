#!/usr/bin/env bash
set -euo pipefail

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-fixture-completion.XXXXXX")"
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
# shellcheck source=scripts/ci/fixture-retention-probes.sh
source "$root/scripts/ci/fixture-retention-probes.sh"
trap 'fixture_finish "$?" "$fixture" "Failed fixture-completion checks retained"' EXIT

# Copy each actual initialization/EXIT boundary, including this suite's own.
# Stop before the fixture body so these probes cannot dispatch tools, builds,
# release helpers, Git mutation or another fixture. Shared copies stay upstream.
fixtures=(
    scripts/ci/test-failure-evidence.sh:root:fixture
    scripts/ci/test-fixture-completion.sh:root:fixture
    scripts/ci/test-git-hook.sh:repository_root:temporary_root
    scripts/ci/test-repository-checks.sh:repository_root:temporary_root
    scripts/ci/test-shared-snapshots.sh:root:fixture
    scripts/ci/test-testkit-server.sh:root:fixture
    scripts/release/test-committed-release.sh:root:fixture
    scripts/release/test-finalize-changelog.sh:repository_root:temporary_root
    scripts/release/test-lockfiles.sh:repository_root:temporary_root
    scripts/release/test-release-gate.sh:repository_root:temporary_root
    scripts/release/test-release-impact.sh:repository_root:temporary_root
    scripts/release/test-release-index.sh:root:fixture
    scripts/release/test-release-prose-warning.sh:repository_root:fixture_root
    scripts/release/test-tag-at-head.sh:repository_root:temporary_root
    scripts/release/test-version-preparation.sh:repository_root:temporary_root
)
mkdir -p "$fixture/probe/scripts/ci" "$fixture/owned scratch"
cp -p "$root/scripts/ci/fixture-lifecycle.sh" "$fixture/probe/scripts/ci/"
export RETENTION_EXIT_PATH="$fixture/exit-path" RETENTION_SOURCE_ROOT="$root"
for entry in "${fixtures[@]}"; do
    source_path="${entry%%:*}"
    binding="${entry#*:}"
    root_variable="${binding%:*}"
    variable="${entry##*:}"
    # Canonical completed bodies must opt in after their required assertions.
    grep -Fxq 'fixture_complete=true' "$root/$source_path"
    FIXTURE_PROBE_ROOT_VARIABLE="$root_variable" FIXTURE_PROBE_DIRECTORY_VARIABLE="$variable" \
        fixture_probe_exits "$root/$source_path" "$fixture/owned scratch" "$root" \
        "${source_path##*/}" || exit 1
done
# A completion marker cannot detect an assertion that Bash 3.2 silently skips.
# Copy actual mandatory assertions into this fixture's initialization boundary,
# make their inputs contradictory, and attempt completion only after the check.
# shellcheck disable=SC2016 # Match literal source assertions, not this shell's values.
for assertion in retained-path status-match host-os host-architecture host-version; do
    case "$assertion" in
        retained-path)
            assertion_source=scripts/ci/test-fixture-completion.sh
            assertion_prefix='[[ -n "$retained" ]]'
            ;;
        status-match)
            assertion_source=scripts/ci/test-failure-evidence.sh
            assertion_prefix='[[ "$status" == "$expected" ]]'
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
    assertion_line="$(FIXTURE_ASSERTION="$assertion_prefix" awk '
        { line=$0; sub(/^[[:space:]]*/, "", line) }
        index(line, ENVIRON["FIXTURE_ASSERTION"]) == 1 { print; found++ }
        END { if (found != 1) exit 1 }
    ' "$root/$assertion_source")" || exit 1
    printf -v injection 'retained=""; status=0; expected=1; host_os=Linux; host_arch=wrong; EXPECTED_ARCHITECTURE=arm64; host_version=14.0\n%s\nfixture_complete=true; exit 0' "$assertion_line"
    probe="$fixture/probe/scripts/ci/contradictory-assertion.sh"
    fixture_probe_prefix "$root/scripts/ci/test-fixture-completion.sh" "$probe" root fixture "$injection"
    rm -f -- "$RETENTION_EXIT_PATH"
    status=0
    TMPDIR="$fixture/owned scratch" "$BASH" "$probe" \
        > "$fixture/contradictory-$assertion.log" 2>&1 || status=$?
    [[ "$status" == 1 ]] || exit 1
    retained="$(cat "$RETENTION_EXIT_PATH")"
    [[ -n "$retained" ]] || exit 1
    [[ -d "$retained" && "$(cat "$retained/exit-evidence")" == 'retained evidence' ]] || exit 1
    grep -Fq "Failed fixture-completion checks retained: $retained" "$fixture/contradictory-$assertion.log"
done
fixture_complete=true
echo 'Local fixture completion, mandatory assertions, failure status and evidence retention checks passed'
