//! Canonical and heterogeneous unsatisfiable control formulas.
//!
//! # Mathematical meaning
//!
//! These deterministic builders reproduce the unsatisfiable instances of the
//! manuscript: the canonical minimal `n = 3` formula (all `2^3 = 8` sign
//! patterns on `(x0, x1, x2)`, which no assignment satisfies) and the
//! degree-heterogeneous `n = 4` control (the eight-pattern core plus three
//! auxiliary clauses that introduce degree heterogeneity and unbalanced local
//! polarities, so the tensor keeps non-zero imaginary components).

use crate::generators::sat::control_clause;
use crate::models::cnf::{Clause, Literal};

/// The canonical **UNSAT** 3-variable formula: all `2^3 = 8` sign patterns over
/// `(x0, x1, x2)`. Every truth assignment falsifies exactly the one clause whose
/// literals it makes all-false, so no assignment satisfies the whole set.
pub fn canonical_unsat_clauses() -> Vec<Clause> {
    (0u8..8)
        .map(|mask| control_clause((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0))
        .collect()
}

/// The degree-heterogeneous **UNSAT** `n = 4` control formula.
///
/// The eight sign patterns on `{0,1,2}` already force unsatisfiability (adding
/// clauses cannot restore satisfiability); three extra clauses on `{0,2,3}` and
/// `{1,2,3}` introduce the degree profile
/// `(deg 0, deg 1, deg 2, deg 3) = (10, 9, 11, 3)` and locally unbalanced
/// polarities that survive the Fourier cancellation.
pub fn heterogeneous_unsat_clauses() -> Vec<Clause> {
    let mut clauses: Vec<Clause> = (0u8..8)
        .map(|mask| {
            Clause::new(vec![
                Literal::new(0, (mask & 1) != 0),
                Literal::new(1, (mask & 2) != 0),
                Literal::new(2, (mask & 4) != 0),
            ])
        })
        .collect();
    clauses.push(Clause::new(vec![
        Literal::new(0, true),
        Literal::new(2, true),
        Literal::new(3, true),
    ]));
    clauses.push(Clause::new(vec![
        Literal::new(0, true),
        Literal::new(2, true),
        Literal::new(3, false),
    ]));
    clauses.push(Clause::new(vec![
        Literal::new(1, false),
        Literal::new(2, true),
        Literal::new(3, true),
    ]));
    clauses
}
