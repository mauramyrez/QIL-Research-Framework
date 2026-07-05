//! Worked example of the non-linear (tensor) phase: assembling the order-3
//! tensor, contracting it against the canonical spatial vector, and reading off
//! both the abelian and the non-commutative invariants -- illustrating the
//! Extrinsic Terminal Abelianization.
//!
//! ```text
//! cargo run --example tensor_example
//! ```

use qil::generators::unsat::heterogeneous_unsat_clauses;
use qil::utils::render_weight;
use qil::{HypergraphTensor3, WORD_TRACE_POWER};

fn main() {
    let clauses = heterogeneous_unsat_clauses();
    let m = clauses.len();
    let tensor = HypergraphTensor3::build_tensor_from_cnf(4, &clauses);

    println!("QIL tensor example: heterogeneous UNSAT (n = 4, m = {m})");
    println!();
    println!("Selected non-zero tensor entries (Topological Interlinkage Model):");
    for &(i, j, k) in &[(0usize, 1usize, 2usize), (0, 2, 3), (1, 2, 3)] {
        println!("  T_({i},{j},{k}) = {}", render_weight(tensor.get(i, j, k)));
    }
    println!("  tensor is S_3-symmetric: {}", tensor.is_symmetric());

    // Transverse contraction against V = [i, j, k, 1].
    let v = tensor.canonical_spatial_vector();
    let marginal = tensor.contract_axis(&v);
    println!();
    println!("Contracted marginal matrix M = contract(T, [i,j,k,1]):");
    for i in 0..marginal.dimension {
        let row: Vec<String> = (0..marginal.dimension)
            .map(|j| render_weight(marginal.get(i, j)))
            .collect();
        println!("  [ {} ]", row.join(" , "));
    }

    // Invariants: the abelian Frobenius norm, and the word-trace with its
    // terminal central projection (Trd, Nrd).
    let word_trace = marginal.word_trace(WORD_TRACE_POWER);
    let terminal = tensor.terminal_projection(WORD_TRACE_POWER);
    println!();
    println!(
        "Abelian Frobenius invariant  Phi          = {}",
        tensor.compute_spectral_invariant()
    );
    println!(
        "Non-commutative word-trace   Tr(M^{WORD_TRACE_POWER})     = {}",
        render_weight(&word_trace)
    );
    println!(
        "Terminal central projection  (Trd, Nrd)   = ({}, {})",
        terminal.reduced_trace, terminal.reduced_norm
    );
    println!(
        "Normalized invariant         Nrd/m^2     = {}",
        tensor.compute_normalized_invariant(m)
    );
    println!();
    println!("The terminal reduced norm collapses Tr(M^{WORD_TRACE_POWER}) onto the central pair");
    println!("(Trd, Nrd): the oriented non-commutative content is erased at the readout.");
}
