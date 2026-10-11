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
# The shared selector owns local Rust evidence and referenced shared generations,
# and freshly checks the exact managed host/IC selections with our pins.
# Failed, changed and unselected bundles stay full; verified sets retain checks
# and receipts. Keep selection output on disk so producer failure cannot be lost
# in a process substitution or mistaken for an empty successful selection.
mkdir "$metadata/tool-evidence"
selection_status=0
bash "$script_root/scripts/ci/select-tool-evidence.sh" compact "$root" \
    "$metadata/tool-evidence" "$root/ci/tool-versions.env" "$root/ci/ic-tools.tsv" \
    > "$metadata/tool-selections.nul" 2> "$metadata/tool-evidence/selection.log" || selection_status=$?
cat "$metadata/tool-evidence/selection.log" >&2
if [[ "$selection_status" == 0 ]]; then
    while IFS= read -r -d '' selected_root && IFS= read -r -d '' selected_path; do
        selections+=("$selected_root" "$selected_path")
    done < "$metadata/tool-selections.nul"
else
    # An unadmitted store must not prevent safe local evidence retention. Ignore
    # all partial selector output; preserve local references without traversing
    # them. Only the shared owner may admit physical store payloads.
    printf 'status=%s\nshared_collection=incomplete\n' "$selection_status" \
        > "$metadata/tool-evidence/selection-status.txt"
    cp -p "$metadata/tool-selections.nul" "$metadata/tool-evidence/partial-selections.nul"
    if [[ ! -L "$root/.tools" && -d "$root/.tools" ]]; then
        if [[ ! -L "$root/.tools/rust" && -d "$root/.tools/rust" ]]; then
            selections+=("$root" .tools/rust)
        fi
        for candidate in "$root"/.tools/host "$root"/.tools/ic \
            "$root"/.tools/host-set.* "$root"/.tools/ic-set.*; do
            [[ -e "$candidate" || -L "$candidate" ]] || continue
            selections+=("$root" "${candidate#"$root"/}")
        done
    fi
fi
if [[ "$selection_status" != 0 || -d "$metadata/tool-evidence/host" || -d "$metadata/tool-evidence/ic" ]]; then
    selections+=("$metadata" tool-evidence)
fi
archive_started=$SECONDS
bash "$script_root/scripts/ci/archive-evidence.sh" "$archive" "${selections[@]}"
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
