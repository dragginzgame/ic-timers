#!/usr/bin/env bash
set -euo pipefail

repository_root="$(git rev-parse --show-toplevel)"
finalizer="${repository_root}/scripts/release/finalize-changelog.sh"
temporary_root="$(mktemp -d)"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf -- "${temporary_root}"
    else
        printf "Failed changelog fixture retained: %s\n" "${temporary_root}" >&2
    fi
    exit "$status"
}
trap finish EXIT
export IC_TIMERS_RELEASE_PREVIOUS=0.1.0

# Keep successful child output private, but show the full diagnostic if
# finalization unexpectedly fails. Empty-note fixtures intentionally warn.
finalize_fixture() {
    if ! bash "${finalizer}" "$@" > "${temporary_root}/finalizer.log" 2>&1; then
        cat "${temporary_root}/finalizer.log" >&2
        return 1
    fi
}

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
    finalize_fixture --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    finalize_fixture 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
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
    finalize_fixture --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    if [[ "${scenario}" == missing-file ]]; then
        test ! -e "${temporary_root}/CHANGELOG.md"
    else
        cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    fi
    finalize_fixture 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    test "$(awk '/^## / { print; exit }' "${temporary_root}/CHANGELOG.md")" = '## [0.1.1] - 2026-08-02'
    if [[ "${scenario}" != missing-file ]]; then
        perl -0ne 'if (/^(## \[0.1.0\].*)/ms) { my $history = $1;
            $history =~ s/\n+\z/\n/; print $history; }' \
            "${temporary_root}/CHANGELOG.md" > "${temporary_root}/preserved-history.md"
        cmp "${temporary_root}/history.md" "${temporary_root}/preserved-history.md"
    fi
done

# Imported undated versions at or below the saved previous version are history,
# including their original whitespace. Only the current pending notes move.
printf '## [0.1.0]  \t\n\n- Imported previous notes.\n\n## [0.0.9]\n\n- Older notes.\n' \
    > "${temporary_root}/undated-history.md"
for label in Draft 0.1.1 absent; do
    printf '# Changelog\n\n' > "${temporary_root}/CHANGELOG.md"
    printf '# Changelog\n\n## [0.1.1] - 2026-08-02\n\n' > "${temporary_root}/expected.md"
    if [[ "${label}" != absent ]]; then
        printf '## [%s]  \t\n\n- Pending notes.\n\n' "${label}" >> "${temporary_root}/CHANGELOG.md"
        printf '%s\n\n' '- Pending notes.' >> "${temporary_root}/expected.md"
    fi
    cat "${temporary_root}/undated-history.md" >> "${temporary_root}/CHANGELOG.md"
    cat "${temporary_root}/undated-history.md" >> "${temporary_root}/expected.md"
    cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
    finalize_fixture --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    finalize_fixture 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
    cmp "${temporary_root}/expected.md" "${temporary_root}/CHANGELOG.md"
done

# Historical bytes must survive finalization without adding a terminal newline
# or normalizing whitespace. Compare whole files: extracting lines or trimming
# the suffix would hide the shared selector's former EOF preservation failure.
for history_heading in '## [0.1.0] - 2026-08-01' '## [0.1.0]'; do
    for ending in absent newline whitespace; do
        printf '%s  \t\n\n- Published behavior.  \t' "${history_heading}" \
            > "${temporary_root}/byte-history.md"
        case "${ending}" in
            newline) printf '\n' >> "${temporary_root}/byte-history.md" ;;
            whitespace) printf '\n\n \t\n' >> "${temporary_root}/byte-history.md" ;;
        esac
        for label in Draft absent; do
            printf '# Changelog\n\n' > "${temporary_root}/CHANGELOG.md"
            printf '# Changelog\n\n## [0.1.1] - 2026-08-02\n\n' > "${temporary_root}/expected.md"
            if [[ "${label}" == Draft ]]; then
                printf '## [Draft]\n\n- Pending notes.\n\n' >> "${temporary_root}/CHANGELOG.md"
                printf '%s\n\n' '- Pending notes.' >> "${temporary_root}/expected.md"
            fi
            cat "${temporary_root}/byte-history.md" >> "${temporary_root}/CHANGELOG.md"
            cat "${temporary_root}/byte-history.md" >> "${temporary_root}/expected.md"
            cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
            finalize_fixture --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
            cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
            finalize_fixture 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md"
            cmp "${temporary_root}/expected.md" "${temporary_root}/CHANGELOG.md"
        done
    done
done

# Comparison must not lose precision above the exact floating-point range.
# These adjacent components are also within Cargo SemVer's u64 range.
previous=0.9007199254740992.0
candidate=0.9007199254740993.0
printf '# Changelog\n\n## [%s]\n\n- Pending notes.\n\n## [%s]\n\n- History.\n' \
    "${candidate}" "${previous}" > "${temporary_root}/CHANGELOG.md"
printf '# Changelog\n\n## [%s] - 2026-08-02\n\n- Pending notes.\n\n## [%s]\n\n- History.\n' \
    "${candidate}" "${previous}" > "${temporary_root}/expected.md"
IC_TIMERS_RELEASE_PREVIOUS="${previous}" \
    finalize_fixture "${candidate}" 2026-08-02 "${temporary_root}/CHANGELOG.md"
cmp "${temporary_root}/expected.md" "${temporary_root}/CHANGELOG.md"

# Release preparation must refuse competing pending batches without editing them.
printf '# Changelog\n\n## [Draft]\n\n- First batch.\n\n## [0.1.1]\n\n- Second batch.\n' > "${temporary_root}/CHANGELOG.md"
cat "${temporary_root}/undated-history.md" >> "${temporary_root}/CHANGELOG.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
if bash "${finalizer}" --check 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
    echo 'error: release preparation accepted competing drafts' >&2
    exit 1
fi
cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"

# Malformed/non-increasing previous identities and dated targets never authorize
# a rewrite. Spaced dated headings retain the consumer's existing refusal.
printf '# Changelog\n\n## [Draft]\n\n- Pending notes.\n' > "${temporary_root}/CHANGELOG.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
for previous in malformed 00.1.0 0.1.1 0.2.0; do
    if IC_TIMERS_RELEASE_PREVIOUS="${previous}" bash "${finalizer}" \
        0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
        echo "error: finalization accepted invalid previous identity ${previous}" >&2
        exit 1
    fi
    cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
done
for date in 2026-08-01 2026-08-02; do
    printf '# Changelog\n\n## [0.1.1]  - \t%s \t\n\n- Published notes.\n' \
        "${date}" > "${temporary_root}/CHANGELOG.md"
    cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
    if bash "${finalizer}" 0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md" >/dev/null 2>&1; then
        echo 'error: finalization accepted a spaced dated target' >&2
        exit 1
    fi
    cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
done

# Failed readers and candidate producers may print plausible bytes. Status owns
# admission, and failure must preserve the complete original changelog.
mkdir "${temporary_root}/bin"
printf '# Changelog\n\n## [Draft]\n\n- Pending notes.\n' > "${temporary_root}/CHANGELOG.md"
cp "${temporary_root}/CHANGELOG.md" "${temporary_root}/original.md"
for producer in awk cat; do
    emitted_heading='## [0.1.1] - 2026-08-02'
    # Reader output must be admissible to the real selector, so only the
    # reader's failure status prevents replacement in this case.
    if [[ "${producer}" == cat ]]; then emitted_heading='## [Draft]'; fi
    printf '%s\n' '#!/usr/bin/env bash' \
        "printf '# Changelog\\n\\n${emitted_heading}\\n\\n- Partial notes.\\n'" \
        'exit 1' > "${temporary_root}/bin/${producer}"
    chmod +x "${temporary_root}/bin/${producer}"
    for operation in check prepare; do
        arguments=(0.1.1 2026-08-02 "${temporary_root}/CHANGELOG.md")
        if [[ "${operation}" == check ]]; then arguments=(--check "${arguments[@]}"); fi
        if PATH="${temporary_root}/bin:${PATH}" bash "${finalizer}" \
            "${arguments[@]}" >/dev/null 2>&1; then
            echo "error: ${operation} accepted failed ${producer} output" >&2
            exit 1
        fi
        cmp "${temporary_root}/original.md" "${temporary_root}/CHANGELOG.md"
    done
    rm "${temporary_root}/bin/${producer}"
done

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

fixture_complete=true
echo 'Draft finalization checks passed'
