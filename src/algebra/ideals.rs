//! Ideal-theoretic primitives over `Z` and the commutative quotient of `H(Q)`.
//!
//! # Mathematical meaning
//!
//! Two exact operations support the obstruction analysis of
//! [`crate::collapse::obstruction`]:
//!
//! * [`factorize`] — the prime factorisation of an integer by deterministic
//!   trial division. Used to read the prime support of the reduced norm
//!   `Nrd(det_D)` (the Pure-Power Lemma diagnostics).
//! * [`project_to_clause_torsion`] — the non-commutative quotient
//!   `H(Q) -> Q[k]` that eliminates the literal directions `i`, `j`, isolating
//!   the clause-structure axis `k`. The image `Q[k]` (with `k^2 = -1`) is a
//!   commutative field isomorphic to `Q(i)`. By the Commutative Collapse
//!   Theorem this projection is the identity on the incidence Dieudonne
//!   determinant.
//!
//! Strict policy: everything is exact (`BigInt`/`BigRational`), no floating
//! point and no probabilistic primality.
//!
//! # Complexity
//!
//! [`factorize`] is `O(sqrt(n))` trial divisions on arbitrary-precision
//! integers; [`project_to_clause_torsion`] is `O(1)`.

use crate::algebra::QuaternionicWeight;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// Exact prime factorisation of `|value|` by deterministic trial division.
///
/// Returns the multiset of `(prime, exponent)` pairs with primes in ascending
/// order. Units carry no prime content, so `0`, `1` and `-1` yield an empty
/// vector. No floating point and no probabilistic tests are used.
pub fn factorize(value: &BigInt) -> Vec<(BigInt, u32)> {
    let one = BigInt::from(1u32);
    let mut remaining = value.abs();
    let mut factors: Vec<(BigInt, u32)> = Vec::new();
    if remaining <= one {
        return factors;
    }

    // Strip the single even prime first so the main loop can step by two.
    let two = BigInt::from(2u32);
    let mut exponent: u32 = 0;
    while (&remaining % &two).is_zero() {
        remaining = &remaining / &two;
        exponent += 1;
    }
    if exponent > 0 {
        factors.push((two, exponent));
    }

    // Trial-divide by odd candidates up to sqrt(remaining).
    let step = BigInt::from(2u32);
    let mut divisor = BigInt::from(3u32);
    while &divisor * &divisor <= remaining {
        let mut local_exponent: u32 = 0;
        while (&remaining % &divisor).is_zero() {
            remaining = &remaining / &divisor;
            local_exponent += 1;
        }
        if local_exponent > 0 {
            factors.push((divisor.clone(), local_exponent));
        }
        divisor = &divisor + &step;
    }

    // Whatever survives is a prime cofactor larger than its own square root.
    if remaining > one {
        factors.push((remaining, 1));
    }

    factors
}

/// Homotopic projection operator: the non-commutative quotient
/// `H(Q) -> Q[k]` that algebraically eliminates the literal-free directions
/// `i` and `j`, isolating the pure clause torsion carried by `k`.
///
/// Concretely `a + b*i + c*j + d*k` maps to `a + d*k`. The image `Q[k]` (with
/// `k^2 = -1`) is a commutative subring isomorphic to `Q(i)`, so this projection
/// trivialises the literal polarities while retaining the clause-structure axis.
pub fn project_to_clause_torsion(weight: &QuaternionicWeight) -> QuaternionicWeight {
    match weight {
        QuaternionicWeight::Zero => QuaternionicWeight::Zero,
        QuaternionicWeight::Value { a, d, .. } => QuaternionicWeight::new(
            a.clone(),
            BigRational::zero(),
            BigRational::zero(),
            d.clone(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;

    fn product_of_factors(factors: &[(BigInt, u32)]) -> BigInt {
        let mut accumulator = BigInt::one();
        for (prime, exponent) in factors {
            for _ in 0..*exponent {
                accumulator = &accumulator * prime;
            }
        }
        accumulator
    }

    #[test]
    fn factorization_units_have_no_prime_content() {
        assert!(factorize(&BigInt::from(0)).is_empty());
        assert!(factorize(&BigInt::from(1)).is_empty());
        assert!(factorize(&BigInt::from(-1)).is_empty());
    }

    #[test]
    fn factorization_of_composites_and_primes() {
        // 360 = 2^3 * 3^2 * 5.
        let composite = BigInt::from(360);
        assert_eq!(
            factorize(&composite),
            vec![
                (BigInt::from(2), 3),
                (BigInt::from(3), 2),
                (BigInt::from(5), 1),
            ]
        );
        assert_eq!(product_of_factors(&factorize(&composite)), composite);

        // A large prime must survive as a single cofactor.
        let prime = BigInt::from(1_000_003);
        assert_eq!(factorize(&prime), vec![(prime.clone(), 1)]);
    }

    #[test]
    fn factorization_handles_negative_magnitude() {
        let value = BigInt::from(-84); // -(2^2 * 3 * 7)
        let factors = factorize(&value);
        assert_eq!(
            factors,
            vec![
                (BigInt::from(2), 2),
                (BigInt::from(3), 1),
                (BigInt::from(7), 1),
            ]
        );
        assert_eq!(product_of_factors(&factors), BigInt::from(84));
    }

    #[test]
    fn projection_eliminates_literal_directions() {
        let determinant = QuaternionicWeight::from_integers(5, 3, -2, 7);
        let projected = project_to_clause_torsion(&determinant);
        assert_eq!(projected, QuaternionicWeight::from_integers(5, 0, 0, 7));

        // A determinant already in Q[k] is a projection fixed point.
        let pure = QuaternionicWeight::from_integers(1, 0, 0, 4);
        assert_eq!(project_to_clause_torsion(&pure), pure);
    }
}
