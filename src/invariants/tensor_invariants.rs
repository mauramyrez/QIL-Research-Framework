//! Scalar invariants of the order-3 tensor: Frobenius reduced-norm and the
//! normalised non-commutative word-trace invariant.
//!
//! # Mathematical meaning
//!
//! Two scalar reductions of the contracted marginal matrix `M` are provided:
//!
//! * the **Frobenius reduced-norm** `Phi = sum_{i,j} Nrd(M_{i,j})`, an abelian
//!   cell-by-cell aggregate that discards the non-commutative cross-terms;
//! * the **non-commutative word-trace invariant**
//!   `Psi_p = Nrd(Tr(M^p))`, whose ordered matrix power retains non-abelian
//!   interference before the terminal reduced norm.
//!
//! For size normalisation against a formula of `m` clauses we divide by `m^2`.
//! The empirical size-matched sweeps show both invariants interleave the SAT and
//! UNSAT classes; the Extrinsic Terminal Abelianization theorem
//! ([`crate::collapse::extrinsic`]) explains why the terminal `Nrd` re-abelianizes
//! the signal. See the manuscript, Sections on the contraction operator and the
//! Terminal Abelianization theorem.
//!
//! # Complexity
//!
//! `Phi` is `O(n^2)` reduced norms; `Psi_p` is `O(p n^3)` for the word-trace.

use crate::models::contractions::TensorMatrixProjection;
use crate::models::hypergraph_tensor::HypergraphTensor3;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

/// Cyclic word length `p` used for the non-commutative trace `Tr(M^p)`.
///
/// An odd cube (rather than the square) is chosen so the ordered products echo
/// the order-3 structure of the source tensor and retain sign-sensitive,
/// non-abelian interference rather than collapsing to a norm-like quantity.
pub const WORD_TRACE_POWER: usize = 3;

impl TensorMatrixProjection {
    /// The exact Frobenius reduced-norm of the matrix,
    /// `sum_{i,j} Nrd(M_{i,j}) = sum_{i,j} (a^2 + b^2 + c^2 + d^2)`.
    ///
    /// A non-negative exact rational. Note it is a *cell-by-cell* aggregate that
    /// discards the non-commutative cross-terms.
    pub fn reduced_norm_sum(&self) -> BigRational {
        let mut total = BigRational::zero();
        for row in &self.entries {
            for entry in row {
                total += entry.norm_squared();
            }
        }
        total
    }
}

impl HypergraphTensor3 {
    /// Condense the entire order-3 hypergraph structure into a single exact
    /// rational invariant: the Frobenius reduced-norm of the marginal matrix
    /// obtained by contracting against the canonical spatial vector.
    pub fn compute_spectral_invariant(&self) -> BigRational {
        let test_vector = self.canonical_spatial_vector();
        self.contract_axis(&test_vector).reduced_norm_sum()
    }

    /// The **non-commutative word-trace invariant**, normalised by formula
    /// volume.
    ///
    /// Contracts the tensor to the marginal matrix `M`, forms the ordered matrix
    /// power `M^p` (with `p = WORD_TRACE_POWER`), takes the cyclic trace, and
    /// returns `Nrd(Tr(M^p)) / m^2`. Kept exact in `BigRational`. A
    /// `clause_count` of `0` returns `0` rather than dividing by zero.
    pub fn compute_normalized_invariant(&self, clause_count: usize) -> BigRational {
        let test_vector = self.canonical_spatial_vector();
        let projection = self.contract_axis(&test_vector);
        // Absolute rational norm of the non-commutative trace scalar Tr(M^p).
        let raw = projection.word_trace(WORD_TRACE_POWER).norm_squared();
        if clause_count == 0 {
            return BigRational::zero();
        }
        let scale = BigInt::from(clause_count) * BigInt::from(clause_count);
        raw / BigRational::from(scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::QuaternionicWeight;

    #[test]
    fn spectral_invariant_is_exact_reduced_norm_sum() {
        // dim-2 fibre: canonical vector [i, j], only non-zero marginal is
        // M_{0,0} = -2, with Nrd(-2) = 4.
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 0, 0, QuaternionicWeight::i());
        tensor.set(0, 0, 1, QuaternionicWeight::j());

        let invariant = tensor.compute_spectral_invariant();
        assert_eq!(invariant, BigRational::from_integer(4.into()));
    }

    #[test]
    fn spectral_invariant_of_zero_tensor_is_zero() {
        let tensor = HypergraphTensor3::new(3);
        assert_eq!(tensor.compute_spectral_invariant(), BigRational::zero());
    }

    #[test]
    fn normalized_invariant_divides_by_clause_count_squared() {
        // dim-2 fibre: Tr(M^3) = -8, Nrd(-8) = 64. Normalizing by m^2 gives an
        // exact rational: 64 / 2^2 = 16, and 64 / 3^2 = 64/9.
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 0, 0, QuaternionicWeight::i());
        tensor.set(0, 0, 1, QuaternionicWeight::j());

        assert_eq!(
            tensor.compute_normalized_invariant(2),
            BigRational::from_integer(16.into())
        );
        assert_eq!(
            tensor.compute_normalized_invariant(3),
            BigRational::new(64.into(), 9.into())
        );
        // Empty formula guard: no division by zero.
        assert_eq!(tensor.compute_normalized_invariant(0), BigRational::zero());
    }
}
