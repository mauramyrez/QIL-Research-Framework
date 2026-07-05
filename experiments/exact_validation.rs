//! Exact validation of the published constants.
//!
//! Re-derives, with exact rational arithmetic, the pinned numerical claims of
//! the QIL manuscript and asserts them. A non-zero exit status indicates a
//! discrepancy. This is the machine-checkable companion to the paper's tables.
//!
//! ```text
//! cargo run --example exact_validation
//! ```

use num_bigint::BigInt;
use qil::generators::sat::canonical_sat_clauses;
use qil::generators::unsat::{canonical_unsat_clauses, heterogeneous_unsat_clauses};
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3, QuaternionicWeight};

fn main() {
    let mut failures = 0usize;

    let mut check = |name: &str, condition: bool| {
        if condition {
            println!("  [ok]   {name}");
        } else {
            println!("  [FAIL] {name}");
            failures += 1;
        }
    };

    println!("QIL exact validation");

    // Minimal linear control pair: Nrd(UNSAT) = 2^16, Nrd(SAT) = 2^8 * 13^2.
    let sat_matrix = AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_sat_clauses());
    let unsat_matrix = AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_unsat_clauses());
    let sat_nrd = sat_matrix.dieudonne_reduced_norm().expect("non-singular");
    let unsat_nrd = unsat_matrix.dieudonne_reduced_norm().expect("non-singular");
    check(
        "Nrd(det) UNSAT = 65536 = 2^16",
        unsat_nrd == num_rational::BigRational::from_integer(BigInt::from(65536)),
    );
    check(
        "Nrd(det) SAT = 43264 = 2^8 * 13^2",
        sat_nrd == num_rational::BigRational::from_integer(BigInt::from(43264)),
    );

    // Intrinsic collapse: literal directions vanish (b = c = 0) on incidence.
    let sat_det = sat_matrix
        .compute_dieudonne_determinant()
        .expect("non-singular");
    let collapse_ok = match &sat_det {
        QuaternionicWeight::Zero => true,
        QuaternionicWeight::Value { b, c, .. } => {
            use num_traits::Zero;
            b.is_zero() && c.is_zero()
        }
    };
    check(
        "Commutative Collapse: b = c = 0 on incidence det",
        collapse_ok,
    );

    // Minimal tensor control pair (degree-modulated): UNSAT scalar 4096, SAT
    // entry 2401 + 343(i + j + k) on each orbit slot.
    let unsat_tensor = HypergraphTensor3::build_tensor_from_cnf(3, &canonical_unsat_clauses());
    let sat_tensor = HypergraphTensor3::build_tensor_from_cnf(3, &canonical_sat_clauses());
    check(
        "tensor UNSAT (0,1,2) = 4096 (pure scalar)",
        *unsat_tensor.get(0, 1, 2) == QuaternionicWeight::from_integers(4096, 0, 0, 0),
    );
    check(
        "tensor SAT (0,1,2) = 2401 + 343i + 343j + 343k",
        *sat_tensor.get(0, 1, 2) == QuaternionicWeight::from_integers(2401, 343, 343, 343),
    );

    // Heterogeneous n=4 UNSAT: surviving imaginary components on {0,2,3},{1,2,3}.
    let het = HypergraphTensor3::build_tensor_from_cnf(4, &heterogeneous_unsat_clauses());
    check(
        "heterogeneous UNSAT (0,1,2) = 7920 (pure scalar core)",
        *het.get(0, 1, 2) == QuaternionicWeight::from_integers(7920, 0, 0, 0),
    );
    check(
        "heterogeneous UNSAT (0,2,3) = 660 + 660i + 660j",
        *het.get(0, 2, 3) == QuaternionicWeight::from_integers(660, 660, 660, 0),
    );
    check(
        "heterogeneous UNSAT (1,2,3) = 297 - 297i + 297j + 297k",
        *het.get(1, 2, 3) == QuaternionicWeight::from_integers(297, -297, 297, 297),
    );

    println!();
    if failures == 0 {
        println!("All exact validations passed.");
    } else {
        eprintln!("{failures} validation(s) FAILED.");
        std::process::exit(1);
    }
}
