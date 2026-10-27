#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-repository-check-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed repository-check fixture retained"' EXIT

git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{.githooks,scripts/{ci,dev,release},crates/ic-timers/src}
bash "$repository_root/scripts/ci/prepare-script-fixture.sh" "$repository_root" "$temporary_root" \
    Makefile make/consumer.mk scripts/ci/check-provider-boundary.sh scripts/ci/check-dependency-pins.sh
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
cd "${temporary_root}"

for script in .githooks/pre-commit scripts/ci/valid.sh scripts/dev/valid.sh scripts/release/valid.sh; do
    printf '%s\n' '#!/usr/bin/env bash' 'exit 0' > "${script}"
done

expect_failure() {
    local expected="${1}"
    shift
    local output
    if output="$("$@" 2>&1)"; then
        echo "error: unexpectedly accepted: $*" >&2
        exit 1
    fi
    if [[ "${output}" != *"${expected}"* ]]; then
        echo "error: failure did not contain ${expected}: ${output}" >&2
        exit 1
    fi
}

make --no-print-directory shell-check >/dev/null
for directory in scripts/ci scripts/dev scripts/release; do
    broken="${directory}/broken.sh"
    printf '%s\n' 'if then' > "${broken}"
    expect_failure "${broken}" make --no-print-directory shell-check
    rm -- "${broken}"
done

# Shared owns workflow discovery and failed-inventory permutations. Keep one
# real caller smoke check before the local orchestration and provider boundaries.
mkdir -p .github/workflows scan-bin
printf '%s\n' 'jobs:' '  test:' '    steps:' \
    '      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1' \
    > .github/workflows/ci.yml
bash scripts/ci/check-dependency-pins.sh --consumer "$temporary_root"
printf '%s\n' 'jobs: {test: {steps: [{uses: "actions/checkout@main"}]}}' \
    > .github/workflows/ci.yml
expect_failure 'action-ref' bash scripts/ci/check-dependency-pins.sh --consumer "$temporary_root"
rm .github/workflows/ci.yml

cat > scripts/release/classify-release-impact.sh <<'EOF'
#!/usr/bin/env bash
if [[ "${CLASSIFICATION:-repository}" == error ]]; then
    echo 'classification failed' >&2
    exit 1
fi
printf '%s\n' "${CLASSIFICATION:-repository}"
EOF
cat > overrides.mk <<'EOF'
# The shared formatter fixture owns tool admission; this fixture owns ordering.
format-tools-check host-tools-check:
	@:
actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package:
	@printf '%s\n' '$@' >> checks-ran
	@if [ '$@' = '$(FAIL_TARGET)' ]; then echo 'failed $@' >&2; exit 1; fi
EOF
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')

"${fixture_make[@]}" repository-check > /dev/null 2>&1
targets=(actions-check shell-check release-check provider-check fmt-check)
printf '%s\n' "${targets[@]}" > expected-checks
if ! cmp -s expected-checks checks-ran; then
    cat checks-ran >&2
    echo "error: repository-check skipped a required check" >&2
    exit 1
fi
: > expected-checks
for target in "${targets[@]}"; do
    printf '%s\n' "${target}" >> expected-checks
    rm -- checks-ran
    expect_failure "failed ${target}" "${fixture_make[@]}" repository-check "FAIL_TARGET=${target}"
    if ! cmp -s expected-checks checks-ran; then
        cat checks-ran >&2
        echo "error: repository-check skipped checks or continued after ${target} failed" >&2
        exit 1
    fi
done
rm -- checks-ran

# The release-gate fixture owns the complete CI roster and leaf failure order.
expect_failure 'classification failed' env CLASSIFICATION=error "${fixture_make[@]}" repository-check
expect_failure 'classification failed' env CLASSIFICATION=error "${fixture_make[@]}" release-impact
expect_failure 'crate-impacting changes' env CLASSIFICATION=crate "${fixture_make[@]}" repository-check
if [[ -e checks-ran ]]; then
    echo 'error: repository checks ran after impact classification failed or rejected crate work' >&2
    exit 1
fi

mkdir -p crates/ic-timers/src/platform
printf '%s\n' 'pub(crate) struct TimerHandle;' > crates/ic-timers/src/platform/mod.rs
printf '%s\n' '#![allow(unsafe_code)]' > crates/ic-timers/src/platform/ffi.rs
facade_header=('#![forbid(private_interfaces)]' '#![deny(unsafe_code)]' 'mod platform;')
printf '%s\n' "${facade_header[@]}" > crates/ic-timers/src/lib.rs
bash scripts/ci/check-provider-boundary.sh >/dev/null
printf '%s\n' 'mod platform;' > crates/ic-timers/src/lib.rs
expect_failure 'public interfaces' bash scripts/ci/check-provider-boundary.sh
printf '%s\n' "${facade_header[@]}" > crates/ic-timers/src/lib.rs
for operation in 'unsafe fn leak() {}' 'fn leak() { ic_platform::ic0::global_timer_set(1); }' \
    'fn leak() { ic_platform::executor::spawn_protected(async {}); }' \
    'use ic_platform::ic0; fn leak() { ic0::global_timer_set(1); }' \
    'use ic_platform::{executor as ex}; fn leak() { ex::spawn_protected(async {}); }' \
    'use ic_platform as platform_alias;'; do
    printf '%s\n' "${operation}" > crates/ic-timers/src/leak.rs
    if [[ "${operation}" == unsafe* ]]; then expected=FFI; else expected='private platform'; fi
    expect_failure "${expected}" bash scripts/ci/check-provider-boundary.sh
done
rm crates/ic-timers/src/leak.rs
for declaration in 'pub fn leaked() {}' 'pub async fn leaked() {}' 'pub mod leaked {}'; do
    printf '%s\n' "${declaration}" > crates/ic-timers/src/platform/mod.rs
    expect_failure 'restricted visibility' bash scripts/ci/check-provider-boundary.sh
done
printf '%s\n' '#![forbid(private_interfaces)]' '#![deny(unsafe_code)]' 'pub mod platform;' > crates/ic-timers/src/lib.rs
expect_failure 'private module' bash scripts/ci/check-provider-boundary.sh
# Rust resolves direct, grouped and chained aliases. No warning-denial flag is
# needed: private-interface leaks are forbidden by the facade itself.
printf '%s\n' 'pub(crate) struct TimerHandle;' 'pub(crate) fn set_timer() {}' > crates/ic-timers/src/platform/mod.rs
facade_header+=('mod snapshot { pub struct TimerEpoch; }' 'pub fn public_api() {}')
fixture_rustc=(rustc --edition=2024 --crate-type=lib --crate-name boundary_fixture
    -A dead_code -A unused_imports crates/ic-timers/src/lib.rs -o libboundary_fixture.rlib)
for export in \
    'pub use platform::TimerHandle;' \
    'pub use crate::platform::TimerHandle;' \
    'pub use crate::platform::TimerHandle as PublicHandle;' \
    'pub use self::platform::{TimerHandle, set_timer};' \
    'pub use crate::{snapshot::TimerEpoch, platform::{TimerHandle}};' \
    'use crate::platform as p; pub use p::TimerHandle;' \
    'use crate::platform as p; use p as q; pub use q::TimerHandle;' \
    'use crate::platform::{TimerHandle as H}; pub use H;' \
    'use crate::platform as p; pub use p as PublicPlatform;' \
    'use crate::platform::set_timer as arm; pub use arm as public_arm;' \
    'pub type PublicHandle = crate::platform::TimerHandle;' \
    'use crate::platform as p; pub type PublicHandle = p::TimerHandle;' \
    $'pub\nuse crate::{\n    platform::{TimerHandle},\n};'; do
    printf '%s\n' "${facade_header[@]}" "${export}" > crates/ic-timers/src/lib.rs
    case "${export}" in
        *'pub type'*) expected=private_interfaces ;;
        *) expected='cannot be re-exported' ;;
    esac
    expect_failure "${expected}" "${fixture_rustc[@]}"
done
for suppression in allow expect; do
    printf '%s\n' "${facade_header[@]}" "#[${suppression}(private_interfaces)]" \
        'pub type PublicHandle = crate::platform::TimerHandle;' > crates/ic-timers/src/lib.rs
    expect_failure 'forbid' "${fixture_rustc[@]}"
done

# A glob cannot widen the crate-visible items. Internal imports still work;
# an external caller can reach the normal API but neither provider item.
printf '%s\n' "${facade_header[@]}" 'pub use crate::platform::*;' \
    'pub(crate) use crate::platform::TimerHandle as InternalHandle;' > crates/ic-timers/src/lib.rs
"${fixture_rustc[@]}"
consumer_rustc=(rustc --edition=2024 --crate-type=lib
    --extern boundary_fixture=libboundary_fixture.rlib consumer.rs -o consumer.rlib)
printf '%s\n' 'pub fn check() { boundary_fixture::public_api(); }' > consumer.rs
"${consumer_rustc[@]}"
for item in TimerHandle set_timer; do
    printf 'use boundary_fixture::%s;\n' "${item}" > consumer.rs
    expect_failure "${item}" "${consumer_rustc[@]}"
done

# The owner checker rejects Candid codecs and missing Wire admission hooks.
probe_dir=testing/crates/ic-timers-runtime-probe/src
mkdir -p "$probe_dir"
for codec in 'use ic_platform::candid::DecodeLimits;' 'use candid::;' \
    'fn checkpoint() { stable_save(()); }'; do
    printf '%s\n' "$codec" > "$probe_dir/leak.rs"
    expect_failure 'probe protocols must use Wire' bash scripts/ci/check-provider-boundary.sh
done
printf '%s\n' '#[ic_platform::cdk::update]' 'fn leaked() {}' > "$probe_dir/leak.rs"
expect_failure 'admitted Wire hooks' bash scripts/ci/check-provider-boundary.sh
# Platform 0.2 requires explicit codecs and has no implicit Candid registration;
# the obsolete hidden flag is no longer necessary for valid Wire entrypoints.
cat > "$probe_dir/leak.rs" <<'EOF'
#[ic_platform::cdk::update(
    guard = "crate::codec::admit_request",
    decode_with = "crate::codec::decode",
    encode_with = "crate::codec::encode"
)]
fn admitted() {}
EOF
bash scripts/ci/check-provider-boundary.sh >/dev/null
perl -ni -e 'print unless /guard =/' "$probe_dir/leak.rs"
expect_failure 'admitted Wire hooks' bash scripts/ci/check-provider-boundary.sh
rm "$probe_dir/leak.rs"
bash scripts/ci/check-provider-boundary.sh >/dev/null

fixture_complete=true
echo 'Repository check regression tests passed'
