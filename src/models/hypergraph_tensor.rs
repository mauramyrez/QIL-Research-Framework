//! The symmetric order-3 adjacency tensor `T_phi` on a 3-uniform hypergraph.
//!
//! # Mathematical meaning
//!
//! This is the *non-linear* (order-3) model of the QIL program. The structural
//! vertices are the Boolean variables themselves; a width-3 clause is a
//! hyperedge `{i, j, k}` on the variable set, and the formula is a 3-uniform
//! hypergraph carried by a symmetric tensor
//!
//! ```text
//! T_phi in (H(Q)^n)^{tensor 3},   (T_phi)_{i,j,k} in H(Q).
//! ```
//!
//! The clause weight uses the **Topological Interlinkage Model**: a
//! sign-independent structural constant plus per-literal spatial polarities on
//! the anticommuting axes, all modulated by a non-local vertex-degree product
//!
//! ```text
//! D = deg(i) deg(j) deg(k),
//! base = D (1 + s_i i + s_j j + s_k k).
//! ```
//!
//! Because the vertices form a single class (no clause-vertex block), the
//! diagonal bipartite gauge involution of the matrix model has no analogue here
//! (see the manuscript, "Inevitability of Trilinear Objects"), and genuine
//! non-commutative structure survives.
//!
//! # Complexity
//!
//! Dense storage of `n^3` exact quaternions; assembly is `O(m)` clause deposits
//! plus an `O(n^3)` symmetrisation sweep.
//!
//! # Limitations
//!
//! Dense `n^3` storage; intended for the small `n` used to probe the collapse
//! mechanisms exactly. The connectivity factor `D` is an `i64` product of
//! degrees, ample for these regimes.

use crate::algebra::QuaternionicWeight;
use num_bigint::BigInt;
use num_rational::BigRational;

/// The CNF clause type consumed by the tensor builder.
///
/// We re-export the shared incidence type from [`crate::models::cnf`] rather
/// than defining a parallel one, so both models can be driven from the same
/// generator for head-to-head benchmark contrasts.
pub use crate::models::cnf::Clause as CNFClause;

/// A symmetric order-3 adjacency tensor `T_phi in (H(Q)^n)^{tensor 3}` on the
/// variables of a CNF formula, modelled as a 3-uniform hypergraph.
///
/// Storage is a dense, row-major `n x n x n` block of exact quaternionic
/// weights (length `n^3`), addressed through
/// [`HypergraphTensor3::linear_index`]. Symmetry (invariance under the full
/// permutation group `S_3` on the three index slots) is established explicitly
/// by [`HypergraphTensor3::symmetrize`] and checked by
/// [`HypergraphTensor3::is_symmetric`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HypergraphTensor3 {
    /// Number of structural vertices `n`, i.e. the count of boolean variables.
    dimension: usize,
    /// Flattened `n^3` entries in row-major order: the weight `T_{i,j,k}` lives
    /// at offset `((i * n) + j) * n + k`.
    entries: Vec<QuaternionicWeight>,
}

impl HypergraphTensor3 {
    /// Allocate the zero tensor of the given vertex dimension `n`.
    ///
    /// All `n^3` entries are the canonical [`QuaternionicWeight::Zero`].
    pub fn new(dimension: usize) -> Self {
        let cells = dimension
            .checked_pow(3)
            .expect("order-3 tensor dimension n^3 overflows usize");
        Self {
            dimension,
            entries: vec![QuaternionicWeight::zero(); cells],
        }
    }

    /// The vertex dimension `n` (number of boolean variables).
    #[inline]
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Row-major flattening of a `(i, j, k)` triple into the backing store.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is `>= dimension`.
    #[inline]
    fn linear_index(&self, i: usize, j: usize, k: usize) -> usize {
        let n = self.dimension;
        assert!(
            i < n && j < n && k < n,
            "tensor index ({i}, {j}, {k}) out of bounds for dimension {n}"
        );
        (i * n + j) * n + k
    }

    /// Borrow the entry `T_{i,j,k}`.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is out of bounds.
    #[inline]
    pub fn get(&self, i: usize, j: usize, k: usize) -> &QuaternionicWeight {
        &self.entries[self.linear_index(i, j, k)]
    }

    /// Overwrite the entry `T_{i,j,k}`.
    ///
    /// This is a raw single-slot write and does **not** preserve permutation
    /// symmetry; call [`HypergraphTensor3::symmetrize`] afterwards to restore it.
    ///
    /// # Panics
    ///
    /// Panics if any coordinate is out of bounds.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, k: usize, value: QuaternionicWeight) {
        let idx = self.linear_index(i, j, k);
        self.entries[idx] = value;
    }

    /// The six index permutations of an `(i, j, k)` triple under `S_3`.
    #[inline]
    fn s3_orbit(i: usize, j: usize, k: usize) -> [(usize, usize, usize); 6] {
        [
            (i, j, k),
            (i, k, j),
            (j, i, k),
            (j, k, i),
            (k, i, j),
            (k, j, i),
        ]
    }

    /// Build the symmetric order-3 tensor `T_phi` of a 3-CNF formula via the
    /// **Topological Interlinkage Model**, treating each width-3 clause as a
    /// hyperedge on its variables.
    ///
    /// A pre-pass computes the global vertex degree map `deg(v)` (literal
    /// occurrences of `v` across the formula, independent of polarity). Each
    /// clause on variables `(i, j, k)` with signs `(s_i, s_j, s_k)` is scaled by
    /// the connectivity factor `D = deg(i) deg(j) deg(k)` and deposited as the
    /// multi-vector `D (1 + s_i i + s_j j + s_k k)` across all six `S_3`
    /// permutations, after which a closing [`HypergraphTensor3::symmetrize`]
    /// sweep enforces exact `S_3` invariance. All arithmetic is exact.
    ///
    /// # Panics
    ///
    /// * Panics if any literal references a variable index `>= variables_count`.
    /// * Panics if any clause does not have exactly three literals: the tensor is
    ///   an invariant of a *3-uniform* hypergraph.
    pub fn build_tensor_from_cnf(variables_count: usize, clauses: &[CNFClause]) -> Self {
        // Initialise the dense n^3 store with the canonical zero weight.
        let mut tensor = Self::new(variables_count);

        // Pre-pass: the global vertex degree map. `degrees[v]` counts every
        // literal occurrence of variable `v` across the entire formula,
        // irrespective of polarity. This also validates vertex indices.
        let mut degrees = vec![0i64; variables_count];
        for clause in clauses {
            for literal in &clause.literals {
                assert!(
                    literal.variable_idx < variables_count,
                    "literal references variable {} but only {} variables were declared",
                    literal.variable_idx,
                    variables_count
                );
                degrees[literal.variable_idx] += 1;
            }
        }

        for clause in clauses {
            assert_eq!(
                clause.literals.len(),
                3,
                "3-uniform hypergraph tensor requires width-3 clauses, found a clause of width {}",
                clause.literals.len()
            );

            // Extract the three variable indices and their sign polarities.
            let (i, j, k) = (
                clause.literals[0].variable_idx,
                clause.literals[1].variable_idx,
                clause.literals[2].variable_idx,
            );
            let literal_sign_i = if clause.literals[0].is_positive {
                1i64
            } else {
                -1i64
            };
            let literal_sign_j = if clause.literals[1].is_positive {
                1i64
            } else {
                -1i64
            };
            let literal_sign_k = if clause.literals[2].is_positive {
                1i64
            } else {
                -1i64
            };

            // Non-linear connectivity modulation: scale the spatial + constant
            // multi-vector by the product of the three vertex degrees.
            // base = D * (1 + s_i i + s_j j + s_k k).
            let degree_product = degrees[i] * degrees[j] * degrees[k];
            let base_weight = QuaternionicWeight::from_integers(
                degree_product,
                literal_sign_i * degree_product,
                literal_sign_j * degree_product,
                literal_sign_k * degree_product,
            );

            // Scatter the weight across all six S_3 permutations, accumulating so
            // that repeated hyperedges reinforce one another. The flat offset is
            // ((i*n)+j)*n+k, computed inside `linear_index`.
            for (pi, pj, pk) in Self::s3_orbit(i, j, k) {
                let idx = tensor.linear_index(pi, pj, pk);
                let updated = &tensor.entries[idx] + &base_weight;
                tensor.entries[idx] = updated;
            }
        }

        // Enforce exact S_3 invariance. For distinct-index orbits this is the
        // identity (all six slots already agree); it also normalises orbits with
        // repeated indices consistently.
        tensor.symmetrize();
        tensor
    }

    /// Trilinear `S_3` symmetrisation: replace every entry `T_{i,j,k}` by the
    /// average of its value over the six permutations in `S_3`,
    ///
    /// ```text
    /// T^sym_{i,j,k} = (1/6) sum_{sigma in S_3} T_{sigma(i),sigma(j),sigma(k)}.
    /// ```
    ///
    /// The scaling factor `1/6` is exact rational arithmetic (`BigRational`), so
    /// the result is a genuine `S_3`-invariant tensor with no floating-point
    /// error. After this call [`HypergraphTensor3::is_symmetric`] returns `true`.
    pub fn symmetrize(&mut self) {
        let n = self.dimension;
        // Exact central scalar 1/6 used to average the six permutations.
        let one_sixth = BigRational::new(BigInt::from(1), BigInt::from(6));

        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let orbit = Self::s3_orbit(i, j, k);

                    let mut sum = QuaternionicWeight::zero();
                    for &(pi, pj, pk) in &orbit {
                        sum = &sum + self.get(pi, pj, pk);
                    }
                    let averaged = sum.scale(&one_sixth);

                    for &(pi, pj, pk) in &orbit {
                        self.set(pi, pj, pk, averaged.clone());
                    }
                }
            }
        }
    }

    /// Returns `true` iff the tensor is invariant under every permutation in
    /// `S_3`, i.e. `T_{i,j,k}` is constant across each index orbit.
    pub fn is_symmetric(&self) -> bool {
        let n = self.dimension;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let base = self.get(i, j, k);
                    for &(pi, pj, pk) in &Self::s3_orbit(i, j, k) {
                        if self.get(pi, pj, pk) != base {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::Literal;

    #[test]
    fn new_allocates_zero_tensor() {
        let tensor = HypergraphTensor3::new(3);
        assert_eq!(tensor.dimension(), 3);
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    assert!(tensor.get(i, j, k).is_zero());
                }
            }
        }
        assert!(tensor.is_symmetric());
    }

    #[test]
    fn set_and_get_roundtrip() {
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 1, 1, QuaternionicWeight::i());
        assert_eq!(*tensor.get(0, 1, 1), QuaternionicWeight::i());
        assert!(tensor.get(1, 0, 1).is_zero());
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn out_of_bounds_access_panics() {
        let tensor = HypergraphTensor3::new(2);
        let _ = tensor.get(2, 0, 0);
    }

    #[test]
    fn symmetrize_averages_a_single_seeded_entry() {
        let mut tensor = HypergraphTensor3::new(3);
        tensor.set(0, 1, 2, QuaternionicWeight::from_integers(6, 0, 0, 0));
        tensor.symmetrize();

        let one = QuaternionicWeight::one();
        for &(i, j, k) in &[
            (0usize, 1usize, 2usize),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 0),
            (2, 0, 1),
            (2, 1, 0),
        ] {
            assert_eq!(*tensor.get(i, j, k), one);
        }
        assert!(tensor.is_symmetric());
    }

    #[test]
    fn symmetrize_is_idempotent() {
        let mut tensor = HypergraphTensor3::new(3);
        tensor.set(0, 0, 1, QuaternionicWeight::from_integers(3, 0, 0, 0));
        tensor.set(2, 1, 0, QuaternionicWeight::k());
        tensor.symmetrize();
        let once = tensor.clone();
        tensor.symmetrize();
        assert_eq!(tensor, once);
    }

    #[test]
    fn build_tensor_preserves_spatial_polarity_axes() {
        // Clause (x0 or not x1 or x2): signs (+1, -1, +1) on distinct axes plus
        // the structural constant give 1 + i - j + k, deposited symmetrically
        // across the whole {0,1,2} orbit (degree 1 each, D = 1).
        let clauses = vec![CNFClause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
            Literal::new(2, true),
        ])];
        let tensor = HypergraphTensor3::build_tensor_from_cnf(3, &clauses);
        assert_eq!(tensor.dimension(), 3);

        let expected = QuaternionicWeight::from_integers(1, 1, -1, 1);
        for &(i, j, k) in &[
            (0usize, 1usize, 2usize),
            (0, 2, 1),
            (1, 0, 2),
            (1, 2, 0),
            (2, 0, 1),
            (2, 1, 0),
        ] {
            assert_eq!(*tensor.get(i, j, k), expected);
        }
        assert!(tensor.is_symmetric());
        assert!(tensor.get(0, 0, 1).is_zero());
    }

    #[test]
    fn build_tensor_all_positive_is_spatial_sum() {
        let clauses = vec![CNFClause::new(vec![
            Literal::new(0, true),
            Literal::new(1, true),
            Literal::new(2, true),
        ])];
        let tensor = HypergraphTensor3::build_tensor_from_cnf(3, &clauses);
        assert_eq!(
            *tensor.get(1, 2, 0),
            QuaternionicWeight::from_integers(1, 1, 1, 1)
        );
    }

    #[test]
    fn build_tensor_accumulates_shared_hyperedges() {
        // Two identical clauses on triple {0,1,2}. Each variable now has degree 2,
        // so D = 2*2*2 = 8. Per clause the base weight is 8*(1 + i - j + k) =
        // 8 + 8i - 8j + 8k; accumulated over both clauses the orbit holds
        // 16 + 16i - 16j + 16k.
        let clause = CNFClause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
            Literal::new(2, true),
        ]);
        let clauses = vec![clause.clone(), clause];
        let tensor = HypergraphTensor3::build_tensor_from_cnf(3, &clauses);
        assert_eq!(
            *tensor.get(0, 1, 2),
            QuaternionicWeight::from_integers(16, 16, -16, 16)
        );
        assert!(tensor.is_symmetric());
    }

    #[test]
    #[should_panic(expected = "requires width-3 clauses")]
    fn build_tensor_rejects_non_width_three_clause() {
        let clauses = vec![CNFClause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
        ])];
        HypergraphTensor3::build_tensor_from_cnf(3, &clauses);
    }

    #[test]
    #[should_panic(expected = "literal references variable")]
    fn build_tensor_from_cnf_rejects_out_of_range_variable() {
        let clauses = vec![CNFClause::new(vec![Literal::new(9, true)])];
        HypergraphTensor3::build_tensor_from_cnf(3, &clauses);
    }
}
