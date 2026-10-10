#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
export PATH="$root/.tools/host/bin:$PATH"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/timer-testkit-server.XXXXXX")"
fixture="$(cd -P "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
trap 'if [[ $? == 0 ]]; then rm -rf "$fixture"; else printf "Failed Testkit adapter fixture retained: %s\n" "$fixture" >&2; fi' EXIT
consumer="$fixture/consumer with spaces"
mkdir -p "$consumer/scripts/dev" "$consumer/.tools/host/bin"
cp "$root/scripts/dev/testkit-server.sh" "$consumer/scripts/dev/"
export TESTKIT_ADAPTER_FIXTURE="$fixture"
cat > "$fixture/lock" <<'EOF'
version = 4
[[package]]
name = "ic-testkit"
version = "0.26.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
EOF
cp "$fixture/lock" "$consumer/Cargo.lock"
cat > "$consumer/scripts/dev/install-rust-tools.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$@" > "$TESTKIT_ADAPTER_FIXTURE/installer-arguments"
printf '%s\n' "${CARGO_NET_OFFLINE:-unset}" > "$TESTKIT_ADAPTER_FIXTURE/installer-offline"
exit_status="${FIXTURE_INSTALLER_STATUS:-0}"
if [[ "$exit_status" != 0 ]]; then
    echo 'fixture selected Cargo tool refused' >&2
    # A plausible partial path on stdout must not authorize owner execution.
    printf '%s\n' "$TESTKIT_ADAPTER_FIXTURE/owner"
    exit "$exit_status"
fi
printf '%s\n' "$TESTKIT_ADAPTER_FIXTURE/owner"
EOF
cat > "$fixture/owner" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$@" > "$TESTKIT_ADAPTER_FIXTURE/owner-arguments"
printf '%s\n' "${CARGO_NET_OFFLINE:-unset}" > "$TESTKIT_ADAPTER_FIXTURE/offline"
exit_status="${FIXTURE_OWNER_STATUS:-0}"
[[ "$exit_status" == 0 ]] || exit "$exit_status"
printf '%s\n' "$TESTKIT_ADAPTER_FIXTURE/admitted-server"
EOF
chmod +x "$fixture/owner"
for mode in setup check; do
    env -u CARGO_NET_OFFLINE bash "$consumer/scripts/dev/testkit-server.sh" "$mode" > "$fixture/path"
    printf '%s\n' "$fixture/admitted-server" > "$fixture/expected-path"
    cmp "$fixture/expected-path" "$fixture/path"
    printf '%s\n' --consumer "$consumer" --package ic-testkit --version 0.26.0 \
        --bin ic-testkit-server --profile release > "$fixture/expected-installer"
    [[ "$mode" != check ]] || printf '%s\n' --check >> "$fixture/expected-installer"
    cmp "$fixture/expected-installer" "$fixture/installer-arguments"
    printf '%s\n' "$mode" --directory "$consumer/.tools/testkit-server" > "$fixture/expected-owner"
    cmp "$fixture/expected-owner" "$fixture/owner-arguments"
    if [[ "$mode" == check ]]; then expected=true; else expected=unset; fi
    printf '%s\n' "$expected" > "$fixture/expected-offline"
    cmp "$fixture/expected-offline" "$fixture/offline"
    cmp "$fixture/expected-offline" "$fixture/installer-offline"
done
# A failed offline installation admission must not execute the owner or set up.
rm "$fixture/owner-arguments"
status=0
FIXTURE_INSTALLER_STATUS=23 bash "$consumer/scripts/dev/testkit-server.sh" check > "$fixture/failure.out" 2> "$fixture/failure.log" || status=$?
test "$status" -eq 23
test ! -s "$fixture/failure.out"
grep -Fq 'fixture selected Cargo tool refused' "$fixture/failure.log"
grep -Fq 'run make install-testkit-server' "$fixture/failure.log"
test ! -e "$fixture/owner-arguments"
status=0
FIXTURE_OWNER_STATUS=31 bash "$consumer/scripts/dev/testkit-server.sh" check > "$fixture/failure.log" 2>&1 || status=$?
test "$status" -eq 31

# Setup inherits explicit offline policy, and a changed lock selects only the
# new CLI. The shared installer owns installation reuse and receipt admission.
CARGO_NET_OFFLINE=true bash "$consumer/scripts/dev/testkit-server.sh" setup > "$fixture/path"
printf 'true\n' > "$fixture/expected-offline"
cmp "$fixture/expected-offline" "$fixture/installer-offline"
cmp "$fixture/expected-offline" "$fixture/offline"
sed 's/0.26.0/0.27.0/' "$fixture/lock" > "$consumer/Cargo.lock"
cp "$consumer/Cargo.lock" "$fixture/next-lock"
bash "$consumer/scripts/dev/testkit-server.sh" setup > "$fixture/path"
printf '%s\n' --consumer "$consumer" --package ic-testkit --version 0.27.0 \
    --bin ic-testkit-server --profile release > "$fixture/expected-installer"
cmp "$fixture/expected-installer" "$fixture/installer-arguments"
cmp "$fixture/expected-path" "$fixture/path"
cmp "$fixture/next-lock" "$consumer/Cargo.lock"

for invalid in missing duplicate unqualified; do
    case "$invalid" in
        missing) printf 'version = 4\npackage = []\n' > "$consumer/Cargo.lock" ;;
        duplicate) cat "$fixture/lock" > "$consumer/Cargo.lock"; sed '1d' "$fixture/lock" >> "$consumer/Cargo.lock" ;;
        unqualified) sed 's/registry+https:\/\/github.com\/rust-lang\/crates.io-index/path/' "$fixture/lock" > "$consumer/Cargo.lock" ;;
    esac
    rm "$fixture/installer-arguments" "$fixture/owner-arguments" 2>/dev/null || :
    if bash "$consumer/scripts/dev/testkit-server.sh" check > "$fixture/invalid.log" 2>&1; then
        echo "error: invalid $invalid Testkit selection was accepted" >&2; exit 1
    fi
    test ! -e "$fixture/installer-arguments"
    test ! -e "$fixture/owner-arguments"
done
echo 'Testkit adapter setup/offline admission, lock selection and failure propagation passed'
