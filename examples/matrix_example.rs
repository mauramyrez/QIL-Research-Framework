//! Worked example of the linear (matrix) phase: the Dieudonne determinant, its
//! reduced norm and prime support, and the annihilated literal commutator
//! torsion, on the minimal `n = 3` control pair.
//!
//! ```text
//! cargo run --example matrix_example
//! ```

use qil::generators::sat::canonical_sat_clauses;
use qil::generators::unsat::canonical_unsat_clauses;
use qil::utils::render_weight;
use qil::AlgebraicAdjacencyMatrix;

fn report(label: &str, matrix: &AlgebraicAdjacencyMatrix) {
    println!("--- {label} ---");
    let det = matrix
        .compute_dieudonne_determinant()
        .expect("non-singular control instance");
    println!("  Dieudonne determinant : {}", render_weight(&det));
    println!(
        "  reduced norm Nrd(det) : {}",
        matrix.dieudonne_reduced_norm().expect("non-singular")
    );

    let operator = matrix
        .compute_annihilating_functional()
        .expect("non-singular");
    println!("  radical b^2+c^2+d^2   : {}", operator.radical_component);
    println!("  commutator torsion 2bc: {}", operator.commutator_torsion);

    let analysis = matrix
        .analyze_algebraic_obstruction()
        .expect("non-singular");
    println!(
        "  Nrd prime factors     : {:?}",
        analysis.nrd_prime_factors_numerator
    );
    println!(
        "  literal torsion pure? : b = c = 0 -> {}",
        analysis.is_pure_torsion
    );
}

fn main() {
    println!("QIL matrix example: Intrinsic Bipartite Gauge Collapse (n = 3)");
    println!();
    report(
        "SAT (7 clauses, all patterns but all-negative)",
        &AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_sat_clauses()),
    );
    println!();
    report(
        "UNSAT (8 clauses, every sign pattern)",
        &AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_unsat_clauses()),
    );
    println!();
    println!("Note: on both incidence instances the literal directions cancel (b = c = 0),");
    println!("so the commutator torsion 2bc vanishes -- the Commutative Collapse Theorem.");
}
