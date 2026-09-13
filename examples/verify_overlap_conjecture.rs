//! Verify the overlap conjecture: parity residue of the injective eulerian
//! 6-cycle vs the quadratic operator `B^chi`.
//!
//! ```text
//! cargo run --example verify_overlap_conjecture
//! ```
//!
//! Checks three families of size-matched instances:
//! 1. Canonical pair `(n=3, m=8)`: full cube (UNSAT) vs `Sigma_even` × 2 (SAT).
//! 2. Padded pair `(n=5, m=12)`: the same cores plus a common 4-clause tail.
//! 3. One size-matched random SAT/UNSAT pair at `(n=4, m=10)`.

use qil::generators::random_formula::{is_satisfiable, random_3cnf, SplitMix64};
use qil::generators::sat::control_clause;
use qil::invariants::clause_overlap::{
    eval_b_chi, eval_p6_from_injective_and_parity, eval_p6_ie_closed_form, eval_p6_injective,
    eval_parity_residue_linear, parity_sums,
};
use qil::models::cnf::{Clause, Cnf, Literal};

fn full_cube_unsat() -> Cnf {
    let clauses = (0u8..8)
        .map(|mask| control_clause((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0))
        .collect();
    Cnf::new(3, clauses)
}

/// Satisfiable size-matched mate: the four even-parity sign patterns on
/// `{x0,x1,x2}`, each with multiplicity two (`m = 8`).
fn sigma_even_sat() -> Cnf {
    // chi = +1 masks under bit0→x0, bit1→x1, bit2→x2 with set-bit = positive:
    // 7 (+++), 1 (+--), 4 (-+-), 2 (--+).
    let mut clauses = Vec::with_capacity(8);
    for mask in [7u8, 1, 4, 2] {
        let clause = control_clause((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0);
        clauses.push(clause.clone());
        clauses.push(clause);
    }
    Cnf::new(3, clauses)
}

/// Common independent-ish padding: four clauses on the fresh triple `{2,3,4}`
/// (shares the boundary variable `x2` with the core, never the core triple
/// `{0,1,2}` itself).
fn padding_tail() -> Vec<Clause> {
    vec![
        Clause::new(vec![
            Literal::new(2, true),
            Literal::new(3, true),
            Literal::new(4, true),
        ]),
        Clause::new(vec![
            Literal::new(2, true),
            Literal::new(3, false),
            Literal::new(4, false),
        ]),
        Clause::new(vec![
            Literal::new(2, false),
            Literal::new(3, true),
            Literal::new(4, false),
        ]),
        Clause::new(vec![
            Literal::new(2, false),
            Literal::new(3, false),
            Literal::new(4, true),
        ]),
    ]
}

fn with_padding(core: &Cnf, n_vars: usize) -> Cnf {
    let mut clauses = core.clauses.clone();
    clauses.extend(padding_tail());
    Cnf::new(n_vars, clauses)
}

fn report_instance(label: &str, formula: &Cnf, expected_sat: Option<bool>) {
    let sat = is_satisfiable(formula.n_vars, &formula.clauses);
    let b_chi = eval_b_chi(formula);
    let p6_inj = eval_p6_injective(formula);
    let p6_ie = eval_p6_ie_closed_form(formula);
    let p6_composite = eval_p6_from_injective_and_parity(formula);
    let linear = eval_parity_residue_linear(formula);
    let residue_sq: i64 = parity_sums(formula).into_iter().map(|(_, s, _)| s * s).sum();

    println!("--- {label} ---");
    println!(
        "  n={}, m={}, satisfiable={}{}",
        formula.n_vars,
        formula.num_clauses(),
        sat,
        match expected_sat {
            Some(e) if e == sat => " (as expected)".to_string(),
            Some(e) => format!(" (EXPECTED {e} — LABEL MISMATCH)"),
            None => String::new(),
        }
    );
    println!("  B^chi                         = {b_chi}");
    println!("  P6 injective (used-edge)      = {p6_inj}");
    println!("  P6 IE closed form (-12m+3S)   = {p6_ie}");
    println!("  P6 composite (-P6_inj + 3S)   = {p6_composite}");
    println!("  linear parity residue sum S_t = {linear}");
    println!("  sum_t S_t^2 (parity residue²) = {residue_sq}");
    println!(
        "  residue² == B^chi? {} | IE == composite? {}",
        residue_sq == b_chi,
        p6_ie == p6_composite
    );
    println!();
}

fn report_pair(title: &str, unsat: &Cnf, sat: &Cnf) {
    println!("============================================================");
    println!("{title}");
    println!("============================================================");
    report_instance("UNSAT", unsat, Some(false));
    report_instance("SAT", sat, Some(true));

    let db = eval_b_chi(sat) - eval_b_chi(unsat);
    let dp_ie = eval_p6_ie_closed_form(sat) - eval_p6_ie_closed_form(unsat);
    let dp_inj = eval_p6_injective(sat) - eval_p6_injective(unsat);
    let d_linear = eval_parity_residue_linear(sat) - eval_parity_residue_linear(unsat);

    println!("Pair deltas (SAT − UNSAT):");
    println!("  Δ B^chi              = {db}");
    println!("  Δ P6 IE closed form  = {dp_ie}");
    println!("  Δ P6 injective       = {dp_inj}");
    println!("  Δ linear residue S   = {d_linear}");
    println!();
}

fn sample_random_pair(seed: u64) -> Option<(Cnf, Cnf)> {
    let mut rng = SplitMix64::new(seed);
    let mut sat: Option<Cnf> = None;
    let mut unsat: Option<Cnf> = None;
    for _ in 0..50_000 {
        if sat.is_some() && unsat.is_some() {
            break;
        }
        let clauses = random_3cnf(&mut rng, 4, 10);
        let formula = Cnf::new(4, clauses);
        if is_satisfiable(4, &formula.clauses) {
            if sat.is_none() {
                sat = Some(formula);
            }
        } else if unsat.is_none() {
            unsat = Some(formula);
        }
    }
    match (unsat, sat) {
        (Some(u), Some(s)) => Some((u, s)),
        _ => None,
    }
}

fn main() {
    println!("QIL — verify overlap conjecture (B^chi vs injective 6-cycle)\n");
    println!("Layer word: (+,-,-,+,-,-) = {:?}\n", qil::invariants::LAYER_WORD_P6);

    // 1. Canonical pair (n=3, m=8)
    let unsat = full_cube_unsat();
    let sat = sigma_even_sat();
    report_pair(
        "1) Canonical pair (n=3, m=8): full cube vs Sigma_even × 2",
        &unsat,
        &sat,
    );

    println!("Canonical questions:");
    println!(
        "  B^chi(UNSAT)=0 && B^chi(SAT)=64 ? {}",
        eval_b_chi(&unsat) == 0 && eval_b_chi(&sat) == 64
    );
    println!(
        "  P6 IE gives -96 (UNSAT) and -72 (SAT) ? {}",
        eval_p6_ie_closed_form(&unsat) == -96 && eval_p6_ie_closed_form(&sat) == -72
    );
    println!();

    // 2. Padded pair (n=5, m=12)
    let unsat_pad = with_padding(&unsat, 5);
    let sat_pad = with_padding(&sat, 5);
    report_pair(
        "2) Padded pair (n=5, m=12): canonical cores + common 4-clause tail on {{2,3,4}}",
        &unsat_pad,
        &sat_pad,
    );

    let delta_core = eval_b_chi(&sat) - eval_b_chi(&unsat);
    let delta_pad = eval_b_chi(&sat_pad) - eval_b_chi(&unsat_pad);
    println!("Padding preservation:");
    println!("  Δ B^chi core = {delta_core}, Δ B^chi padded = {delta_pad}, equal? {}", delta_core == delta_pad);
    println!(
        "  Δ P6 IE core = {}, Δ P6 IE padded = {}, equal? {}",
        eval_p6_ie_closed_form(&sat) - eval_p6_ie_closed_form(&unsat),
        eval_p6_ie_closed_form(&sat_pad) - eval_p6_ie_closed_form(&unsat_pad),
        (eval_p6_ie_closed_form(&sat) - eval_p6_ie_closed_form(&unsat))
            == (eval_p6_ie_closed_form(&sat_pad) - eval_p6_ie_closed_form(&unsat_pad))
    );
    println!();

    // Discrepancy analysis on the padded formulas (multiple active triples).
    let lin_u = eval_parity_residue_linear(&unsat_pad);
    let lin_s = eval_parity_residue_linear(&sat_pad);
    let b_u = eval_b_chi(&unsat_pad);
    let b_s = eval_b_chi(&sat_pad);
    println!("Discrepancy check (padded instances, multiple triples):");
    println!("  linear residue S_tot: UNSAT={lin_u}, SAT={lin_s}");
    println!("  (S_tot)^2:            UNSAT={}, SAT={}", lin_u * lin_u, lin_s * lin_s);
    println!("  B^chi = sum S_t^2:    UNSAT={b_u}, SAT={b_s}");
    println!(
        "  (S_tot)^2 == B^chi ? UNSAT={}, SAT={}",
        lin_u * lin_u == b_u,
        lin_s * lin_s == b_s
    );
    println!(
        "  Note: per-triple sum_t S_t^2 equals B^chi by definition; the squared"
    );
    println!(
        "  global linear residue (sum_t S_t)^2 differs whenever ≥2 triples have S_t≠0."
    );
    println!();

    // 3. Random size-matched pair (n=4, m=10)
    println!("============================================================");
    println!("3) Random size-matched pair (n=4, m=10), seed=0xC0FF_EE00_1234_5678");
    println!("============================================================");
    match sample_random_pair(0xC0FF_EE00_1234_5678) {
        Some((rand_unsat, rand_sat)) => {
            report_pair("Random pair", &rand_unsat, &rand_sat);
            println!(
                "  Random pair: raw injective P6 separates? {}",
                eval_p6_injective(&rand_sat) != eval_p6_injective(&rand_unsat)
            );
            println!(
                "  Random pair: B^chi separates? {}",
                eval_b_chi(&rand_sat) != eval_b_chi(&rand_unsat)
            );
        }
        None => println!("  Could not sample a SAT/UNSAT pair within the attempt budget."),
    }

    println!("\nDone.");
}
