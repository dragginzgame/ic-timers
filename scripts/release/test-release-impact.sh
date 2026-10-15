#!/usr/bin/env bash
set -euo pipefail
repository_root="$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timers-impact-adapter.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed release-impact fixture retained"' EXIT
mkdir -p "$temporary_root/consumer with spaces/scripts/release" "$temporary_root/consumer with spaces/scripts/ci" "$temporary_root/caller"
cp "$repository_root/scripts/release/classify-release-impact.sh" "$temporary_root/consumer with spaces/scripts/release/"
collector="$temporary_root/consumer with spaces/scripts/ci/collect-release-changes.sh"
cat > "$collector" <<'STUB'
#!/usr/bin/env bash
printf '%s\0' "$PWD" "$@" > "$IMPACT_CALL"
cat "$IMPACT_PATHS"
exit "$IMPACT_STATUS"
STUB
export IMPACT_CALL="$temporary_root/call" IMPACT_PATHS="$temporary_root/paths" IMPACT_STATUS=0
classifier="$temporary_root/consumer with spaces/scripts/release/classify-release-impact.sh"
: > "$IMPACT_PATHS"
output="$(cd "$temporary_root/caller" && bash "$classifier")"
[[ "$output" == none ]] || exit 1
printf '%s\0' "$temporary_root/caller" > "$temporary_root/expected-call"
cmp "$temporary_root/expected-call" "$IMPACT_CALL"
for path in 'docs/note.md' 'testing/probe/src/lib.rs' 'crates/ic-timers/src-other/lib.rs' \
    Cargo.toml crates/ic-timers/Cargo.toml crates/ic-timers/build.rs $'crates/ic-timers/src/line\nname.rs'; do
    printf '%s\0' "$path" > "$IMPACT_PATHS"
    case "$path" in
        docs/*|testing/*|crates/ic-timers/src-other/*) expected=repository ;;
        *) expected=crate ;;
    esac
    output="$(cd "$temporary_root/caller" && bash "$classifier" v0.3.8)"
    [[ "$output" == "$expected" ]] || exit 1
    printf '%s\0' "$temporary_root/caller" v0.3.8 > "$temporary_root/expected-call"
    cmp "$temporary_root/expected-call" "$IMPACT_CALL"
done
# Both empty and plausible failed streams must refuse rather than emit policy.
for partial in false true; do
    : > "$IMPACT_PATHS"
    if [[ "$partial" == true ]]; then printf '%s\0' crates/ic-timers/src/lib.rs > "$IMPACT_PATHS"; fi
    status=0
    IMPACT_STATUS=37 bash "$classifier" > "$temporary_root/output" || status=$?
    [[ "$status" == 37 && ! -s "$temporary_root/output" ]] || exit 1
done
# A crate path cannot stop draining the owner before it reports final success.
printf '%s\0' Cargo.toml docs/note.md > "$IMPACT_PATHS"
[[ "$(bash "$classifier")" == crate ]] || exit 1
bash "$repository_root/scripts/ci/test-release-changes.sh"
fixture_complete=true
echo 'Release-impact local policy, owner binding and failure propagation checks passed'
