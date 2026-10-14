#!/usr/bin/env bash
set -euo pipefail

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
temporary="${TMPDIR:-/tmp}"
[[ "$temporary" == /* ]] || temporary="$PWD/$temporary"
temporary="$(cd -P "$temporary" && printf '%s/.' "$PWD")"
temporary="${temporary%/.}"
fixture="$(mktemp -d "$temporary/timer-evidence-test.XXXXXX")"
fixture="$(cd -P "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
# All actual installer/check/collector calls in this fixture share an isolated
# store; no developer installation can be read, mutated or reused accidentally.
export SHARED_TOOLING_STORE="$fixture/shared-store"
# Bash 3.2 can report zero on nounset; cleanup also requires completion.
fixture_complete=false
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
trap 'fixture_finish "$?" "$fixture" "Failed evidence fixture retained"' EXIT
export GITHUB_WORKSPACE="$fixture/repo" RUNNER_TEMP="$fixture/runner"
export GITHUB_SHA=fixture-event-sha GITHUB_JOB=fixture-job
export RUNNER_OS=fixture-os RUNNER_ARCH=fixture-arch GITHUB_RUN_ID=123 GITHUB_RUN_ATTEMPT=2
mkdir -p "$GITHUB_WORKSPACE" "$RUNNER_TEMP"
# Probe the actual collector EXIT boundary, truncating before Git inspection,
# tool selection or archiving. Existing full collection cases below still prove
# the successful archive path. Use the invoking Bash, including native Bash 3.2.
mkdir -p "$fixture/collector-probe/scripts/ci" "$fixture/collector scratch"
export CI_EVIDENCE_EXIT_PATH="$fixture/collector-exit-path"
for failure in nounset command nonzero premature completed failed-completion; do
    # shellcheck disable=SC2016 # Expanded only by the disposable collector copy.
    case "$failure" in
        nounset) injection='unset COLLECTOR_UNBOUND; printf "%s\n" "$COLLECTOR_UNBOUND"'; expected=1 ;;
        command) injection='false'; expected=1 ;;
        nonzero) injection='exit 23'; expected=23 ;;
        premature) injection='exit 0'; expected=1 ;;
        completed) injection='collection_complete=true; exit 0'; expected=0 ;;
        failed-completion) injection='collection_complete=true; exit 23'; expected=23 ;;
    esac
    COLLECTOR_INJECTION="$injection" awk '
        { print }
        /^trap finish EXIT$/ {
            print "printf \"%s\\n\" \"$metadata\" > \"$CI_EVIDENCE_EXIT_PATH\""
            print "printf evidence > \"$metadata/exit-evidence\""
            print ENVIRON["COLLECTOR_INJECTION"]
            print "exit 99"
            injected=1
            exit
        }
        END { if (!injected) exit 1 }
    ' "$root/scripts/ci/collect-failure-evidence.sh" \
        > "$fixture/collector-probe/scripts/ci/collect-failure-evidence.sh"
    rm -f -- "$CI_EVIDENCE_EXIT_PATH"
    status=0
    RUNNER_TEMP="$fixture/collector scratch" \
        "$BASH" "$fixture/collector-probe/scripts/ci/collect-failure-evidence.sh" \
        > "$fixture/collector-exit-$failure.log" 2>&1 || status=$?
    [[ "$status" == "$expected" ]] || exit 1
    retained="$(cat "$CI_EVIDENCE_EXIT_PATH")"
    [[ -n "$retained" ]] || exit 1
    if [[ "$expected" == 0 ]]; then [[ ! -e "$retained" ]] || exit 1
    else
        [[ -d "$retained" && "$(cat "$retained/exit-evidence")" == evidence ]] || exit 1
        grep -Fq "Failed evidence collection retained: $retained" "$fixture/collector-exit-$failure.log"
    fi
done
# Read-only reuse of committed objects gives the fixture an actual checkout SHA.
objects="$(git -C "$root" rev-parse --git-path objects && printf '.')"
objects="${objects%$'\n'.}"
case "$objects" in /*) ;; *) objects="$root/$objects" ;; esac
git init -q "$GITHUB_WORKSPACE"
mkdir -p "$GITHUB_WORKSPACE/.git/objects/info"
printf '%s\n' "$objects" > "$GITHUB_WORKSPACE/.git/objects/info/alternates"
git -C "$GITHUB_WORKSPACE" update-ref HEAD "$(git -C "$root" rev-parse HEAD)"
scratch="$RUNNER_TEMP/ic-timers-fixtures/failed case"
mkdir -p "$scratch/.git" "$GITHUB_WORKSPACE/.git/release-state/validation-failures" \
    "$GITHUB_WORKSPACE/target/validation-failures" "$GITHUB_WORKSPACE/target/build-cache" \
    "$GITHUB_WORKSPACE/.tools/host-set.failed" "$GITHUB_WORKSPACE/.tools/ic-set.failed" \
    "$GITHUB_WORKSPACE/.tools/host-set.active" "$GITHUB_WORKSPACE/.tools/ic-set.active"
printf 'scenario output\n' > "$scratch/scenario.log"
printf 'original metadata\n' > "$scratch/before"
chmod 0640 "$scratch/before"
printf 'fixture Git configuration\n' > "$scratch/.git/config"
printf 'release raw output\n' > "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log"
printf 'validation raw output\n' > "$GITHUB_WORKSPACE/target/validation-failures/raw.log"
printf 'failed host payload\n' > "$GITHUB_WORKSPACE/.tools/host-set.failed/tool"
chmod 0751 "$GITHUB_WORKSPACE/.tools/host-set.failed/tool"
printf 'failed IC payload\n' > "$GITHUB_WORKSPACE/.tools/ic-set.failed/tool"
printf 'selected host tool\n' > "$GITHUB_WORKSPACE/.tools/host-set.active/tool"
printf 'selected IC tool\n' > "$GITHUB_WORKSPACE/.tools/ic-set.active/tool"
ln -s host-set.active "$GITHUB_WORKSPACE/.tools/host"
ln -s ic-set.active "$GITHUB_WORKSPACE/.tools/ic"
printf 'build cache\n' > "$GITHUB_WORKSPACE/target/build-cache/object"
printf 'outside scratch\n' > "$fixture/outside"
ln -s "$fixture/outside" "$scratch/alias"
unusual=$'colon:and\nnewline\n'
printf 'unusual filename bytes\n' > "$scratch/$unusual"
chmod 0640 "$scratch/$unusual"

mkdir -p "$GITHUB_WORKSPACE/.tools/rust/build/failed-cli" \
    "$GITHUB_WORKSPACE/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/bin" \
    "$GITHUB_WORKSPACE/.tools/testkit-server/.setup-v1-failed" \
    "$GITHUB_WORKSPACE/.tools/testkit-server/admitted"
printf 'failed Cargo build\n' > "$GITHUB_WORKSPACE/.tools/rust/build/failed-cli/install.log"
rust_slot="$GITHUB_WORKSPACE/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed"
printf 'completed Cargo build\n' > "$rust_slot/install.log"
printf 'selected Cargo receipt\n' > "$rust_slot/selection.json"
printf 'admitted Rust executable\n' > "$rust_slot/bin/cargo-sort"
chmod 0751 "$rust_slot/bin/cargo-sort"
ln -s cargo-sort-2.1.4-bin-cargo-sort-release/installed "$GITHUB_WORKSPACE/.tools/rust/common"
# Create a failed shared Rust generation and reference through the canonical
# store owner. No successful Cargo receipt or reusable build identity is forged;
# owner installer tests qualify compiler/package matching and two-checkout reuse.
(
    source "$root/scripts/dev/tool-store.sh"
    consumer="$GITHUB_WORKSPACE"
    tool_root="$consumer/.tools/rust"
    tool_store_init rust timer-evidence-fixture
    tool_store_acquire
    trap tool_store_release EXIT
    tool_store_new
    mkdir "$stage/build"
    printf 'failed shared Cargo compilation\n' > "$stage/install.log"
    printf 'shared failed build bytes\n' > "$stage/build/object"
    chmod 0640 "$stage/build/object"
    printf '%s\n' "$stage" > "$fixture/shared-rust-path"
    printf '%s\n' "$store_reference" > "$fixture/shared-rust-reference"
)
shared_rust="$(cat "$fixture/shared-rust-path")"
shared_rust_relative="${shared_rust#"$SHARED_TOOLING_STORE/"}"
shared_rust_reference="$(cat "$fixture/shared-rust-reference")"
mkdir "$fixture/external-rust"
printf 'outside Rust evidence root\n' > "$fixture/external-rust/not-selected"
ln -s "$fixture/external-rust" "$GITHUB_WORKSPACE/.tools/rust/external"
printf 'failed server download\n' > "$GITHUB_WORKSPACE/.tools/testkit-server/.setup-v1-failed/failure.txt"
printf 'admitted server bytes\n' > "$GITHUB_WORKSPACE/.tools/testkit-server/admitted/pocket-ic"
for phase in install check; do
    printf 'aggregate %s stdout\naggregate %s stderr\n' "$phase" "$phase" \
        > "$RUNNER_TEMP/tools-$phase.log"
    chmod 0640 "$RUNNER_TEMP/tools-$phase.log"
done
# A real reporter failure preserves its original status and both streams. The
# consumer archive must contain those exact diagnostic bytes and mode.
format_status=0
bash "$root/scripts/ci/run-formatting.sh" --check bash -c \
    'echo "formatter stdout details"; echo "formatter stderr details" >&2; exit 23' \
    > "$fixture/formatting-output.log" 2>&1 || format_status=$?
test "$format_status" -eq 23
grep -Fxq 'Checking formatting... FAILED (exit 23)' "$fixture/formatting-output.log"
format_logs=("$RUNNER_TEMP"/formatting.*)
test "${#format_logs[@]}" -eq 1
format_log="${format_logs[0]}"
printf 'formatter stdout details\nformatter stderr details\n' > "$fixture/expected-format-log"
cmp "$fixture/expected-format-log" "$format_log"
printf 'Details: %q\n' "$format_log" > "$fixture/expected-format-path"
grep -Fxf "$fixture/expected-format-path" "$fixture/formatting-output.log"
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/collection.log" 2>&1
archive_bytes="$(wc -c < "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz")"
archive_bytes="${archive_bytes//[[:space:]]/}"
grep -Eq "^CI failure evidence measurements: archive_bytes=${archive_bytes} archive_seconds=[0-9]+$" \
    "$fixture/collection.log"
mkdir "$fixture/extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/extracted"
cmp "$format_log" "$fixture/extracted/${format_log##*/}"
perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost formatter log mode\n";' \
    "$format_log" "$fixture/extracted/${format_log##*/}"
rm "$format_log"
for phase in install check; do
    cmp "$RUNNER_TEMP/tools-$phase.log" "$fixture/extracted/tools-$phase.log"
    perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
        @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost aggregate log mode\n";' \
        "$RUNNER_TEMP/tools-$phase.log" "$fixture/extracted/tools-$phase.log"
    rm "$RUNNER_TEMP/tools-$phase.log"
done
for path in .tools/rust/build/failed-cli/install.log \
    .tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/install.log \
    .tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/selection.json \
    .tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/bin/cargo-sort \
    .tools/testkit-server/.setup-v1-failed/failure.txt; do
    cmp "$GITHUB_WORKSPACE/$path" "$fixture/extracted/$path"
done
perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost Rust executable mode\n";' \
    "$rust_slot/bin/cargo-sort" \
    "$fixture/extracted/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/bin/cargo-sort"
test -L "$fixture/extracted/.tools/rust/common"
test "$(readlink "$fixture/extracted/.tools/rust/common")" = cargo-sort-2.1.4-bin-cargo-sort-release/installed
test -L "$fixture/extracted/.tools/rust/external"
test "$(readlink "$fixture/extracted/.tools/rust/external")" = "$fixture/external-rust"
tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/archive-paths"
if grep -Fq '/not-selected' "$fixture/archive-paths"; then exit 1; fi
cmp "$shared_rust/install.log" "$fixture/extracted/$shared_rust_relative/install.log"
cmp "$shared_rust/build/object" "$fixture/extracted/$shared_rust_relative/build/object"
perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost shared Rust build mode\n";' \
    "$shared_rust/build/object" "$fixture/extracted/$shared_rust_relative/build/object"
test -L "$fixture/extracted/.tools/rust/$shared_rust_reference"
test "$(readlink "$fixture/extracted/.tools/rust/$shared_rust_reference")" = "$shared_rust"
test ! -e "$fixture/extracted/.tools/testkit-server/admitted"

for path in scenario.log before "$unusual"; do
    cmp "$scratch/$path" "$fixture/extracted/ic-timers-fixtures/failed case/$path"
done
cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" \
    "$fixture/extracted/validation-failures/raw.log"
for path in target/validation-failures/raw.log .tools/host-set.failed/tool .tools/ic-set.failed/tool \
    .tools/host-set.active/tool .tools/ic-set.active/tool; do
    cmp "$GITHUB_WORKSPACE/$path" "$fixture/extracted/$path"
done
test ! -e "$fixture/extracted/ic-timers-fixtures/failed case/.git"
test ! -e "$fixture/extracted/.tools/host"
test ! -e "$fixture/extracted/.tools/ic"
test ! -e "$fixture/extracted/target/build-cache"
test -L "$fixture/extracted/ic-timers-fixtures/failed case/alias"
test "$(readlink "$fixture/extracted/ic-timers-fixtures/failed case/alias")" = "$fixture/outside"
perl -e 'for my $pair (0, 2) { my @a=stat $ARGV[$pair]; my @b=stat $ARGV[$pair+1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost file modes\n"; }' \
    "$scratch/before" "$fixture/extracted/ic-timers-fixtures/failed case/before" \
    "$GITHUB_WORKSPACE/.tools/host-set.failed/tool" "$fixture/extracted/.tools/host-set.failed/tool"
perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
    @a && @b && ($a[2]&07777)==($b[2]&07777) or die "archive lost unusual-name mode\n";' \
    "$scratch/$unusual" "$fixture/extracted/ic-timers-fixtures/failed case/$unusual"
grep -Fxq "checkout_sha=$(git -C "$root" rev-parse HEAD)" "$fixture/extracted/identity.txt"
grep -Fxq 'event_sha=fixture-event-sha' "$fixture/extracted/identity.txt"
grep -Fxq 'job=fixture-job' "$fixture/extracted/identity.txt"
grep -Fxq 'host=fixture-os/fixture-arch' "$fixture/extracted/identity.txt"
grep -Fxq 'run=123' "$fixture/extracted/identity.txt"
grep -Fxq 'attempt=2' "$fixture/extracted/identity.txt"
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/initial-evidence.tar.gz"

# Exercise the actual collector, selector and offline installers with tiny
# authenticated executables. These are fixture bytes, not real tool admission
# or a compression benchmark. Failed/unselected sets and logs must remain full.
mkdir -p "$GITHUB_WORKSPACE/ci" "$GITHUB_WORKSPACE/.tools/host-set.active/bin" \
    "$GITHUB_WORKSPACE/.tools/ic-set.active/bin"
host_pins="$GITHUB_WORKSPACE/ci/tool-versions.env"
printf 'export SHARED_TOOLING_JQ_VERSION=1.8.2\nexport SHARED_TOOLING_YQ_VERSION=4.47.2\nexport SHARED_TOOLING_CLOC_VERSION=2.10\nexport SHARED_TOOLING_RIPGREP_VERSION=15.2.0\n' > "$host_pins"
for tool in jq yq cloc; do
    case "$tool" in
        jq) report=jq-1.8.2; key=JQ ;;
        yq) report='yq (https://github.com/mikefarah/yq/) version v4.47.2'; key=YQ ;;
        cloc) report=2.10; key=CLOC ;;
    esac
    payload="$GITHUB_WORKSPACE/.tools/host-set.active/bin/$tool"
    printf '#!/bin/sh\nprintf "%%s\\n" "%s"\n' "$report" > "$payload"
    chmod 0751 "$payload"
    digest="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$payload")"
    if [[ "$tool" == cloc ]]; then
        printf 'export SHARED_TOOLING_CLOC_SHA256=%s\n' "$digest" >> "$host_pins"
    else
        for host in LINUX_AMD64 LINUX_ARM64 DARWIN_AMD64 DARWIN_ARM64; do
            printf 'export SHARED_TOOLING_%s_SHA256_%s=%s\n' "$key" "$host" "$digest" >> "$host_pins"
        done
    fi
done
case "$(uname -s):$(uname -m)" in
    Linux:x86_64|Linux:amd64) host=LINUX_AMD64; target=x86_64-unknown-linux-musl; ic_host=linux-x86_64; ic_target=x86_64-unknown-linux-gnu; ic_os=linux; ic_arch=x86_64 ;;
    Darwin:x86_64|Darwin:amd64) host=DARWIN_AMD64; target=x86_64-apple-darwin; ic_host=darwin-x86_64; ic_target=x86_64-apple-darwin; ic_os=macos; ic_arch=x86_64 ;;
    Darwin:arm64|Darwin:aarch64) host=DARWIN_ARM64; target=aarch64-apple-darwin; ic_host=darwin-arm64; ic_target=aarch64-apple-darwin; ic_os=macos; ic_arch=arm64 ;;
    *) echo 'no complete native evidence fixture for this host' >&2; exit 1 ;;
esac
cat > "$GITHUB_WORKSPACE/.tools/host-set.active/bin/rg" <<'SCRIPT'
#!/bin/sh
case "$1" in
    --version) echo 'ripgrep 15.2.0' ;;
    --pcre2-version) echo 'PCRE2 available' ;;
    *) exit 2 ;;
esac
SCRIPT
chmod 0751 "$GITHUB_WORKSPACE/.tools/host-set.active/bin/rg"
mkdir "$fixture/ripgrep-15.2.0-$target"
cp -p "$GITHUB_WORKSPACE/.tools/host-set.active/bin/rg" "$fixture/ripgrep-15.2.0-$target/rg"
tar -czf "$GITHUB_WORKSPACE/.tools/host-set.active/ripgrep.tar.gz" \
    -C "$fixture" "ripgrep-15.2.0-$target/rg"
digest="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$GITHUB_WORKSPACE/.tools/host-set.active/ripgrep.tar.gz")"
printf 'export SHARED_TOOLING_RIPGREP_SHA256_%s=%s\n' "$host" "$digest" >> "$host_pins"
cp "$root/ci/ic-tools-qualified.tsv" "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv"
cp "$root/ci/ic-tools.tsv" "$GITHUB_WORKSPACE/ci/ic-tools.tsv"
mkdir "$fixture/assets" "$fixture/payloads" "$fixture/installer-bin"
export TIMER_EVIDENCE_ASSETS="$fixture/assets"
for tool in jq yq cloc; do
    cp -p "$GITHUB_WORKSPACE/.tools/host-set.active/bin/$tool" "$fixture/assets/$tool"
done
cp "$GITHUB_WORKSPACE/.tools/host-set.active/ripgrep.tar.gz" "$fixture/assets/ripgrep.tar.gz"
: > "$fixture/ic-archive-digests"
while IFS=$'\t' read -r tool version selected_host _; do
    [[ "$tool" != \#* && "$selected_host" == "$ic_host" ]] || continue
    report="$tool $version"
    [[ "$tool" != wasm-opt ]] || report="wasm-opt version $version"
    payload="$GITHUB_WORKSPACE/.tools/ic-set.active/bin/$tool"
    printf '#!/bin/sh\nprintf "%%s\\n" "%s"\n' "$report" > "$payload"
    chmod 0751 "$payload"
    case "$tool" in
        quill)
            asset="quill-$ic_os-$ic_arch"
            cp -p "$payload" "$fixture/assets/$asset" ;;
        wasm-opt)
            asset="binaryen-version_$version-$ic_arch-$ic_os.tar.gz"
            member="binaryen-version_$version"
            mkdir -p "$fixture/payloads/$member/bin" "$fixture/payloads/$member/lib"
            cp -p "$payload" "$fixture/payloads/$member/bin/wasm-opt"
            printf 'fixture native library\n' > "$fixture/payloads/$member/lib/libfixture.so"
            tar -czf "$fixture/assets/$asset" -C "$fixture/payloads" "$member" ;;
        *)
            package="$tool"; [[ "$tool" != icp ]] || package=icp-cli
            asset="$package-$ic_target.tar.xz"
            member="$package-$ic_target"
            mkdir "$fixture/payloads/$member"
            cp -p "$payload" "$fixture/payloads/$member/$tool"
            tar -cJf "$fixture/assets/$asset" -C "$fixture/payloads" "$member" ;;
    esac
    digest="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$fixture/assets/$asset")"
    printf '%s\t%s\n' "$tool" "$digest" >> "$fixture/ic-archive-digests"
done < "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv"
awk -F '\t' -v OFS='\t' -v host="$ic_host" '
    NR==FNR { digests[$1]=$2; next }
    $3==host { $4=digests[$1] }
    { print }
' "$fixture/ic-archive-digests" "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv" > "$fixture/authenticated-ic-pins"
mv "$fixture/authenticated-ic-pins" "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv"
cat > "$fixture/installer-bin/curl" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
output=''; url=''
while [[ $# -gt 0 ]]; do
    case "$1" in
        -o) output="$2"; shift 2 ;;
        https://*) url="$1"; shift ;;
        *) shift ;;
    esac
done
[[ -n "$output" && -n "$url" ]] || exit 2
asset="${url##*/}"
case "$asset" in
    jq-*) asset=jq ;;
    yq_*) asset=yq ;;
    cloc-*) asset=cloc ;;
    ripgrep-*) asset=ripgrep.tar.gz ;;
esac
cp -p "$TIMER_EVIDENCE_ASSETS/$asset" "$output"
SCRIPT
chmod +x "$fixture/installer-bin/curl"
# Use the actual canonical installers to generate admitted references, identities
# and receipts. Only their download boundary is substituted with tiny archives.
PATH="$fixture/installer-bin:$PATH" bash "$root/scripts/dev/install-host-tools.sh" \
    --consumer "$GITHUB_WORKSPACE" --versions "$host_pins" > "$fixture/host-store-install.log" 2>&1
PATH="$fixture/installer-bin:$PATH" bash "$root/scripts/dev/install-ic-tools.sh" \
    --consumer "$GITHUB_WORKSPACE" --pins "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv" > "$fixture/ic-store-install.log" 2>&1
host_bundle="$(cd -P "$GITHUB_WORKSPACE/.tools/host" && printf '%s' "$PWD")"
ic_bundle="$(cd -P "$GITHUB_WORKSPACE/.tools/ic" && printf '%s' "$PWD")"
host_stored="${host_bundle#"$SHARED_TOOLING_STORE/"}"
ic_stored="${ic_bundle#"$SHARED_TOOLING_STORE/"}"
# This unreferenced store entry must never enter a checkout's failure artifact.
mkdir "$SHARED_TOOLING_STORE/unrelated"
printf 'unrelated store bytes\n' > "$SHARED_TOOLING_STORE/unrelated/not-selected"
# Preserve installed provenance while the caller adds notes/reorders the same
# admitted records. Compaction must retain both distinct inputs without downloads.
awk '{ rows[NR]=$0 } END { print "# updated caller notes"; for (i=NR; i>0; i--) print rows[i] }' \
    "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv" > "$fixture/reordered-caller-pins"
mv "$fixture/reordered-caller-pins" "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv"
if cmp -s "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv" "$ic_bundle/pins.tsv"; then
    echo 'fixture needs distinct caller/installed pin provenance' >&2
    exit 1
fi
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/compact-collection.log" 2>&1
archive_bytes="$(wc -c < "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz")"
archive_bytes="${archive_bytes//[[:space:]]/}"
grep -Eq "^CI failure evidence measurements: archive_bytes=${archive_bytes} archive_seconds=[0-9]+$" \
    "$fixture/compact-collection.log"
mkdir "$fixture/compact-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/compact-extracted"
for kind in host ic; do
    selected="$(readlink "$GITHUB_WORKSPACE/.tools/$kind")"
    test ! -e "$fixture/compact-extracted/.tools/$selected"
    stored="$host_stored"; [[ "$kind" != ic ]] || stored="$ic_stored"
    test ! -e "$fixture/compact-extracted/$stored"
    cmp "$GITHUB_WORKSPACE/.tools/$kind-set.failed/tool" "$fixture/compact-extracted/.tools/$kind-set.failed/tool"
    test -s "$fixture/compact-extracted/tool-evidence/$kind/check.log"
    printf '%s\n' "$GITHUB_WORKSPACE/.tools/$selected" > "$fixture/expected-selection"
    cmp "$fixture/expected-selection" "$fixture/compact-extracted/tool-evidence/$kind/selection.txt"
done
cmp "$host_pins" "$fixture/compact-extracted/tool-evidence/host/caller-pins"
cmp "$GITHUB_WORKSPACE/ci/ic-tools-qualified.tsv" "$fixture/compact-extracted/tool-evidence/ic/caller-pins"
for receipt in pins.tsv host files.sha256; do
    cmp "$ic_bundle/$receipt" "$fixture/compact-extracted/tool-evidence/ic/$receipt"
done
cmp "$fixture/extracted/identity.txt" "$fixture/compact-extracted/identity.txt"
cmp "$scratch/scenario.log" "$fixture/compact-extracted/ic-timers-fixtures/failed case/scenario.log"
cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" "$fixture/compact-extracted/validation-failures/raw.log"
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/compact-evidence.tar.gz"

# Prior installation success must not hide an actively corrupted selection.
printf '\nchanged\n' >> "$host_bundle/bin/yq"
printf '\nchanged\n' >> "$ic_bundle/bin/quill"
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/changed-collection.log" 2>&1
mkdir "$fixture/changed-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/changed-extracted"
cmp "$host_bundle/bin/yq" "$fixture/changed-extracted/$host_stored/bin/yq"
cmp "$ic_bundle/bin/quill" "$fixture/changed-extracted/$ic_stored/bin/quill"
cmp "$ic_bundle/lib/libfixture.so" "$fixture/changed-extracted/$ic_stored/lib/libfixture.so"
for path in .tools/host-set.active/bin/yq .tools/ic-set.active/bin/quill \
    .tools/host-set.failed/tool .tools/ic-set.failed/tool; do
    cmp "$GITHUB_WORKSPACE/$path" "$fixture/changed-extracted/$path"
done
for kind in host ic; do
    selected="$(readlink "$GITHUB_WORKSPACE/.tools/$kind")"
    test -L "$fixture/changed-extracted/.tools/$selected"
    test "$(readlink "$fixture/changed-extracted/.tools/$selected")" = "$(readlink "$GITHUB_WORKSPACE/.tools/$selected")"
done
tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/shared-archive-paths"
if grep -Fq 'unrelated/not-selected' "$fixture/shared-archive-paths"; then exit 1; fi
for kind in host ic; do
    test ! -e "$fixture/changed-extracted/tool-evidence/$kind/selection.txt"
    grep -Fq 'Full bundle retained:' "$fixture/changed-extracted/tool-evidence/$kind/check.log"
done
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/changed-evidence.tar.gz"

# A failed selector must retain local evidence, ignore partial NUL selections
# and expose the original failure instead of reporting complete collection.
mkdir "$fixture/selection-bin"
printf '#!%s\n' "$BASH" > "$fixture/selection-bin/bash"
cat >> "$fixture/selection-bin/bash" <<'SCRIPT'
case "${1:-}" in
    */select-tool-evidence.sh)
        printf '%s\0%s\0partial-selection\0' "$EVIDENCE_OUTSIDE_ROOT" not-selected
        echo 'injected selection failure' >&2
        exit 23 ;;
esac
exec "$EVIDENCE_REAL_BASH" "$@"
SCRIPT
chmod +x "$fixture/selection-bin/bash"
collection_status=0
EVIDENCE_REAL_BASH="$BASH" EVIDENCE_OUTSIDE_ROOT="$fixture/external-rust" PATH="$fixture/selection-bin:$PATH" \
    "$BASH" "$root/scripts/ci/collect-failure-evidence.sh" \
    > "$fixture/failed-selection.log" 2>&1 || collection_status=$?
test "$collection_status" -eq 23
grep -Fq 'injected selection failure' "$fixture/failed-selection.log"
grep -Fq 'shared collection incomplete (exit 23)' "$fixture/failed-selection.log"
if grep -Fq 'CI failure evidence measurements:' "$fixture/failed-selection.log"; then exit 1; fi
mkdir "$fixture/failed-selection-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/failed-selection-extracted"
cmp "$GITHUB_WORKSPACE/target/validation-failures/raw.log" \
    "$fixture/failed-selection-extracted/target/validation-failures/raw.log"
cmp "$rust_slot/install.log" \
    "$fixture/failed-selection-extracted/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/install.log"
printf '23\n' > "$fixture/expected-selection-status"
cmp "$fixture/expected-selection-status" "$fixture/failed-selection-extracted/tool-evidence/selection-status"
grep -Fxq 'injected selection failure' "$fixture/failed-selection-extracted/tool-evidence/selection.log"
retained_selections=("$RUNNER_TEMP"/ic-timers-evidence.*)
test "${#retained_selections[@]}" -eq 1
printf '%s\0%s\0partial-selection\0' "$fixture/external-rust" not-selected > "$fixture/expected-selection-output"
cmp "$fixture/expected-selection-output" "${retained_selections[0]}/tool-evidence/selections.nul"
cmp "$fixture/expected-selection-output" "$fixture/failed-selection-extracted/tool-evidence/selections.nul"
cmp "$fixture/extracted/identity.txt" "${retained_selections[0]}/identity.txt"
tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/failed-selection-paths"
if grep -Eq '(^|/)not-selected$' "$fixture/failed-selection-paths"; then exit 1; fi
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/failed-selection-evidence.tar.gz"

# Exercise actual store admission, including failure before any tools exist.
# A redirected .tools root must not cause collection to traverse outside links.
ln -s "$SHARED_TOOLING_STORE" "$fixture/redirected-store"
for layout in physical no-tools redirected; do
    if [[ "$layout" != physical ]]; then
        mv "$GITHUB_WORKSPACE/.tools" "$fixture/retained-tools"
        if [[ "$layout" == redirected ]]; then
            ln -s "$fixture/retained-tools" "$GITHUB_WORKSPACE/.tools"
        fi
    fi
    collection_status=0
    SHARED_TOOLING_STORE="$fixture/redirected-store" \
        "$BASH" "$root/scripts/ci/collect-failure-evidence.sh" \
        > "$fixture/rejected-store-$layout.log" 2>&1 || collection_status=$?
    test "$collection_status" -ne 0
    grep -Fq 'tool store root may not be a symlink' "$fixture/rejected-store-$layout.log"
    mkdir "$fixture/rejected-store-$layout-extracted"
    tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/rejected-store-$layout-extracted"
    extracted="$fixture/rejected-store-$layout-extracted"
    cmp "$GITHUB_WORKSPACE/target/validation-failures/raw.log" "$extracted/target/validation-failures/raw.log"
    cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" "$extracted/validation-failures/raw.log"
    printf '%s\n' "$collection_status" > "$fixture/expected-rejection-status"
    cmp "$fixture/expected-rejection-status" "$extracted/tool-evidence/selection-status"
    grep -Fq 'tool store root may not be a symlink' "$extracted/tool-evidence/selection.log"
    if [[ "$layout" == physical ]]; then
        cmp "$rust_slot/install.log" \
            "$extracted/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/install.log"
        cmp "$GITHUB_WORKSPACE/.tools/testkit-server/.setup-v1-failed/failure.txt" \
            "$extracted/.tools/testkit-server/.setup-v1-failed/failure.txt"
        test -L "$extracted/.tools/rust/$shared_rust_reference"
        test "$(readlink "$extracted/.tools/rust/$shared_rust_reference")" = "$shared_rust"
        perl -e 'my @a=stat $ARGV[0]; my @b=stat $ARGV[1];
            @a && @b && ($a[2]&07777)==($b[2]&07777) or die "rejection archive lost executable mode\n";' \
            "$rust_slot/bin/cargo-sort" \
            "$extracted/.tools/rust/cargo-sort-2.1.4-bin-cargo-sort-release/installed/bin/cargo-sort"
        for kind in host ic; do
            selected="$(readlink "$GITHUB_WORKSPACE/.tools/$kind")"
            test -L "$extracted/.tools/$kind"
            test -L "$extracted/.tools/$selected"
            test "$(readlink "$extracted/.tools/$selected")" = "$(readlink "$GITHUB_WORKSPACE/.tools/$selected")"
        done
    else
        test ! -e "$extracted/.tools"
        if [[ "$layout" == redirected ]]; then rm "$GITHUB_WORKSPACE/.tools"; fi
        mv "$fixture/retained-tools" "$GITHUB_WORKSPACE/.tools"
    fi
    tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/rejected-store-$layout-paths"
    if grep -Eq '/not-selected|^\./(host|ic|rust)-[[:xdigit:]]{64}/' "$fixture/rejected-store-$layout-paths"; then exit 1; fi
    mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/rejected-store-$layout-evidence.tar.gz"
done

# Collection errors must remain failures and leave the diagnostic inputs intact.
mkdir "$fixture/bin"
printf '%s\n' '#!/bin/sh' 'printf "partial archive bytes"' \
    'echo "injected archive failure" >&2' 'exit 23' > "$fixture/bin/tar"
chmod +x "$fixture/bin/tar"
collection_status=0
PATH="$fixture/bin:$PATH" bash "$root/scripts/ci/collect-failure-evidence.sh" \
    > "$fixture/failed-collection.log" 2>&1 || collection_status=$?
test "$collection_status" -eq 1
grep -Fq 'injected archive failure' "$fixture/failed-collection.log"
if grep -Fq 'CI failure evidence measurements:' "$fixture/failed-collection.log"; then
    echo 'error: failed collection reported completed archive measurements' >&2
    exit 1
fi
printf 'partial archive bytes' > "$fixture/expected-partial"
cmp "$fixture/expected-partial" "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz"
# A retained partial output must not be overwritten by a subsequent attempt.
collection_status=0
bash "$root/scripts/ci/collect-failure-evidence.sh" \
    > "$fixture/occupied-collection.log" 2>&1 || collection_status=$?
test "$collection_status" -eq 1
grep -Fq 'evidence output already exists' "$fixture/occupied-collection.log"
cmp "$fixture/expected-partial" "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz"
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/partial-evidence.tar.gz"
cmp "$scratch/scenario.log" "$fixture/extracted/ic-timers-fixtures/failed case/scenario.log"
cmp "$GITHUB_WORKSPACE/.git/release-state/validation-failures/raw.log" \
    "$fixture/extracted/validation-failures/raw.log"

# Early failures can have no fixture, logger or installer payload yet.
rm -rf "$RUNNER_TEMP/ic-timers-fixtures" "$GITHUB_WORKSPACE/target" \
    "$GITHUB_WORKSPACE/.tools" "$GITHUB_WORKSPACE/.git/release-state"
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/empty-collection.log" 2>&1
tar -tzf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" > "$fixture/empty-entries"
printf './identity.txt\n' > "$fixture/expected-empty-entries"
cmp "$fixture/expected-empty-entries" "$fixture/empty-entries"
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/empty-evidence.tar.gz"
if bash "$root/scripts/ci/collect-failure-evidence.sh" unexpected > "$fixture/invalid-input.log" 2>&1; then exit 1; fi
if env -u RUNNER_TEMP bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/missing-input.log" 2>&1; then exit 1; fi
# Drive the actual local fixtures to an early tool failure. A retained input
# and the original status must survive their EXIT handlers; no builds or real
# Git writes are selected by these substitute tools.
mkdir -p "$fixture/retention-bin"
RETENTION_REAL_GIT="$(command -v git)"
export RETENTION_REAL_GIT
cat > "$fixture/retention-bin/tool-stub" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${0##*/}" == git && "${1:-}" != init && "${1:-}" != clone ]]; then
    exec "$RETENTION_REAL_GIT" "$@"
fi
for path in "$TMPDIR"/*; do
    [[ -d "$path" ]] || continue
    printf 'retained injected input\n' > "$path/injected-input"
done
echo 'injected fixture tool failure' >&2
exit 23
STUB
chmod +x "$fixture/retention-bin/tool-stub"
for tool in git cargo cp; do ln -s tool-stub "$fixture/retention-bin/$tool"; done
retained_inputs=()
for script in scripts/release/test-committed-release.sh scripts/release/test-release-index.sh \
    scripts/release/test-lockfiles.sh scripts/ci/test-repository-checks.sh; do
    name="${script##*/}"
    scratch="$RUNNER_TEMP/ic-timers-fixtures/retention cases/$name"
    mkdir -p "$scratch"
    status=0
    TMPDIR="$scratch" PATH="$fixture/retention-bin:$PATH" bash "$root/$script" \
        > "$fixture/$name-retention.log" 2>&1 || status=$?
    test "$status" -eq 23
    retained=("$scratch"/*)
    test "${#retained[@]}" -eq 1
    test -d "${retained[0]}"
    grep -Fxq 'retained injected input' "${retained[0]}/injected-input"
    retained_inputs+=("${retained[0]}/injected-input")
    grep -Fq 'fixture retained:' "$fixture/$name-retention.log"
    grep -Fq 'injected fixture tool failure' "$fixture/$name-retention.log"
done
# Archive the actual retained inputs through the unchanged collector.
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/retained-collection.log" 2>&1
mkdir "$fixture/retained-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/retained-extracted"
for input in "${retained_inputs[@]}"; do
    cmp "$input" "$fixture/retained-extracted/${input#"$RUNNER_TEMP/"}"
done
mv "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" "$fixture/retained-evidence.tar.gz"

# Exercise the manual qualification driver in the isolated checkout, with fake
# CI identity, a substitute download and a failing scratch Make recipe. This
# proves local retained bytes/status; it does not dispatch or qualify uploads.
mkdir -p "$GITHUB_WORKSPACE/scripts/ci" "$GITHUB_WORKSPACE/scripts/dev" "$GITHUB_WORKSPACE/ci"
for script in qualify-failure-evidence qualify-hosted-failure run-validation-targets check-make-execution verify-file-checksum; do
    cp "$root/scripts/ci/$script.sh" "$GITHUB_WORKSPACE/scripts/ci/"
done
cp "$root/scripts/dev/install-host-tools.sh" "$root/scripts/dev/tool-store.sh" "$GITHUB_WORKSPACE/scripts/dev/"
cp "$root/ci/tool-versions.env" "$GITHUB_WORKSPACE/ci/"
status=0
GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
    bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" unknown \
    > "$fixture/invalid-qualification.log" 2>&1 || status=$?
test "$status" -eq 2
status=0
GITHUB_ACTIONS=true GITHUB_EVENT_NAME=push \
    bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" early \
    > "$fixture/nonmanual-qualification.log" 2>&1 || status=$?
test "$status" -eq 2
for stage in early late; do
    status=0
    GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
        bash "$GITHUB_WORKSPACE/scripts/ci/qualify-failure-evidence.sh" "$stage" \
        > "$fixture/$stage-qualification.log" 2>&1 || status=$?
    expected=22
    if [[ "$stage" == late ]]; then expected=2; fi
    if [[ "$status" != "$expected" ]]; then
        cat "$fixture/$stage-qualification.log" >&2
        exit 1
    fi
    scenarios=("$RUNNER_TEMP/ic-timers-fixtures/hosted-${stage}."*)
    test "${#scenarios[@]}" -eq 1
    scenario="${scenarios[0]}"
    printf 'stage=%s\nstatus=%s\nexpected=%s\n' "$stage" "$expected" "$expected" > "$fixture/expected-status"
    cmp "$fixture/expected-status" "$scenario/status.txt"
    cmp "$scenario/before.txt" "$scenario/after.txt"
    if [[ "$stage" == early ]]; then
        candidates=("$scenario/consumer/.tools/host-set."*)
        test "${#candidates[@]}" -eq 1
        grep -Fxq 'controlled rejected download bytes' "${candidates[0]}/bin/jq"
    else
        grep -Fxq 'error: controlled late validation failure' \
            "$GITHUB_WORKSPACE/target/validation-failures/latest-combined.log"
    fi
done
bash "$root/scripts/ci/collect-failure-evidence.sh" > "$fixture/qualification-collection.log" 2>&1
mkdir "$fixture/qualification-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/qualification-extracted"
for scenario in "$RUNNER_TEMP/ic-timers-fixtures"/hosted-*; do
    for file in before.txt after.txt scenario.log status.txt; do
        cmp "$scenario/$file" "$fixture/qualification-extracted/${scenario#"$RUNNER_TEMP/"}/$file"
    done
    if [[ "${scenario##*/}" == hosted-early.* ]]; then
        candidates=("$scenario/consumer/.tools/host-set."*)
        candidate="$(cd -P "${candidates[0]}" && printf '%s' "$PWD")"
        cmp "$candidate/bin/jq" "$fixture/qualification-extracted/${candidate#"$RUNNER_TEMP/"}/bin/jq"
    fi
done
# Bootstrap regression: invoke copied consumer entrypoints by relative name
# with inherited CDPATH, spaces and physical checkout/workspace/temp aliases.
# CR/LF roots are rejected by shared storage admission, not positive callers.
# Select the canonical installer/logger and their declared companions.
copied="$fixture/copied checkout"
export RUNNER_TEMP="$fixture/copied runner"
mkdir -p "$copied/scripts/ci" "$copied/scripts/dev" "$copied/ci" "$RUNNER_TEMP"
git init -q "$copied"
mkdir -p "$copied/.git/objects/info"
printf '%s\n' "$objects" > "$copied/.git/objects/info/alternates"
git -C "$copied" update-ref HEAD "$(git -C "$root" rev-parse HEAD)"
for script in collect-failure-evidence collect-tool-evidence qualify-failure-evidence qualify-hosted-failure archive-evidence \
    select-tool-evidence run-validation-targets check-make-execution verify-file-checksum \
    verify-evidence-checksums; do
    cp "$root/scripts/ci/$script.sh" "$copied/scripts/ci/"
done
cp "$root/ci/tool-versions.env" "$copied/ci/"
cp "$root/ci/ic-tools-qualified.tsv" "$copied/ci/"
cp "$root/ci/ic-tools.tsv" "$copied/ci/"
cp "$root/scripts/ci/ic-tool-pins.awk" "$copied/scripts/ci/"
cp "$root/scripts/dev/install-host-tools.sh" "$root/scripts/dev/install-ic-tools.sh" \
    "$root/scripts/dev/tool-store.sh" "$copied/scripts/dev/"
# The selected workspace alias must compare equal to the physical script root.
ln -s "$copied" "$fixture/selected-checkout"
export GITHUB_WORKSPACE="$fixture/selected-checkout" CDPATH="$copied"
status=0
(
    cd -P "$copied"
    GITHUB_WORKSPACE="$fixture" GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
        bash scripts/ci/qualify-failure-evidence.sh late
) > "$fixture/foreign-checkout.log" 2>&1 || status=$?
test "$status" -eq 2
grep -Fq 'qualification must use the selected CI checkout' "$fixture/foreign-checkout.log"
for stage in early late; do
    status=0
    (
        cd -P "$copied"
        GITHUB_ACTIONS=true GITHUB_EVENT_NAME=workflow_dispatch \
            bash scripts/ci/qualify-failure-evidence.sh "$stage"
    ) > "$fixture/$stage-path-qualification.log" 2>&1 || status=$?
    expected=22
    [[ "$stage" != late ]] || expected=2
    if [[ "$status" != "$expected" ]]; then
        cat "$fixture/$stage-path-qualification.log" >&2
        exit 1
    fi
    scenarios=("$RUNNER_TEMP/ic-timers-fixtures/hosted-${stage}."*)
    test "${#scenarios[@]}" -eq 1
    scenario="${scenarios[0]}"
    printf 'stage=%s\nstatus=%s\nexpected=%s\n' "$stage" "$expected" "$expected" > "$fixture/expected-path-status"
    cmp "$fixture/expected-path-status" "$scenario/status.txt"
    if [[ "$stage" == early ]]; then
        candidates=("$scenario/consumer/.tools/host-set."*)
        test "${#candidates[@]}" -eq 1
        grep -Fxq 'controlled rejected download bytes' "${candidates[0]}/bin/jq"
    else
        grep -Fxq 'error: controlled late validation failure' \
            "$copied/target/validation-failures/latest-combined.log"
    fi
done
(
    cd -P "$copied"
    bash scripts/ci/collect-failure-evidence.sh
) > "$fixture/path-collection.log" 2>&1
mkdir "$fixture/path-extracted"
tar -xzpf "$RUNNER_TEMP/ic-timers-failure-evidence.tar.gz" -C "$fixture/path-extracted"
for scenario in "$RUNNER_TEMP/ic-timers-fixtures"/hosted-*; do
    for file in before.txt after.txt scenario.log status.txt consumer/input.txt; do
        cmp "$scenario/$file" "$fixture/path-extracted/${scenario#"$RUNNER_TEMP/"}/$file"
    done
done
grep -Fxq "checkout_sha=$(git -C "$root" rev-parse HEAD)" "$fixture/path-extracted/identity.txt"
grep -Fxq 'event_sha=fixture-event-sha' "$fixture/path-extracted/identity.txt"
cmp "$copied/target/validation-failures/latest.log" \
    "$fixture/path-extracted/target/validation-failures/latest.log"
fixture_complete=true
echo 'CI failure evidence selection, metadata, modes, empty-input, retention and qualification-driver checks passed'
