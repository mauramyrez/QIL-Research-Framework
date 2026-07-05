//! DIMACS CNF parser.
//!
//! # Mathematical meaning
//!
//! Reads the standard DIMACS CNF text format into QIL's [`Clause`] model so that
//! externally produced 3-SAT instances can be fed through the exact-arithmetic
//! pipeline. A `p cnf <vars> <clauses>` header declares the variable count;
//! `c ...` lines are comments; each clause is a whitespace-separated list of
//! signed 1-based literals terminated by `0`.
//!
//! Malformed input yields a typed [`QilError::DimacsParse`] rather than a panic,
//! supporting robust reproducible tooling.
//!
//! # Complexity
//!
//! Linear in the size of the input text.

use crate::models::cnf::{Clause, Literal};
use crate::utils::error::{QilError, QilResult};

/// Parse a DIMACS CNF document, returning `(num_variables, clauses)`.
///
/// Variables are converted to zero-based indices (DIMACS literal `v` maps to
/// variable `|v| - 1` with polarity `v > 0`). If a `p cnf` header is present its
/// declared variable count is used (and must cover every literal); otherwise the
/// count is inferred as the maximum variable index seen.
///
/// # Errors
///
/// Returns [`QilError::DimacsParse`] on a malformed token, a missing clause
/// terminator, or a literal exceeding the declared variable count.
pub fn parse_dimacs(input: &str) -> QilResult<(usize, Vec<Clause>)> {
    let mut declared_vars: Option<usize> = None;
    let mut clauses: Vec<Clause> = Vec::new();
    let mut current: Vec<Literal> = Vec::new();
    let mut max_var_seen: usize = 0;

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('c') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("p ") {
            // Header: `p cnf <vars> <clauses>`.
            let mut fields = rest.split_whitespace();
            match fields.next() {
                Some("cnf") => {}
                other => {
                    return Err(QilError::DimacsParse(format!(
                        "expected 'p cnf ...' header, found 'p {}'",
                        other.unwrap_or("")
                    )))
                }
            }
            let vars: usize = fields
                .next()
                .ok_or_else(|| QilError::DimacsParse("header missing variable count".to_string()))?
                .parse()
                .map_err(|_| {
                    QilError::DimacsParse("invalid variable count in header".to_string())
                })?;
            declared_vars = Some(vars);
            continue;
        }

        for token in trimmed.split_whitespace() {
            let value: i64 = token
                .parse()
                .map_err(|_| QilError::DimacsParse(format!("invalid literal token '{token}'")))?;
            if value == 0 {
                // End of the current clause.
                clauses.push(Clause::new(std::mem::take(&mut current)));
            } else {
                let variable_idx = (value.unsigned_abs() as usize) - 1;
                max_var_seen = max_var_seen.max(variable_idx + 1);
                if let Some(declared) = declared_vars {
                    if variable_idx >= declared {
                        return Err(QilError::VariableOutOfRange {
                            index: variable_idx,
                            declared,
                        });
                    }
                }
                current.push(Literal::new(variable_idx, value > 0));
            }
        }
    }

    if !current.is_empty() {
        return Err(QilError::DimacsParse(
            "final clause is not terminated by 0".to_string(),
        ));
    }

    let num_variables = declared_vars.unwrap_or(max_var_seen);
    Ok((num_variables, clauses))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_simple_dimacs_document() {
        let input = "\
c a tiny formula
p cnf 3 2
1 -2 3 0
-1 2 -3 0
";
        let (n, clauses) = parse_dimacs(input).expect("well-formed DIMACS");
        assert_eq!(n, 3);
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0].literals[0], Literal::new(0, true));
        assert_eq!(clauses[0].literals[1], Literal::new(1, false));
        assert_eq!(clauses[1].literals[2], Literal::new(2, false));
    }

    #[test]
    fn rejects_unterminated_clause() {
        let input = "p cnf 2 1\n1 2\n";
        assert!(matches!(parse_dimacs(input), Err(QilError::DimacsParse(_))));
    }

    #[test]
    fn rejects_out_of_range_literal() {
        let input = "p cnf 2 1\n1 2 3 0\n";
        assert!(matches!(
            parse_dimacs(input),
            Err(QilError::VariableOutOfRange {
                index: 2,
                declared: 2
            })
        ));
    }

    #[test]
    fn infers_variable_count_without_header() {
        let input = "1 -4 2 0\n";
        let (n, clauses) = parse_dimacs(input).expect("headerless DIMACS");
        assert_eq!(n, 4);
        assert_eq!(clauses.len(), 1);
    }
}
