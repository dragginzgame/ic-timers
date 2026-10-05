#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/release-retry.XXXXXX")"
trap 'rm -rf "$fixture"' EXIT

new_fixture() {
    mkdir -p "$fixture/$1/scripts/ci" "$fixture/$1/scripts/release"
    cd "$fixture/$1"
    git init -q
    cp "$root/scripts/release/workspace-version.sh" scripts/release/
    cp "$root/scripts/ci/next-release-version.sh" scripts/ci/
    printf '[workspace.package]\nversion = "0.12.0"\n' > Cargo.toml
    cat > scripts/ci/run-release.sh <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> events
# A failed runner keeps its intent; a retry must admit it again, never reuse
# an older source's validation. This stub performs no release effects.
if [[ "${FAIL_VALIDATION:-0}" == 1 ]]; then
    cp pending-plan .git/release-state/0.13.0.plan
    exit 7
fi
STUB
    mkdir -p .git/release-state
    plan=.git/release-state/0.13.0.plan
}
write_plan() {
    printf '%s\n' release-plan-1 "${3:-minor}" 0.12.0 "${2:-0.13.0}" 2026-10-05 \
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa origin main \
        bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb '' "$1" > "$plan"
}
retry() { bash "$root/scripts/release/run-standard-release.sh" "$@" > output 2>&1; }
expect_rejection() {
    cp "$plan" before
    if retry minor origin main; then echo 'retry unexpectedly accepted invalid state' >&2; exit 1; fi
    cmp before "$plan"
    [[ ! -e events ]]
    [[ ! -e .git/release-state/lock ]]
}
assert_retained_attempt() {
    set -- "$plan".retry.*
    [[ $# == 1 && -f "$1" ]]
    cmp before "$1"
    [[ ! -e "$plan" && ! -e .git/release-state/lock ]]
}

for kind in patch minor major; do
    case "$kind" in patch) candidate=0.12.1 ;; minor) candidate=0.13.0 ;; major) candidate=1.0.0 ;; esac
    for phase in preflight validate; do
        new_fixture "$kind-$phase"
        plan=".git/release-state/$candidate.plan"
        write_plan "$phase" "$candidate" "$kind"
        cp "$plan" before
        retry "$kind" origin main
        assert_retained_attempt
        printf '%s\n' "$kind origin main" > expected
        cmp expected events
    done
done

new_fixture fresh
retry minor origin main
[[ ! -e "$plan" ]]
printf '%s\n' 'minor origin main' > expected
cmp expected events

for phase in prepare stage commit tag push complete; do
    new_fixture "prepared-$phase"
    write_plan "$phase"
    expect_rejection
done

for invalid in extra truncated identity index files; do
    new_fixture "$invalid"
    write_plan validate
    case "$invalid" in
        extra) printf 'unexpected unterminated record' >> "$plan" ;;
        truncated) head -n 10 "$plan" > partial; mv partial "$plan" ;;
        identity) write_plan validate 0.13.0 patch ;;
        index) perl -pi -e '$_ = "cccccccccccccccccccccccccccccccccccccccc\n" if $. == 10' "$plan" ;;
        files) touch "$plan.files" ;;
    esac
    expect_rejection
done

new_fixture symlink
write_plan validate
mv "$plan" retained-plan
ln -s ../../retained-plan "$plan"
if retry minor origin main; then exit 1; fi
[[ -L "$plan" && ! -e events && ! -e .git/release-state/lock ]]

new_fixture concurrent
write_plan validate
cp "$plan" before
mkdir .git/release-state/lock
printf 'other owner\n' > .git/release-state/lock/owner
if retry minor origin main; then exit 1; fi
cmp before "$plan"
[[ "$(cat .git/release-state/lock/owner)" == 'other owner' && ! -e events ]]

new_fixture failed-gate
write_plan validate
cp "$plan" pending-plan
if FAIL_VALIDATION=1 retry minor origin main; then exit 1; fi
cmp pending-plan "$plan"
retry minor origin main
printf '%s\n' 'minor origin main' 'minor origin main' > expected
cmp expected events
[[ ! -e "$plan" && ! -e .git/release-state/lock ]]

echo 'preparation-free release retry fixtures passed (runner stub; no release effects)'
