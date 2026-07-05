# The Quaternionic Order-3 Tensor Model (non-linear phase)

The second, *non-linear* model discards the clause-vertex class: the vertices
are the Boolean variables, a width-3 clause is a hyperedge `{i, j, k}`, and the
formula is a 3-uniform hypergraph carried by a symmetric order-3 tensor

```
T_phi in (H(Q)^n)^{tensor 3},   (T_phi)_{i,j,k} in H(Q).
```

Because the vertices form a single class, the diagonal bipartite gauge
involution of the matrix model has no analogue, and genuine non-commutative
structure survives.

## Topological Interlinkage Model (clause weight)

A pre-pass computes the global vertex degree map `deg(v)` (literal occurrences
of `v`, any polarity). Each clause on `(i, j, k)` with signs `(s_i, s_j, s_k)`
is scaled by the non-local connectivity factor `D = deg(i) deg(j) deg(k)` and
deposited as

```
base = D (1 + s_i i + s_j j + s_k k)
```

across all six `S_3` permutations, then a closing symmetrisation (`1/6` exact
average) enforces `S_3` invariance.

## Fourier Hypercube Annihilation

On a hyperedge whose clauses enumerate the full sign cube `{+-1}^3`, character
orthogonality kills the odd Fourier modes and the entry collapses to a pure
real scalar `8 D`. This is why the minimal `n = 3` UNSAT control is a scalar
`4096`. Degree heterogeneity at `n >= 4` (with locally unbalanced sign coverage)
breaks the cancellation: the `n = 4` UNSAT tensor retains
`660 + 660 i + 660 j` on `{0,2,3}` and `297 - 297 i + 297 j + 297 k` on `{1,2,3}`.

## Where this lives in QIL

- `src/models/hypergraph_tensor.rs` — `HypergraphTensor3`, `build_tensor_from_cnf`, `symmetrize`.
- `src/models/contractions.rs` — `contract_axis`, `canonical_spatial_vector`.
- `src/invariants/tensor_invariants.rs` — Frobenius invariant, normalized word-trace invariant.

## Example

```
cargo run --example tensor_example
cargo run --example reproduce_figures
```

## Paper reference

Sections "The Quaternionic Order-3 Tensor Construction" and "Fourier Hypercube
Annihilation and Degree Heterogeneity".
