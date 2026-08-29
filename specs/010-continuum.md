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
| `field.dispersion-continuum-limit` | long-wavelength ω² matches m² + k² | computed relative error < 5% |
| `field.stable` | no tachyonic mode | `min_j ω_j² ≥ 0` |
| `field.causal` | group velocity ≤ c | `max_j |dω/dk| ≤ c` |
| `field.local` | nearest-neighbour coupling | structural |

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
| `gauge.confining` | static charges are confined | theorem (encoded) in 2D/3D; heuristic across the 4D transition near β ≈ 1.01 |

```
physis experiment gauge-lattice
physis set wilson-u1 beta 2         # 4D: confining → Coulomb (deconfined)
physis set wilson-u1 dimension 3    # confines at any β
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
