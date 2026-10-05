#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
makefile="${repository_root}/Makefile"
bump_script="${repository_root}/scripts/release/bump-version.sh"
impact_checker="${repository_root}/scripts/release/check-bump-impact.sh"

if grep -RE --include='*.sh' \
    '(^|[;&|[:space:]])rg([[:space:]]|$)' \
    "${repository_root}/scripts/ci" "${repository_root}/scripts/release" >/dev/null; then
    echo "error: runner-executed scripts must not require ripgrep" >&2
    exit 1
fi

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

# Host-specific artifact admission is exercised by test-pocketic-verification.sh.
# Execute the real orchestration with cheap leaf targets. Expected checks remain
# independent of Makefile variables; their spelling and recipe layout do not.
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT
cp "${makefile}" "${temporary_root}/Makefile"
cd "${temporary_root}"

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
fetch_calls=('fetch --manifest-path Cargo.toml --locked'
    'fetch --manifest-path testing/Cargo.toml --locked')
for failed_manifest in '' Cargo.toml testing/Cargo.toml; do
    if PATH="${temporary_root}/bin:${PATH}" FIXTURE_FAIL_FETCH="${failed_manifest}" \
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

# Execute the test and MSRV recipes only through the recording Cargo stub.
# API compile-fail doctests must run in both toolchains, after their first check;
# failures at either command stop the owning target.
for target in test msrv; do
    case "${target}" in
        test) calls=('test --workspace --all-targets --all-features --locked'
            'test --workspace --doc --all-features --locked') ;;
        msrv) calls=('+1.88.0 check --workspace --all-targets --all-features --locked'
            '+1.88.0 test --workspace --doc --all-features --locked') ;;
    esac
    for failed_call in '' "${calls[@]}"; do
        : > api-test-events
        if PATH="${temporary_root}/bin:${PATH}" FIXTURE_CARGO_LOG=api-test-events \
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

# Exercise the actual recipe and Make variable origins without provisioning.
mkdir -p scripts/ci
cat > scripts/ci/check-pocketic.sh <<'EOF'
#!/usr/bin/env bash
printf '%s\n%s\n' "${POCKET_IC_BIN}" "${POCKET_IC_AUTO_INSTALL}" > provisioning
if [[ -z "${POCKET_IC_BIN}" ]]; then exit 2; fi
EOF
default_binary="${temporary_root}/target/tools/pocket-ic/16.0.0/pocket-ic"
for source in default environment command-line same-as-default empty; do
    case "${source}" in
        default)
            env -u POCKET_IC_BIN make --no-print-directory pocketic-check >/dev/null
            expected_path="${default_binary}"; expected_install=1 ;;
        environment)
            POCKET_IC_BIN=/explicit/environment make --no-print-directory pocketic-check >/dev/null
            expected_path=/explicit/environment; expected_install=0 ;;
        command-line)
            make --no-print-directory pocketic-check POCKET_IC_BIN=/explicit/command-line >/dev/null
            expected_path=/explicit/command-line; expected_install=0 ;;
        same-as-default)
            make --no-print-directory pocketic-check "POCKET_IC_BIN=${default_binary}" >/dev/null
            expected_path="${default_binary}"; expected_install=0 ;;
        empty)
            if make --no-print-directory pocketic-check POCKET_IC_BIN= >/dev/null 2>&1; then
                echo 'error: empty PocketIC override was accepted' >&2
                exit 1
            fi
            expected_path=''; expected_install=0 ;;
    esac
    printf '%s\n%s\n' "${expected_path}" "${expected_install}" > expected-provisioning
    if ! cmp -s expected-provisioning provisioning; then
        cat provisioning >&2
        echo "error: ${source} PocketIC selection was incorrect" >&2
        exit 1
    fi
done
rm provisioning
cat > overrides.mk <<'EOF'
fetch actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package pocketic-check msrv testing-check pocketic-watchdog pocketic-cohorts:
	@printf '%s\n' '$@' >> checks-ran
	@if [ '$@' = '$(FAIL_TARGET)' ]; then echo 'failed $@' >&2; exit 1; fi
EOF
fixture_make=(make --no-print-directory -f Makefile -f overrides.mk
    'MAKE=make --no-print-directory -f Makefile -f overrides.mk')
ci_targets=(actions-check shell-check release-check provider-check fmt-check check clippy docs-check test wasm-check package)
release_targets=(fetch pocketic-check "${ci_targets[@]}" msrv testing-check
    pocketic-check pocketic-watchdog pocketic-check pocketic-cohorts)
for gate in ci release-verify pocketic-watchdog pocketic-cohorts; do
    case "${gate}" in
        ci) expected=("${ci_targets[@]}") ;;
        release-verify) expected=("${release_targets[@]}") ;;
        *) expected=(pocketic-check "${gate}") ;;
    esac
    "${fixture_make[@]}" "${gate}" >/dev/null 2>&1
    printf '%s\n' "${expected[@]}" > expected-checks
    if ! cmp -s expected-checks checks-ran; then
        cat checks-ran >&2
        echo "error: ${gate} ran unexpected checks" >&2
        exit 1
    fi
    rm checks-ran
    for target in "${expected[@]}"; do
        if "${fixture_make[@]}" "${gate}" "FAIL_TARGET=${target}" >/dev/null 2>&1; then
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

echo "Release gate execution checks passed"
