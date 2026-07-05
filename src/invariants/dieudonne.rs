//! The Dieudonne determinant of the incidence matrix over `H(Q)`.
//!
//! # Mathematical meaning
//!
//! Over a non-commutative division ring the ordinary determinant is undefined;
//! the Dieudonne determinant is the universal multiplicative map to the
//! abelianised unit group, identified for `H(Q)` with `Q_{>0}` through the
//! reduced norm. We fix a concrete representative `det_D(A_phi) in H(Q)` by
//! left-ordered Gaussian elimination and return the ordered product of the
//! diagonal pivots (with a sign flip per row swap).
//!
//! This representative is the object of the **Commutative Collapse Theorem**:
//! for every non-singular incidence matrix it lies in the commutative subfield
//! `Q[k] ~ Q(sqrt(-1))`, so its literal components vanish (`b = c = 0`). Its
//! reduced norm `Nrd(det_D) = a^2 + d^2` is the canonical abelian invariant. See
//! the manuscript and [`crate::collapse::intrinsic`].
//!
//! # Complexity
//!
//! `O(size^3)` exact quaternionic operations (Gaussian elimination), with
//! arbitrary-precision rational coefficients.

use crate::algebra::QuaternionicWeight;
use crate::models::incidence_matrix::AlgebraicAdjacencyMatrix;
use num_rational::BigRational;

impl AlgebraicAdjacencyMatrix {
    /// Exact quaternionic Dieudonne determinant via left-structured Gaussian
    /// elimination over `Q`.
    ///
    /// Reduces the matrix to upper-triangular form using strictly *left*
    /// elimination and returns the ordered product of the resulting diagonal
    /// pivots (with a sign flip recorded per row swap). Returns [`None`] iff the
    /// matrix is singular (an irreducible zero pivot). The determinant of the
    /// empty `0 x 0` matrix is the multiplicative identity.
    pub fn compute_dieudonne_determinant(&self) -> Option<QuaternionicWeight> {
        let n = self.size;
        let mut a = self.weights.clone();
        let mut det = QuaternionicWeight::one();

        for j in 0..n {
            if a[j][j].is_zero() {
                match (j + 1..n).find(|&k| !a[k][j].is_zero()) {
                    Some(pivot_row) => {
                        a.swap(j, pivot_row);
                        // -1 is central, so the sign may be folded in immediately.
                        det = -det;
                    }
                    None => return None,
                }
            }

            // `A[j][j]` is now guaranteed non-zero, hence invertible.
            let pivot_inverse = a[j][j].inverse()?;

            for k in j + 1..n {
                if a[k][j].is_zero() {
                    continue;
                }
                // Left factor f = A[k][j] . A[j][j]^{-1} (order matters).
                let factor = &a[k][j] * &pivot_inverse;
                // Split the matrix so the pivot row `j` and target row `k` can be
                // borrowed simultaneously (k > j, so they are disjoint).
                let (upper, lower) = a.split_at_mut(k);
                let pivot_row = &upper[j];
                let target_row = &mut lower[0];
                for (target, pivot) in target_row[j..n].iter_mut().zip(&pivot_row[j..n]) {
                    let term = &factor * pivot;
                    *target = &*target - &term;
                }
            }

            // Elimination only touches rows below `j`, so `A[j][j]` is the final
            // pivot; multiply it in on the right to preserve word order.
            det = &det * &a[j][j];
        }

        Some(det)
    }

    /// The canonical abelianised Dieudonne invariant: the reduced norm
    /// `Nrd(det) = a^2 + b^2 + c^2 + d^2` of
    /// [`Self::compute_dieudonne_determinant`].
    ///
    /// The raw quaternionic determinant is only well defined up to the commutator
    /// subgroup of `H(Q)*`; its reduced norm is a genuine, basis-independent
    /// rational invariant in `Q_{>0}`. Returns [`None`] for a singular matrix.
    pub fn dieudonne_reduced_norm(&self) -> Option<BigRational> {
        self.compute_dieudonne_determinant()
            .map(|det| det.norm_squared())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dieudonne_of_scaled_identity_is_scalar_product() {
        // diag(2, 3, 5) over the scalar centre: det must be the exact product 30.
        let mut matrix = AlgebraicAdjacencyMatrix::new(3);
        matrix.weights[0][0] = QuaternionicWeight::from_integers(2, 0, 0, 0);
        matrix.weights[1][1] = QuaternionicWeight::from_integers(3, 0, 0, 0);
        matrix.weights[2][2] = QuaternionicWeight::from_integers(5, 0, 0, 0);

        assert_eq!(
            matrix.compute_dieudonne_determinant(),
            Some(QuaternionicWeight::from_integers(30, 0, 0, 0))
        );
    }

    #[test]
    fn dieudonne_of_empty_matrix_is_one() {
        let matrix = AlgebraicAdjacencyMatrix::new(0);
        assert_eq!(
            matrix.compute_dieudonne_determinant(),
            Some(QuaternionicWeight::one())
        );
    }

    #[test]
    fn dieudonne_singular_matrix_is_none() {
        // Column 0 is identically zero -> the matrix is singular.
        let mut matrix = AlgebraicAdjacencyMatrix::new(2);
        matrix.weights[0][1] = QuaternionicWeight::i();
        matrix.weights[1][1] = QuaternionicWeight::j();

        assert_eq!(matrix.compute_dieudonne_determinant(), None);
        assert_eq!(matrix.dieudonne_reduced_norm(), None);
    }

    #[test]
    fn dieudonne_upper_triangular_preserves_word_order() {
        // [[i, k], [0, j]] is already triangular: det = i . j = k (not j . i = -k).
        let mut matrix = AlgebraicAdjacencyMatrix::new(2);
        matrix.weights[0][0] = QuaternionicWeight::i();
        matrix.weights[0][1] = QuaternionicWeight::k();
        matrix.weights[1][1] = QuaternionicWeight::j();

        assert_eq!(
            matrix.compute_dieudonne_determinant(),
            Some(QuaternionicWeight::k())
        );
    }

    #[test]
    fn dieudonne_row_swap_flips_sign() {
        // [[0, i], [j, 0]] forces a single row swap; det evaluates to k.
        let mut matrix = AlgebraicAdjacencyMatrix::new(2);
        matrix.weights[0][1] = QuaternionicWeight::i();
        matrix.weights[1][0] = QuaternionicWeight::j();

        assert_eq!(
            matrix.compute_dieudonne_determinant(),
            Some(QuaternionicWeight::k())
        );
    }

    #[test]
    fn dieudonne_with_nontrivial_left_elimination() {
        // [[i, k], [j, i]] requires a genuine elimination step.
        // Hand computation yields det = -1 + i.
        let mut matrix = AlgebraicAdjacencyMatrix::new(2);
        matrix.weights[0][0] = QuaternionicWeight::i();
        matrix.weights[0][1] = QuaternionicWeight::k();
        matrix.weights[1][0] = QuaternionicWeight::j();
        matrix.weights[1][1] = QuaternionicWeight::i();

        let expected = QuaternionicWeight::from_integers(-1, 1, 0, 0);
        assert_eq!(
            matrix.compute_dieudonne_determinant(),
            Some(expected.clone())
        );
        // The reduced norm is the canonical abelianised invariant: (-1)^2 + 1^2 = 2.
        assert_eq!(
            matrix.dieudonne_reduced_norm(),
            Some(expected.norm_squared())
        );
    }
}
