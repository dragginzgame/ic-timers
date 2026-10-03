#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT

git init -q "${temporary_root}"
mkdir -p "${temporary_root}"/{.githooks,scripts/{ci,dev,release},crates/ic-timers/src}
cp "${repository_root}/Makefile" "${temporary_root}/Makefile"
cp "${repository_root}/scripts/ci/check-provider-boundary.sh" "${temporary_root}/scripts/ci/"
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
for directory in ci dev release; do
    broken="scripts/${directory}/broken.sh"
    printf '%s\n' 'if then' > "${broken}"
    expect_failure "${broken}" make --no-print-directory shell-check
    rm -- "${broken}"
done

cat > scripts/release/classify-release-impact.sh <<'EOF'
#!/usr/bin/env bash
if [[ "${CLASSIFICATION:-repository}" == error ]]; then
    echo 'classification failed' >&2
    exit 1
fi
printf '%s\n' "${CLASSIFICATION:-repository}"
EOF
cat > overrides.mk <<'EOF'
actions-check shell-check release-check provider-check fmt-check:
	@printf '%s\n' '$@' >> checks-ran
	@if [ '$@' = '$(FAIL_TARGET)' ]; then echo 'failed $@' >&2; exit 1; fi
EOF
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')

"${fixture_make[@]}" repository-check > /dev/null 2>&1
mapfile -t targets < checks-ran
if [[ "${targets[*]}" != 'actions-check shell-check release-check provider-check fmt-check' ]]; then
    echo "error: repository-check skipped a required check" >&2
    exit 1
fi
for target in "${targets[@]}"; do
    rm -- checks-ran
    expect_failure "failed ${target}" "${fixture_make[@]}" repository-check "FAIL_TARGET=${target}"
    if [[ "$(tail -n 1 checks-ran)" != "${target}" ]]; then
        echo "error: repository-check continued after ${target} failed" >&2
        exit 1
    fi
done
rm -- checks-ran
expect_failure 'classification failed' env CLASSIFICATION=error "${fixture_make[@]}" repository-check
expect_failure 'classification failed' env CLASSIFICATION=error "${fixture_make[@]}" release-impact
expect_failure 'crate-impacting changes' env CLASSIFICATION=crate "${fixture_make[@]}" repository-check
if [[ -e checks-ran ]]; then
    echo 'error: repository checks ran after impact classification failed or rejected crate work' >&2
    exit 1
fi

cat > crates/ic-timers/src/platform.rs <<'EOF'
use ic_cdk_timers::TimerId;
pub(crate) struct TimerHandle(TimerId);
EOF
facade_header=('#![forbid(private_interfaces)]' 'mod platform;')
printf '%s\n' "${facade_header[@]}" > crates/ic-timers/src/lib.rs
bash scripts/ci/check-provider-boundary.sh >/dev/null
printf '%s\n' 'mod platform;' > crates/ic-timers/src/lib.rs
expect_failure 'public interfaces' bash scripts/ci/check-provider-boundary.sh
for export in 'pub use ic_cdk_timers::TimerId;' 'pub extern crate ic_cdk_timers;'; do
    printf '%s\n' "${facade_header[@]}" "${export}" > crates/ic-timers/src/lib.rs
    expect_failure 'direct ic-cdk-timers use' bash scripts/ci/check-provider-boundary.sh
done
printf '%s\n' "${facade_header[@]}" 'pub(crate) use crate::platform::TimerHandle;' > crates/ic-timers/src/lib.rs
bash scripts/ci/check-provider-boundary.sh >/dev/null
printf '%s\n' 'pub(super) use super::platform::TimerHandle;' > crates/ic-timers/src/runtime.rs
bash scripts/ci/check-provider-boundary.sh >/dev/null
printf '%s\n' 'use super::platform::TimerHandle;' > crates/ic-timers/src/runtime.rs
bash scripts/ci/check-provider-boundary.sh >/dev/null
printf '%s\n' 'use ic_cdk_timers::TimerId;' > crates/ic-timers/src/runtime.rs
expect_failure 'direct ic-cdk-timers use' bash scripts/ci/check-provider-boundary.sh
rm -- crates/ic-timers/src/runtime.rs
printf '%s\n' '#![forbid(private_interfaces)]' 'pub mod platform;' > crates/ic-timers/src/lib.rs
expect_failure 'private module' bash scripts/ci/check-provider-boundary.sh

printf '%s\n' "${facade_header[@]}" > crates/ic-timers/src/lib.rs
cp crates/ic-timers/src/platform.rs private-platform.rs
for declaration in 'pub fn leaked() {}' 'pub async fn leaked() {}' \
    'pub mod leaked {}' 'pub extern crate ic_cdk_timers as provider;'; do
    cp private-platform.rs crates/ic-timers/src/platform.rs
    printf '%s\n' "${declaration}" >> crates/ic-timers/src/platform.rs
    expect_failure 'restricted visibility' bash scripts/ci/check-provider-boundary.sh
done

# Rust resolves direct, grouped and chained aliases. No warning-denial flag is
# needed: private-interface leaks are forbidden by the facade itself.
printf '%s\n' 'pub(crate) struct TimerHandle;' 'pub(crate) fn set_timer() {}' > crates/ic-timers/src/platform.rs
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

echo 'Repository check regression tests passed'
