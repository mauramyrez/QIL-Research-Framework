//! `models` — the two algebraic encodings of a 3-CNF formula.
//!
//! QIL studies two carriers for the same combinatorial object, matching the two
//! phases of the manuscript:
//!
//! * [`incidence_matrix`] — the linear (order-2) bipartite clause/variable
//!   incidence matrix `A_phi` over `H(Q)`, whose two-colourability drives the
//!   Intrinsic Bipartite Gauge Collapse.
//! * [`hypergraph_tensor`] — the non-linear (order-3) symmetric adjacency tensor
//!   `T_phi` on a 3-uniform hypergraph, with non-local vertex-degree modulation.
//! * [`contractions`] — the transverse contraction operator projecting `T_phi`
//!   to a marginal matrix `M`.
//!
//! The shared CNF data model lives in [`cnf`].

pub mod cnf;
pub mod contractions;
pub mod hypergraph_tensor;
pub mod incidence_matrix;

pub use cnf::{Clause, Literal};
pub use contractions::TensorMatrixProjection;
pub use hypergraph_tensor::{CNFClause, HypergraphTensor3};
pub use incidence_matrix::AlgebraicAdjacencyMatrix;
