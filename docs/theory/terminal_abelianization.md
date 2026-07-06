# Extrinsic Terminal Abelianization

The second collapse mechanism. Unlike the intrinsic collapse, this is a
property of the *measurement*, not the object: the tensor stays
non-commutative, but the scalar readout abelianizes it.

## Transverse contraction

Contract one axis of `T_phi` against a fixed test vector `V` (canonical
`V = [i, j, k, 1]`) to obtain the marginal matrix

```
M_{i,j} = sum_k (T_phi)_{i,j,k} . V_k,
```

with the tensor entry kept on the left of each product (order preserved).

## Two invariants

- **Frobenius reduced-norm** `Phi = sum_{i,j} Nrd(M_{i,j})`: an abelian
  cell-by-cell aggregate.
- **Non-commutative word-trace** `Psi_p = Nrd(Tr(M^p))`: the ordered power
  `M^p` retains non-abelian interference; the terminal `Nrd` reduces the trace
  scalar.

## Statement (Terminal Abelianization Theorem)

Any readout `F = g o Nrd` applied to `t_p = Tr(M^p)` depends on `t_p` only
through the conjugation-invariant central pair

```
(Trd(t_p), Nrd(t_p)) in Q^2.
```

Because `Nrd(u x u^{-1}) = Nrd(x)` and the conjugacy class of a quaternion is
determined by `(Trd, Nrd)`, the readout is invariant under every `SO(3)`
rotation of the imaginary part: the three imaginary components enter only
through `b^2 + c^2 + d^2`, so orientation (the relative phase among `i, j, k`)
is erased. This is the "wall" at which the non-commutative content is lost.

## Empirical corollary

On size-matched sweeps (`n = 4`, `m = 10` fixed) both `Phi/m^2` and
`Nrd(Tr(M^3))/m^2` interleave the SAT and UNSAT classes (Tables 1 and 2 of the
paper): the surviving quantity tracks connectivity topology, not
satisfiability.

## Where this lives in QIL

- `src/invariants/quaternionic_trace.rs` — `matrix_power`, `word_trace`.
- `src/invariants/tensor_invariants.rs` — `compute_spectral_invariant`, `compute_normalized_invariant`.
- `src/collapse/extrinsic.rs` — `TerminalProjection { reduced_trace, reduced_norm }`.

## Example

```
cargo run --example reproduce_table_1
cargo run --example reproduce_table_2
```

## Paper reference

Sections "Non-Commutative Contraction and the Spectrum" and "The Terminal
Abelianization Theorem" of
`docs/submission_computational_complexity_springer/Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex`.
