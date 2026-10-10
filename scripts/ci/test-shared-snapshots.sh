#!/usr/bin/env bash
set -euo pipefail

# Test consumer exports, never corrupt the real checkout or execute changed code.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
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
    # The trusted verifier must not trim a forbidden root into this valid
    # neighbor. Such directory names are deliberate negative fixtures only.
    cp -p "$consumer/.shared-tooling.snapshot" "$consumer/saved-manifest"
    for suffix in $'\n' $'\r' $'\r\n'; do
        forbidden="$consumer$suffix"
        mkdir "$forbidden"
        # LF tests the missing-manifest/trimmed-neighbor case. CR variants
        # contain valid payloads, so admission must refuse the path itself.
        if [[ "$suffix" != $'\n' ]]; then cp -R "$consumer/." "$forbidden/"; fi
        alias="$fixture/alias-$index"
        ln -s "$forbidden" "$alias"
        for selected in "$forbidden" "$alias"; do
            status=0
            CDPATH="$fixture" bash "$verifier" --consumer "$selected" \
                > "$consumer/forbidden-root.log" 2>&1 || status=$?
            if [[ "$status" != 1 ]]; then
                cat "$consumer/forbidden-root.log" >&2
                echo "Snapshot $index accepted a forbidden directory name" >&2
                exit 1
            fi
            if [[ "$suffix" == $'\n' ]]; then
                test ! -e "$forbidden/.shared-tooling.snapshot"
            else
                cmp "$consumer/saved-manifest" "$forbidden/.shared-tooling.snapshot"
            fi
            cmp "$consumer/saved-manifest" "$consumer/.shared-tooling.snapshot"
        done
        rm "$alias"
    done
    # Ordinary aliases and relative inputs still select the exact snapshot,
    # including when the caller has CDPATH set.
    alias="$fixture/alias-$index"
    ln -s "$consumer" "$alias"
    (
        cd "$fixture"
        CDPATH="$fixture" bash "$verifier" --consumer "alias-$index"
    ) > "$consumer/alias.log" 2>&1
    cmp "$consumer/saved-manifest" "$consumer/.shared-tooling.snapshot"
    rm "$alias"
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

echo 'Consumer snapshot exports, directory admission and corruption refusals passed'
