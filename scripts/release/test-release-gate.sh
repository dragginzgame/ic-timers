#!/usr/bin/env bash
set -euo pipefail

# This independent fixture owns its Make selections and logger checkout.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset VALIDATION_REPOSITORY_ROOT VALIDATION_RUNNER_SNAPSHOT_PATH
# Synthetic failures belong to this fixture, never its caller's evidence roots.
unset VALIDATION_LOG_DIR VALIDATION_FAILURE_LOG_DIR

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
makefile="${repository_root}/Makefile"
bump_script="${repository_root}/scripts/release/bump-version.sh"
export GITHUB_ACTIONS=false GITHUB_STEP_SUMMARY=''

# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$repository_root/scripts/ci/fixture-lifecycle.sh"
# shellcheck source=scripts/ci/make-adapter-probes.sh
source "$repository_root/scripts/ci/make-adapter-probes.sh"
temporary_root="$(fixture_create "${TMPDIR:-/tmp}/timer-release-gate.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
trap 'fixture_finish "$?" "$temporary_root" "Failed release-gate fixture retained"' EXIT

if downgrade_output="$(bash "${bump_script}" 0.0.0 2>&1)"; then
    echo "error: version bump accepted a downgrade" >&2
    exit 1
fi
if [[ "${downgrade_output}" != *"must be greater than"* ]]; then
    echo "error: version bump did not reject a downgrade before release work" >&2
    exit 1
fi

if invalid_output="$(bash "${bump_script}" 00.4.0 2>&1)"; then
    echo "error: version bump accepted a leading-zero SemVer component" >&2
    exit 1
fi
if [[ "${invalid_output}" != Usage:* ]]; then
    echo "error: version bump did not reject invalid SemVer before release work" >&2
    exit 1
fi

# Testkit owns host-specific artifact admission; test-testkit-server.sh checks our adapter.
# Execute the real orchestration with cheap leaf targets. Expected checks remain
# independent of Makefile variables; their spelling and recipe layout do not.
# Make's CURDIR is physical, including under macOS's /var -> /private/var.
# Enter through an alias on every host so this path distinction stays covered;
# a space in the physical root also checks argument boundaries.
mkdir "${temporary_root}/workspace with spaces"
ln -s 'workspace with spaces' "${temporary_root}/workspace-alias"
cp "${makefile}" "${temporary_root}/workspace with spaces/Makefile"
bash "$repository_root/scripts/ci/prepare-script-fixture.sh" "$repository_root" "$temporary_root/workspace with spaces" make/consumer.mk
mkdir -p "$temporary_root/workspace with spaces/ci"
cp "$repository_root/ci/release-delivery.txt" "$temporary_root/workspace with spaces/ci/"
cd "${temporary_root}/workspace-alias"
fixture_root="$(pwd -P)"
git init -q
export VALIDATION_REPOSITORY_ROOT="$fixture_root" VALIDATION_RUNNER_SNAPSHOT_PATH='' VALIDATION_RUNNER_DEPTH=0

# Exercise real fetch recipes with a recording Cargo stub, never the network.
mkdir -p bin
cat > bin/cargo <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "${FIXTURE_CARGO_LOG:-fetch-events}"
if [[ "$1" == publish ]]; then
    printf 'publish\n' >> checks-ran
    if [[ "${FAIL_TARGET:-}" == publish ]]; then exit 101; fi
fi
if [[ -n "${FIXTURE_TOOLCHAIN_LOG:-}" ]]; then
    printf '%s\n' "${RUSTUP_TOOLCHAIN:-unset}" >> "$FIXTURE_TOOLCHAIN_LOG"
fi
if [[ "${FIXTURE_FAIL_CARGO_CALL:-}" == "$*" ]]; then
    echo 'injected Cargo failure' >&2
    exit 101
fi
if [[ "${FIXTURE_FAIL_FETCH:-}" == "${3:-}" ]]; then
    echo "injected fetch failure: ${3}" >&2
    exit 101
fi
EOF
chmod +x bin/cargo
fetch_calls=('fetch --manifest-path Cargo.toml --locked')
for failed_manifest in '' Cargo.toml; do
    if PATH="${fixture_root}/bin:${PATH}" FIXTURE_FAIL_FETCH="${failed_manifest}" \
        make --no-print-directory fetch >fetch-output 2>&1; then
        if [[ -n "${failed_manifest}" ]]; then
            echo "error: dependency preparation accepted failed ${failed_manifest} fetch" >&2
            exit 1
        fi
    elif [[ -z "${failed_manifest}" ]]; then
        cat fetch-output >&2
        echo 'error: dependency preparation rejected successful fetches' >&2
        exit 1
    fi
    expected_fetch_calls=("${fetch_calls[@]}")
    if [[ "${failed_manifest}" == Cargo.toml ]]; then expected_fetch_calls=("${fetch_calls[0]}"); fi
    printf '%s\n' "${expected_fetch_calls[@]}" > expected-fetch-events
    if ! cmp -s expected-fetch-events fetch-events; then
        cat fetch-events >&2
        echo 'error: dependency preparation changed locked inputs or continued after failure' >&2
        exit 1
    fi
    rm fetch-events
done

# Exercise library build, API/protocol tests and MSRV through the Cargo stub.
# API compile-fail doctests run after native and minimum-Wasm compilation;
# failures at any command stop the owning target.
for target in build test msrv; do
    case "${target}" in
        build) calls=('build -p ic-timers --all-targets --all-features --locked') ;;
        test) calls=('test -p ic-timers --all-targets --all-features --locked'
            'test -p ic-timers --doc --all-features --locked'
            'test -p ic-timers-test-protocol --lib --locked'
            'test -p ic-timers-runtime-probe --lib --locked') ;;
        msrv) calls=('+1.88.0 check -p ic-timers --all-targets --all-features --locked'
            '+1.88.0 check -p ic-timers --all-features --locked --target wasm32-unknown-unknown'
            '+1.88.0 test -p ic-timers --doc --all-features --locked') ;;
    esac
    for failed_call in '' "${calls[@]}"; do
        : > api-test-events
        if PATH="${fixture_root}/bin:${PATH}" FIXTURE_CARGO_LOG=api-test-events \
            FIXTURE_FAIL_CARGO_CALL="${failed_call}" make --no-print-directory \
            "${target}" MSRV=1.88.0 >api-test-output 2>&1; then
            if [[ -n "${failed_call}" ]]; then
                echo "error: ${target} ignored Cargo failure" >&2
                exit 1
            fi
        elif [[ -z "${failed_call}" ]]; then
            cat api-test-output >&2
            echo "error: ${target} rejected recorded Cargo success" >&2
            exit 1
        fi
        : > expected-api-test-events
        for call in "${calls[@]}"; do
            printf '%s\n' "${call}" >> expected-api-test-events
            if [[ "${call}" == "${failed_call}" ]]; then break; fi
        done
        cmp expected-api-test-events api-test-events
    done
done

# Exercise explicit setup and offline admission through the real Make adapter.
mkdir -p scripts/ci scripts/dev
cp "$repository_root/scripts/ci/run-validation-targets.sh" \
    "$repository_root/scripts/ci/verify-file-checksum.sh" scripts/ci/
cat > scripts/dev/testkit-server.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$@" >> server-events
exit "${FIXTURE_SERVER_STATUS:-0}"
EOF
make --no-print-directory install-testkit-server >/dev/null
make --no-print-directory pocketic-check >/dev/null
printf 'setup\ncheck\n' > expected-server-events
cmp expected-server-events server-events
if FIXTURE_SERVER_STATUS=31 make --no-print-directory pocketic-check >/dev/null 2>&1; then
    echo 'error: failed Testkit offline admission was accepted' >&2
    exit 1
fi
rm server-events

# Standalone qualification must refuse before any probe compilation, including
# under parallel Make. The real dependency edges dispatch only offline admission.
for target in pocketic-watchdog pocketic-cohorts; do
    : > early-cargo-events
    status=0
    PATH="${fixture_root}/bin:${PATH}" FIXTURE_CARGO_LOG=early-cargo-events \
        FIXTURE_SERVER_STATUS=31 make --no-print-directory -j2 "$target" \
        > early-admission-output 2>&1 || status=$?
    test "$status" -eq 2
    test ! -s early-cargo-events
    printf 'check\n' > expected-server-events
    cmp expected-server-events server-events
    rm server-events
done

# Make admits the server before delegating to the consumer build driver and
# propagates its selected toolchain and failures. Rust/PocketIC evidence owns
# isolated feature builds and exact artifact loading.
for mode in watchdog cohorts; do
    call="run -p ic-timers-pocketic --locked -- $mode"
    for failed_call in '' "$call"; do
        : > driver-events
        : > driver-toolchains
        status=0
        PATH="$fixture_root/bin:$PATH" FIXTURE_CARGO_LOG=driver-events \
            FIXTURE_TOOLCHAIN_LOG=driver-toolchains FIXTURE_FAIL_CARGO_CALL="$failed_call" \
            make --no-print-directory "pocketic-$mode" MSRV=1.88.0 >driver-output 2>&1 || status=$?
        if [[ -z "$failed_call" ]]; then test "$status" -eq 0; else test "$status" -eq 2; fi
        printf '%s\n' "$call" > expected-driver-events
        printf '%s\n' '1.88.0' > expected-driver-toolchains
        cmp expected-driver-events driver-events
        cmp expected-driver-toolchains driver-toolchains
        rm server-events
    done
done

# Exercise the actual shared entrypoints with an inert runner. Delivery remains
# direct, while inherited or command-line requests for PR refuse before effects.
mkdir -p external-snapshot/scripts/ci
external_root="$fixture_root/external-snapshot"
cat > external-snapshot/scripts/ci/run-release.sh <<'EOF'
#!/usr/bin/env bash
printf '%s\n' 'external snapshot runner executed' > external-runner-events
exit 99
EOF
cat > external-snapshot/scripts/ci/check-make-execution.sh <<'EOF'
#!/usr/bin/env bash
printf '%s\n' 'external snapshot admission executed' > external-admission-events
exit 0
EOF
cat > scripts/ci/run-release.sh <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\0' "${RELEASE_DELIVERY:?}" "$@" > standard-entrypoint-events
exit "${FIXTURE_ENTRYPOINT_STATUS:-0}"
EOF
make --no-print-directory > default-goal-output
test ! -e standard-entrypoint-events
for target in release-patch release-minor release-major release-resume; do
    arguments=("${target#release-}" fixture-origin fixture-branch)
    if [[ "$target" == release-resume ]]; then
        arguments=(resume 0.1.1 fixture-origin fixture-branch)
    fi
    printf '%s\0' direct "${arguments[@]}" > expected-entrypoint-events
    for policy in environment command-line; do
        variables=(RELEASE_REMOTE=fixture-origin RELEASE_BRANCH=fixture-branch VERSION=0.1.1)
        if [[ "$policy" == command-line ]]; then
            variables+=(RELEASE_DELIVERY=direct "SHARED_TOOLING_ROOT=$external_root")
        fi
        env RELEASE_DELIVERY=direct SHARED_TOOLING_ROOT="$external_root" \
            make --no-print-directory "$target" "${variables[@]}" \
            > entrypoint-output 2>&1
        cmp expected-entrypoint-events standard-entrypoint-events
        test ! -e external-runner-events
        test ! -e external-admission-events
        if env RELEASE_DELIVERY=direct SHARED_TOOLING_ROOT="$external_root" FIXTURE_ENTRYPOINT_STATUS=37 \
            make --no-print-directory "$target" "${variables[@]}" > entrypoint-output 2>&1; then
            echo "error: $target ignored its selected runner failure" >&2
            exit 1
        fi
        cmp expected-entrypoint-events standard-entrypoint-events
        test ! -e external-runner-events
        test ! -e external-admission-events
        rm standard-entrypoint-events
        refused_variables=(RELEASE_REMOTE=fixture-origin RELEASE_BRANCH=fixture-branch VERSION=0.1.1)
        if [[ "$policy" == command-line ]]; then
            refused_variables+=(RELEASE_DELIVERY=pr "SHARED_TOOLING_ROOT=$external_root")
        fi
        if env RELEASE_DELIVERY=pr SHARED_TOOLING_ROOT="$external_root" \
            make --no-print-directory "$target" "${refused_variables[@]}" > delivery-refusal-output 2>&1; then
            echo "error: $target admitted unsupported PR delivery" >&2
            exit 1
        fi
        grep -Fq 'consumer does not support PR delivery' delivery-refusal-output
        test ! -e standard-entrypoint-events
        test ! -e external-runner-events
        test ! -e external-admission-events
    done
done
if make --no-print-directory release-patch release-minor > conflicting-goals-output 2>&1; then
    echo 'error: standard release entrypoints accepted conflicting goals' >&2
    exit 1
fi
test ! -e standard-entrypoint-events

# Shared owns flag aliases and replacements; our real release targets must
# refuse each unsafe behavior before either selected or external runner effects.
mkdir "$temporary_root/release-admission"
SHARED_TOOLING_ROOT="$external_root" VERSION=0.1.1 \
    make_probe_target_admission "$(command -v make)" "$fixture_root/Makefile" \
        "$temporary_root/release-admission" "$fixture_root/standard-entrypoint-events" \
        release-patch release-minor release-major release-resume || exit 1
test ! -e external-runner-events
test ! -e external-admission-events

cat > overrides.mk <<'EOF'
# Keep this orchestration fixture independent of prepared formatter tools.
format-tools-check:
	@:
ensure-clean release-tag-check fetch install-release-editor install-testkit-server host-tools-check actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package pocketic-check msrv testing-check pocketic-watchdog pocketic-cohorts:
	@printf '%s\n' '$@' >> checks-ran
	@if [ '$@' = '$(FAIL_TARGET)' ]; then echo 'failed $@' >&2; exit 1; fi
EOF
printf '\ninclude overrides.mk\n' >> Makefile
fixture_make=(make --no-print-directory -f Makefile
    'MAKE=make --no-print-directory -f Makefile')
# actions-check retains its real host-tools-check prerequisite when its recipe
# is overridden. Record that admission before the action/dependency leaf.
ci_targets=(host-tools-check actions-check shell-check provider-check fmt-check clippy release-check docs-check test wasm-check package)
release_targets=(fetch install-release-editor install-testkit-server pocketic-check "${ci_targets[@]}" msrv testing-check
    pocketic-check pocketic-watchdog pocketic-check pocketic-cohorts)
# Every leaf, including lint and publication, is substituted. Parallel Make must
# still stop at each failed admission/lint before packaging or external effects.
export PATH="$fixture_root/bin:$PATH"
for gate in ci release-verify publish pocketic-watchdog pocketic-cohorts; do
    case "${gate}" in
        ci) expected=("${ci_targets[@]}") ;;
        release-verify) expected=("${release_targets[@]}") ;;
        publish) expected=(ensure-clean release-tag-check clippy package publish) ;;
        *) expected=(pocketic-check "${gate}") ;;
    esac
    if ! "${fixture_make[@]}" -j4 "${gate}" > "$gate-output" 2>&1; then
        cat "$gate-output" >&2
        echo "error: ${gate} rejected recorded check success" >&2
        exit 1
    fi
    printf '%s\n' "${expected[@]}" > expected-checks
    if ! cmp -s expected-checks checks-ran; then
        cat checks-ran >&2
        echo "error: ${gate} ran unexpected checks" >&2
        exit 1
    fi
    rm checks-ran
    for target in "${expected[@]}"; do
        if "${fixture_make[@]}" -j4 "${gate}" "FAIL_TARGET=${target}" > "$gate-$target-output" 2>&1; then
            echo "error: ${gate} ignored failed ${target}" >&2
            exit 1
        fi
        : > expected-checks
        for check in "${expected[@]}"; do
            printf '%s\n' "${check}" >> expected-checks
            if [[ "${check}" == "${target}" ]]; then break; fi
        done
        if ! cmp -s expected-checks checks-ran; then
            cat checks-ran >&2
            echo "error: ${gate} skipped checks or continued after failed ${target}" >&2
            exit 1
        fi
        rm checks-ran
    done
done

# Failures in the actual adapter keep unique raw logs across subsequent attempts.
failure_root="$fixture_root/.git/release-state/validation-failures"
failure_logs=("$failure_root"/*-0-fetch.log)
[[ -f "${failure_logs[0]}" ]] || exit 1
cp "${failure_logs[0]}" retained-fetch-log
"${fixture_make[@]}" release-verify >/dev/null 2>&1
cmp retained-fetch-log "${failure_logs[0]}"
if "${fixture_make[@]}" release-verify FAIL_TARGET=fetch > retained-failure-output 2>&1; then exit 1; fi
new_failure_logs=("$failure_root"/*-0-fetch.log)
[[ "${#new_failure_logs[@]}" -gt "${#failure_logs[@]}" ]] || exit 1
grep -Fq 'failed fetch' "$failure_root/latest.log"
cmp retained-fetch-log "${failure_logs[0]}"

# Shared qualifies diagnostic families, search backends and Make-mode rejection.
# Keep one real Timers adapter failure: routing, status and selected evidence.
cat >> overrides.mk <<'EOF'
logging-fail:
	@echo 'error: local-adapter-marker'
	@exit 7
logging-parent:
	+bash child/scripts/ci/run-validation-targets.sh child-gate
EOF
status=0
GITHUB_STEP_SUMMARY="$fixture_root/adapter-summary.md" \
    "${fixture_make[@]}" release-verify RELEASE_TARGETS=logging-fail \
    > adapter-failure-output 2>&1 || status=$?
# Make maps the leaf's exit 7 to 2; the adapter must propagate that failure.
[[ "$status" == 2 ]] || exit 1
for output in adapter-failure-output "$failure_root/latest-errors.log" adapter-summary.md; do
    grep -Fxq '[ERR:logging-fail] error: local-adapter-marker' "$output"
done
for output in "$failure_root/latest.log" "$failure_root/latest-combined.log"; do
    grep -Fxq 'error: local-adapter-marker' "$output"
    if grep -Fq '[ERR:' "$output"; then exit 1; fi
done
grep -Fq 'Error 7' "$failure_root/latest.log"
grep -Fq 'Full failure log retained at:' adapter-failure-output

# A child logger must select its own checkout while retaining release identity.
mkdir -p child/scripts/ci
cp "$repository_root/scripts/ci/run-validation-targets.sh" \
    "$repository_root/scripts/ci/check-make-execution.sh" \
    "$repository_root/scripts/ci/verify-file-checksum.sh" child/scripts/ci/
cat > child/Makefile <<'EOF'
child-gate:
	@test "$(RELEASE_VERSION)" = 9.8.7
	@test "$(RELEASE_COMMIT)" = fixture-selected-commit
	@test "$$VALIDATION_RUNNER_DEPTH" = 2
	@echo child-checkout-marker
EOF
"${fixture_make[@]}" -j2 release-verify RELEASE_TARGETS=logging-parent \
    RELEASE_VERSION=9.8.7 RELEASE_COMMIT=fixture-selected-commit \
    > nested-logging-output 2>&1
grep -Fq child-checkout-marker nested-logging-output
if grep -Fq 'jobserver unavailable' nested-logging-output; then exit 1; fi

fixture_complete=true
echo "Release gate execution checks passed"
