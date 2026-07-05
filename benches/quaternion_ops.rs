//! Criterion micro-benchmarks for the core `H(Q)` ring operations.
//!
//! These measure performance only; they never influence the exact-arithmetic
//! results reported in the QIL manuscript.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qil::QuaternionicWeight;

fn bench_multiplication(c: &mut Criterion) {
    let p = QuaternionicWeight::from_integers(3, -2, 5, 7);
    let q = QuaternionicWeight::from_integers(-1, 4, 2, -6);
    c.bench_function("quaternion_mul", |bencher| {
        bencher.iter(|| black_box(&p) * black_box(&q))
    });
}

fn bench_reduced_norm(c: &mut Criterion) {
    let q = QuaternionicWeight::from_integers(11, -13, 17, 19);
    c.bench_function("quaternion_reduced_norm", |bencher| {
        bencher.iter(|| black_box(&q).norm_squared())
    });
}

fn bench_inverse(c: &mut Criterion) {
    let q = QuaternionicWeight::from_integers(2, 3, 5, 7);
    c.bench_function("quaternion_inverse", |bencher| {
        bencher.iter(|| black_box(&q).inverse())
    });
}

criterion_group!(
    benches,
    bench_multiplication,
    bench_reduced_norm,
    bench_inverse
);
criterion_main!(benches);
