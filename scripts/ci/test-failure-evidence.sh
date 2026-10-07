#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-evidence-test.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf "$fixture"; else printf "Failed evidence fixture retained: %s\n" "$fixture" >&2; fi' EXIT
export GITHUB_WORKSPACE="$fixture/repo" RUNNER_TEMP="$fixture/runner"
export GITHUB_SHA=fixture-event-sha GITHUB_JOB=fixture-job
export RUNNER_OS=fixture-os RUNNER_ARCH=fixture-arch GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=2
mkdir -p "$GITHUB_WORKSPACE" "$RUNNER_TEMP"
# Read-only reuse of committed objects gives the fixture an actual checkout SHA.
objects="$(git rev-parse --git-path objects)"
case "$objects" in /*) ;; *) objects="$root/$objects" ;; esac
git init -q "$GITHUB_WORKSPACE"
mkdir -p "$GITHUB_WORKSPACE/.git/objects/info"
printf '%s\n' "$objects" > "$GITHUB_WORKSPACE/.git/objects/info/alternates"
git -C "$GITHUB_WORKSPACE" update-ref HEAD "$(git rev-parse HEAD)"
scratch="$RUNNER_TEMP/ic-timers-fixtures/failed case"
mkdir -p "$scratch/.git" "$GITHUB_WORKSPACE/.git/release-state/validation-failures" \
    "$GITHUB_WORKSPACE/target/validation-failures" "$GITHUB_WORKSPACE/target/build-cache" \
    "$GITHUB_WORKSPACE/.tools/host-set.failed" "$GITHUB_WORKSPACE/.tools/ic-set.failed" \
    "$GITHUB_WORKSPACE/.tools/host"
printf 'scenario output\n' > "$scratch/scenario.log"
printf 'original metadata\n' > "$scratch/before"
chmod 0640 "$scratch/before"
printf 'fixture Git configuration\n' > "$scratch/.git/config"
printf 'release raw output\n' > "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log"
printf 'validation raw output\n' > "$GITHUB_WORKSPACE/target/validation-failures/raw.log"
printf 'failed host payload\n' > "$GITHUB_WORKSPACE/.tools/host-set.failed/tool"
chmod 0751 "$GITHUB_WORKSPACE/.tools/host-set.failed/tool"
printf 'failed IC payload\n' > "$GITHUB_WORKSPACE/.tools/ic-set.failed/tool"
printf 'successful installed tool\n' > "$GITHUB_WORKSPACE/.tools/host/tool"
printf 'build cache\n' > "$GITHUB_WORKSPACE/target/build-cache/object"
printf 'outside scratch\n' > "$fixture/outside"
ln -s "$fixture/outside" "$scratch/alias"

bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/collection.log" 2>&1
mkdir "$fixture/extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/extracted"
for path in scenario.log before; do
    cmp "$scratch/$path" "$fixture/extracted/ic-timers-fixtures/failed case/$path"
done
cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" \
    "$fixture/extracted/validation-failures/raw.log"
for path in target/validation-failures/raw.log .tools/host-set.failed/tool .tools/ic-set.failed/tool; do
    cmp "$GITHUB_WORKSPACE/$path" "$fixture/extracted/$path"
done
test ! -e "$fixture/extracted/ic-timers-fixtures/failed case/.git"
test ! -e "$fixture/extracted/.tools/host"
test ! -e "$fixture/extracted/target/build-cache"
test -L "$fixture/extracted/ic-timers-fixtures/failed case/alias"
test "$(readlink "$fixture/extracted/ic-timers-fixtures/failed case/alias")" = "$fixture/outside"
perl -e 'for my $pair (0, 2) { my @a=stat $ARGV[$pair]; my @b=stat $ARGV[$pair+1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost file modes\n"; }' \
    "$scratch/before" "$fixture/extracted/ic-timers-fixtures/failed case/before" \
    "$GITHUB_WORKSPACE/.tools/host-set.failed/tool" "$fixture/extracted/.tools/host-set.failed/tool"
grep -Fxq "checkout_sha=$(git rev-parse HEAD)" "$fixture/extracted/identity.txt"
grep -Fxq 'event_sha=fixture-event-sha' "$fixture/extracted/identity.txt"
grep -Fxq 'job=fixture-job' "$fixture/extracted/identity.txt"
grep -Fxq 'host=fixture-os/fixture-arch' "$fixture/extracted/identity.txt"
grep -Fxq 'run=123' "$fixture/extracted/identity.txt"
grep -Fxq 'attempt=2' "$fixture/extracted/identity.txt"

# Collection errors must remain failures and leave the diagnostic inputs intact.
mkdir "$fixture/bin"
printf '%s\n' '#!/bin/sh' 'echo "injected archive failure" >&2' 'exit 23' > "$fixture/bin/tar"
chmod +x "$fixture/bin/tar"
collection_status=0
PATH="$fixture/bin:$PATH" bash "$root/scripts/ci/collect-failure-evidence.sh" \
    > "$fixture/failed-collection.log" 2>&1 || collection_status=$?
test "$collection_status" -eq 23
grep -Fq 'injected archive failure' "$fixture/failed-collection.log"
cmp "$scratch/scenario.log" "$fixture/extracted/ic-timers-fixtures/failed case/scenario.log"
cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" \
    "$fixture/extracted/validation-failures/raw.log"

# Early failures can have no fixture, logger or installer payload yet.
rm -rf "$RUNNER_TEMP/ic-timers-fixtures" "$GITHUB_WORKSPACE/target" \
    "$GITHUB_WORKSPACE/.tools" "$GITHUB_WORKSPACE/.git/release-state"
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/empty-collection.log" 2>&1
tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/empty-entries"
printf 'identity.txt\n' > "$fixture/expected-empty-entries"
cmp "$fixture/expected-empty-entries" "$fixture/empty-entries"
if bash "$root/scripts/ci/collect-failure-evidence.sh" unexpected > "$fixture/invalid-input.log" 2>&1; then exit 1; fi
if env -u RUNNER_TEMP bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/missing-input.log" 2>&1; then exit 1; fi
# Drive the actual local fixtures to an early tool failure. A retained input
# and the original status must survive their EXIT handlers; no builds or real
# Git writes are selected by these substitute tools.
mkdir -p "$fixture/retention-bin"
export RETENTION_REAL_GIT="$(command -v git)"
cat > "$fixture/retention-bin/tool-stub" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${0##*/}" == git && "${1:-}" != init && "${1:-}" != clone ]]; then
    exec "$RETENTION_REAL_GIT" "$@"
fi
for path in "$TMPDIR"/*; do
    [[ -d "$path" ]] || continue
    printf 'retained injected input\n' > "$path/injected-input"
done
echo 'injected fixture tool failure' >&2
exit 23
STUB
chmod +x "$fixture/retention-bin/tool-stub"
for tool in git cargo cp; do ln -s tool-stub "$fixture/retention-bin/$tool"; done
retained_inputs=()
for script in scripts/release/test-committed-release.sh scripts/release/test-release-index.sh \
    scripts/release/test-lockfiles.sh scripts/ci/test-repository-checks.sh; do
    name="${script##*/}"
    scratch="$RUNNER_TEMP/ic-timers-fixtures/retention cases/$name"
    mkdir -p "$scratch"
    status=0
    TMPDIR="$scratch" PATH="$fixture/retention-bin:$PATH" bash "$root/$script" \
        > "$fixture/$name-retention.log" 2>&1 || status=$?
    test "$status" -eq 23
    retained=("$scratch"/*)
    test "${#retained[@]}" -eq 1
    test -d "${retained[0]}"
    grep -Fxq 'retained injected input' "${retained[0]}/injected-input"
    retained_inputs+=("${retained[0]}/injected-input")
    grep -Fq 'fixture retained:' "$fixture/$name-retention.log"
    grep -Fq 'injected fixture tool failure' "$fixture/$name-retention.log"
done
# Archive the actual retained inputs through the unchanged collector.
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/retained-collection.log" 2>&1
mkdir "$fixture/retained-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/retained-extracted"
for input in "${retained_inputs[@]}"; do
    cmp "$input" "$fixture/retained-extracted/${input#"$RUNNER_TEMP/"}"
done

# Exercise the manual qualification driver in the isolated checkout, with fake
# CI identity, a substitute download and a failing scratch Make recipe. This
# proves local retained bytes/status; it does not dispatch or qualify uploads.
mkdir -p "$GITHUB_WORKSPACE/scripts/ci" "$GITHUB_WORKSPACE/scripts/dev" "$GITHUB_WORKSPACE/ci"
for script in qualify-failure-evidence run-validation-targets check-make-execution verify-file-checksum; do
    cp "$root/scripts/ci/$script.sh" "$GITHUB_WORKSPACE/scripts/ci/"
done
cp "$root/scripts/dev/install-host-tools.sh" "$GITHUB_WORKSPACE/scripts/dev/"
cp "$root/ci/tool-versions.env" "$GITHUB_WORKSPACE/ci/"
status=0
GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
    bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" unknown \
    > "$fixture/invalid-qualification.log" 2>&1 || status=$?
test "$status" -eq 2
status=0
GITHUB_ACTIONS=true GITHUB_EVENT_NAME=push \
    bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" early \
    > "$fixture/nonmanual-qualification.log" 2>&1 || status=$?
test "$status" -eq 2
for stage in early late; do
    status=0
    GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
        bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" "$stage" \
        > "$fixture/$stage-qualification.log" 2>&1 || status=$?
    expected=22
    if [[ "$stage" == late ]]; then expected=2; fi
    if [[ "$status" != "$expected" ]]; then
        cat "$fixture/$stage-qualification.log" >&2
        exit 1
    fi
    scenarios=("$RUNNER_TEMP/ic-timers-fixtures/hosted-${stage}."*)
    test "${#scenarios[@]}" -eq 1
    scenario="${scenarios[0]}"
    printf 'stage=%s\nstatus=%s\nexpected=%s\n' "$stage" "$expected" "$expected" > "$fixture/expected-status"
    cmp "$fixture/expected-status" "$scenario/status.txt"
    cmp "$scenario/before.txt" "$scenario/after.txt"
    if [[ "$stage" == early ]]; then
        candidates=("$scenario/consumer/.tools/host-set."*)
        test "${#candidates[@]}" -eq 1
        grep -Fxq 'controlled rejected download bytes' "${candidates[0]}/bin/jq"
    else
        grep -Fxq 'error: controlled late validation failure' \
            "$GITHUB_WORKSPACE/target/validation-failures/latest-combined.log"
    fi
done
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/qualification-collection.log" 2>&1
mkdir "$fixture/qualification-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/qualification-extracted"
for scenario in "$RUNNER_TEMP/ic-timers-fixtures"/hosted-*; do
    for file in before.txt after.txt scenario.log status.txt; do
        cmp "$scenario/$file" "$fixture/qualification-extracted/${scenario#"$RUNNER_TEMP/"}/$file"
    done
done
echo 'CI failure evidence selection, metadata, modes, empty-input, retention and qualification-driver checks passed'
