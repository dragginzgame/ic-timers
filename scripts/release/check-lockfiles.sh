#!/usr/bin/env bash
set -euo pipefail

# Dependency resolution validates both locks without building either workspace.
# --no-deps would skip resolution and accept stale path-package versions.
cargo metadata --locked --offline --format-version 1 >/dev/null
cargo metadata --manifest-path testing/Cargo.toml \
    --locked --offline --format-version 1 >/dev/null
