# 002 — Ontology layers

Status: active
Layer: all

## The tower

Finest to coarsest:

| Layer | What lives here | v0 encoding |
|---|---|---|
| mathematical | groups, algebras, type-level proofs | `typenum` SI, `SimpleGroup` |
| spacetime | dimension, signature, extras, topology hint | `Manifold` |
| quantum | amplitudes, operators | finite `Ket`, Pauli, CHSH Bell test (`bell` lab, `specs/012`) |
| field | local DoF, Lagrangians | string kind, SUSY flag (placeholder) |
| particle | spectrum | SM catalog + graviton |
| interaction | gauge groups, couplings | `GaugeGroup`, SM embed table |
| effective | cutoffs, compactification radii | hidden-extra-dims claim |
| statistical | temperature, ensembles | ideal gas (`thermo` lab, `specs/011`); cavity radiation (`blackbody` lab, `specs/016`) |
| information | records, computation | journal is the seed |
| agent | observers who turn knobs | `Lab`, protocol |

Layers are scales of mechanism, not a morality of “more true.” A claim always names a layer.

## Smallest level of modern physics

**Empirically confirmed** (as of this encoding):

- Standard Model quantum fields on 3+1 Lorentzian spacetime
- Electroweak scale accessed; QCD; atomic; GR as classical gravity
- Higgs observed; neutrinos oscillate (masses exist; the minimal SM encoding currently stores neutrino mass as 0 — a known lie, labelled by the UV-completion fail of SM)

**Not empirically confirmed**, and therefore *theories* not substrate:

- strings, extra dimensions, supersymmetry, gravitons as particles
- loop quantum gravity, causal sets, asymptotic safety
- unique-geometry / Geometric Unity-style programs

v0 bottoms the *substrate* at typed SI + finite Hilbert space + a particle catalog + gauge groups. Planck-scale objects enter only as `Theory` implementations.

## Coupling

Changing a spacetime knob (e.g. `total_dim`) is allowed to change:

- consistency claims on spacetime
- particle content (bosonic vs superstring)
- predictivity (landscape heuristic scales with extra dims)
- empirical hidden-extra-dims

That coupling is the product. If a knob is dead — no claim can see it — it is a bug or a future hook and must be documented as such.

## Future layers (not v0)

- A real field layer: typed Lagrangians, gauge covariant derivatives
- Statistical mechanics / thermodynamics for electricity and materials (ideal gas and cavity radiation now live here; more equations of state later)
- Information / computation as a domain reuse (see `specs/007-reuse-domains.md`)
- Continuum QFT (almost certainly never “full”; finite modes first)
