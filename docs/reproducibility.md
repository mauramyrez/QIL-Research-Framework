# Reproducibility Guide

Every quantitative claim in the QIL manuscript is reproducible bit-for-bit with
exact arithmetic. This guide lists the exact commands.

## Prerequisites

- Rust (edition 2021, `rust-version = 1.95` or later), `cargo`.
- No system libraries beyond the `num-*` crates fetched by Cargo.
- A LaTeX toolchain (only to recompile the paper PDF).

## Reproduce the tables

```
cargo run --example reproduce_table_1     # Table 1: abelian Frobenius invariant Phi/m^2
cargo run --example reproduce_table_2     # Table 2: word-trace Nrd(Tr(M^3))/m^2
```

Both draw the identical size-matched batch (`n = 4`, `m = 10`, seed
`0xC0FFEE00_1234_5678`, six SAT and six UNSAT instances) and print exact
rationals matching the manuscript. The sorted vectors are also asserted by the
integration test `tests/reproduce_manuscript.rs`.

## Reproduce the figures' data

```
cargo run --example reproduce_figures
```

Prints the minimal `n = 3` contrast (UNSAT `4096`, SAT `2401 + 343(i+j+k)`,
squared Frobenius distance `19355832`) and the heterogeneous `n = 4` contrast
(surviving imaginary entries `660 + 660i + 660j`, `297 - 297i + 297j + 297k`).

## Reproduce the worked examples

```
cargo run --example matrix_example        # Dieudonne determinant + torsion (n = 3)
cargo run --example tensor_example         # tensor assembly + contraction + terminal projection
```

## Machine-checked constants

```
cargo run --example exact_validation
```

Re-derives and asserts the pinned constants (minimal-pair reduced norms
`2^16` and `2^8 * 13^2`, the `b = c = 0` collapse, the tensor control values).

## Research drivers

```
cargo run --example random_instances -- 5 10 <seed>   # JSONL dataset on stdout
cargo run --example stress_tests -- 10 20 <seed>      # 160 instances (n=3..=10), collapse + symmetry
cargo run --release --example benchmark               # coarse timings
```

## Quality gates

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo bench            # Criterion (performance only; optional)
```

## Recompile the paper

```
cd docs/submission_algorithmica_springer
pdflatex Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex
bibtex Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT
pdflatex Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex
pdflatex Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex
```

(Or `latexmk -pdf Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex` if Perl is available.)

## Determinism notes

- No floating point is used anywhere; all results are exact rationals.
- All randomness is a deterministic `SplitMix64` integer stream seeded by fixed
  constants, so datasets reproduce identically across platforms.
- Satisfiability labels come from an exhaustive exact solver over all `2^n`
  assignments.
