//! The transverse contraction operator: order-3 tensor to marginal matrix.
//!
//! # Mathematical meaning
//!
//! To obtain a basis-independent scalar invariant of the order-3 tensor
//! `T_phi`, QIL first contracts one axis against a fixed test vector `V`,
//! producing an induced `n x n` marginal matrix
//!
//! ```text
//! M_{i,j} = sum_{k=0}^{n-1} (T_phi)_{i,j,k} . V_k,
//! ```
//!
//! with each product taken in the fixed Hamiltonian order (tensor entry on the
//! left, test component on the right), so the non-commutative structure is
//! preserved. The canonical spatial test vector is `V = [i, j, k, 1]` for
//! `n = 4`, extended cyclically for other dimensions.
//!
//! Downstream, [`crate::invariants`] extracts scalars from `M` (the abelian
//! Frobenius reduced-norm, and the non-commutative word-trace `Tr(M^p)`), and
//! [`crate::collapse::extrinsic`] shows the terminal reduced norm re-abelianizes
//! the word-trace (the Extrinsic Terminal Abelianization theorem).
//!
//! # Complexity
//!
//! Contraction is `O(n^3)` ordered quaternion products.

use crate::algebra::QuaternionicWeight;
use crate::models::hypergraph_tensor::HypergraphTensor3;

/// An `n x n` quaternionic marginal matrix obtained by contracting the third
/// axis of an [`HypergraphTensor3`] against a test vector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TensorMatrixProjection {
    /// Side length `n`, equal to the dimension of the source tensor.
    pub dimension: usize,
    /// Row-major entries; `entries[i][j]` is the marginal weight `M_{i,j}`.
    pub entries: Vec<Vec<QuaternionicWeight>>,
}

impl TensorMatrixProjection {
    /// Borrow the marginal entry `M_{i,j}`.
    ///
    /// # Panics
    ///
    /// Panics if either coordinate is `>= dimension`.
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> &QuaternionicWeight {
        &self.entries[i][j]
    }
}

impl HypergraphTensor3 {
    /// Contract the third axis of the tensor against a test vector `V`, producing
    /// the induced `n x n` marginal matrix
    ///
    /// ```text
    /// M_{i,j} = sum_{k=0}^{n-1} T_{i,j,k} . V_k.
    /// ```
    ///
    /// The tensor entry stays on the left of each product, so the non-commutative
    /// Hamiltonian order is respected exactly.
    ///
    /// # Panics
    ///
    /// Panics if `vector_v.len() != self.dimension()`.
    pub fn contract_axis(&self, vector_v: &[QuaternionicWeight]) -> TensorMatrixProjection {
        let n = self.dimension();
        assert_eq!(
            vector_v.len(),
            n,
            "test vector length {} must equal tensor dimension {}",
            vector_v.len(),
            n
        );

        let mut entries = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in 0..n {
                let mut sum = QuaternionicWeight::zero();
                for (k, v_k) in vector_v.iter().enumerate() {
                    // Ordered product T_{i,j,k} . V_k (tensor entry on the left).
                    let term = self.get(i, j, k) * v_k;
                    sum = &sum + &term;
                }
                row.push(sum);
            }
            entries.push(row);
        }

        TensorMatrixProjection {
            dimension: n,
            entries,
        }
    }

    /// The canonical spatial test vector `V` of length `n`, cycling through the
    /// quaternionic generators `[i, j, k, 1, i, j, k, 1, ...]`.
    ///
    /// For the `n = 4` benchmark this is exactly `V = [i, j, k, 1]`; the cyclic
    /// extension keeps the construction well defined for any dimension.
    pub fn canonical_spatial_vector(&self) -> Vec<QuaternionicWeight> {
        (0..self.dimension())
            .map(|m| match m % 4 {
                0 => QuaternionicWeight::i(),
                1 => QuaternionicWeight::j(),
                2 => QuaternionicWeight::k(),
                _ => QuaternionicWeight::one(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_axis_rejects_mismatched_vector_length() {
        let tensor = HypergraphTensor3::new(2);
        let bad_vector = vec![QuaternionicWeight::i()]; // length 1, dimension 2
        let result = std::panic::catch_unwind(|| tensor.contract_axis(&bad_vector));
        assert!(result.is_err());
    }

    #[test]
    fn contract_axis_respects_noncommutative_order() {
        // dim 2, single non-zero fibre T_{0,0,0}=i, T_{0,0,1}=j.
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 0, 0, QuaternionicWeight::i());
        tensor.set(0, 0, 1, QuaternionicWeight::j());

        // Contract against V = [i, j]: M_{0,0} = i*i + j*j = -1 + -1 = -2.
        let v = vec![QuaternionicWeight::i(), QuaternionicWeight::j()];
        let matrix = tensor.contract_axis(&v);
        assert_eq!(
            *matrix.get(0, 0),
            QuaternionicWeight::from_integers(-2, 0, 0, 0)
        );
        // The remaining fibres are zero, so their marginals vanish.
        assert!(matrix.get(0, 1).is_zero());
        assert!(matrix.get(1, 0).is_zero());
        assert!(matrix.get(1, 1).is_zero());
    }

    #[test]
    fn canonical_vector_matches_ijk1_for_dimension_four() {
        let tensor = HypergraphTensor3::new(4);
        let v = tensor.canonical_spatial_vector();
        assert_eq!(
            v,
            vec![
                QuaternionicWeight::i(),
                QuaternionicWeight::j(),
                QuaternionicWeight::k(),
                QuaternionicWeight::one(),
            ]
        );
    }
}
