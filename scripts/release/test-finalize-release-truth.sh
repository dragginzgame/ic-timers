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
    "${temporary_root}/docs/changelog" \
    "${temporary_root}/docs/adoption"
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
cat > "${temporary_root}/docs/adoption/canic.md" <<'EOF'
# Canic adapter contract

Status: validated downstream adoption worktree; Canic release remains pending.
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
for invalid in duplicate-status empty-status wrong-heading duplicate-workspace finalized-note missing-note; do
    cp "${temporary_root}/original-status.md" "${temporary_root}/docs/status/current.md"
    cp "${temporary_root}/original-note.md" "${temporary_root}/docs/changelog/0.3.4.md"
    case "${invalid}" in
        duplicate-status) printf '%s\n' 'Status: another status.' >> "${temporary_root}/docs/changelog/0.3.4.md" ;;
        empty-status) sed -i 's/^Status: .*/Status:   /' "${temporary_root}/docs/changelog/0.3.4.md" ;;
        wrong-heading) sed -i 's/^# 0.3.4$/# 0.3.5/' "${temporary_root}/docs/changelog/0.3.4.md" ;;
        duplicate-workspace) printf '%s\n' '- Workspace package version: `0.3.3`.' >> "${temporary_root}/docs/status/current.md" ;;
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

echo "Release-truth finalization checks passed"
