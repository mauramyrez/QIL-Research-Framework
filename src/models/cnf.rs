//! The CNF data model: literals and clauses.
//!
//! # Mathematical meaning
//!
//! A 3-CNF formula `phi = C_1 and ... and C_m` over Boolean variables
//! `x_0, ..., x_{n-1}` is the shared input of every model in QIL. A
//! [`Literal`] is a variable together with a polarity, and a [`Clause`] is a
//! disjunction of literals (width `3` for 3-SAT). Both the bipartite incidence
//! matrix ([`crate::models::incidence_matrix`]) and the order-3 hypergraph
//! tensor ([`crate::models::hypergraph_tensor`]) consume this representation,
//! as do the generators ([`crate::generators`]) and the DIMACS parser
//! ([`crate::io::parser`]).
//!
//! # Complexity
//!
//! Construction is `O(width)`; the types are plain data with no arithmetic.

/// A single literal of a CNF clause.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Literal {
    /// Zero-based index of the underlying boolean variable.
    pub variable_idx: usize,
    /// `true` for `x`, `false` for the negation `not x`.
    pub is_positive: bool,
}

impl Literal {
    /// Construct a literal from a variable index and polarity.
    #[inline]
    pub fn new(variable_idx: usize, is_positive: bool) -> Self {
        Self {
            variable_idx,
            is_positive,
        }
    }

    /// The sign of the literal as an exact integer: `+1` for a positive literal,
    /// `-1` for a negative one. This is the polarity used by the tensor
    /// encoding.
    #[inline]
    pub fn sign(&self) -> i64 {
        if self.is_positive {
            1
        } else {
            -1
        }
    }
}

/// A disjunctive clause, typically of width 3 for 3-SAT.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clause {
    /// The literals joined by logical OR.
    pub literals: Vec<Literal>,
}

impl Clause {
    /// Construct a clause from its literals.
    #[inline]
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    /// The number of literals in the clause (its width).
    #[inline]
    pub fn width(&self) -> usize {
        self.literals.len()
    }
}
