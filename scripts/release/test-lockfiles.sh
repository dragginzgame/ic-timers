#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
checker="${repository_root}/scripts/release/check-lockfiles.sh"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-lockfile-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$temporary_root" "Failed lockfile fixture retained"' EXIT
temporary_root="$(cd "$temporary_root" && pwd -P)"

# Generic Cargo graph cases belong to Shared's metadata owner suite. This
# fixture qualifies the local adapter's helper selection, context and status.
consumer="${temporary_root}/consumer"
caller="${temporary_root}/caller"
mkdir -p "$consumer/scripts/release" "$consumer/.shared-tooling/helpers/scripts/ci" "$caller"
cp "$checker" "$consumer/scripts/release/check-lockfiles.sh"
helper="$consumer/.shared-tooling/helpers/scripts/ci/cargo-release-metadata.sh"
cat > "$helper" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "$#" == 2 && "$1" == check-graph && "$2" == ic-timers ]]
[[ "${RELEASE_COMMIT+x}" == x && -z "$RELEASE_COMMIT" ]]
[[ "$PWD" == "$EXPECTED_CALLER" && "${BASH_SOURCE[0]}" == "$EXPECTED_HELPER" ]]
printf '%s\n' "$PWD" "$1" "$2" > "$ADAPTER_LOG"
echo 'selected graph adapter helper' >&2
exit "$ADAPTER_STATUS"
EOF
printf 'fixture manifest\n' > "$caller/Cargo.toml"
printf 'fixture lock\n' > "$caller/Cargo.lock"
for file in Cargo.toml Cargo.lock; do cp "$caller/$file" "$temporary_root/$file.before"; done
for expected_status in 0 37; do
    observed_status=0
    (cd "$caller" && RELEASE_COMMIT=unrelated-saved-release \
        EXPECTED_CALLER="$caller" EXPECTED_HELPER="$helper" \
        ADAPTER_LOG="$temporary_root/events" ADAPTER_STATUS="$expected_status" \
        bash "$consumer/scripts/release/check-lockfiles.sh") \
        > "$temporary_root/output" 2>&1 || observed_status=$?
    [[ "$observed_status" == "$expected_status" ]] || {
        cat "$temporary_root/output" >&2
        echo 'error: graph adapter changed helper status or selected the wrong context' >&2
        exit 1
    }
    printf '%s\n' "$caller" check-graph ic-timers > "$temporary_root/expected-events"
    cmp "$temporary_root/expected-events" "$temporary_root/events"
    grep -q 'selected graph adapter helper' "$temporary_root/output"
    for file in Cargo.toml Cargo.lock; do cmp "$caller/$file" "$temporary_root/$file.before"; done
done
# Keep one actual selected-worktree smoke check, using the adopted helper and
# full five-member graph rather than a second implementation-level toy graph.
(cd "$repository_root" && RELEASE_COMMIT=unrelated-saved-release bash "$checker")
fixture_complete=true
echo 'Locked workspace metadata adapter checks passed'
