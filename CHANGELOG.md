# Changelog

All notable changes to QIL are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- Rebranded and restructured the project into the **Quaternionic Invariant
  Laboratory (QIL)**. The Cargo package was renamed from `gct-lab` to `qil`.
- Reorganised the source tree so that each module corresponds to a concept of
  the manuscript: `algebra/`, `models/`, `invariants/`, `collapse/`,
  `generators/`, `io/`, `utils/`.
- Unified the two research phases (linear incidence matrix and non-linear
  hypergraph tensor) into a single manuscript at
  `docs/submission_algorithmica_springer/Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex`.

### Added
- `collapse/extrinsic.rs`: the `TerminalProjection (Trd, Nrd)` formalising the
  Extrinsic Terminal Abelianization theorem.
- `algebra/trace.rs`: the reduced trace `Trd`.
- `io/parser.rs`: a DIMACS CNF parser with typed errors (`QilError`).
- `io/export.rs`: exact JSON-Lines / CSV serialisation.
- `examples/`: reproduction drivers (`reproduce_table_1`, `reproduce_table_2`,
  `reproduce_figures`, `matrix_example`, `tensor_example`).
- `experiments/`: research drivers (`random_instances`, `benchmark`,
  `exact_validation`, `stress_tests`).
- `benches/`: Criterion micro-benchmarks (performance only).
- `tests/`: integration tests reproducing the manuscript's pinned values.
- Repository metadata: `CITATION.cff`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`,
  dual `LICENSE-MIT` / `LICENSE-APACHE`, and concept notes under `docs/theory/`.

### Preserved
- All mathematical results are unchanged and exact: the minimal-pair reduced
  norms `2^16` and `2^8 * 13^2`, the tensor control values, and the size-matched
  invariant tables reproduce bit-for-bit. No floating-point arithmetic is used
  anywhere.
