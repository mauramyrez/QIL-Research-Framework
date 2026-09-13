//! `invariants` — scalar and quaternionic invariants extracted from the models.
//!
//! * [`dieudonne`] — the Dieudonne determinant of the incidence matrix and its
//!   reduced norm (matrix phase).
//! * [`spectral`] — the non-commutative word-trace spectrum `Tr(A^p)` of the
//!   incidence matrix.
//! * [`quaternionic_trace`] — the ordered matrix word-trace `Tr(M^p)` of the
//!   contracted marginal matrix (tensor phase).
//! * [`tensor_invariants`] — the Frobenius reduced-norm and the normalised
//!   word-trace invariant of the order-3 tensor.
//! * [`clause_overlap`] — the quadratic overlap operator `B^chi` and the
//!   injective eulerian 6-cycle with layer word `(+,-,-,+,-,-)`.
//!
//! These are the candidate local non-commutative invariants the manuscript
//! evaluates; the collapse mechanisms that neutralise them live in
//! [`crate::collapse`].

pub mod clause_overlap;
pub mod dieudonne;
pub mod quaternionic_trace;
pub mod spectral;
pub mod tensor_invariants;

pub use clause_overlap::{
    eval_b_chi, eval_p6_from_injective_and_parity, eval_p6_ie_closed_form, eval_p6_injective,
    eval_parity_residue_linear, overlap_profile, OverlapProfile, LAYER_WORD_P6,
};
pub use tensor_invariants::WORD_TRACE_POWER;
