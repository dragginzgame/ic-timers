//! Bounded, deterministically ordered timer identity.

use std::{fmt, ops::Deref};
use thiserror::Error;

/// Maximum encoded length of one timer identity label.
///
/// The limit is measured in UTF-8 bytes so storage and metrics cardinality do
/// not depend on Unicode scalar width.
pub const MAX_TIMER_LABEL_BYTES: usize = 64;

/// One bounded component of a timer identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerLabel(Box<str>);

impl TimerLabel {
    /// Validate and copy a timer label.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TimerLabelError> {
        let value = value.as_ref();
        if value.is_empty() {
            return Err(TimerLabelError::Empty);
        }
        if value.len() > MAX_TIMER_LABEL_BYTES {
            return Err(TimerLabelError::TooLong {
                actual_bytes: value.len(),
                max_bytes: MAX_TIMER_LABEL_BYTES,
            });
        }
        if value.trim() != value {
            return Err(TimerLabelError::SurroundingWhitespace);
        }
        if let Some((byte_index, character)) = value.char_indices().find(|(_, ch)| ch.is_control())
        {
            return Err(TimerLabelError::ControlCharacter {
                byte_index,
                character,
            });
        }

        Ok(Self(value.into()))
    }

    /// Borrow the exact validated label.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for TimerLabel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for TimerLabel {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for TimerLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<&str> for TimerLabel {
    type Error = TimerLabelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for TimerLabel {
    type Error = TimerLabelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

/// Invalid timer identity label.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum TimerLabelError {
    /// Labels must not be empty.
    #[error("timer identity label must not be empty")]
    Empty,
    /// Labels must fit the documented UTF-8 byte bound.
    #[error("timer identity label is {actual_bytes} bytes; maximum is {max_bytes}")]
    TooLong {
        /// Encoded length of the rejected value.
        actual_bytes: usize,
        /// Maximum supported encoded length.
        max_bytes: usize,
    },
    /// Leading and trailing whitespace makes metric labels hard to distinguish.
    #[error("timer identity label must not have leading or trailing whitespace")]
    SurroundingWhitespace,
    /// Control characters are not portable operator labels.
    #[error("timer identity label contains control character {character:?} at byte {byte_index}")]
    ControlCharacter {
        /// UTF-8 byte offset of the rejected character.
        byte_index: usize,
        /// Rejected control character.
        character: char,
    },
}

/// The identity component whose validation failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerIdentityField {
    /// Scheduling owner, such as a framework or application.
    Owner,
    /// Functional area within the owner.
    Subsystem,
    /// Stable logical timer name.
    Name,
}

impl fmt::Display for TimerIdentityField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Owner => "owner",
            Self::Subsystem => "subsystem",
            Self::Name => "name",
        })
    }
}

/// Invalid structured timer identity.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("invalid timer {field}: {source}")]
pub struct TimerIdentityError {
    /// Identity component that failed validation.
    pub field: TimerIdentityField,
    /// Label validation error.
    #[source]
    pub source: TimerLabelError,
}

/// Stable, low-cardinality identity for one logical canister timer.
///
/// Derived ordering is lexicographic by owner, subsystem, then name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerIdentity {
    owner: TimerLabel,
    subsystem: TimerLabel,
    name: TimerLabel,
}

impl TimerIdentity {
    /// Construct an identity from already validated labels.
    #[must_use]
    pub const fn new(owner: TimerLabel, subsystem: TimerLabel, name: TimerLabel) -> Self {
        Self {
            owner,
            subsystem,
            name,
        }
    }

    /// Validate all three identity components.
    pub fn try_new(
        owner: impl AsRef<str>,
        subsystem: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Self, TimerIdentityError> {
        Ok(Self::new(
            validate_component(TimerIdentityField::Owner, owner)?,
            validate_component(TimerIdentityField::Subsystem, subsystem)?,
            validate_component(TimerIdentityField::Name, name)?,
        ))
    }

    /// Return the scheduling owner.
    #[must_use]
    pub const fn owner(&self) -> &TimerLabel {
        &self.owner
    }

    /// Return the functional subsystem.
    #[must_use]
    pub const fn subsystem(&self) -> &TimerLabel {
        &self.subsystem
    }

    /// Return the stable logical timer name.
    #[must_use]
    pub const fn name(&self) -> &TimerLabel {
        &self.name
    }
}

fn validate_component(
    field: TimerIdentityField,
    value: impl AsRef<str>,
) -> Result<TimerLabel, TimerIdentityError> {
    TimerLabel::new(value).map_err(|source| TimerIdentityError { field, source })
}
