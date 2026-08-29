# 015 — Discrete exterior calculus / de Rham cohomology

Status: active
Layer: mathematical
Id: `de-rham`

## Purpose

The mathematical layer earns a first-class object that scrutinizes a piece of
pure mathematics with the same machinery as the physics theories — and leans
hard on Rust's type system, the project's core premise.

Differential-form **grade** is carried at the type level, so the compiler
forbids adding a 1-form to a 2-form and the exterior derivative `d` provably
raises grade by exactly one. On a simplicial complex the coboundary makes
`d ∘ d = 0` an *exact* identity — the algebra behind `curl grad = 0` and behind
the homogeneous Maxwell equations `dF = 0` when `F = dA`. The same coboundary
computes topology: the first Betti number counts holes, and whether every closed
1-form is exact (the Poincaré lemma) detects them.

## Object

| id | object |
|---|---|
| `de-rham` | discrete exterior calculus on a small simplicial complex |

## Knob

| knob | effect |
|---|---|
| `shape` | the complex to evaluate on: `disk` (`b₁ = 0`), `circle` (`b₁ = 1`), or `torus` (`b₁ = 2`). Changing it changes the topology and flips `dec.closed-equals-exact`. |

## Claims (all computed theorems)

| id | meaning | how it is decided |
|---|---|---|
| `dec.d-squared-zero` | `d ∘ d = 0` | `d₁(d₀ f) = 0` for every basis 0-form, exactly |
| `dec.first-betti-number` | the number of holes `b₁` | `b₁ = n_edges − rank(d₁) − rank(d₀)`, ranks by Gaussian elimination |
| `dec.closed-equals-exact` | every closed 1-form is exact (Poincaré) | holds iff `b₁ = 0` |
| `dec.euler-poincare` | `V−E+F = b₀−b₁+b₂` | the Euler characteristic computed two independent ways, checked equal |
| `dec.hodge-harmonic` | `dim(harmonic 1-forms) = b₁` (Hodge) | nullity of the Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁`, checked against `b₁` |

## Type-level grade

A `Cochain<G>` carries its grade `G ∈ {G0, G1, G2}` as a type parameter. `d₀`
maps `Cochain<G0> → Cochain<G1>` and `d₁` maps `Cochain<G1> → Cochain<G2>`, so
`d₁(d₀(f))` type-checks and lands in grade 2. Mixing grades is a compile error,
enforced by a `compile_fail` doctest:

```rust,compile_fail
use physis_theory::dec::{Cochain, G0, G1};
let a = Cochain::<G0>::zero(3);
let b = Cochain::<G1>::zero(3);
let _ = a + b; // different grades: does not type-check
```

This mirrors the `Qty<D>` dimensional contracts in `physis-core`: the type
system rules out a whole class of nonsense before any value is computed.

## `d² = 0` and Maxwell

For any 0-form `f`, `(d₁ d₀ f)[a,b,c] = (f[b]−f[a]) − (f[c]−f[a]) + (f[c]−f[b])
= 0` identically — the discrete `curl grad = 0`. The same nilpotency is why
`F = dA` forces `dF = 0`: the homogeneous Maxwell equations (Faraday's law and
the absence of magnetic monopoles) are not extra assumptions, they are `d² = 0`.
`specs/008-electromagnetism.md` checks the Maxwell equations numerically; here
the *homogeneous* half is an exact topological identity.

## Betti numbers and the knob → verdict diff

```
physis run de-rham                # disk: b₁ = 0, closed = exact (Poincaré holds)
physis set de-rham shape circle   # circle: b₁ = 1, closed ≠ exact
physis set de-rham shape torus    # torus: b₁ = 2, χ = 0, harmonic dim 2
```

Changing the `shape` changes the topology. The first Betti number, computed from
the ranks of the incidence matrices, is `0` for the disk, `1` for the circle,
and `2` for the torus, and `dec.closed-equals-exact` flips `holds → fails` once a
hole appears: a closed 1-form that is not exact now exists. The torus (a
triangulated 3×3 flat torus: 9 vertices, 27 edges, 18 triangles) is a
non-trivial check — `b₀ = 1`, `b₁ = 2`, `b₂ = 1`, `χ = 0`, all computed from the
coboundary, with the Hodge Laplacian's harmonic dimension matching `b₁ = 2`.
Topology is detected mechanically, by linear algebra on the coboundary.

## Two invariants cross-checked

Two more theorems each compute a classical invariant *two independent ways* and
check they agree — the kind of redundant, mechanical cross-check the lab is for:

- **Euler–Poincaré** (`dec.euler-poincare`): the Euler characteristic from cell
  counts, `χ = V − E + F`, equals the alternating sum of Betti numbers,
  `b₀ − b₁ + b₂`. Disk: `χ = 1`; circle: `χ = 0` — both agree on both sides.
- **Hodge theorem** (`dec.hodge-harmonic`): the dimension of harmonic 1-forms —
  the nullity of the combinatorial Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁` — equals
  `b₁`. Disk: `0`; circle: `1`. Harmonic representatives ≅ cohomology, computed
  from the coboundary matrices.

Both are identities, so they `hold` on the disk and the circle alike; only the
numbers change with the `filled` knob.

## Non-goals (this milestone)

- Higher-dimensional complexes (only vertices/edges/triangles, grades 0–2; the
  torus is a genuine closed surface, but simplices stop at triangles).
- The Hodge *star* and codifferential as first-class operators (the Laplacian
  is built here directly from `d`; a metric-dependent star is a later increment).
- General mesh input; the two complexes are the disk and the circle.

## Related

- `specs/001-type-system.md` — the `Qty<D>` type-level contracts this mirrors
- `specs/008-electromagnetism.md` — Maxwell's equations numerically
- `specs/002-ontology-layers.md` — the mathematical layer
