use std::fmt;

/// Errors produced by confusable detection and scoring.
#[derive(Clone, PartialEq, Eq)]
pub enum ConfusableError {
    /// The input string was empty.
    EmptyInput,
}

impl fmt::Display for ConfusableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(
                f,
                "input must not be empty; pass a non-empty value to --input"
            ),
        }
    }
}

impl fmt::Debug for ConfusableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for ConfusableError {}
