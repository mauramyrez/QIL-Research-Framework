# The Quaternionic Incidence Model (order-2 / matrix phase)

The first, *linear* model encodes a 3-CNF formula `phi` into the bipartite
clause/variable incidence graph `G_phi` and its algebraic adjacency matrix
`A_phi in H(Q)^{(n+m) x (n+m)}`.

## Encoding

- Vertices `0..n` are variables; vertices `n..(n+m)` are clauses.
- A positive literal contributes the unit `i`, a negative literal the unit `j`,
  placed symmetrically on the variable/clause edge.
- Each clause vertex carries `k` on its diagonal.

So every cross (variable-clause) weight lies in `span{i, j}` and every diagonal
weight lies in `Q + Q k`. These two facts, plus the two-colouring
`(variables, clauses)`, are the entire structural input to the collapse.

## Invariants

- **Dieudonne determinant** `det_D(A_phi)`: over a non-commutative division ring
  the ordinary determinant is undefined; we fix a representative by left-ordered
  Gaussian elimination and take the ordered product of pivots. Its reduced norm
  `Nrd(det_D) in Q_{>0}` is the canonical abelian invariant.
- **Word-trace spectrum** `Tr(A^p)`: closed-walk invariants that resist the
  unitary gauge change `A -> U* A U`.

## Where this lives in QIL

- `src/models/incidence_matrix.rs` — `AlgebraicAdjacencyMatrix`, `build_from_cnf`.
- `src/invariants/dieudonne.rs` — `compute_dieudonne_determinant`, `dieudonne_reduced_norm`.
- `src/invariants/spectral.rs` — `word_trace_spectrum`.

## Example

```
cargo run --example matrix_example
```

prints the determinant, reduced norm, prime support, and the (vanishing)
literal commutator torsion for the minimal `n = 3` control pair.

## Paper reference

Sections "The Quaternionic Incidence Model" and "Global Torsion and the
Commutative Collapse Theorem" of
`docs/submission_computational_complexity_springer/Impossibility_of_Local_Quaternionic_Invariants_for_3_SAT_Separation.tex`.
