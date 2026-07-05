//! Non-commutative matrix word-trace of the contracted marginal matrix.
//!
//! # Mathematical meaning
//!
//! Given the marginal matrix `M` produced by
//! [`crate::models::contractions`], the cyclic word-trace `Tr(M^p)` is formed by
//! **ordered** non-commutative matrix multiplication,
//! `(M^2)_{i,j} = sum_k M_{i,k} M_{k,j}`, followed by the diagonal sum
//! `Tr(M^p) = sum_i (M^p)_{ii} in H(Q)`. Because the power is built from ordered
//! Hamiltonian products, the trace retains the non-abelian interference between
//! coupled hyperedges -- the content a cell-by-cell reduced norm discards.
//!
//! The subsequent terminal reduction of this trace to a rational scalar is where
//! the Extrinsic Terminal Abelianization occurs (see
//! [`crate::collapse::extrinsic`] and [`crate::invariants::tensor_invariants`]).
//!
//! # Complexity
//!
//! `O(p * n^3)` ordered quaternion products for `M^p`.

use crate::algebra::QuaternionicWeight;
use crate::models::contractions::TensorMatrixProjection;

impl TensorMatrixProjection {
    /// Non-commutative matrix product `lhs . rhs` over the Hamiltonian ring.
    ///
    /// Entry multiplication is order-sensitive (`i j != j i`), so `lhs` is always
    /// kept on the left of each term. Shared with the invariant layer; hence
    /// `pub(crate)`.
    pub(crate) fn multiply(
        lhs: &[Vec<QuaternionicWeight>],
        rhs: &[Vec<QuaternionicWeight>],
        n: usize,
    ) -> Vec<Vec<QuaternionicWeight>> {
        let mut product = vec![vec![QuaternionicWeight::zero(); n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = QuaternionicWeight::zero();
                for k in 0..n {
                    // Ordered product: lhs entry stays left of the rhs entry.
                    let term = &lhs[i][k] * &rhs[k][j];
                    sum = &sum + &term;
                }
                product[i][j] = sum;
            }
        }
        product
    }

    /// Compute `M^power` via ordered non-commutative products.
    ///
    /// # Panics
    ///
    /// Panics if `power == 0` (the identity is not meaningful for this invariant).
    pub fn matrix_power(&self, power: usize) -> Vec<Vec<QuaternionicWeight>> {
        assert!(power >= 1, "word-trace power must be at least 1");
        let mut current = self.entries.clone();
        for _ in 1..power {
            current = Self::multiply(&current, &self.entries, self.dimension);
        }
        current
    }

    /// The non-commutative word-trace `Tr(M^power) = sum_i (M^power)_{ii}`.
    ///
    /// The matrix power is formed by ordered Hamiltonian products *before* the
    /// diagonal sum, so the trace retains the non-abelian interference.
    pub fn word_trace(&self, power: usize) -> QuaternionicWeight {
        let powered = self.matrix_power(power);
        let mut trace = QuaternionicWeight::zero();
        for (i, row) in powered.iter().enumerate() {
            trace = &trace + &row[i];
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::hypergraph_tensor::HypergraphTensor3;

    #[test]
    fn word_trace_uses_ordered_matrix_powers() {
        // dim-2 fibre -> M = [[-2, 0], [0, 0]]. Then M^3 = [[-8, 0], [0, 0]] and
        // Tr(M^3) = -8, a pure scalar produced by ordered products.
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 0, 0, QuaternionicWeight::i());
        tensor.set(0, 0, 1, QuaternionicWeight::j());

        let v = vec![QuaternionicWeight::i(), QuaternionicWeight::j()];
        let projection = tensor.contract_axis(&v);
        assert_eq!(
            projection.word_trace(3),
            QuaternionicWeight::from_integers(-8, 0, 0, 0)
        );
    }
}
