#!/usr/bin/env bash
set -euo pipefail

# CI owns the checkout and scratch directory. Archive only the maintained
# evidence roots; the shared archiver owns modes, link handling and Git exclusion.
[[ $# == 0 ]] || { echo 'usage: collect-failure-evidence.sh' >&2; exit 2; }
: "${GITHUB_WORKSPACE:?CI collection requires GITHUB_WORKSPACE}"
: "${RUNNER_TEMP:?CI collection requires RUNNER_TEMP}"
: "${GITHUB_SHA:?CI collection requires GITHUB_SHA}"
root="$GITHUB_WORKSPACE"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "$root" && printf '%s/.' "$PWD")"
root="${root%/.}"
script_root="${BASH_SOURCE[0]}"
[[ "$script_root" == /* ]] || script_root="$PWD/$script_root"
script_root="$(cd -P "${script_root%/*}/../.." && printf '%s/.' "$PWD")"
script_root="${script_root%/.}"
temporary="$RUNNER_TEMP"
[[ "$temporary" == /* ]] || temporary="$PWD/$temporary"
temporary="$(cd -P "$temporary" && printf '%s/.' "$PWD")"
temporary="${temporary%/.}"
metadata="$(mktemp -d "$temporary/ic-timers-evidence.XXXXXX")"
# Bash 3.2 can report zero after a fatal expansion. Only a completed collection
# may remove its scratch evidence and return success.
collection_complete=false
finish() {
    local status=$?
    [[ "$collection_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "$metadata"
    else printf 'Failed evidence collection retained: %s\n' "$metadata" >&2; fi
    exit "$status"
}
trap finish EXIT
checkout_sha="$(git -C "$root" rev-parse HEAD)"
printf 'checkout_sha=%s\nevent_sha=%s\njob=%s\nhost=%s/%s\nrun=%s\nattempt=%s\n' \
    "$checkout_sha" "$GITHUB_SHA" "${GITHUB_JOB:-unknown}" \
    "${RUNNER_OS:-unknown}" "${RUNNER_ARCH:-unknown}" \
    "${GITHUB_RUN_ID:-unknown}" "${GITHUB_RUN_ATTEMPT:-unknown}" > "$metadata/identity.txt"
archive="$temporary/ic-timers-failure-evidence.tar.gz"
selections=("$metadata" identity.txt)

if [[ -d "$temporary/ic-timers-fixtures" ]]; then
    selections+=("$temporary" ic-timers-fixtures)
fi
# The canonical formatter inherits RUNNER_TEMP in CI. Retain complete failed
# stdout/stderr logs from that selected root; successful logs remove themselves.
for log in "$temporary"/formatting.* "$temporary"/tools-*.log; do
    [[ -f "$log" && ! -L "$log" ]] || continue
    selections+=("$temporary" "${log#"$temporary"/}")
done
# Select the raw release logs without admitting the surrounding Git metadata.
if [[ -d "$root/.git/release-state/validation-failures" ]]; then
    selections+=("$root/.git/release-state" validation-failures)
fi
if [[ -d "$root/target/validation-failures" ]]; then
    selections+=("$root" target/validation-failures)
fi
# Preserve Testkit provisioning attempts, never its admitted server bundles.
# Testkit owns the attempt prefix and preserves those directories.
if [[ ! -L "$root/.tools" && ! -L "$root/.tools/testkit-server" ]]; then
    for attempt in "$root"/.tools/testkit-server/.setup-v1-*; do
        [[ -d "$attempt" && ! -L "$attempt" ]] || continue
        selections+=("$root" "${attempt#"$root"/}")
    done
fi
# Shared owns tool admission, safe partial retention and archive orchestration.
# Timers selects host-tool compaction. Unselected legacy IC sets remain full
# evidence through the Shared selector, without IC admission or provisioning.
mkdir "$metadata/tool-evidence"
archive_started=$SECONDS
collector_status=0
bash "$script_root/scripts/ci/collect-tool-evidence.sh" "$archive" "$root" \
    "$metadata/tool-evidence" compact "$root/ci/tool-versions.env" "" \
    "${selections[@]}" > "$metadata/archive-path" || collector_status=$?
case "$collector_status" in
    0|1) ;;
    2)
        archive_status="$(cat "$metadata/tool-evidence/archive-status")" || exit 2
        [[ "$archive_status" =~ ^[1-9][0-9]*$ ]] || exit 2
        exit "$archive_status" ;;
    *) exit "$collector_status" ;;
esac
selection_status="$(cat "$metadata/tool-evidence/selection-status")"
[[ "$selection_status" =~ ^[0-9]+$ ]] || exit 2
archive_seconds=$((SECONDS - archive_started))
archive_bytes="$(wc -c < "$archive")"
archive_bytes="${archive_bytes//[[:space:]]/}"
if [[ "$selection_status" != 0 ]]; then
    printf 'Partial CI failure evidence archived at: %s; shared collection incomplete (exit %s)\n' \
        "$archive" "$selection_status" >&2
    exit "$selection_status"
fi
printf 'CI failure evidence measurements: archive_bytes=%s archive_seconds=%s\n' \
    "$archive_bytes" "$archive_seconds"
printf 'CI failure evidence archived at: %s\n' "$archive"
collection_complete=true
