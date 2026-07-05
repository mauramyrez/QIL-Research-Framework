//! `algebra` — the exact Hamiltonian division ring `H(Q)` and its ideal-theoretic
//! primitives.
//!
//! This module is the arithmetic core of the Quaternionic Invariant Laboratory
//! (QIL). It defines the quaternionic weight type and the operations that make
//! `H(Q)` a division ring over the exact rationals, split by mathematical
//! concept:
//!
//! * [`quaternion`] — the [`QuaternionicWeight`] type, constructors, and the
//!   non-commutative ring operations (`+`, `-`, `*`, unary `-`).
//! * [`reduced_norm`] — the reduced norm `Nrd`.
//! * [`trace`] — the reduced trace `Trd`.
//! * [`division_ring`] — conjugation, exact inversion, central scaling.
//! * [`ideals`] — integer factorisation and the `H(Q) -> Q[k]` clause-torsion
//!   projection.
//!
//! All arithmetic is exact; there is no floating point anywhere in QIL.

pub mod division_ring;
pub mod ideals;
pub mod quaternion;
pub mod reduced_norm;
pub mod trace;

pub use quaternion::QuaternionicWeight;
