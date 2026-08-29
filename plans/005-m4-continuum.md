# M4 — Continuum (fields as local objects, not flags)

Goal: stop representing "fields" as boolean flags (`supersymmetry`, `kind`) and
start representing at least one field as an **actual local object** with degrees
of freedom, computed dynamics, and claims that are theorems *of the computation*
rather than tables.

Per `specs/002` and the roadmap, this will "almost certainly never be full":
finite modes first, continuum limit as a checked approximation.

## First increment (this milestone's seed) — ✅ done

New theory `klein-gordon`: a real scalar field on a finite 1D periodic lattice.

- Knobs: `sites` (N), `mass_squared` (m², may be negative), `spacing` (a).
- Local object: nearest-neighbour discrete Laplacian on N sites, so the field
  has real, computed normal modes `ω_j² = m² + (4/a²) sin²(π j / N)`.
- Claims computed from the lattice, not tabulated:
  - `field.finite-modes` — N normal modes.
  - `field.dispersion-continuum-limit` — the long-wavelength mode matches the
    continuum `ω² = m² + k²` within tolerance (a theorem of the computation).
  - `field.stable` — no tachyonic mode (`min ω² ≥ 0`); a negative `mass_squared`
    knob flips it to `fails`, a real computed instability.
  - `field.causal` — the group velocity is bounded by c.
  - `field.local` — nearest-neighbour coupling.
- Experiment `field-modes`.

## Gauge field on links — ✅ done

New theory `wilson-u1`: compact U(1) lattice gauge theory (`gauge_field.rs`,
experiment `gauge-lattice`). The gauge field lives on links; the action sums
`1 − cos(θ)` over plaquettes.

- `gauge.invariant` and `gauge.local` are structural theorems of the Wilson
  construction.
- `gauge.confining` is a lattice-gauge result: a theorem (encoded) at all β in
  2D/3D, and a knob-sensitive heuristic in 4D across the transition near
  `β ≈ 1.01`. `set wilson-u1 beta 2` deconfines the 4D theory (Coulomb phase).

## Later in M4

- 3+1 lattice and a non-abelian gauge field on links (Wilson-style), still finite.
- Connect the scalar tachyon here to the string bosonic tachyon conceptually
  (both are `min ω² < 0`), so the two domains share a stability notion.
- A `Layer`-based observable so a field's world projection is not the
  physics-shaped `World` placeholder (also unblocks the computation domain's
  documented rough edge).

## Done when (seed)

- `physis experiment field-modes` prints a matrix
- `set klein-gordon mass_squared -1` flips `field.stable` holds → fails
- Spec `specs/010-continuum.md` exists
