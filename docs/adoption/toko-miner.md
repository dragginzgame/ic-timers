# Toko Miner adoption evidence

Status: scoped 0.8.0 composition evidence recorded; inspected 2026-10-02.

## Published runtime and current dependency inspection

The read-only Toko Miner worktree at
`5e7f675dddd5bf2126e5d3068c2fe75776e4a2a9` selects registry `ic-timers`
0.8.0, Canic 0.110.49 and IcyDB 0.262.2. Its inspected lockfile contains
one `ic-timers` package. The downstream owner reports `make timer-check`
passing for Game Shard and Translation; this repository did not rerun it.

The cached registry `ic-timers` 0.8.0 package's `.cargo_vcs_info.json`
identifies commit `fba369df1a31b2c37ce31f4f775f816a7c9de1f9`, matching the
local release tag. The registry lock checksum is
`c1474fd7c9bcc237404173c2689c4175e61ef2c9670bccf539ca10ccd193d132`.
These artifacts corroborate publication of 0.8.0. The targeted 0.8.1 tooling,
documentation and lint-expectation changes remain unpublished.

Inspection provenance: Toko Miner's `docs/upstream/ic-timers.md` and the
2026-10-02 local ic-timers audit in `docs/upstream/scan-log.md`. The inspected
Cargo.lock SHA-256 is
`d49922b78bcf5959bd209b76d8db860d5d1b8b17d12c7898789f3c61b520bc72`.
Dirty downstream state was retained; this is not a deployed-artifact identity.

## Retained composition receipt — 2026-09-20

The downstream owner's
[Canic 0.110.33 adoption receipt](https://github.com/dragginzgame/toko-miner/blob/5e7f675dddd5bf2126e5d3068c2fe75776e4a2a9/docs/upstream/artifacts/canic-0.110.33-adoption-2026-09-20.json)
identifies application HEAD `ada2c2c634b98692c5b70333e718e7ef9b1f8826`,
Canic 0.110.33 at `914a0507483874506538f5b25b4c6ebce22bc389` and IcyDB
0.261.0 at `c1a5028eebed7b2f04cce00d49a4ea6b9c58512c`.

Every application role resolves one `ic-timers` 0.8.0 package. Two managed
scenarios record shared gameplay timer inventory, two idle windows without
checkpoint callbacks, wake on accepted work, same-release state and timer
recovery, and timer registration-identity/saturation metadata. The old
lifecycle-composition blocker is resolved within that recorded scope.

The receipt qualifies a frozen snapshot that excluded concurrent gameplay
edits. It also records unrelated full-CI, browser and lint failures. It does
not establish current staging deployment, live cost savings or qualification
of all later Canic/IcyDB/application combinations. The receipt SHA-256 is
`2328efc4b1a4f351391618ccfbc2d35e0960537bb562cadaced24c084f2c08ea`.
This repository inspected the owner-supplied evidence without rerunning its
managed scenarios or editing any sibling repository.

## Remaining application work

- The checkpoint timer still uses an auxiliary `OnceRegistration` to wake its
  Watchdog. The released exact-deadline API supports removing that hand-off;
  Toko Miner owns the scheduling change and its managed proof.
- Game Shard's diagnostic `TimerMetric` omits registration sequence. The
  released Canic counter projection already carries epoch and sequence;
  `TimerSnapshot::registration_id()` supplies the canonical identity.
  Callback generation alone cannot qualify counter subtraction.
- Current deployment, complete transfer coverage and controlled live cost
  comparisons remain unmeasured here. No scheduler defect or cycle saving is
  established by this review.
