#!/usr/bin/env bash
set -euo pipefail

unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-version-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed version-preparation fixture retained"' EXIT
git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{scripts/release,scripts/ci,docs/status,docs/changelog,crates/ic-timers/src,testing/probe/src}
cp "${repository_root}/scripts/release/bump-version.sh" "${temporary_root}/scripts/release/"
cp "${repository_root}/scripts/ci/next-release-version.sh" \
    "${repository_root}/scripts/ci/validate-release-inputs.pl" \
    "${repository_root}/scripts/ci/check-make-execution.sh" \
    "${repository_root}/scripts/ci/run-formatting.sh" \
    "${repository_root}/scripts/ci/finalize-release-changelog.awk" \
    "${repository_root}/scripts/ci/check-release-changelog.sh" "${temporary_root}/scripts/ci/"
cp "${repository_root}/scripts/ci/read-cargo-workspace-version.sh" \
    "${repository_root}/scripts/ci/render-cargo-workspace-version.pl" \
    "${repository_root}/scripts/ci/cargo-release-metadata.sh" \
    "${repository_root}/scripts/ci/cargo-workspace-members.jq" \
    "${repository_root}/scripts/ci/compare-stable-versions.pl" \
    "${repository_root}/scripts/ci/StableVersion.pm" \
    "${repository_root}/scripts/ci/rewrite-local-lock-versions.pl" \
    "${temporary_root}/scripts/ci/"
# Classification is an isolated fixture input; preparation must not run tests.
cat > "${temporary_root}/scripts/release/classify-release-impact.sh" <<'EOF'
#!/usr/bin/env bash
if [[ "${FIXTURE_RELEASE_IMPACT-crate}" == error ]]; then
    echo 'fixture classification failure' >&2
    exit 1
fi
echo "${FIXTURE_RELEASE_IMPACT-crate}"
EOF
cat > "${temporary_root}/Cargo.toml" <<'EOF'
[workspace]
members = ["crates/ic-timers", "testing/probe"]
resolver = "3"
[workspace.dependencies]
ic-timers = { path = "crates/ic-timers" }
[workspace.dependencies.fixture]
version = "0.1.0"
[workspace.package]
  version = "0.1.0" # Workspace truth; preserve spacing and this comment.
EOF
cat > "${temporary_root}/crates/ic-timers/Cargo.toml" <<'EOF'
[package]
name = "ic-timers"
version.workspace = true
edition = "2024"
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/crates/ic-timers/src/lib.rs"
cat > "${temporary_root}/testing/probe/Cargo.toml" <<'EOF'
[package]
name = "probe"
version.workspace = true
edition = "2024"
[dependencies]
ic-timers.workspace = true
EOF
printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/testing/probe/src/lib.rs"
cat > "${temporary_root}/CHANGELOG.md" <<'EOF'
# Changelog

## [0.1.1]

- Fix terminal cleanup.

## [0.1.0]

- Initial release.
EOF
cat > "${temporary_root}/docs/status/current.md" <<'EOF'
# Current status

Read Cargo for package identity. This handoff has no release marker.
EOF
# shellcheck disable=SC2016 # Markdown fixture keeps literal backticks.
printf '%s\n' '# Fixture' '| API line | `0.0` |' 'ic-timers = "=0.0.7"' \
    'ic-timers = "0.0" # Another legitimate documentation example.' \
    > "${temporary_root}/README.md"
printf '%s\n' 'release-verify:' $'\t@touch unexpected-gate' $'\t@exit 1' \
    > "${temporary_root}/Makefile"
printf '%s\n' 'Unrelated work must survive preparation.' > "${temporary_root}/unrelated.txt"
cd "${temporary_root}"
cargo generate-lockfile --offline --quiet

# Preflight validates metadata without running deployment tests or changing it.
metadata_files=(Cargo.toml Cargo.lock CHANGELOG.md README.md \
    docs/status/current.md unrelated.txt)
chmod 0640 CHANGELOG.md

# Keep actual bytes and modes rather than separate checksum and mode manifests.
capture_metadata() {
    local snapshot="${1}"
    local path
    for path in "${metadata_files[@]}"; do
        mkdir -p "${snapshot}/$(dirname "${path}")"
        cp -p "${path}" "${snapshot}/${path}"
    done
}

assert_metadata_unchanged() {
    perl -MFile::Compare=compare -e '
        my $snapshot = shift @ARGV;
        for my $path (@ARGV) {
            my $original = "$snapshot/$path";
            compare($original, $path) == 0
                or die "error: fixture file contents changed: $path\n";
            my @before = stat $original;
            my @after = stat $path;
            @before && @after or die "error: cannot read fixture mode: $path\n";
            ($before[2] & 07777) == ($after[2] & 07777)
                or die "error: fixture file mode changed: $path\n";
        }
    ' "${1}" "${metadata_files[@]}"
}

capture_metadata original-files
# Reject malformed invocation before metadata reads, gates or mutation. A
# misplaced --check must never be silently ignored by a real bump.
for invocation in no-arguments missing-version extra-argument extra-version misplaced-check duplicate-check; do
    case "${invocation}" in
        no-arguments) set -- ;;
        missing-version) set -- --check ;;
        extra-argument) set -- --check patch extra ;;
        extra-version) set -- 0.1.1 extra ;;
        misplaced-check) set -- patch --check ;;
        duplicate-check) set -- --check --check patch ;;
    esac
    if output="$(bash scripts/release/bump-version.sh "$@" 2>&1)"; then
        echo "error: version preparation accepted ${invocation}" >&2
        exit 1
    else
        rejection_status=$?
    fi
    if [[ "${rejection_status}" != 2 || "${output}" != Usage:* ]]; then
        echo "error: invalid ${invocation} reached preparation: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged original-files
done
# Shared owns reader permutations; keep rejection at the actual bump boundary.
cp -p Cargo.toml valid-manifest.toml
perl -pi -e 's/\[workspace\.package\]/[workspace.metadata]/' Cargo.toml
capture_metadata rejected-version-files
reader_status=0
bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml > reader-rejection.log 2>&1 || reader_status=$?
[[ "$reader_status" != 0 ]] || exit 1
for operation in check prepare; do
    arguments=(patch)
    if [[ "$operation" == check ]]; then arguments=(--check patch); fi
    status=0
    bash scripts/release/bump-version.sh "${arguments[@]}" > version-rejection.log 2>&1 || status=$?
    [[ "$status" == "$reader_status" ]] || exit 1
    assert_metadata_unchanged rejected-version-files
done
cp -p valid-manifest.toml Cargo.toml
rm valid-manifest.toml

# Tag-query errors must stop both preflight and the bump before any mutation.
mkdir -p bin
IC_TIMERS_FIXTURE_GIT="$(command -v git)"
export IC_TIMERS_FIXTURE_GIT
cat > bin/git <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == tag && "${2:-}" == --list ]]; then
    printf '%s' "${FIXTURE_TAG_LOOKUP_OUTPUT:-}"
    echo 'injected release-tag lookup failure' >&2
    exit 128
fi
exec "${IC_TIMERS_FIXTURE_GIT}" "$@"
EOF
chmod +x bin/git
for operation in preflight bump; do
    arguments=(patch)
    if [[ "${operation}" == preflight ]]; then arguments=(--check patch); fi
    for lookup_output in '' v0.1.1; do
        if output="$(PATH="${temporary_root}/bin:${PATH}" \
            FIXTURE_TAG_LOOKUP_OUTPUT="${lookup_output}" \
            bash scripts/release/bump-version.sh "${arguments[@]}" 2>&1)"; then
            echo "error: ${operation} accepted a failed release-tag lookup" >&2
            exit 1
        fi
        if [[ "${output}" != *'injected release-tag lookup failure'* ]]; then
            echo "error: unexpected tag-query rejection: ${output}" >&2
            exit 1
        fi
        assert_metadata_unchanged original-files
        staged_paths="$(git diff --cached --name-only)"
        test -z "${staged_paths}"
    done
done
rm bin/git

for kind in patch minor major; do
    case "${kind}" in
        patch) candidate=0.1.1 ;;
        minor) candidate=0.2.0 ;;
        major) candidate=1.0.0 ;;
    esac
    # Each requested increment has its own explicit numbered pending release.
    CANDIDATE="${candidate}" perl -pi -e 's/^## \[0\.1\.1\]$/## [$ENV{CANDIDATE}]/' CHANGELOG.md
    capture_metadata preflight-files
    output="$(bash scripts/release/bump-version.sh --check "${kind}" 2>&1)"
    if [[ "${output}" != *"Release preflight passed: 0.1.0 -> ${candidate}"* ]]; then
        echo "error: ${kind} preflight selected an unexpected version: ${output}" >&2
        exit 1
    fi
    assert_metadata_unchanged preflight-files
    cp -p original-files/CHANGELOG.md CHANGELOG.md
done

# README shape, freshness and presence are not release preconditions.
mv README.md owned-readme.md
bash scripts/release/bump-version.sh --check patch >/dev/null
test ! -e README.md
ln -s owned-readme.md README.md
bash scripts/release/bump-version.sh --check patch >/dev/null
test -L README.md
rm README.md
mv owned-readme.md README.md
assert_metadata_unchanged original-files

# Preparation must not replace a consumer-owned metadata symlink.
mv Cargo.lock owned-lock
ln -s owned-lock Cargo.lock
if bash scripts/release/bump-version.sh patch >/dev/null 2>&1; then
    echo 'error: version preparation accepted a symlinked metadata output' >&2
    exit 1
fi
test -L Cargo.lock
assert_metadata_unchanged original-files
rm Cargo.lock
mv owned-lock Cargo.lock

# Qualify policy at its actual preparation boundary, before metadata mutation.
for operation in check prepare; do
    arguments=(patch)
    if [[ "$operation" == check ]]; then arguments=(--check patch); fi
    for impact in none unexpected '' error; do
        case "$impact" in
            none) expected=1; diagnostic='no changes exist since v0.1.0' ;;
            unexpected) expected=2; diagnostic='unknown release-impact classification: unexpected' ;;
            '') expected=2; diagnostic='unknown release-impact classification: <empty>' ;;
            error) expected=1; diagnostic='fixture classification failure' ;;
        esac
        status=0
        output="$(FIXTURE_RELEASE_IMPACT="$impact" bash scripts/release/bump-version.sh \
            "${arguments[@]}" 2>&1)" || status=$?
        if [[ "$status" != "$expected" || "$output" != *"$diagnostic"* ]]; then
            printf 'error: %s mishandled release impact <%s> (exit %s): %s\n' \
                "$operation" "$impact" "$status" "$output" >&2
            exit 1
        fi
        assert_metadata_unchanged original-files
    done
done
output="$(FIXTURE_RELEASE_IMPACT=repository bash scripts/release/bump-version.sh --check patch 2>&1)"
for advisory in 'changes since v0.1.0 are repository-only' \
    'continuing because the maintainer invoked an explicit version bump'; do
    if [[ "$output" != *"$advisory"* ]]; then
        echo 'error: repository-only preflight omitted its advisory' >&2
        exit 1
    fi
done
assert_metadata_unchanged original-files

# Standard delivery ordering and Make execution admission belong to the
# shared release runner and its adopted fixtures, not version preparation.

# Shared owns mutation/rollback. Inject a failure and interruption into its
# post-mutation graph check, retaining this consumer's exact files and modes.
mkdir -p bin
IC_TIMERS_FIXTURE_CARGO="$(command -v cargo)"
export IC_TIMERS_FIXTURE_CARGO
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
    set-version|sort) echo 'unexpected manifest mutation tool' >&2; exit 91 ;;
    metadata)
        if grep -Fq 'version = "0.1.1"' Cargo.toml; then
            case "${FIXTURE_FAIL_STAGE:-}" in
                root-metadata) echo 'injected root-metadata failure' >&2; exit 23 ;;
                interrupt) kill -TERM "$PPID"; exit 1 ;;
            esac
        fi ;;
esac
exec "${IC_TIMERS_FIXTURE_CARGO}" "$@"
EOF
chmod +x bin/cargo
for stage in root-metadata interrupt; do
    status=0
    output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE="${stage}" \
        bash scripts/release/bump-version.sh patch 2>&1)" || status=$?
    case "$stage" in root-metadata) expected=23 ;; interrupt) expected=143 ;; esac
    [[ "$status" == "$expected" && "$output" == *'restoring release metadata'* ]] || exit 1
    assert_metadata_unchanged original-files
done

# A missing changelog is refused before mutation.
mv CHANGELOG.md existing-changelog.md
for operation in check prepare; do
    arguments=(patch)
    if [[ "${operation}" == check ]]; then arguments=(--check patch); fi
    if output="$(PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_STAGE=root-metadata \
        bash scripts/release/bump-version.sh "${arguments[@]}" 2>&1)"; then
        echo "error: ${operation} accepted a missing changelog" >&2
        exit 1
    fi
    if [[ "${output}" == *'injected root-metadata failure'* ||
        "${output}" == *'restoring release metadata'* ]]; then
        echo "error: missing-changelog refusal occurred after mutation: ${output}" >&2
        exit 1
    fi
    test ! -e CHANGELOG.md
    cmp original-files/Cargo.toml Cargo.toml
    cmp original-files/Cargo.lock Cargo.lock
done
mv existing-changelog.md CHANGELOG.md
assert_metadata_unchanged original-files

# Prepare the admitted candidate after all rejection cases preserved metadata.
bash scripts/release/bump-version.sh patch > "$temporary_root/preparation.log" 2>&1
test ! -f unexpected-gate
grep -Fqx '  version = "0.1.1" # Workspace truth; preserve spacing and this comment.' Cargo.toml
test "$(bash scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml)" = 0.1.1
grep -Fqx '## [0.1.0]' CHANGELOG.md
cmp original-files/README.md README.md
# The earlier dependency version must survive the bump unchanged.
grep -Fqx 'version = "0.1.0"' Cargo.toml
RELEASE_COMMIT='' bash scripts/ci/cargo-release-metadata.sh check-graph ic-timers
grep -Fqx 'Unrelated work must survive preparation.' unrelated.txt
fixture_tags="$(git tag --list)"
if git rev-parse --verify HEAD >/dev/null 2>&1 || [[ -n "${fixture_tags}" ]]; then
    echo 'error: version preparation committed or tagged fixture changes' >&2
    exit 1
fi
staged_paths="$(git diff --cached --name-only)"
if [[ -n "${staged_paths}" ]]; then
    echo 'error: version preparation staged unrelated fixture changes' >&2
    exit 1
fi

# A same-named branch is harmless; only an existing exact release tag blocks.
git config user.name 'ic-timers release test'
git config user.email 'release-test@example.invalid'
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -qm fixture
git branch v0.1.2
perl -0pi -e 's/^(## \[)/## [0.1.2]\n\n- Next fixture release.\n\n$1/m' CHANGELOG.md
bash scripts/release/bump-version.sh --check patch >/dev/null
git tag v0.1.2
capture_metadata tagged-files
if output="$(bash scripts/release/bump-version.sh --check patch 2>&1)"; then
    echo 'error: version preflight accepted an existing release tag' >&2
    exit 1
fi
if [[ "${output}" != *'tag v0.1.2 already exists'* ]]; then
    echo "error: unexpected tag rejection: ${output}" >&2
    exit 1
fi
assert_metadata_unchanged tagged-files
fixture_complete=true
echo 'Version preflight, rollback and dirty-worktree preparation checks passed'
