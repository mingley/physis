# Theories

A theory is not a PDF. It is an object that can be wrong *inside the lab*.

## Controls

**Standard Model.** The thing that actually describes collider physics. Fails gravity. Fails UV-completion. Holds three generations (until you turn the knob).

**General relativity.** The thing that actually describes gravity. Fails SM matter. Fails perturbative UV-completion.

## String constructions

`StringTheory` is parameterized by `StringKind`. Critical dimension is a theorem of the construction. Landscape uniqueness is a heuristic that grows with extra dimensions and `flux_bits`.

The bosonic string is included because it is the cleanest *theorem-level* failure mode we have (tachyon, no fermions). Superstrings are the serious candidates; they should not be confused with it.

## Observer geometry

A scaffold for programs that say: start from geometry, demand uniqueness, try to derive the gauge group. Default `total_dim=14` echoes public discussion of 4D plus a 10D fibre. It is **not** a derivation and **not** Geometric Unity.

This object exists so uniqueness can sit on the same matrix as the string landscape without anyone pretending a podcast is a proof.

## Cavity radiation

**Rayleigh–Jeans** is classical equipartition applied to electromagnetic cavity modes — a standing 19th-century theory. It holds its axiom (`thermo.mode-equipartition`) and fails the observations (finite energy, Stefan–Boltzmann T⁴, Wien peak). **Planck** replaces the mode energy with Bose occupation; the same typed integrals then hold those observations and fail classical equipartition. `set planck quantum false` is the 1900 revolution run backwards.

## Solid heat capacity

**Dulong–Petit** is classical equipartition applied to 3N lattice oscillators — an 1819 standing theory. It holds `C_V = 3 N k` at every T and fails the third law. **Einstein** (1907) uses the same Bose factor as Planck; `C_V → 0` as T → 0, and `C_V → 3 N k` only for `T ≫ Θ_E`. `set einstein-solid temperature 4000` is the correspondence as a knob turn.

## Adding one

Implement `Knobbed + Theory`, reuse claim ids, add a knob-diff test, register in `Lab::standard` only if it belongs in the default comparison.
