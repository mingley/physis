# 007 — Domain reuse

Status: active
Layer: all

`physis` is a physics lab first. It is designed so that **electricity, computation, chemistry, …** do not fork the repo. They add theories (and sometimes layers) on the same machine.

## How to add a domain

1. Write a spec: objects, knobs, claims, honesty tags, controls.
2. Map the domain onto existing layers if possible.
   - Electricity: `field` + `interaction` + `particle` (electrons, photons) + `effective`
   - Computation: `information` + `mathematical` (state machines, complexity classes as claims)
3. Add a `LayerId` only if the tower is genuinely missing a stratum. This is rare.
4. Implement `Theory`. Put domain-specific model types in `physis-model` or a new crate `physis-<domain>` that depends on core+model.
5. Register an experiment. Do not teach the CLI a special case beyond `experiment <id>`.
6. Tests: at least one knob → verdict diff.

## Shared goods every domain gets for free

- Dimensional quantities (electricity needs this immediately)
- Knobs, claims, verdicts, epistemic tags
- Agent protocol and journal
- The rule that illegal states are type errors or domain errors

## Electricity (planned)

See `plans/004-m3-domain-reuse.md`. Sketch:

- Knobs: permittivity, permeability, or more honestly: unit system + Maxwell constitutive knobs in a linear medium
- Claims: Gauss, Faraday, speed of EM waves `1/√(εμ)` matching `c` in vacuum (this should become a theorem in the encoding)
- Control: circuit theory as an *effective* layer of Maxwell

## Computation (planned)

- States as typed configurations
- Claims: invariants, complexity upper/lower bounds tagged honestly
- A reversible-computing / Landauer experiment sitting on `information` + `statistical` once those layers exist

## What reuse is not

Copy-pasting `StringTheory` and renaming it `Maxwell`. If two domains share structure, that structure belongs in `physis-core` / `physis-model`.
