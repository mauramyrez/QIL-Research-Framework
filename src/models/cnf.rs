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

    /// Support of the clause as a sorted list of distinct variable indices.
    #[inline]
    pub fn support(&self) -> Vec<usize> {
        let mut vars: Vec<usize> = self.literals.iter().map(|lit| lit.variable_idx).collect();
        vars.sort_unstable();
        vars.dedup();
        vars
    }

    /// Sign of the literal on `variable`, or `None` if the variable is absent.
    #[inline]
    pub fn sign_on(&self, variable: usize) -> Option<i64> {
        self.literals
            .iter()
            .find(|lit| lit.variable_idx == variable)
            .map(Literal::sign)
    }
}

/// A CNF formula: a finite set of clauses over `n` Boolean variables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cnf {
    /// Number of Boolean variables (indices in `0..n_vars`).
    pub n_vars: usize,
    /// Clauses of the formula (order is part of the concrete presentation).
    pub clauses: Vec<Clause>,
}

impl Cnf {
    /// Construct a CNF from a variable count and clause list.
    #[inline]
    pub fn new(n_vars: usize, clauses: Vec<Clause>) -> Self {
        Self { n_vars, clauses }
    }

    /// Number of clauses `m`.
    #[inline]
    pub fn num_clauses(&self) -> usize {
        self.clauses.len()
    }
}
