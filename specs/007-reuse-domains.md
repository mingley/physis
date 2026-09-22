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
- Knobs, claims, verdicts, assurance axes (`class`, `derivation`, `empirical`, `semantic`)
- Agent protocol and journal
- The rule that illegal states are type errors or domain errors

## Electricity (shipped)

See `specs/008-electromagnetism.md` and `plans/004-m3-domain-reuse.md`
(`em-vacuum` experiment: `maxwell-vacuum`, `linear-medium`, `ohm-circuit`):

- Knobs: `epsilon_r`, `mu_r` on `linear-medium`; `frequency_hz` on `ohm-circuit`
- Claims: Gauss, Faraday, Ampère–Maxwell, speed of EM waves `1/√(εμ)` matching `c` in vacuum (`em.wave-speed-c` holds as a model-internal evaluated claim, not a stored fact)
- Control: circuit theory as an *effective* layer of Maxwell (`ohm-circuit`)

## Computation (shipped)

See `specs/009-computation.md` (`computation` experiment:
`combinational-circuit`, `turing-machine`, plus the `landauer-engine` bridge):

- States as typed configurations
- Claims: invariants, complexity upper/lower bounds tagged honestly (`comp.p-equals-np` is `undecidable`, class `open-problem`)
- A reversible-computing / Landauer check on `information` + `statistical` (`run landauer-engine`)

## What reuse is not

Copy-pasting `StringTheory` and renaming it `Maxwell`. If two domains share structure, that structure belongs in `physis-core` / `physis-model`.
