#!/usr/bin/env bash
set -euo pipefail
repository_root="$(git rev-parse --show-toplevel)"
export PATH="$repository_root/.tools/host/bin:$PATH"
export YQ="$repository_root/.tools/host/bin/yq"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-ci-tag.XXXXXX")"
fixture_complete=false
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$fixture" "Failed CI tag fixture retained"' EXIT
mkdir -p "$fixture/repository/scripts/release" "$fixture/repository/scripts/ci" \
    "$fixture/repository/.shared-tooling/helpers/scripts/ci" "$fixture/repository/crates/ic-timers/src" "$fixture/bin" "$fixture/runner"
for name in check-ci-tag check-tag-at-head workspace-version; do
    cp "$repository_root/scripts/release/$name.sh" "$fixture/repository/scripts/release/"
done
cp "$repository_root/scripts/ci/check-release-source.sh" "$fixture/repository/scripts/ci/"
cp "$repository_root/.shared-tooling/helpers/scripts/ci/read-cargo-workspace-version.sh" \
    "$repository_root/.shared-tooling/helpers/scripts/ci/check-release-tag.sh" \
    "$fixture/repository/.shared-tooling/helpers/scripts/ci/"
cat > "$fixture/repository/scripts/ci/verify-shared-tooling-snapshot.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
stage=root-snapshot
[[ "$*" != *--manifest* ]] || stage=audit-snapshot
[[ "$0" != */.shared-tooling/helpers/* ]] || stage=helper-snapshot
printf '%s\n' "$stage" >> "$TAG_EVENTS"
[[ "${TAG_FAILURE:-}" != "$stage" ]] || { printf 'plausible snapshot receipt\n'; exit 37; }
STUB
cp "$fixture/repository/scripts/ci/verify-shared-tooling-snapshot.sh" "$fixture/repository/.shared-tooling/helpers/scripts/ci/"
cat > "$fixture/repository/scripts/release/check-lockfiles.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf 'graph\n' >> "$TAG_EVENTS"
[[ "${TAG_FAILURE:-}" != graph ]] || { printf 'plausible graph result\n'; exit 37; }
STUB
cat > "$fixture/repository/Cargo.toml" <<'TOML'
[workspace]
members = ["crates/ic-timers"]
resolver = "3"
[workspace.package]
version = "0.1.0"
TOML
printf '[package]\nname = "ic-timers"\nversion.workspace = true\nedition = "2024"\n' > "$fixture/repository/crates/ic-timers/Cargo.toml"
printf 'pub fn fixture() {}\n' > "$fixture/repository/crates/ic-timers/src/lib.rs"
printf '# Changelog\n\n## [0.1.0] - 2026-10-14\n\n- Fixture release.\n' > "$fixture/repository/CHANGELOG.md"
cd "$fixture/repository"
git init -q
git config user.name 'CI tag fixture'
git config user.email fixture@example.invalid
git add Cargo.toml CHANGELOG.md scripts .shared-tooling crates
git commit -qm fixture
git commit --allow-empty -qm 'release fixture'
export GITHUB_WORKSPACE="$PWD" GITHUB_EVENT_NAME=push GITHUB_REF_NAME=v0.1.0 GITHUB_REF=refs/tags/v0.1.0
GITHUB_SHA="$(git rev-parse HEAD)"
export GITHUB_SHA TAG_EVENTS="$fixture/events" RUNNER_TEMP="$fixture/runner"
git tag -a v0.1.0 -m fixture
git update-ref refs/remotes/origin/main HEAD
checker="$PWD/scripts/release/check-ci-tag.sh"
: > "$TAG_EVENTS"
bash "$checker" > "$fixture/accepted.log"
printf 'root-snapshot\naudit-snapshot\nhelper-snapshot\ngraph\n' > "$fixture/expected-events"
cmp "$fixture/expected-events" "$TAG_EVENTS"
reject() {
    local status=0
    : > "$TAG_EVENTS"
    bash "$checker" > "$fixture/rejected.log" 2>&1 || status=$?
    [[ "$status" != 0 ]] || { echo 'tag admission accepted contradicted input' >&2; exit 1; }
    if grep -Fq 'full qualification remains' "$fixture/rejected.log"; then exit 1; fi
}
# Actual Git refs and event identity, not synthetic successful tag observations.
GITHUB_REF_NAME=v0.1.1 GITHUB_REF=refs/tags/v0.1.1 reject
GITHUB_SHA="$(git rev-parse HEAD^)" reject
GITHUB_EVENT_NAME=pull_request reject
GITHUB_REF=refs/heads/main reject
git update-ref refs/remotes/origin/main HEAD^
reject
git update-ref refs/remotes/origin/main HEAD
git tag -d v0.1.0 > /dev/null
git tag v0.1.0
reject
git tag -d v0.1.0 > /dev/null
git tag -a v0.1.0 -m fixture
printf 'untracked source\n' > untracked
reject
rm untracked
for stage in root-snapshot audit-snapshot helper-snapshot graph; do
    TAG_FAILURE="$stage" reject
    case "$stage" in
        root-snapshot) printf 'root-snapshot\n' ;;
        audit-snapshot) printf 'root-snapshot\naudit-snapshot\n' ;;
        helper-snapshot) printf 'root-snapshot\naudit-snapshot\nhelper-snapshot\n' ;;
        graph) cat "$fixture/expected-events" ;;
    esac > "$fixture/expected-prefix"
    cmp "$fixture/expected-prefix" "$TAG_EVENTS"
done
# Newly introduced Git observations must reject plausible output on failure.
TAG_REAL_GIT="$(command -v git)"
export TAG_REAL_GIT
cat > "$fixture/bin/git" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${TAG_GIT_FAILURE:-}" == "$*" ]]; then
    printf '%s\n' "$GITHUB_SHA"
    exit 37
fi
exec "$TAG_REAL_GIT" "$@"
STUB
chmod +x "$fixture/bin/git"
for query in 'rev-parse --verify HEAD' "merge-base --is-ancestor $GITHUB_SHA refs/remotes/origin/main"; do
    TAG_GIT_FAILURE="$query" PATH="$fixture/bin:$PATH" reject
    [[ ! -s "$TAG_EVENTS" ]] || exit 1
done
rm "$fixture/bin/git"

# An unfinalized or duplicate current section cannot be substituted for release notes.
cp CHANGELOG.md "$fixture/finalized-notes"
for notes in undated wrong-version duplicate; do
    cp "$fixture/finalized-notes" CHANGELOG.md
    case "$notes" in
        undated) sed 's/ - 2026-10-14//' CHANGELOG.md > "$fixture/notes"; cp "$fixture/notes" CHANGELOG.md ;;
        wrong-version) sed 's/\[0.1.0\]/[0.1.1]/' CHANGELOG.md > "$fixture/notes"; cp "$fixture/notes" CHANGELOG.md ;;
        duplicate) printf '\n## [0.1.0] - 2026-10-14\n' >> CHANGELOG.md ;;
    esac
    git add CHANGELOG.md
    git commit -qm "$notes fixture"
    git tag -fa v0.1.0 -m fixture > /dev/null
    git update-ref refs/remotes/origin/main HEAD
    GITHUB_SHA="$(git rev-parse HEAD)"
    reject
    [[ ! -s "$TAG_EVENTS" ]] || exit 1
done
cp "$fixture/finalized-notes" CHANGELOG.md
git add CHANGELOG.md
git commit -qm finalized
git tag -fa v0.1.0 -m fixture > /dev/null
git update-ref refs/remotes/origin/main HEAD
GITHUB_SHA="$(git rev-parse HEAD)"
# Execute the actual workflow body. Any full setup/build/regression target fails.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -er '.jobs["tag-truth"].steps[] | select(.name == "Verify release tag") | .run' > "$fixture/workflow.sh"
cat > "$fixture/bin/make" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TAG_MAKE_EVENTS"
case "$*" in
    install-host-tools|host-tools-check|fetch) ;;
    release-tag-admission) exec bash "$GITHUB_WORKSPACE/scripts/release/check-ci-tag.sh" ;;
    *) echo 'tag workflow dispatched a forbidden target' >&2; exit 97 ;;
esac
STUB
chmod +x "$fixture/bin/make"
TAG_MAKE_EVENTS="$fixture/make-events" PATH="$fixture/bin:$PATH" \
    bash -e -o pipefail "$fixture/workflow.sh" > "$fixture/workflow.log" 2>&1
printf 'install-host-tools\nhost-tools-check\nfetch\nrelease-tag-admission\n' > "$fixture/expected-make-events"
cmp "$fixture/expected-make-events" "$fixture/make-events"
# Keep the full regression in its owner, and preserve every tag failure stage.
awk '/^release-check:/ { selected=1; next } selected && /^[^[:space:]#]/ { exit } selected { print }' "$repository_root/Makefile" |
    grep -Fq 'bash scripts/release/test-ci-tag.sh'
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -e '.jobs["tag-truth"].steps | any(.name == "Archive failure evidence") and any(.name == "Upload failure evidence")' > /dev/null
# No other tag-job step may reintroduce complete provisioning/regressions.
yq -o json '.' "$repository_root/.github/workflows/ci.yml" |
    jq -e '.jobs["tag-truth"].steps | all(.[];
        ((.run // "") | test("make[^\\n]* (install-tools|tools-check|release-check|ci|testing-check|pocketic-watchdog|pocketic-cohorts)([ \\t\\n]|$)") | not)
        and ((.with.targets // "") == "") and ((.with.components // "") == ""))' > /dev/null
fixture_complete=true
echo 'Actual event/tag/source admission, rejection propagation and narrow workflow dispatch passed'
