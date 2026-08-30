# 010 — Continuum

Status: active
Layer: field
Id: `field-modes`

## Purpose

M4's seed: represent a **field as an actual local object**, not a boolean flag.
`klein-gordon` is a real scalar field on a finite 1D periodic lattice of `N`
sites coupled by a nearest-neighbour discrete Laplacian. Its normal modes are
*computed*, so stability, causality, and the continuum dispersion are theorems
of the computation rather than tabulated facts.

## Objects

| id | object | experiment |
|---|---|---|
| `klein-gordon` | real scalar field on an N-site 1D periodic lattice | `field-modes` |
| `wilson-u1` | compact U(1) lattice gauge field (links + plaquettes) | `gauge-lattice` |
| `wilson-su2` | non-abelian SU(2) Wilson lattice gauge field | `gauge-lattice` |
| `wilson-su3` | non-abelian SU(3) Wilson lattice gauge field (QCD group) | `gauge-lattice` |

## Knobs

| knob | effect |
|---|---|
| `sites` | number of lattice sites N (the local degrees of freedom) |
| `mass_squared` | m² in natural units; **negative values make the zero mode tachyonic** |
| `spacing` | lattice spacing a |

## Computed spectrum

```
ω_j² = m² + (4/a²) · sin²(π j / N),   j = 0 … N-1
```

There is nothing tabulated here: the module computes ω_j² for every mode.

## Claims

| id | meaning | how it is decided |
|---|---|---|
| `field.finite-modes` | N normal modes | N = `sites` |
| `field.dispersion-continuum-limit` | long-wavelength ω² matches m² + k² | computed relative error < 5% on the longest non-zero mode. Domain: that mode, not Nyquist, not the Richardson `|k a| < 1` probe |
| `field.stable` | no tachyonic mode | `min_j ω_j² ≥ 0` |
| `field.causal` | group velocity ≤ c | `max_j |dω/dk| ≤ c` |
| `field.local` | nearest-neighbour coupling | structural |
| `field.second-order-accurate` | discretization error ∝ a² | computed Richardson order p ≈ 2 when `|k a| < 1` at a fixed probe k. If `|k a| ≥ 1`, **undecidable** / `inconclusive` (`InsufficientPrecision`): too coarse to certify the stencil, not a failed theorem. The 1.8–2.2 window is not P3N. |

## Knob → verdict

```
physis experiment field-modes
physis set klein-gordon mass_squared -1
```

`mass_squared: 1 → -1` flips `field.stable` `holds → fails` (the zero mode has
`ω² < 0`) and also `field.causal` `holds → fails` (a tachyonic mode has
imaginary frequency and an ill-defined, effectively superluminal group
velocity). This is the *same* instability notion as the bosonic string's
tachyon (`consistency.no-tachyon` in the string lab), but here it is computed
from the lattice spectrum rather than encoded.

## Gauge field on links: `wilson-u1`

A compact U(1) lattice gauge theory whose degrees of freedom live on the links;
the Wilson action sums `1 − cos(θ_plaquette)`. Knobs: `dimension` (2–4), `beta`
(β = 1/g²), `sites_per_side`.

| claim | meaning | how it is decided |
|---|---|---|
| `gauge.invariant` | invariance under `U_μ(x) → g(x) U_μ(x) g(x+μ̂)†` | structural theorem |
| `gauge.local` | only neighbouring links couple (plaquettes) | structural theorem |
| `gauge.confining` | static charges are confined | U(1): encoded in 2D/3D, heuristic across the 4D transition near β ≈ 1.01. SU(N): encoded in 2D/3D, **conjecture** in 4D (mass gap) |
| `gauge.asymptotic-freedom` | the coupling runs to zero at high energy | U(1) `fails` (Landau pole); SU(N) `holds` (Gross–Wilczek–Politzer) |
| `gauge.strong-coupling-area-law` | leading strong-coupling expansion gives an area law | **computed**: `σ = −ln(β/2N²) > 0` (theorem of the convergent expansion); fails at weak coupling |
| `gauge.exact-area-law-2d` | 2D gauge theory confines at all couplings | **exactly computed**: U(1) via `σ = −ln(I₁(β)/I₀(β))`, SU(N) via `σ = (N²−1)/(2β)` (quadratic Casimir); `> 0` for every β (theorem); `inapplicable` in D > 2 |

### Abelian vs non-abelian

The lab contrasts compact U(1) (QED-like) with SU(2)/SU(3) Yang–Mills:

- U(1) is **not** asymptotically free and **deconfines** in 4D above β ≈ 1.01.
- SU(N) **is** asymptotically free and is *expected* to confine in 4D — but 4D
  Yang–Mills existence and the mass gap are unproven (a Clay Millennium
  Problem), so `gauge.confining` for SU(N) in 4D `holds` with epistemic tag
  `conjecture`, not `theorem`. This is the honesty discipline again: a famous
  open problem is recorded as open.

In two dimensions the gauge theories are **exactly solvable**. For compact U(1)
the gauge integral factorizes plaquette by plaquette, so the fundamental Wilson
loop of area `A` is exactly `⟨W⟩ = (I₁(β)/I₀(β))ᴬ = e^{−σA}` with
`σ = −ln(I₁(β)/I₀(β))`. Because `0 < I₁/I₀ < 1` for every finite `β`, `σ > 0`
always — 2D compact U(1) confines at *all* couplings, a genuine theorem
(`gauge.exact-area-law-2d`, computed from the modified Bessel-function ratio by a
convergent series).

The non-abelian case is equally exact (2D Yang–Mills is solvable, Migdal/Witten):
the fundamental Wilson loop has string tension `σ = (g²/2)·C₂(fund) = (N²−1)/(2β)`
from the quadratic Casimir, positive for every finite `β`, so 2D SU(N) confines
at all couplings too — a **theorem**, in pointed contrast to the 4D mass gap,
which stays a `conjecture`. Both are `inapplicable` in `D > 2`, where the
factorization fails. `set wilson-u1 dimension 2` (or `set wilson-su3 dimension 2`)
turns this claim from `inapplicable` to `holds`.

The `gauge.confining` verdict is the *physical* claim (heuristic/conjecture);
`gauge.strong-coupling-area-law` is the **computed** companion — the first term
of the convergent strong-coupling expansion, `σ = −ln(β/2N²)`, which is a
genuine area-law theorem where `σ > 0` and fails once the coupling is too weak
for the expansion.

```
physis experiment gauge-lattice
physis set wilson-u1 beta 2         # 4D U(1): confining → Coulomb (deconfined)
physis set wilson-u1 dimension 3    # U(1) confines at any β
physis set wilson-su3 beta 100      # weak coupling: strong-coupling area law fails
```

## Non-goals (this milestone seed)

- A full 3+1 lattice field theory or a real-time PDE solver.
- Interactions / renormalization.
- Gauge fields on links (Wilson) — planned later in M4.

## Spacetime projection

`Theory::world()` returns `Option<World>`. `klein-gordon` reports an honest
1+1-dimensional world (one time direction, one spatial lattice direction)
rather than borrowing 3+1 Minkowski; `wilson-u1` reports its lattice dimension.
Non-physics domains (computation) return `None`. This replaced the earlier
placeholder-world rough edge.

## Related

- `plans/005-m4-continuum.md`
- `specs/002-ontology-layers.md` (the `field` layer)
