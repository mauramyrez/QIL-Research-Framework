//! # QIL — Quaternionic Invariant Laboratory
//!
//! QIL is an open-source, exact-arithmetic research framework for the study of
//! **non-commutative quaternionic invariants of 3-SAT** and the structural
//! collapse mechanisms that arise in the Geometric Complexity Theory (GCT)
//! program. It is the official laboratory accompanying the manuscript
//! *"Two Collapse Mechanisms for Non-Commutative Invariants of 3-SAT"*
//! (`docs/submission_algorithmica_springer/sn-article.tex`).
//!
//! The framework does not claim to separate `P` from `NP`. It answers a sharper,
//! verifiable question: *why do local, low-degree non-commutative invariants
//! built on quaternionic encodings of 3-SAT structurally fail, even when the
//! obvious gauge symmetries are broken?* QIL implements exactly the two central
//! results:
//!
//! 1. **Intrinsic Bipartite Gauge Collapse** — the two-colourability of the
//!    incidence matrix induces a diagonal gauge involution that forces the
//!    Dieudonne determinant into a commutative subfield
//!    ([`collapse::intrinsic`], [`invariants::dieudonne`]).
//! 2. **Extrinsic Terminal Abelianization** — the reduced-norm readout of the
//!    tensor word-trace factors through the conjugation quotient, collapsing the
//!    non-commutative spectrum onto the central pair `(Trd, Nrd)`
//!    ([`collapse::extrinsic`], [`invariants::tensor_invariants`]).
//!
//! ## Architecture
//!
//! Each module corresponds to a concept of the manuscript:
//!
//! * [`algebra`] — the exact Hamiltonian division ring `H(Q)`.
//! * [`models`] — the incidence matrix (order-2) and the hypergraph tensor
//!   (order-3), plus the transverse contraction operator.
//! * [`invariants`] — Dieudonne determinant, word-trace spectra, and tensor
//!   invariants.
//! * [`collapse`] — the two collapse mechanisms and the prime-support obstruction
//!   diagnostics.
//! * [`generators`] — reproducible CNF generation and exact labelling.
//! * [`io`] — DIMACS parsing and exact JSON/CSV export.
//! * [`utils`] — the typed error surface and shared helpers.
//!
//! ## Exactness
//!
//! Every arithmetic operation is performed over the exact rationals via
//! arbitrary-precision integers (`num-bigint`) and fractions (`num-rational`).
//! There is no floating-point operation anywhere in this crate, so all reduced
//! norms, traces, tensor entries and invariants are exact and reproducible
//! bit-for-bit.
//!
//! ## Reproducing the manuscript
//!
//! ```text
//! cargo run --example reproduce_table_1     # abelian Frobenius invariant sweep
//! cargo run --example reproduce_table_2     # non-commutative word-trace sweep
//! cargo run --example reproduce_figures     # minimal (n=3) and heterogeneous (n=4) contrasts
//! cargo run --example matrix_example        # Dieudonne determinant + torsion
//! cargo run --example tensor_example        # tensor assembly + contraction
//! ```

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![deny(clippy::all)]

pub mod algebra;
pub mod collapse;
pub mod generators;
pub mod invariants;
pub mod io;
pub mod models;
pub mod utils;

// ---------------------------------------------------------------------------
// Convenience re-exports: the most frequently used public API of QIL.
// ---------------------------------------------------------------------------

pub use algebra::QuaternionicWeight;

pub use models::{
    AlgebraicAdjacencyMatrix, CNFClause, Clause, HypergraphTensor3, Literal, TensorMatrixProjection,
};

pub use invariants::WORD_TRACE_POWER;

pub use collapse::{
    is_unsat_pure_power_of_two, max_odd_prime, sat_prime_evolution, AnnihilatorOperator,
    IdealAnalysis, ObstructionReport, TerminalProjection,
};

pub use generators::{
    critical_clause_count, is_satisfiable, random_3cnf, SplitMix64, RATIO_DENOMINATOR,
    RATIO_NUMERATOR,
};

pub use io::{instance_record_json, parse_dimacs};

pub use utils::{QilError, QilResult};
