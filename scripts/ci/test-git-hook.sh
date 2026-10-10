#!/usr/bin/env bash
set -euo pipefail

# The fixture owns its Make selections; negative cases set them explicitly.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
repository_root="$(git rev-parse --show-toplevel)"
source "${repository_root}/ci/tool-versions.env"
temporary_root="$(mktemp -d "${TMPDIR:-/tmp}/timer-hook-test.XXXXXX")"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "${temporary_root}"
    else
        printf "Failed hook fixture retained: %s\n" "${temporary_root}" >&2
    fi
    exit "$status"
}
trap finish EXIT
bash "${repository_root}/scripts/ci/check-format-tools.sh" \
    "${SHARED_TOOLING_CARGO_SORT_VERSION}"
# Reuse committed objects read-only; the fixture never creates a commit.
source_commit="$(git rev-parse HEAD)"
source_objects="$(git rev-parse --git-path objects)"
case "${source_objects}" in /*) ;; *) source_objects="${repository_root}/${source_objects}" ;; esac
mkdir "${temporary_root}/repo"
git init -q "${temporary_root}/repo"
cd "${temporary_root}/repo"
mkdir -p .git/objects/info
printf '%s\n' "${source_objects}" > .git/objects/info/alternates
git update-ref HEAD "${source_commit}"
git read-tree HEAD
git checkout-index --all
cp "${repository_root}/Makefile" Makefile
mkdir -p make
cp "${repository_root}/make/tools.mk" "${repository_root}/make/rust-format.mk" \
    "${repository_root}/make/release.mk" "${repository_root}/make/execution.mk" make/
mkdir -p ci
cp "${repository_root}/ci/tool-versions.env" ci/
# The current fmt prerequisite must also exist in the fixture's exact index.
cp -p "${repository_root}/scripts/ci/check-format-tools.sh" \
    scripts/ci/
cp -p "${repository_root}/scripts/ci/check-make-execution.sh" \
    "${repository_root}/scripts/ci/run-formatting.sh" scripts/ci/
# The fixture overlays both members into the one root-owned workspace.
mkdir -p crates/hook-fixture/src testing/crates/hook-probe/src
cat > Cargo.toml <<'EOF'
[workspace]
members = ["crates/hook-fixture", "testing/crates/hook-probe"]
resolver = "3"
[workspace.package]
version = "0.0.0"
edition = "2024"
EOF
for member in crates/hook-fixture testing/crates/hook-probe; do
    cat > "${member}/Cargo.toml" <<EOF
[package]
name = "$(basename "$member")"
version.workspace = true
edition.workspace = true
EOF
    printf 'pub fn fixture( ){}\n' > "${member}/src/lib.rs"
done
git add Makefile make/tools.mk make/rust-format.mk make/release.mk make/execution.mk ci/tool-versions.env Cargo.toml \
    crates/hook-fixture testing/crates/hook-probe \
    scripts/ci/check-format-tools.sh scripts/ci/check-make-execution.sh scripts/ci/run-formatting.sh
printf 'unrelated working edit\n' >> README.md
cp README.md "${temporary_root}/unrelated-readme"
# Preserve distinct index and working lock bytes across selected-file formatting.
git show :Cargo.lock > "${temporary_root}/index-lock"
printf '\n# unrelated working lock edit\n' >> Cargo.lock
cp Cargo.lock "${temporary_root}/working-lock"

working_files=(Makefile Cargo.toml Cargo.lock crates/hook-fixture/Cargo.toml testing/crates/hook-probe/Cargo.toml
    crates/hook-fixture/src/lib.rs testing/crates/hook-probe/src/lib.rs README.md)
capture_before_hook() {
    local path
    for path in "${working_files[@]}"; do
        mkdir -p "${temporary_root}/before-files/$(dirname "${path}")"
        cp "${path}" "${temporary_root}/before-files/${path}"
    done
    git diff --cached --binary > "${temporary_root}/before-index"
}
assert_unchanged() {
    local path
    for path in "${working_files[@]}"; do
        cmp "${path}" "${temporary_root}/before-files/${path}"
    done
    git diff --cached --binary > "${temporary_root}/after-index"
    cmp "${temporary_root}/before-index" "${temporary_root}/after-index"
}

# A rejected mode must not format working files or refresh the real index.
capture_before_hook
# GNU Make 3.81 on macOS ignores GNUMAKEFLAGS; 4.0 introduced it.
mode_variables=(MAKEFLAGS)
make_version="$(make --version)"
case "$make_version" in 'GNU Make 3.'*) ;; *) mode_variables+=(GNUMAKEFLAGS) ;; esac
for variable in "${mode_variables[@]}"; do
    for flags in i n q t v --ignore-errors --dry-run --question --touch --version; do
        mode_output="${temporary_root}/${variable}-${flags}.log"
        if env "$variable=$flags" bash "${repository_root}/.githooks/pre-commit" \
            > "${mode_output}" 2>&1; then
            cat "${mode_output}" >&2
            echo "error: commit hook accepted $variable=$flags" >&2
            exit 1
        fi
        grep -Fq 'requires recipe execution and failure propagation' "${mode_output}"
        assert_unchanged
    done
done

# The actual consumer Makefile must refuse before its formatter prerequisites,
# even if Make would ignore a recipe failure or skip execution.
for target in fmt fmt-check; do
    for mode in --ignore-errors --dry-run --touch --question; do
        for replacement in preserved cleared replaced both; do
            flags=(--no-print-directory)
            case "$replacement" in
                cleared) flags=(MAKEFLAGS=) ;;
                replaced) flags=(MAKEFLAGS=--no-print-directory) ;;
                both) flags=(MAKEFLAGS= MFLAGS=) ;;
            esac
            status=0
            SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
                make --no-print-directory "$mode" "$target" "${flags[@]}" \
                "SHARED_TOOLING_ROOT=${temporary_root}/unselected-snapshot" \
                > "${temporary_root}/make-${target}-${mode}-${replacement}.log" 2>&1 || status=$?
            if [[ "$status" != 2 ]]; then
                cat "${temporary_root}/make-${target}-${mode}-${replacement}.log" >&2
                echo "error: consumer formatting admitted $mode with $replacement flags" >&2
                exit 1
            fi
            assert_unchanged
        done
    done
done

# Failed initial, post-snapshot and post-format tree reads must stop before
# copying or staging, even when Git prints the expected tree before failing.
mkdir "$temporary_root/tree-failure-bin"
cat > "$temporary_root/tree-failure-bin/git" <<'GIT'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$HOOK_TREE_STATE/commands"
if [[ "$*" == write-tree ]]; then
    count=0
    [[ ! -f "$HOOK_TREE_STATE/count" ]] || read -r count < "$HOOK_TREE_STATE/count"
    count=$((count + 1))
    printf '%s\n' "$count" > "$HOOK_TREE_STATE/count"
    tree="$("$HOOK_TREE_GIT" "$@")" || exit $?
    if [[ "$count" == "$HOOK_TREE_OBSERVATION" ]]; then
        [[ "$HOOK_TREE_OUTPUT" != matching ]] || printf '%s\n' "$tree"
        echo 'injected write-tree observation failure' >&2
        exit 23
    fi
    printf '%s\n' "$tree"
    exit 0
fi
exec "$HOOK_TREE_GIT" "$@"
GIT
chmod +x "$temporary_root/tree-failure-bin/git"
real_git="$(type -P git)"
for observation in 1 2 3; do
    for output in empty matching; do
        state="$temporary_root/tree-failure-$observation-$output"
        mkdir "$state"
        capture_before_hook
        cp .git/index "$state/index-before"
        status=0
        CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 \
            PATH="$temporary_root/tree-failure-bin:$PATH" \
            HOOK_TREE_GIT="$real_git" HOOK_TREE_STATE="$state" \
            HOOK_TREE_OBSERVATION="$observation" HOOK_TREE_OUTPUT="$output" \
            "$BASH" "$repository_root/.githooks/pre-commit" \
            > "$state/output.log" 2>&1 || status=$?
        [[ "$status" == 23 && "$(cat "$state/count")" == "$observation" ]]
        grep -Fq 'injected write-tree observation failure' "$state/output.log"
        [[ "$(tail -n 1 "$state/commands")" == write-tree ]]
        cmp .git/index "$state/index-before"
        assert_unchanged
        if [[ "$observation" == 3 ]]; then
            grep -Fxq 'Formatting... ok' "$state/output.log"
        elif grep -Fq 'Formatting...' "$state/output.log"; then
            echo 'error: formatting ran after an earlier tree observation failed' >&2
            exit 1
        fi
    done
done

# The actual consumer fmt target formats and refreshes both root workspace members,
# preserving unrelated edits and requiring no dependencies, builds or network.
if ! CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 \
    SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" \
    bash "${repository_root}/.githooks/pre-commit" \
    > "${temporary_root}/format.log" 2>&1; then
    cat "${temporary_root}/format.log" >&2
    exit 1
fi
# The formatter owns one success line; the hook separately confirms index refresh.
printf '%s\n' 'Formatting... ok' \
    'Pre-commit formatting passed; refreshed only the selected files.' \
    > "${temporary_root}/expected-format-output"
cmp "${temporary_root}/expected-format-output" "${temporary_root}/format.log"
for path in crates/hook-fixture/src/lib.rs testing/crates/hook-probe/src/lib.rs; do
    formatted="$(git show ":${path}")"
    test "${formatted}" = 'pub fn fixture() {}'
done
cmp "${temporary_root}/unrelated-readme" README.md
cmp "${temporary_root}/working-lock" Cargo.lock
git show :Cargo.lock > "${temporary_root}/after-hook-index-lock"
cmp "${temporary_root}/index-lock" "${temporary_root}/after-hook-index-lock"
if ! SHARED_TOOLING_ROOT="${temporary_root}/unselected-snapshot" \
    HOST_TOOL_VERSIONS="${temporary_root}/unselected-pins.env" make --no-print-directory \
    fmt-check "SHARED_TOOLING_ROOT=${temporary_root}/unselected-snapshot" \
    "HOST_TOOL_VERSIONS=${temporary_root}/unselected-pins.env" \
    > "${temporary_root}/fmt-check.log" 2>&1; then
    cat "${temporary_root}/fmt-check.log" >&2
    exit 1
fi
printf 'Checking formatting... ok\n' > "${temporary_root}/expected-check-output"
cmp "${temporary_root}/expected-check-output" "${temporary_root}/fmt-check.log"
cmp "${temporary_root}/working-lock" Cargo.lock
git show :Cargo.lock > "${temporary_root}/after-check-index-lock"
cmp "${temporary_root}/index-lock" "${temporary_root}/after-check-index-lock"

# Use the actual consumer recipe and reporter with a sorter that fails after
# both output streams. Rustfmt must not run, and no working/index input changes.
mkdir "${temporary_root}/format-logs"
cat > "${temporary_root}/failing-cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    'sort --version') echo "cargo-sort $SHARED_TOOLING_CARGO_SORT_VERSION"; exit 0 ;;
    'fmt --version') exit 0 ;;
    sort*) echo 'sorter stdout details'; echo 'sorter stderr details' >&2; exit 23 ;;
    *) echo 'unexpected rustfmt execution' > "$FORMAT_TEST_EVENTS"; exit 99 ;;
esac
CARGO
chmod +x "${temporary_root}/failing-cargo"
printf 'sorter stdout details\nsorter stderr details\n' > "${temporary_root}/expected-diagnostics"
capture_before_hook
for target in fmt fmt-check; do
    status=0
    RUNNER_TEMP="${temporary_root}/format-logs" \
        SHARED_TOOLING_CARGO_SORT_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION" \
        FORMAT_TEST_EVENTS="${temporary_root}/unexpected-format-events" \
        make --no-print-directory "$target" "FORMAT_CARGO=${temporary_root}/failing-cargo" \
        > "${temporary_root}/$target-failure.log" 2>&1 || status=$?
    test "$status" -eq 2
    label=Formatting
    [[ "$target" != fmt-check ]] || label='Checking formatting'
    grep -Fxq "$label... FAILED (exit 23)" "${temporary_root}/$target-failure.log"
    logs=("${temporary_root}"/format-logs/formatting.*)
    test "${#logs[@]}" -eq 1
    printf 'Details: %q\n' "${logs[0]}" > "${temporary_root}/expected-log-path"
    grep -Fxf "${temporary_root}/expected-log-path" "${temporary_root}/$target-failure.log"
    cmp "${temporary_root}/expected-diagnostics" "${logs[0]}"
    test ! -e "${temporary_root}/unexpected-format-events"
    assert_unchanged
    mv "${logs[0]}" "${temporary_root}/$target-diagnostics.log"
done

# Partial staging must reject before formatting or refreshing the real index.
printf 'pub fn fixture() {}\n// unstaged edit\n' > testing/crates/hook-probe/src/lib.rs
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/partial-staging.log" 2>&1; then
    echo 'error: commit hook accepted partially staged probe Rust' >&2
    exit 1
fi
assert_unchanged
git checkout-index -f -- testing/crates/hook-probe/src/lib.rs

# A failed consumer formatter must not copy or stage even its earlier changes.
printf 'pub fn fixture( ){}\n' > testing/crates/hook-probe/src/lib.rs
git add testing/crates/hook-probe/src/lib.rs
printf 'fmt:\n\t@printf "pub fn fixture() {}\\n" > testing/crates/hook-probe/src/lib.rs\n\t@exit 1\n' > Makefile
git add Makefile
capture_before_hook
if bash "${repository_root}/.githooks/pre-commit" > "${temporary_root}/failed-formatter.log" 2>&1; then
    echo 'error: commit hook accepted failed snapshot formatting' >&2
    exit 1
fi
assert_unchanged
fixture_complete=true
echo 'Consumer hook auto-formatting, nested selection and failure-isolation checks passed'
