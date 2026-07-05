//! Structural stress tests over a reproducible random corpus.
//!
//! Draws many exactly-labelled critical instances and checks the *structural*
//! predicates the theory guarantees (rather than performance): on every
//! non-singular incidence matrix the Dieudonne determinant collapses into
//! `Q[k]` (literal components `b = c = 0`), and every assembled tensor is
//! `S_3`-symmetric. A non-zero exit status signals a structural violation.
//!
//! ```text
//! cargo run --example stress_tests -- [max_n] [samples_per_n] [seed]
//! ```

use qil::generators::random_formula::{critical_clause_count, random_3cnf, SplitMix64};
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3, QuaternionicWeight};

fn literal_components_vanish(det: &QuaternionicWeight) -> bool {
    match det {
        QuaternionicWeight::Zero => true,
        QuaternionicWeight::Value { b, c, .. } => {
            use num_traits::Zero;
            b.is_zero() && c.is_zero()
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_n = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(9usize);
    let samples = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(20usize);
    let seed = args
        .get(3)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0xF00D_CAFE_u64);

    let mut rng = SplitMix64::new(seed);
    let mut checked_collapse = 0usize;
    let mut checked_symmetry = 0usize;
    let mut violations = 0usize;

    for n in 3..=max_n {
        let m = critical_clause_count(n);
        for _ in 0..samples {
            let clauses = random_3cnf(&mut rng, n, m);

            let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);
            if let Some(det) = matrix.compute_dieudonne_determinant() {
                checked_collapse += 1;
                if !literal_components_vanish(&det) {
                    eprintln!("collapse violation at n={n}: {det}");
                    violations += 1;
                }
            }

            // The tensor requires distinct-variable width-3 clauses, which the
            // generator guarantees; check S_3 symmetry of the assembly.
            let tensor = HypergraphTensor3::build_tensor_from_cnf(n, &clauses);
            checked_symmetry += 1;
            if !tensor.is_symmetric() {
                eprintln!("symmetry violation at n={n}");
                violations += 1;
            }
        }
        eprintln!("  n={n} m={m} done");
    }

    println!(
        "stress complete :: {checked_collapse} collapse checks, {checked_symmetry} symmetry checks, {violations} violations"
    );
    if violations > 0 {
        std::process::exit(1);
    }
}
