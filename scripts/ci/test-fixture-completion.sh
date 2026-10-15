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
    scripts/ci/test-failure-evidence.sh:fixture:root
    scripts/ci/test-fixture-completion.sh:fixture:root
    scripts/ci/test-git-hook.sh:temporary_root:repository_root
    scripts/ci/test-repository-checks.sh:temporary_root:repository_root
    scripts/ci/test-shared-snapshots.sh:fixture:root
    scripts/ci/test-testkit-server.sh:fixture:root
    scripts/release/test-committed-release.sh:fixture:root
    scripts/release/test-finalize-changelog.sh:temporary_root:repository_root
    scripts/release/test-lockfiles.sh:temporary_root:repository_root
    scripts/release/test-release-gate.sh:temporary_root:repository_root
    scripts/release/test-release-impact.sh:temporary_root:repository_root
    scripts/release/test-release-index.sh:fixture:root
    scripts/release/test-release-prose-warning.sh:fixture:repository_root
    scripts/release/test-tag-at-head.sh:temporary_root:repository_root
    scripts/release/test-version-preparation.sh:temporary_root:repository_root
)
mkdir -p "$fixture/probe/scripts/ci" "$fixture/probe/scripts/release" "$fixture/owned scratch"
RETENTION_EXIT_PATH="$fixture/exit-path"
export RETENTION_EXIT_PATH RETENTION_SOURCE_ROOT="$root"
ln -s "$fixture/owned scratch" "$fixture/aliased scratch"
relative_scratch="$(perl -MFile::Spec -e 'print File::Spec->abs2rel($ARGV[0], $ARGV[1])' "$fixture/owned scratch" "$root")"
for scratch in "$fixture/owned scratch" "$relative_scratch" "$fixture/aliased scratch"; do
    for entry in "${fixtures[@]}"; do
        root_variable="${entry##*:}"
        subject="${entry%:*}"
        source_path="${subject%:*}"
        variable="${subject##*:}"
        # Canonical completed bodies must opt in after their required assertions.
        grep -Fxq 'fixture_complete=true' "$root/$source_path"
        for failure in nounset command nonzero premature completed failed-completion; do
            # shellcheck disable=SC2016 # Expanded only by the disposable prefix copy.
            case "$failure" in
                nounset) injection='unset FIXTURE_UNBOUND; printf "%s\n" "$FIXTURE_UNBOUND"'; expected=1 ;;
                command) injection='false'; expected=1 ;;
                nonzero) injection='exit 23'; expected=23 ;;
                premature) injection='exit 0'; expected=1 ;;
                completed) injection='fixture_complete=true; exit 0'; expected=0 ;;
                failed-completion) injection='fixture_complete=true; exit 23'; expected=23 ;;
            esac
            probe="$fixture/probe/$source_path"
            fixture_probe_prefix "$root/$source_path" "$probe" "$root_variable" "$variable" "$injection"
            rm -f -- "$RETENTION_EXIT_PATH"
            status=0
            (cd "$root" && CDPATH="$fixture" TMPDIR="$scratch" "$BASH" "$probe") \
                > "$fixture/${source_path##*/}-$failure.log" 2>&1 || status=$?
            if [[ "$status" != "$expected" ]]; then
                printf '%s: %s returned %s, expected %s\n' "$source_path" "$failure" "$status" "$expected" >&2
                exit 1
            fi
            retained="$(cat "$RETENTION_EXIT_PATH")"
            [[ -n "$retained" ]] || exit 1
            [[ "$retained" == "$fixture/owned scratch/"* ]] || exit 1
            if [[ "$expected" == 0 ]]; then [[ ! -e "$retained" ]] || exit 1
            else
                [[ -d "$retained" && "$(cat "$retained/exit-evidence")" == 'retained evidence' ]] || exit 1
            fi
        done
    done
done
# A completion marker cannot detect an assertion that Bash 3.2 silently skips.
# Copy actual mandatory assertions into this fixture's initialization boundary,
# make their inputs contradictory, and attempt completion only after the check.
for assertion in retained-path status-match host-os host-architecture host-version; do
    # shellcheck disable=SC2016 # Match literal source expressions in the copied assertions.
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
    # shellcheck disable=SC2016 # Contradictory inputs belong to the child probe.
    fixture_probe_assertion "$root/scripts/ci/test-fixture-completion.sh" "$fixture" "$root" "$assertion" \
        --selected "$root/$assertion_source" "$assertion_prefix" \
        'retained=""; status=0; expected=1; host_os=Linux; host_arch=wrong; EXPECTED_ARCHITECTURE=arm64; host_version=14.0' || exit 1
done
fixture_complete=true
echo 'Local fixture completion, mandatory assertions, failure status and evidence retention checks passed'
