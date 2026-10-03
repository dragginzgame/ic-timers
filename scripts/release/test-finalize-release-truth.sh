#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
cleanup() {
    rm -rf -- "${temporary_root}"
}
trap cleanup EXIT

git init -q "${temporary_root}"
mkdir -p \
    "${temporary_root}/docs/status" \
    "${temporary_root}/docs/changelog"
cat > "${temporary_root}/Cargo.toml" <<'EOF'
[workspace.package]
version = "0.3.3"
EOF
cat > "${temporary_root}/CHANGELOG.md" <<'EOF'
# Changelog

## [Unreleased]

## [0.3.4]

- Candidate notes.

## [0.3.3] - 2026-08-13

- Released notes.
EOF
cat > "${temporary_root}/README.md" <<'EOF'
# Fixture

The current release is described without duplicating its version.

| API line | `0.3` |
ic-timers = "=0.3.3"
EOF
cat > "${temporary_root}/docs/status/current.md" <<'EOF'
# Current status

- Workspace package version: `0.3.3`.
- Latest release line: `0.3.3`.
EOF
cat > "${temporary_root}/docs/changelog/0.3.4.md" <<'EOF'
# 0.3.4

Status: prepared for 0.3.4; validation and delivery are user-owned.
EOF
# Preflight accepts normal release-note wording without another target marker
# in the handoff, and does not mutate either document.
(
    cd "${temporary_root}"
    sha256sum docs/status/current.md docs/changelog/0.3.4.md > original.sha256
    bash "${repository_root}/scripts/release/finalize-release-truth.sh" --check 0.3.3 0.3.4
    sha256sum --check --quiet original.sha256
)

# Structural errors still fail before mutation.
cp "${temporary_root}/docs/status/current.md" "${temporary_root}/original-status.md"
cp "${temporary_root}/docs/changelog/0.3.4.md" "${temporary_root}/original-note.md"
for invalid in duplicate-status empty-status blank-second-status wrong-heading duplicate-heading \
    duplicate-workspace conflicting-workspace wrong-workspace conflicting-latest wrong-latest finalized-note missing-note; do
    cp "${temporary_root}/original-status.md" "${temporary_root}/docs/status/current.md"
    cp "${temporary_root}/original-note.md" "${temporary_root}/docs/changelog/0.3.4.md"
    case "${invalid}" in
        duplicate-status) printf '%s\n' 'Status: another status.' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        empty-status) sed -i 's/^Status: .*/Status:   /' "${temporary_root}/docs/changelog/0.3.4.md" ;;
        blank-second-status) printf '%s\n' 'Status:' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        wrong-heading) sed -i 's/^# 0.3.4$/# 0.3.5/' "${temporary_root}/docs/changelog/0.3.4.md" ;;
        duplicate-heading) printf '%s\n' '# 0.3.5' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        duplicate-workspace) printf '%s\n' '- Workspace package version: `0.3.3`.' >> "${temporary_root}/docs/status/current.md" ;;
        conflicting-workspace) printf '%s\n' '- Workspace package version: `0.3.2`.' >> "${temporary_root}/docs/status/current.md" ;;
        wrong-workspace) sed -i 's/Workspace package version: `0.3.3`/Workspace package version: `0.3.2`/' "${temporary_root}/docs/status/current.md" ;;
        conflicting-latest) printf '%s\n' '- Latest release line: `0.3.2`.' >> "${temporary_root}/docs/status/current.md" ;;
        wrong-latest) sed -i 's/Latest release line: `0.3.3`/Latest release line: `0.3.2`/' "${temporary_root}/docs/status/current.md" ;;
        finalized-note) sed -i 's/^Status: .*/Status: released 0.3.4./' "${temporary_root}/docs/changelog/0.3.4.md" ;;
        missing-note) rm "${temporary_root}/docs/changelog/0.3.4.md" ;;
    esac
    if (
        cd "${temporary_root}"
        bash "${repository_root}/scripts/release/finalize-release-truth.sh" --check 0.3.3 0.3.4
    ) >/dev/null 2>&1; then
        echo "error: release preflight accepted ${invalid}" >&2
        exit 1
    fi
done
cp "${temporary_root}/original-status.md" "${temporary_root}/docs/status/current.md"
cp "${temporary_root}/original-note.md" "${temporary_root}/docs/changelog/0.3.4.md"

(
    cd "${temporary_root}"
    bash "${repository_root}/scripts/release/finalize-release-truth.sh" --check 0.3.3 0.3.4
    bash "${repository_root}/scripts/release/finalize-release-truth.sh" 0.3.3 0.3.4
    bash "${repository_root}/scripts/release/finalize-changelog.sh" 0.3.4 2026-08-14
    sed -i 's/^version = "0.3.3"$/version = "0.3.4"/' Cargo.toml
    bash "${repository_root}/scripts/release/readme-version.sh" --update
    bash "${repository_root}/scripts/release/check-release-truth.sh"
)

grep -Fqx -- '- Workspace package version: `0.3.4`.' \
    "${temporary_root}/docs/status/current.md"
grep -Fqx -- '- Latest release line: `0.3.4`.' \
    "${temporary_root}/docs/status/current.md"
grep -Fqx -- 'Status: released 0.3.4.' \
    "${temporary_root}/docs/changelog/0.3.4.md"

if (
    cd "${temporary_root}"
    bash "${repository_root}/scripts/release/finalize-release-truth.sh" --check 0.3.3 0.3.4
) >/dev/null 2>&1; then
    echo "error: finalized release truth was accepted as a candidate" >&2
    exit 1
fi

# Final validation must reject contradictory fields even if a correct marker
# is also present. Free-form prose remains outside this structural gate.
cp "${temporary_root}/docs/status/current.md" "${temporary_root}/released-status.md"
cp "${temporary_root}/docs/changelog/0.3.4.md" "${temporary_root}/released-note.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/released-changelog.md"
cp "${temporary_root}/README.md" "${temporary_root}/released-readme.md"
for invalid in conflicting-workspace conflicting-latest wrong-latest duplicate-status blank-status duplicate-heading duplicate-release stale-pin stale-api duplicate-pin missing-api; do
    cp "${temporary_root}/released-status.md" "${temporary_root}/docs/status/current.md"
    cp "${temporary_root}/released-note.md" "${temporary_root}/docs/changelog/0.3.4.md"
    cp "${temporary_root}/released-changelog.md" "${temporary_root}/CHANGELOG.md"
    cp "${temporary_root}/released-readme.md" "${temporary_root}/README.md"
    case "${invalid}" in
        conflicting-workspace) printf '%s\n' '- Workspace package version: `0.3.3`.' >> "${temporary_root}/docs/status/current.md" ;;
        conflicting-latest) printf '%s\n' '- Latest release line: `0.3.3`.' >> "${temporary_root}/docs/status/current.md" ;;
        wrong-latest) sed -i 's/Latest release line: `0.3.4`/Latest release line: `0.3.3`/' "${temporary_root}/docs/status/current.md" ;;
        duplicate-status) printf '%s\n' 'Status: prepared.' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        blank-status) printf '%s\n' 'Status:' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        duplicate-heading) printf '%s\n' '# 0.3.5' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        duplicate-release) printf '%s\n' '## [0.3.4]' >> "${temporary_root}/CHANGELOG.md" ;;
        stale-pin) sed -i 's/=0.3.4/=0.3.3/' "${temporary_root}/README.md" ;;
        stale-api) sed -i 's/`0.3`/`0.2`/' "${temporary_root}/README.md" ;;
        duplicate-pin) printf '%s\n' 'ic-timers = "=0.3.4"' >> "${temporary_root}/README.md" ;;
        missing-api) sed -i '/API line/d' "${temporary_root}/README.md" ;;
    esac
    if (cd "${temporary_root}"; bash "${repository_root}/scripts/release/check-release-truth.sh") >/dev/null 2>&1; then
        echo "error: finalized release accepted ${invalid}" >&2
        exit 1
    fi
done

# A minor/major change projects both fields and preserves unrelated prose.
cp "${temporary_root}/released-readme.md" "${temporary_root}/README.md"
(
    cd "${temporary_root}"
    bash "${repository_root}/scripts/release/workspace-version.sh" set 0.3.4 1.2.0
    bash "${repository_root}/scripts/release/readme-version.sh" --update
    bash "${repository_root}/scripts/release/readme-version.sh" --check
)
sed 's/`0.3`/`1.2`/; s/=0.3.4/=1.2.0/' "${temporary_root}/released-readme.md" \
    > "${temporary_root}/expected-readme.md"
cmp "${temporary_root}/expected-readme.md" "${temporary_root}/README.md"

echo "Release-truth finalization checks passed"
