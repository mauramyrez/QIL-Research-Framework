//! Extrinsic Terminal Abelianization: the central projection of the word-trace.
//!
//! # Mathematical meaning
//!
//! This module formalises the second collapse mechanism of the QIL program.
//! Even when the contracted marginal matrix `M` and its ordered power `M^p`
//! carry genuine non-commutative structure, the scalar readout used to obtain a
//! rational certificate -- the reduced norm -- factors through the
//! abelianisation `H(Q)^x -> Q_{>0}` and is invariant under conjugation by
//! units. By the Terminal Abelianization Theorem the certificate depends on the
//! word-trace `t_p = Tr(M^p)` only through the conjugation-invariant central
//! pair
//!
//! ```text
//! (Trd(t_p), Nrd(t_p)) in Q^2,
//! ```
//!
//! i.e. only through the scalar part and the *isotropic magnitude*
//! `b^2 + c^2 + d^2` of the imaginary part. All oriented (non-commutative)
//! information -- the relative phase among `i, j, k` -- is erased at the readout.
//! This is an *extrinsic* collapse: the object stays non-commutative, but the
//! measurement abelianises it. See the manuscript, Terminal Abelianization
//! Theorem.
//!
//! # Complexity
//!
//! `O(1)` given the word-trace (which itself costs `O(p n^3)`).

use crate::algebra::QuaternionicWeight;
use crate::models::contractions::TensorMatrixProjection;
use crate::models::hypergraph_tensor::HypergraphTensor3;
use num_rational::BigRational;

/// The terminal, conjugation-invariant central projection `(Trd, Nrd)` of a
/// quaternion.
///
/// Two quaternions related by an inner automorphism (equivalently, differing by
/// a rotation of their imaginary triple) share the same [`TerminalProjection`];
/// this is exactly the information a reduced-norm readout can retain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalProjection {
    /// The reduced trace `Trd = 2a`, the (central) scalar datum.
    pub reduced_trace: BigRational,
    /// The reduced norm `Nrd = a^2 + b^2 + c^2 + d^2`, the isotropic magnitude.
    pub reduced_norm: BigRational,
}

impl TerminalProjection {
    /// The central pair `(Trd(q), Nrd(q))` onto which any reduced-norm readout
    /// collapses `q`.
    #[inline]
    pub fn of(weight: &QuaternionicWeight) -> Self {
        Self {
            reduced_trace: weight.reduced_trace(),
            reduced_norm: weight.norm_squared(),
        }
    }
}

impl TensorMatrixProjection {
    /// The terminal projection of the word-trace `Tr(M^power)`: the central pair
    /// `(Trd, Nrd)` that survives an abelianised readout.
    pub fn terminal_word_trace_projection(&self, power: usize) -> TerminalProjection {
        TerminalProjection::of(&self.word_trace(power))
    }
}

impl HypergraphTensor3 {
    /// Convenience: contract against the canonical spatial vector, take the
    /// word-trace `Tr(M^power)`, and return its terminal central projection.
    pub fn terminal_projection(&self, power: usize) -> TerminalProjection {
        let test_vector = self.canonical_spatial_vector();
        self.contract_axis(&test_vector)
            .terminal_word_trace_projection(power)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_is_erased_by_the_central_projection() {
        // Three quaternions along orthogonal imaginary axes: distinct elements,
        // identical (Trd, Nrd) = (6, 25). The terminal projection cannot tell
        // them apart -- the orientation is erased.
        let t = QuaternionicWeight::from_integers(3, 4, 0, 0);
        let t_prime = QuaternionicWeight::from_integers(3, 0, 4, 0);
        let t_double_prime = QuaternionicWeight::from_integers(3, 0, 0, 4);

        assert_ne!(t, t_prime);
        assert_ne!(t_prime, t_double_prime);

        let p = TerminalProjection::of(&t);
        assert_eq!(p, TerminalProjection::of(&t_prime));
        assert_eq!(p, TerminalProjection::of(&t_double_prime));
        assert_eq!(p.reduced_trace, BigRational::from_integer(6.into()));
        assert_eq!(p.reduced_norm, BigRational::from_integer(25.into()));
    }

    #[test]
    fn central_projection_is_conjugation_invariant() {
        // Nrd and Trd are invariant under conjugation by any non-zero unit u.
        let x = QuaternionicWeight::from_integers(2, -3, 1, 5);
        let u = QuaternionicWeight::from_integers(1, 1, 0, 1);
        let u_inv = u.inverse().expect("non-zero unit is invertible");
        let conjugated = &(&u * &x) * &u_inv;

        assert_eq!(
            TerminalProjection::of(&x),
            TerminalProjection::of(&conjugated)
        );
    }

    #[test]
    fn terminal_projection_matches_word_trace() {
        // dim-2 fibre: Tr(M^3) = -8, so (Trd, Nrd) = (-16, 64).
        let mut tensor = HypergraphTensor3::new(2);
        tensor.set(0, 0, 0, QuaternionicWeight::i());
        tensor.set(0, 0, 1, QuaternionicWeight::j());

        let projection = tensor.terminal_projection(3);
        assert_eq!(
            projection.reduced_trace,
            BigRational::from_integer((-16).into())
        );
        assert_eq!(
            projection.reduced_norm,
            BigRational::from_integer(64.into())
        );
    }
}
