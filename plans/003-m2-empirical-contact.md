# M2 — Empirical contact that is not a table

Goal: replace the most embarrassing `EncodedFact` tables with typed checks, and give empirical rows teeth.

## Work

1. **Root systems / Dynkin.** Enough to *verify* (not discover) that SM embeds in SU(5), SO(10), E₆, E₈, Spin(10). Then retag those verdicts.
2. **Neutrino masses.** Stop storing ν mass as 0 without a claim. Add a knob `neutrino_masses` and a claim.
3. **Coupling constants as quantities.** `α`, `α_s`, `G_F` as `Qty` / dimensionless. Running is M4; existence is M2.
4. **Electroweak scale vs compactification radius.** Replace the `1e16` magic cut with typed lengths (`Scale::Electroweak` vs `R`).
5. **A tiny empirical fixture file.** JSON of “what we require of a low-energy world” that theories are scored against, so the target is data, not a function named `empirical_target()`.

## Retag policy

When a table becomes a check:

- `EncodedFact` → `Theorem` only if the check is actually a proof in this model
- Keep evidence lines with the literature name (Georgi–Glashow, etc.)

## Done when

- SM ⊂ SU(5) is verified by code, not `== GaugeGroup::su5()`
- Hidden extra dims uses `Qty<Length>`, not a raw float compared to `1e16`
