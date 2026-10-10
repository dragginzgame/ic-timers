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
        [[ -n "$retained" ]]
        if [[ "$expected" == 0 ]]; then [[ ! -e "$retained" ]]
        else
            [[ -d "$retained" && "$(cat "$retained/exit-evidence")" == evidence ]]
        fi
    done
done
fixture_complete=true
echo 'Local fixture completion, failure status and evidence retention checks passed'
