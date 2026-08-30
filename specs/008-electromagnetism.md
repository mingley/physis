# 008 — Electromagnetism

Status: active
Layer: field / interaction
Id: `em-vacuum`

## Purpose

Electromagnetism is the first **domain reuse** (see `specs/007-reuse-domains.md`).
It proves the workspace is not a string-theory toy: the same typed substrate —
`Qty` quantities, layers, knobs, claims, verdicts, the `Theory` trait, and the
experiment/matrix machinery — hosts classical electromagnetism without forking
`physis-core` or `physis-model`.

It also earns a genuine **theorem**: in vacuum the wave speed is `1/√(ε₀μ₀)`,
and that equals `c`. This is not stored as a fact; it is checked from the typed
constants `ε₀`, `μ₀`, `c` in `physis-model::constants`.

## Objects

| id | object |
|---|---|
| `maxwell-vacuum` | Classical EM in vacuum: a U(1) gauge field of light |
| `linear-medium` | Classical EM in a linear medium (`ε_r`, `μ_r` knobs) |
| `ohm-circuit` | Lumped-element circuit theory: the quasi-static effective limit of Maxwell (`frequency_hz` knob; lumped branch IR) |

## Knobs

| theory | knob | effect |
|---|---|---|
| `linear-medium` | `epsilon_r` | relative permittivity ε_r ≥ 1; raises the refractive index n = √(ε_r μ_r) |
| `linear-medium` | `mu_r` | relative permeability μ_r ≥ 1; raises n |
| `ohm-circuit` | `frequency_hz` | operating frequency; the lumped model holds while the wavelength c/f dwarfs the circuit. Topology is not this knob: `add-tline` is an IR mutation |

`maxwell-vacuum` has no knobs (vacuum is the unit medium).

## Claims

| id | meaning | epistemic |
|---|---|---|
| `em.wave-speed-c` | EM waves travel at c | theorem (vacuum); fails in a medium |
| `em.gauss` | Gauss's law | theorem (vacuum: `∇·E = 0` verified on a Coulomb field); encoded-fact in a medium |
| `em.faraday` | Faraday's law | theorem (vacuum: verified numerically on a plane wave); encoded-fact in a medium |
| `em.ampere` | Ampère–Maxwell law | theorem (vacuum: verified numerically on a plane wave); encoded-fact in a medium |
| `em.charge-conservation` | ∂ρ/∂t + ∇·J = 0 | theorem in Maxwell (backed by a numerically-verified `∇·(∇×A) = 0`); on `ohm-circuit`, Kirchhoff current law of the lumped branch netlist. Domain: lumped Kirchhoff nodes. `add-tline` appends `tline 0 1` and this cell fails. That is not a knob. Maxwell's continuity copy stays encoding-wide |
| `em.lorentz-invariance` | boost invariance of the field equations | theorem (vacuum); fails in a medium or circuit |
| `em.quasi-static-valid` | the lumped-element approximation is valid | encoded-fact (ohm-circuit names `λ > 100 ×` circuit size); inapplicable to full Maxwell (encoding-wide) |

## The control: `ohm-circuit`

Lumped circuit theory is the quasi-static, long-wavelength limit of Maxwell.
Kirchhoff's current law *is* charge conservation on a lumped node graph;
Kirchhoff's voltage law *is* Faraday's law. Wave propagation is dropped
(`em.wave-speed-c` inapplicable) and the theory has a preferred rest frame
(`em.lorentz-invariance` fails). The lumped branch lives on the IR package
(`branch R 0 1`). A transmission-line delay is a package mutation
(`add-tline`), not a knob: `em.charge-conservation` fails on the mutant.
It is valid only while the wavelength `c/f` dwarfs the circuit: raising
`frequency_hz` past that point flips `em.quasi-static-valid` from `holds`
to `fails`, using typed `Qty<Length>` wavelengths. That frequency knob is
orthogonal to the tline encoding. The ohm-circuit quasi-static cell names
`λ > 100 ×` circuit-size regime; Maxwell's copy of the slug stays
encoding-wide and inapplicable.

## Homogeneous Maxwell equations, verified

In vacuum, all three of `em.gauss`, `em.faraday`, and `em.ampère` are **computed
theorems**. Gauss: a Coulomb field `E = r̂/r²` is checked to have `∇·E = 0` away
from the source (residual ≲ 1e-4). Faraday/Ampère: a plane wave
`E = ŷ cos(x−t)`, `B = ẑ cos(x−t)` (natural units) is checked by central finite
differences to satisfy `∂B/∂t + ∇×E = 0` and `∂E/∂t − ∇×B = 0` to residual
≲ 1e-6. In a medium these revert to encoded facts (macroscopic form).

## The theorem

`physis_model::constants` encodes `ε₀` and `μ₀` as typed quantities with the
correct SI dimensions. Then

```
ε₀ · μ₀ · c²   :  Qty<Dimensionless>
```

type-checks (the units cancel by construction) and evaluates to `1` to CODATA
precision. That *is* `1/√(ε₀μ₀) = c`. The `em.wave-speed-c` claim reports it as
a theorem in vacuum.

## Knob → verdict

A linear medium with `n = √(ε_r μ_r) > 1` slows light below `c` and selects a
rest frame:

```
physis experiment em-vacuum
physis set linear-medium epsilon_r 1   # n → 1, wave-speed-c and lorentz-invariance flip to holds
physis set ohm-circuit frequency_hz 1e10   # electrically short → lumped model fails
physis hypothesize ohm-circuit             # add-tline is IR, not set
```

The knob turn `epsilon_r: 2.25 → 1` flips both `em.wave-speed-c` and
`em.lorentz-invariance` from `fails` to `holds`.

## Non-goals (this milestone)

- A PDE field solver or a numerical FDTD engine.
- Typed exterior calculus / differential forms (a later milestone may encode
  Faraday/Ampère as `dF = 0`, `d⋆F = ⋆J`).
- Circuit theory (`ohm-circuit`) as a full SPICE engine or transmission-line
  PDE. The IR fork is a delay equation, not a simulator.

## Related

- `specs/007-reuse-domains.md` — how domains are added
- `plans/004-m3-domain-reuse.md` — the milestone
