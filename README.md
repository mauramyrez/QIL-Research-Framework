# QIL — Quaternionic Invariant Laboratory

[![Repository](https://img.shields.io/badge/GitHub-QIL--Research--Framework-blue)](https://github.com/mauramyrez11/QIL-Research-Framework)

QIL is an open-source, **exact-arithmetic** research framework in Rust for the
study of non-commutative quaternionic invariants of 3-SAT and the structural
collapse mechanisms that arise in the Geometric Complexity Theory (GCT)
program.

> QIL does **not** claim to separate P from NP. It answers a sharper,
> machine-checkable question: *why do local, low-degree non-commutative
> invariants built on quaternionic encodings of 3-SAT structurally fail, even
> when the obvious gauge symmetries are broken?*

## Scientific Manuscript

The Springer Nature / *Algorithmica* submission lives in
[`docs/submission_algorithmica_springer/`](docs/submission_algorithmica_springer/).

| Item | Path |
|------|------|
| Main source | [`sn-article.tex`](docs/submission_algorithmica_springer/sn-article.tex) |
| Bibliography | [`sn-bibliography.bib`](docs/submission_algorithmica_springer/sn-bibliography.bib) |
| Document class | [`sn-jnl.cls`](docs/submission_algorithmica_springer/sn-jnl.cls) |

**Compile the PDF** (from the repository root):

```bash
cd docs/submission_algorithmica_springer
pdflatex sn-article.tex
bibtex sn-article
pdflatex sn-article.tex
pdflatex sn-article.tex
```

On Windows (PowerShell), the same sequence applies. If `bibtex` is unavailable,
run `pdflatex` twice; citations may be incomplete until BibTeX is run once.

## Reproducibility with QIL

This repository root is the **Quaternionic Invariant Laboratory (QIL)**. It is
the official computational companion to the manuscript: every exact rational in
Tables 1–2, the figure contrast data, and the pinned constants cited in
Section “Exact Size-Matched Experiments” are emitted and machine-checked by the
Rust framework here—not by hand.

QIL verifies:

- **Table 1** — abelian Frobenius reduced-norm invariant `Φ/m²`
- **Table 2** — non-commutative word-trace invariant `Nrd(Tr(M³))/m²`
- **Figure data** — minimal (`n = 3`) and heterogeneous (`n = 4`) tensor contrasts
- **Worked examples** — Dieudonné determinant, torsion collapse, terminal projection
- **Pinned constants** — minimal-pair reduced norms, exact validation suite

All coefficients use arbitrary-precision rationals; there is no floating-point
anywhere. Randomness is a deterministic `SplitMix64` stream; satisfiability
labels come from an exhaustive exact solver.

**Run the experiments** (from the repository root):

```bash
cargo run --example reproduce_table_1      # Table 1
cargo run --example reproduce_table_2      # Table 2
cargo run --example reproduce_figures      # figure contrast data
cargo run --example exact_validation       # pinned constants
cargo test                                 # integration checks (incl. manuscript tables)
```

See [docs/reproducibility.md](docs/reproducibility.md) for the full command list,
and [docs/theory/](docs/theory) for concept-by-concept notes.

## The two central results

QIL implements exactly the two collapse mechanisms of the research program:

1. **Intrinsic Bipartite Gauge Collapse.** The two-colourability of the
   clause/variable incidence matrix induces a diagonal gauge involution
   `sigma_k(A) = G A G^{-1}` that forces the Dieudonne determinant into the
   commutative subfield `Q[k]`, annihilating the literal commutator torsion
   `2bc`. (A symmetry of the *object*.)
2. **Extrinsic Terminal Abelianization.** The reduced-norm readout of the tensor
   word-trace `Nrd(Tr(M^p))` factors through the conjugation quotient,
   collapsing the non-commutative spectrum onto the central pair `(Trd, Nrd)`.
   (A property of the *measurement*.)

## Why exact arithmetic

Every coefficient is an arbitrary-precision rational (`num-rational` over
`num-bigint`). **There is no floating-point operation anywhere in QIL**, so all
reduced norms, traces, tensor entries and invariants are exact and reproducible
bit-for-bit.

## Architecture

Each module corresponds to a concept of the paper (see
[docs/architecture.md](docs/architecture.md)):

- `algebra/` — the exact Hamiltonian division ring `H(Q)`.
- `models/` — the incidence matrix (order-2) and the hypergraph tensor
  (order-3), plus the transverse contraction operator.
- `invariants/` — Dieudonne determinant, word-trace spectra, tensor invariants.
- `collapse/` — the two collapse mechanisms and prime-support diagnostics.
- `generators/` — reproducible CNF generation and exact labelling.
- `io/` — DIMACS parsing and exact JSON/CSV export.
- `utils/` — typed error surface and shared helpers.

## Extending QIL

QIL is a platform, not a single-paper script. New invariants attach to an
existing model in `invariants/`; new non-commutative algebras or higher-order
tensors slot into `algebra/` and `models/`; new experiments are self-contained
drivers under `examples/`. The `collapse/` module lets a candidate successor
invariant be tested against both barriers (two-colourable bilinear format;
terminally-abelianized readout) before it is trusted.

## Citing QIL

If you use QIL in academic work, please cite it via [CITATION.cff](CITATION.cff)
and the accompanying manuscript.

## License

Dual-licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option.
