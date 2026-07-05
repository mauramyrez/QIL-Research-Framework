# Intrinsic Bipartite Gauge Collapse

The first collapse mechanism. It is a property of the *object* (the matrix),
induced by the two-colourability of the incidence graph.

## Statement (Commutative Collapse Theorem)

For every non-singular incidence matrix `A_phi`, the Dieudonne determinant
representative lies in the commutative subfield

```
Q[k] = Q + Q k  ~  Q(sqrt(-1)),
```

i.e. its literal components vanish, `b = c = 0`. Consequently the inter-literal
commutator torsion

```
[b i, c j] = 2 b c k   =>   torsion 2 b c = 0
```

is annihilated identically on the entire satisfiability variety, and
`Nrd(det_D) = a^2 + d^2`.

## Mechanism

Let `sigma_k(x) = k x k^{-1}` be conjugation by `k`; it fixes `Q + Q k` and
negates `span{i, j}`. Let `G = diag(+1 on variables, -1 on clauses)`. Then the
two-colouring *is* a gauge involution:

```
sigma_k(A_phi) = G A_phi G^{-1}.
```

Left-ordered elimination propagates this two-sided gauge, and a parity identity
`prod eta_t = (prod epsilon_p)^2 = 1` forces the determinant into
`Fix(sigma_k) = Q[k]`. See the paper for the full proof, including the general
form (Theorem "Intrinsic bipartite gauge collapse") covering every
polarity-on-`{i,j}`, two-colourable, bilinear encoding.

## Consequence

The collapse cannot be repaired inside order-2 (matrix) formats. Faithfully
representing the simultaneous three-literal interaction of a width-3 clause
without an auxiliary vertex class forces the carrier's arity up to three,
motivating the tensor model.

## Where this lives in QIL

- `src/collapse/intrinsic.rs` — `AnnihilatorOperator`, `compute_annihilating_functional`.
- `src/collapse/obstruction.rs` — prime-support diagnostics (Pure-Power Lemma).

## Paper reference

Sections "Global Torsion and the Commutative Collapse Theorem" and "The
Inevitability of Trilinear Objects".
