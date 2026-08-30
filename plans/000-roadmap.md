# Roadmap

Physis is a laboratory that grows by making more of its claims *checkable* and more of its domains *real*. Calendar estimates are not used here. Milestones are mechanical.

```
M0 foundation     ✓ typed layers, knobs, claims, string-critique lab, CLI
M1 string lab     ✓ journal replay, all constructions, anomaly claim, moduli
M2 empirical      ✓ SM embedding verified by code, typed lengths, fixture + score
M3 domain reuse   ✓ electricity (Maxwell + ohm-circuit) and computation
M4 continuum      ◑ fields as actual local objects (klein-gordon lattice seed)

Level-3 trust
  L3-M1 trust model        ✓ MachineProved unforgeable; FormalClaim; assumptions; why
  L3-M2 exact dual-check   ✓ catalog identities + dual expanders
  L3-M3 provenance         ✓ SourceRecord rejects slogan locators
  L3-M4 numerics           ✓ Ratio / Interval; empirical receipts
  L3-M5 artifact DAG       ✓ content-addressed store + descendant invalidation
  L3-M6 protocol v2        ✓ prove, falsify, sweep, branch, compare, sensitivity
  L3-M7 physis-ir          ✓ line-oriented theory packages
  L3-M8 red-team corpus    ✓ physis audit
  L3-M9 experiment rank    ✓ physis design
  L3-M10 research loop     ✓ physis loop (orchestrator)
  L3 Lean/nanoda kernels   ✓ Physlib d²=0 and interval; lake + nanoda on export
  L3 inspect / origin      ✓ ParameterOrigin; physis inspect
  L3 roles / budget        ✓ Role gates exec; formalize is untrusted; budget caps prove/review/set
  L3 reproduce             ✓ in-process remint of a stored receipt; explicitly not P4
  L3 gap graph             ✓ physis gaps rebuilds a content-addressed snapshot
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
