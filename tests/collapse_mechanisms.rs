//! Integration tests for the two collapse mechanisms (boundary theorems).
//!
//! These exercise the public API end-to-end (as an external crate would),
//! checking the Intrinsic Bipartite Gauge Collapse and the Extrinsic Terminal
//! Abelianization on the manuscript's control instances.

use num_traits::Zero;
use qil::generators::sat::canonical_sat_clauses;
use qil::generators::unsat::canonical_unsat_clauses;
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3, QuaternionicWeight, TerminalProjection};

#[test]
fn intrinsic_collapse_annihilates_literal_torsion() {
    // On every incidence instance the Dieudonne determinant lies in Q[k]:
    // the literal components b, c vanish, so the commutator torsion 2bc = 0.
    for clauses in [canonical_sat_clauses(), canonical_unsat_clauses()] {
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses);
        let det = matrix
            .compute_dieudonne_determinant()
            .expect("non-singular control");
        match det {
            QuaternionicWeight::Zero => {}
            QuaternionicWeight::Value { b, c, .. } => {
                assert!(b.is_zero(), "positive-literal direction must cancel");
                assert!(c.is_zero(), "negative-literal direction must cancel");
            }
        }
        let operator = matrix
            .compute_annihilating_functional()
            .expect("non-singular control");
        assert!(operator.commutator_torsion.is_zero());
    }
}

#[test]
fn terminal_abelianization_erases_orientation() {
    // Two distinct quaternions with the same (Trd, Nrd) map to the same terminal
    // central projection: the reduced-norm readout cannot see orientation.
    let along_i = QuaternionicWeight::from_integers(3, 4, 0, 0);
    let along_j = QuaternionicWeight::from_integers(3, 0, 4, 0);
    assert_ne!(along_i, along_j);
    assert_eq!(
        TerminalProjection::of(&along_i),
        TerminalProjection::of(&along_j)
    );
}

#[test]
fn terminal_projection_is_conjugation_invariant() {
    let x = QuaternionicWeight::from_integers(2, -3, 1, 5);
    let u = QuaternionicWeight::from_integers(1, 1, 0, 1);
    let u_inv = u.inverse().expect("unit is invertible");
    let conjugated = &(&u * &x) * &u_inv;
    assert_eq!(
        TerminalProjection::of(&x),
        TerminalProjection::of(&conjugated)
    );
}

#[test]
fn heterogeneous_tensor_retains_noncommutative_structure() {
    // The n=4 heterogeneous UNSAT tensor keeps non-zero imaginary components,
    // so the bipartite gauge collapse does not transport to the tensor model.
    let tensor = HypergraphTensor3::build_tensor_from_cnf(
        4,
        &qil::generators::unsat::heterogeneous_unsat_clauses(),
    );
    let entry = tensor.get(0, 2, 3);
    let has_imaginary = match entry {
        QuaternionicWeight::Zero => false,
        QuaternionicWeight::Value { b, c, d, .. } => !b.is_zero() || !c.is_zero() || !d.is_zero(),
    };
    assert!(
        has_imaginary,
        "heterogeneous UNSAT must retain imaginary structure"
    );
}
