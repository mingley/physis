# 017 — Einstein solid (Dulong–Petit vs quantum oscillators)

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

This is the lattice sibling of `specs/016-blackbody.md`: another 19th-century
equipartition claim, another Bose occupation, another knob → verdict.

## Objects

| id | object | default `quantum` |
|---|---|---|
| `dulong-petit` | classical 3N oscillators (standing theory) | false |
| `einstein-solid` | Einstein (1907) Bose oscillators | true |

## Knobs

| knob | layer | effect |
|---|---|---|
| `quantum` | quantum | true: Einstein; false: every oscillator has energy `kT` |
| `temperature` | statistical | lattice temperature (K) |
| `einstein_temp` | statistical | `Θ_E = ħω/k` (K). Classical physics ignores it |
| `oscillators` | statistical | number of atoms N (3N oscillators) |

Default: `T = 60 K`, `Θ_E = 300 K` so `T/Θ_E = 0.2` — deep in the quantum
regime, where Dulong–Petit is most clearly false.

## Claims

| id | meaning | Dulong–Petit | Einstein (default T) |
|---|---|---|---|
| `thermo.dulong-petit` | `C_V = 3 N k` at the current T | **holds** | **fails** (`C_V/(3Nk) ≈ 0.17`) |
| `thermo.high-t-classical` | `T ≫ Θ_E` recovers `3 N k` | **holds** (always) | **fails** (not yet high-T) |
| `thermo.third-law` | `C_V → 0` as T → 0 | **fails** (`C_V` stays `3 N k` at `Θ_E/40`) | **holds** (exponential freeze-out) |

Raising `einstein-solid` `temperature` to 4000 K (`T/Θ_E ≈ 13`) flips
`thermo.dulong-petit` **fails → holds** and `thermo.high-t-classical`
**fails → holds**. The third law still holds: it is a statement about
T → 0, probed at `Θ_E/40`, not about the current T.

## What is computed

- Internal energy `U` as `Qty<Energy>`: classical `3 N k T`; Einstein
  `3 N k Θ_E / (e^{Θ_E/T} − 1)`.
- Heat capacity `C_V = 3 N k · x² e^x / (e^x − 1)²` with `x = Θ_E/T`,
  checked against a finite-difference `dU/dT`.
- Third-law probe: `C_V(Θ_E/40) / (3 N k)` vanishes or does not.

## Knob → verdict

```
physis experiment solid
physis experiment thermo          # third-law row: gas and Dulong–Petit fail; Einstein holds
physis set einstein-solid temperature 4000   # Dulong–Petit recovered as correspondence
physis set einstein-solid quantum false      # 1819 theory restored
```

## Honesty

- Dulong–Petit as encoded here is the *classical oscillator* law, not a fit
  to 1819 room-temperature data (those metals happened to be in the high-T
  regime). The lab challenges the law as a T-independent claim.
- Einstein's model over-suppresses `C_V` (exponential vs the Debye `T³` law).
  That is a known limitation, not encoded as a claim. Debye is a later
  increment.
- Sharing `thermo.third-law` with the ideal gas is deliberate: one row, three
  theories, two honest classical failures.

## Related

- `specs/011-thermodynamics.md` (ideal gas; same third-law row)
- `specs/016-blackbody.md` (the other Bose-occupation challenge to equipartition)
- `specs/009-computation.md` (Landauer: information → typed energy)
