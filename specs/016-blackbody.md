# 016 — Blackbody radiation (Rayleigh–Jeans vs Planck)

Status: active
Layer: statistical / quantum
Id: `blackbody`

## Purpose

Put a long-standing 19th-century theory on trial with the typed substrate.
Classical statistical mechanics applied to the electromagnetic modes of a
cavity (Rayleigh 1900, Jeans 1905) is not a slogan in this lab: it is an
object whose claims are *computed*. Equipartition assigns every mode energy
`kT`. The integrals then do what they do — and what they do is fail.

Planck's 1900 replacement — Bose occupation `hν / (e^{hν/kT} − 1)` — is the
same integrals with a different mean energy. Finite `u = a T⁴`, Wien's peak,
and the infrared Rayleigh–Jeans limit fall out as theorems.

This is the ultraviolet catastrophe as a knob → verdict diff, not a textbook
paragraph.

## Objects

| id | object | default `quantum` |
|---|---|---|
| `rayleigh-jeans` | classical cavity radiation (standing theory) | false |
| `planck` | Bose-occupied cavity radiation (1900 resolution) | true |

Both objects share knobs. The id is fixed at construction: `set planck quantum false`
restores Rayleigh–Jeans *physics* on the Planck object without renaming it.

## Knobs

| knob | layer | effect |
|---|---|---|
| `quantum` | quantum | true: Planck; false: every mode has energy `kT` |
| `temperature` | statistical | cavity temperature (K) |
| `cutoff_hz` | effective | ultraviolet frequency cutoff (Hz) |

## Claims

| id | meaning | Rayleigh–Jeans | Planck |
|---|---|---|---|
| `thermo.mode-equipartition` | a UV mode (`hν = 8 kT`) still carries `kT` | **holds** (axiom) | **fails** (freeze-out) |
| `thermo.uv-finite` | `∫_0^∞ u(ν) dν` converges | **fails** (`u ∝ ν_max³`, ratio 8) | **holds** (`u_∞ = a T⁴`, even if the current cutoff is still infrared) |
| `thermo.stefan-boltzmann` | `u(2T)/u(T) = 16` at fixed bandwidth | **fails** (`u ∝ T`, ratio 2) | **holds** (`u = a T⁴`) |
| `thermo.wien-displacement` | finite `λ_max` with `λ_max T` constant | **fails** (`u(λ) ∝ λ⁻⁴`, no peak) | **holds** (computed peak = `hc/(k x)`) |
| `thermo.rj-ir-limit` | `hν ≪ kT` matches Rayleigh–Jeans | **holds** (identity) | **holds** (correspondence) |

The standing theory holds its own axiom and fails the observations. The
resolution fails the axiom and holds the observations. That is the challenge.

## What is computed (theorems, not tables)

- Mode energy: `kT` vs `hν/(e^{hν/kT}−1)`, as `Qty<Energy>` (`h · ν` is energy
  by construction).
- Spectral density `u(ν) = (8πν²/c³) · ⟨E⟩`, as `Qty<SpectralEnergyDensity>`.
- Integrated `u` to a cutoff: analytic `8π kT ν_max³ / (3 c³)` classically.
  Planck's *improper* integral is the Bose trapezoid out to `x = 40` (the tail
  is negligible), checked against `π⁴/15` and against typed `a T⁴`. Verdicts
  for `thermo.uv-finite` and `thermo.stefan-boltzmann` use this improper
  integral, so they do not silently fail when the `cutoff_hz` knob sits in
  the infrared.
- Stefan–Boltzmann constant `σ = 2π⁵ k⁴ / (15 h³ c²)` *derived* from exact SI
  `h`, `k_B`, `c`, typed as `Qty<StefanBoltzmann>`. Radiation constant
  `a = 4σ/c`. Photon-gas energy density `u = a T⁴` is `Qty<EnergyDensity>`.
- Wien root `x = 5(1−e^{-x})` and `λ_max T = hc/(k x)`, checked against a
  ternary search on sampled `u(λ)`. Rayleigh–Jeans is a computed *absence* of
  an interior peak: the sampled maximum sits at the UV endpoint of the window.

Energy density is *not* energy: assigning `Qty<EnergyDensity>` to `Qty<Energy>`
is a compile-fail contract in `physis-core`.

## Knob → verdict

```
physis experiment blackbody
physis run planck
physis set planck quantum false   # ultraviolet catastrophe restored
physis run rayleigh-jeans
```

## Honesty

- The Rayleigh–Jeans *failure* is a real property of classical equipartition
  applied to an unbounded frequency tower, not a modelling shortcut.
- Planck's law here is the photon-gas Bose integral, not a full QED derivation
  of blackbody radiation from the SM. The epistemic tag is `theorem` of *this*
  encoding.
- The `rj-ir-limit` correspondence is the statement that Planck contains
  Rayleigh–Jeans at low frequency; it is not a claim that classical theory is
  "approximately true in the UV."

## Non-goals (this increment)

- Einstein / Debye solids (Dulong–Petit vs quantum oscillators) — a sibling
  standing-theory challenge, not this object.
- A real cavity-mode solver or measured lamp spectrum.
- Photon shot noise, Lamb shift, or other QED corrections.

## Related

- `specs/011-thermodynamics.md` (ideal gas; same statistical layer)
- `specs/012-quantum-foundations.md` (another standing theory — local realism — on trial)
- `specs/001-type-system.md` (`EnergyDensity` vs `Energy`)
- `specs/008-electromagnetism.md` (the U(1) field whose modes are being counted)
