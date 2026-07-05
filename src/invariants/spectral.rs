//! Non-commutative word-trace spectrum of the incidence matrix.
//!
//! # Mathematical meaning
//!
//! Standard eigenvalues are ill-defined over a non-commutative division ring, so
//! the naive spectrum is replaced by the family of **word traces**
//! `Tr(A^p) = sum_i (A^p)_{ii}`. Each `Tr(A^p)` aggregates the closed structural
//! walks of length `p` in `G_phi`, weighted by the ordered (non-commuting)
//! product of their edge quaternions. Unlike a commutative spectral invariant it
//! is not trivialised by a unitary gauge change `A' = U^dagger A U`.
//!
//! # Complexity
//!
//! `O(p * size^3)` exact quaternionic operations for a spectrum up to power `p`.

use crate::algebra::QuaternionicWeight;
use crate::models::incidence_matrix::AlgebraicAdjacencyMatrix;

impl AlgebraicAdjacencyMatrix {
    /// The non-commutative word-trace invariant `Tr(A^p)`.
    ///
    /// Computes the matrix power `A^p` via ordered products and returns the trace
    /// `sum_i (A^p)_{ii}`, an element of `H(Q)`.
    pub fn compute_word_trace(&self, power: usize) -> QuaternionicWeight {
        let current_pow = self.matrix_power(power);

        let mut trace = QuaternionicWeight::zero();
        for (i, row) in current_pow.iter().enumerate() {
            trace = &trace + &row[i];
        }
        trace
    }

    /// The word-trace spectrum `[Tr(A^1), .., Tr(A^max_power)]`.
    ///
    /// Reuses each computed power to obtain the next one, so the whole spectrum
    /// costs the same as a single `Tr(A^{max_power})` plus a trace per level.
    ///
    /// # Panics
    ///
    /// Panics if `max_power == 0`.
    pub fn word_trace_spectrum(&self, max_power: usize) -> Vec<QuaternionicWeight> {
        assert!(max_power >= 1, "max_power must be at least 1");
        let mut spectrum = Vec::with_capacity(max_power);
        let mut current = self.weights.clone();

        loop {
            let mut trace = QuaternionicWeight::zero();
            for (i, row) in current.iter().enumerate() {
                trace = &trace + &row[i];
            }
            spectrum.push(trace);
            if spectrum.len() == max_power {
                break;
            }
            current = Self::multiply(&current, &self.weights, self.size);
        }

        spectrum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::{Clause, Literal};

    fn sample_formula() -> (usize, Vec<Clause>) {
        let clauses = vec![
            Clause::new(vec![
                Literal::new(0, true),
                Literal::new(1, false),
                Literal::new(2, true),
            ]),
            Clause::new(vec![
                Literal::new(0, false),
                Literal::new(1, true),
                Literal::new(2, false),
            ]),
        ];
        (3, clauses)
    }

    #[test]
    fn word_trace_matches_manual_power() {
        let (n, clauses) = sample_formula();
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);

        // Tr(A^1) is the sum of the diagonal: only the two clause units k.
        let t1 = matrix.compute_word_trace(1);
        assert_eq!(t1, QuaternionicWeight::from_integers(0, 0, 0, 2));
    }

    #[test]
    fn spectrum_matches_individual_traces() {
        let (n, clauses) = sample_formula();
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);

        let spectrum = matrix.word_trace_spectrum(4);
        for (idx, value) in spectrum.iter().enumerate() {
            let power = idx + 1;
            assert_eq!(*value, matrix.compute_word_trace(power));
        }
    }

    #[test]
    fn power_two_trace_is_consistent() {
        let (n, clauses) = sample_formula();
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);

        // Independent recomputation of Tr(A^2) = sum_{i,k} A_{ik} A_{ki}.
        let mut expected = QuaternionicWeight::zero();
        for i in 0..matrix.size {
            for k in 0..matrix.size {
                expected = &expected + &(&matrix.weights[i][k] * &matrix.weights[k][i]);
            }
        }
        assert_eq!(matrix.compute_word_trace(2), expected);
    }
}
