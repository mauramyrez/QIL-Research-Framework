//! Exact serialisation of instances and invariants to JSON Lines and CSV.
//!
//! # Mathematical meaning
//!
//! Reproducible datasets are the backbone of the QIL reproducibility story. This
//! module renders CNF instances and their exact quaternionic invariants as
//! machine-readable text: every quaternion is emitted as its four exact rational
//! components (as strings, to preserve arbitrary precision), and every reduced
//! norm as an exact rational. No floating-point formatting is used.
//!
//! # Complexity
//!
//! Rendering is linear in the size of the object; [`instance_record_json`] also
//! computes the word-trace spectrum and Dieudonne determinant it serialises.

use crate::algebra::QuaternionicWeight;
use crate::generators::random_formula::{RATIO_DENOMINATOR, RATIO_NUMERATOR};
use crate::models::cnf::Clause;
use crate::models::incidence_matrix::AlgebraicAdjacencyMatrix;

/// Render a [`QuaternionicWeight`] as a JSON object of its four exact rational
/// components (as strings).
pub fn quaternion_to_json(weight: &QuaternionicWeight) -> String {
    let (a, b, c, d) = match weight {
        QuaternionicWeight::Zero => (
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
            "0".to_string(),
        ),
        QuaternionicWeight::Value { a, b, c, d } => {
            (a.to_string(), b.to_string(), c.to_string(), d.to_string())
        }
    };
    format!("{{\"a\":\"{a}\",\"b\":\"{b}\",\"c\":\"{c}\",\"d\":\"{d}\"}}")
}

/// Render a single clause in DIMACS-style signed-integer form, e.g. `[1,-2,3]`.
pub fn clause_to_json(clause: &Clause) -> String {
    let literals: Vec<String> = clause
        .literals
        .iter()
        .map(|literal| {
            let magnitude = (literal.variable_idx + 1) as i64;
            let signed = if literal.is_positive {
                magnitude
            } else {
                -magnitude
            };
            signed.to_string()
        })
        .collect();
    format!("[{}]", literals.join(","))
}

/// Render an entire formula as a JSON array of DIMACS-style clauses.
pub fn formula_to_json(clauses: &[Clause]) -> String {
    let rendered: Vec<String> = clauses.iter().map(clause_to_json).collect();
    format!("[{}]", rendered.join(","))
}

/// Build the full JSON-Lines record for one labelled instance, returning the
/// rendered line together with its ground-truth satisfiability label.
///
/// The record carries the variable/clause counts, the exact critical ratio, the
/// satisfiability label, the clause list, the word-trace spectrum entries
/// `Tr(A^2), Tr(A^4), Tr(A^6)`, and the Dieudonne determinant with its reduced
/// norm.
pub fn instance_record_json(
    num_variables: usize,
    num_clauses: usize,
    sample_idx: usize,
    clauses: &[Clause],
    satisfiable: bool,
) -> String {
    let matrix = AlgebraicAdjacencyMatrix::build_from_cnf(num_variables, clauses);

    let spectrum = matrix.word_trace_spectrum(6);
    let trace_2 = quaternion_to_json(&spectrum[1]);
    let trace_4 = quaternion_to_json(&spectrum[3]);
    let trace_6 = quaternion_to_json(&spectrum[5]);

    let determinant_json = match matrix.compute_dieudonne_determinant() {
        Some(det) => quaternion_to_json(&det),
        None => "null".to_string(),
    };
    let reduced_norm_json = match matrix.dieudonne_reduced_norm() {
        Some(norm) => format!("\"{norm}\""),
        None => "null".to_string(),
    };

    format!(
        concat!(
            "{{\"n\":{n},\"m\":{m},\"ratio\":\"{rn}/{rd}\",\"sample\":{sample},",
            "\"sat\":{sat},\"clauses\":{clauses},",
            "\"word_traces\":{{\"2\":{t2},\"4\":{t4},\"6\":{t6}}},",
            "\"dieudonne_determinant\":{det},\"dieudonne_reduced_norm\":{norm}}}"
        ),
        n = num_variables,
        m = num_clauses,
        rn = RATIO_NUMERATOR,
        rd = RATIO_DENOMINATOR,
        sample = sample_idx,
        sat = satisfiable,
        clauses = formula_to_json(clauses),
        t2 = trace_2,
        t4 = trace_4,
        t6 = trace_6,
        det = determinant_json,
        norm = reduced_norm_json,
    )
}

/// CSV header for a labelled invariant sweep.
pub fn invariant_csv_header() -> &'static str {
    "status,invariant"
}

/// One CSV row pairing a satisfiability status with an exact rational invariant.
pub fn invariant_csv_row(status: &str, invariant: &num_rational::BigRational) -> String {
    format!("{status},{invariant}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cnf::Literal;

    #[test]
    fn quaternion_json_carries_exact_components() {
        let q = QuaternionicWeight::from_integers(1, -2, 3, 4);
        assert_eq!(
            quaternion_to_json(&q),
            "{\"a\":\"1\",\"b\":\"-2\",\"c\":\"3\",\"d\":\"4\"}"
        );
        assert_eq!(
            quaternion_to_json(&QuaternionicWeight::Zero),
            "{\"a\":\"0\",\"b\":\"0\",\"c\":\"0\",\"d\":\"0\"}"
        );
    }

    #[test]
    fn clause_and_formula_json_use_dimacs_signs() {
        let clause = Clause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
            Literal::new(2, true),
        ]);
        assert_eq!(clause_to_json(&clause), "[1,-2,3]");
        assert_eq!(
            formula_to_json(&[clause.clone(), clause]),
            "[[1,-2,3],[1,-2,3]]"
        );
    }

    #[test]
    fn instance_record_is_valid_shape() {
        let clauses = vec![Clause::new(vec![
            Literal::new(0, true),
            Literal::new(1, false),
            Literal::new(2, true),
        ])];
        let record = instance_record_json(3, 1, 0, &clauses, true);
        assert!(record.starts_with("{\"n\":3,\"m\":1"));
        assert!(record.contains("\"sat\":true"));
        assert!(record.contains("\"word_traces\""));
    }
}
