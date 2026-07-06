# Contributing to QIL

QIL is a research platform. Contributions that add new invariants, algebraic
carriers, experiments, or documentation are welcome, provided they uphold the
project's core guarantees.

## Non-negotiable invariants

1. **No floating point.** Every arithmetic operation must be exact over `Z`/`Q`
   (`num-bigint` / `num-rational`). Pull requests introducing `f32`/`f64` in the
   computational path will not be accepted.
2. **Reproducibility.** Randomness must come from the deterministic
   `SplitMix64` stream seeded by explicit constants. Results must reproduce
   bit-for-bit.
3. **Concept-per-module.** New code belongs in the module matching its
   mathematical concept (see [docs/architecture.md](docs/architecture.md)).
4. **Documented public API.** Every public item carries Rustdoc explaining its
   mathematical meaning, a paper reference, complexity, and limitations.

## Development workflow

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

All three must pass before a change is proposed. Add unit tests next to the code
and, for manuscript-level claims, an integration test under `tests/`.

## Git commits without Cursor co-authorship

Cursor may append `Co-authored-by: Cursor <cursoragent@cursor.com>` to commit
messages, which GitHub displays as a second author. To avoid this:

1. **One-time hook setup** (recommended):  
   `git config core.hooksPath .githooks`
2. **Or** use:  
   `.\scripts\commit-author-only.ps1 -Subject "..." -Body "..."`

## Adding a new invariant

1. Implement it as a method on the relevant model in `src/invariants/`.
2. Add a unit test with an exact expected value.
3. If it is a manuscript result, add a reproduction example under `examples/`
   and an assertion in `tests/`.
4. If it is a candidate obstruction, test it against both barriers in
   `src/collapse/` (two-colourable bilinear format; terminally-abelianized
   readout).

## Commit and PR hygiene

- Keep changes focused and describe the mathematical intent, not just the diff.
- Update `CHANGELOG.md` under `[Unreleased]`.
- Do not commit generated artifacts (PDFs, `target/`).

## Licensing

By contributing you agree that your contributions are dual-licensed under
MIT and Apache-2.0, consistent with the project license.
