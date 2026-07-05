//! Reproduce Table 1 of the QIL manuscript: the abelian Frobenius reduced-norm
//! invariant `Phi / m^2` on a size-matched batch (`n = 4`, `m = 10` fixed).
//!
//! ```text
//! cargo run --example reproduce_table_1
//! ```
//!
//! The batch is drawn deterministically from a fixed seed and labelled by the
//! exact solver, so the exact rational values printed here match the manuscript
//! bit-for-bit.

use num_bigint::BigInt;
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
    let scale = BigRational::from(BigInt::from((M * M) as i64));
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
        // Abelian Frobenius reduced-norm invariant, normalized by m^2.
        let normalized = tensor.compute_spectral_invariant() / &scale;
        if satisfiable {
            sat.push(normalized);
        } else {
            unsat.push(normalized);
        }
    }

    sat.sort();
    unsat.sort();

    println!("=== Table 1: abelian Frobenius invariant Phi/m^2 (n = {N}, m = {M}) ===");
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
