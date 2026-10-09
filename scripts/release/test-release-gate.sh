#!/usr/bin/env bash
set -euo pipefail

# This independent fixture owns its Make selections and logger checkout.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset VALIDATION_REPOSITORY_ROOT VALIDATION_RUNNER_SNAPSHOT_PATH

repository_root="$(git rev-parse --show-toplevel)"
export PATH="${repository_root}/.tools/host/bin:${PATH}"
export YQ="${repository_root}/.tools/host/bin/yq"
makefile="${repository_root}/Makefile"
bump_script="${repository_root}/scripts/release/bump-version.sh"
impact_checker="${repository_root}/scripts/release/check-bump-impact.sh"
export GITHUB_ACTIONS=false GITHUB_STEP_SUMMARY=''

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

if ! bash "${impact_checker}" crate 0.6.0 >/dev/null 2>&1; then
    echo "error: crate-impacting release subject was rejected" >&2
    exit 1
fi
if ! repository_output="$(bash "${impact_checker}" repository 0.6.0 2>&1)"; then
    echo "error: repository-only release subject was rejected" >&2
    exit 1
fi
repository_advisory="continuing because the maintainer invoked an explicit version bump"
if [[ "${repository_output}" != *"${repository_advisory}"* ]]; then
    echo "error: repository-only release subject did not emit its advisory" >&2
    exit 1
fi
if none_output="$(bash "${impact_checker}" none 0.6.0 2>&1)"; then
    echo "error: empty release subject was accepted" >&2
    exit 1
fi
if [[ "${none_output}" != *"no changes exist since v0.6.0"* ]]; then
    echo "error: empty release subject did not explain its rejection" >&2
    exit 1
fi
if bash "${impact_checker}" unexpected 0.6.0 >/dev/null 2>&1; then
    echo "error: unknown release-impact classification was accepted" >&2
    exit 1
fi

# Testkit owns host-specific artifact admission; test-testkit-server.sh checks our adapter.
# Execute the real orchestration with cheap leaf targets. Expected checks remain
# independent of Makefile variables; their spelling and recipe layout do not.
temporary_root="$(mktemp -d)"
trap 'if [[ $? == 0 ]]; then rm -rf -- "${temporary_root}"; else printf "Failed release-gate fixture retained: %s\n" "${temporary_root}" >&2; fi' EXIT
# Make's CURDIR is physical, including under macOS's /var -> /private/var.
# Enter through an alias on every host so this path distinction stays covered;
# a space in the physical root also checks argument boundaries.
mkdir "${temporary_root}/workspace with spaces"
ln -s 'workspace with spaces' "${temporary_root}/workspace-alias"
cp "${makefile}" "${temporary_root}/workspace with spaces/Makefile"
mkdir -p "${temporary_root}/workspace with spaces/make"
cp "${repository_root}/make/tools.mk" "${temporary_root}/workspace with spaces/make/"
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

# Exercise library build, test and MSRV recipes through the recording Cargo stub.
# API compile-fail doctests must run in both toolchains, after their first check;
# failures at either command stop the owning target.
for target in build test msrv; do
    case "${target}" in
        build) calls=('build -p ic-timers --all-targets --all-features --locked') ;;
        test) calls=('test -p ic-timers --all-targets --all-features --locked'
            'test -p ic-timers --doc --all-features --locked') ;;
        msrv) calls=('+1.88.0 check -p ic-timers --all-targets --all-features --locked'
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
    "$repository_root/scripts/ci/check-make-execution.sh" scripts/ci/
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
cat > overrides.mk <<'EOF'
# Keep this orchestration fixture independent of prepared formatter tools.
format-tools-check:
	@:
fetch install-testkit-server host-tools-check actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package pocketic-check msrv testing-check pocketic-watchdog pocketic-cohorts:
	@printf '%s\n' '$@' >> checks-ran
	@if [ '$@' = '$(FAIL_TARGET)' ]; then echo 'failed $@' >&2; exit 1; fi
EOF
printf '\ninclude overrides.mk\n' >> Makefile
fixture_make=(make --no-print-directory -f Makefile
    'MAKE=make --no-print-directory -f Makefile')
# actions-check retains its real host-tools-check prerequisite when its recipe
# is overridden. Record that admission before the action/dependency leaf.
ci_targets=(host-tools-check actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package)
release_targets=(fetch install-testkit-server pocketic-check "${ci_targets[@]}" msrv testing-check
    pocketic-check pocketic-watchdog pocketic-check pocketic-cohorts)
for gate in ci release-verify pocketic-watchdog pocketic-cohorts; do
    case "${gate}" in
        ci) expected=("${ci_targets[@]}") ;;
        release-verify) expected=("${release_targets[@]}") ;;
        *) expected=(pocketic-check "${gate}") ;;
    esac
    if ! "${fixture_make[@]}" "${gate}" > "$gate-output" 2>&1; then
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
        if "${fixture_make[@]}" "${gate}" "FAIL_TARGET=${target}" > "$gate-$target-output" 2>&1; then
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
[[ -f "${failure_logs[0]}" ]]
cp "${failure_logs[0]}" retained-fetch-log
"${fixture_make[@]}" release-verify >/dev/null 2>&1
cmp retained-fetch-log "${failure_logs[0]}"
if "${fixture_make[@]}" release-verify FAIL_TARGET=fetch > retained-failure-output 2>&1; then exit 1; fi
new_failure_logs=("$failure_root"/*-0-fetch.log)
[[ "${#new_failure_logs[@]}" -gt "${#failure_logs[@]}" ]]
grep -Fq 'failed fetch' "$failure_root/latest.log"
cmp retained-fetch-log "${failure_logs[0]}"

# The canonical logger's optional ripgrep branch must also work on stock hosts.
mkdir -p stock-bin
# Make can execute simple recipes directly, so printf/echo need external binaries.
# type -P resolves executables even when Bash supplies a builtin of the same name.
for tool in bash sh make git cp mktemp rm mkdir date tee sed grep awk tail dirname echo printf; do
    ln -s "$(type -P "$tool")" "stock-bin/$tool"
done
if PATH="$fixture_root/stock-bin" "${fixture_make[@]}" release-verify \
    FAIL_TARGET=fetch > stock-host-output 2>&1; then exit 1; fi
grep -Fq 'failed fetch' "$failure_root/latest.log"
grep -Fq 'Full failure log retained at:' stock-host-output

# Qualify the actual consumer logger with both its optional search backends.
cat >> overrides.mk <<'EOF'
logging-pass:
	@echo 'test error::tests::passing ... ok'
	@echo 'test error::tests::ignored ... ignored'
logging-fail:
	@echo 'test error::tests::context ... ok'
	@echo 'error[E0308]: typed-marker'
	@echo 'error:no-space-marker'
	@echo 'error:'
	@echo 'test error::tests::actual ... FAILED'
	@exit 7
logging-parent:
	+bash child/scripts/ci/run-validation-targets.sh child-gate
logging-mode-probe:
	@touch unexpected-logger-dispatch
EOF
# Reject non-executing/failure-masking modes before dispatch.
# GNU Make 3.81 on macOS ignores GNUMAKEFLAGS; 4.0 introduced it.
mode_variables=(MAKEFLAGS)
make_version="$(make --version)"
case "$make_version" in 'GNU Make 3.'*) ;; *) mode_variables+=(GNUMAKEFLAGS) ;; esac
for variable in "${mode_variables[@]}"; do
    for flags in i n q t v --ignore-errors --dry-run --question --touch --version; do
        mode_output="${variable}-${flags}.log"
        if env "$variable=$flags" bash scripts/ci/run-validation-targets.sh logging-mode-probe \
            > "${mode_output}" 2>&1; then
            cat "${mode_output}" >&2
            echo "error: logger accepted $variable=$flags" >&2
            exit 1
        fi
        grep -Fq 'requires recipe execution and failure propagation' "${mode_output}"
        test ! -e unexpected-logger-dispatch
    done
done
for backend in prepared stock; do
    logger_path="$PATH"
    if [[ "$backend" == stock ]]; then logger_path="$fixture_root/stock-bin"; fi
    PATH="$logger_path" bash scripts/ci/run-validation-targets.sh logging-pass \
        > "$backend-passing-output" 2>&1
    grep -Fxq 'test error::tests::passing ... ok' "$backend-passing-output"
    grep -Fxq 'test error::tests::ignored ... ignored' "$backend-passing-output"
    if grep -Fq '[ERR:' "$backend-passing-output"; then exit 1; fi
    if PATH="$logger_path" "${fixture_make[@]}" release-verify \
        RELEASE_TARGETS=logging-fail > "$backend-failing-output" 2>&1; then exit 1; fi
    for log in "$backend-failing-output" "$failure_root/latest-errors.log"; do
        grep -Fq '[logging-fail] test error::tests::context ... ok' "$log"
        for diagnostic in 'error[E0308]: typed-marker' 'error:no-space-marker' \
            'error:' 'test error::tests::actual ... FAILED'; do
            grep -Fxq "[ERR:logging-fail] $diagnostic" "$log"
        done
        if grep -Fq '[ERR:logging-fail] test error::tests::context ... ok' "$log"; then exit 1; fi
    done
    grep -Fxq 'test error::tests::context ... ok' "$failure_root/latest.log"
    grep -Fxq 'error:no-space-marker' "$failure_root/latest.log"
    if grep -Fq '[ERR:' "$failure_root/latest.log"; then exit 1; fi
done

# A child logger must select its own checkout while retaining release identity.
mkdir -p child/scripts/ci
cp "$repository_root/scripts/ci/run-validation-targets.sh" \
    "$repository_root/scripts/ci/check-make-execution.sh" child/scripts/ci/
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

echo "Release gate execution checks passed"
