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
for relative_payload in DRAGGINZGAME.md audits/README.md scripts/ci/dependency-pins.jq; do
    payload="$consumer/$relative_payload"
    cp -p "$payload" "$consumer/saved-payload"
    printf '\nchanged\n' >> "$payload"
    if bash "$verifier" --consumer "$consumer" > "$consumer/payload.log" 2>&1; then
        echo "Snapshot accepted consumer payload corruption: $relative_payload" >&2
        exit 1
    fi
    cp -p "$consumer/saved-payload" "$payload"
    bash "$verifier" --consumer "$consumer" > "$consumer/restored.log" 2>&1
done

fixture_complete=true
echo 'Consumer snapshot export and payload corruption checks passed'
