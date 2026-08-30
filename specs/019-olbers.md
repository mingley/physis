# 019 — Olbers' paradox (static Euclidean sky vs a horizon)

Status: active
Layer: spacetime / statistical
Id: `olbers`

## Purpose

Put a long-standing 19th-century cosmology on trial with the typed substrate.
An infinite, eternal, Euclidean universe uniformly filled with stars is not a
slogan in this lab: it is an object whose claims are *computed*. Inverse-square
dilution of each star is cancelled by the area of the spherical shell it sits
on, so the flux contribution of a shell is independent of radius: `dF = ρ_L dr`.
Integrate to infinity and the night sky is as bright as a stellar surface.

Two independent resolutions sit on the same knobs:

- a finite age: light has only travelled `c t`
- Hubble dimming: `dF = ρ_L dr / (1 + H r/c)²`, saturating at `ρ_L c / H`

This is Olbers' paradox as a knob → verdict diff, not a textbook paragraph.

## Objects

| id | object | default knobs |
|---|---|---|
| `olbers-static` | infinite static Euclidean starlight (standing theory) | `finite_age=false`, `expanding=false` |
| `olbers-horizon` | finite-age Euclidean sky | `finite_age=true`, `expanding=false` |

Both objects share knobs. The id is fixed at construction: `set olbers-static finite_age true`
is the finite-age resolution on the standing object without renaming it.
`set olbers-static expanding true` is Hubble dimming, an independent flip.

## Knobs

| knob | layer | effect |
|---|---|---|
| `finite_age` | spacetime | true: light-travel horizon at `c t`. Turning this on is the finite-age resolution. |
| `expanding` | spacetime | true: linear Hubble flow, `z = H r/c`. Independent of finite age. |
| `age_yr` | spacetime | cosmic age in years. Used when `finite_age` is true. Making the universe old enough that `τ = n σ c t ≳ 1` makes the sky photosphere-bright again. |
| `cutoff_m` | effective | radial cutoff in metres. Standing-theory verdicts use the improper `R → ∞` limit, not this cutoff (a large but finite cutoff can still look dark). Weakly live: it appears in the world note as `R_eff` when no horizon is tighter. |

## Claims

| id | meaning | static Euclidean | finite-age horizon |
|---|---|---|---|
| `astro.shell-cancellation` | `dF/dr` independent of `r` | **holds** (axiom) | **holds** (same Euclidean shells; expansion is the knob that breaks this) |
| `astro.sky-finite` | integrated brightness stays finite as the radial cutoff is removed | **fails** (`F ∝ R`, `F(2R)/F(R) = 2` at `R = c t`, independent of cutoff) | **holds** (`F = ρ_L c t`) |
| `astro.night-sky-dark` | night sky far dimmer than a stellar photosphere (`τ ≪ 1`) | **fails** (`τ = n σ R → ∞`) | **holds** (`τ ~ 10⁻¹⁵` at a Hubble time) |

The standing theory holds its own axiom and fails the observations. A finite
age keeps the axiom and holds the observations. Hubble dimming **fails** the
axiom (`dF/dr` falls as `1/(1+z)²`) and holds the observations. That is the
challenge: two independent resolutions, one of which keeps the 19th-century
shell theorem.

## What is computed (theorems, not tables)

- Static unocculted flux `F = ρ_L R`, as `Qty<Irradiance>` (`ρ_L` is
  `Qty<LuminosityDensity>`; times a length, irradiance).
- Expanding flux `F = (ρ_L c/H) [1 − 1/(1 + H r/c)]`. Linear Hubble flow, not
  a full FLRW integral. Saturates at `ρ_L c/H`. Probe:
  `F(100 c/H)/F(c/H) ≈ 2` (static Euclidean at the same radii gives 100).
- Optical depth `τ = n σ R` with `n = ρ_L / L_☉` and `σ = π R_☉²`.
  Dimensionless by construction: number density × area × length.
- Night-sky compare: unocculted `F` versus `σ T⁴` at 5772 K, using the typed
  Stefan–Boltzmann constant.
- Standing-theory `sky-finite` and `night-sky-dark` use the improper
  `R → ∞` limit (sampled by doubling a Hubble-time probe, or by `τ → ∞`),
  not the current `cutoff_m`. Same lesson as Planck's improper `u_∞ = a T⁴`.

Irradiance is *not* energy, and luminosity density is *not* irradiance:
assigning either to the other is a compile-fail contract in `physis-core`.

## Knob → verdict

```
physis experiment olbers
physis run olbers-static
physis set olbers-static finite_age true   # finite-age resolution
physis set olbers-static expanding true    # Hubble dimming (fresh lab: both catastrophes flip, and cancellation fails)
physis set olbers-horizon age_yr 1e26      # finite but ancient: τ ≳ 1, sky photosphere-bright
```

`set olbers-static finite_age true` flips `astro.sky-finite` and
`astro.night-sky-dark` fails → holds, and does **not** touch
`astro.shell-cancellation`.

`set olbers-static expanding true` flips cancellation holds → fails *and*
the two catastrophe cells fails → holds.

## Honesty

- Linear Hubble `z = H r/c` is not a full FLRW integral (no scale-factor
  history, no cosmological redshift of the source spectrum beyond the
  `1/(1+z)²` energy-and-rate dimming of the Euclidean shells).
- `ρ_L` is a cosmic-mean luminosity density (~10⁸ L_☉/Mpc³), not the solar
  neighbourhood packed out to infinity. Optical depth therefore uses a
  mean stellar-disk covering, not a galaxy-survey luminosity function.
- `H₀ ≈ 70 km s⁻¹ Mpc⁻¹` is order-of-magnitude cosmology, not a precision fit.
- The unocculted flux `F = ρ_L R` overestimates once `τ ≳ 1`; the night-sky
  claim fails on `τ` (and on `F/σT⁴`) before that modelling hole matters.
- A `Holds` here is a theorem of *this* encoding, not a declaration that the
  night sky is dark in nature because we said so.

## Non-goals (this increment)

- A real FLRW luminosity-distance integral, or Olbers in ΛCDM.
- Dust, interstellar absorption, or a galaxy luminosity function.
- Olbers as a constraint on the cosmological constant.
- Surface-brightness conservation in expanding space as its own claim
  (the `1/(1+z)²` factor is the Euclidean-shell encoding of that fact).

## Related

- `specs/016-blackbody.md` (standing 19th-c theory; improper integral independent of cutoff)
- `specs/018-light-deflection.md` (another spacetime-layer computed theorem)
- `specs/001-type-system.md` (`Irradiance` vs `Energy`; `LuminosityDensity` vs `Irradiance`)
- `specs/002-ontology-layers.md` (spacetime + statistical)
