//! Reproduce the numeric data behind the manuscript's contrast figures: the
//! minimal `n = 3` control pair (Fourier hypercube annihilation) and the
//! degree-heterogeneous `n = 4` control pair (surviving non-commutative
//! structure).
//!
//! ```text
//! cargo run --example reproduce_figures
//! ```

use num_rational::BigRational;
use num_traits::Zero;
use qil::generators::sat::{canonical_sat_clauses, heterogeneous_sat_clauses};
use qil::generators::unsat::{canonical_unsat_clauses, heterogeneous_unsat_clauses};
use qil::utils::render_weight;
use qil::HypergraphTensor3;

fn squared_frobenius_distance(a: &HypergraphTensor3, b: &HypergraphTensor3) -> BigRational {
    let n = a.dimension();
    let mut total = BigRational::zero();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let delta = a.get(i, j, k) - b.get(i, j, k);
                total += delta.norm_squared();
            }
        }
    }
    total
}

fn main() {
    // ---- Minimal control pair (n = 3) ----
    let sat3 = HypergraphTensor3::build_tensor_from_cnf(3, &canonical_sat_clauses());
    let unsat3 = HypergraphTensor3::build_tensor_from_cnf(3, &canonical_unsat_clauses());

    println!("=== minimal tensor contrast (n = 3) ===");
    println!("  (i,j,k) |            SAT |          UNSAT");
    println!("  --------+----------------+---------------");
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                let s = render_weight(sat3.get(i, j, k));
                let u = render_weight(unsat3.get(i, j, k));
                if s != "0" || u != "0" {
                    println!("  ({i},{j},{k}) | {s:>14} | {u:>14}");
                }
            }
        }
    }
    println!(
        "  squared Frobenius distance = {} (exact rational)",
        squared_frobenius_distance(&sat3, &unsat3)
    );

    // ---- Heterogeneous control pair (n = 4) ----
    let sat4 = HypergraphTensor3::build_tensor_from_cnf(4, &heterogeneous_sat_clauses());
    let unsat4 = HypergraphTensor3::build_tensor_from_cnf(4, &heterogeneous_unsat_clauses());

    println!();
    println!("=== heterogeneous tensor contrast (n = 4) ===");
    println!("  UNSAT entries with surviving non-zero imaginary components:");
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let w = unsat4.get(i, j, k);
                let has_imaginary = match w {
                    qil::QuaternionicWeight::Zero => false,
                    qil::QuaternionicWeight::Value { b, c, d, .. } => {
                        !b.is_zero() || !c.is_zero() || !d.is_zero()
                    }
                };
                if has_imaginary {
                    println!("    ({i},{j},{k}) = {}", render_weight(w));
                }
            }
        }
    }
    println!(
        "  SAT spectral invariant   = {}",
        sat4.compute_spectral_invariant()
    );
    println!(
        "  UNSAT spectral invariant = {}",
        unsat4.compute_spectral_invariant()
    );
    println!(
        "  squared Frobenius distance = {} (exact rational)",
        squared_frobenius_distance(&sat4, &unsat4)
    );
}
