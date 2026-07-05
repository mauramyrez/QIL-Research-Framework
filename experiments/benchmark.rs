//! Lightweight wall-clock benchmark of the QIL invariant pipelines.
//!
//! Unlike the Criterion micro-benchmarks under `benches/`, this driver reports a
//! single coarse timing per pipeline so a researcher can gauge the cost of the
//! exact-arithmetic computation without extra tooling. Timing does not affect
//! any exact result.
//!
//! ```text
//! cargo run --release --example benchmark
//! ```

use std::time::Instant;

use qil::generators::random_formula::{critical_clause_count, random_3cnf, SplitMix64};
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3};

const SEED: u64 = 0x0BAD_F00D_1234_5678;

fn main() {
    let mut rng = SplitMix64::new(SEED);

    // Dieudonne determinant over a batch of critical n=8 instances.
    let n = 8usize;
    let m = critical_clause_count(n);
    let batch = 64usize;
    let start = Instant::now();
    let mut nonsingular = 0usize;
    for _ in 0..batch {
        let clauses = random_3cnf(&mut rng, n, m);
        let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(n, &clauses);
        if matrix.dieudonne_reduced_norm().is_some() {
            nonsingular += 1;
        }
    }
    let elapsed = start.elapsed();
    println!(
        "Dieudonne reduced-norm: {batch} instances (n={n}, m={m}) in {:?} ({nonsingular} non-singular)",
        elapsed
    );

    // Normalized tensor word-trace invariant over a batch of n=4 instances.
    let n = 4usize;
    let m = 10usize;
    let start = Instant::now();
    let mut acc = 0usize;
    for _ in 0..batch {
        let clauses = random_3cnf(&mut rng, n, m);
        let tensor = HypergraphTensor3::build_tensor_from_cnf(n, &clauses);
        let invariant = tensor.compute_normalized_invariant(clauses.len());
        if invariant != num_rational::BigRational::from_integer(0.into()) {
            acc += 1;
        }
    }
    let elapsed = start.elapsed();
    println!(
        "Normalized word-trace invariant: {batch} instances (n={n}, m={m}) in {:?} ({acc} non-zero)",
        elapsed
    );
}
