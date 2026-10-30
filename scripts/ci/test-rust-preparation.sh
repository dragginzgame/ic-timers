#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/timer-rust-preparation.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Failed Rust preparation fixture retained"' EXIT

export PATH="$root/.tools/host/bin:$PATH"
real_make="$(command -v make)"
mkdir -p "$fixture/repo" "$fixture/bin"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture/repo" \
    Makefile make/consumer.mk Cargo.toml rust-toolchain.toml scripts/dev/update-dev.sh
cat > "$fixture/bin/rustup" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$PREPARATION_CALLS"
if [[ "$3" == "$PREPARATION_FAIL_VERSION" ]]; then
    echo 'injected Rust installation failure' >&2
    exit 49
fi
STUB
cat > "$fixture/bin/make" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "${*: -1}" >> "$PREPARATION_CALLS"
[[ "${*: -1}" != "$PREPARATION_FAIL_TARGET" ]] || exit 53
STUB
chmod +x "$fixture/bin/"*
export PATH="$fixture/bin:$PATH" PREPARATION_CALLS="$fixture/calls"
export PREPARATION_FAIL_VERSION='' PREPARATION_FAIL_TARGET=''
# Change the canonical files, without editing Make or CI. All actual callers
# must observe these fixture values rather than their previous literal pins.
perl -pi -e 's/^channel = .*/channel = "1.99.7"/' "$fixture/repo/rust-toolchain.toml"
perl -pi -e 's/^rust-version = .*/rust-version = "1.91.6"/' "$fixture/repo/Cargo.toml"
development=1.99.7
minimum=1.91.6
expected_install() {
    printf 'toolchain install %s --profile minimal --component clippy --component rustfmt --target wasm32-unknown-unknown\n' "$1"
}
# The real Make adapter passes its selected minimum; the updater must prepare
# both compilers before common tools. No real rustup, tools or hooks are called.
for failure in success host development minimum tools; do
    PREPARATION_FAIL_VERSION='' PREPARATION_FAIL_TARGET=''
    expected=0
    case "$failure" in
        host) PREPARATION_FAIL_TARGET=install-host-tools; expected=2 ;;
        development) PREPARATION_FAIL_VERSION="$development"; expected=2 ;;
        minimum) PREPARATION_FAIL_VERSION=1.91.7; expected=2 ;;
        tools) PREPARATION_FAIL_TARGET=install-tools; expected=2 ;;
    esac
    : > "$fixture/calls"
    status=0
    "$real_make" --no-print-directory -C "$fixture/repo" update-dev MSRV=1.91.7 \
        > "$fixture/output" 2>&1 || status=$?
    [[ "$status" == "$expected" ]] || exit 1
    printf 'install-host-tools\n' > "$fixture/expected"
    if [[ "$failure" != host ]]; then expected_install "$development" >> "$fixture/expected"; fi
    if [[ "$failure" != host && "$failure" != development ]]; then expected_install 1.91.7 >> "$fixture/expected"; fi
    if [[ "$failure" == tools || "$failure" == success ]]; then printf 'install-tools\n' >> "$fixture/expected"; fi
    if [[ "$failure" == success ]]; then printf 'tools-check\ninstall-hooks\n' >> "$fixture/expected"; fi
    cmp "$fixture/expected" "$fixture/calls"
done
PREPARATION_FAIL_VERSION='' PREPARATION_FAIL_TARGET=''
# Default Make selection and the command/environment override remain distinct.
for selection in default environment; do
    : > "$fixture/calls"
    if [[ "$selection" == environment ]]; then
        MSRV=1.91.8 "$real_make" --no-print-directory -C "$fixture/repo" update-dev > "$fixture/output" 2>&1
        selected=1.91.8
    else
        env -u MSRV "$real_make" --no-print-directory -C "$fixture/repo" update-dev > "$fixture/output" 2>&1
        selected="$minimum"
    fi
    { printf 'install-host-tools\n'; expected_install "$development"; expected_install "$selected";
      printf 'install-tools\ntools-check\ninstall-hooks\n'; } > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/calls"
done
# Read the actual Make default without invoking Cargo, installation or a gate.
# shellcheck disable=SC2016 # Preserve Make's variable reference in the fixture.
printf 'fixture-minimum:\n\t@printf "%%s\\n" "$(MSRV)"\n' > "$fixture/minimum.mk"
env -u MSRV "$real_make" --no-print-directory -C "$fixture/repo" -f Makefile -f "$fixture/minimum.mk" fixture-minimum > "$fixture/minimum"
printf '%s\n' "$minimum" > "$fixture/expected-minimum"
cmp "$fixture/expected-minimum" "$fixture/minimum"
# Invalid minimum input refuses before the development installation too.
for invalid in '' '--help' 'two words'; do
    : > "$fixture/calls"
    status=0
    "$BASH" "$fixture/repo/scripts/dev/update-dev.sh" "$invalid" > "$fixture/output" 2>&1 || status=$?
    [[ "$status" == 2 && ! -s "$fixture/calls" ]] || exit 1
done
# Execute the actual workflow bodies with the same substitute installer. Keep
# each lane's compiler selection and the tag lane's narrower component policy.
yq -o json '.' "$root/.github/workflows/ci.yml" > "$fixture/workflow.json"
jq -e '(.env // {}) | (has("IC_TIMERS_MSRV") or has("IC_TIMERS_INTERNAL_TOOLCHAIN")) | not' "$fixture/workflow.json" > /dev/null
export GITHUB_WORKSPACE="$fixture/repo" RUNNER_TEMP="$fixture" GITHUB_ENV="$fixture/github-env"
for entry in msrv:minimum checks:development macos:minimum macos:development tag-truth:development; do
    job="${entry%:*}" selection="${entry#*:}"
    jq --arg job "$job" '.jobs[$job].steps' "$fixture/workflow.json" > "$fixture/steps.json"
    jq -e '[.[] | select(.id == "rust-policy")] | length == 1' "$fixture/steps.json" > /dev/null
    jq -e 'to_entries as $steps |
        ($steps | map(select(.value.id == "rust-policy"))[0].key) as $policy |
        all($steps[] | select((.value.id // "") | startswith("prepare-rust-")); .key > $policy)' \
        "$fixture/steps.json" > /dev/null
    "$BASH" "$root/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" rust-policy > "$fixture/policy.sh"
    "$BASH" "$root/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" "prepare-rust-$selection" > "$fixture/step.sh"
    : > "$GITHUB_ENV"
    : > "$fixture/calls"
    (cd "$fixture/repo" && "$BASH" -e -o pipefail "$fixture/policy.sh")
    printf 'IC_TIMERS_MSRV=%s\nIC_TIMERS_INTERNAL_TOOLCHAIN=%s\n' "$minimum" "$development" > "$fixture/expected-policy"
    cmp "$fixture/expected-policy" "$GITHUB_ENV"
    # The tested reader emits only admitted assignments; execute the actual
    # preparation step using precisely the values GitHub would make available.
    # shellcheck source=/dev/null # Assignments are verified against expected-policy above.
    (cd "$fixture/repo" && set -a && source "$GITHUB_ENV" && "$BASH" -e -o pipefail "$fixture/step.sh")
    version="$development"
    [[ "$selection" != minimum ]] || version="$minimum"
    printf 'install-host-tools\n' > "$fixture/expected"
    if [[ "$job" == tag-truth ]]; then
        printf 'toolchain install %s --profile minimal --component rustfmt\n' "$version" >> "$fixture/expected"
    else expected_install "$version" >> "$fixture/expected"
    fi
    cmp "$fixture/expected" "$fixture/calls"
done
# Failed/malformed policy observations publish nothing and cannot select a
# fallback compiler. No real tools or compiler installations run in this suite.
for invalid in minimum development; do
    cp "$fixture/repo/Cargo.toml" "$fixture/manifest"
    cp "$fixture/repo/rust-toolchain.toml" "$fixture/toolchain"
    if [[ "$invalid" == minimum ]]; then
        perl -pi -e 's/^rust-version = .*/rust-version = "invalid"/' "$fixture/repo/Cargo.toml"
    else
        perl -pi -e 's/^channel = .*/channel = "bad channel"/' "$fixture/repo/rust-toolchain.toml"
    fi
    status=0
    bash "$fixture/repo/scripts/ci/read-rust-policy.sh" > "$fixture/invalid.out" 2> "$fixture/invalid.log" || status=$?
    [[ "$status" != 0 && ! -s "$fixture/invalid.out" ]] || exit 1
    if [[ "$invalid" == minimum ]]; then
        if env -u MSRV "$real_make" --no-print-directory -C "$fixture/repo" -f Makefile -f "$fixture/minimum.mk" fixture-minimum > "$fixture/refused" 2>&1; then exit 1; fi
    fi
    cp "$fixture/manifest" "$fixture/repo/Cargo.toml"
    cp "$fixture/toolchain" "$fixture/repo/rust-toolchain.toml"
done
# A parser can emit plausible JSON before failing; neither observation may be
# published. Keep the real parser for the other policy file in each case.
PREPARATION_REAL_YQ="$(command -v yq)"
export PREPARATION_REAL_YQ
mkdir -p "$fixture/repo/.tools/host/bin"
cat > "$fixture/repo/.tools/host/bin/yq" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${*: -1}" == *"$PREPARATION_FAILED_POLICY" ]]; then
    printf '%s\n' '{"workspace":{"package":{"rust-version":"1.91.6"}},"toolchain":{"channel":"1.99.7"}}'
    exit 37
fi
exec "$PREPARATION_REAL_YQ" "$@"
STUB
chmod +x "$fixture/repo/.tools/host/bin/yq"
for file in Cargo.toml rust-toolchain.toml; do
    status=0
    PREPARATION_FAILED_POLICY="$file" bash "$fixture/repo/scripts/ci/read-rust-policy.sh" \
        > "$fixture/partial.out" 2> "$fixture/partial.log" || status=$?
    [[ "$status" == 37 && ! -s "$fixture/partial.out" ]] || exit 1
done
fixture_complete=true
printf 'Explicit development/MSRV preparation, failure order and workflow callers passed\n'
