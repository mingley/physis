# Roadmap

Physis is a laboratory that grows by making more of its claims *checkable* and more of its domains *real*. Calendar estimates are not used here. Milestones are mechanical.

```
M0 foundation     ✓ typed layers, knobs, claims, string-critique lab, CLI
M1 string lab     ✓ journal replay, all constructions, anomaly claim, moduli
M2 empirical      ✓ SM embedding verified by code, typed lengths, fixture + score
M3 domain reuse   ✓ electricity (Maxwell + ohm-circuit) and computation
M4 continuum      ◑ fields as actual local objects (klein-gordon lattice seed)

Level-3 trust
  L3-M1 trust model   ✓ MachineProved unforgeable; FormalClaim; assumptions; why
  L3-M2 Lean proofs     planned (specs/020)
  L3-M3 provenance      planned
  ...
```

Each milestone must:

- keep `unsafe`-free pure Rust (proof *artifacts* from isolated checkers are L3-M2)
- keep assurance axes honest (`executed` is not a kernel proof)
- add tests that are knob → verdict diffs, not snapshot goldens of prose
- update specs if contracts change

See:

- `specs/020-proof-carrying.md`

- `plans/001-m0-foundation.md`
- `plans/002-m1-string-lab.md`
- `plans/003-m2-empirical-contact.md`
- `plans/004-m3-domain-reuse.md`
- `plans/005-m4-continuum.md`
