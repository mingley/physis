# Roadmap

Physis is a laboratory that grows by making more of its claims *checkable* and more of its domains *real*. Calendar estimates are not used here. Milestones are mechanical.

```
M0 foundation     ✓ typed layers, knobs, claims, string-critique lab, CLI
M1 string lab     ✓ journal replay, all constructions, anomaly claim, moduli
M2 empirical      ✓ SM embedding verified by code, typed lengths, fixture + score
M3 domain reuse   ✓ electricity (Maxwell + ohm-circuit) and computation
  M4 continuum      ◑ fields as actual local objects (klein-gordon lattice seed; dirac-fermion naive 1D Dirac; wilson-u1 / wilson-su2 / wilson-su3 plaquettes are live IR packages)

Level-3 trust
  L3-M1 trust model        ✓ MachineProved unforgeable; FormalClaim commitments; assumptions; why
  L3-M2 exact dual-check   ✓ catalog identities + dual expanders
  L3-M3 provenance         ✓ SourceRecord rejects slogan locators
  L3-M4 numerics           ✓ Ratio / Interval; empirical receipts
  L3-M5 artifact DAG       ✓ content-addressed store + descendant invalidation
  L3-M6 protocol v2        ✓ prove, falsify, sweep, branch, compare, sensitivity
  L3-M7 physis-ir          ✓ line-oriented theory packages plus constrained mutations; combinational NAND, Klein-Gordon stencil, Wilson U(1)/SU(2)/SU(3) plaquettes, ohm-circuit lumped branches, bell-test singlet ket, newtonian-gravity inverse-square Binet rhs, linear-medium isotropic-linear constitutive law, maxwell-vacuum source-free homogeneous Faraday, ideal-gas Maxwell-Boltzmann statistics, landauer-engine kT ln2 bound, dirac-fermion naive 1D operator, general-relativity Einstein-Hilbert action, special-relativity Lorentz boost plus catalog interval, composition, and mass-shell trees, planck Bose occupation, de-rham discrete coboundary, turing-machine unrelativized TM, olbers-static inverse-square Euclidean shells, and su5-gut complete 5bar + 10, and debye-solid 3D ω² continuum, and standard-model complete one-generation Weyl, and observer-geometry Spin(10) on 10-fibre, and dulong-petit harmonic U = 3 N k T, and heterotic-e8e8 complete E8 x E8, and heterotic-so32 complete SO(32), and type-i Chan-Paton SO(32) are live packages
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
  L3 correspondence domain ✓ high-T T/Θ ≥ 8, Debye T³ at Θ/20, RJ infrared hν = 0.01 kT, exact 2D area law; dulong-petit names harmonic U = 3 N k T; Poincaré and Dulong–Petit on einstein-solid/debye-solid stay encoding-wide; heterotic-e8e8 names complete E8 x E8; heterotic-so32 names complete SO(32); type-i names Chan-Paton SO(32); Type II Green-Schwarz stays encoding-wide
  L3 judgment seal         ✓ Judgment has no Deserialize; from_lab projects Proved from a receipt; LogicalJudgment has no public Proved constructor; NumericJudgment has no public Certified constructor; EmpiricalJudgment has no public Compatible constructor; HeuristicJudgment has no public Suggestive constructor; StatisticalJudgment has no public Computed constructor; Verdict has no Deserialize; Verdict overlay fields are private
  L3 formal-claim seal     ✓ FormalClaim is from_claim-only; no Deserialize; a forged Claim hash is not copied through
  L3 claim-hash derived    ✓ Claim::statement_hash is a getter; no stored field; id/statement/class/layer/assumptions/domain/commitments are private; mutating the sentence cannot keep a stale receipt; derivation/empirical/semantic fields are private
  L3 causal diffs          ✓ set/sweep/compare/replay carry derivation, empirical, and judgment axes; legacy kind-only journals still replay
  L3 precision gap         ✓ coarse field.second-order-accurate is InsufficientPrecision, not Fails; not P3N
  L3 hypothesize           ✓ chosen/fitted knob probes and IR package forks; measured knobs frozen; mutants not installed; explorer can observe; does not mint; combinational add-feedback and add-contention, klein-gordon add-next-nearest and add-quartic, wilson add-rectangle and add-higgs (U(1)/SU(2)/SU(3)), ohm-circuit add-tline and add-flux, bell-test add-product and add-pr-box, newtonian-gravity add-schwarzschild and add-yukawa, linear-medium add-tellegen and add-chiral, maxwell-vacuum add-monopole and add-proca, ideal-gas add-bose and add-fermi, landauer-engine add-kt and add-demon, dirac-fermion add-wilson and add-next-nearest, general-relativity add-r-squared and add-brans-dicke, special-relativity add-binomial-gamma and add-minus-uv, planck add-wien and add-zero-point, de-rham add-sign-flip and add-down-laplacian, turing-machine add-oracle, olbers-static add-tired-light, su5-gut add-missing-10, debye-solid add-2d, and standard-model add-missing-eR, and observer-geometry add-missing-spin10, and dulong-petit add-quartic, and heterotic-e8e8 add-missing-e8, and heterotic-so32 add-so16, and type-i add-chan-paton-16 are package mutations, not knobs
  L3 evidence graph        ✓ physis evidence groups by statement hash and inserts a content-addressed Evidence DAG (Statement + Evaluation parents); competing encodings vs evaluations; derived TrustProfile, not a numeric score; not Canonical; not P4
  L3 journal evidence      ✓ Evidence events restore by rebuild from live evaluations; graph_hash is not deserialized; restore does not journal again; not Canonical; not P4
  L3 Super-K dataset       ✓ gut.proton-lifetime-sk compares dim-6 M_GUT^4 scaling to Takenaka et al. PRD 102 112011; not P3N; not dim-5; Tr Q stays encoding-wide
  L3 unique-vacuum domain  ✓ four FormalClaims of predictivity.unique-vacuum name landscape / program axiom / Einstein-Hilbert / Higgs vacuum; still Asserted; not Canonical
  L3 statistical NLL       ✓ PDG sin²θ_W(M_Z) is a Gaussian; GQW centre is an exact π-free Ratio enclosed by sourced PDG 2022 α_s / α_em^{-1} one-sigma hulls; from_lab projects statistical computed from an exact Ratio NLL at the PDG 10^{-5} scale; SU(5) is disjoint, MSSM overlaps without containment; Super-K stays empirical interval-subset; not P3N
  L3 proposer split        ✓ proof-searcher cannot remint; replication-agent reproduces (not P4); explorer cannot score; empirical-analyst scores
  L3 inspect judgment      ✓ physis inspect judgment inverts projected labels; statistical-computed is the PDG GQW cell; empirical-excluded is Super-K; logical-proved requires a receipt
  L3 numerical enclose     ✓ physis enclose independently parses CertifiedNumeric Ratio strings into a NumericCertificate DAG; numerical-verifier unique op; restore rebuilds; not P3F, not Canonical, not P4; P3N count stays 4
  L3 provenance cite       ✓ physis cite independently rebuilds SourceRecord for PDG/Super-K datasets and catalog dossiers; provenance-auditor unique op; restore rebuilds; not P3S, not Canonical, not P4
  L3 encoding round-trip   ✓ physis encode independently parses, round-trips, and reconstructs live IR packages (combinational NAND, Klein-Gordon stencil, Wilson U(1)/SU(2)/SU(3) plaquettes, ohm-circuit lumped branches, bell-test singlet ket, newtonian-gravity inverse-square Binet rhs, linear-medium isotropic-linear constitutive law, maxwell-vacuum source-free homogeneous Faraday, ideal-gas Maxwell-Boltzmann statistics, landauer-engine kT ln2 bound, dirac-fermion naive 1D operator, general-relativity Einstein-Hilbert action, special-relativity Lorentz boost plus catalog interval, composition, and mass-shell trees, planck Bose occupation, de-rham discrete coboundary, turing-machine unrelativized TM, olbers-static inverse-square Euclidean shells, su5-gut complete 5bar + 10, debye-solid 3D ω² continuum, standard-model complete one-generation Weyl, observer-geometry Spin(10) on 10-fibre, dulong-petit harmonic U = 3 N k T, heterotic-e8e8 complete E8 x E8, heterotic-so32 complete SO(32), type-i Chan-Paton SO(32)); a package with lean_ref must bind the catalog identity tree; encode lists each bound identity by claim id (not a kernel proof); encoding-auditor unique op; restore rebuilds; not P3S, not Canonical, not P4
  L3 SI constants          ✓ physis-constants versions SI 2019 defining c, Δν_Cs, e, k, N_A, K_cd as Ratio; h is SI-exact but not a Ratio (i128 denominator overflow); theories still use physis_model f64 Qty
  L3 CODATA G              ✓ physis-constants versions CODATA 2018 G as a one-sigma Interval (JPCRD table XXXI UNIVERSAL); not P3N; h is still not a Ratio; theories still use physis_model f64 Qty
  L3 CODATA alpha          ✓ physis-constants versions CODATA 2018 alpha as a one-sigma Interval 7.2973525693(11)e-3 (JPCRD table XXXI ATOMIC AND NUCLEAR); not inverse-alpha; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA m_p            ✓ physis-constants versions CODATA 2018 proton mass as a one-sigma Interval 1.67262192369(51)e-27 kg (JPCRD table XXXI Proton, p); 10^38 fits i128; m_e is not stored (10^42 overflows); not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA mu0            ✓ physis-constants versions CODATA 2018 vacuum permeability as a one-sigma Interval 1.25663706212(19)e-6 N A^{-2} (JPCRD table XXXI UNIVERSAL); measured after SI 2019, not exact 4pi 10^{-7}; epsilon0 is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA epsilon0       ✓ physis-constants versions CODATA 2018 vacuum permittivity as a one-sigma Interval 8.8541878128(13)e-12 F m^{-1} (JPCRD table XXXI UNIVERSAL); 1/(mu0 c^2) after SI 2019, not exact; Z0 is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA Z0             ✓ physis-constants versions CODATA 2018 characteristic impedance as a one-sigma Interval 376.730313668(57) ohm (JPCRD table XXXI UNIVERSAL); mu0 c after SI 2019, not exact; Y0 is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA inv_alpha      ✓ physis-constants versions CODATA 2018 inverse fine-structure as a one-sigma Interval 137.035999084(21) (JPCRD table XXXI ATOMIC AND NUCLEAR); a different recommended hull from alpha, not 1/alpha as a Ratio; Rydberg is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA Rinf           ✓ physis-constants versions CODATA 2018 Rydberg constant as a one-sigma Interval 10973731.568160(21) m^{-1} (JPCRD table XXXI ATOMIC AND NUCLEAR); not c Rinf; Bohr radius is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA cRinf          ✓ physis-constants versions CODATA 2018 Rydberg frequency as a one-sigma Interval 3.2898419602508(64)e15 Hz (JPCRD table XXXI ATOMIC AND NUCLEAR); not hcRinf; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA hcRinf         ✓ physis-constants versions CODATA 2018 Rydberg energy equivalent as a one-sigma Interval 2.1798723611035(42)e-18 J (JPCRD table XXXI ATOMIC AND NUCLEAR); not the eV conversion; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_mmu         ✓ physis-constants versions CODATA 2018 electron-muon mass ratio as a one-sigma Interval 4.83633169(11)e-3 (JPCRD table XXXI Electron, e-); not electron mass; not the quantum of circulation; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_mp          ✓ physis-constants versions CODATA 2018 electron-proton mass ratio as a one-sigma Interval 5.44617021487(33)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_mn          ✓ physis-constants versions CODATA 2018 electron-neutron mass ratio as a one-sigma Interval 5.4386734424(26)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_md          ✓ physis-constants versions CODATA 2018 electron-deuteron mass ratio as a one-sigma Interval 2.724437107462(96)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_mt          ✓ physis-constants versions CODATA 2018 electron-triton mass ratio as a one-sigma Interval 1.819200062251(90)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_mh          ✓ physis-constants versions CODATA 2018 electron-helion mass ratio as a one-sigma Interval 1.819543074573(79)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA me_malpha      ✓ physis-constants versions CODATA 2018 electron-alpha mass ratio as a one-sigma Interval 1.370933554787(45)e-4 (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA e_me           ✓ physis-constants versions CODATA 2018 electron charge to mass quotient as a one-sigma Interval -1.75882001076(53)e11 C kg^{-1} (JPCRD table XXXI Electron, e-); not electron mass; not P3N; Qty is the recommended signed centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA M_e            ✓ physis-constants versions CODATA 2018 electron molar mass as a one-sigma Interval 5.4857990888(17)e-7 kg mol^{-1} (JPCRD table XXXI Electron, e-); not electron mass in kg; not the mass-in-u row; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA lambdabar_C    ✓ physis-constants versions CODATA 2018 reduced Compton wavelength as a one-sigma Interval 3.8615926796(12)e-13 m (JPCRD table XXXI Electron, e-); not a certificate of alpha a0; not the Compton wavelength; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA lambda_C       ✓ physis-constants versions CODATA 2018 Compton wavelength as a one-sigma Interval 2.42631023867(73)e-12 m (JPCRD table XXXI Electron, e-); not a certificate of 2pi lambdabar_C; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA re             ✓ physis-constants versions CODATA 2018 classical electron radius as a one-sigma Interval 2.8179403262(13)e-15 m (JPCRD table XXXI Electron, e-); not a certificate of alpha^2 a0; Thomson skipped (pi); not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA mu_e           ✓ physis-constants versions CODATA 2018 electron magnetic moment as a one-sigma Interval -9.2847647043(28)e-24 J T^{-1} (JPCRD table XXXI Electron, e-); Thomson skipped (pi); not P3N; Qty is the recommended signed centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA mu_e_muB       ✓ physis-constants versions CODATA 2018 electron magnetic moment to Bohr magneton ratio as a one-sigma Interval -1.00115965218128(18) (JPCRD table XXXI Electron, e-); not the g-factor; not the anomaly; Thomson skipped (pi); not P3N; Qty is the recommended signed centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA mu_e_muN       ✓ physis-constants versions CODATA 2018 electron magnetic moment to nuclear magneton ratio as a one-sigma Interval -1838.28197188(11) (JPCRD table XXXI Electron, e-); not the g-factor; not the anomaly; Thomson skipped (pi); not P3N; Qty is the recommended signed centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA a0             ✓ physis-constants versions CODATA 2018 Bohr radius as a one-sigma Interval 5.29177210903(80)e-11 m (JPCRD table XXXI ATOMIC AND NUCLEAR); Hartree energy is not stored; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 CODATA Eh             ✓ physis-constants versions CODATA 2018 Hartree energy as a one-sigma Interval 4.3597447222071(85)e-18 J (JPCRD table XXXI ATOMIC AND NUCLEAR); not the eV conversion; not P3N; Qty is the recommended centre inside the hull; theories still use physis_model f64 Qty
  L3 Planck h              ✓ physis-constants versions SI 2019 h as SciExact 662607015e-42 J s; 10^42 overflows i128 so it is not a Ratio; ħ is not stored; theories still use physis_model f64 Qty
  L3 constant rebuild      ✓ physis constant independently rebuilds versioned SI/CODATA hashes into a VersionedConstant DAG; provenance-auditor unique op with cite; restore rebuilds; not P3N, not Canonical, not P4; P3N count stays 4
  L3 constant ledger       ✓ physis constant with no name independently rebuilds every LEDGER entry into one VersionedConstant bundle; empty journal name; restore rebuilds; not P3N, not Canonical, not P4; P3N count stays 4
  L3 Qty lockstep          ✓ physis_model c Qty matches integer Ratio to_f64; e/k match IEEE rounding of the SI decimal (SciExact::to_f64, not reduced Ratio::to_f64); h matches SciExact to_f64; G, mu0, epsilon0, Z0, alpha, inv_alpha, cRinf, hcRinf, Rinf, a0, Eh, me_mmu, me_mp, me_mn, me_md, me_mt, me_mh, me_malpha, e_me, M_e, lambdabar_C, lambda_C, re, mu_e, mu_e_muB, mu_e_muN, and m_p Qty values are the CODATA 2018 centres inside the one-sigma hulls; ħ is not stored; theories still evaluate with f64 Qty; not a kernel proof
  L3 loop constant         ✓ physis loop rebuilds the full LEDGER VersionedConstant bundle after cite; empty journal name; not P3N, not Canonical, not P4; P3N count stays 4
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
