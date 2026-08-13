//! Deterministic arbitration for one logical timer identity.
//!
//! This module owns no task execution, platform timer handles, persistence, or
//! time source.

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PendingCommand {
    Cancel {
        sequence: u64,
    },
    #[allow(dead_code)] // Lifecycle reconciliation binds this in Patch 5.
    Reconcile {
        sequence: u64,
        deadline_ns: u64,
    },
    Schedule {
        sequence: u64,
        deadline_ns: u64,
    },
}

/// Side effect requested from the timer platform boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerControlAction {
    /// No platform change is required.
    None,
    /// Arm a new callback.
    Arm {
        /// Generation the callback must present when it begins.
        generation: u64,
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// Clear the existing handle and arm this replacement.
    Replace {
        /// Generation the replacement callback must present.
        generation: u64,
        /// Absolute IC timestamp in nanoseconds.
        deadline_ns: u64,
    },
    /// Clear the existing scheduled handle.
    Clear,
    /// The completed run leaves no scheduled successor.
    Disarm {
        /// Whether an explicit cancellation won over the run's directive.
        cancelled: bool,
    },
}

/// Invalid or exhausted timer-control transition.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TimerControlError {
    /// The monotonic request sequence cannot be incremented.
    #[error("timer request sequence exhausted")]
    RequestSequenceExhausted,
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
    request_sequence: u64,
    registration: TimerRegistration,
    pending: Option<PendingCommand>,
}

impl TimerControl {
    /// Return the latest allocated callback generation.
    #[must_use]
    #[allow(dead_code)] // Retained for lifecycle reconciliation in Patch 5.
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Return the current logical registration.
    #[must_use]
    pub const fn registration(&self) -> TimerRegistration {
        self.registration
    }

    /// Terminate pure control after a checked terminal failure.
    ///
    /// Returns whether a scheduled wake-up must be cleared. A running callback
    /// has already consumed its provider wake-up.
    pub(crate) const fn terminate(&mut self) -> bool {
        let clear_wakeup = matches!(self.registration, TimerRegistration::Scheduled { .. });
        self.registration = TimerRegistration::Unregistered;
        self.pending = None;
        clear_wakeup
    }

    /// Schedule a deadline, retaining an already scheduled earlier deadline.
    pub fn schedule(&mut self, deadline_ns: u64) -> Result<TimerControlAction, TimerControlError> {
        let sequence = self.next_request_sequence()?;

        match self.registration {
            TimerRegistration::Unregistered => {
                let generation = self.next_generation()?;
                self.request_sequence = sequence;
                self.generation = generation;
                self.registration = TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                };
                Ok(TimerControlAction::Arm {
                    generation,
                    deadline_ns,
                })
            }
            TimerRegistration::Scheduled {
                deadline_ns: current_deadline,
                ..
            } if deadline_ns < current_deadline => {
                let generation = self.next_generation()?;
                self.request_sequence = sequence;
                self.generation = generation;
                self.registration = TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                };
                Ok(TimerControlAction::Replace {
                    generation,
                    deadline_ns,
                })
            }
            TimerRegistration::Scheduled { .. } => {
                self.request_sequence = sequence;
                Ok(TimerControlAction::None)
            }
            TimerRegistration::Running { .. } => {
                self.request_sequence = sequence;
                self.pending = Some(match self.pending {
                    Some(
                        PendingCommand::Schedule {
                            deadline_ns: current_deadline,
                            ..
                        }
                        | PendingCommand::Reconcile {
                            deadline_ns: current_deadline,
                            ..
                        },
                    ) => PendingCommand::Schedule {
                        sequence,
                        deadline_ns: current_deadline.min(deadline_ns),
                    },
                    Some(PendingCommand::Cancel { .. }) | None => PendingCommand::Schedule {
                        sequence,
                        deadline_ns,
                    },
                });
                Ok(TimerControlAction::None)
            }
        }
    }

    /// Cancel this timer, deferring cancellation safely when a callback runs.
    pub fn cancel(&mut self) -> Result<TimerControlAction, TimerControlError> {
        let sequence = self.next_request_sequence()?;

        match self.registration {
            TimerRegistration::Unregistered => {
                self.request_sequence = sequence;
                Ok(TimerControlAction::None)
            }
            TimerRegistration::Scheduled { .. } => {
                let generation = self.next_generation()?;
                self.request_sequence = sequence;
                self.generation = generation;
                self.registration = TimerRegistration::Unregistered;
                self.pending = None;
                Ok(TimerControlAction::Clear)
            }
            TimerRegistration::Running { .. } => {
                self.request_sequence = sequence;
                self.pending = Some(PendingCommand::Cancel { sequence });
                Ok(TimerControlAction::None)
            }
        }
    }

    /// Reconcile this timer to one authoritative deadline.
    #[allow(dead_code)] // Lifecycle reconciliation binds this in Patch 5.
    pub fn reconcile(&mut self, deadline_ns: u64) -> Result<TimerControlAction, TimerControlError> {
        let sequence = self.next_request_sequence()?;

        match self.registration {
            TimerRegistration::Unregistered => {
                let generation = self.next_generation()?;
                self.request_sequence = sequence;
                self.generation = generation;
                self.registration = TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                };
                Ok(TimerControlAction::Arm {
                    generation,
                    deadline_ns,
                })
            }
            TimerRegistration::Scheduled {
                deadline_ns: current_deadline,
                ..
            } if deadline_ns == current_deadline => {
                self.request_sequence = sequence;
                Ok(TimerControlAction::None)
            }
            TimerRegistration::Scheduled { .. } => {
                let generation = self.next_generation()?;
                self.request_sequence = sequence;
                self.generation = generation;
                self.registration = TimerRegistration::Scheduled {
                    generation,
                    deadline_ns,
                };
                Ok(TimerControlAction::Replace {
                    generation,
                    deadline_ns,
                })
            }
            TimerRegistration::Running { .. } => {
                self.request_sequence = sequence;
                self.pending = Some(PendingCommand::Reconcile {
                    sequence,
                    deadline_ns,
                });
                Ok(TimerControlAction::None)
            }
        }
    }

    /// Begin the scheduled generation, rejecting stale callbacks.
    pub const fn begin(&mut self, generation: u64) -> bool {
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

    /// Complete the running generation and arbitrate its proposed successor
    /// against commands received during execution.
    pub fn complete(
        &mut self,
        generation: u64,
        directive_deadline_ns: Option<u64>,
    ) -> Result<TimerControlAction, TimerControlError> {
        if self.registration != (TimerRegistration::Running { generation }) {
            return Err(TimerControlError::StaleCompletion);
        }

        let (deadline_ns, cancelled) = match self.pending {
            Some(PendingCommand::Cancel { .. }) => (None, true),
            Some(PendingCommand::Reconcile { deadline_ns, .. }) => (Some(deadline_ns), false),
            Some(PendingCommand::Schedule { deadline_ns, .. }) => (
                Some(
                    directive_deadline_ns
                        .map_or(deadline_ns, |directive| directive.min(deadline_ns)),
                ),
                false,
            ),
            None => (directive_deadline_ns, false),
        };

        let next_generation = if deadline_ns.is_some() {
            Some(self.next_generation()?)
        } else {
            None
        };

        self.pending = None;
        if let (Some(deadline_ns), Some(next_generation)) = (deadline_ns, next_generation) {
            self.generation = next_generation;
            self.registration = TimerRegistration::Scheduled {
                generation: next_generation,
                deadline_ns,
            };
            Ok(TimerControlAction::Arm {
                generation: next_generation,
                deadline_ns,
            })
        } else {
            self.registration = TimerRegistration::Unregistered;
            Ok(TimerControlAction::Disarm { cancelled })
        }
    }

    fn next_generation(&self) -> Result<u64, TimerControlError> {
        self.generation
            .checked_add(1)
            .ok_or(TimerControlError::GenerationExhausted)
    }

    fn next_request_sequence(&self) -> Result<u64, TimerControlError> {
        self.request_sequence
            .checked_add(1)
            .ok_or(TimerControlError::RequestSequenceExhausted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arm(control: &mut TimerControl, deadline_ns: u64) -> u64 {
        let TimerControlAction::Arm { generation, .. } = control
            .schedule(deadline_ns)
            .expect("initial schedule should succeed")
        else {
            panic!("initial schedule should arm");
        };
        generation
    }

    #[test]
    fn duplicate_and_later_schedules_keep_one_earliest_handle() {
        let mut control = TimerControl::default();
        assert_eq!(arm(&mut control, 100), 1);
        assert_eq!(control.schedule(100), Ok(TimerControlAction::None));
        assert_eq!(control.schedule(200), Ok(TimerControlAction::None));
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
        assert_eq!(
            control.schedule(50),
            Ok(TimerControlAction::Replace {
                generation: 2,
                deadline_ns: 50
            })
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn authoritative_reconciliation_can_move_scheduled_deadline_later() {
        let mut control = TimerControl::default();
        let old_generation = arm(&mut control, 100);
        assert_eq!(
            control.reconcile(200),
            Ok(TimerControlAction::Replace {
                generation: 2,
                deadline_ns: 200
            })
        );
        assert!(!control.begin(old_generation));
        assert!(control.begin(2));
    }

    #[test]
    fn authoritative_reconciliation_while_running_replaces_callback_deadline() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.reconcile(300), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, Some(150)),
            Ok(TimerControlAction::Arm {
                generation: 2,
                deadline_ns: 300
            })
        );
    }

    #[test]
    fn schedule_while_running_waits_and_survives_callback_stop() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.schedule(90), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, None),
            Ok(TimerControlAction::Arm {
                generation: 2,
                deadline_ns: 90
            })
        );
    }

    #[test]
    fn running_callback_can_request_its_own_cancellation() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.cancel(), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, Some(200)),
            Ok(TimerControlAction::Disarm { cancelled: true })
        );
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
    }

    #[test]
    fn running_schedule_coalesces_to_earliest_deadline() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.schedule(300), Ok(TimerControlAction::None));
        assert_eq!(control.schedule(250), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, Some(275)),
            Ok(TimerControlAction::Arm {
                generation: 2,
                deadline_ns: 250
            })
        );
    }

    #[test]
    fn later_cancel_suppresses_callback_rearm() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.schedule(80), Ok(TimerControlAction::None));
        assert_eq!(control.cancel(), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, Some(75)),
            Ok(TimerControlAction::Disarm { cancelled: true })
        );
    }

    #[test]
    fn schedule_after_cancel_reenables_only_after_completion() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert!(control.begin(generation));
        assert_eq!(control.cancel(), Ok(TimerControlAction::None));
        assert_eq!(control.schedule(90), Ok(TimerControlAction::None));
        assert_eq!(
            control.complete(generation, Some(95)),
            Ok(TimerControlAction::Arm {
                generation: 2,
                deadline_ns: 90
            })
        );
    }

    #[test]
    fn scheduled_cancel_invalidates_consumed_generation() {
        let mut control = TimerControl::default();
        let generation = arm(&mut control, 100);
        assert_eq!(control.cancel(), Ok(TimerControlAction::Clear));
        assert!(!control.begin(generation));
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
    fn exhausted_request_sequence_fails_without_mutating_registration() {
        let mut control = TimerControl {
            request_sequence: u64::MAX,
            ..TimerControl::default()
        };

        assert_eq!(
            control.schedule(100),
            Err(TimerControlError::RequestSequenceExhausted)
        );
        assert_eq!(control.registration(), TimerRegistration::Unregistered);
    }

    #[test]
    fn exhausted_generation_fails_without_replacing_current_handle() {
        let mut control = TimerControl {
            generation: u64::MAX,
            registration: TimerRegistration::Scheduled {
                generation: u64::MAX,
                deadline_ns: 100,
            },
            ..TimerControl::default()
        };

        assert_eq!(
            control.schedule(50),
            Err(TimerControlError::GenerationExhausted)
        );
        assert_eq!(
            control.registration(),
            TimerRegistration::Scheduled {
                generation: u64::MAX,
                deadline_ns: 100
            }
        );
    }
}
