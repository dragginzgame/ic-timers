//! Observable and recovery-aware timer primitives for Internet Computer
//! canisters.
//!
//! This initial scaffold exposes the deterministic control and platform
//! boundaries extracted from Canic. It does not yet provide the complete
//! registry or watchdog runtime described in the workspace architecture note.

#![forbid(unsafe_code)]

pub mod control;
pub mod platform;
pub mod schedule;

pub use control::{TimerControl, TimerControlAction, TimerControlError, TimerRegistration};
pub use platform::{TimerHandle, clear_timer, set_timer};
pub use schedule::{ScheduleError, TimerDirective};
