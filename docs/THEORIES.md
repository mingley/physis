# Theories

A theory is not a PDF. It is an object that can be wrong *inside the lab*.

## Controls

**Standard Model.** The thing that actually describes collider physics. Fails gravity. Fails UV-completion. Holds three generations (until you turn the knob).

**General relativity.** The thing that actually describes gravity. Grazing solar deflection 1.75″ and Mercury's 43″ perihelion are computed Schwarzschild integrals. Fails SM matter. Fails perturbative UV-completion.

**Newtonian gravity.** Inverse-square control. Holds Soldner's 0.87″ half-angle and closed ellipses; fails Eddington and the 43″ remainder.

## String constructions

`StringTheory` is parameterized by `StringKind`. Critical dimension is a theorem of the construction. Landscape uniqueness is a heuristic that grows with extra dimensions and `flux_bits`.

The bosonic string is included because it is the cleanest *theorem-level* failure mode we have (tachyon, no fermions). Superstrings are the serious candidates; they should not be confused with it.

## Observer geometry

A scaffold for programs that say: start from geometry, demand uniqueness, try to derive the gauge group. Default `total_dim=14` echoes public discussion of 4D plus a 10D fibre. It is **not** a derivation and **not** Geometric Unity.

This object exists so uniqueness can sit on the same matrix as the string landscape without anyone pretending a podcast is a proof.

## Cavity radiation

**Rayleigh–Jeans** is classical equipartition applied to electromagnetic cavity modes — a standing 19th-century theory. It holds its axiom (`thermo.mode-equipartition`) and fails the observations (finite energy, Stefan–Boltzmann T⁴, Wien peak). **Planck** replaces the mode energy with Bose occupation; the same typed integrals then hold those observations and fail classical equipartition. `set planck quantum false` is the 1900 revolution run backwards.

## Solid heat capacity

**Dulong–Petit** is classical equipartition applied to 3N lattice oscillators — an 1819 standing theory. It holds `C_V = 3 N k` at every T and fails the third law. **Einstein** (1907) uses the same Bose factor as Planck; `C_V → 0` exponentially as T → 0, and `C_V → 3 N k` only for `T ≫ Θ_E`. That exponential over-suppresses `C_V` relative to the observed `T³` phonon law. **Debye** (1912) puts an `ω²` density of states under the same Bose factor; `C_V(2T)/C_V(T) = 8` at `T = Θ_D/20` is a computed theorem. `set einstein-solid spectrum debye` is that 1912 correction as a knob turn. `set einstein-solid temperature 4000` is the high-T correspondence.

## Olbers' paradox

**Static Euclidean starlight** is inverse-square cancellation applied to an infinite, eternal, uniformly filled sky — a standing 19th-century theory. It holds its axiom (`astro.shell-cancellation`: `dF/dr` independent of `r`) and fails the observations (finite integrated brightness, a dark night sky). A **finite-age horizon** keeps the axiom and holds those observations: `F = ρ_L c t`, `τ ~ 10⁻¹⁵` at a Hubble time. **Hubble dimming** is an independent knob: `dF = ρ_L dr / (1+z)²` saturates at `ρ_L c/H`, so cancellation *fails* and the sky stays dark. **Tired light** is an IR mutation on `olbers-static`, not those knobs: `dF ∝ e^{-Hr/c} dr` fails cancellation and caps energy, while covering `τ = n σ R` still diverges. `set olbers-static finite_age true` is the finite-age resolution. `set olbers-horizon age_yr 1e26` is the reminder that a merely finite universe is not automatically dark.

## Adding one

Implement `Knobbed + Theory`, reuse claim ids, add a knob-diff test, register in `Lab::standard` only if it belongs in the default comparison.
