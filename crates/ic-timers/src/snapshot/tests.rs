use super::*;
use crate::TimerRegistration;
use std::time::Duration;

fn identity(owner: &str, subsystem: &str, name: &str) -> TimerIdentity {
    TimerIdentity::try_new(owner, subsystem, name).expect("fixture identity should be valid")
}

#[derive(Debug, Eq, PartialEq)]
struct CanicProjection<'a> {
    name: &'a str,
    subsystem: &'a str,
    timer_mode: &'static str,
    configured_cadence_ns: Option<u64>,
    latest_delay_ms: Option<u64>,
    scheduling_mode: &'static str,
    registration: &'static str,
    condition: &'static str,
    enabled: bool,
    generation: u64,
    next_due_at_ns: Option<u64>,
    last_outcome: Option<&'static str>,
    last_work_count: u64,
    last_success_at_ns: Option<u64>,
    last_failure_at_ns: Option<u64>,
    consecutive_expected_failures: u64,
    schedules_since_runtime_start: u64,
    arms_since_runtime_start: u64,
    executions_since_runtime_start: u64,
    successes_since_runtime_start: u64,
    expected_failures_since_runtime_start: u64,
    invariant_failures_since_runtime_start: u64,
    stale_callbacks_since_runtime_start: u64,
    completed_since_runtime_start: u64,
    total_instructions: u64,
}

fn project_to_canic(snapshot: &TimerSnapshot) -> CanicProjection<'_> {
    let counters = snapshot.observability.counters();
    let outcomes = snapshot.observability.outcomes();
    let timer_mode = match snapshot.scheduling.configured_policy {
        TimerPolicy::Once => "once",
        TimerPolicy::AfterCompletion { .. } | TimerPolicy::Watchdog { .. } => "interval",
    };
    CanicProjection {
        name: snapshot.identity.name().as_str(),
        subsystem: snapshot.identity.subsystem().as_str(),
        timer_mode,
        configured_cadence_ns: snapshot.scheduling.configured_policy.cadence_ns(),
        latest_delay_ms: snapshot
            .scheduling
            .latest_requested_delay_ns
            .map(|nanoseconds| nanoseconds / 1_000_000),
        scheduling_mode: snapshot.scheduling.current_mode.label(),
        registration: snapshot.state.registration.label(),
        condition: snapshot.state.condition.label(),
        enabled: snapshot.state.enabled,
        generation: snapshot.state.generation,
        next_due_at_ns: snapshot.scheduling.next_deadline_ns,
        last_outcome: match outcomes.last_outcome() {
            Some(TimerLastOutcome::Completed(outcome)) => Some(outcome.label()),
            Some(TimerLastOutcome::Interrupted) | None => None,
        },
        last_work_count: outcomes.last_work_count().unwrap_or_default(),
        last_success_at_ns: outcomes.last_success_at_ns(),
        last_failure_at_ns: outcomes.last_failure_at_ns(),
        consecutive_expected_failures: outcomes.consecutive_expected_failures(),
        schedules_since_runtime_start: counters.requested(),
        arms_since_runtime_start: counters.armed(),
        executions_since_runtime_start: counters.started(),
        successes_since_runtime_start: counters.succeeded().saturating_add(counters.no_work()),
        expected_failures_since_runtime_start: counters.retryable_failure(),
        invariant_failures_since_runtime_start: counters.invariant_failure(),
        stale_callbacks_since_runtime_start: counters.stale(),
        completed_since_runtime_start: counters.completed(),
        total_instructions: snapshot.observability.performance().instructions().total(),
    }
}

#[test]
fn labels_are_bounded_in_utf8_bytes() {
    assert!(TimerLabel::new("a".repeat(MAX_TIMER_LABEL_BYTES)).is_ok());
    assert_eq!(
        TimerLabel::new("a".repeat(MAX_TIMER_LABEL_BYTES + 1)),
        Err(TimerLabelError::TooLong {
            actual_bytes: MAX_TIMER_LABEL_BYTES + 1,
            max_bytes: MAX_TIMER_LABEL_BYTES,
        })
    );

    assert!(TimerLabel::new("é".repeat(MAX_TIMER_LABEL_BYTES / 2)).is_ok());
    assert!(matches!(
        TimerLabel::new("é".repeat(MAX_TIMER_LABEL_BYTES / 2 + 1)),
        Err(TimerLabelError::TooLong { .. })
    ));
}

#[test]
fn labels_reject_ambiguous_operator_text() {
    assert_eq!(TimerLabel::new(""), Err(TimerLabelError::Empty));
    assert_eq!(
        TimerLabel::new(" timer"),
        Err(TimerLabelError::SurroundingWhitespace)
    );
    assert_eq!(
        TimerLabel::new("timer\nname"),
        Err(TimerLabelError::ControlCharacter {
            byte_index: 5,
            character: '\n',
        })
    );
}

#[test]
fn identity_errors_name_the_invalid_component() {
    assert_eq!(
        TimerIdentity::try_new("canic", "", "renewal"),
        Err(TimerIdentityError {
            field: TimerIdentityField::Subsystem,
            source: TimerLabelError::Empty,
        })
    );
}

#[test]
fn identities_order_by_owner_subsystem_and_name() {
    let mut identities = [
        identity("icydb", "recovery", "drive"),
        identity("canic", "cycles", "top_up"),
        identity("canic", "auth", "renewal"),
        identity("canic", "auth", "cleanup"),
    ];
    identities.sort();

    let labels = identities
        .iter()
        .map(|value| {
            (
                value.owner().as_str(),
                value.subsystem().as_str(),
                value.name().as_str(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        vec![
            ("canic", "auth", "cleanup"),
            ("canic", "auth", "renewal"),
            ("canic", "cycles", "top_up"),
            ("icydb", "recovery", "drive"),
        ]
    );
}

#[test]
fn policy_and_directive_modes_are_distinct() {
    let policy = TimerPolicy::Watchdog { cadence_ns: 5_000 };
    assert_eq!(policy.label(), "watchdog");
    assert_eq!(policy.cadence_ns(), Some(5_000));
    assert_eq!(policy.initial_mode(), TimerSchedulingMode::Watchdog);

    assert_eq!(
        TimerDirectiveSnapshot::RetryAfter { delay_ns: 10 }.scheduling_mode(),
        Some(TimerSchedulingMode::Retry)
    );
    assert_eq!(
        TimerDirectiveSnapshot::ContinueImmediately.scheduling_mode(),
        Some(TimerSchedulingMode::Continuation)
    );
    assert_eq!(TimerDirectiveSnapshot::Stop.scheduling_mode(), None);
}

#[test]
fn directives_convert_to_portable_nanoseconds_without_truncation() {
    let directive = crate::TimerDirective::RetryAfter(Duration::from_millis(25));
    let snapshot = TimerDirectiveSnapshot::try_from(directive)
        .expect("25 milliseconds should fit in nanoseconds");
    assert_eq!(
        snapshot,
        TimerDirectiveSnapshot::RetryAfter {
            delay_ns: 25_000_000,
        }
    );
    assert_eq!(crate::TimerDirective::from(snapshot), directive);

    assert_eq!(
        TimerDirectiveSnapshot::try_from(crate::TimerDirective::RecurAfter(Duration::from_secs(
            u64::MAX
        ),)),
        Err(crate::ScheduleError::DelayOutOfRange)
    );
}

#[test]
fn control_registration_has_a_portable_projection() {
    assert_eq!(
        TimerRegistrationStatus::from(TimerRegistration::Unregistered),
        TimerRegistrationStatus::Unregistered
    );
    assert_eq!(
        TimerRegistrationStatus::from(TimerRegistration::Scheduled {
            generation: 2,
            deadline_ns: 50,
        }),
        TimerRegistrationStatus::Scheduled
    );
    assert_eq!(
        TimerRegistrationStatus::from(TimerRegistration::Running { generation: 2 }),
        TimerRegistrationStatus::Running
    );
}

#[test]
fn outcome_transitions_match_canic_failure_streak_semantics() {
    let mut outcomes = TimerOutcomeSnapshot::default();
    outcomes.record_completion(TimerCompletion::retryable_failure(2), 10);
    outcomes.record_completion(TimerCompletion::retryable_failure(1), 20);
    assert_eq!(outcomes.consecutive_expected_failures(), 2);
    assert_eq!(outcomes.last_failure_at_ns(), Some(20));
    assert_eq!(outcomes.last_work_count(), Some(1));

    outcomes.record_interruption(25);
    assert_eq!(outcomes.last_outcome(), Some(TimerLastOutcome::Interrupted));
    assert_eq!(outcomes.last_interrupted_at_ns(), Some(25));
    assert_eq!(outcomes.last_work_count(), None);
    assert_eq!(outcomes.consecutive_expected_failures(), 2);

    outcomes.record_completion(TimerCompletion::no_work(), 30);
    assert_eq!(outcomes.consecutive_expected_failures(), 0);
    assert_eq!(outcomes.last_success_at_ns(), Some(30));
    assert_eq!(outcomes.last_work_count(), Some(0));

    outcomes.record_completion(TimerCompletion::retryable_failure(0), 40);
    outcomes.record_completion(TimerCompletion::invariant_failure(0), 50);
    assert_eq!(outcomes.consecutive_expected_failures(), 0);
    assert_eq!(outcomes.last_failure_at_ns(), Some(50));
}

#[test]
fn canonical_snapshot_exposes_functional_failure_state_directly() {
    let mut snapshot = TimerSnapshot::new(
        identity("canic", "cycles", "top_up"),
        TimerPolicy::Once,
        TimerEpoch {
            id: 1,
            started_at_ns: 10,
        },
    );
    snapshot
        .observability
        .record_completion(TimerCompletion::retryable_failure(0), 20, None);

    assert_eq!(snapshot.consecutive_expected_failures(), 1);
}

#[test]
fn requests_arms_starts_and_completions_remain_separate() {
    let mut counters = TimerCounters::default();
    counters.record_request();
    counters.record_request();
    counters.record_coalesced();
    counters.record_arm();
    counters.record_start();

    assert_eq!(counters.requested(), 2);
    assert_eq!(counters.coalesced(), 1);
    assert_eq!(counters.armed(), 1);
    assert_eq!(counters.started(), 1);
    assert_eq!(counters.completed(), 0);

    counters.record_completion(TimerCompletionOutcome::NoWork);
    assert_eq!(counters.completed(), 1);
    assert_eq!(counters.no_work(), 1);
    assert!(counters.completion_partition_is_valid());
}

#[test]
fn missing_end_measurement_does_not_create_a_zero_sample() {
    let epoch = TimerEpoch {
        id: 7,
        started_at_ns: 100,
    };
    let mut observations = TimerObservabilitySnapshot::new(epoch);
    observations.record_start();
    observations.record_completion(TimerCompletion::success(3), 150, None);

    assert_eq!(observations.counters().started(), 1);
    assert_eq!(observations.counters().completed(), 1);
    assert_eq!(observations.performance().instructions().samples(), 0);
    assert_eq!(observations.performance().instructions().latest(), None);
}

#[test]
fn performance_tracks_total_latest_and_maximum() {
    let mut performance = TimerPerformance::default();
    performance.record(TimerMeasurement {
        instructions: 100,
        elapsed_ns: 10,
    });
    performance.record(TimerMeasurement {
        instructions: 75,
        elapsed_ns: 20,
    });

    assert_eq!(performance.instructions().samples(), 2);
    assert_eq!(performance.instructions().total(), 175);
    assert_eq!(performance.instructions().latest(), Some(75));
    assert_eq!(performance.instructions().maximum(), Some(100));
    assert_eq!(performance.elapsed_ns().total(), 30);
    assert_eq!(performance.elapsed_ns().maximum(), Some(20));
}

#[test]
fn beginning_an_epoch_resets_all_observation_state() {
    let mut observations = TimerObservabilitySnapshot::new(TimerEpoch {
        id: 1,
        started_at_ns: 100,
    });
    observations.record_request();
    observations.record_start();
    observations.record_completion(
        TimerCompletion::retryable_failure(0),
        110,
        Some(TimerMeasurement {
            instructions: 50,
            elapsed_ns: 5,
        }),
    );

    let next_epoch = TimerEpoch {
        id: 2,
        started_at_ns: 200,
    };
    observations.begin_epoch(next_epoch);

    assert_eq!(observations.epoch(), next_epoch);
    assert_eq!(observations.counters(), TimerCounters::default());
    assert_eq!(observations.outcomes(), TimerOutcomeSnapshot::default());
    assert_eq!(observations.performance(), TimerPerformance::default());
    assert_eq!(observations.consecutive_expected_failures(), 0);
}

#[test]
fn canic_operator_surface_projects_without_parallel_metrics() {
    let mut snapshot = TimerSnapshot::new(
        identity("canic", "auth", "renewal"),
        TimerPolicy::AfterCompletion { cadence_ns: 1_000 },
        TimerEpoch {
            id: 4,
            started_at_ns: 100,
        },
    );
    snapshot.scheduling.latest_requested_delay_ns = Some(2_000_000_000);
    snapshot.scheduling.latest_armed_delay_ns = Some(2_000_000_000);
    snapshot.scheduling.next_deadline_ns = Some(500);
    snapshot.state = TimerStateSnapshot {
        enabled: true,
        registration: TimerRegistrationStatus::Scheduled,
        condition: TimerProcessCondition::Active,
        generation: 3,
        in_flight: false,
    };
    snapshot.observability.record_request();
    snapshot.observability.record_arm();
    snapshot.observability.record_start();
    snapshot.observability.record_completion(
        TimerCompletion::success(2),
        300,
        Some(TimerMeasurement {
            instructions: 90,
            elapsed_ns: 8,
        }),
    );

    assert_eq!(
        project_to_canic(&snapshot),
        CanicProjection {
            name: "renewal",
            subsystem: "auth",
            timer_mode: "interval",
            configured_cadence_ns: Some(1_000),
            latest_delay_ms: Some(2_000),
            scheduling_mode: "after_completion",
            registration: "scheduled",
            condition: "active",
            enabled: true,
            generation: 3,
            next_due_at_ns: Some(500),
            last_outcome: Some("success"),
            last_work_count: 2,
            last_success_at_ns: Some(300),
            last_failure_at_ns: None,
            consecutive_expected_failures: 0,
            schedules_since_runtime_start: 1,
            arms_since_runtime_start: 1,
            executions_since_runtime_start: 1,
            successes_since_runtime_start: 1,
            expected_failures_since_runtime_start: 0,
            invariant_failures_since_runtime_start: 0,
            stale_callbacks_since_runtime_start: 0,
            completed_since_runtime_start: 1,
            total_instructions: 90,
        }
    );
}
