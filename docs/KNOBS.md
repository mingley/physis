# Knobs

See `specs/003-knobs-and-causality.md`.

Knobs are the only way an agent is allowed to change a theory. They have:

- a stable name
- a layer
- a domain
- a one-line `doc` that says what turning them *means*

## Inventory (default lab)

### standard-model
- `generations` (1–4)
- `include_higgs`
- `include_gravity`

### general-relativity
- `dim`
- `cosmological_constant` (Planck units, order-of-magnitude)

### type-iib / heterotic-e8e8 / bosonic
- `kind`
- `total_dim`
- `observed_dim`
- `compact_radius_planck`
- `supersymmetry`
- `flux_bits`

### observer-geometry
- `total_dim` (default 14, scaffold choice)
- `observed_dim`
- `derive_gauge`
- `unique_vacuum`

## Dead knobs

If you find a knob that no claim reads, either wire it or delete it. v0 `cosmological_constant` is *weakly* dead (it appears in the world note, not in a verdict). That is an accepted M0 hole; M1 should give Λ a claim or drop the knob.
