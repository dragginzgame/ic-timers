#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
checker="${repository_root}/scripts/release/warn-release-prose.sh"
fixture_root="$(mktemp -d)"
status_file="${fixture_root}/current.md"
trap 'rm -f "${status_file}"; rmdir "${fixture_root}"' EXIT

printf '%s\n' \
    '# Current status' \
    '' \
    '- Open release line: `0.3.7`; package remains `0.3.6`.' \
    '' \
    '## Next action' \
    '' \
    'Review the bounded hygiene patch.' > "${status_file}"
clean_output="$(bash "${checker}" 0.3.7 "${status_file}" 2>&1)"
if [[ -n "${clean_output}" ]]; then
    echo "error: advisory warned about clean candidate prose" >&2
    echo "${clean_output}" >&2
    exit 1
fi

printf '%s\n' \
    '# Current status' \
    '' \
    'Version 0.3.7 remains a candidate.' > "${status_file}"
candidate_output="$(bash "${checker}" 0.3.7 "${status_file}" 2>&1)"
if [[ "${candidate_output}" != *"advisory only; release validation continues"* ]]; then
    echo "error: advisory missed target-version candidate wording" >&2
    exit 1
fi

printf '%s\n' \
    '# Current status' \
    '' \
    '## Next action' \
    '' \
    'Publish after release.' > "${status_file}"
next_action_output="$(bash "${checker}" 0.3.7 "${status_file}" 2>&1)"
if [[ "${next_action_output}" != *"next-action wording"* ]]; then
    echo "error: advisory missed stale next-action wording" >&2
    exit 1
fi

missing_output="$(bash "${checker}" 0.3.7 "${fixture_root}/missing.md" 2>&1)"
if [[ "${missing_output}" != *"continuing"* ]]; then
    echo "error: advisory did not fail open for a missing status file" >&2
    exit 1
fi

echo "Release-prose advisory checks passed"
