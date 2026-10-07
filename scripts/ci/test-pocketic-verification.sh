#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
checker="${repository_root}/scripts/ci/check-pocketic.sh"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
mkdir -p "${temporary_root}"/{bin,cache}
export IC_TIMERS_FIXTURE_LOG="${temporary_root}/events"
export IC_TIMERS_FIXTURE_ARCHIVE="${temporary_root}/download.gz"
IC_TIMERS_FIXTURE_GZIP="$(command -v gzip)"
export IC_TIMERS_FIXTURE_GZIP
cat > "${temporary_root}/candidate" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' execute >> "${IC_TIMERS_FIXTURE_LOG}"
if [[ "${FIXTURE_VERSION_OK:-1}" == 1 ]]; then
    echo "${FIXTURE_OBSERVED_VERSION:-pocket-ic-server 16.0.0}"
else
    echo 'wrong version'
fi
exit "${FIXTURE_VERSION_STATUS:-0}"
EOF
chmod 0755 "${temporary_root}/candidate"
cp "${temporary_root}/candidate" "${temporary_root}/cached-candidate"
printf '%s\n' '# Existing cache sentinel.' >> "${temporary_root}/cached-candidate"
chmod 0750 "${temporary_root}/cached-candidate"
gzip -c "${temporary_root}/candidate" > "${IC_TIMERS_FIXTURE_ARCHIVE}"
cat > "${temporary_root}/bin/sha256sum" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
# The canonical checksum owner hashes stdin and admits a checksum record. This
# substitute controls digest/status only; it never replaces the admission owner.
cat > "${IC_TIMERS_FIXTURE_LOG}.checksum-input"
if cmp -s "${IC_TIMERS_FIXTURE_LOG}.checksum-input" "${IC_TIMERS_FIXTURE_ARCHIVE}"; then
    stage=archive; digest="${IC_TIMERS_FIXTURE_ARCHIVE_SHA}"; valid="${FIXTURE_ARCHIVE_HASH_OK:-1}"
else
    stage=binary; digest="${IC_TIMERS_FIXTURE_BINARY_SHA}"; valid="${FIXTURE_BINARY_HASH_OK:-0}"
fi
printf '%s\n' "${stage}-hash" >> "${IC_TIMERS_FIXTURE_LOG}"
if [[ "${valid}" != 1 ]]; then digest="$(printf '%064d' 0)"; fi
# Plausible output from a failed producer must still reject the artifact.
printf '%s  -\n' "${digest}"
if [[ "${FIXTURE_HASH_FAIL:-}" == "${stage}" ]]; then exit 1; fi
EOF
cat > "${temporary_root}/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' download >> "${IC_TIMERS_FIXTURE_LOG}"
output=''
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output) output="$2"; shift ;;
        https://*) [[ "$1" == "${IC_TIMERS_FIXTURE_URL}" ]] ;;
    esac
    shift
done
cp "${IC_TIMERS_FIXTURE_ARCHIVE}" "${output}"
if [[ "${FIXTURE_DOWNLOAD_FAIL:-0}" == 1 ]]; then exit 22; fi
EOF
cat > "${temporary_root}/bin/gzip" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' decompress >> "${IC_TIMERS_FIXTURE_LOG}"
"${IC_TIMERS_FIXTURE_GZIP}" "$@"
if [[ "${FIXTURE_DECOMPRESS_FAIL:-0}" == 1 ]]; then exit 1; fi
EOF
cat > "${temporary_root}/bin/uname" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FIXTURE_UNAME_FAIL:-}" == "$1" ]]; then
    echo 'injected host query failure' >&2
    exit 1
fi
case "$1" in -s) echo "${IC_TIMERS_FIXTURE_OS}" ;; -m) echo "${IC_TIMERS_FIXTURE_ARCH}" ;; *) exit 1 ;; esac
EOF
chmod +x "${temporary_root}/bin/"*
export PATH="${temporary_root}/bin:${PATH}"
export POCKET_IC_BIN="${temporary_root}/cache/pocket-ic"
export POCKET_IC_AUTO_INSTALL=0

expect_events() {
    printf '%s\n' "$@" > "${temporary_root}/expected-events"
    cmp "${temporary_root}/expected-events" "${IC_TIMERS_FIXTURE_LOG}"
}

assert_cache_preserved() {
    local expected_cache="${1:-${temporary_root}/cached-candidate}"
    cmp "${expected_cache}" "${POCKET_IC_BIN}"
    perl -e '
        my @before = stat $ARGV[0];
        my @after = stat $ARGV[1];
        @before && @after or die "cannot read cache modes\n";
        ($before[2] & 07777) == ($after[2] & 07777)
            or die "PocketIC cache mode changed\n";
    ' "${expected_cache}" "${POCKET_IC_BIN}"
    # Capture producer status before treating an empty record as no debris.
    find "${temporary_root}/cache" -name '.pocket-ic-install.*' -print > "${temporary_root}/installation-debris"
    if [[ -s "${temporary_root}/installation-debris" ]]; then
        echo 'error: rejected download left installation debris' >&2
        exit 1
    fi
}

reject_without_execution() {
    : > "${IC_TIMERS_FIXTURE_LOG}"
    local output
    if output="$(bash "${checker}" 2>&1)"; then
        echo 'error: unaudited PocketIC artifact was accepted' >&2
        exit 1
    fi
    expect_events "$@"
    assert_cache_preserved
}

# Independent pins exercise actual host selection, URLs and artifact admission;
# they are not read from the implementation under test.
for host in x86_64-linux x86_64-darwin arm64-darwin; do
    case "${host}" in
        x86_64-linux)
            IC_TIMERS_FIXTURE_OS=Linux; IC_TIMERS_FIXTURE_ARCH=x86_64
            IC_TIMERS_FIXTURE_ARCHIVE_SHA=268ba79ec7fe9a563a575adf4983c69627093cce2711d142e476cdc7ad04249e
            IC_TIMERS_FIXTURE_BINARY_SHA=69e324bdb68d32d878b7a9504b1379f08f8d1921272bacb065b0fabb3d0f3792 ;;
        x86_64-darwin)
            IC_TIMERS_FIXTURE_OS=Darwin; IC_TIMERS_FIXTURE_ARCH=x86_64
            IC_TIMERS_FIXTURE_ARCHIVE_SHA=9710b9c4ac4eaa7eb10bddaa2aba80560a59362610f1bcd8c6e23be82a39c327
            IC_TIMERS_FIXTURE_BINARY_SHA=b8233ebee53452db7465b43e7b2ff80f2e1445dc148eb2b4b237493d8d15ec66 ;;
        arm64-darwin)
            IC_TIMERS_FIXTURE_OS=Darwin; IC_TIMERS_FIXTURE_ARCH=arm64
            IC_TIMERS_FIXTURE_ARCHIVE_SHA=41cf77e24effc381e21f5e07e908ed078783646e6de05ed52fd6973221f07e64
            IC_TIMERS_FIXTURE_BINARY_SHA=781f643d4b16105e7544ca810a972f99c0ef1919016c680faa93f10909a14496 ;;
    esac
    IC_TIMERS_FIXTURE_URL="https://github.com/dfinity/pocketic/releases/download/16.0.0/pocket-ic-${host}.gz"
    export IC_TIMERS_FIXTURE_OS IC_TIMERS_FIXTURE_ARCH IC_TIMERS_FIXTURE_ARCHIVE_SHA
    export IC_TIMERS_FIXTURE_BINARY_SHA IC_TIMERS_FIXTURE_URL
    cp -p "${temporary_root}/cached-candidate" "${POCKET_IC_BIN}"

    # Strict overrides never download, even when missing or hash-mismatched.
    reject_without_execution binary-hash
    FIXTURE_BINARY_HASH_OK=1 FIXTURE_HASH_FAIL=binary reject_without_execution binary-hash
    rm -- "${POCKET_IC_BIN}"
    : > "${IC_TIMERS_FIXTURE_LOG}"
    if bash "${checker}" >/dev/null 2>&1; then
        echo 'error: missing override was accepted' >&2
        exit 1
    fi
    test ! -s "${IC_TIMERS_FIXTURE_LOG}"
    test ! -e "${POCKET_IC_BIN}"
    cp -p "${temporary_root}/cached-candidate" "${POCKET_IC_BIN}"

    # Download/archive failures cannot reach decompression or binary execution.
    POCKET_IC_AUTO_INSTALL=1 FIXTURE_DOWNLOAD_FAIL=1 \
        reject_without_execution binary-hash download
    POCKET_IC_AUTO_INSTALL=1 FIXTURE_ARCHIVE_HASH_OK=0 \
        reject_without_execution binary-hash download archive-hash
    POCKET_IC_AUTO_INSTALL=1 FIXTURE_HASH_FAIL=archive \
        reject_without_execution binary-hash download archive-hash
    POCKET_IC_AUTO_INSTALL=1 FIXTURE_DECOMPRESS_FAIL=1 \
        reject_without_execution binary-hash download archive-hash decompress
    POCKET_IC_AUTO_INSTALL=1 \
        reject_without_execution binary-hash download archive-hash decompress binary-hash
    POCKET_IC_AUTO_INSTALL=1 FIXTURE_BINARY_HASH_OK=1 FIXTURE_HASH_FAIL=binary \
        reject_without_execution binary-hash download archive-hash decompress binary-hash

    : > "${IC_TIMERS_FIXTURE_LOG}"
    FIXTURE_BINARY_HASH_OK=1 bash "${checker}"
    expect_events binary-hash execute
    for result in wrong-version old-server-version failed-command; do
        : > "${IC_TIMERS_FIXTURE_LOG}"
        version_ok=1; version_status=0; observed_version='pocket-ic-server 16.0.0'
        case "${result}" in
            wrong-version) version_ok=0 ;;
            old-server-version) observed_version='pocket-ic-server 15.0.0' ;;
            failed-command) version_status=1 ;;
        esac
        if output="$(FIXTURE_BINARY_HASH_OK=1 FIXTURE_VERSION_OK="${version_ok}" \
            FIXTURE_VERSION_STATUS="${version_status}" FIXTURE_OBSERVED_VERSION="${observed_version}" \
            bash "${checker}" 2>&1)"; then
            echo "error: accepted a hash-matching binary with ${result}" >&2
            exit 1
        fi
        expect_events binary-hash execute
        if [[ "${result}" == wrong-version && "${output}" != *'PocketIC version mismatch'* ]]; then
            echo 'error: rejection did not identify a version mismatch' >&2
            exit 1
        fi
        if [[ "${result}" == old-server-version && "${output}" != *'PocketIC version mismatch'* ]]; then
            echo 'error: rejection did not identify the old server version mismatch' >&2
            exit 1
        fi
    done

    : > "${IC_TIMERS_FIXTURE_LOG}"
    if FIXTURE_BINARY_HASH_OK=1 FIXTURE_VERSION_OK=0 POCKET_IC_AUTO_INSTALL=1 \
        bash "${checker}" >/dev/null 2>&1; then
        echo 'error: downloaded a hash-matching binary with the wrong version' >&2
        exit 1
    fi
    expect_events binary-hash execute download archive-hash decompress binary-hash execute
    assert_cache_preserved
    rm -- "${POCKET_IC_BIN}"
    : > "${IC_TIMERS_FIXTURE_LOG}"
    FIXTURE_BINARY_HASH_OK=1 POCKET_IC_AUTO_INSTALL=1 bash "${checker}"
    expect_events download archive-hash decompress binary-hash execute
    assert_cache_preserved "${temporary_root}/candidate"
done

# Invalid cache types must reject before download. A directory destination must
# not receive a nested executable, and rejected links retain their exact target.
rm -- "${POCKET_IC_BIN}"
for kind in directory fifo file-link directory-link dangling-link; do
    link_target=''
    case "${kind}" in
        directory) mkdir "${POCKET_IC_BIN}" ;;
        fifo) mkfifo "${POCKET_IC_BIN}" ;;
        file-link) link_target="${temporary_root}/cached-candidate" ;;
        directory-link)
            link_target="${temporary_root}/linked-cache"
            mkdir "${link_target}" ;;
        dangling-link) link_target="${temporary_root}/absent-cache" ;;
    esac
    if [[ -n "${link_target}" ]]; then ln -s "${link_target}" "${POCKET_IC_BIN}"; fi
    for automatic in 0 1; do
        : > "${IC_TIMERS_FIXTURE_LOG}"
        if output="$(POCKET_IC_AUTO_INSTALL="${automatic}" bash "${checker}" 2>&1)"; then
            echo "error: accepted ${kind} cache with automatic=${automatic}" >&2
            exit 1
        fi
        if [[ "${automatic}" == 1 && "${output}" != *'needs a regular cache file or absent path'* ]]; then
            echo "error: ${kind} cache reached installation: ${output}" >&2
            exit 1
        fi
        if [[ "${kind}" == file-link ]]; then
            expect_events binary-hash
        else
            test ! -s "${IC_TIMERS_FIXTURE_LOG}"
        fi
        case "${kind}" in
            directory) test -d "${POCKET_IC_BIN}" ;;
            fifo) test -p "${POCKET_IC_BIN}" ;;
            *)
                test -L "${POCKET_IC_BIN}"
                actual_target="$(readlink "${POCKET_IC_BIN}")"
                test "${actual_target}" = "${link_target}" ;;
        esac
    done
    if [[ "${kind}" == directory ]]; then rmdir "${POCKET_IC_BIN}"; else rm "${POCKET_IC_BIN}"; fi
    if [[ "${kind}" == directory-link ]]; then rmdir "${link_target}"; fi
done

# A pinned executable reached through a symlink remains admissible without
# provisioning or replacing the link, in explicit and automatic selection.
ln -s "${temporary_root}/candidate" "${POCKET_IC_BIN}"
for automatic in 0 1; do
    : > "${IC_TIMERS_FIXTURE_LOG}"
    FIXTURE_BINARY_HASH_OK=1 POCKET_IC_AUTO_INSTALL="${automatic}" bash "${checker}"
    expect_events binary-hash execute
    test -L "${POCKET_IC_BIN}"
    actual_target="$(readlink "${POCKET_IC_BIN}")"
    test "${actual_target}" = "${temporary_root}/candidate"
done
rm "${POCKET_IC_BIN}"

# Unknown hosts and failed host queries reject before inspecting/executing a cache.
cp -p "${temporary_root}/cached-candidate" "${POCKET_IC_BIN}"
for failure in unknown-os unknown-arch os-query arch-query; do
    : > "${IC_TIMERS_FIXTURE_LOG}"
    case "${failure}" in
        unknown-os) IC_TIMERS_FIXTURE_OS=Unknown; IC_TIMERS_FIXTURE_ARCH=arm64 ;;
        unknown-arch) IC_TIMERS_FIXTURE_OS=Darwin; IC_TIMERS_FIXTURE_ARCH=unknown ;;
        *) IC_TIMERS_FIXTURE_OS=Darwin; IC_TIMERS_FIXTURE_ARCH=arm64 ;;
    esac
    failed_query=''
    if [[ "${failure}" == os-query ]]; then failed_query=-s; fi
    if [[ "${failure}" == arch-query ]]; then failed_query=-m; fi
    if output="$(FIXTURE_UNAME_FAIL="${failed_query}" FIXTURE_BINARY_HASH_OK=1 \
        POCKET_IC_AUTO_INSTALL=1 bash "${checker}" 2>&1)"; then
        echo "error: accepted ${failure}" >&2
        exit 1
    fi
    test ! -s "${IC_TIMERS_FIXTURE_LOG}"
    assert_cache_preserved
    if [[ "${failure}" == *-query && "${output}" != *'injected host query failure'* ]]; then
        echo 'error: failed host query was hidden' >&2
        exit 1
    fi
done

echo 'PocketIC archive-before-decompression and host-pinned binary checks passed'
