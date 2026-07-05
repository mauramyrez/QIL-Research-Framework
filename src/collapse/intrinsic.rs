//! Intrinsic Bipartite Gauge Collapse: the central annihilation operator `F`.
//!
//! # Mathematical meaning
//!
//! This module measures the first collapse mechanism of the QIL program. It
//! evaluates, exactly, how far the Dieudonne determinant
//! `det_D(A_phi) = a + b i + c j + d k` sits from the centre of `H(Q)` and how
//! the two literal directions couple through the non-commutative product:
//!
//! * **Radical component** -- the squared distance to the centre,
//!   `Nrd(det - a) = b^2 + c^2 + d^2 = Nrd(det) - a^2`;
//! * **Commutator torsion** -- the clause-axis coefficient of the literal
//!   commutator, `[b i, c j] = 2 b c k`, i.e. the exact rational `2 b c`.
//!
//! By the Commutative Collapse Theorem the incidence Dieudonne determinant lies
//! in `Q[k]`, so `b = c = 0` and the torsion `2 b c` vanishes identically on the
//! whole satisfiability variety. This is the *intrinsic* collapse: a symmetry of
//! the object itself, induced by the bipartite two-colouring of `G_phi`. See the
//! manuscript, Sections on the Commutative Collapse Theorem and the taxonomy.
//!
//! # Complexity
//!
//! Dominated by the Dieudonne determinant it consumes (`O(size^3)`).

use crate::algebra::QuaternionicWeight;
use crate::models::incidence_matrix::AlgebraicAdjacencyMatrix;
use num_rational::BigRational;

/// The exact output of the central annihilation operator `F(A_phi)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnnihilatorOperator {
    /// Squared algebraic distance of the determinant to the centre `Q`:
    /// `b^2 + c^2 + d^2`. Zero iff the determinant is central.
    pub radical_component: BigRational,
    /// Clause-axis torsion induced by the literal-direction commutator
    /// `[b i, c j]`, equal to the exact rational `2 b c`.
    pub commutator_torsion: BigRational,
}

/// Decompose a quaternionic weight into its four exact rational components.
fn components(weight: &QuaternionicWeight) -> (BigRational, BigRational, BigRational, BigRational) {
    match weight {
        QuaternionicWeight::Zero => (
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
            BigRational::from_integer(0.into()),
        ),
        QuaternionicWeight::Value { a, b, c, d } => (a.clone(), b.clone(), c.clone(), d.clone()),
    }
}

impl AlgebraicAdjacencyMatrix {
    /// Compute the global analytic annihilation operator `F(A_phi)`.
    ///
    /// Isolates the algebraic components of the Dieudonne determinant and returns
    /// the functional pair `(radical_component, commutator_torsion)` without any
    /// floating-point approximation.
    ///
    /// Returns [`None`] iff the matrix is singular (the determinant is undefined).
    pub fn compute_annihilating_functional(&self) -> Option<AnnihilatorOperator> {
        let determinant = self.compute_dieudonne_determinant()?;
        let (_a, b, c, d) = components(&determinant);

        // Distance^2 to the centre: the reduced norm of the radical part.
        let radical_component = &b * &b + &c * &c + &d * &d;

        // Literal-direction commutator on the clause axis: [b i, c j] = 2 b c k.
        let literal_product = &b * &c;
        let commutator_torsion = &literal_product + &literal_product;

        Some(AnnihilatorOperator {
            radical_component,
            commutator_torsion,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::{Clause, Literal};
    use num_traits::Zero;

    fn satisfiable_instance() -> AlgebraicAdjacencyMatrix {
        // Seven non-zero sign patterns over {x0, x1, x2}: satisfiable, non-singular.
        let mut clauses = Vec::with_capacity(7);
        for mask in 1u8..8 {
            clauses.push(Clause::new(vec![
                Literal::new(0, mask & 1 == 1),
                Literal::new(1, mask & 2 == 2),
                Literal::new(2, mask & 4 == 4),
            ]));
        }
        AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses)
    }

    fn unsatisfiable_instance() -> AlgebraicAdjacencyMatrix {
        // All eight sign patterns: the canonical minimal unsatisfiable 3-CNF.
        let mut clauses = Vec::with_capacity(8);
        for mask in 0u8..8 {
            clauses.push(Clause::new(vec![
                Literal::new(0, mask & 1 == 1),
                Literal::new(1, mask & 2 == 2),
                Literal::new(2, mask & 4 == 4),
            ]));
        }
        AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses)
    }

    #[test]
    fn functional_matches_exact_determinant_components() {
        for matrix in [satisfiable_instance(), unsatisfiable_instance()] {
            let determinant = matrix
                .compute_dieudonne_determinant()
                .expect("non-singular instance");
            let (a, b, c, d) = components(&determinant);
            let operator = matrix
                .compute_annihilating_functional()
                .expect("non-singular instance");

            assert_eq!(operator.radical_component, &b * &b + &c * &c + &d * &d);
            assert_eq!(
                operator.radical_component,
                determinant.norm_squared() - &a * &a
            );
            let two_bc = {
                let bc = &b * &c;
                &bc + &bc
            };
            assert_eq!(operator.commutator_torsion, two_bc);
        }
    }

    #[test]
    fn singular_matrix_has_no_functional() {
        let clauses = vec![Clause::new(vec![
            Literal::new(0, true),
            Literal::new(1, true),
            Literal::new(2, true),
        ])];
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses);
        assert!(matrix.compute_annihilating_functional().is_none());
    }

    #[test]
    fn operator_differentiates_constraint_structure() {
        let sat = satisfiable_instance()
            .compute_annihilating_functional()
            .expect("non-singular");
        let unsat = unsatisfiable_instance()
            .compute_annihilating_functional()
            .expect("non-singular");

        assert_ne!(
            (
                sat.radical_component.clone(),
                sat.commutator_torsion.clone()
            ),
            (
                unsat.radical_component.clone(),
                unsat.commutator_torsion.clone()
            )
        );
    }

    #[test]
    fn commutator_torsion_on_constructed_quaternionic_matrix() {
        // A directly built (non-incidence) quaternionic matrix whose determinant
        // excites both literal directions: det = (1 + i)(1 + j) = 1 + i + j + k.
        let mut matrix = AlgebraicAdjacencyMatrix::new(2);
        matrix.weights[0][0] = QuaternionicWeight::from_integers(1, 1, 0, 0); // 1 + i
        matrix.weights[1][1] = QuaternionicWeight::from_integers(1, 0, 1, 0); // 1 + j

        let determinant = matrix
            .compute_dieudonne_determinant()
            .expect("upper-triangular, non-singular");
        assert_eq!(determinant, QuaternionicWeight::from_integers(1, 1, 1, 1));

        let operator = matrix
            .compute_annihilating_functional()
            .expect("non-singular");
        assert_eq!(
            operator.radical_component,
            BigRational::from_integer(3.into())
        );
        assert_eq!(
            operator.commutator_torsion,
            BigRational::from_integer(2.into())
        );
        assert!(!operator.commutator_torsion.is_zero());
    }

    #[test]
    fn incidence_determinant_is_pure_clause_torsion() {
        // Structural property: the incidence Dieudonne determinant lives in Q[k];
        // the literal directions i, j cancel (b = c = 0), so 2 b c is identically
        // zero and the radical is exactly d^2.
        for matrix in [satisfiable_instance(), unsatisfiable_instance()] {
            let determinant = matrix
                .compute_dieudonne_determinant()
                .expect("non-singular");
            let (_a, b, c, d) = components(&determinant);
            assert!(b.is_zero(), "positive-literal direction must cancel");
            assert!(c.is_zero(), "negative-literal direction must cancel");

            let operator = matrix
                .compute_annihilating_functional()
                .expect("non-singular");
            assert!(operator.commutator_torsion.is_zero());
            assert_eq!(operator.radical_component, &d * &d);
        }
    }
}
