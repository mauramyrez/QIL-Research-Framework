# QIL Architecture

QIL (Quaternionic Invariant Laboratory) is organised so that **each module
corresponds to a concept of the manuscript**. The dependency direction flows
from the algebra core outward to the models, invariants, collapse diagnostics,
generators, and I/O.

## Crate layout

```
src/
  algebra/        exact Hamiltonian division ring H(Q)
    quaternion.rs       QuaternionicWeight, ring ops, Display
    reduced_norm.rs     Nrd
    trace.rs            Trd
    division_ring.rs    conjugate, inverse, scale
    ideals.rs           factorize, project_to_clause_torsion (H(Q) -> Q[k])
  models/         the two encodings of a CNF formula
    cnf.rs              Literal, Clause (shared data model)
    incidence_matrix.rs AlgebraicAdjacencyMatrix (order-2, bipartite)
    hypergraph_tensor.rs HypergraphTensor3 (order-3, 3-uniform)
    contractions.rs     TensorMatrixProjection, contract_axis
  invariants/     candidate local non-commutative invariants
    dieudonne.rs        Dieudonne determinant + reduced norm
    spectral.rs         matrix word-trace spectrum Tr(A^p)
    quaternionic_trace.rs  ordered word-trace Tr(M^p) of the marginal
    tensor_invariants.rs   Frobenius + normalized word-trace invariant
  collapse/       the two collapse mechanisms
    intrinsic.rs        Intrinsic Bipartite Gauge Collapse (torsion 2bc)
    extrinsic.rs        Extrinsic Terminal Abelianization ((Trd, Nrd))
    obstruction.rs      prime-support diagnostics (Pure-Power Lemma)
  generators/     reproducible CNF generation + exact labelling
    random_formula.rs   SplitMix64, random_3cnf, is_satisfiable, ratio 213/50
    sat.rs              canonical + heterogeneous SAT controls
    unsat.rs            canonical + heterogeneous UNSAT controls
  io/             reproducible input/output
    parser.rs           DIMACS CNF reader (typed errors)
    export.rs           exact JSON-Lines / CSV serialisation
  utils/          error surface + shared helpers
    mod.rs              rational_from_i64, render_weight
    error.rs            QilError, QilResult
  lib.rs          module declarations + public re-exports (QIL prelude)

examples/         reproduction drivers (cargo run --example ...)
experiments/      research drivers (declared as example targets)
benches/          Criterion micro-benchmarks (performance only)
tests/            integration tests (external-crate view)
docs/             paper/, theory/, architecture.md, reproducibility.md
```

## Design principles

- **Concept-per-module.** A reader of the paper can locate any object by name.
- **Exact arithmetic only.** No floating point anywhere; results are exact
  rationals reproducible bit-for-bit.
- **Split inherent impls.** A model type (e.g. `AlgebraicAdjacencyMatrix`) has
  its construction in `models/` and its invariants across `invariants/` and
  `collapse/`; Rust permits inherent-impl blocks in sibling modules of the same
  crate, keeping each file single-concept.
- **Typed errors.** Fallible operations (e.g. DIMACS parsing) return
  `Result<_, QilError>` rather than panicking.
- **Extensibility.** New invariants attach to an existing model; new algebras or
  tensors slot into `algebra`/`models`; new experiments are self-contained
  example drivers. The `collapse` module lets a candidate invariant be tested
  against both barriers before it is trusted.

## Dependencies

Runtime: `num-bigint`, `num-rational`, `num-traits` (exact arithmetic only).
Dev-only: `criterion` (benchmarks; never affects results).
