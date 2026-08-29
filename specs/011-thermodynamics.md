# 011 — Thermodynamics

Status: active
Layer: statistical
Id: `thermo`

## Purpose

Thermodynamics is the fourth domain reuse (after electromagnetism and
computation), and it populates the `statistical` layer that earlier milestones
left empty (`specs/002`). It also exercises the type system where it matters
most for thermo: `Qty<Temperature>` (kelvin) and `Qty<Energy>` (joules) are
distinct types, so `k_B · T` is an energy by construction and cannot be confused
with a temperature.

## Object

| id | object |
|---|---|
| `ideal-gas` | a monatomic classical ideal gas |
| `dulong-petit` | classical lattice oscillators (see `specs/017`) |
| `einstein-solid` | Einstein (1907) Bose oscillators (see `specs/017`) |

## Knobs

| knob | effect |
|---|---|
| `temperature` | temperature in kelvin |
| `volume_ratio` | V_f/V_i for an isothermal expansion; > 1 is a spontaneous free expansion |
| `particles` | number of gas particles N |

## Claims

| id | meaning | how it is decided |
|---|---|---|
| `thermo.equipartition` | C_v = (3/2) N k | computed: `C_v = dU/dT` by finite difference, `U = (3/2) N k T` |
| `thermo.second-law` | a free expansion does not decrease entropy | computed: `ΔS = N k ln(V_f/V_i) ≥ 0`; fails for `volume_ratio < 1` |
| `thermo.third-law` | S → 0 as T → 0 | **fails**: classical `S ∝ (3/2) ln T → −∞`; the third law needs quantum statistics |

## Honest failure

The third-law verdict is `fails` for the classical ideal gas, and that is
correct physics: the Sackur–Tetrode entropy has an unbounded-below `ln T` term,
so a purely classical gas cannot satisfy the third law. Recording this as a
genuine failure (rather than quietly omitting it) is the epistemic honesty the
project is built on — the classical model is wrong here, and the lab says so.

## Knob → verdict

```
physis experiment thermo
physis set ideal-gas volume_ratio 0.5   # a compression: second-law flips to fails
```

## Non-goals (this milestone)

- Quantum statistics (Bose/Fermi), which would fix the third law.
- A real equation-of-state solver or Monte-Carlo; the laws here are the encoded
  classical results with the computed pieces (C_v, ΔS) checked.

## Related

- `specs/002-ontology-layers.md` (the `statistical` layer)
- `specs/007-reuse-domains.md` (how domains are added)
- `specs/016-blackbody.md` (Rayleigh–Jeans vs Planck on the same layer)
- `specs/017-einstein-solid.md` (Dulong–Petit vs Einstein; shares `thermo.third-law`)
