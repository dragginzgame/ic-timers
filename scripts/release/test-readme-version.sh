#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
cat > "${temporary_root}/Cargo.toml" <<'MANIFEST'
[workspace.package]
version = "0.3.4"
MANIFEST
cat > "${temporary_root}/README.md" <<'README'
# Fixture

| API line | `0.3` |
ic-timers = "=0.3.4"

Historical release prose is not a version selector.
README
cd "${temporary_root}"
cp README.md original-readme.md
bash "${repository_root}/scripts/release/readme-version.sh" --check
cmp original-readme.md README.md

# Structured package pins must remain correct even without changelog/status files.
for invalid in stale-pin stale-api duplicate-pin duplicate-api missing-pin missing-api; do
    cp original-readme.md README.md
    case "${invalid}" in
        stale-pin) perl -pi -e 's/=0\.3\.4/=0.3.3/' README.md ;;
        stale-api) perl -pi -e 's/`0\.3`/`0.2`/' README.md ;;
        duplicate-pin) printf '%s\n' 'ic-timers = "=0.3.4"' >> README.md ;;
        duplicate-api) printf '%s\n' '| API line | `0.3` |' >> README.md ;;
        missing-pin) perl -ni -e 'print unless /^ic-timers =/' README.md ;;
        missing-api) perl -ni -e 'print unless /API line/' README.md ;;
    esac
    cp README.md rejected-readme.md
    if bash "${repository_root}/scripts/release/readme-version.sh" --check >/dev/null 2>&1; then
        echo "error: README version check accepted ${invalid}" >&2
        exit 1
    fi
    cmp rejected-readme.md README.md
done

# A minor/major change projects both fields and preserves unrelated prose.
cp original-readme.md README.md
bash "${repository_root}/scripts/release/workspace-version.sh" set 0.3.4 1.2.0
bash "${repository_root}/scripts/release/readme-version.sh" --update
bash "${repository_root}/scripts/release/readme-version.sh" --check
sed 's/`0.3`/`1.2`/; s/=0.3.4/=1.2.0/' original-readme.md > expected-readme.md
cmp expected-readme.md README.md

echo 'README version projection checks passed'
