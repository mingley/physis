# Roadmap

Physis is a laboratory that grows by making more of its claims *checkable* and more of its domains *real*. Calendar estimates are not used here. Milestones are mechanical.

```
M0 foundation     ✓ typed layers, knobs, claims, string-critique lab, CLI
M1 string lab     ✓ journal replay, all constructions, anomaly claim, moduli
M2 empirical      ✓ SM embedding verified by code, typed lengths, fixture + score
M3 domain reuse   ✓ electricity (Maxwell + ohm-circuit) and computation
M4 continuum      ◑ fields as actual local objects (klein-gordon lattice seed)

Level-3 trust
  L3-M1 trust model        ✓ MachineProved unforgeable; FormalClaim commitments; assumptions; why
  L3-M2 exact dual-check   ✓ catalog identities + dual expanders
  L3-M3 provenance         ✓ SourceRecord rejects slogan locators
  L3-M4 numerics           ✓ Ratio / Interval; empirical receipts
  L3-M5 artifact DAG       ✓ content-addressed store + descendant invalidation
  L3-M6 protocol v2        ✓ prove, falsify, sweep, branch, compare, sensitivity
  L3-M7 physis-ir          ✓ line-oriented theory packages
  L3-M8 red-team corpus    ✓ physis audit
  L3-M9 experiment rank    ✓ physis design
  L3-M10 research loop     ✓ physis loop (orchestrator)
  L3 Lean/nanoda kernels   ✓ Physlib d²=0, interval, Einstein composition, and mass shell; lake + nanoda on export
  L3 inspect / origin      ✓ ParameterOrigin; physis inspect
  L3 roles / budget        ✓ Role gates exec; formalize is untrusted; budget caps prove/review/set
  L3 reproduce             ✓ in-process remint of a stored receipt; explicitly not P4
  L3 gap graph             ✓ physis gaps rebuilds a content-addressed snapshot; MissingTheorem is Holds-only; lemma edges are live
  L3 trust gate            ✓ reproduce and loop-review require P3F; standalone review stays encoding-axis
  L3 challenge seal        ✓ Challenge is generate-only; no Deserialize; solver cannot set the obligation
  L3 journal identity      ✓ prove/review restore remints only the recorded FormalClaim; slug is not enough
  L3 catalog domain        ✓ catalog identities name a DomainOfValidity; encoding-wide physlib is not the catalog claim
  L3 GUT/SM domains        ✓ mixing-angle and SM P3N cells name DomainOfValidity; Super-K and Tr Q stay encoding-wide
  L3 catalog assumptions   ✓ catalog lab_claim includes IdentitySpec axioms; encoding-is-the-model alone is not the catalog identity
  L3 catalog identity      ✓ ExactIdentity and review bind to FormalClaim, not slug; live claims are IdentitySpec::lab_claim
  L3 P3S semantic          ✓ encoding review bound to statement_hash, not slug; Canonical is not a variant; P3S is not an evaluator field
  L3 P3N numeric           ✓ SM anomalies, hypercharge solve, hydrogen Q=T3+Y, GUT-scale 3/8; Tr Q is ΣY already certified, not a second P3N; not GQW at M_Z
  L3 P2 cross-check        ✓ Hodge Laplacian nullity vs b1; Euler–Poincaré is rank-cancellation, not P2
  L3 Hodge domain          ✓ dec.hodge-harmonic names discrete Laplacian DomainOfValidity; Euler and Poincaré stay encoding-wide
  L3 long-wavelength domain ✓ dispersion and ohm-circuit quasi-static name DomainOfValidity; Maxwell copy stays encoding-wide
  L3 judgment seal         ✓ Judgment has no Deserialize; from_lab projects Proved from a receipt; LogicalJudgment has no public Proved constructor; NumericJudgment has no public Certified constructor; EmpiricalJudgment has no public Compatible constructor; HeuristicJudgment has no public Suggestive constructor; StatisticalJudgment has no public Computed constructor; Verdict has no Deserialize; Verdict overlay fields are private
  L3 formal-claim seal     ✓ FormalClaim is from_claim-only; no Deserialize; a forged Claim hash is not copied through
  L3 claim-hash derived    ✓ Claim::statement_hash is a getter; no stored field; id/statement/class/layer/assumptions/domain/commitments are private; mutating the sentence cannot keep a stale receipt; derivation/empirical/semantic fields are private
  L3 causal diffs          ✓ set/sweep/compare/replay carry derivation, empirical, and judgment axes; legacy kind-only journals still replay
  L3 precision gap         ✓ coarse field.second-order-accurate is InsufficientPrecision, not Fails; not P3N
  L3 hypothesize           ✓ constrained structural mutation of chosen/fitted knobs; measured knobs frozen; explorer can observe; does not mint
  L3 evidence graph        ✓ physis evidence groups by statement hash; competing encodings vs evaluations; derived TrustProfile, not a numeric score
  L3 Super-K dataset       ✓ gut.proton-lifetime-sk compares dim-6 M_GUT^4 scaling to Takenaka et al. PRD 102 112011; not P3N; not dim-5; Tr Q stays encoding-wide
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
