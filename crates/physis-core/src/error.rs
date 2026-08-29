//! Error type for the substrate.

use std::fmt;

/// Recoverable failure at the core layer (unknown knobs, domain violations).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreError {
    /// No knob with this name on the object.
    UnknownKnob {
        /// Requested name.
        name: String,
    },
    /// Value is not in the knob's domain.
    Domain {
        /// Knob name.
        name: String,
        /// Why it failed.
        reason: String,
    },
    /// Value kind does not match the knob (bool vs float, …).
    TypeMismatch {
        /// Knob name.
        name: String,
        /// Expected kind.
        expected: String,
        /// Got kind.
        got: String,
    },
    /// Named theory is not in the lab.
    UnknownTheory {
        /// Requested id.
        id: String,
    },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::UnknownKnob { name } => write!(f, "unknown knob '{name}'"),
            CoreError::Domain { name, reason } => write!(f, "knob '{name}' domain: {reason}"),
            CoreError::TypeMismatch {
                name,
                expected,
                got,
            } => {
                write!(f, "knob '{name}' expected {expected}, got {got}")
            }
            CoreError::UnknownTheory { id } => write!(f, "unknown theory '{id}'"),
        }
    }
}

impl std::error::Error for CoreError {}
