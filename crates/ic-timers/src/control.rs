//! Pure callback-generation state for one ordinary timer.
//!
//! This module owns no task execution, platform timer handles, persistence, or
//! time source. The canonical registry owns pending-command arbitration.

use thiserror::Error;

/// Current registration state for one timer identity.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TimerRegistration {
    /// No callback is scheduled or running.
    #[default]
    Unregistered,
    /// One generation is scheduled for an absolute nanosecond deadline.
    Scheduled {
        /// Generation owned by the scheduled callback.
        generation: u64,
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// One generation currently owns logical execution.
    Running {
        /// Generation owned by the running callback.
        generation: u64,
    },
}

/// Whether one arm fills an empty wake-up slot or replaces its current handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WakeupArm {
    Initial,
    Replacement,
}

impl WakeupArm {
    pub(crate) const fn replaces_existing(self) -> bool {
        matches!(self, Self::Replacement)
    }
}

/// Invalid or exhausted timer-control transition.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TimerControlError {
    /// The callback generation cannot be incremented.
    #[error("timer generation exhausted")]
    GenerationExhausted,
    /// Completion did not present the generation that owns execution.
    #[error("timer completion does not own the running generation")]
    StaleCompletion,
}

/// Pure state machine for one logical timer identity.
#[derive(Debug, Default)]
pub struct TimerControl {
    generation: u64,
    registration: TimerRegistration,
}

#[derive(Clone, Copy)]
enum DeadlineSelection {
    Earliest,
    Exact,
}

impl DeadlineSelection {
    const fn replaces(self, current_deadline_ns: u64, requested_deadline_ns: u64) -> bool {
        match self {
            Self::Earliest => requested_deadline_ns < current_deadline_ns,
            Self::Exact => requested_deadline_ns != current_deadline_ns,
        }
    }
}

impl TimerControl {
    /// Return the latest allocated callback generation.
    #[must_use]
    #[cfg(test)]
    pub(crate) const fn generation(&self) -> u64 {
        self.generation
    }

    #[cfg(test)]
    pub(crate) const fn exhaust_generation_for_test(&mut self) {
        self.generation = u64::MAX;
    }

    /// Return the current logical registration.
    #[must_use]
    pub(crate) const fn registration(&self) -> TimerRegistration {
        self.registration
    }

    /// Terminate pure control after a checked terminal failure.
    ///
    /// Returns whether a scheduled wake-up must be cleared. A running callback
    /// has already consumed its provider wake-up.
    pub(crate) const fn terminate(&mut self) -> bool {
        let clear_wakeup = matches!(self.registration, TimerRegistration::Scheduled { .. });
        self.registration = TimerRegistration::Unregistered;
        clear_wakeup
    }

    /// Schedule a deadline, retaining an already scheduled earlier deadline.
    /// Return the arm kind when scheduled state changes; the registry reads
    /// the authoritative generation and deadline from that resulting state.
    pub(crate) fn schedule(
        &mut self,
        deadline_ns: u64,
    ) -> Result<Option<WakeupArm>, TimerControlError> {
        self.request_deadline(deadline_ns, DeadlineSelection::Earliest)
    }

    /// Cancel scheduled state immediately.
    ///
    /// Running work is unchanged so the canonical registry can arbitrate its
    /// pending command. Provider cleanup also remains the registry's decision.
    pub(crate) fn cancel(&mut self) -> Result<(), TimerControlError> {
        if matches!(self.registration, TimerRegistration::Scheduled { .. }) {
            let generation = self.next_generation()?;
            self.generation = generation;
            self.registration = TimerRegistration::Unregistered;
        }
        Ok(())
    }

    /// Reconcile this timer to one authoritative deadline.
    pub(crate) fn reconcile(
        &mut self,
        deadline_ns: u64,
    ) -> Result<Option<WakeupArm>, TimerControlError> {
        self.request_deadline(deadline_ns, DeadlineSelection::Exact)
    }

    fn request_deadline(
        &mut self,
        deadline_ns: u64,
        selection: DeadlineSelection,
    ) -> Result<Option<WakeupArm>, TimerControlError> {
        let kind = match self.registration {
            TimerRegistration::Unregistered => WakeupArm::Initial,
            TimerRegistration::Scheduled {
                deadline_ns: current_deadline_ns,
                ..
            } if selection.replaces(current_deadline_ns, deadline_ns) => WakeupArm::Replacement,
            TimerRegistration::Scheduled { .. } | TimerRegistration::Running { .. } => {
                return Ok(None);
            }
        };

        let generation = self.next_generation()?;
        self.generation = generation;
        self.registration = TimerRegistration::Scheduled {
            generation,
            deadline_ns,
        };
        Ok(Some(kind))
    }

    /// Begin the scheduled generation, rejecting stale callbacks.
    pub(crate) const fn begin(&mut self, generation: u64) -> bool {
        match self.registration {
            TimerRegistration::Scheduled {
                generation: scheduled_generation,
                ..
            } if scheduled_generation == generation => {
                self.registration = TimerRegistration::Running { generation };
                true
            }
            TimerRegistration::Unregistered
            | TimerRegistration::Scheduled { .. }
            | TimerRegistration::Running { .. } => false,
        }
    }

    /// Complete the running generation with the registry's already-arbitrated
    /// successor decision. The registry observes the resulting registration;
    /// cancellation policy and provider effects remain its responsibility.
    pub(crate) fn complete(
        &mut self,
        generation: u64,
        next_deadline_ns: Option<u64>,
    ) -> Result<(), TimerControlError> {
        if self.registration != (TimerRegistration::Running { generation }) {
            return Err(TimerControlError::StaleCompletion);
        }

        if let Some(deadline_ns) = next_deadline_ns {
            let next_generation = self.next_generation()?;
            self.generation = next_generation;
            self.registration = TimerRegistration::Scheduled {
                generation: next_generation,
                deadline_ns,
            };
        } else {
            self.registration = TimerRegistration::Unregistered;
        }
        Ok(())
    }

    fn next_generation(&self) -> Result<u64, TimerControlError> {
        self.generation
            .checked_add(1)
            .ok_or(TimerControlError::GenerationExhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arm(control: &mut TimerControl, deadline_ns: u64) -> u64 {
        assert_eq!(control.schedule(deadline_ns), Ok(Some(WakeupArm::Initial)));
        let TimerRegistration::Scheduled { generation, .. } = control.registration() else {
            panic!("initial schedule should arm");
        };
        generation
    }

    #[test]
    fn duplicate_and_later_schedules_keep_one_earliest_handle() {
        let mut control = TimerControl::default();
        assert_eq!(arm(&mut control, 100), 1);
        assert_eq!(control.schedule(100), Ok(None));
        assert_eq!(control.schedule(200), Ok(None));
        assert_eq!(
            control.registration(),
            TimerRegistration::Scheduled {
                generation: 1,
                deadline_ns: 100
            }
        );
    }

    #[test]
    fn earlier_schedule_replaces_and_invalidates_old_generation() {
        let mut control = TimerControl::default();
        let old_generation = arm(&mut control, 100);
        assert_eq!(control.schedule(50), Ok(Some(WakeupArm::Replacement)));
        assert_eq!(
            control.registration(),
            TimerRegistration::Scheduled {
                generation: 2,
                deadline_ns: 50,
            }
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn authoritative_reconciliation_can_move_scheduled_deadline_later() {
        let mut control = TimerControl::default();
        let old_generation = arm(&mut control, 100);
        assert_eq!(control.reconcile(200), Ok(Some(WakeupArm::Replacement)));
        assert_eq!(
            control.registration(),
            TimerRegistration::Scheduled {
                generation: 2,
                deadline_ns: 200,
            }
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn running_requests_leave_completion_selection_to_the_registry() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.reconcile(300), Ok(None));
        assert_eq!(control.schedule(90), Ok(None));
        assert_eq!(control.cancel(), Ok(()));
        assert_eq!(
            control.registration(),
            TimerRegistration::Running { generation }
        );
        assert_eq!(control.generation(), generation);
    }

    #[test]
    fn completion_arms_the_supplied_deadline() {
        for deadline_ns in [90, 300] {
            let mut control = TimerControl::default();
            let generation = arm(&mut control, 100);
            assert!(control.begin(generation));
            assert_eq!(control.complete(generation, Some(deadline_ns)), Ok(()));
            assert_eq!(
                control.registration(),
                TimerRegistration::Scheduled {
                    generation: 2,
                    deadline_ns,
                }
            );
            assert_eq!(control.generation(), 2);
        }
    }

    #[test]
    fn completion_without_a_successor_stops_running_work() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.complete(generation, None), Ok(()));
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
    }

    #[test]
    fn scheduled_cancel_invalidates_consumed_generation() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert_eq!(control.cancel(), Ok(()));
        assert!(!control.begin(generation));
        assert_eq!(control.generation(), 2);
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
        assert_eq!(control.cancel(), Ok(()));
        assert_eq!(control.generation(), 2);
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
    }

    #[test]
    fn stale_completion_cannot_change_current_registration() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(
            control.complete(generation + 1, None),
            Err(TimerControlError::StaleCompletion)
        );
        assert_eq!(
            control.registration(),
            TimerRegistration::Running { generation }
        );
    }

    #[test]
    fn exhausted_generation_rejects_successors_without_preventing_stop() {
        let mut control = TimerControl {
            generation: u64::MAX,
            registration: TimerRegistration::Scheduled {
                generation: u64::MAX,
                deadline_ns: 100,
            },
        };

        assert_eq!(
            control.schedule(50),
            Err(TimerControlError::GenerationExhausted)
        );
        assert_eq!(
            control.cancel(),
            Err(TimerControlError::GenerationExhausted)
        );
        assert_eq!(
            control.registration(),
            TimerRegistration::Scheduled {
                generation: u64::MAX,
                deadline_ns: 100
            }
        );

        assert!(control.begin(u64::MAX));
        assert_eq!(
            control.complete(u64::MAX, Some(200)),
            Err(TimerControlError::GenerationExhausted)
        );
        assert_eq!(
            control.registration(),
            TimerRegistration::Running {
                generation: u64::MAX
            }
        );
        assert_eq!(
            control.complete(u64::MAX, None),
            Ok(()),
            "stopping does not allocate a successor generation"
        );
        assert_eq!(control.generation(), u64::MAX);
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
    }
}
