#!/usr/bin/env bash
set -euo pipefail

# Test consumer exports, never corrupt the real checkout or execute changed code.
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-snapshot-test.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf "$fixture"; else printf "Snapshot fixtures retained: %s\n" "$fixture" >&2; fi' EXIT
verifier="$root/scripts/ci/verify-shared-tooling-snapshot.sh"
sources=("$root" "$root" "$root/.shared-tooling/helpers")
manifests=(.shared-tooling.snapshot .shared-tooling-audits.snapshot .shared-tooling.snapshot)
payloads=(DRAGGINZGAME.md audits/README.md scripts/ci/check-release-commands.sh)

for index in 0 1 2; do
    consumer="$fixture/$index"
    mkdir -p "$consumer"
    manifest="${sources[$index]}/${manifests[$index]}"
    cp "$manifest" "$consumer/.shared-tooling.snapshot"
    while IFS=$'\t' read -r record digest mode path; do
        [[ "$record" == file ]] || continue
        mkdir -p "$consumer/$(dirname "$path")"
        cp -p "${sources[$index]}/$path" "$consumer/$path"
    done < "$manifest"
    if ! bash "$verifier" --consumer "$consumer" > "$consumer/unchanged.log" 2>&1; then
        cat "$consumer/unchanged.log" >&2
        exit 1
    fi
    payload="$consumer/${payloads[$index]}"
    checksum="$consumer/scripts/ci/verify-file-checksum.sh"
    cp -p "$payload" "$consumer/saved-payload"
    cp -p "$checksum" "$consumer/saved-checksum"
    export SNAPSHOT_HELPER_MARKER="$consumer/helper-executed"
    for corruption in payload helper both; do
        cp -p "$consumer/saved-payload" "$payload"
        cp -p "$consumer/saved-checksum" "$checksum"
        if [[ "$corruption" != helper ]]; then printf '\nchanged\n' >> "$payload"; fi
        if [[ "$corruption" != payload ]]; then
            printf '%s\n' '#!/usr/bin/env bash' \
                'printf executed > "$SNAPSHOT_HELPER_MARKER"' 'exit 0' > "$checksum"
        fi
        if bash "$verifier" --consumer "$consumer" > "$consumer/$corruption.log" 2>&1; then
            echo "Snapshot $index accepted $corruption corruption" >&2
            exit 1
        fi
        if [[ -e "$SNAPSHOT_HELPER_MARKER" ]]; then
            echo "Snapshot $index executed the inspected helper" >&2
            exit 1
        fi
    done
done

echo 'Consumer snapshot exports and corruption refusals passed'
