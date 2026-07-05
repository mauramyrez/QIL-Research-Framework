//! Canonical and heterogeneous satisfiable control formulas.
//!
//! # Mathematical meaning
//!
//! These deterministic builders reproduce the satisfiable instances used in the
//! manuscript's worked examples: the minimal `n = 3` control (all sign patterns
//! except the all-negative clause, so the all-true assignment satisfies it) and
//! the degree-heterogeneous `n = 4` control whose tensor retains non-zero
//! imaginary components. All are exact and reproducible.

use crate::models::cnf::{Clause, Literal};

/// Build the width-3 control clause over variables `(0, 1, 2)` with the given
/// literal polarities (`true` = positive `x_t`, `false` = negated `not x_t`).
pub fn control_clause(sign_0: bool, sign_1: bool, sign_2: bool) -> Clause {
    Clause::new(vec![
        Literal::new(0, sign_0),
        Literal::new(1, sign_1),
        Literal::new(2, sign_2),
    ])
}

/// The canonical **SAT** 3-variable formula: the eight sign patterns over
/// `(x0, x1, x2)` with the all-negative clause (`mask == 0`) omitted. The
/// all-true assignment satisfies every remaining clause.
pub fn canonical_sat_clauses() -> Vec<Clause> {
    (1u8..8)
        .map(|mask| control_clause((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0))
        .collect()
}

/// The degree-heterogeneous **SAT** `n = 4` control formula.
///
/// Satisfied by the all-true assignment; clauses span the triplets
/// `{0,1,2}`, `{0,2,3}`, `{1,2,3}`, so the degree profile
/// `(deg 0, deg 1, deg 2, deg 3) = (4, 3, 5, 3)` is non-uniform.
pub fn heterogeneous_sat_clauses() -> Vec<Clause> {
    vec![
        Clause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
            Literal::new(2, true),
        ]),
        Clause::new(vec![
            Literal::new(0, true),
            Literal::new(2, true),
            Literal::new(3, false),
        ]),
        Clause::new(vec![
            Literal::new(1, false),
            Literal::new(2, true),
            Literal::new(3, true),
        ]),
        Clause::new(vec![
            Literal::new(0, true),
            Literal::new(1, true),
            Literal::new(2, true),
        ]),
        Clause::new(vec![
            Literal::new(0, false),
            Literal::new(2, true),
            Literal::new(3, true),
        ]),
    ]
}
