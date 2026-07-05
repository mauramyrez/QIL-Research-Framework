//! Criterion micro-benchmarks for the invariant pipelines.
//!
//! Performance only; the exact-arithmetic results are unaffected.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qil::generators::sat::canonical_sat_clauses;
use qil::generators::unsat::heterogeneous_unsat_clauses;
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3};

fn bench_dieudonne(c: &mut Criterion) {
    let clauses = canonical_sat_clauses();
    let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses);
    c.bench_function("dieudonne_reduced_norm_n3", |bencher| {
        bencher.iter(|| black_box(&matrix).dieudonne_reduced_norm())
    });
}

fn bench_tensor_invariant(c: &mut Criterion) {
    let clauses = heterogeneous_unsat_clauses();
    let tensor = HypergraphTensor3::build_tensor_from_cnf(4, &clauses);
    c.bench_function("normalized_invariant_n4", |bencher| {
        bencher.iter(|| black_box(&tensor).compute_normalized_invariant(clauses.len()))
    });
}

criterion_group!(benches, bench_dieudonne, bench_tensor_invariant);
criterion_main!(benches);
