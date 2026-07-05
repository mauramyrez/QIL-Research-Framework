//! `collapse` — the two structural collapse mechanisms of the QIL program.
//!
//! The central thesis of the accompanying manuscript is a structural theory of
//! why local, low-degree non-commutative invariants of 3-SAT fail. Two
//! mechanisms are identified and implemented exactly here:
//!
//! * [`intrinsic`] — the **Intrinsic Bipartite Gauge Collapse**: the
//!   two-colourability of the incidence matrix induces a diagonal gauge
//!   involution that forces the Dieudonne determinant into a commutative
//!   subfield, annihilating the literal commutator torsion `2 b c`.
//! * [`extrinsic`] — the **Extrinsic Terminal Abelianization**: the
//!   reduced-norm readout of the tensor word-trace factors through the
//!   conjugation quotient, collapsing the non-commutative spectrum onto the
//!   central pair `(Trd, Nrd)`.
//! * [`obstruction`] — prime-support diagnostics (Pure-Power Lemma and its
//!   non-scaling), used to probe the incidence invariant as a candidate
//!   obstruction.

pub mod extrinsic;
pub mod intrinsic;
pub mod obstruction;

pub use extrinsic::TerminalProjection;
pub use intrinsic::AnnihilatorOperator;
pub use obstruction::{
    is_unsat_pure_power_of_two, max_odd_prime, sat_prime_evolution, IdealAnalysis,
    ObstructionReport,
};
