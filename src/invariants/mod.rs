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
//!
//! These are the candidate GCT obstructions the manuscript evaluates; the
//! collapse mechanisms that neutralise them live in [`crate::collapse`].

pub mod dieudonne;
pub mod quaternionic_trace;
pub mod spectral;
pub mod tensor_invariants;

pub use tensor_invariants::WORD_TRACE_POWER;
