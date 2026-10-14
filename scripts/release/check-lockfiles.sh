#!/usr/bin/env bash
set -euo pipefail

# Shared owns complete coordinated graph admission, independently of dated notes.
# Check the selected worktree; an ambient delivery commit cannot redirect it.
script_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
RELEASE_COMMIT= exec bash "$script_root/.shared-tooling/helpers/scripts/ci/cargo-release-metadata.sh" \
    check-graph ic-timers
