use super::*;
use crate::{
    TimerCadence, TimerDirective, TimerRunResult, TimerSchedule,
    registry::{CallbackAcceptance, RegistryEffect, TimerRegistry},
};
use std::time::Duration;

fn identity(owner: &str, subsystem: &str, name: &str) -> TimerIdentity {
    TimerIdentity::try_new(owner, subsystem, name).expect("fixture identity should be valid")
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
fn identities_validate_components_and_order_deterministically() {
    assert_eq!(
        TimerIdentity::try_new("canic", "", "renewal"),
        Err(TimerIdentityError {
            field: TimerIdentityField::Subsystem,
            source: TimerLabelError::Empty,
        })
    );

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
fn policies_and_directives_have_one_cadence_owner() {
    let cadence = TimerCadence::from_nanos(5_000).expect("fixture cadence should be valid");
    let policy = TimerPolicy::Watchdog { cadence };
    assert_eq!(policy.label(), "watchdog");
    assert_eq!(policy.cadence_ns(), Some(5_000));

    assert_eq!(
        TimerDirectiveSnapshot::RetryAfter { delay_ns: 10 }.scheduling_mode(),
        Some(TimerSchedulingMode::Retry)
    );
    assert_eq!(
        TimerDirectiveSnapshot::ContinueImmediately.scheduling_mode(),
        Some(TimerSchedulingMode::Continuation)
    );
    assert_eq!(
        TimerDirectiveSnapshot::RecurAfterCompletion.scheduling_mode(),
        Some(TimerSchedulingMode::AfterCompletion)
    );
    assert_eq!(TimerDirectiveSnapshot::Stop.scheduling_mode(), None);

    let directive = TimerDirective::RetryAfter(Duration::from_millis(25));
    let snapshot = TimerDirectiveSnapshot::try_from(directive)
        .expect("25 milliseconds should fit in nanoseconds");
    assert_eq!(
        snapshot,
        TimerDirectiveSnapshot::RetryAfter {
            delay_ns: 25_000_000,
        }
    );
    assert_eq!(TimerDirective::from(snapshot), directive);
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
    last_failure_at_ns: Option<u64>,
    consecutive_expected_failures: u64,
    schedules_since_runtime_start: u64,
    arms_since_runtime_start: u64,
    executions_since_runtime_start: u64,
    completed_since_runtime_start: u64,
    expected_failures_since_runtime_start: u64,
    total_instructions: u64,
}

fn project_to_canic(snapshot: &TimerSnapshot) -> CanicProjection<'_> {
    let observations = snapshot.observability();
    let counters = observations.counters();
    let outcomes = observations.outcomes();
    CanicProjection {
        name: snapshot.identity().name().as_str(),
        subsystem: snapshot.identity().subsystem().as_str(),
        timer_mode: match snapshot.policy() {
            TimerPolicy::Once => "once",
            TimerPolicy::AfterCompletion { .. } | TimerPolicy::Watchdog { .. } => "interval",
        },
        configured_cadence_ns: snapshot.policy().cadence_ns(),
        latest_delay_ms: snapshot
            .latest_requested_delay_ns()
            .map(|nanoseconds| nanoseconds / 1_000_000),
        scheduling_mode: snapshot.scheduling_mode().label(),
        registration: snapshot.registration_status().label(),
        condition: snapshot.process_condition().label(),
        enabled: snapshot.process_condition() != TimerProcessCondition::Disabled,
        generation: snapshot.generation().unwrap_or_default(),
        next_due_at_ns: snapshot.next_deadline_ns(),
        last_outcome: match outcomes.last_outcome() {
            Some(TimerLastOutcome::Completed(outcome)) => Some(outcome.label()),
            Some(TimerLastOutcome::Unacknowledged) | None => None,
        },
        last_work_count: outcomes.last_work_count().unwrap_or_default(),
        last_failure_at_ns: outcomes.last_failure_at_ns(),
        consecutive_expected_failures: outcomes.consecutive_expected_failures(),
        schedules_since_runtime_start: counters.schedule_requests(),
        arms_since_runtime_start: counters.provider_arms(),
        executions_since_runtime_start: counters.work_started(),
        completed_since_runtime_start: counters.work_completed(),
        expected_failures_since_runtime_start: counters.retryable_failure(),
        total_instructions: observations.performance().work_instructions().total(),
    }
}

#[test]
fn canonical_registry_snapshot_projects_canic_surface_without_parallel_metrics() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(4, 100));
    let timer = identity("canic", "auth", "renewal");
    let cadence = TimerCadence::from_nanos(1_000_000_000).expect("fixture cadence should be valid");
    let claim = registry
        .register_after_completion(timer.clone(), cadence, DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let initial_transition = registry
        .ensure_recurring(&claim, 100)
        .expect("ensure should succeed");
    registry
        .confirm_effect_applied(initial_transition.effect())
        .expect("fixture provider effect should apply");
    let token = match initial_transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token,
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert_eq!(
        registry.begin_ordinary(&token),
        CallbackAcceptance::Accepted
    );
    let completion_transition = registry
        .complete_ordinary(
            &token,
            200,
            TimerRunResult::new(
                TimerCompletion::retryable_failure(2),
                TimerDirective::RetryAfter(Duration::from_secs(2)),
            ),
        )
        .expect("completion should succeed");
    registry
        .confirm_effect_applied(completion_transition.effect())
        .expect("fixture provider effect should apply");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(
        project_to_canic(&snapshot),
        CanicProjection {
            name: "renewal",
            subsystem: "auth",
            timer_mode: "interval",
            configured_cadence_ns: Some(1_000_000_000),
            latest_delay_ms: Some(2_000),
            scheduling_mode: "retry",
            registration: "scheduled",
            condition: "retrying",
            enabled: true,
            generation: 2,
            next_due_at_ns: Some(2_000_000_200),
            last_outcome: Some("retryable_failure"),
            last_work_count: 2,
            last_failure_at_ns: Some(200),
            consecutive_expected_failures: 1,
            schedules_since_runtime_start: 1,
            arms_since_runtime_start: 2,
            executions_since_runtime_start: 1,
            completed_since_runtime_start: 1,
            expected_failures_since_runtime_start: 1,
            total_instructions: 0,
        }
    );
}

#[test]
fn retryable_terminal_completion_projects_failed_canic_condition() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(5, 100));
    let timer = identity("canic", "cycles", "topup");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let transition = registry
        .ensure_once(&claim, 100, TimerSchedule::At(101))
        .expect("ensure should succeed");
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
    let token = match transition.into_effect() {
        RegistryEffect::ArmWakeup { token, .. } => token,
        effect => panic!("expected arm effect, got {effect:?}"),
    };
    assert_eq!(
        registry.begin_ordinary(&token),
        CallbackAcceptance::Accepted
    );
    registry
        .complete_ordinary(
            &token,
            102,
            TimerRunResult::new(TimerCompletion::retryable_failure(0), TimerDirective::Stop),
        )
        .expect("terminal retryable completion should succeed");

    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.process_condition(), TimerProcessCondition::Failed);
    assert_eq!(project_to_canic(&snapshot).condition, "failed");
}

#[test]
fn absolute_past_deadline_remains_coherent_and_arms_with_zero_delay() {
    let mut registry = TimerRegistry::new(TimerEpoch::new(1, 0));
    let timer = identity("test", "snapshot", "past");
    let claim = registry
        .register_once(timer.clone(), DeclarationLifetime::Retained)
        .expect("registration should succeed");
    let transition = registry
        .ensure_once(&claim, 100, TimerSchedule::At(50))
        .expect("past deadline should be valid");
    registry
        .confirm_effect_applied(transition.effect())
        .expect("fixture provider effect should apply");
    let snapshot = registry.snapshot(&timer).expect("snapshot should exist");
    assert_eq!(snapshot.next_deadline_ns(), Some(50));
    assert_eq!(snapshot.latest_requested_delay_ns(), None);
    assert_eq!(snapshot.latest_armed_delay_ns(), Some(0));
}
