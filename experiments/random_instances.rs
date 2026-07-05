//! Combinatorial stress-testing orchestrator for QIL.
//!
//! Generates exact, satisfiability-labelled 3-CNF datasets at the critical
//! clause/variable ratio and emits, for every instance, the full vector of
//! non-commutative structural invariants as a single JSON object (JSON Lines) on
//! stdout. A human-readable progress trace is written to stderr. The stream is
//! meant to be mined statistically for an explicit obstruction separating SAT
//! from UNSAT.
//!
//! Strict policy: no floating point and no placeholders. The phase-transition
//! ratio `4.26` is encoded exactly as `213/50`, and randomness comes from an
//! integer SplitMix64 stream, so every run is bit-for-bit reproducible.
//!
//! # Usage
//!
//! ```text
//! cargo run --example random_instances -- [samples_per_n] [max_n] [seed]
//! ```

use qil::generators::random_formula::{
    critical_clause_count, is_satisfiable, random_3cnf, SplitMix64,
};
use qil::io::instance_record_json;

/// Smallest variable count for which width-3 clauses with distinct variables
/// are well defined.
const MIN_VARIABLES: usize = 3;
/// Default number of random instances generated per variable count.
const DEFAULT_SAMPLES_PER_N: usize = 5;
/// Default largest variable count in the sweep.
const DEFAULT_MAX_N: usize = 10;
/// Default PRNG seed, chosen so the dataset is reproducible out of the box.
const DEFAULT_SEED: u64 = 0x0123_4567_89AB_CDEF;

fn parse_arguments() -> (usize, usize, u64) {
    let arguments: Vec<String> = std::env::args().collect();
    let samples_per_n = arguments
        .get(1)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&value| value >= 1)
        .unwrap_or(DEFAULT_SAMPLES_PER_N);
    let max_n = arguments
        .get(2)
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&value| value >= MIN_VARIABLES)
        .unwrap_or(DEFAULT_MAX_N);
    let seed = arguments
        .get(3)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_SEED);
    (samples_per_n, max_n, seed)
}

fn main() {
    let (samples_per_n, max_n, seed) = parse_arguments();
    let mut rng = SplitMix64::new(seed);

    eprintln!(
        "QIL random_instances :: n in {MIN_VARIABLES}..={max_n}, {samples_per_n} samples/n, seed={seed}"
    );

    let mut sat_count: usize = 0;
    let mut total: usize = 0;

    for num_variables in MIN_VARIABLES..=max_n {
        let num_clauses = critical_clause_count(num_variables);
        for sample_idx in 0..samples_per_n {
            let clauses = random_3cnf(&mut rng, num_variables, num_clauses);
            let satisfiable = is_satisfiable(num_variables, &clauses);
            let record = instance_record_json(
                num_variables,
                num_clauses,
                sample_idx,
                &clauses,
                satisfiable,
            );
            println!("{record}");

            if satisfiable {
                sat_count += 1;
            }
            total += 1;
        }
        eprintln!("  n={num_variables} m={num_clauses} done ({samples_per_n} instances)");
    }

    eprintln!(
        "complete :: {total} instances, {sat_count} SAT, {} UNSAT",
        total - sat_count
    );
}
