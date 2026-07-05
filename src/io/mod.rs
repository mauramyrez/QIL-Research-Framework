//! `io` — reproducible input/output for QIL.
//!
//! * [`parser`] — a DIMACS CNF reader returning the shared [`crate::models::cnf`]
//!   model with typed errors.
//! * [`export`] — exact JSON-Lines and CSV serialisation of instances and
//!   invariants (all quaternion components emitted as exact rational strings).

pub mod export;
pub mod parser;

pub use export::{
    clause_to_json, formula_to_json, instance_record_json, invariant_csv_header, invariant_csv_row,
    quaternion_to_json,
};
pub use parser::parse_dimacs;
