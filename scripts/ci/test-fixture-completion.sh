#!/usr/bin/env bash
set -euo pipefail

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-fixture-completion.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "$fixture"
    else printf 'Failed fixture-completion checks retained: %s\n' "$fixture" >&2; fi
    exit "$status"
}
trap finish EXIT

# Copy each actual initialization/EXIT boundary, including this suite's own.
# Stop before the fixture body so these probes cannot dispatch tools, builds,
# release helpers, Git mutation or another fixture. Shared copies stay upstream.
fixtures=(
    scripts/ci/test-failure-evidence.sh:fixture
    scripts/ci/test-fixture-completion.sh:fixture
    scripts/ci/test-git-hook.sh:temporary_root
    scripts/ci/test-repository-checks.sh:temporary_root
    scripts/ci/test-shared-snapshots.sh:fixture
    scripts/ci/test-testkit-server.sh:fixture
    scripts/release/test-commit-release.sh:temporary_root
    scripts/release/test-committed-release.sh:fixture
    scripts/release/test-finalize-changelog.sh:temporary_root
    scripts/release/test-lockfiles.sh:temporary_root
    scripts/release/test-release-gate.sh:temporary_root
    scripts/release/test-release-impact.sh:temporary_root
    scripts/release/test-release-index.sh:fixture
    scripts/release/test-release-prose-warning.sh:fixture_root
    scripts/release/test-tag-at-head.sh:temporary_root
    scripts/release/test-version-preparation.sh:temporary_root
)
mkdir -p "$fixture/probe/scripts/ci" "$fixture/probe/scripts/release" "$fixture/owned scratch"
export FIXTURE_EXIT_PATH="$fixture/exit-path"
for entry in "${fixtures[@]}"; do
    source_path="${entry%:*}"
    variable="${entry##*:}"
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
        FIXTURE_INJECTION="$injection" FIXTURE_VARIABLE="$variable" awk '
            { print }
            /^trap finish EXIT$/ {
                print "printf \"%s\\n\" \"$" ENVIRON["FIXTURE_VARIABLE"] "\" > \"$FIXTURE_EXIT_PATH\""
                print "printf evidence > \"$" ENVIRON["FIXTURE_VARIABLE"] "/exit-evidence\""
                print ENVIRON["FIXTURE_INJECTION"]
                print "exit 99"
                injected=1
                exit
            }
            END { if (!injected) exit 1 }
        ' "$root/$source_path" > "$probe"
        rm -f -- "$FIXTURE_EXIT_PATH"
        status=0
        TMPDIR="$fixture/owned scratch" "$BASH" "$probe" \
            > "$fixture/${source_path##*/}-$failure.log" 2>&1 || status=$?
        if [[ "$status" != "$expected" ]]; then
            printf '%s: %s returned %s, expected %s\n' "$source_path" "$failure" "$status" "$expected" >&2
            exit 1
        fi
        retained="$(cat "$FIXTURE_EXIT_PATH")"
        [[ -n "$retained" ]] || exit 1
        if [[ "$expected" == 0 ]]; then [[ ! -e "$retained" ]] || exit 1
        else
            [[ -d "$retained" && "$(cat "$retained/exit-evidence")" == evidence ]] || exit 1
        fi
    done
done
# A completion marker cannot detect an assertion that Bash 3.2 silently skips.
# Copy actual mandatory assertions into this fixture's initialization boundary,
# make their inputs contradictory, and attempt completion only after the check.
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
    FIXTURE_INJECTION="$injection" awk '
        { print }
        /^trap finish EXIT$/ {
            print "printf \"%s\\n\" \"$fixture\" > \"$FIXTURE_EXIT_PATH\""
            print "printf evidence > \"$fixture/exit-evidence\""
            print ENVIRON["FIXTURE_INJECTION"]
            injected=1
            exit
        }
        END { if (!injected) exit 1 }
    ' "$root/scripts/ci/test-fixture-completion.sh" > "$probe"
    rm -f -- "$FIXTURE_EXIT_PATH"
    status=0
    TMPDIR="$fixture/owned scratch" "$BASH" "$probe" \
        > "$fixture/contradictory-$assertion.log" 2>&1 || status=$?
    [[ "$status" == 1 ]] || exit 1
    retained="$(cat "$FIXTURE_EXIT_PATH")"
    [[ -n "$retained" && -d "$retained" && "$(cat "$retained/exit-evidence")" == evidence ]] || exit 1
    grep -Fq "Failed fixture-completion checks retained: $retained" "$fixture/contradictory-$assertion.log"
done
fixture_complete=true
echo 'Local fixture completion, mandatory assertions, failure status and evidence retention checks passed'
