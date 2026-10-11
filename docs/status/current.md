![IC Timers — Schedules and tracks background work](https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-timers/ic-timers-readme-header.svg)

# Current status

Last updated: 2026-10-11

This handoff links [release evidence](../releasing.md),
[Shared adoption](../shared-tooling.md), [safety](../../SAFETY.md) and
[the archive](archive-2026-10-10.md).

## Released source

Released **0.17.14** is `cb86d87ce67f7aac1b127bdfaa2c6f8a7f68c2a0`.
It selects Metrics **0.6.4**, Testkit **0.34.0**, all four Host **0.14.1**,
PocketIC **16.1.0** and Shared **0.6.3**
`4fd979d13c32adb3cb2f51f213ebfc4f98339b3d` through **56/33/11** exports.
The release delivers the early verifier correction; it adds no native Python
prerequisite, independent tool-store manager or direct Host dependency.
[Full CI](https://github.com/dragginzgame/ic-timers/actions/runs/38153029865)
passes Linux/MSRV; Intel is in progress and ARM queued at inspection.
Previous 0.17.13 full CI was cancelled by the newer push. Historical observations
remain bound to their actual sources and graphs.

## Prepared 0.18.0

The maintainer authorized atomic initial registration and continuing #44.
All three direct registration functions now take `initial: Option<TimerSchedule>`
before the callback. Scheduled creation returns authority after provider binding;
failed creation retires its exact claim and handles before releasing captures.
`None` deliberately declares inactive retained authority; transient `None`
returns `InitialScheduleRequired`. Cadence is independent of the first deadline.
Later control, callback policies and lifecycle reconciliation retain their own
existing boundaries. There is no old-signature forwarder or persisted intent.

The [construction decision](../design/0.18-atomic-registration.md) owns the
contract and complete qualification scope. Runtime/size probes, native fixtures,
positive/negative documentation and the undated **0.18.0** changelog are updated.
`make fmt`, diff checks and locked offline no-dependency metadata pass. Tests,
builds, lint gates, PocketIC and Wasm/instruction measurements are not run locally;
this is prepared source, not qualified release readiness. Cargo package versions
remain 0.17.14. A separate worktree lock update selects `syn` 3.0.8; it is preserved
and must not be qualified with older released-source evidence.

Canic's lifecycle create/arm rollback branch is the concrete consumer benefit.
Its separate custody rollback remains necessary. No sibling files or pins were
changed. The [owner feedback](https://github.com/dragginzgame/canic/issues/33#issuecomment-6111101911)
and [Canic mapping](../adoption/canic.md#operation-mapping) identify the coordinated
consumer adoption after Timers publishes and qualifies this minor cut.

## Remaining issue and authorization

[#46](https://github.com/dragginzgame/ic-timers/issues/46) is closed from actual
released Linux execution; it does not establish IC persistence or rollback.
[#44](https://github.com/dragginzgame/ic-timers/issues/44) is open for complete
native and controlled early/late artifact acceptance. Its snapshots, caller
adoption and 0.17.14 correction are delivered.

The explicitly approved frozen `v0.17.13` early and late stages are dispatched
sequentially. [Early 38150377457](https://github.com/dragginzgame/ic-timers/actions/runs/38150377457)
is complete: all original producers reach exit 22 and upload, but all three
frozen download verifiers reject the obsolete candidate path. The released
0.17.14 verifier accepts the exact downloaded Linux/ARM/Intel archives locally,
with frozen source identities and unchanged caller pins. That replay does not
turn the frozen hosted verifier green.

After early completed, [late 38154578484](https://github.com/dragginzgame/ic-timers/actions/runs/38154578484)
was dispatched at the same annotated ref, verified to resolve to
`e6ac645aac1b185bfaa3b146c2b8e1dd38b3299f`. Linux is in progress; both native
producers are queued. Do not repeat the dispatch or close #44 on pending status.
Archives, original failure logs, replay outputs, digests and dispatch records
remain at `/tmp/ic-timers-01713-qualification.n07_3uur/`.
[The evidence owner](../releasing.md#01714-delivery-and-frozen-failure-qualification-follow-up)
records these exact observations.

A separate asynchronous approval request asks to run corrected-source early then
late at frozen `v0.17.14` after the ongoing late run completes. No answer has
arrived yet; elapsed time is not authorization. Preserve the original approved
ref and failed observations. Carry any later explicit approval forward without
asking again, and record its exact source before dispatch. Neither this product
batch nor issue continuation authorizes arbitrary tests/builds/lint/setup.

Only inspection, syntax/diff, formatting and cheap metadata checks may run
locally. Staging, commits, package versions, tags, pushes and release execution
remain user-owned. Qualifier artifact inspection is covered by the explicit
stage authorization; native substitutes remain separate from IC recovery.
