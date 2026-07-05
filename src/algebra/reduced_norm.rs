//! The reduced norm `Nrd` of `H(Q)`.
//!
//! # Mathematical meaning
//!
//! For `q = a + b*i + c*j + d*k`, the reduced norm is the exact rational
//!
//! ```text
//! Nrd(q) = q * conj(q) = conj(q) * q = a^2 + b^2 + c^2 + d^2 in Q_{>=0}.
//! ```
//!
//! It is the scalar part of `q * conj(q)` and is **multiplicative**:
//! `Nrd(p*q) = Nrd(p) * Nrd(q)`. Because `Q` is formally real, `Nrd(q) = 0` iff
//! `q = 0`, which is exactly the statement that `H(Q)` is a division ring.
//!
//! In the QIL program the reduced norm plays a double role: it is the canonical
//! abelian value of the Dieudonne determinant (matrix phase), and it is the
//! terminal, commutator-quotient projection responsible for the Extrinsic
//! Terminal Abelianization theorem (tensor phase). See the manuscript for both.
//!
//! # Complexity
//!
//! `O(1)` rational multiplications and additions (arbitrary precision).

use crate::algebra::QuaternionicWeight;
use num_rational::BigRational;
use num_traits::Zero;

impl QuaternionicWeight {
    /// The reduced norm `Nrd(q) = a^2 + b^2 + c^2 + d^2`, an exact rational.
    ///
    /// Equals the scalar part of `q * conj(q) = conj(q) * q`. It is
    /// multiplicative: `Nrd(p*q) = Nrd(p) * Nrd(q)`.
    #[inline]
    pub fn norm_squared(&self) -> BigRational {
        match self {
            QuaternionicWeight::Zero => BigRational::zero(),
            QuaternionicWeight::Value { a, b, c, d } => a * a + b * b + c * c + d * d,
        }
    }

    /// Alias for [`QuaternionicWeight::norm_squared`], named after the
    /// mathematical notation `Nrd` used throughout the manuscript.
    #[inline]
    pub fn reduced_norm(&self) -> BigRational {
        self.norm_squared()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::rational_from_i64;

    fn r(n: i64) -> BigRational {
        rational_from_i64(n)
    }

    #[test]
    fn conjugate_and_norm() {
        let q = QuaternionicWeight::new(r(1), r(2), r(3), r(4));
        // q * conj(q) = Nrd(q) (a pure scalar).
        let product = &q * &q.conjugate();
        assert_eq!(product, QuaternionicWeight::scalar(r(30)));
        assert_eq!(q.norm_squared(), r(30));
        assert_eq!(q.reduced_norm(), r(30));
    }

    #[test]
    fn norm_is_multiplicative() {
        let p = QuaternionicWeight::new(r(1), r(1), r(0), r(2));
        let q = QuaternionicWeight::new(r(2), r(-1), r(3), r(1));
        let lhs = (&p * &q).norm_squared();
        let rhs = p.norm_squared() * q.norm_squared();
        assert_eq!(lhs, rhs);
    }
}
