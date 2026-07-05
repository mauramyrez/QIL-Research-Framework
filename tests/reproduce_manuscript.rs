//! Integration tests that reproduce the manuscript's pinned exact values:
//! the minimal linear control pair and the two size-matched invariant tables.

use num_bigint::BigInt;
use num_rational::BigRational;
use qil::generators::random_formula::{is_satisfiable, random_3cnf, SplitMix64};
use qil::generators::sat::canonical_sat_clauses;
use qil::generators::unsat::canonical_unsat_clauses;
use qil::{AlgebraicAdjacencyMatrix, HypergraphTensor3};

const N: usize = 4;
const M: usize = 10;
const TARGET_PER_CLASS: usize = 6;
const MAX_SAMPLES: usize = 20_000;
const SWEEP_SEED: u64 = 0xC0FF_EE00_1234_5678;

fn rat(s: &str) -> BigRational {
    s.parse::<BigRational>().expect("valid rational literal")
}

/// Collect the size-matched batch and apply `invariant` to each instance,
/// returning the ascending-sorted SAT and UNSAT value vectors.
fn sweep(
    invariant: impl Fn(&HypergraphTensor3, usize) -> BigRational,
) -> (Vec<BigRational>, Vec<BigRational>) {
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
        let value = invariant(&tensor, clauses.len());
        if satisfiable {
            sat.push(value);
        } else {
            unsat.push(value);
        }
    }
    sat.sort();
    unsat.sort();
    (sat, unsat)
}

#[test]
fn minimal_linear_control_pair_reduced_norms() {
    let sat = AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_sat_clauses());
    let unsat = AlgebraicAdjacencyMatrix::build_from_cnf(3, &canonical_unsat_clauses());
    assert_eq!(
        unsat.dieudonne_reduced_norm().unwrap(),
        BigRational::from_integer(BigInt::from(65536)) // 2^16
    );
    assert_eq!(
        sat.dieudonne_reduced_norm().unwrap(),
        BigRational::from_integer(BigInt::from(43264)) // 2^8 * 13^2
    );
}

#[test]
fn reproduces_table_1_frobenius_invariant() {
    let scale = BigRational::from(BigInt::from((M * M) as i64));
    let (sat, unsat) = sweep(|tensor, _m| tensor.compute_spectral_invariant() / &scale);

    assert_eq!(
        sat,
        vec![
            rat("13369344/25"),
            rat("3493504/5"),
            rat("780864"),
            rat("20499738/25"),
            rat("20715828/25"),
            rat("27062016/25"),
        ]
    );
    assert_eq!(
        unsat,
        vec![
            rat("8410752/25"),
            rat("13284096/25"),
            rat("14591664/25"),
            rat("3482352/5"),
            rat("18790912/25"),
            rat("19735488/25"),
        ]
    );
}

#[test]
fn reproduces_table_2_word_trace_invariant() {
    let (sat, unsat) = sweep(|tensor, m| tensor.compute_normalized_invariant(m));

    assert_eq!(
        sat,
        vec![
            rat("339620951499636473856/25"),
            rat("672746534417312251904/25"),
            rat("96796381412602574976"),
            rat("4276884660455051577408/25"),
            rat("320669276649881600000"),
            rat("8226579718747975581696/25"),
        ]
    );
    assert_eq!(
        unsat,
        vec![
            rat("7985421406009556992"),
            rat("542248315424444776448/25"),
            rat("818551990853997428736/25"),
            rat("879166583696909139968/25"),
            rat("920134426135328980992/25"),
            rat("1967557322831742959616/25"),
        ]
    );
}
