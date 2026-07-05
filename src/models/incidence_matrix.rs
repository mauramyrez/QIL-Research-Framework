//! The bipartite clause/variable incidence matrix `A_phi` over `H(Q)`.
//!
//! # Mathematical meaning
//!
//! This is the *linear* (order-2) model of the QIL program. A 3-CNF formula
//! `phi` is embedded into the clause/variable incidence graph `G_phi`, a
//! bipartite graph whose vertex set is
//! `V = {x_0, ..., x_{n-1}} u {C_0, ..., C_{m-1}}`. Variable index `p < n`
//! addresses a variable vertex; index `n + s` addresses clause `C_s`. A
//! positive literal contributes the unit `i`, a negative literal the unit `j`
//! (placed symmetrically), and each clause carries `k` on its diagonal.
//!
//! It is exactly the two-colourability of `G_phi` that induces the diagonal
//! gauge involution behind the **Intrinsic Bipartite Gauge Collapse** (see the
//! manuscript and [`crate::collapse::intrinsic`]). The Dieudonne determinant and
//! word-trace invariants of this matrix live in [`crate::invariants`].
//!
//! # Complexity
//!
//! Storage is dense `size x size` with `size = n + m`. Matrix multiplication is
//! the naive `O(size^3)` ordered product over exact quaternions.
//!
//! # Limitations
//!
//! Dense storage; intended for the small instances (`n <= ~12`) used to probe
//! the collapse mechanisms exactly, not for large-scale SAT solving.

use crate::algebra::QuaternionicWeight;
use crate::models::cnf::Clause;

/// The quaternionic adjacency matrix of the incidence graph `G_phi`.
///
/// Indices `0..n` address variable vertices; indices `n..(n + m)` address clause
/// vertices. The backing storage is a dense `size x size` matrix of
/// [`QuaternionicWeight`], stored exactly with no floating-point approximation.
#[derive(Clone, Debug)]
pub struct AlgebraicAdjacencyMatrix {
    /// Dimension of the matrix, equal to `n_variables + m_clauses`.
    pub size: usize,
    /// Row-major weights; `weights[i][j]` is the edge from vertex `i` to `j`.
    pub weights: Vec<Vec<QuaternionicWeight>>,
}

impl AlgebraicAdjacencyMatrix {
    /// Allocate a zero matrix of the given dimension.
    pub fn new(size: usize) -> Self {
        Self {
            size,
            weights: vec![vec![QuaternionicWeight::zero(); size]; size],
        }
    }

    /// Build the incidence matrix of a CNF formula.
    ///
    /// Variables occupy indices `0..num_variables`; clauses occupy
    /// `num_variables..(num_variables + clauses.len())`. A positive literal
    /// contributes the unit `i`, a negative literal the unit `j`, placed
    /// symmetrically so the (undirected) graph yields a self-adjoint-by-structure
    /// incidence. Each clause vertex carries the unit `k` on its diagonal,
    /// injecting the clause's structural constraint.
    ///
    /// # Panics
    ///
    /// Panics if any literal references a variable index `>= num_variables`.
    pub fn build_from_cnf(num_variables: usize, clauses: &[Clause]) -> Self {
        let m = clauses.len();
        let size = num_variables + m;
        let mut matrix = Self::new(size);

        for (j, clause) in clauses.iter().enumerate() {
            let clause_matrix_idx = num_variables + j;
            for literal in &clause.literals {
                assert!(
                    literal.variable_idx < num_variables,
                    "literal references variable {} but only {} variables were declared",
                    literal.variable_idx,
                    num_variables
                );
                let var_matrix_idx = literal.variable_idx;

                let weight = if literal.is_positive {
                    QuaternionicWeight::i()
                } else {
                    QuaternionicWeight::j()
                };

                matrix.weights[var_matrix_idx][clause_matrix_idx] = weight.clone();
                matrix.weights[clause_matrix_idx][var_matrix_idx] = weight;
            }

            matrix.weights[clause_matrix_idx][clause_matrix_idx] = QuaternionicWeight::k();
        }

        matrix
    }

    /// The number of variable vertices implied by `size` and a clause count.
    #[inline]
    pub fn variable_count(&self, num_clauses: usize) -> usize {
        self.size - num_clauses
    }

    /// Non-commutative matrix product `lhs . rhs` (order-sensitive).
    ///
    /// Shared with the word-trace invariant in
    /// [`crate::invariants::spectral`]; hence `pub(crate)`.
    pub(crate) fn multiply(
        lhs: &[Vec<QuaternionicWeight>],
        rhs: &[Vec<QuaternionicWeight>],
        size: usize,
    ) -> Vec<Vec<QuaternionicWeight>> {
        let mut product = vec![vec![QuaternionicWeight::zero(); size]; size];
        for i in 0..size {
            for j in 0..size {
                let mut sum = QuaternionicWeight::zero();
                for k in 0..size {
                    // Edge quaternions do not commute, so `lhs` must stay on the
                    // left of `rhs` in this ordered product.
                    let prod = &lhs[i][k] * &rhs[k][j];
                    sum = &sum + &prod;
                }
                product[i][j] = sum;
            }
        }
        product
    }

    /// Compute `A^power` as a fresh matrix.
    ///
    /// # Panics
    ///
    /// Panics if `power == 0` (the identity over a non-commutative ring is not
    /// needed by the invariant and is intentionally excluded).
    pub fn matrix_power(&self, power: usize) -> Vec<Vec<QuaternionicWeight>> {
        assert!(power >= 1, "word power must be at least 1");
        let mut current = self.weights.clone();
        for _ in 1..power {
            current = Self::multiply(&current, &self.weights, self.size);
        }
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::Literal;

    /// `(x0 or not x1 or x2) and (not x0 or x1 or not x2)` over 3 variables.
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
    fn incidence_layout_is_correct() {
        let (n, clauses) = sample_formula();
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);
        assert_eq!(matrix.size, 5); // 3 variables + 2 clauses

        // Clause 0 lives at index 3; positive literal x0 -> unit i.
        assert_eq!(matrix.weights[0][3], QuaternionicWeight::i());
        assert_eq!(matrix.weights[3][0], QuaternionicWeight::i());
        // Negative literal not x1 -> unit j.
        assert_eq!(matrix.weights[1][3], QuaternionicWeight::j());
        // Clause diagonal carries the structural unit k.
        assert_eq!(matrix.weights[3][3], QuaternionicWeight::k());
        assert_eq!(matrix.weights[4][4], QuaternionicWeight::k());
    }

    #[test]
    #[should_panic(expected = "literal references variable")]
    fn out_of_range_variable_panics() {
        let clauses = vec![Clause::new(vec![Literal::new(7, true)])];
        AlgebraicAdjacencyMatrix::build_from_cnf(2, &clauses);
    }
}
