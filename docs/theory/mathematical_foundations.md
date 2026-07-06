# Mathematical Foundations: the rational quaternion division ring `H(Q)`

QIL is built on the Hamiltonian quaternion algebra over the exact rationals,

```
H(Q) = Q + Q*i + Q*j + Q*k,
```

with Hamilton's relations

```
i^2 = j^2 = k^2 = ijk = -1,
ij =  k,  jk =  i,  ki =  j,
ji = -k,  kj = -i,  ik = -j.
```

Multiplication is non-commutative (`ij = -ji`), which is the structural feature
the whole program exploits: it is what prevents the linear gauge collapse of
commutative spectral invariants.

## Conjugate, reduced norm, reduced trace

For `q = a + b i + c j + d k`:

- conjugate `conj(q) = a - b i - c j - d k`;
- reduced norm `Nrd(q) = q conj(q) = a^2 + b^2 + c^2 + d^2 in Q_{>=0}` (multiplicative);
- reduced trace `Trd(q) = q + conj(q) = 2a in Q`.

Because `Q` is formally real, `Nrd(q) = 0` iff `q = 0`. Hence every non-zero
element is invertible, `q^{-1} = conj(q) / Nrd(q)`, and `H(Q)` is a division
ring whose centre is exactly the scalar field `Q`.

The conjugation-invariant pair `(Trd, Nrd)` determines the conjugacy class of a
quaternion; this pair is the central datum onto which the Terminal
Abelianization theorem collapses the non-commutative word-trace.

## Where this lives in QIL

- `src/algebra/quaternion.rs` — the `QuaternionicWeight` type and ring ops.
- `src/algebra/reduced_norm.rs` — `Nrd`.
- `src/algebra/trace.rs` — `Trd`.
- `src/algebra/division_ring.rs` — conjugation, inversion, central scaling.
- `src/algebra/ideals.rs` — integer factorisation and the `H(Q) -> Q[k]` quotient.

## Exactness

Every coefficient is an arbitrary-precision rational (`num-rational` over
`num-bigint`). There is no floating-point arithmetic anywhere in QIL.

## Paper reference

Section "The Quaternionic Incidence Model" (rational quaternion division ring)
of `docs/submission_computational_complexity_springer/Two_Collapse_Mechanisms_for_Non_Commutative_Invariants_of_3_SAT.tex`
(Springer *Computational Complexity* submission).
