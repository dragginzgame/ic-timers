#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
finalizer="${repository_root}/scripts/release/finalize-changelog.sh"
test_directory="$(mktemp -d)"

cleanup() {
    rm -rf -- "${test_directory}"
}
trap cleanup EXIT

actual="${test_directory}/CHANGELOG.md"
original="${test_directory}/original.md"
expected="${test_directory}/expected.md"

cat > "${actual}" <<'EOF'
# Changelog

## [Unreleased]

### Added

- Add release automation.

## [0.1.0] - 2026-08-01

- Initial release.
EOF

cp "${actual}" "${original}"

cat > "${expected}" <<'EOF'
# Changelog

## [Unreleased]

## [0.2.0] - 2026-08-13

### Added

- Add release automation.

## [0.1.0] - 2026-08-01

- Initial release.
EOF

bash "${finalizer}" --check 0.2.0 2026-08-13 "${actual}"
diff -u "${original}" "${actual}"

bash "${finalizer}" 0.2.0 2026-08-13 "${actual}" >/dev/null
diff -u "${expected}" "${actual}"

if bash "${finalizer}" 0.2.0 2026-08-13 "${actual}" >/dev/null 2>&1; then
    echo "error: duplicate release heading was accepted" >&2
    exit 1
fi

cat > "${actual}" <<'EOF'
# Changelog

## [Unreleased]

## [0.1.0] - 2026-08-01
EOF

if bash "${finalizer}" --check 0.2.0 2026-08-13 "${actual}" >/dev/null 2>&1; then
    echo "error: empty Unreleased section was accepted" >&2
    exit 1
fi

echo "Changelog finalization checks passed"
