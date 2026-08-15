# Recurring code-hygiene audit

Use this narrow audit before a minor release or after a substantial public API
change. It is not a substitute for the recovery/PocketIC evidence in
`SAFETY.md`.

## Mechanical checks

Run:

```text
make ci
make msrv
git diff --check
```

Then inspect:

```text
rg "unwrap\(|expect\(|panic!|todo!|unimplemented!|TODO|FIXME|HACK" \
  crates/ic-timers/src
rg "^pub |pub struct|pub enum|pub trait|pub fn|pub const" \
  crates/ic-timers/src
rg "ic_cdk_timers|ic-cdk-timers" crates/ic-timers/src Cargo.toml \
  crates/ic-timers/Cargo.toml
cargo tree --workspace --duplicates
cargo package --locked --offline --allow-dirty --list -p ic-timers
```

## Review questions

- Does production code avoid panics for invalid input or recoverable state?
- Is every public value inert data, validated configuration, or documented
  runtime authority, and is that role clear in its name and rustdoc?
- Does the crate root expose only the intended facade, with provider, registry,
  control, and dispatch modules still private?
- Do implementation modules import from the defining module rather than
  depending on accidental crate-root re-exports?
- Are long registry/runtime functions still one atomic transition or binding
  path? Split by responsibility, but do not fragment rollback-sensitive state
  merely to reduce line counts.
- Does every delegated callback capability expire with its exact work attempt,
  rather than inheriting the longer lifetime of a registration claim?
- Can any public constructor bypass a validation or control invariant?
- Does every validation boundary have a negative test?
- Do scheduler starts, work dispatches, work starts, completions,
  unacknowledged attempts, and measurements remain distinct?
- Are counters, totals, deadlines, and generations overflow-safe?
- Does `platform` remain the only direct `ic-cdk-timers` boundary?
- Do README, architecture, status, changelog, and `SAFETY.md` make the same
  implementation and recovery claims?
- Are external GitHub Actions pinned and dependencies still necessary?
- Does the package contain only intended public source and metadata?
- Can an explicit release target only move the package version forward, using
  canonical SemVer components and an exact tag-name check?

Classify findings as mechanical, behavioral, or design. Fix mechanical and
clearly safe behavioral findings in the audit change. Keep recovery semantics,
wire formats, and cross-consumer API choices in design review.
