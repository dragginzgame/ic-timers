#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
checker="${repository_root}/scripts/ci/check-pocketic.sh"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir -p "${temporary_root}"/{bin,cache}
export IC_TIMERS_FIXTURE_LOG="${temporary_root}/events"
IC_TIMERS_FIXTURE_SHA="$(sed -n 's/^expected_sha256="\([^"]*\)"$/\1/p' "${checker}")"
export IC_TIMERS_FIXTURE_SHA
export IC_TIMERS_FIXTURE_ARCHIVE="${temporary_root}/download.gz"
cat > "${temporary_root}/candidate" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' execute >> "${IC_TIMERS_FIXTURE_LOG}"
if [[ "${FIXTURE_VERSION_OK:-1}" == 1 ]]; then
    echo 'pocket-ic-server 15.0.0'
else
    echo 'wrong version'
fi
EOF
chmod +x "${temporary_root}/candidate"
gzip -c "${temporary_root}/candidate" > "${IC_TIMERS_FIXTURE_ARCHIVE}"
cat > "${temporary_root}/bin/sha256sum" <<'EOF'
#!/usr/bin/env bash
cat >/dev/null
printf '%s\n' hash >> "${IC_TIMERS_FIXTURE_LOG}"
if [[ "${FIXTURE_SHA_FAIL:-0}" == 1 ]]; then exit 1; fi
if [[ "${FIXTURE_HASH_OK:-0}" == 1 ]]; then
    printf '%s  -\n' "${IC_TIMERS_FIXTURE_SHA}"
else
    printf '%064d  -\n' 0
fi
EOF
cat > "${temporary_root}/bin/curl" <<'EOF'
#!/usr/bin/env bash
while [[ $# -gt 0 ]]; do
    if [[ "$1" == --output ]]; then
        cp "${IC_TIMERS_FIXTURE_ARCHIVE}" "$2"
        exit
    fi
    shift
done
exit 1
EOF
cat > "${temporary_root}/bin/uname" <<'EOF'
#!/usr/bin/env bash
case "$1" in -s) echo Linux ;; -m) echo x86_64 ;; *) exit 1 ;; esac
EOF
chmod +x "${temporary_root}/bin/"*
export PATH="${temporary_root}/bin:${PATH}"
export POCKET_IC_BIN="${temporary_root}/cache/pocket-ic"
export POCKET_IC_AUTO_INSTALL=0
cp -p "${temporary_root}/candidate" "${POCKET_IC_BIN}"

reject_without_execution() {
    : > "${IC_TIMERS_FIXTURE_LOG}"
    if output="$(bash "${checker}" 2>&1)"; then
        echo 'error: unaudited PocketIC binary was accepted' >&2
        exit 1
    fi
    if grep -Fqx execute "${IC_TIMERS_FIXTURE_LOG}"; then
        echo "error: unaudited PocketIC binary executed: ${output}" >&2
        exit 1
    fi
    cmp "${temporary_root}/candidate" "${POCKET_IC_BIN}"
}

# Explicit bad overrides and unreadable hashes must fail without execution.
reject_without_execution
FIXTURE_SHA_FAIL=1 reject_without_execution
# A rejected automatic download must neither execute nor replace the cache.
POCKET_IC_AUTO_INSTALL=1 reject_without_execution
if [[ -n "$(find "${temporary_root}/cache" -name '.pocket-ic-install.*' -print)" ]]; then
    echo 'error: rejected download left installation debris' >&2
    exit 1
fi

: > "${IC_TIMERS_FIXTURE_LOG}"
FIXTURE_HASH_OK=1 bash "${checker}"
test "$(cat "${IC_TIMERS_FIXTURE_LOG}")" == $'hash\nexecute'
: > "${IC_TIMERS_FIXTURE_LOG}"
if FIXTURE_HASH_OK=1 FIXTURE_VERSION_OK=0 bash "${checker}" >/dev/null 2>&1; then
    echo 'error: accepted a hash-matching binary with the wrong version' >&2
    exit 1
fi
test "$(head -n 1 "${IC_TIMERS_FIXTURE_LOG}")" == hash
rm -- "${POCKET_IC_BIN}"
: > "${IC_TIMERS_FIXTURE_LOG}"
FIXTURE_HASH_OK=1 POCKET_IC_AUTO_INSTALL=1 bash "${checker}"
test "$(cat "${IC_TIMERS_FIXTURE_LOG}")" == $'hash\nexecute'
cmp "${temporary_root}/candidate" "${POCKET_IC_BIN}"
echo 'PocketIC hash-before-execution checks passed'
