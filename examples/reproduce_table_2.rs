//! Reproduce Table 2 of the QIL manuscript: the non-commutative word-trace
//! invariant `Nrd(Tr(M^3)) / m^2` on the same size-matched batch (`n = 4`,
//! `m = 10` fixed) as Table 1.
//!
//! ```text
//! cargo run --example reproduce_table_2
//! ```
//!
//! Because the batch is generated from the identical seed, the two tables refer
//! to the same twelve instances; only the extracted invariant differs.

use num_rational::BigRational;
use qil::generators::random_formula::{is_satisfiable, random_3cnf, SplitMix64};
use qil::HypergraphTensor3;

const N: usize = 4;
const M: usize = 10;
const TARGET_PER_CLASS: usize = 6;
const MAX_SAMPLES: usize = 20_000;
const SWEEP_SEED: u64 = 0xC0FF_EE00_1234_5678;

fn main() {
    let mut rng = SplitMix64::new(SWEEP_SEED);
    let mut sat: Vec<BigRational> = Vec::new();
    let mut unsat: Vec<BigRational> = Vec::new();

    for _ in 0..MAX_SAMPLES {
        if sat.len() >= TARGET_PER_CLASS && unsat.len() >= TARGET_PER_CLASS {
            break;
        }
        let clauses = random_3cnf(&mut rng, N, M);
        let satisfiable = is_satisfiable(N, &clauses);
        if satisfiable && sat.len() >= TARGET_PER_CLASS {
            continue;
        }
        if !satisfiable && unsat.len() >= TARGET_PER_CLASS {
            continue;
        }
        let tensor = HypergraphTensor3::build_tensor_from_cnf(N, &clauses);
        // Non-commutative word-trace invariant Nrd(Tr(M^3)) / m^2.
        let normalized = tensor.compute_normalized_invariant(clauses.len());
        if satisfiable {
            sat.push(normalized);
        } else {
            unsat.push(normalized);
        }
    }

    sat.sort();
    unsat.sort();

    println!("=== Table 2: word-trace invariant Nrd(Tr(M^3))/m^2 (n = {N}, m = {M}) ===");
    println!("  status | normalized spectral invariant (exact rational)");
    println!("  -------+------------------------------------------------");
    for value in &sat {
        println!("  SAT    | {value}");
    }
    for value in &unsat {
        println!("  UNSAT  | {value}");
    }
    println!();
    println!(
        "  collected {} SAT and {} UNSAT at fixed m = {M}",
        sat.len(),
        unsat.len()
    );
}
