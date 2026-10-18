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
    Makefile make/consumer.mk rust-toolchain.toml scripts/dev/update-dev.sh
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
development="$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/rust-toolchain.toml")"
minimum="$(sed -n 's/^MSRV ?= //p' "$root/Makefile")"
expected_install() {
    printf 'toolchain install %s --profile minimal --component clippy --component rustfmt --target wasm32-unknown-unknown\n' "$1"
}
# The real Make adapter passes its selected minimum; the updater must prepare
# both compilers before common tools. No real rustup, tools or hooks are called.
for failure in success development minimum tools; do
    PREPARATION_FAIL_VERSION='' PREPARATION_FAIL_TARGET=''
    expected=0
    case "$failure" in
        development) PREPARATION_FAIL_VERSION="$development"; expected=2 ;;
        minimum) PREPARATION_FAIL_VERSION=1.91.7; expected=2 ;;
        tools) PREPARATION_FAIL_TARGET=install-tools; expected=2 ;;
    esac
    : > "$fixture/calls"
    status=0
    "$real_make" --no-print-directory -C "$fixture/repo" update-dev MSRV=1.91.7 \
        > "$fixture/output" 2>&1 || status=$?
    [[ "$status" == "$expected" ]] || exit 1
    expected_install "$development" > "$fixture/expected"
    if [[ "$failure" != development ]]; then expected_install 1.91.7 >> "$fixture/expected"; fi
    if [[ "$failure" == tools || "$failure" == success ]]; then printf 'install-tools\n' >> "$fixture/expected"; fi
    if [[ "$failure" == success ]]; then printf 'tools-check\ninstall-hooks\n' >> "$fixture/expected"; fi
    cmp "$fixture/expected" "$fixture/calls"
done
PREPARATION_FAIL_VERSION='' PREPARATION_FAIL_TARGET=''
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
export IC_TIMERS_MSRV="$minimum" IC_TIMERS_INTERNAL_TOOLCHAIN="$development"
for entry in msrv:minimum checks:development macos:minimum macos:development tag-truth:development; do
    job="${entry%:*}" selection="${entry#*:}"
    jq --arg job "$job" '.jobs[$job].steps' "$fixture/workflow.json" > "$fixture/steps.json"
    "$BASH" "$root/scripts/ci/extract-workflow-step.sh" "$fixture/steps.json" "prepare-rust-$selection" > "$fixture/step.sh"
    : > "$fixture/calls"
    (cd "$fixture/repo" && "$BASH" -e -o pipefail "$fixture/step.sh")
    version="$development"
    [[ "$selection" != minimum ]] || version="$minimum"
    if [[ "$job" == tag-truth ]]; then
        printf 'toolchain install %s --profile minimal --component rustfmt\n' "$version" > "$fixture/expected"
    else expected_install "$version" > "$fixture/expected"
    fi
    cmp "$fixture/expected" "$fixture/calls"
done
fixture_complete=true
printf 'Explicit development/MSRV preparation, failure order and workflow callers passed\n'
