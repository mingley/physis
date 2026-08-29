# M2 — Empirical contact that is not a table

Goal: replace the most embarrassing `EncodedFact` tables with typed checks, and give empirical rows teeth.

## Work

1. **Root systems / Dynkin.** ◑ *Partly done (M2.1).* SM containment is now *verified by code* — `GaugeGroup::verified_contains_sm` walks the standard maximal-subgroup chain (E₈ ⊃ E₆ ⊃ SO(10) ⊃ SU(5) ⊃ SM, plus SO(32) ⊃ SO(10) and Spin(10) ⊃ SU(5)) and checks the necessary rank and dimension inequalities at each step, replacing the old `== GaugeGroup::su5()` equality table. `sm_embed` and the `empirical.sm-gauge` verdicts now carry the verified chain as evidence. **Retag policy:** these stay `EncodedFact`, not `Theorem` — rank/dimension are necessary but not sufficient, and the chain of maximal subgroups is still encoded. Full root-system / Dynkin branching (which would justify `Theorem`) remains open.
2. **Neutrino masses.** ✅ *Done (M2.2).* Added the `neutrino_masses` knob to `standard-model` and the `empirical.neutrino-masses` claim. The minimal SM (default `false`) now *fails* this claim honestly — "stores neutrino masses as 0, but oscillations prove they are nonzero" — instead of silently storing 0; flipping the knob to `true` makes it hold. Knob-diff test in `standard_model.rs`.
3. **Coupling constants as quantities.** ✅ *Done (M2.3).* `fine_structure_constant` (α) and `strong_coupling_mz` (α_s) are `Qty<Dimensionless>`; `fermi_coupling` (G_F) is a typed `energy⁻²` quantity. The type system enforces it: `G_F · E · E` type-checks to `Qty<Dimensionless>` by construction, and multiplying G_F by anything else is a compile error. Running the couplings with energy is deferred to M4. Test in `constants.rs`.
4. **Electroweak scale vs compactification radius.** ✅ *Done (M2.4).* `empirical.hidden-extra-dims` now compares a typed effective radius `Qty<Length>` (Kähler volume × √g_s × Planck length) against `Scale::Electroweak.typical_length()`, instead of a raw float against `1e16`. See `StringTheory::effective_radius`.
5. **A tiny empirical fixture file.** ✅ *Done (M2.5).* `data/empirical-world.json` states the low-energy requirements (observed 3+1, gauge ⊃ SM, chiral fermions, three generations, gravity). `physis_theory::target` parses it and `score(target, theory)` grades any theory's projected `World` against it; `physis score <theory>` prints the scorecard. Results are honest and illustrative: heterotic E₈×E₈ scores 5/5, the SM misses only gravity (4/5), Type IIB misses the gauge sector, GR scores 2/5. The target is now versioned data, not a function body.

## M2 status

All five work items are complete and both "Done when" criteria are met. Next: M3 (`plans/004`).

## Retag policy

When a table becomes a check:

- `EncodedFact` → `Theorem` only if the check is actually a proof in this model
- Keep evidence lines with the literature name (Georgi–Glashow, etc.)

## Done when

- ✅ SM ⊂ SU(5) is verified by code, not `== GaugeGroup::su5()` (M2.1)
- ✅ Hidden extra dims uses `Qty<Length>`, not a raw float compared to `1e16` (M2.4)
