//! Reproducible, float-free generation and exact solving of 3-CNF instances.
//!
//! # Mathematical meaning
//!
//! This module supplies the labelled corpus for the reproducibility experiments:
//! a deterministic integer PRNG ([`SplitMix64`]), random 3-CNF sampling at the
//! phase-transition ratio (encoded exactly as `alpha = 213/50`), and an
//! exhaustive exact satisfiability oracle ([`is_satisfiable`]) that scans all
//! `2^n` assignments. Randomness is a pure `u64` stream, so every dataset is
//! reproducible bit-for-bit from its seed.
//!
//! Strict policy: no floating point and no probabilistic tests.
//!
//! # Complexity
//!
//! [`is_satisfiable`] is `O(2^n * m)`, exact; sampling is `O(m)`.

use crate::models::cnf::{Clause, Literal};

/// Exact numerator of the critical clause/variable ratio `4.26 = 213/50`.
pub const RATIO_NUMERATOR: usize = 213;
/// Exact denominator of the critical clause/variable ratio `4.26 = 213/50`.
pub const RATIO_DENOMINATOR: usize = 50;

/// Deterministic, allocation-free SplitMix64 pseudo-random generator.
///
/// Operates purely on `u64` state -- no floating point is involved.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Create a generator from a 64-bit seed.
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advance the stream and return the next 64-bit word.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform-ish integer in `0..bound` via modular reduction (`bound > 0`).
    #[inline]
    pub fn next_below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    /// A fair coin flip used for equiprobable literal polarity.
    #[inline]
    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }
}

/// The critical clause count `m = floor(4.26 * n) = floor(213 * n / 50)`,
/// computed with exact integer arithmetic.
#[inline]
pub fn critical_clause_count(num_variables: usize) -> usize {
    (RATIO_NUMERATOR * num_variables) / RATIO_DENOMINATOR
}

/// Sample `count` distinct variable indices from `0..num_variables`.
fn sample_distinct_variables(
    rng: &mut SplitMix64,
    num_variables: usize,
    count: usize,
) -> Vec<usize> {
    let mut chosen: Vec<usize> = Vec::with_capacity(count);
    while chosen.len() < count {
        let candidate = rng.next_below(num_variables as u64) as usize;
        if !chosen.contains(&candidate) {
            chosen.push(candidate);
        }
    }
    chosen
}

/// Generate a single width-3 clause with three distinct variables and
/// equiprobable polarities.
pub fn random_clause(rng: &mut SplitMix64, num_variables: usize) -> Clause {
    let variables = sample_distinct_variables(rng, num_variables, 3);
    let literals = variables
        .into_iter()
        .map(|variable_idx| Literal::new(variable_idx, rng.next_bool()))
        .collect();
    Clause::new(literals)
}

/// Generate a random 3-CNF formula with `num_clauses` clauses.
pub fn random_3cnf(rng: &mut SplitMix64, num_variables: usize, num_clauses: usize) -> Vec<Clause> {
    (0..num_clauses)
        .map(|_| random_clause(rng, num_variables))
        .collect()
}

/// Ground-truth oracle `I_0`: exhaustive search over all `2^n` assignments.
///
/// Returns `true` iff some assignment satisfies every clause. This is exact,
/// not heuristic; bit `v` of `assignment` is the boolean value of variable `v`.
pub fn is_satisfiable(num_variables: usize, clauses: &[Clause]) -> bool {
    let total_assignments: u64 = 1u64 << num_variables;
    (0..total_assignments).any(|assignment| {
        clauses.iter().all(|clause| {
            clause.literals.iter().any(|literal| {
                let value = (assignment >> literal.variable_idx) & 1 == 1;
                value == literal.is_positive
            })
        })
    })
}
