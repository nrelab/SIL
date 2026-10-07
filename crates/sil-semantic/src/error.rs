use std::fmt;

/// Errors produced by semantic analysis.
#[derive(Clone, PartialEq, Eq)]
pub enum SemanticError {
    /// One of the compared strings was empty.
    EmptyInput,
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "both strings must be non-empty to compare"),
        }
    }
}

impl fmt::Debug for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for SemanticError {}
