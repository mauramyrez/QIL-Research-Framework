//! Typed error surface for the Quaternionic Invariant Laboratory (QIL).
//!
//! QIL deliberately keeps a minimal dependency footprint (only exact-arithmetic
//! crates), so the error type is hand-written rather than derived through
//! `thiserror`. Every fallible public operation in the framework returns a
//! [`QilResult`], allowing downstream researchers to distinguish, e.g., a
//! malformed DIMACS input from a structurally singular matrix without resorting
//! to panics.
//!
//! # Paper reference
//!
//! This module is infrastructural; it does not implement a mathematical object
//! of the manuscript, but it underpins the reproducible I/O described in the
//! "The Quaternionic Invariant Laboratory (QIL)" section.

use std::fmt;

/// The unified error type for QIL fallible operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QilError {
    /// A DIMACS CNF stream was malformed; the payload describes the offending
    /// token or line.
    DimacsParse(String),
    /// A literal referenced a variable index outside the declared variable
    /// count. Carries `(offending_index, declared_count)`.
    VariableOutOfRange { index: usize, declared: usize },
    /// A clause did not have the width required by the target model (the
    /// order-3 tensor requires width-3 clauses). Carries `(found_width)`.
    ClauseWidth { found: usize, expected: usize },
    /// A contraction or matrix operation received mismatched dimensions.
    DimensionMismatch { expected: usize, found: usize },
}

impl fmt::Display for QilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QilError::DimacsParse(message) => {
                write!(f, "DIMACS parse error: {message}")
            }
            QilError::VariableOutOfRange { index, declared } => write!(
                f,
                "literal references variable {index} but only {declared} variables were declared"
            ),
            QilError::ClauseWidth { found, expected } => write!(
                f,
                "clause width {found} is invalid; a {expected}-uniform model requires width-{expected} clauses"
            ),
            QilError::DimensionMismatch { expected, found } => write!(
                f,
                "dimension mismatch: expected {expected}, found {found}"
            ),
        }
    }
}

impl std::error::Error for QilError {}

/// The QIL result alias.
pub type QilResult<T> = Result<T, QilError>;
