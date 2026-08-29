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

## Object

| id | object |
|---|---|
| `klein-gordon` | real scalar field on an N-site 1D periodic lattice |

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

## Non-goals (this milestone seed)

- A full 3+1 lattice field theory or a real-time PDE solver.
- Interactions / renormalization.
- Gauge fields on links (Wilson) — planned later in M4.

## Known limitation

Like the computation domain, a field does not have a meaningful physics-shaped
`World` projection; `klein-gordon` uses a placeholder world whose `note` carries
the computed spectrum. Generalizing `Theory`'s projection is tracked in
`plans/005`.

## Related

- `plans/005-m4-continuum.md`
- `specs/002-ontology-layers.md` (the `field` layer)
