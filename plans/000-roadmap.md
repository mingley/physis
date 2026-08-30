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
  L3-M7 physis-ir          ✓ line-oriented theory packages plus constrained mutations; combinational NAND netlist is a live package
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
  L3 correspondence domain ✓ high-T T/Θ ≥ 8, Debye T³ at Θ/20, RJ infrared hν = 0.01 kT, exact 2D area law; Dulong–Petit and Poincaré stay encoding-wide
  L3 judgment seal         ✓ Judgment has no Deserialize; from_lab projects Proved from a receipt; LogicalJudgment has no public Proved constructor; NumericJudgment has no public Certified constructor; EmpiricalJudgment has no public Compatible constructor; HeuristicJudgment has no public Suggestive constructor; StatisticalJudgment has no public Computed constructor; Verdict has no Deserialize; Verdict overlay fields are private
  L3 formal-claim seal     ✓ FormalClaim is from_claim-only; no Deserialize; a forged Claim hash is not copied through
  L3 claim-hash derived    ✓ Claim::statement_hash is a getter; no stored field; id/statement/class/layer/assumptions/domain/commitments are private; mutating the sentence cannot keep a stale receipt; derivation/empirical/semantic fields are private
  L3 causal diffs          ✓ set/sweep/compare/replay carry derivation, empirical, and judgment axes; legacy kind-only journals still replay
  L3 precision gap         ✓ coarse field.second-order-accurate is InsufficientPrecision, not Fails; not P3N
  L3 hypothesize           ✓ chosen/fitted knob probes and IR package forks; measured knobs frozen; mutants not installed; explorer can observe; does not mint; combinational add-feedback and klein-gordon add-next-nearest are package mutations, not knobs
  L3 evidence graph        ✓ physis evidence groups by statement hash and inserts a content-addressed Evidence DAG (Statement + Evaluation parents); competing encodings vs evaluations; derived TrustProfile, not a numeric score; not Canonical; not P4
  L3 journal evidence      ✓ Evidence events restore by rebuild from live evaluations; graph_hash is not deserialized; restore does not journal again; not Canonical; not P4
  L3 Super-K dataset       ✓ gut.proton-lifetime-sk compares dim-6 M_GUT^4 scaling to Takenaka et al. PRD 102 112011; not P3N; not dim-5; Tr Q stays encoding-wide
  L3 unique-vacuum domain  ✓ four FormalClaims of predictivity.unique-vacuum name landscape / program axiom / Einstein-Hilbert / Higgs vacuum; still Asserted; not Canonical
  L3 statistical NLL       ✓ PDG sin²θ_W(M_Z) is a Gaussian; from_lab projects statistical computed from an exact Ratio NLL; Super-K stays empirical interval-subset; not P3N
  L3 proposer split        ✓ proof-searcher cannot remint; replication-agent reproduces (not P4); explorer cannot score; empirical-analyst scores
  L3 inspect judgment      ✓ physis inspect judgment inverts projected labels; statistical-computed is the PDG GQW cell; empirical-excluded is Super-K; logical-proved requires a receipt
  L3 numerical enclose     ✓ physis enclose independently parses CertifiedNumeric Ratio strings into a NumericCertificate DAG; numerical-verifier unique op; restore rebuilds; not P3F, not Canonical, not P4; P3N count stays 4
  L3 provenance cite       ✓ physis cite independently rebuilds SourceRecord for PDG/Super-K datasets and catalog dossiers; provenance-auditor unique op; restore rebuilds; not P3S, not Canonical, not P4
  L3 encoding round-trip   ✓ physis encode independently parses, round-trips, and reconstructs live IR packages (combinational NAND, Klein-Gordon stencil); encoding-auditor unique op; restore rebuilds; not P3S, not Canonical, not P4
  L3 judgment projection   ✓ physis judge independently rebuilds Judgment::from_lab into a JudgmentProjection DAG; judge unique op; unique-vacuum stays heuristic failed; JSON cannot mint logical proved; restore rebuilds; not Canonical, not P4
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
