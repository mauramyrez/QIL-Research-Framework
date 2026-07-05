//! Division-ring structure of `H(Q)`: conjugation, exact inversion, and central
//! scaling.
//!
//! # Mathematical meaning
//!
//! The Hamiltonian quaternions form a division ring: every non-zero element is
//! invertible. The three operations here witness that structure exactly over
//! `Q`:
//!
//! * conjugation `conj(a + b*i + c*j + d*k) = a - b*i - c*j - d*k`, the standard
//!   anti-involution;
//! * inversion `q^{-1} = conj(q) / Nrd(q)`, defined for every `q != 0`;
//! * central scaling `q |-> lambda * q` by a rational `lambda in Q` (the centre).
//!
//! Non-commutativity is essential to the QIL program: it is what prevents the
//! linear gauge collapse of commutative spectral invariants (see the
//! manuscript). Inversion is used by the left-ordered Gaussian elimination that
//! realises the Dieudonne determinant in [`crate::invariants::dieudonne`].
//!
//! # Complexity
//!
//! Each operation is `O(1)` in the number of components with arbitrary-precision
//! rational coefficients. No floating point.

use crate::algebra::QuaternionicWeight;
use num_rational::BigRational;

impl QuaternionicWeight {
    /// The quaternionic conjugate `conj(q) = a - b*i - c*j - d*k`.
    #[inline]
    pub fn conjugate(&self) -> Self {
        match self {
            QuaternionicWeight::Zero => QuaternionicWeight::Zero,
            QuaternionicWeight::Value { a, b, c, d } => QuaternionicWeight::Value {
                a: a.clone(),
                b: -b,
                c: -c,
                d: -d,
            },
        }
    }

    /// The multiplicative inverse `q^{-1} = conj(q) / Nrd(q)`, or [`None`] for
    /// `0`.
    ///
    /// This witnesses that `H(Q)` is a division ring.
    pub fn inverse(&self) -> Option<Self> {
        match self {
            QuaternionicWeight::Zero => None,
            QuaternionicWeight::Value { a, b, c, d } => {
                let norm = a * a + b * b + c * c + d * d;
                // `norm` is strictly positive for any non-zero quaternion.
                Some(QuaternionicWeight::Value {
                    a: a / &norm,
                    b: -b / &norm,
                    c: -c / &norm,
                    d: -d / &norm,
                })
            }
        }
    }

    /// Scale every component by an exact rational `lambda` (central action).
    pub fn scale(&self, lambda: &BigRational) -> Self {
        match self {
            QuaternionicWeight::Zero => QuaternionicWeight::Zero,
            QuaternionicWeight::Value { a, b, c, d } => {
                Self::new(a * lambda, b * lambda, c * lambda, d * lambda)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::rational_from_i64;
    use num_traits::Zero;

    fn r(n: i64) -> BigRational {
        rational_from_i64(n)
    }

    #[test]
    fn multiplicative_inverse_is_exact() {
        let q = QuaternionicWeight::new(r(1), r(2), r(3), r(4));
        let inv = q.inverse().expect("non-zero element is invertible");
        assert_eq!(&q * &inv, QuaternionicWeight::one());
        assert_eq!(&inv * &q, QuaternionicWeight::one());
        assert!(QuaternionicWeight::Zero.inverse().is_none());
    }

    #[test]
    fn scaling_distributes() {
        let q = QuaternionicWeight::new(r(1), r(2), r(3), r(4));
        assert_eq!(
            q.scale(&r(2)),
            QuaternionicWeight::new(r(2), r(4), r(6), r(8))
        );
        assert_eq!(q.scale(&BigRational::zero()), QuaternionicWeight::Zero);
    }
}
