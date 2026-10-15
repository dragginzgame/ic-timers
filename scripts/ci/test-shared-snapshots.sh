#!/usr/bin/env bash
set -euo pipefail

# Test consumer exports, never corrupt the real checkout or execute changed code.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/timer-snapshot-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Snapshot fixtures retained"' EXIT
verifier="$root/scripts/ci/verify-shared-tooling-snapshot.sh"
consumer="$fixture/consumer"
mkdir -p "$consumer"
manifest="$root/.shared-tooling.snapshot"
cp "$manifest" "$consumer/.shared-tooling.snapshot"
while IFS=$'\t' read -r record _digest _mode path; do
    [[ "$record" == file ]] || continue
    mkdir -p "$consumer/$(dirname "$path")"
    cp -p "$root/$path" "$consumer/$path"
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
    alias="$fixture/alias"
    ln -s "$forbidden" "$alias"
    for selected in "$forbidden" "$alias"; do
        status=0
        CDPATH="$fixture" bash "$verifier" --consumer "$selected" \
            > "$consumer/forbidden-root.log" 2>&1 || status=$?
        if [[ "$status" != 1 ]]; then
            cat "$consumer/forbidden-root.log" >&2
            echo "Snapshot accepted a forbidden directory name" >&2
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
alias="$fixture/alias"
ln -s "$consumer" "$alias"
(
    cd "$fixture"
    CDPATH="$fixture" bash "$verifier" --consumer "alias"
) > "$consumer/alias.log" 2>&1
cmp "$consumer/saved-manifest" "$consumer/.shared-tooling.snapshot"
rm "$alias"
for relative_payload in DRAGGINZGAME.md audits/README.md scripts/ci/dependency-pins.jq; do
    payload="$consumer/$relative_payload"
    checksum="$consumer/scripts/ci/verify-file-checksum.sh"
    cp -p "$payload" "$consumer/saved-payload"
    cp -p "$checksum" "$consumer/saved-checksum"
    export SNAPSHOT_HELPER_MARKER="$consumer/helper-executed"
    for corruption in payload helper both; do
        cp -p "$consumer/saved-payload" "$payload"
        cp -p "$consumer/saved-checksum" "$checksum"
        if [[ "$corruption" != helper ]]; then printf '\nchanged\n' >> "$payload"; fi
        if [[ "$corruption" != payload ]]; then
            # shellcheck disable=SC2016 # Expanded only by the disposable inspected helper.
            printf '%s\n' '#!/usr/bin/env bash' \
                'printf executed > "$SNAPSHOT_HELPER_MARKER"' 'exit 0' > "$checksum"
        fi
        if bash "$verifier" --consumer "$consumer" > "$consumer/$corruption.log" 2>&1; then
            echo "Snapshot accepted $corruption corruption" >&2
            exit 1
        fi
        if [[ -e "$SNAPSHOT_HELPER_MARKER" ]]; then
            echo "Snapshot executed the inspected helper" >&2
            exit 1
        fi
    done
    cp -p "$consumer/saved-payload" "$payload"
    cp -p "$consumer/saved-checksum" "$checksum"
    bash "$verifier" --consumer "$consumer" > "$consumer/restored.log" 2>&1
done

fixture_complete=true
echo 'Consumer snapshot exports, directory admission and corruption refusals passed'
