//! Shared, dependency-light utilities for the Quaternionic Invariant Laboratory
//! (QIL).
//!
//! This module hosts small exact-arithmetic helpers and the typed error surface
//! ([`error`]). Nothing here performs floating-point arithmetic.

use crate::algebra::QuaternionicWeight;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

pub mod error;

pub use error::{QilError, QilResult};

/// Exact promotion of an `i64` into a [`BigRational`] (denominator `1`).
///
/// Used throughout QIL to build quaternionic weights from small integer
/// coefficients without ever touching floating point.
#[inline]
pub fn rational_from_i64(value: i64) -> BigRational {
    BigRational::from(BigInt::from(value))
}

/// Render a quaternionic weight compactly for console/table output.
///
/// Pure scalars collapse to their rational scalar part; any element with a
/// non-zero imaginary component falls back to the full `a + bi + cj + dk`
/// display. This is exact and purely presentational.
pub fn render_weight(weight: &QuaternionicWeight) -> String {
    match weight {
        QuaternionicWeight::Zero => "0".to_string(),
        QuaternionicWeight::Value { a, b, c, d } => {
            if b.is_zero() && c.is_zero() && d.is_zero() {
                a.to_string()
            } else {
                weight.to_string()
            }
        }
    }
}
