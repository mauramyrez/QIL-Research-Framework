//! The Hamiltonian quaternion `H(Q)`: element type, constructors, and the
//! non-commutative ring operations.
//!
//! # Mathematical meaning
//!
//! [`QuaternionicWeight`] is an element of the Hamiltonian quaternion algebra
//! over the exact rationals,
//!
//! ```text
//! H(Q) = Q + Q*i + Q*j + Q*k,
//! ```
//!
//! with the defining relations
//!
//! ```text
//! i^2 = j^2 = k^2 = ijk = -1,
//! ij =  k,   jk =  i,   ki =  j,
//! ji = -k,   kj = -i,   ik = -j.
//! ```
//!
//! Multiplication is non-commutative (`ij = -ji`), which is the structural
//! feature the QIL program exploits to resist the linear gauge collapse that
//! trivialises commutative spectral invariants (see the manuscript, Section on
//! non-commutativity as structural rigidity). `H(Q)` is a division ring; its
//! reduced norm, inverse and conjugate live in the sibling modules
//! [`crate::algebra::reduced_norm`], [`crate::algebra::division_ring`] and the
//! reduced trace in [`crate::algebra::trace`].
//!
//! # Complexity
//!
//! Every operation is `O(1)` in the number of quaternion components but carries
//! arbitrary-precision rational coefficients, so cost scales with the bit-length
//! of the operands. No floating point is used anywhere.
//!
//! # Limitations
//!
//! Coefficients are exact rationals; there is intentionally no floating-point
//! or fixed-precision fallback.

use crate::utils::rational_from_i64;
use num_rational::BigRational;
use num_traits::Zero;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// An element of the Hamiltonian quaternion algebra over `Q`.
///
/// The dedicated [`QuaternionicWeight::Zero`] variant is a canonical
/// representation of the additive identity. The constructor surface guarantees
/// that a [`QuaternionicWeight::Value`] is **never** the zero quaternion, so
/// structural equality coincides with mathematical equality.
// The `Zero` variant is a deliberate canonical representation of the additive
// identity that lets arithmetic short-circuit and keeps structural equality in
// step with mathematical equality. Boxing the rational fields to equalise the
// variant sizes would add pointer indirection on the arithmetic hot path for no
// real benefit, so the size asymmetry is accepted on purpose.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuaternionicWeight {
    /// The canonical additive identity `0`.
    Zero,
    /// A non-zero quaternion `q = a + b*i + c*j + d*k`.
    Value {
        /// Scalar component (basis `1`).
        a: BigRational,
        /// `i` component — polarity of a positive literal.
        b: BigRational,
        /// `j` component — polarity of a negative literal.
        c: BigRational,
        /// `k` component — structural index of a clause.
        d: BigRational,
    },
}

impl QuaternionicWeight {
    /// The additive identity `0`.
    #[inline]
    pub fn zero() -> Self {
        QuaternionicWeight::Zero
    }

    /// Construct `q = a + b*i + c*j + d*k`, normalising the all-zero element to
    /// the canonical [`QuaternionicWeight::Zero`].
    #[inline]
    pub fn new(a: BigRational, b: BigRational, c: BigRational, d: BigRational) -> Self {
        if a.is_zero() && b.is_zero() && c.is_zero() && d.is_zero() {
            QuaternionicWeight::Zero
        } else {
            QuaternionicWeight::Value { a, b, c, d }
        }
    }

    /// Convenience constructor from four `i64` integer coefficients.
    #[inline]
    pub fn from_integers(a: i64, b: i64, c: i64, d: i64) -> Self {
        Self::new(
            rational_from_i64(a),
            rational_from_i64(b),
            rational_from_i64(c),
            rational_from_i64(d),
        )
    }

    /// The multiplicative identity `1`.
    #[inline]
    pub fn one() -> Self {
        Self::from_integers(1, 0, 0, 0)
    }

    /// A purely scalar quaternion `a + 0*i + 0*j + 0*k`.
    #[inline]
    pub fn scalar(a: BigRational) -> Self {
        let zero = BigRational::zero();
        Self::new(a, zero.clone(), zero.clone(), zero)
    }

    /// The imaginary unit `i` (positive-literal polarity).
    #[inline]
    pub fn i() -> Self {
        Self::from_integers(0, 1, 0, 0)
    }

    /// The imaginary unit `j` (negative-literal polarity).
    #[inline]
    pub fn j() -> Self {
        Self::from_integers(0, 0, 1, 0)
    }

    /// The imaginary unit `k` (clause structural index).
    #[inline]
    pub fn k() -> Self {
        Self::from_integers(0, 0, 0, 1)
    }

    /// Returns `true` iff this element is the additive identity.
    #[inline]
    pub fn is_zero(&self) -> bool {
        matches!(self, QuaternionicWeight::Zero)
    }
}

impl Default for QuaternionicWeight {
    #[inline]
    fn default() -> Self {
        QuaternionicWeight::Zero
    }
}

impl fmt::Display for QuaternionicWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuaternionicWeight::Zero => write!(f, "0"),
            QuaternionicWeight::Value { a, b, c, d } => {
                write!(f, "{a} + {b}i + {c}j + {d}k")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Reference arithmetic: the canonical, allocation-conscious implementations.
// Owned operators below forward to these so there is a single source of truth.
// ---------------------------------------------------------------------------

impl Add<&QuaternionicWeight> for &QuaternionicWeight {
    type Output = QuaternionicWeight;

    fn add(self, rhs: &QuaternionicWeight) -> QuaternionicWeight {
        match (self, rhs) {
            (QuaternionicWeight::Zero, other) | (other, QuaternionicWeight::Zero) => other.clone(),
            (
                QuaternionicWeight::Value {
                    a: a1,
                    b: b1,
                    c: c1,
                    d: d1,
                },
                QuaternionicWeight::Value {
                    a: a2,
                    b: b2,
                    c: c2,
                    d: d2,
                },
            ) => QuaternionicWeight::new(a1 + a2, b1 + b2, c1 + c2, d1 + d2),
        }
    }
}

impl Sub<&QuaternionicWeight> for &QuaternionicWeight {
    type Output = QuaternionicWeight;

    fn sub(self, rhs: &QuaternionicWeight) -> QuaternionicWeight {
        match (self, rhs) {
            (QuaternionicWeight::Zero, QuaternionicWeight::Zero) => QuaternionicWeight::Zero,
            (QuaternionicWeight::Zero, value @ QuaternionicWeight::Value { .. }) => -value.clone(),
            (value @ QuaternionicWeight::Value { .. }, QuaternionicWeight::Zero) => value.clone(),
            (
                QuaternionicWeight::Value {
                    a: a1,
                    b: b1,
                    c: c1,
                    d: d1,
                },
                QuaternionicWeight::Value {
                    a: a2,
                    b: b2,
                    c: c2,
                    d: d2,
                },
            ) => QuaternionicWeight::new(a1 - a2, b1 - b2, c1 - c2, d1 - d2),
        }
    }
}

impl Mul<&QuaternionicWeight> for &QuaternionicWeight {
    type Output = QuaternionicWeight;

    /// Non-commutative Hamiltonian product.
    ///
    /// ```text
    /// i^2 = j^2 = k^2 = -1, ij = k, jk = i, ki = j, ji = -k, kj = -i, ik = -j
    /// ```
    fn mul(self, rhs: &QuaternionicWeight) -> QuaternionicWeight {
        match (self, rhs) {
            (QuaternionicWeight::Zero, _) | (_, QuaternionicWeight::Zero) => {
                QuaternionicWeight::Zero
            }
            (
                QuaternionicWeight::Value {
                    a: a1,
                    b: b1,
                    c: c1,
                    d: d1,
                },
                QuaternionicWeight::Value {
                    a: a2,
                    b: b2,
                    c: c2,
                    d: d2,
                },
            ) => {
                let a = a1 * a2 - b1 * b2 - c1 * c2 - d1 * d2;
                let b = a1 * b2 + b1 * a2 + c1 * d2 - d1 * c2;
                let c = a1 * c2 - b1 * d2 + c1 * a2 + d1 * b2;
                let d = a1 * d2 + b1 * c2 - c1 * b2 + d1 * a2;
                QuaternionicWeight::new(a, b, c, d)
            }
        }
    }
}

impl Neg for &QuaternionicWeight {
    type Output = QuaternionicWeight;

    fn neg(self) -> QuaternionicWeight {
        match self {
            QuaternionicWeight::Zero => QuaternionicWeight::Zero,
            QuaternionicWeight::Value { a, b, c, d } => QuaternionicWeight::Value {
                a: -a,
                b: -b,
                c: -c,
                d: -d,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Owned operators (the ergonomic API surface). Each forwards to the reference
// implementation, so no multiplication logic is duplicated.
// ---------------------------------------------------------------------------

impl Add for QuaternionicWeight {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        &self + &rhs
    }
}

impl Sub for QuaternionicWeight {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        &self - &rhs
    }
}

impl Mul for QuaternionicWeight {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        &self * &rhs
    }
}

impl Neg for QuaternionicWeight {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        -&self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64) -> BigRational {
        rational_from_i64(n)
    }

    #[test]
    fn zero_normalisation() {
        assert_eq!(
            QuaternionicWeight::from_integers(0, 0, 0, 0),
            QuaternionicWeight::Zero
        );
        assert!(QuaternionicWeight::from_integers(0, 0, 0, 0).is_zero());
        assert!(!QuaternionicWeight::i().is_zero());
    }

    #[test]
    fn hamilton_squares_are_minus_one() {
        let neg_one = QuaternionicWeight::from_integers(-1, 0, 0, 0);
        assert_eq!(QuaternionicWeight::i() * QuaternionicWeight::i(), neg_one);
        assert_eq!(QuaternionicWeight::j() * QuaternionicWeight::j(), neg_one);
        assert_eq!(QuaternionicWeight::k() * QuaternionicWeight::k(), neg_one);
    }

    #[test]
    fn hamilton_cyclic_products() {
        let (i, j, k) = (
            QuaternionicWeight::i(),
            QuaternionicWeight::j(),
            QuaternionicWeight::k(),
        );
        assert_eq!(i.clone() * j.clone(), k.clone()); // ij = k
        assert_eq!(j.clone() * k.clone(), i.clone()); // jk = i
        assert_eq!(k.clone() * i.clone(), j.clone()); // ki = j
    }

    #[test]
    fn multiplication_is_non_commutative() {
        let (i, j) = (QuaternionicWeight::i(), QuaternionicWeight::j());
        let ij = i.clone() * j.clone();
        let ji = j * i;
        assert_ne!(ij, ji);
        assert_eq!(ij, -ji); // ij = -ji
    }

    #[test]
    fn anti_commutators_match_spec() {
        let (i, j, k) = (
            QuaternionicWeight::i(),
            QuaternionicWeight::j(),
            QuaternionicWeight::k(),
        );
        assert_eq!(j.clone() * i.clone(), -k.clone()); // ji = -k
        assert_eq!(k.clone() * j.clone(), -i.clone()); // kj = -i
        assert_eq!(i * k, -j); // ik = -j
    }

    #[test]
    fn additive_inverse_and_subtraction() {
        let q = QuaternionicWeight::new(r(3), r(-2), r(5), r(-7));
        assert_eq!(q.clone() + (-q.clone()), QuaternionicWeight::Zero);
        assert_eq!(q.clone() - q, QuaternionicWeight::Zero);
    }
}
