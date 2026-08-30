# 017 — Solid heat capacity (Dulong–Petit vs Einstein vs Debye)

Status: active
Layer: statistical / quantum
Id: `solid`

## Purpose

Put the 1819 standing theory of solid heat capacity on trial. Dulong and Petit
observed `C_V ≈ 3 R` per mole for many solids and the kinetic theory of the
day explained it: each atom is three classical oscillators, equipartition
assigns `kT` per oscillator, so `C_V = 3 N k` *independent of temperature*.

That law is false at low T. Heat capacities vanish as T → 0, which is what
the third law requires. Einstein (1907) replaced the mode energy with the
same Bose factor Planck used for the cavity: `ħω / (e^{ħω/kT} − 1)`. Then
`C_V → 0` exponentially as T → 0, and `C_V → 3 N k` only for `T ≫ Θ_E`.

Einstein over-suppresses `C_V`. The observed low-T law of insulating crystals
is `C_V ∝ T³`. Debye (1912) replaces the single Einstein frequency with an
acoustic continuum of density of states `g(ω) ∝ ω²` up to `ω_D`, and the
low-T heat capacity is the computed theorem
`C_V = (12/5) π⁴ N k (T/Θ_D)³`.

This is the lattice sibling of `specs/016-blackbody.md`: another 19th-century
equipartition claim, another Bose occupation — and here a second quantum
correction, the phonon continuum, judged by a sampled doubling
`C_V(2T)/C_V(T) = 8`.

## Objects

| id | object | default `quantum` | default `spectrum` |
|---|---|---|---|
| `dulong-petit` | classical 3N oscillators (standing theory) | false | einstein |
| `einstein-solid` | Einstein (1907) Bose oscillators | true | einstein |
| `debye-solid` | Debye (1912) phonon continuum | true | debye |

The lab id is fixed at construction. `set einstein-solid spectrum debye`
changes the physics, not the id.

The 3D `ω²` continuum lives on the IR package of `debye-solid`. A 2D
`ω` continuum (`add-2d`) is not a `spectrum` knob. `einstein-solid` and
`dulong-petit` have no package.

## Knobs

| knob | layer | effect |
|---|---|---|
| `quantum` | quantum | true: Bose occupation; false: every oscillator has energy `kT` |
| `spectrum` | quantum | `einstein` (single `ω`) or `debye` (`ω²` DOS). Ignored classically. A 2D `ω` continuum is not this knob: `add-2d` is an IR mutation on `debye-solid`. |
| `temperature` | statistical | lattice temperature (K) |
| `einstein_temp` | statistical | characteristic `Θ` (K): Einstein `Θ_E = ħω/k` or Debye `Θ_D = ħω_D/k`. Classical physics ignores it |
| `oscillators` | statistical | number of atoms N (3N oscillators) |

Default: `T = 60 K`, `Θ = 300 K` so `T/Θ = 0.2` — deep in the quantum
regime, where Dulong–Petit is most clearly false and Einstein over-freezes
relative to Debye.

## Claims

| id | meaning | Dulong–Petit | Einstein (default T) | Debye (default T) |
|---|---|---|---|---|
| `thermo.dulong-petit` | `C_V = 3 N k` at the current T (encoding-wide) | **holds** | **fails** (`C_V/(3Nk) ≈ 0.17`) | **fails** (frozen acoustic modes) |
| `thermo.high-t-classical` | `T ≫ Θ` recovers `3 N k`. Domain: `T/Θ ≥ 8` | **holds** (always) | **fails** (not yet high-T) | **fails** (not yet high-T) |
| `thermo.third-law` | `C_V → 0` as T → 0 (encoding-wide probe at `Θ/40`) | **fails** (`C_V` stays `3 N k` at `Θ/40`) | **holds** (exponential freeze-out) | **holds** (`T³ → 0`) |
| `thermo.debye-t3` | low-T `C_V ∝ T³`. Domain: `T = Θ/20` 3D `ω²` phonon probe | **fails** (doubling = 1) | **fails** (exponential, doubling ≫ 8) | **holds** (doubling ≈ 8 at `Θ/20`) |

Raising `einstein-solid` or `debye-solid` `temperature` to 4000 K
(`T/Θ ≈ 13`) flips `thermo.dulong-petit` **fails → holds** and
`thermo.high-t-classical` **fails → holds**. The third law and `T³` still
hold or fail as before: they are statements about T → 0, probed at `Θ/40`
and `Θ/20`, not about the current T.

`set einstein-solid spectrum debye` flips `thermo.debye-t3` **fails → holds**
without restoring Dulong–Petit.

`physis hypothesize debye-solid` forks the package with a 2D `ω`
continuum (`equation g(w) = w`); that is not a knob (`set debye-solid two_d
true` is unknown). `C_V(2T)/C_V(T) ≈ 4` at `Θ/20`, so T³ fails, while
freeze-out still holds. Mutants stay `debye-solid` and are not installed.

## What is computed

- Internal energy `U` as `Qty<Energy>`: classical `3 N k T`; Einstein
  `3 N k Θ_E / (e^{Θ_E/T} − 1)`; Debye
  `9 N k T (T/Θ_D)³ ∫_0^{x_D} x³/(e^x − 1) dx`.
- Heat capacity as `Qty<HeatCapacity>` (J/K, distinct from energy): Einstein
  `C_V = 3 N k · x² e^x / (e^x − 1)²`; Debye
  `C_V = 9 N k (T/Θ_D)³ ∫_0^{x_D} x^4 e^x / (e^x − 1)² dx`. Both checked
  against a finite-difference `dU/dT`.
- The Debye integrals to `x = 40` recover `π⁴/15` and `4π⁴/15`.
- Third-law probe: `C_V(Θ/40) / (3 N k)` vanishes or does not.
- `T³` probe (independent of the current T): `C_V(2T)/C_V(T)` at `T = Θ/20`
  must sit in `[7, 9]`, and `C_V(Θ/20)` must match `(4π⁴/5)(T/Θ)³` to 8%.

## Knob → verdict

```
physis experiment solid
physis experiment thermo          # third-law row: gas and Dulong–Petit fail; Einstein and Debye hold
physis set einstein-solid temperature 4000   # Dulong–Petit recovered as correspondence
physis set einstein-solid spectrum debye     # T³ fails → holds
physis hypothesize debye-solid               # add-2d is IR, not set
physis encode debye-solid                    # 3D ω²; not P3S, not a kernel proof
physis set einstein-solid quantum false      # 1819 theory restored
```

## Honesty

- Dulong–Petit as encoded here is the *classical oscillator* law, not a fit
  to 1819 room-temperature data (those metals happened to be in the high-T
  regime). The lab challenges the law as a T-independent claim.
- Einstein and Debye share one characteristic temperature knob. For a real
  crystal `Θ_E` and `Θ_D` differ; here they are the scale of each model, not
  a material fit.
- Debye `T³` is the acoustic continuum with a sharp cutoff `ω_D`. Optical
  branches, real density-of-states structure, and anharmonicity are not
  encoded. The sampled doubling is the theorem of *this* encoding. A 2D
  `ω` continuum is a different encoding (`add-2d`), not a silent Einstein
  spectrum.
- Sharing `thermo.third-law` with the ideal gas is deliberate: one row, four
  theories, two honest classical failures, two quantum holds that then split
  on `thermo.debye-t3`.

## Related

- `specs/011-thermodynamics.md` (ideal gas; same third-law row)
- `specs/016-blackbody.md` (the other Bose-occupation challenge to equipartition)
- `specs/001-type-system.md` (`HeatCapacity` vs `Energy`)
- `specs/009-computation.md` (Landauer: information → typed energy)
