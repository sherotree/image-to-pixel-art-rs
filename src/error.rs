//! Errors returned by buffer validation and conversion helpers.

use std::fmt;

/// Library error type (buffer validation, empty palette, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Width or height is zero or otherwise invalid.
    InvalidDimensions {
        /// Human-readable label (e.g. `"source"`).
        label: &'static str,
        /// Detail message.
        message: &'static str,
    },
    /// `data.len()` does not equal `width * height * 4`.
    LengthMismatch {
        /// Human-readable label.
        label: &'static str,
        /// Actual byte length.
        actual: usize,
        /// Expected byte length.
        expected: usize,
    },
    /// Palette must contain at least one color.
    EmptyPalette,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDimensions { label, message } => {
                write!(f, "{label}: {message}")
            }
            Self::LengthMismatch {
                label,
                actual,
                expected,
            } => {
                write!(f, "{label}: data length {actual} !== {expected}")
            }
            Self::EmptyPalette => write!(f, "Palette must contain at least one color"),
        }
    }
}

impl std::error::Error for Error {}

/// Convenient result alias.
pub type Result<T> = std::result::Result<T, Error>;
