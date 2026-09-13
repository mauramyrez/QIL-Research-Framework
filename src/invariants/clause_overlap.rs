//! Clause-overlap parity operators: `B^chi` and the injective eulerian 6-cycle.
//!
//! # Mathematical meaning
//!
//! For a 3-CNF formula `phi` and a variable triple `t = {v0, v1, v2}`, write
//! `chi_t(C) = σ_{v0}(C) σ_{v1}(C) σ_{v2}(C)` for every clause `C` whose
//! support equals `t`. The quadratic overlap operator of the conjecture is
//!
//! ```text
//! B^chi(phi) = sum_{t in binom([n],3)} ( sum_{C : supp(C)=t} chi_t(C) )^2.
//! ```
//!
//! Independently, the injective eulerian 6-cycle with layer word
//! `(+,-,-,+,-,-)` sums signed length-6 closed walks that use three distinct
//! clauses and the three variables of a common support triple. On
//! Fourier-balanced cores (vanishing single-variable and pairwise moments) an
//! inclusion-exclusion correction of that cycle against non-injective
//! degeneracies evaluates to the closed form
//!
//! ```text
//! P6_t = -12 m_t + 3 S_t,    S_t = sum_{C:supp(C)=t} chi_t(C),
//! ```
//! hence `P6(phi) = sum_t P6_t`. The parity residue of that expansion is
//! proportional to `S_t`; squaring and summing recovers exactly `B^chi`.

use crate::models::cnf::{Clause, Cnf};
use std::collections::BTreeMap;

/// Layer word `(+,-,-,+,-,-)` along the six edges of the eulerian 6-cycle
/// `C0 — v_a — C1 — v_b — C2 — v_c — C0`.
pub const LAYER_WORD_P6: [i64; 6] = [1, -1, -1, 1, -1, -1];

/// Sign pattern of a width-3 clause on a concrete ordered triple `(v0,v1,v2)`.
fn ordered_signs(clause: &Clause, v0: usize, v1: usize, v2: usize) -> Option<[i64; 3]> {
    Some([
        clause.sign_on(v0)?,
        clause.sign_on(v1)?,
        clause.sign_on(v2)?,
    ])
}

/// `chi_t(C) = σ0 σ1 σ2` for a clause whose support is exactly `{v0,v1,v2}`
/// (any order). Returns `None` if the support does not match.
pub fn chi_on_triple(clause: &Clause, v0: usize, v1: usize, v2: usize) -> Option<i64> {
    let mut support = clause.support();
    let mut target = [v0, v1, v2];
    target.sort_unstable();
    support.sort_unstable();
    if support != target {
        return None;
    }
    let s0 = clause.sign_on(v0)?;
    let s1 = clause.sign_on(v1)?;
    let s2 = clause.sign_on(v2)?;
    Some(s0 * s1 * s2)
}

/// Group clauses by their (sorted) width-3 support triple.
fn clauses_by_triple(formula: &Cnf) -> BTreeMap<[usize; 3], Vec<&Clause>> {
    let mut map: BTreeMap<[usize; 3], Vec<&Clause>> = BTreeMap::new();
    for clause in &formula.clauses {
        let support = clause.support();
        if support.len() != 3 {
            continue;
        }
        let key = [support[0], support[1], support[2]];
        map.entry(key).or_default().push(clause);
    }
    map
}

/// Exact evaluation of the quadratic overlap operator
/// `B^chi(phi) = sum_t (sum_{C:supp(C)=t} chi_t(C))^2`.
pub fn eval_b_chi(formula: &Cnf) -> i64 {
    let mut total = 0i64;
    for (triple, clauses) in clauses_by_triple(formula) {
        let [v0, v1, v2] = triple;
        let mut parity_sum = 0i64;
        for clause in clauses {
            if let Some(chi) = chi_on_triple(clause, v0, v1, v2) {
                parity_sum += chi;
            }
        }
        total += parity_sum * parity_sum;
    }
    total
}

/// Per-triple parity sums `S_t = sum_{C:supp(C)=t} chi_t(C)`.
pub fn parity_sums(formula: &Cnf) -> Vec<([usize; 3], i64, usize)> {
    let mut out = Vec::new();
    for (triple, clauses) in clauses_by_triple(formula) {
        let [v0, v1, v2] = triple;
        let mut parity_sum = 0i64;
        for clause in &clauses {
            if let Some(chi) = chi_on_triple(clause, v0, v1, v2) {
                parity_sum += chi;
            }
        }
        out.push((triple, parity_sum, clauses.len()));
    }
    out
}

/// Layer-weighted used-edge product on one directed 6-cycle realisation.
#[inline]
fn layer_used_weight(signs0: [i64; 3], signs1: [i64; 3], signs2: [i64; 3], va: usize, vb: usize, vc: usize) -> i64 {
    let edges = [
        signs0[va],
        signs1[va],
        signs1[vb],
        signs2[vb],
        signs2[vc],
        signs0[vc],
    ];
    let mut weight = 1i64;
    for (layer, edge) in LAYER_WORD_P6.iter().zip(edges.iter()) {
        weight *= layer * edge;
    }
    weight
}

/// Injective eulerian 6-cycle sum with layer word `(+,-,-,+,-,-)`.
///
/// Sums, over every support triple and every ordered triple of **distinct**
/// clauses on that support, the layer-weighted used-edge products for all
/// `3! = 6` variable permutations. This is the raw injective cycle enumerator
/// (no inclusion-exclusion correction yet).
pub fn eval_p6_injective(formula: &Cnf) -> i64 {
    let mut total = 0i64;
    for (triple, clauses) in clauses_by_triple(formula) {
        let [v0, v1, v2] = triple;
        let patterns: Vec<[i64; 3]> = clauses
            .iter()
            .filter_map(|clause| ordered_signs(clause, v0, v1, v2))
            .collect();
        let m = patterns.len();
        for i in 0..m {
            for j in 0..m {
                if i == j {
                    continue;
                }
                for k in 0..m {
                    if k == i || k == j {
                        continue;
                    }
                    for va in 0..3usize {
                        for vb in 0..3usize {
                            if vb == va {
                                continue;
                            }
                            for vc in 0..3usize {
                                if vc == va || vc == vb {
                                    continue;
                                }
                                total += layer_used_weight(patterns[i], patterns[j], patterns[k], va, vb, vc);
                            }
                        }
                    }
                }
            }
        }
    }
    total
}

/// Theorem-37 IE-corrected 6-cycle functional on Fourier-balanced triples:
/// `P6(phi) = sum_t (-12 m_t + 3 S_t)`.
///
/// On the canonical cube / `Sigma_even` cores the raw injective cycle sum
/// equals `12 m_t` per triple, so this coincides with
/// `-eval_p6_injective(phi) + 3 sum_t S_t`. The second form is returned by
/// [`eval_p6_from_injective_and_parity`] for cross-checks on general instances.
pub fn eval_p6_ie_closed_form(formula: &Cnf) -> i64 {
    parity_sums(formula)
        .into_iter()
        .map(|(_, s_t, m_t)| -12 * (m_t as i64) + 3 * s_t)
        .sum()
}

/// Composite ` -P6_injective + 3 sum_t S_t `.
///
/// Agrees with [`eval_p6_ie_closed_form`] precisely when every active triple is
/// pairwise Fourier-balanced (`Q_{ij}=0` for `i≠j`), as on the canonical pair.
pub fn eval_p6_from_injective_and_parity(formula: &Cnf) -> i64 {
    let injective = eval_p6_injective(formula);
    let three_s: i64 = parity_sums(formula).into_iter().map(|(_, s_t, _)| 3 * s_t).sum();
    -injective + three_s
}

/// Linear parity residue `sum_t S_t` extracted by the IE correction.
pub fn eval_parity_residue_linear(formula: &Cnf) -> i64 {
    parity_sums(formula).into_iter().map(|(_, s_t, _)| s_t).sum()
}

/// Collision / multiplicity profile of clause supports.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OverlapProfile {
    /// Number of distinct occupied triples `|{ t : m_t >= 1 }|`.
    pub occupied_triples: usize,
    /// Number of colliding triples `|{ t : m_t > 1 }|`.
    pub colliding_triples: usize,
    /// Number of triples with multiplicity at least 3 (needed for an injective 6-cycle).
    pub triples_m_ge_3: usize,
    /// Sum of multiplicities `sum_t m_t` (equals `m` for pure width-3 formulas).
    pub sum_multiplicities: usize,
    /// Excess collisions `sum_t max(m_t - 1, 0)`.
    pub collision_excess: usize,
}

/// Support-multiplicity profile used by the asymptotic dilution analysis.
pub fn overlap_profile(formula: &Cnf) -> OverlapProfile {
    let mut profile = OverlapProfile::default();
    for (_triple, clauses) in clauses_by_triple(formula) {
        let m_t = clauses.len();
        profile.occupied_triples += 1;
        profile.sum_multiplicities += m_t;
        if m_t > 1 {
            profile.colliding_triples += 1;
            profile.collision_excess += m_t - 1;
        }
        if m_t >= 3 {
            profile.triples_m_ge_3 += 1;
        }
    }
    profile
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::Literal;

    fn lit(v: usize, pos: bool) -> Literal {
        Literal::new(v, pos)
    }

    fn clause3(s0: bool, s1: bool, s2: bool) -> Clause {
        Clause::new(vec![lit(0, s0), lit(1, s1), lit(2, s2)])
    }

    fn full_cube() -> Cnf {
        let mut clauses = Vec::new();
        for mask in 0u8..8 {
            clauses.push(clause3((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0));
        }
        Cnf::new(3, clauses)
    }

    fn sigma_even_multiplicity_two() -> Cnf {
        // Even chi patterns: +++ , +-- , -+- , --+  (masks 7,1,4,2).
        let mut clauses = Vec::new();
        for mask in [7u8, 1, 4, 2] {
            let c = clause3((mask & 1) != 0, (mask & 2) != 0, (mask & 4) != 0);
            clauses.push(c.clone());
            clauses.push(c);
        }
        Cnf::new(3, clauses)
    }

    #[test]
    fn b_chi_canonical_pair() {
        assert_eq!(eval_b_chi(&full_cube()), 0);
        assert_eq!(eval_b_chi(&sigma_even_multiplicity_two()), 64);
    }

    #[test]
    fn p6_ie_matches_minus72_minus96() {
        let unsat = full_cube();
        let sat = sigma_even_multiplicity_two();
        assert_eq!(eval_p6_ie_closed_form(&unsat), -96);
        assert_eq!(eval_p6_ie_closed_form(&sat), -72);
        // On the canonical pair the injective sum is 12 m = 96, so the
        // composite -P6_inj + 3S agrees with the closed form.
        assert_eq!(eval_p6_injective(&unsat), 96);
        assert_eq!(eval_p6_injective(&sat), 96);
        assert_eq!(eval_p6_from_injective_and_parity(&unsat), -96);
        assert_eq!(eval_p6_from_injective_and_parity(&sat), -72);
    }

    #[test]
    fn parity_residue_square_equals_b_chi() {
        for phi in [full_cube(), sigma_even_multiplicity_two()] {
            let residue_sq: i64 = parity_sums(&phi).into_iter().map(|(_, s, _)| s * s).sum();
            assert_eq!(residue_sq, eval_b_chi(&phi));
        }
    }
}
