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

### string / M constructions (type-iib, type-iia, type-i, heterotic-e8e8, heterotic-so32, bosonic, m-theory)
- `kind`
- `total_dim`
- `observed_dim`
- `compact_radius_planck` — overall Kähler volume (size) modulus, in Planck lengths
- `supersymmetry`
- `flux_bits`
- `dilaton` — φ; string coupling g_s = e^φ (inflates the effective compact size)
- `h11` — Kähler (size) moduli count, heuristic stand-in for h^{1,1}
- `h21` — complex-structure (shape) moduli count, heuristic stand-in for h^{2,1}

`unique-vacuum` depends on `flux_bits`, `h11`, and `h21` (zero flux **or** zero
moduli ⇒ no landscape ⇒ uniqueness holds). `hidden-extra-dims` depends on the
effective radius `compact_radius_planck · √g_s`, so both the Kähler volume and
the dilaton can expose extra dimensions.

### observer-geometry
- `fibre_dim` (default 10 — minimal fibre that can host Spin(10); total = observed + fibre)
- `observed_dim`
- `derive_gauge`
- `unique_vacuum`

The total geometric dimension is `observed_dim + fibre_dim` (default `4 + 10 = 14`),
not a magic literal. Setting `fibre_dim < 10` with `derive_gauge=true` makes
`empirical.sm-gauge` fail: Spin(10) has no geometric room in a smaller fibre.

## Dead knobs

If you find a knob that no claim reads, either wire it or delete it. v0 `cosmological_constant` is *weakly* dead (it appears in the world note, not in a verdict). That is an accepted M0 hole; M1 should give Λ a claim or drop the knob.
