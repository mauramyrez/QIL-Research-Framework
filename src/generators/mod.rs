//! `generators` — reproducible CNF instance generation and exact labelling.
//!
//! * [`random_formula`] — the deterministic SplitMix64 PRNG, phase-transition
//!   sampling, and the exhaustive exact satisfiability oracle.
//! * [`sat`] — canonical and heterogeneous satisfiable control formulas.
//! * [`unsat`] — canonical and heterogeneous unsatisfiable control formulas.
//!
//! Everything is exact and reproducible bit-for-bit from a seed.

pub mod random_formula;
pub mod sat;
pub mod unsat;

pub use random_formula::{
    critical_clause_count, is_satisfiable, random_3cnf, random_clause, SplitMix64,
    RATIO_DENOMINATOR, RATIO_NUMERATOR,
};
