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
| `ideal-gas` | a monatomic ideal gas (Maxwell–Boltzmann statistics IR) |
| `dulong-petit` | classical lattice oscillators (see `specs/017`) |
| `einstein-solid` | Einstein (1907) Bose oscillators (see `specs/017`) |
| `debye-solid` | Debye (1912) phonon continuum (see `specs/017`) |
| `excursion-counts` | stationary hidden-jump counter on two triangles sharing one visible state |
| `excursion-ring` | hidden-jump counter on a four-site ring with clockwise, anticlockwise and opposite jumps |
| `excursion-square` | unit-rate cycle with one visible site, a missing one-jump count and uniform occupations |

## Knobs

| knob | effect |
|---|---|
| `temperature` | temperature in kelvin. Statistics are not this knob: `add-bose` and `add-fermi` are IR mutations |
| `volume_ratio` | V_f/V_i for an isothermal expansion; > 1 is a spontaneous free expansion |
| `particles` | number of gas particles N |

## Claims

| id | meaning | how it is decided |
|---|---|---|
| `thermo.equipartition` | C_v = (3/2) N k | computed: `C_v = dU/dT` by finite difference. Domain: classical C_V = 3/2 Nk. Live and Bose: `U = (3/2) N k T`. `add-fermi` appends `gas fermi` and Sommerfeld `C_V = (π²/2) N k (T/T_F)` fails this cell. That is not a knob |
| `thermo.second-law` | a free expansion does not decrease entropy | computed: `ΔS = N k ln(V_f/V_i) ≥ 0`; fails for `volume_ratio < 1` |
| `thermo.third-law` | S → 0 as T → 0 | theorem on `ideal-gas` (named domain: classical Sackur–Tetrode). **fails** on the live Maxwell–Boltzmann encoding (`S ∝ ln T`). `add-bose` appends `gas bose` and the low-T Bose entropy `S/Nk ∝ (T/T_c)^{3/2}` vanishes, so this cell holds. `add-fermi` likewise holds (S/Nk = (π²/2)(T/T_F)). That is not a knob. Einstein-solid third law stays encoding-wide (oscillator freeze-out) |

## Knob → verdict

```
physis experiment thermo
physis set ideal-gas volume_ratio 0.5   # a compression: second-law flips to fails
physis hypothesize ideal-gas            # add-bose and add-fermi are IR, not set
```

The knob turn `volume_ratio: 2 → 0.5` flips `thermo.second-law` from `holds` to
`fails`. That is orthogonal to Bose statistics: `thermo.third-law` still
fails after the knob turn, and holds on the `add-bose` and `add-fermi` IR forks.
`add-fermi` also flips `thermo.equipartition` holds to fails (Sommerfeld
`C_V`, not a temperature knob).

## Honest failure

The third-law verdict is `fails` for the classical ideal gas, and that is
correct physics: the Sackur–Tetrode entropy has an unbounded-below `ln T` term,
so a purely classical gas cannot satisfy the third law. Recording this as a
genuine failure (rather than quietly omitting it) is the epistemic honesty the
project is built on — the classical model is wrong here, and the lab says so.
Bose statistics are a package mutation, not a temperature knob, and not a
silent Einstein-solid install. Degenerate Fermi statistics are a second
package mutation: Sommerfeld `C_V` is not `(3/2) N k`.

## Non-goals (this milestone)

- A real equation-of-state solver or Monte-Carlo; the laws here are the encoded
  results with the computed pieces (C_v, ΔS, low-T Bose entropy) checked.
- Converting Einstein-solid `quantum` / `spectrum` into this IR.

## Hidden-jump counter

On `excursion-counts` all holding rates are one per model time unit. Branch
one has mean one hidden jump and mean winding one half turn per excursion.
The chosen `first_percent` sets its entry fraction from 1 to 99 percent
(default 1). `second_mean_jumps` ranges from 1 to 9999 (default 9999).
`second_winding_quarters` ranges from −2 to 2 in quarter turns (default 0).

`excursion.positive-floor` tests whether the measured immediate-return
fraction and mean force positive entropy. `excursion.floor-attained` tests
whether the current network attains it. `excursion.triangle-mean` tests
whether its first two count tails give its mean under the triangle
recurrence. The evaluator uses rational stationary fluxes for all three
model-internal decisions. [Definitions, equality conditions and proof](../docs/research/excursion-activity/RESULT.md)

On `excursion-ring`, `clockwise`, `anticlockwise` and `opposite` are chosen
integer edge weights from 1 through 8, all initially 1. Normalize the three
weights by their sum for unit exit rates. The exact rational verdict for
`excursion.only-joint-detects` tests whether each pair of the first tail,
second tail and mean admits equilibrium while their joint inequality forces
dissipation. [Joint inequality and reversible controls](../docs/research/excursion-activity/JOINT.md)

On the same ring, `excursion.beyond-joint-bound` requires that all pairwise
tests and the earlier joint inequality pass while the exact three-statistic
test forces a positive entropy floor. The full classifier also accepts
arbitrary canonical rational count inputs through the `excursion_equilibrium`
research example. [Exact classification and replay](../docs/research/excursion-equilibrium/CAMPAIGN.md)

On `excursion-square`, `entry_bias` and `continuation_bias` are chosen
integers from −11 to 11 in units of 1/24. The entry and continuation
probabilities start at one half. The two complementary rows keep the four
occupations at 1/4. The product of the two biases fixes the complete
observed jump clock. The two executed claims test whether that clock's two
tails and mean force positive entropy at arbitrary finite hidden order and
whether the square attains that minimum. [Construction and factory replay](../docs/research/excursion-entropy/CAMPAIGN.md)

## Related

- `specs/002-ontology-layers.md` (the `statistical` layer)
- `specs/007-reuse-domains.md` (how domains are added)
- `specs/016-blackbody.md` (Rayleigh–Jeans vs Planck on the same layer)
- `specs/017-einstein-solid.md` (Dulong–Petit vs Einstein vs Debye; shares `thermo.third-law` and `thermo.debye-t3`)
