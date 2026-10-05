#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
finalizer="${repository_root}/scripts/release/finalize-changelog.sh"
temporary_root="$(mktemp -d)"
trap 'rm -rf -- "${temporary_root}"' EXIT

cat > "${temporary_root}/history.md" <<'HISTORY'
## [0.1.0] - 2026-08-01

- Published behavior.
HISTORY
# Both versionless and maintainer-named drafts select the same release flow.
for label in Draft 0.1.1; do
    printf '# Changelog\n\n## [%s]\n\n- Fix terminal cleanup.\n\n' "${label}" > "${temporary_root}/CHANGELOG.md"
    cat "${temporary_root}/history.md" >> "${temporary_root}/CHANGELOG.md"
    cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
    chmod 0640 "${temporary_root}/CHANGELOG.md"
    bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    bash "${finalizer}" 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    printf '# Changelog\n\n## [0.1.1] - 2026-08-02\n\n- Fix terminal cleanup.\n\n' > "${temporary_root}/expected.md"
    cat "${temporary_root}/history.md" >> "${temporary_root}/expected.md"
    cmp "${temporary_root}/expected.md" "${temporary_root}/CHANGELOG.md"
    perl -e 'exit(((stat $ARGV[0])[2] & 07777) == 0640 ? 0 : 1);' "${temporary_root}/CHANGELOG.md"
    if bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
        echo 'error: a finalized release was selected again' >&2
        exit 1
    fi
    cmp "${temporary_root}/expected.md" "${temporary_root}/CHANGELOG.md"
done

# Presentation gaps do not stand in for the classifier's no-change decision.
for scenario in empty-draft missing-draft missing-file misplaced-draft; do
    printf '# Changelog\n\n' > "${temporary_root}/CHANGELOG.md"
    case "${scenario}" in
        empty-draft) printf '## [Draft]\n\n' >> "${temporary_root}/CHANGELOG.md" ;;
        missing-file) rm "${temporary_root}/CHANGELOG.md" ;;
    esac
    if [[ "${scenario}" != missing-file ]]; then
        cat "${temporary_root}/history.md" >> "${temporary_root}/CHANGELOG.md"
        if [[ "${scenario}" == misplaced-draft ]]; then
            printf '\n## [Draft]\n\n- Fix terminal cleanup.\n' >> "${temporary_root}/CHANGELOG.md"
        fi
        cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
    fi
    bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    if [[ "${scenario}" == missing-file ]]; then
        test ! -e "${temporary_root}/CHANGELOG.md"
    else
        cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    fi
    bash "${finalizer}" 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    test "$(awk '/^## / { print; exit }' "${temporary_root}/CHANGELOG.md")" = '## [0.1.1] - 2026-08-02'
    if [[ "${scenario}" != missing-file ]]; then
        perl -0ne 'if (/^(## \[0.1.0\].*)/ms) { my $history = $1;
            $history =~ s/\n+\z/\n/; print $history; }' \
            "${temporary_root}/CHANGELOG.md" > "${temporary_root}/preserved-history.md"
        cmp "${temporary_root}/history.md" "${temporary_root}/preserved-history.md"
    fi
done

# Release preparation must refuse competing pending batches without editing them.
printf '# Changelog\n\n## [Draft]\n\n- First batch.\n\n## [0.1.1]\n\n- Second batch.\n' > "${temporary_root}/CHANGELOG.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
if bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
    echo 'error: release preparation accepted competing drafts' >&2
    exit 1
fi
cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"

# A chosen minor line cannot silently become a patch through its bump command.
printf '# Changelog\n\n## [0.2.0]\n\n- Public contract change.\n' > "${temporary_root}/CHANGELOG.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
if bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
    echo 'error: release preparation overwrote a conflicting chosen version' >&2
    exit 1
fi
cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"

# Reject output paths that could replace another owner's link or directory.
printf '# Changelog\n\n## [Draft]\n\n- Valid pending notes.\n' > "${temporary_root}/owned-target.md"
cp "${temporary_root}/owned-target.md" "${temporary_root}/expected-target.md"
for kind in symlink directory; do
    output="${temporary_root}/output.md"
    if [[ "${kind}" == symlink ]]; then
        ln -s "${temporary_root}/owned-target.md" "${output}"
    else
        mkdir "${output}"
    fi
    if bash "${finalizer}" 0.1.1 2026-08-02 "${output}" >/dev/null 2>&1; then
        echo "error: changelog finalization accepted a ${kind} output" >&2
        exit 1
    fi
    cmp "${temporary_root}/expected-target.md" "${temporary_root}/owned-target.md"
    if [[ "${kind}" == symlink ]]; then
        test -L "${output}"
        rm "${output}"
    else
        rmdir "${output}"
    fi
done

echo 'Draft finalization checks passed'
