//! Prime-support obstruction analysis of the incidence invariant.
//!
//! # Mathematical meaning
//!
//! This module studies the abstract algebraic structure of the Dieudonne
//! determinant as a candidate non-commutative obstruction invariant: the prime
//! factorisation of its reduced norm, its projection onto the commutative
//! quotient `Q[k]`, and how the prime support behaves across a structurally
//! controlled sweep.
//!
//! For the canonical minimal pair the UNSAT reduced norm collapses to a pure
//! power of two (`Nrd = 2^16`) while the SAT one carries an odd prime
//! (`Nrd = 2^8 * 13^2`). The **Pure-Power Lemma** formalises this as an exact
//! predicate; the accompanying sweep shows it does *not* scale (random UNSAT
//! instances at the phase transition carry odd primes), so the clean collapse
//! was a property of the highly symmetric canonical instance rather than a rigid
//! asymptotic obstruction. See the manuscript.
//!
//! Strict policy: every quantity is exact over `BigInt`/`BigRational`.
//!
//! # Complexity
//!
//! Dominated by the Dieudonne determinant (`O(size^3)`) and the trial-division
//! factorisation (`O(sqrt(Nrd))`).

use crate::algebra::ideals::{factorize, project_to_clause_torsion};
use crate::algebra::QuaternionicWeight;
use crate::models::incidence_matrix::AlgebraicAdjacencyMatrix;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

/// The result of analysing the algebraic obstruction carried by an instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdealAnalysis {
    /// The exact reduced norm `Nrd(det_D) = a^2 + b^2 + c^2 + d^2 in Q_{>=0}`.
    pub reduced_norm: BigRational,
    /// Numerator of `reduced_norm` in lowest terms.
    pub nrd_numerator: BigInt,
    /// Denominator of `reduced_norm` in lowest terms (`>= 1`).
    pub nrd_denominator: BigInt,
    /// Prime factorisation `[(p, e), ...]` of `nrd_numerator` (`1` -> empty).
    pub nrd_prime_factors_numerator: Vec<(BigInt, u32)>,
    /// Prime factorisation `[(p, e), ...]` of `nrd_denominator` (`1` -> empty).
    pub nrd_prime_factors_denominator: Vec<(BigInt, u32)>,
    /// The determinant projected onto the commutative subring `Q[k]`.
    pub clause_torsion_projection: QuaternionicWeight,
    /// `true` iff the `i` and `j` components of the determinant already vanish.
    pub is_pure_torsion: bool,
}

impl AlgebraicAdjacencyMatrix {
    /// Analyse whether the quaternionic determinant induces an explicit
    /// obstruction: reduced norm, exact prime factorisation of its numerator and
    /// denominator, projection to the pure-torsion subring `Q[k]`, and whether
    /// the structure forces the literal directions to vanish.
    ///
    /// Returns [`None`] iff the matrix is singular.
    pub fn analyze_algebraic_obstruction(&self) -> Option<IdealAnalysis> {
        let determinant = self.compute_dieudonne_determinant()?;
        let reduced_norm = determinant.norm_squared();

        let nrd_numerator = reduced_norm.numer().clone();
        let nrd_denominator = reduced_norm.denom().clone();
        let nrd_prime_factors_numerator = factorize(&nrd_numerator);
        let nrd_prime_factors_denominator = factorize(&nrd_denominator);

        let clause_torsion_projection = project_to_clause_torsion(&determinant);
        let is_pure_torsion = match &determinant {
            QuaternionicWeight::Zero => true,
            QuaternionicWeight::Value { b, c, .. } => b.is_zero() && c.is_zero(),
        };

        Some(IdealAnalysis {
            reduced_norm,
            nrd_numerator,
            nrd_denominator,
            nrd_prime_factors_numerator,
            nrd_prime_factors_denominator,
            clause_torsion_projection,
            is_pure_torsion,
        })
    }
}

/// A per-dimension report contrasting the prime support of a labelled SAT
/// instance against a labelled UNSAT instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObstructionReport {
    /// The variable count `n` at which the contrast was evaluated.
    pub dimension_n: usize,
    /// `true` iff the UNSAT reduced-norm numerator has prime support in `{2}`.
    pub unsat_conforms_to_pure_2: bool,
    /// The largest odd prime dividing the SAT reduced-norm numerator, or `0`.
    pub max_sat_odd_prime: BigInt,
}

/// Pure-Power predicate: every prime factor of the reduced-norm numerator equals
/// `2`. An empty factorisation (numerator `1 = 2^0`) is treated as conforming.
pub fn is_unsat_pure_power_of_two(analysis: &IdealAnalysis) -> bool {
    let two = BigInt::from(2);
    analysis
        .nrd_prime_factors_numerator
        .iter()
        .all(|(prime, _exponent)| prime == &two)
}

/// The largest odd prime dividing the reduced-norm numerator, or `0` if none.
pub fn max_odd_prime(analysis: &IdealAnalysis) -> BigInt {
    let two = BigInt::from(2);
    analysis
        .nrd_prime_factors_numerator
        .iter()
        .rev()
        .find(|(prime, _exponent)| prime != &two)
        .map(|(prime, _exponent)| prime.clone())
        .unwrap_or_else(|| BigInt::from(0))
}

/// Largest odd prime in the SAT reduced norm per dimension `n` (max across
/// samples). Returns one ascending `(n, max_odd_prime)` entry per dimension.
pub fn sat_prime_evolution(samples: &[(usize, IdealAnalysis)]) -> Vec<(usize, BigInt)> {
    let mut dimensions: Vec<usize> = samples.iter().map(|(dimension, _)| *dimension).collect();
    dimensions.sort_unstable();
    dimensions.dedup();

    dimensions
        .into_iter()
        .map(|dimension| {
            let largest = samples
                .iter()
                .filter(|(sample_dimension, _)| *sample_dimension == dimension)
                .map(|(_, analysis)| max_odd_prime(analysis))
                .max()
                .unwrap_or_else(|| BigInt::from(0));
            (dimension, largest)
        })
        .collect()
}

impl AlgebraicAdjacencyMatrix {
    /// Contrast the prime support of a SAT and an UNSAT instance at a fixed
    /// dimension, reporting whether the minimal-pair divisibility frontier
    /// survives as a scale invariant of the variety.
    pub fn verify_obstruction_scaling(
        dimension: usize,
        sat_analysis: &IdealAnalysis,
        unsat_analysis: &IdealAnalysis,
    ) -> ObstructionReport {
        ObstructionReport {
            dimension_n: dimension,
            unsat_conforms_to_pure_2: is_unsat_pure_power_of_two(unsat_analysis),
            max_sat_odd_prime: max_odd_prime(sat_analysis),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generators::random_formula::{
        critical_clause_count, is_satisfiable, random_3cnf, SplitMix64,
    };
    use crate::models::cnf::{Clause, Literal};
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

    fn satisfiable_instance() -> AlgebraicAdjacencyMatrix {
        let mut clauses = Vec::with_capacity(7);
        for mask in 1u8..8 {
            clauses.push(Clause::new(vec![
                Literal::new(0, mask & 1 == 1),
                Literal::new(1, mask & 2 == 2),
                Literal::new(2, mask & 4 == 4),
            ]));
        }
        AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses)
    }

    fn unsatisfiable_instance() -> AlgebraicAdjacencyMatrix {
        let mut clauses = Vec::with_capacity(8);
        for mask in 0u8..8 {
            clauses.push(Clause::new(vec![
                Literal::new(0, mask & 1 == 1),
                Literal::new(1, mask & 2 == 2),
                Literal::new(2, mask & 4 == 4),
            ]));
        }
        AlgebraicAdjacencyMatrix::build_from_cnf(3, &clauses)
    }

    #[test]
    fn divisibility_frontier_between_sat_and_unsat() {
        let sat = satisfiable_instance()
            .analyze_algebraic_obstruction()
            .expect("SAT instance must be non-singular");
        let unsat = unsatisfiable_instance()
            .analyze_algebraic_obstruction()
            .expect("UNSAT instance must be non-singular");

        for analysis in [&sat, &unsat] {
            assert_eq!(
                product_of_factors(&analysis.nrd_prime_factors_numerator),
                analysis.nrd_numerator
            );
            assert_eq!(
                product_of_factors(&analysis.nrd_prime_factors_denominator),
                analysis.nrd_denominator
            );
            assert_eq!(analysis.reduced_norm.numer(), &analysis.nrd_numerator);
            assert_eq!(analysis.reduced_norm.denom(), &analysis.nrd_denominator);
        }

        assert_ne!(sat.reduced_norm, unsat.reduced_norm);

        // Pinned exact factorisations from the verified computation:
        //   SAT   Nrd = 43264 = 2^8 * 13^2   (prime support {2, 13})
        //   UNSAT Nrd = 65536 = 2^16         (prime support {2})
        assert_eq!(sat.nrd_denominator, BigInt::from(1));
        assert_eq!(unsat.nrd_denominator, BigInt::from(1));
        assert_eq!(
            sat.nrd_prime_factors_numerator,
            vec![(BigInt::from(2), 8), (BigInt::from(13), 2)]
        );
        assert_eq!(
            unsat.nrd_prime_factors_numerator,
            vec![(BigInt::from(2), 16)]
        );

        let sat_has_odd_prime = sat
            .nrd_prime_factors_numerator
            .iter()
            .any(|(prime, _)| prime != &BigInt::from(2));
        let unsat_has_odd_prime = unsat
            .nrd_prime_factors_numerator
            .iter()
            .any(|(prime, _)| prime != &BigInt::from(2));
        assert!(sat_has_odd_prime);
        assert!(!unsat_has_odd_prime);
    }

    /// Base seed for the reproducible search over critical instances.
    const SEARCH_SEED: u64 = 0xDEAD_BEEF_CAFE_F00D;
    /// Upper bound on instances inspected while hunting for a labelled pair.
    const SEARCH_BUDGET: usize = 4096;

    fn labelled_pair(num_variables: usize) -> (IdealAnalysis, IdealAnalysis) {
        let num_clauses = critical_clause_count(num_variables);
        let mut rng = SplitMix64::new(SEARCH_SEED ^ num_variables as u64);

        let mut sat: Option<IdealAnalysis> = None;
        let mut unsat: Option<IdealAnalysis> = None;

        for _ in 0..SEARCH_BUDGET {
            if sat.is_some() && unsat.is_some() {
                break;
            }
            let clauses = random_3cnf(&mut rng, num_variables, num_clauses);
            let satisfiable = is_satisfiable(num_variables, &clauses);
            if satisfiable && sat.is_some() {
                continue;
            }
            if !satisfiable && unsat.is_some() {
                continue;
            }
            let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(num_variables, &clauses);
            if let Some(analysis) = matrix.analyze_algebraic_obstruction() {
                if satisfiable {
                    sat = Some(analysis);
                } else {
                    unsat = Some(analysis);
                }
            }
        }

        (
            sat.expect("a non-singular SAT instance must exist at the critical ratio"),
            unsat.expect("a non-singular UNSAT instance must exist at the critical ratio"),
        )
    }

    #[test]
    fn obstruction_scaling_for_n3_and_n4() {
        let two = BigInt::from(2);
        let zero = BigInt::from(0);
        let mut evolution_samples: Vec<(usize, IdealAnalysis)> = Vec::new();

        for num_variables in [3usize, 4usize] {
            let (sat, unsat) = labelled_pair(num_variables);
            let report =
                AlgebraicAdjacencyMatrix::verify_obstruction_scaling(num_variables, &sat, &unsat);

            assert_eq!(report.dimension_n, num_variables);
            assert_eq!(
                report.unsat_conforms_to_pure_2,
                is_unsat_pure_power_of_two(&unsat)
            );
            assert_eq!(report.max_sat_odd_prime, max_odd_prime(&sat));

            let unsat_has_odd_prime = unsat
                .nrd_prime_factors_numerator
                .iter()
                .any(|(prime, _)| prime != &two);
            assert_eq!(report.unsat_conforms_to_pure_2, !unsat_has_odd_prime);

            if report.max_sat_odd_prime != zero {
                assert_ne!(&report.max_sat_odd_prime % &two, zero);
                assert_eq!(&sat.nrd_numerator % &report.max_sat_odd_prime, zero);
            }

            evolution_samples.push((num_variables, sat));
        }

        let evolution = sat_prime_evolution(&evolution_samples);
        assert_eq!(evolution.len(), 2);
        assert_eq!(evolution[0].0, 3);
        assert_eq!(evolution[1].0, 4);
        for (_, largest_prime) in &evolution {
            assert!(*largest_prime >= zero);
        }
    }
}
