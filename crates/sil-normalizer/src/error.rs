use std::fmt;

/// Errors produced by the normalization pipeline.
#[derive(Clone, PartialEq, Eq)]
pub enum NormalizerError {
    /// The input string was empty.
    EmptyInput,
}

impl fmt::Display for NormalizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(
                f,
                "input must not be empty; pass a non-empty value to --input"
            ),
        }
    }
}

impl fmt::Debug for NormalizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for NormalizerError {}
