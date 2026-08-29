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

## Knobs

| theory | knob | effect |
|---|---|---|
| `linear-medium` | `epsilon_r` | relative permittivity ε_r ≥ 1; raises the refractive index n = √(ε_r μ_r) |
| `linear-medium` | `mu_r` | relative permeability μ_r ≥ 1; raises n |

`maxwell-vacuum` has no knobs (vacuum is the unit medium).

## Claims

| id | meaning | epistemic |
|---|---|---|
| `em.wave-speed-c` | EM waves travel at c | theorem (vacuum); fails in a medium |
| `em.gauss` | Gauss's law | encoded-fact |
| `em.faraday` | Faraday's law | encoded-fact |
| `em.ampere` | Ampère–Maxwell law | encoded-fact |
| `em.charge-conservation` | ∂ρ/∂t + ∇·J = 0 | theorem |
| `em.lorentz-invariance` | boost invariance of the field equations | theorem (vacuum); fails in a medium |

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
```

The knob turn `epsilon_r: 2.25 → 1` flips both `em.wave-speed-c` and
`em.lorentz-invariance` from `fails` to `holds`.

## Non-goals (this milestone)

- A PDE field solver or a numerical FDTD engine.
- Typed exterior calculus / differential forms (a later milestone may encode
  Faraday/Ampère as `dF = 0`, `d⋆F = ⋆J`).
- Circuit theory (`ohm-circuit`) as an effective layer — planned, not yet built.

## Related

- `specs/007-reuse-domains.md` — how domains are added
- `plans/004-m3-domain-reuse.md` — the milestone
