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
echo 'CI failure evidence selection, metadata, modes and empty-input checks passed'
