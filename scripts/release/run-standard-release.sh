#!/usr/bin/env bash
set -euo pipefail

# Retire only an unprepared attempt. The pinned runner still owns every phase
# and all Git effects; prepared releases require its exact-version recovery.
[[ $# == 3 ]] || { echo 'usage: run-standard-release.sh patch|minor|major REMOTE BRANCH' >&2; exit 2; }
kind="$1"
case "$kind" in patch|minor|major) ;; *) exit 2 ;; esac
fail() { echo "release refused: $1" >&2; exit 1; }
cd "$(git rev-parse --show-toplevel)"
previous="$(bash scripts/release/workspace-version.sh)"
candidate="$(bash scripts/ci/next-release-version.sh "$previous" "$kind")"
state_root="$(git rev-parse --git-path release-state)"
[[ ! -L "$state_root" ]] || fail 'release state directory is symlinked'
plan="$state_root/$candidate.plan"

if [[ -e "$plan" || -L "$plan" ]]; then
    mkdir "$state_root/lock" 2>/dev/null || fail 'release lock is occupied; inspect its owner before clearing a stale lock'
    trap 'rm -f "$state_root/lock/owner"; rmdir "$state_root/lock"' EXIT
    trap 'exit 130' INT
    trap 'exit 143' TERM
    printf '%s\n' "$$" > "$state_root/lock/owner"
    [[ -f "$plan" && ! -L "$plan" ]] || fail 'release plan is not a regular file'
    {
        IFS= read -r schema
        IFS= read -r planned_kind
        IFS= read -r planned_previous
        IFS= read -r planned_candidate
        IFS= read -r release_date
        IFS= read -r source
        IFS= read -r planned_remote
        IFS= read -r planned_branch
        IFS= read -r remote_identity
        IFS= read -r index_tree
        IFS= read -r phase
        extra=""
        if IFS= read -r extra || [[ -n "$extra" ]]; then fail 'release plan has extra records'; fi
    } < "$plan"
    [[ "$schema" == release-plan-1 && "$planned_kind" == "$kind" &&
        "$planned_previous" == "$previous" && "$planned_candidate" == "$candidate" ]] || fail 'release plan identity is invalid'
    [[ "$source" =~ ^[0-9a-f]{40,64}$ && "$remote_identity" =~ ^[0-9a-f]{40,64}$ &&
        "$release_date" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ &&
        -n "$planned_remote" && -n "$planned_branch" ]] || fail 'release plan metadata is invalid'
    case "$phase" in
        preflight|validate) ;;
        *) fail "release $candidate reached preparation; use make release-resume VERSION=$candidate" ;;
    esac
    [[ -z "$index_tree" && ! -e "$plan.files" && ! -L "$plan.files" ]] || fail 'unprepared release has staged-file records'
    [[ "$(bash scripts/release/workspace-version.sh)" == "$previous" ]] || fail 'workspace version changed during retry admission'
    tags="$(git tag --list "v$candidate")"
    [[ -z "$tags" ]] || fail 'candidate tag already exists'
    archive="$(mktemp "$plan.retry.XXXXXX")"
    mv "$plan" "$archive"
    printf 'Restarting unprepared release %s with full validation; previous attempt retained at %s\n' "$candidate" "$archive"
    rm -f "$state_root/lock/owner"
    rmdir "$state_root/lock"
    trap - EXIT INT TERM
fi

exec bash scripts/ci/run-release.sh "$@"
