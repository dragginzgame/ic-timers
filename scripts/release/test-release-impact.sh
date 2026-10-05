#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
classifier="${repository_root}/scripts/release/classify-release-impact.sh"
temporary_root="$(mktemp -d)"
cleanup() {
    rm -rf -- "${temporary_root}"
}
trap cleanup EXIT

git init -q "${temporary_root}"
mkdir -p \
    "${temporary_root}/crates/ic-timers/src" \
    "${temporary_root}/docs"
printf '%s\n' \
    '[workspace]' \
    'members = ["crates/ic-timers"]' \
    '' \
    '[workspace.package]' \
    'version = "0.3.8"' \
    > "${temporary_root}/Cargo.toml"
printf '%s\n' \
    '[package]' \
    'name = "ic-timers"' \
    'version.workspace = true' \
    > "${temporary_root}/crates/ic-timers/Cargo.toml"
printf '%s\n' 'pub fn fixture() {}' \
    > "${temporary_root}/crates/ic-timers/src/lib.rs"
git -C "${temporary_root}" add .
git -C "${temporary_root}" \
    -c user.name='ic-timers release test' \
    -c user.email='release-test@example.invalid' \
    commit -qm 'fixture'
git -C "${temporary_root}" tag v0.3.8

classify() {
    (
        cd "${temporary_root}"
        bash "${classifier}" v0.3.8
    )
}

impact="$(classify)"
if [[ "${impact}" != "none" ]]; then
    echo "error: unchanged release subject was not classified as none" >&2
    exit 1
fi

printf '%s\n' 'Repository evidence.' > "${temporary_root}/docs/evidence.md"
impact="$(classify)"
if [[ "${impact}" != "repository" ]]; then
    echo "error: documentation-only work was not classified as repository" >&2
    exit 1
fi

printf '%s\n' 'pub fn added() {}' \
    > "${temporary_root}/crates/ic-timers/src/added.rs"
impact="$(classify)"
if [[ "${impact}" != "crate" ]]; then
    echo "error: untracked crate source was not classified as crate" >&2
    exit 1
fi
rm -f -- "${temporary_root}/crates/ic-timers/src/added.rs"

# Display-quoted paths remain crate changes, before and after staging. Force
# Git's default non-ASCII quoting independently of the maintainer's configuration.
git -C "${temporary_root}" config core.quotePath true
for filename in 'space name.rs' $'tab\tname.rs' $'line\nname.rs' 'quote"name.rs' $'caf\303\251.rs'; do
    path="crates/ic-timers/src/${filename}"
    printf '%s\n' 'pub fn fixture() {}' > "${temporary_root}/${path}"
    impact="$(classify)"
    if [[ "${impact}" != crate ]]; then
        echo 'error: untracked display-quoted crate path was not classified as crate' >&2
        exit 1
    fi
    git -C "${temporary_root}" add -- "${path}"
    impact="$(classify)"
    if [[ "${impact}" != crate ]]; then
        echo 'error: staged display-quoted crate path was not classified as crate' >&2
        exit 1
    fi
    git -C "${temporary_root}" rm -q -f -- "${path}"
done

printf '%s\n' '# package metadata changed' \
    >> "${temporary_root}/crates/ic-timers/Cargo.toml"
impact="$(classify)"
if [[ "${impact}" != "crate" ]]; then
    echo "error: crate-manifest work was not classified as crate" >&2
    exit 1
fi

git -C "${temporary_root}" restore crates/ic-timers/Cargo.toml
printf '%s\n' '# workspace package metadata changed' \
    >> "${temporary_root}/Cargo.toml"
impact="$(classify)"
if [[ "${impact}" != "crate" ]]; then
    echo "error: workspace-manifest work was not classified as crate" >&2
    exit 1
fi

# Default classification must work after a version advance without its tag.
git -C "${temporary_root}" restore Cargo.toml
perl -pi -e 's/^version = "0\.3\.8"$/version = "0.3.9"/' "${temporary_root}/Cargo.toml"
impact="$(cd "${temporary_root}" && bash "${classifier}")"
if [[ "${impact}" != crate ]]; then
    echo 'error: untagged workspace version did not use reachable release history' >&2
    exit 1
fi
if output="$(cd "${temporary_root}" && bash "${classifier}" v0.3.9 2>&1)"; then
    echo 'error: explicit missing release-impact base was accepted' >&2
    exit 1
fi
if [[ "${output}" != *'base does not resolve to a commit: v0.3.9'* ]]; then
    echo "error: explicit base rejection lost its reason: ${output}" >&2
    exit 1
fi

real_git="$(command -v git)"
mkdir -p "${temporary_root}/bin"
mkdir -p "${temporary_root}/scratch"
cat > "${temporary_root}/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "${FAIL_GIT_COMMAND}" ]]; then
    if [[ "${FAIL_GIT_PARTIAL_OUTPUT:-0}" == 1 ]]; then
        case "${1}" in
            diff | ls-files) printf '%s\0' 'crates/ic-timers/src/lib.rs' ;;
            describe) printf '%s\n' v0.3.8 ;;
        esac
    fi
    echo "injected ${FAIL_GIT_COMMAND} failure" >&2
    exit 1
fi
exec "${REAL_GIT}" "$@"
EOF
chmod +x "${temporary_root}/bin/git"
for command in diff ls-files describe; do
    set -- v0.3.8
    if [[ "${command}" == describe ]]; then set --; fi
    for partial_output in 0 1; do
        if output="$(
            cd "${temporary_root}"
            PATH="${temporary_root}/bin:${PATH}" REAL_GIT="${real_git}" \
                TMPDIR="${temporary_root}/scratch" FAIL_GIT_COMMAND="${command}" \
                FAIL_GIT_PARTIAL_OUTPUT="${partial_output}" \
                bash "${classifier}" "$@" 2>&1
        )"; then
            echo "error: impact classification accepted a failed git ${command}" >&2
            exit 1
        fi
        if [[ "${output}" != *"injected ${command} failure"* ]]; then
            echo "error: impact classification lost git ${command} failure: ${output}" >&2
            exit 1
        fi
        debris="$(find "${temporary_root}/scratch" -type f -print)"
        test -z "${debris}"
    done
done

git -C "${temporary_root}" tag -d v0.3.8 >/dev/null
if (cd "${temporary_root}" && bash "${classifier}") >/dev/null 2>&1; then
    echo 'error: default classification accepted missing release history' >&2
    exit 1
fi

echo "Release-impact classification checks passed"
