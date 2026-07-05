//! The reduced trace `Trd` of `H(Q)`.
//!
//! # Mathematical meaning
//!
//! For `q = a + b*i + c*j + d*k`, the reduced trace is the exact rational
//!
//! ```text
//! Trd(q) = q + conj(q) = 2a in Q.
//! ```
//!
//! It depends only on the scalar (central) part and is invariant under
//! conjugation by units, `Trd(u q u^{-1}) = Trd(q)`. Together with the reduced
//! norm it determines the conjugacy class of a quaternion; this pair
//! `(Trd, Nrd)` is precisely the central datum onto which the Extrinsic Terminal
//! Abelianization theorem collapses the word-trace (see the manuscript and
//! [`crate::collapse::extrinsic`]).
//!
//! # Complexity
//!
//! `O(1)` (a single exact rational doubling).

use crate::algebra::QuaternionicWeight;
use num_rational::BigRational;
use num_traits::Zero;

impl QuaternionicWeight {
    /// The reduced trace `Trd(q) = q + conj(q) = 2a`, an exact rational.
    #[inline]
    pub fn reduced_trace(&self) -> BigRational {
        match self {
            QuaternionicWeight::Zero => BigRational::zero(),
            QuaternionicWeight::Value { a, .. } => a + a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::rational_from_i64;

    #[test]
    fn reduced_trace_is_twice_the_scalar_part() {
        let q = QuaternionicWeight::new(
            rational_from_i64(3),
            rational_from_i64(2),
            rational_from_i64(5),
            rational_from_i64(-7),
        );
        assert_eq!(q.reduced_trace(), rational_from_i64(6));
        assert_eq!(
            QuaternionicWeight::Zero.reduced_trace(),
            rational_from_i64(0)
        );
        // Pure imaginary elements are traceless.
        assert_eq!(
            QuaternionicWeight::i().reduced_trace(),
            rational_from_i64(0)
        );
    }
}
