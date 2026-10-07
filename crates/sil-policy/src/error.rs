use std::fmt;

/// Errors produced during policy evaluation.
#[derive(Clone, PartialEq)]
pub enum PolicyError {
    /// A risk dimension was NaN, infinite, or outside `0.0..=1.0`.
    InvalidRiskScore {
        /// Name of the offending field.
        field: &'static str,
        /// Offending value.
        value: f32,
    },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRiskScore { field, value } => write!(
                f,
                "invalid `{field}` value: {value}; risk scores must be finite numbers in the range 0.0..=1.0"
            ),
        }
    }
}

impl fmt::Debug for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for PolicyError {}
