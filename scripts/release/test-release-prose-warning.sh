#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timers-prose-adapter.XXXXXX")"
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$fixture" "Failed prose-adapter fixture retained"' EXIT
mkdir -p "$fixture/consumer with spaces/scripts/release" "$fixture/consumer with spaces/scripts/ci" "$fixture/caller"
cp "$repository_root/scripts/release/warn-release-prose.sh" "$fixture/consumer with spaces/scripts/release/"
cat > "$fixture/consumer with spaces/scripts/ci/warn-release-prose.sh" <<'STUB'
#!/usr/bin/env bash
printf '%s\0' "$PWD" "$@" > "$PROSE_ADAPTER_CALL"
exit "$PROSE_ADAPTER_STATUS"
STUB
export PROSE_ADAPTER_CALL="$fixture/call" PROSE_ADAPTER_STATUS=0
(
    cd "$fixture/caller"
    bash "$fixture/consumer with spaces/scripts/release/warn-release-prose.sh" 0.22.5 'status notes.md'
)
printf '%s\0' "$fixture/caller" 0.22.5 'status notes.md' > "$fixture/expected"
cmp "$fixture/expected" "$fixture/call"
status=0
PROSE_ADAPTER_STATUS=37 bash "$fixture/consumer with spaces/scripts/release/warn-release-prose.sh" 0.22.5 || status=$?
[[ "$status" == 37 ]] || exit 1
printf '%s\0' "$PWD" 0.22.5 docs/status/current.md > "$fixture/expected"
cmp "$fixture/expected" "$fixture/call"
# Shared owns prose heuristics and failure behavior; qualify its exact export.
bash "$repository_root/scripts/ci/test-release-prose-warning.sh"
fixture_complete=true
echo 'Release-prose adapter binding and owner checks passed'
