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
| `shape` | the complex to evaluate on: `disk` (`b₁ = 0`, `b₂ = 0`), `circle` (`b₁ = 1`), `torus` (`b₁ = 2`, `b₂ = 1`), `klein` (Klein bottle: `b₁ = 1`, `b₂ = 0`), or `sphere` (`S²`: `b₁ = 0`, `b₂ = 1`, `χ = 2`). Changing it changes the topology and flips `dec.closed-equals-exact` and/or `dec.fundamental-class`. The coboundary identity is not this knob: `add-sign-flip` is an IR mutation. |

## Claims (all computed theorems)

| id | meaning | how it is decided |
|---|---|---|
| `dec.d-squared-zero` | `d ∘ d = 0` | live IR equation is the catalog polynomial `(b−a)−(c−a)+(c−b)`; the evaluator also checks `d₁(d₀ f) = 0` on the complex. `add-sign-flip` fails this cell. Not a kernel proof by itself |
| `dec.first-betti-number` | the number of holes `b₁` | `b₁ = n_edges − rank(d₁) − rank(d₀)`, ranks by Gaussian elimination |
| `dec.closed-equals-exact` | every closed 1-form is exact (Poincaré) | holds iff `b₁ = 0` |
| `dec.euler-poincare` | `V−E+F = b₀−b₁+b₂` | rank-cancellation of these Betti formulas, not a second path. Domain stays encoding-wide |
| `dec.hodge-harmonic` | `dim(harmonic 1-forms) = b₁` (discrete Hodge) | nullity of the combinatorial Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁`, checked against `b₁`. Domain: finite simplicial 1-cochains, not the smooth Hodge theorem |
| `dec.fundamental-class` | `b₂ = 1` over ℝ | computed `b₂`; holds for the torus and the 2-sphere, fails for the disk, circle, and Klein bottle |

`dec.closed-equals-exact` declares a live lemma edge to `dec.d-squared-zero`
(`Claim::depends_on`). The edge is rebuilt by `physis gaps` / `physis why` and
is not part of the statement hash.

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
physis set de-rham shape sphere   # S²: b₁ = 0, b₂ = 1, χ = 2; Poincaré still holds
physis hypothesize de-rham        # add-sign-flip is IR, not set
physis encode de-rham             # coboundary identity; not P3S, not a kernel proof
```

Changing the `shape` changes the topology. The first Betti number, computed from
the ranks of the incidence matrices, is `0` for the disk, `1` for the circle,
and `2` for the torus, and `dec.closed-equals-exact` flips `holds → fails` once a
hole appears: a closed 1-form that is not exact now exists. The torus (a
triangulated 3×3 flat torus: 9 vertices, 27 edges, 18 triangles) is a
non-trivial check — `b₀ = 1`, `b₁ = 2`, `b₂ = 1`, `χ = 0`, all computed from the
coboundary, with the Hodge Laplacian's harmonic dimension matching `b₁ = 2`.
Topology is detected mechanically, by linear algebra on the coboundary.

## Two invariants, only one of them a second path

- **Euler–Poincaré** (`dec.euler-poincare`): the Euler characteristic from cell
  counts, `χ = V − E + F`, equals the alternating sum of Betti numbers,
  `b₀ − b₁ + b₂`. Disk: `χ = 1`; circle: `χ = 0`. With the rank-nullity
  formulas used here (`b₀ = V−rank(d₀)`, `b₁ = E−rank(d₁)−rank(d₀)`,
  `b₂ = F−rank(d₁)`), that equality is **cancellation**, not a second
  algorithm. The cell `holds` as `executed`. It does not mint P2.
- **Hodge theorem** (`dec.hodge-harmonic`): the dimension of harmonic 1-forms —
  the nullity of the combinatorial Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁` — equals
  `b₁`. Disk: `0`; circle: `1`. That is a different matrix from the coboundary
  rank formula. The cell names that discrete regime (finite simplicial
  1-cochains), not the smooth Hodge theorem on a Riemannian manifold.
  Agreement overlays `DerivationAssurance::CrossChecked` (P2),
  not a Lean receipt, not P3N, and not P4. Forgetting the up or down term of
  `Δ₁` disagrees with `b₁`. A mismatch `fails` and does not mint P2.

Poincaré (`dec.closed-equals-exact`) stays a single `b₁ = 0` check
(`executed`); it is not this overlay.

Both identities `hold` on every shape alike; only the numbers change
with the `shape` knob.

## Non-orientability: the Klein bottle vs the torus

The `klein` shape is a triangulated Klein bottle (a 4×4 grid glued into a torus
in one direction and with a *flip* in the other). It is the sharpest homology
contrast in the lab: it shares the torus's Euler characteristic `χ = 0`, but

- `b₁ = 1`, not 2 — the integral `H₁(K;ℤ) = ℤ ⊕ ℤ/2` carries a `ℤ/2` **torsion**
  summand that is *invisible to real coefficients*, so the rank computation sees
  only one 1-cycle; and
- `b₂ = 0`, not 1 — a non-orientable surface has no fundamental class over `ℝ`.

So two closed surfaces with identical `χ` are told apart mechanically by their
Betti numbers, and the disappearance of the `ℤ/2` torsion under real
coefficients is made concrete. Each construction is validated by
`Complex::is_closed_surface` (every edge borders exactly two triangles).

## The 2-sphere vs the disk

The `sphere` shape is the boundary of a tetrahedron: 4 vertices, 6 edges, 4
triangles. It is the simplest closed orientable surface, and it is the
homology contrast to the disk (which shares `b₁ = 0`):

- `χ = 2`, not 1
- `b₂ = 1`, not 0 — a fundamental class over ℝ
- every edge borders exactly two triangles (closed), which the disk does not

So `set de-rham shape sphere` flips `dec.fundamental-class` **fails → holds**
and does *not* flip Poincaré: both the disk and `S²` are simply connected.
The torus also has `b₂ = 1`, but `b₁ = 2` and `χ = 0`; the sphere is how the
lab tells "a 2-cycle" apart from "a 1-hole."

## Non-goals (this milestone)

- Higher-dimensional complexes (only vertices/edges/triangles, grades 0–2; the
  torus is a genuine closed surface, but simplices stop at triangles).
- The Hodge *star* and codifferential as first-class operators (the Laplacian
  is built here directly from `d`; a metric-dependent star is a later increment).
- General mesh input; the complexes are disk, circle, torus, Klein bottle, and tetrahedron `S²`.

## Related

- `specs/001-type-system.md` — the `Qty<D>` type-level contracts this mirrors
- `specs/008-electromagnetism.md` — Maxwell's equations numerically
- `specs/002-ontology-layers.md` — the mathematical layer
