# 020 — Proof-carrying Physis (Level 3)

Status: active (Milestones 1–10 sliced; Lean/nanoda dual kernel wired for the catalog)
Layer: all
Id: `proof-carrying`

## Objective

Nothing gains authority merely because an agent wrote code that returns
`Holds`. Authority comes from explicit, independently checkable artifacts.

## What is true now

### Milestone 1 — trust model

- Orthogonal axes: `ClaimClass`, `DerivationAssurance`, `EmpiricalStatus`,
  `SemanticAssurance`. No `Epistemic::Theorem`. No `MachineProved` enum.
- `Verified<T>` has private fields, crate-private mint, and **no
  `Deserialize` impl** (JSON cannot mint a kernel proof).
- `Challenge` has private fields, is constructed only by
  `Challenge::generate`, and has **no `Deserialize` impl** (the solver
  cannot choose the statement, Lean type, or polynomial).
- `FormalClaim` has private fields, is constructed only by
  `FormalClaim::from_claim` (which recomputes the statement hash; a
  forged hash on `Claim` is not copied through), and has **no
  `Deserialize` impl** (JSON cannot mint a catalog identity).
- Every claim has assumptions, a domain, and a SHA-256 statement identity
  that is **derived** from the live sentence, class, layer, assumptions,
  domain, and first-class commitments (quantifiers, units, constants,
  boundary conditions, conventions, theory version, definitions, datasets,
  and formal-library identity). There is no stored `Claim.statement_hash`
  field and no `Deserialize` on `Claim`: `Claim.statement`, id, class, layer,
  assumptions, domain, and commitments are private so a public assignment
  cannot rebind a kernel receipt. Same-module mutation of the sentence
  still cannot keep a stale hash. JSON cannot mint a catalog
  identity. Changing ∀/∃, a sign, a unit, a constant, or a boundary is a
  new hash. The lab slug is unchanged. P3F looks up the live hash, not
  the slug. Catalog identities are `forall` in
  unversioned Physlib with a named `DomainOfValidity` (discrete coboundary
  simplex, 1+1 Minkowski `|β|<1`, collinear `|u|<1,|v|<1`). Physlib
  forall with the encoding-wide placeholder is not those identities.
  Catalog `lab_claim` includes the row's axioms (`integer-arithmetic`,
  `discrete-coboundary`, Minkowski signature, Einstein addition) as an
  `AssumptionSet`; Lean kernel axioms stay on the receipt. Physlib forall
  with only `encoding-is-the-model` is not those identities.
  Poincaré is not catalogued. `field.second-order-accurate`
  names `|k a| < 1` as its domain. `field.dispersion-continuum-limit`
  names the longest non-zero lattice mode, not that Richardson probe.
  Ohm-circuit `em.quasi-static-valid` names `λ > 100 ×` circuit size;
  Maxwell's inapplicable copy stays encoding-wide. GUT-scale `3/8`, GQW at `M_Z`, and
  the PDG interval name unification-scale / `M_Z` as
  `DomainOfValidity` (not the encoding-wide placeholder). SM anomalies,
  hypercharge solve, and hydrogen neutrality name one generation /
  hydrogen. Super-K `p→e+π0` names the dim-6 / 90% CL regime
  (Takenaka et al., Phys. Rev. D 102, 112011). GUT `Tr Q` stays
  encoding-wide. `predictivity.unique-vacuum` encodings name the
  flux/moduli landscape, the observer-geometry program axiom,
  classical Einstein–Hilbert plus Λ, and the SM Higgs vacuum;
  they remain `Asserted` (heuristic/conjecture), not a kernel proof.
  `dec.hodge-harmonic` names discrete combinatorial Hodge
  on finite simplicial 1-cochains, not the smooth Hodge theorem.
  `thermo.high-t-classical` names `T/Θ ≥ 8`; `thermo.debye-t3` names the
  `Θ/20` phonon probe; `thermo.rj-ir-limit` names `hν = 0.01 kT`;
  `gauge.exact-area-law-2d` names 2D Wilson plaquette factorization.
  On `dulong-petit`, `thermo.dulong-petit` names harmonic `U = 3 N k T`;
  on `einstein-solid` and `debye-solid` it stays encoding-wide.
  On `heterotic-e8e8`, `consistency.anomaly-cancellation` names complete
  `E8 x E8`; on `heterotic-so32` it names complete `SO(32)`;
  on `type-i` it names complete `Chan-Paton SO(32)`;
  Type II copies stay encoding-wide.
  Lean compiler versions live on the receipt, not
  the sentence.
- `physis why` / `physis epistemics` do not print a `theorem` tag.
  `Judgment` has no `Deserialize` impl: JSON cannot mint
  `logical proved`. `LogicalJudgment` has no public `Proved`
  constructor: only `from_lab` can produce proved, and only with a
  dual-checked receipt. `NumericJudgment` has no public `Certified`
  constructor: only `from_lab` can produce a certified enclosure, and
  only from a `CertifiedNumeric` Holds. `EmpiricalJudgment` has no public
  `Compatible` constructor: only `from_lab` can produce compatible, and
  only from a registered empirical overlay. `HeuristicJudgment` has no
  public `Suggestive` constructor: only `from_lab` can produce it.
  `StatisticalJudgment` has no public `Computed` constructor: `from_lab`
  projects `statistical computed` only from an exact Gaussian NLL overlay
  on an empirical or measurement claim; a crate outside physis-core
  cannot mint one. Super-K interval-subset stays empirical. `Verdict` has no `Deserialize` impl: JSON
  cannot mint a `certified-numeric` overlay or an encoding-review tag.
  `Claim` derivation, empirical, and semantic fields are private: a
  theory cannot assign `CertifiedNumeric` on the claim. Overlays live on
  `Verdict`, whose derivation / empirical / semantic / enclosure / NLL
  fields are also private: a public assignment cannot mint `CertifiedNumeric`.
  The overlay paths are `Verdict::with_certified_numeric` and
  `Verdict::with_statistical_nll`.
  The lab projects `Judgment` from evaluator
  + receipts via `from_lab`. Evaluator `holds` without a dual-checked
  receipt is `logical undetermined`.
-   `TrustProfile` is derived from receipts and reviews. P3F cannot be set
  as an enum; P4 is not assigned from an in-process remint. There is no
  `SemanticAssurance::Canonical` variant; P3S is taken from the review
  store of the live `statement_hash`, not from `Verdict.semantic`. P3N is
  earned when an evaluator overlays `CertifiedNumeric` after an exact
  `Ratio` cancellation or an exact `Ratio` solve (Standard Model chiral
  anomalies, and the hypercharge quadratic whose discriminant is a
  square in Q, hydrogen neutrality from `Q = T₃ + Y`, and GUT-scale
  `sin²θ_W = Tr(T₃²)/Tr(Q²) = 3/8`). GUT `Tr Q = ΣY` is the gravitational
  anomaly already certified, not a second P3N. Georgi–Quinn–Weinberg
  running at `M_Z` and the sourced PDG input-interval enclosure (and the
  3% heuristic hit) are not P3N. `CrossChecked` / P2 is earned when two independent
  executable paths agree (`dec.hodge-harmonic`: Laplacian nullity vs
  coboundary `b₁` on this complex; the cell names that discrete regime).
  Euler–Poincaré stays `executed`: with these Betti
  formulas, `b₀−b₁+b₂ ≡ V−E+F` is rank-cancellation, not a second path.
  Neither overlay is a Lean receipt. A P3F
  `Unreviewed` result is labelled dangerous. `physis why` prints a typed
  `Judgment` (evaluator `holds` is `logical undetermined`; a
  `CertifiedNumeric` Holds is `numeric certified` with a display
  enclosure, not a kernel proof; a coarse numeric order is `numeric
  unresolved`, not a failed theorem; a PDG Gaussian NLL is
  `statistical computed`, not `empirical compatible`) and the
  transitive axiom closure from `AxiomLedger`.

### Milestone 2 — dual-check receipts (exact + Lean)

Trusted side: `physis-proof::Challenge::generate` from a `FormalClaim`.
Both have private fields and no Deserialize. `FormalClaim::from_claim`
recomputes the statement hash from the live sentence. Untrusted side:
`UntrustedProof`. The only public mint is `physis_verifier::verify`,
which *runs* two checkers.

Catalogued polynomial identities, dual-expanded (recursive AST vs postfix
stack) *and* kernel-checked as Physlib theorems (`formal/physlib`):

- `dec.d-squared-zero`: `(b−a)−(c−a)+(c−b) ≡ 0` (`d_squared_zero`, `omega`)
- `sr.invariant-interval`: `(t−βx)² − (x−βt)² − (1−β²)(t²−x²) ≡ 0`
  (`invariant_interval`, `grind`)
- `sr.subluminal-composition`: `(1+uv)² − (u+v)² − (1−u²)(1−v²) ≡ 0`
  (`subluminal_composition`, `grind`). Algebraic content of Einstein
  addition; `|w|<1` over ℝ remains the evaluator.
- `sr.energy-momentum-invariant`: `(E−βp)² − (p−βE)² − (1−β²)(E²−p²) ≡ 0`
  (`energy_momentum_invariant`, `grind`). The Minkowski bilinear form
  on 4-momentum: the same algebraic obligation as the interval with
  `(t,x) → (E,p)`, not a new postulate. Axioms stay
  `integer-arithmetic` and `minkowski-interval-signature`. The typed
  rest-mass check `E² − (pc)² = (mc²)²` remains the evaluator.

A one-byte mutation of the challenge bytes is `ChallengeTampered`.
A sign flip of the identity fails both expanders. `axiom` / `sorry` /
`admit` in Lean source is `UnauthorizedAxiom`. A compiled `True` theorem
is `StatementMismatch` against the d² challenge. `LeanExport` bytes
without a second kernel, or missing `lean`/`lake`/`lean4export`, is
`LeanPipelineNotWired` — **refuses to mint**.

When Lean 4.34.0-rc2 and `lean4export` 3.1.0 (replayed by nanoda 0.4.16)
are present, `verify(LeanSource)` compiles Physlib with the Lean kernel,
exports the theorem whose compacted type matches the challenge, and
replays that export with nanoda. The receipt is `FormalBackend::Lean4`
and lists Lean's standard axioms (`propext`, `Quot.sound`,
`Classical.choice`) plus the catalog's physical postulates. CI installs
those tools; a local checkout without them still mints
`ExactCertificate` from `physis prove`.

`ExactCertificate` is not a Lean kernel proof; the receipt says so.

### Milestones 3–10 — first slices

| Slice | Crate / CLI | What it does |
|---|---|---|
| 3 | `physis-provenance` | Rejects `source: textbook`; requires a page/equation/… locator |
| 4 | `physis-numeric`, `physis-data` | Exact `Ratio` / `Interval`; SU(5) `3/8` disjoint from PDG `sin²θ_W(M_Z)` |
| 5 | `physis-store` | Content-addressed DAG; descendants only are invalidated |
| 6 | `prove falsify sweep branch compare sensitivity` | Structured agent ops. Knob diffs are scientific-axis (`VerdictKind` plus derivation / empirical / projected judgment); legacy kind-only journals still replay |
| 7 | `physis-ir` | Line-oriented theory package plus constrained mutations (`render_package`, `apply_mutation`, `certify_round_trip`); combinational NAND netlist, Klein–Gordon stencil, Wilson U(1)/SU(2)/SU(3) 1×1 plaquettes, ohm-circuit lumped branches, bell-test singlet ket, newtonian-gravity inverse-square Binet rhs, linear-medium isotropic-linear constitutive law, maxwell-vacuum source-free homogeneous Faraday, ideal-gas Maxwell–Boltzmann statistics, landauer-engine `kT ln2` bound, dirac-fermion naive 1D operator, general-relativity Einstein–Hilbert action, special-relativity Lorentz boost plus catalog interval, composition, and mass-shell trees, and planck Bose occupation, and de-rham discrete coboundary, and turing-machine unrelativized TM, and olbers-static inverse-square Euclidean shells, and su5-gut complete `5bar + 10`, and debye-solid 3D `ω²` continuum, and standard-model complete one-generation Weyl, and observer-geometry Spin(10) on 10-fibre, and dulong-petit harmonic `U = 3 N k T`, and heterotic-e8e8 complete `E8 x E8`, and heterotic-so32 complete `SO(32)`, and type-i `Chan-Paton SO(32)` are live packages; `physis encode` independently round-trips them and binds a live `lean_ref` to the catalog identity tree (token packages skip; a cosmetic Physlib pointer without the tree fails closed); not a kernel proof, not a Lean replacement |
| 8 | `physis-audit`, `physis audit` | Red-team corpus must fail to promote |
| 9 | `physis design` | Rank theory pairs by discriminating claim count |
| 10 | `physis loop` | Observe → hypothesize (chosen/fitted knobs and IR package forks) → prove → falsify → enclose → cite → constant → encode → judge → replicate → design → audit → review |
| origin | `KnobSpec.origin`, `physis inspect` | Distinguish chosen/fitted knobs from measured ones; invert trust/class/origin/gap/judgment |
| gaps | `physis gaps`, `NodeKind::KnowledgeGap` | Live gap graph, content-addressed; rebuilt, not deserialized. `MissingTheorem` only for evaluator-Holds claims without a receipt; Fails is decided, not a missing lemma. `InsufficientPrecision` is overlap without containment on an empirical receipt, **or** a lattice too coarse to certify a numerical order (`field.second-order-accurate`, `|k a| ≥ 1`). `ComputationallyIntractable` is coNP-complete / exponential search, not Rice. `MissingDataset` is an empirical prediction with no registered dataset. Super-K `p→e+π0` is registered; `gut.proton-lifetime-sk` is the dim-6 comparison, not that hole |
| lemmas | `Claim.depends_on` | Live lemma edges in `gaps` / `why`; not statement identity; never deserialized as authority |
| evidence | `physis evidence`, `NodeKind::Evidence` | Competing encodings (distinct FormalClaims of one slug) and competing evaluations; Statement + Evaluation parents; content-addressed, not deserialized; confidence is derived TrustProfile, not a numeric score; never Canonical or P4 |
| trust-gate | `Lab::exec` | `reproduce` and loop-review require P3F. Standalone `review` stays encoding-axis. Observation is free |
| roles | `Role`, `ResearchBudget`, `physis formalize` | Named processes propose; only `verify` mints. Explorer cannot prove or score. Proof-searcher cannot remint. Replication-agent remints (not P4). Empirical-analyst scores. Numerical-verifier encloses. Provenance-auditor cites and independently rebuilds versioned constants (`physis constant`). Encoding-auditor round-trips live IR packages. Judge rebuilds `from_lab` (JSON cannot mint `logical proved`). Budget is a cap, not a proof |
| semantic | `physis-semantic`, `physis review` | Provenance + independent IR encoding + corpus, bound to the catalog FormalClaim; never `Canonical` |
| constants | `physis-constants`, `physis constant` | Versioned SI 2019 defining `c`, `Δν_Cs`, `e`, `k`, `N_A`, `K_cd` as exact `Ratio`. Planck `h` is SI-exact `SciExact` `662607015e-42` (not a `Ratio`: `i128` denominator overflow). `ħ` is not stored. CODATA 2018 `G` is a one-sigma `Interval` (JPCRD table XXXI UNIVERSAL). CODATA 2018 `μ₀` is a one-sigma `Interval` `1.25663706212(19)×10^{-6}` N A⁻² (JPCRD table XXXI UNIVERSAL). CODATA 2018 `ε₀` is a one-sigma `Interval` `8.8541878128(13)×10^{-12}` F m⁻¹ (JPCRD table XXXI UNIVERSAL). CODATA 2018 `Z₀` is a one-sigma `Interval` `376.730313668(57)` ohm (JPCRD table XXXI UNIVERSAL); `Y₀` is not stored. CODATA 2018 `α` is a one-sigma `Interval` `7.2973525693(11)×10^{-3}` (JPCRD table XXXI ATOMIC AND NUCLEAR). CODATA 2018 inverse-α is a one-sigma `Interval` `137.035999084(21)` from the same table. CODATA 2018 Rydberg frequency `cR∞` is a one-sigma `Interval` `3.2898419602508(64)×10^{15}` Hz from the same table. CODATA 2018 Rydberg energy equivalent `hcR∞` is a one-sigma `Interval` `2.1798723611035(42)×10^{-18}` J from the same table; the eV conversion is not stored. CODATA 2018 Rydberg `R∞` is a one-sigma `Interval` `10973731.568160(21)` m⁻¹ from the same table. CODATA 2018 Bohr radius `a₀` is a one-sigma `Interval` `5.29177210903(80)×10^{-11}` m from the same table. CODATA 2018 Hartree energy `E_h` is a one-sigma `Interval` `4.3597447222071(85)×10^{-18}` J from the same table; the eV conversion is not stored. CODATA 2018 electron-muon mass ratio `m_e/m_μ` is a one-sigma `Interval` `4.83633169(11)×10^{-3}` (JPCRD table XXXI Electron, e-). CODATA 2018 electron-proton mass ratio `m_e/m_p` is a one-sigma `Interval` `5.44617021487(33)×10^{-4}` from the same section. CODATA 2018 electron-neutron mass ratio `m_e/m_n` is a one-sigma `Interval` `5.4386734424(26)×10^{-4}` from the same section. CODATA 2018 electron-deuteron mass ratio `m_e/m_d` is a one-sigma `Interval` `2.724437107462(96)×10^{-4}` from the same section. CODATA 2018 electron-triton mass ratio `m_e/m_t` is a one-sigma `Interval` `1.819200062251(90)×10^{-4}` from the same section. CODATA 2018 electron-helion mass ratio `m_e/m_h` is a one-sigma `Interval` `1.819543074573(79)×10^{-4}` from the same section. CODATA 2018 electron to alpha particle mass ratio `m_e/m_α` is a one-sigma `Interval` `1.370933554787(45)×10^{-4}` from the same section. CODATA 2018 electron charge to mass quotient `−e/m_e` is a one-sigma `Interval` `−1.75882001076(53)×10^{11}` C kg⁻¹ from the same section. CODATA 2018 electron molar mass `M_e` is a one-sigma `Interval` `5.4857990888(17)×10^{-7}` kg mol⁻¹ from the same section. CODATA 2018 reduced Compton wavelength `ƛ_C` is a one-sigma `Interval` `3.8615926796(12)×10^{-13}` m from the same section. CODATA 2018 Compton wavelength `λ_C` is a one-sigma `Interval` `2.42631023867(73)×10^{-12}` m from the same section. CODATA 2018 classical electron radius `r_e` is a one-sigma `Interval` `2.8179403262(13)×10^{-15}` m from the same section. CODATA 2018 electron magnetic moment `μ_e` is a one-sigma `Interval` `−9.2847647043(28)×10^{-24}` J T⁻¹ from the same section. CODATA 2018 electron magnetic moment to Bohr magneton ratio `μ_e/μ_B` is a one-sigma `Interval` `−1.00115965218128(18)` from the same section. CODATA 2018 electron magnetic moment to nuclear magneton ratio `μ_e/μ_N` is a one-sigma `Interval` `−1838.28197188(11)` from the same section. CODATA 2018 electron magnetic-moment anomaly `ae` is a one-sigma `Interval` `1.15965218128(18)×10^{-3}` from the same section. CODATA 2018 electron g-factor `ge` is a one-sigma `Interval` `−2.00231930436256(35)` from the same section. CODATA 2018 electron-muon magnetic-moment ratio `mu_e_mmu` is a one-sigma `Interval` `206.7669883(46)` from the same section. CODATA 2018 electron-proton magnetic-moment ratio `mu_e_mup` is a one-sigma `Interval` `−658.21068789(20)` from the same section. CODATA 2018 electron to shielded-proton magnetic-moment ratio `mu_e_mu0p` is a one-sigma `Interval` `−658.2275971(72)` from the same section. CODATA 2018 electron-neutron magnetic-moment ratio `mu_e_mun` is a one-sigma `Interval` `960.92050(23)` from the same section. CODATA 2018 electron-deuteron magnetic-moment ratio `mu_e_mud` is a one-sigma `Interval` `−2143.9234915(56)` from the same section. CODATA 2018 electron to shielded-helion magnetic-moment ratio `mu_e_mu0h` is a one-sigma `Interval` `864.058257(10)` from the same section. CODATA 2018 muon mass `m_mu` is a one-sigma `Interval` `1.883531627(42)×10^{-28}` kg (JPCRD table XXXI Muon, mu-). CODATA 2018 muon mass in u `m_mu_u` is a one-sigma `Interval` `0.1134289259(25)` u from the same section. CODATA 2018 muon mass energy equivalent `m_mu_c2` is a one-sigma `Interval` `1.692833804(38)×10^{-11}` J from the same section. CODATA 2018 muon mass energy equivalent in MeV `m_mu_c2_MeV` is a one-sigma `Interval` `105.6583755(23)` MeV from the same section. CODATA 2018 muon-electron mass ratio `mmu_me` is a one-sigma `Interval` `206.7682830(46)` from the same section. The muon-tau mass ratio is a PDG reprint of `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored. CODATA 2018 muon-proton mass ratio `mmu_mp` is a one-sigma `Interval` `0.1126095264(25)` from the same section. CODATA 2018 muon-neutron mass ratio `mmu_mn` is a one-sigma `Interval` `0.1124545170(25)` from the same section. CODATA 2018 muon molar mass `M_mu` is a one-sigma `Interval` `1.134289259(25)×10^{-4}` kg mol⁻¹ from the same section. The reduced muon Compton wavelength is ħ/m_μc and is not stored. CODATA 2018 muon Compton wavelength `lambda_C_mu` is a one-sigma `Interval` `1.173444110(26)×10^{-14}` m from the same section. CODATA 2018 muon magnetic moment `mu_mu` is a one-sigma `Interval` `−4.49044830(10)×10^{-26}` J T⁻¹ from the same section. CODATA 2018 muon magnetic moment to Bohr magneton ratio `mu_mu_muB` is a one-sigma `Interval` `−4.84197047(11)×10^{-3}` from the same section. CODATA 2018 muon magnetic moment to nuclear magneton ratio `mu_mu_muN` is a one-sigma `Interval` `−8.89059703(20)` from the same section. CODATA 2018 muon magnetic-moment anomaly `amu` is a one-sigma `Interval` `1.16592089(63)×10^{-3}` from the same section. CODATA 2018 muon g-factor `gmu` is a one-sigma `Interval` `−2.0023318418(13)` from the same section. CODATA 2018 muon-proton magnetic-moment ratio `mu_mu_mup` is a one-sigma `Interval` `−3.183345142(71)` from the same section. CODATA 2018 `m_p` is a one-sigma `Interval` `1.67262192369(51)×10^{-27}` kg (JPCRD table XXXI Proton, p). CODATA 2018 proton mass in u `m_p_u` is a one-sigma `Interval` `1.007276466621(53)` u from the same section. CODATA 2018 proton mass energy equivalent `m_p_c2` is a one-sigma `Interval` `1.50327761598(46)×10^{-10}` J from the same section. CODATA 2018 proton mass energy equivalent in MeV `m_p_c2_MeV` is a one-sigma `Interval` `938.27208816(29)` MeV from the same section. CODATA 2018 proton-electron mass ratio `mp_me` is a one-sigma `Interval` `1836.15267343(11)` from the same section. CODATA 2018 proton-muon mass ratio `mp_mmu` is a one-sigma `Interval` `8.88024337(20)` from the same section. The proton-tau mass ratio is a PDG reprint of `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored. CODATA 2018 proton-neutron mass ratio `mp_mn` is a one-sigma `Interval` `0.99862347812(49)` from the same section. CODATA 2018 proton charge-to-mass quotient `e_mp` is a one-sigma `Interval` `9.5788331560(29)×10^{7}` C kg⁻¹ from the same section. CODATA 2018 proton molar mass `M_p` is a one-sigma `Interval` `1.00727646627(31)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 proton Compton wavelength `lambda_C_p` is a one-sigma `Interval` `1.32140985539(40)×10^{-15}` m from the same section; CODATA 2018 proton rms charge radius `rp` is a one-sigma `Interval` `8.414(19)×10^{-16}` m from the same section; CODATA 2018 proton magnetic moment `mu_p` is a one-sigma `Interval` `1.41060679736(60)×10^{-26}` J T⁻¹ from the same section; CODATA 2018 proton magnetic moment to Bohr magneton ratio `mu_p_muB` is a one-sigma `Interval` `1.52103220230(46)×10^{-3}` from the same section; CODATA 2018 proton magnetic moment to nuclear magneton ratio `mu_p_muN` is a one-sigma `Interval` `2.79284734463(82)` from the same section; CODATA 2018 proton g-factor `gp` is a one-sigma `Interval` `5.5856946893(16)` from the same section; CODATA 2018 proton-neutron magnetic-moment ratio `mu_p_mun` is a one-sigma `Interval` `−1.45989805(34)` from the same section; CODATA 2018 shielded proton magnetic moment `mu0p` is a one-sigma `Interval` `1.410570560(15)×10^{-26}` J T⁻¹ from the same section; CODATA 2018 shielded proton magnetic moment to Bohr magneton ratio `mu0p_muB` is a one-sigma `Interval` `1.520993128(17)×10^{-3}` from the same section; CODATA 2018 shielded proton magnetic moment to nuclear magneton ratio `mu0p_muN` is a one-sigma `Interval` `2.792775599(30)` from the same section; CODATA 2018 proton magnetic shielding correction `sigma0p` is a one-sigma `Interval` `2.5689(11)×10^{-5}` from the same section; CODATA 2018 neutron mass `m_n` is a one-sigma `Interval` `1.67492749804(95)×10^{-27}` kg (JPCRD table XXXI Neutron, n); CODATA 2018 neutron mass in u `m_n_u` is a one-sigma `Interval` `1.00866491595(49)` u from the same section; CODATA 2018 neutron mass energy equivalent `m_n_c2` is a one-sigma `Interval` `1.50534976287(86)×10^{-10}` J from the same section; CODATA 2018 neutron mass energy equivalent in MeV `m_n_c2_MeV` is a one-sigma `Interval` `939.56542052(54)` MeV from the same section; CODATA 2018 neutron-electron mass ratio `mn_me` is a one-sigma `Interval` `1838.68366173(89)` from the same section; CODATA 2018 neutron-muon mass ratio `mn_mmu` is a one-sigma `Interval` `8.89248406(20)` from the same section; CODATA 2018 neutron-proton mass ratio `mn_mp` is a one-sigma `Interval` `1.00137841931(49)` from the same section; CODATA 2018 neutron-proton mass difference `mn_minus_mp` is a one-sigma `Interval` `2.30557435(82)×10^{-30}` kg from the same section; CODATA 2018 neutron-proton mass difference in u `mn_minus_mp_u` is a one-sigma `Interval` `1.38844933(49)×10^{-3}` u from the same section; CODATA 2018 neutron-proton mass difference energy equivalent `mn_minus_mp_c2` is a one-sigma `Interval` `2.07214689(74)×10^{-13}` J from the same section; CODATA 2018 neutron-proton mass difference energy equivalent in MeV `mn_minus_mp_c2_MeV` is a one-sigma `Interval` `1.29333236(46)` MeV from the same section; CODATA 2018 neutron molar mass `M_n` is a one-sigma `Interval` `1.00866491560(57)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 neutron Compton wavelength `lambda_C_n` is a one-sigma `Interval` `1.31959090581(75)×10^{-15}` m from the same section; CODATA 2018 neutron magnetic moment `mu_n` is a one-sigma `Interval` `−9.6623651(23)×10^{-27}` J T⁻¹ from the same section; CODATA 2018 neutron magnetic moment to Bohr magneton ratio `mu_n_muB` is a one-sigma `Interval` `−1.04187563(25)×10^{-3}` from the same section; CODATA 2018 neutron magnetic moment to nuclear magneton ratio `mu_n_muN` is a one-sigma `Interval` `−1.91304273(45)` from the same section; CODATA 2018 neutron g-factor `gn` is a one-sigma `Interval` `−3.82608545(90)` from the same section; CODATA 2018 neutron-electron magnetic-moment ratio `mu_n_mue` is a one-sigma `Interval` `1.04066882(25)×10^{-3}` from the same section; CODATA 2018 neutron-proton magnetic-moment ratio `mu_n_mup` is a one-sigma `Interval` `−0.68497934(16)` from the same section; CODATA 2018 neutron to shielded-proton magnetic-moment ratio `mu_n_mu0p` is a one-sigma `Interval` `−0.68499694(16)` from the same section; CODATA 2018 deuteron mass `m_d` is a one-sigma `Interval` `3.3435837724(10)×10^{-27}` kg (JPCRD table XXXI Deuteron, d); CODATA 2018 deuteron mass in u `m_d_u` is a one-sigma `Interval` `2.013553212745(40)` u from the same section; CODATA 2018 deuteron mass energy equivalent `m_d_c2` is a one-sigma `Interval` `3.00506323102(91)×10^{-10}` J from the same section; CODATA 2018 deuteron mass energy equivalent in MeV `m_d_c2_MeV` is a one-sigma `Interval` `1875.61294257(57)` MeV from the same section; CODATA 2018 deuteron-electron mass ratio `md_me` is a one-sigma `Interval` `3670.48296788(13)` from the same section; CODATA 2018 deuteron-proton mass ratio `md_mp` is a one-sigma `Interval` `1.99900750139(11)` from the same section; CODATA 2018 deuteron molar mass `M_d` is a one-sigma `Interval` `2.01355321205(61)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 deuteron rms charge radius `rd` is a one-sigma `Interval` `2.12799(74)×10^{-15}` m from the same section; CODATA 2018 deuteron magnetic moment `mu_d` is a one-sigma `Interval` `4.330735094(11)×10^{-27}` J T⁻¹ from the same section; CODATA 2018 deuteron magnetic moment to Bohr magneton ratio `mu_d_muB` is a one-sigma `Interval` `4.669754570(12)×10^{-4}` from the same section; CODATA 2018 deuteron magnetic moment to nuclear magneton ratio `mu_d_muN` is a one-sigma `Interval` `0.8574382338(22)` from the same section; CODATA 2018 deuteron g-factor `gd` is a one-sigma `Interval` `0.8574382338(22)` from the same section; CODATA 2018 deuteron-electron magnetic-moment ratio `mu_d_mue` is a one-sigma `Interval` `−4.664345551(12)×10^{-4}` from the same section; CODATA 2018 deuteron-proton magnetic-moment ratio `mu_d_mup` is a one-sigma `Interval` `0.30701220939(79)` from the same section; CODATA 2018 deuteron-neutron magnetic-moment ratio `mu_d_mun` is a one-sigma `Interval` `−0.44820653(11)` from the same section; CODATA 2018 triton mass `m_t` is a one-sigma `Interval` `5.0073567446(15)×10^{-27}` kg from JPCRD table XXXI Triton, t; CODATA 2018 triton mass in u `m_t_u` is a one-sigma `Interval` `3.01550071621(12)` u from the same section; CODATA 2018 triton mass energy equivalent `m_t_c2` is a one-sigma `Interval` `4.5003878060(14)×10^{-10}` J from the same section; CODATA 2018 triton mass energy equivalent in MeV `m_t_c2_MeV` is a one-sigma `Interval` `2808.92113298(85)` MeV from the same section; CODATA 2018 triton-electron mass ratio `mt_me` is a one-sigma `Interval` `5496.92153573(27)` from the same section; CODATA 2018 triton-proton mass ratio `mt_mp` is a one-sigma `Interval` `2.99371703414(15)` from the same section; CODATA 2018 triton molar mass `M_t` is a one-sigma `Interval` `3.01550071517(92)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 triton magnetic moment `mu_t` is a one-sigma `Interval` `1.5046095202(30)×10^{-26}` J T⁻¹ from the same section; CODATA 2018 triton magnetic moment to Bohr magneton ratio `mu_t_muB` is a one-sigma `Interval` `1.6223936651(32)×10^{-3}` from the same section; CODATA 2018 triton magnetic moment to nuclear magneton ratio `mu_t_muN` is a one-sigma `Interval` `2.9789624656(59)` from the same section; CODATA 2018 triton g-factor `gt` is a one-sigma `Interval` `5.957924931(12)` from the same section; CODATA 2018 helion mass `m_h` is a one-sigma `Interval` `5.0064127796(15)×10^{-27}` kg from JPCRD table XXXI Helion, h; CODATA 2018 helion mass in u `m_h_u` is a one-sigma `Interval` `3.014932247175(97)` u from the same section; CODATA 2018 helion mass energy equivalent `m_h_c2` is a one-sigma `Interval` `4.4995394125(14)×10^{-10}` J from the same section; CODATA 2018 helion mass energy equivalent in MeV `m_h_c2_MeV` is a one-sigma `Interval` `2808.39160743(85)` MeV from the same section; CODATA 2018 helion-electron mass ratio `mh_me` is a one-sigma `Interval` `5495.88528007(24)` from the same section; CODATA 2018 helion-proton mass ratio `mh_mp` is a one-sigma `Interval` `2.99315267167(13)` from the same section; CODATA 2018 helion molar mass `M_h` is a one-sigma `Interval` `3.01493224613(91)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 helion magnetic moment `mu_h` is a one-sigma `Interval` `−1.074617532(13)×10^{-26}` J T⁻¹ from the same section. CODATA 2018 helion magnetic moment to Bohr magneton ratio `mu_h_muB` is a one-sigma `Interval` `−1.158740958(14)×10^{-3}` from the same section. CODATA 2018 helion magnetic moment to nuclear magneton ratio `mu_h_muN` is a one-sigma `Interval` `−2.127625307(25)` from the same section. CODATA 2018 helion g-factor `gh` is a one-sigma `Interval` `−4.255250615(50)` from the same section. CODATA 2018 shielded helion magnetic moment `mu0h` is a one-sigma `Interval` `−1.074553090(13)×10^{-26}` J T⁻¹ from the same section; CODATA 2018 shielded helion mag. mom. to Bohr magneton ratio `mu0h_muB` is a one-sigma `Interval` `−1.158671471(14)×10^{-3}` from the same section; CODATA 2018 shielded helion mag. mom. to nuclear magneton ratio `mu0h_muN` is a one-sigma `Interval` `−2.127497719(25)` from the same section; CODATA 2018 shielded helion to proton mag. mom. ratio `mu0h_mup` is a one-sigma `Interval` `−0.7617665618(89)` from the same section; CODATA 2018 shielded helion to shielded proton mag. mom. ratio `mu0h_mu0p` is a one-sigma `Interval` `−0.7617861313(33)` from the same section; CODATA 2018 alpha particle mass `m_alpha` is a one-sigma `Interval` `6.6446573357(20)×10^{-27}` kg from JPCRD table XXXI Alpha particle, a; CODATA 2018 alpha particle mass in u `m_alpha_u` is a one-sigma `Interval` `4.001506179127(63)` u from the same section; CODATA 2018 alpha particle mass energy equivalent `m_alpha_c2` is a one-sigma `Interval` `5.9719201914(18)×10^{-10}` J from the same section; CODATA 2018 alpha particle mass energy equivalent in MeV `m_alpha_c2_MeV` is a one-sigma `Interval` `3727.3794066(11)` MeV from the same section; CODATA 2018 alpha particle-electron mass ratio `malpha_me` is a one-sigma `Interval` `7294.29954142(24)` from the same section; CODATA 2018 alpha particle-proton mass ratio `malpha_mp` is a one-sigma `Interval` `3.97259969009(22)` from the same section; CODATA 2018 alpha particle molar mass `M_alpha` is a one-sigma `Interval` `4.0015061777(12)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 atomic mass constant `m_u` is a one-sigma `Interval` `1.66053906660(50)×10^{-27}` kg from JPCRD table XXXI PHYSICOCHEMICAL; CODATA 2018 atomic mass constant energy equivalent `m_u_c2` is a one-sigma `Interval` `1.49241808560(45)×10^{-10}` J from the same section; CODATA 2018 atomic mass constant energy equivalent in MeV `m_u_c2_MeV` is a one-sigma `Interval` `931.49410242(28)` MeV from the same section; CODATA 2018 molar mass constant `M_u` is a one-sigma `Interval` `0.99999999965(30)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 molar mass of carbon-12 `M_12C` is a one-sigma `Interval` `11.9999999958(36)×10^{-3}` kg mol⁻¹ from the same section; CODATA 2018 molar Planck constant `NAh` is an exact `Ratio` `3.99031271289343140×10^{-10}` J Hz⁻¹ mol⁻¹ from the same section; CODATA 2018 molar gas constant `NAk` is an exact `Ratio` `8.31446261815324` J mol⁻¹ K⁻¹ from the same section; CODATA 2018 Faraday constant `NAe` is an exact `Ratio` `96485.3321233100184` C mol⁻¹ from the same section; CODATA 2018 standard-state pressure `p0` is an exact `Ratio` `100000` Pa from the same section; CODATA 2018 standard atmosphere `atm` is an exact `Ratio` `101325` Pa from the same section; CODATA 2018 molar volume of ideal gas `Vm` is an exact `Ratio` `0.022710954641485575` m³ mol⁻¹ from the same section at 273.15 K and 100 kPa; CODATA 2018 Loschmidt constant `n0` is an exact `Ratio` `200000000000000000000000000000000000/7542485487` m⁻³ from the same section at 273.15 K and 100 kPa; CODATA 2018 molar volume of ideal gas at 101.325 kPa `Vm_atm` is an exact `Ratio` `378515910691426251/16887500000000000000` m³ mol⁻¹ from the same section at 273.15 K; CODATA 2018 Loschmidt constant at 101.325 kPa `n0_atm` is an exact `Ratio` `67550000000000000000000000000000000/2514161829` m⁻³ from the same section at 273.15 K; CODATA 2018 Sackur-Tetrode constant `S0_R` is a one-sigma `Interval` `-1.15170753706(45)` from the same section at T1 = 1 K and p0 = 100 kPa; CODATA 2018 Sackur-Tetrode constant at 101.325 kPa `S0_R_atm` is a one-sigma `Interval` `-1.16487052358(45)` from the same section at T1 = 1 K; CODATA 2018 first radiation constant for spectral radiance `c1L` is SI-exact `SciExact` `11910429723971884140794892e-41` (not a `Ratio`: `i128` denominator overflow; Stefan-Boltzmann cites π and is not stored); CODATA 2018 second radiation constant `c2` is an exact `Ratio` `272115870842319/18913000000000000` m K from the same section (not a terminating `SciExact`: 18913 remains in the denominator; first radiation constant `c1` cites π and is not stored); CODATA 2018 Josephson constant `KJ` is an exact `Ratio` `21362355120000000000000/44173801` Hz V⁻¹ from JPCRD table XXXI ELECTROMAGNETIC (not a terminating `SciExact`: 7 and 6310543 remain in the denominator; magnetic flux quantum `Phi0` and conductance quantum `G0` cite π and ħ and are not stored); CODATA 2018 von Klitzing constant `RK` is an exact `Ratio` `5521725125000000000000/213914163877964163` ohm from the same section (not a terminating `SciExact`: 3, 19, 389, and 12043 remain in the denominator; JPCRD also writes `2πℏ/e²` and that printed formula is not stored); CODATA 2018 Bohr magneton `muB` is a one-sigma `Interval` `9.2740100783(28)×10^{-24}` J T⁻¹ from the same section (not an SI defining `Ratio`; not electron magnetic moment `mu_e`; not a FormalClaim of `eℏ/2me`); CODATA 2018 Bohr magneton in eV/T `muB_eV` is a one-sigma `Interval` `5.7883818060(17)×10^{-5}` eV T⁻¹ from the same section (not an SI defining `Ratio`; not `muB`; not a FormalClaim of `muB/e`); CODATA 2018 Bohr magneton in Hz/T `muB_Hz` is a one-sigma `Interval` `1.39962449361(42)×10^{10}` Hz T⁻¹ from the same section (not an SI defining `Ratio`; not `muB`; not a FormalClaim of `muB/h`); CODATA 2018 Bohr magneton in inverse meter per tesla `muB_m` is a one-sigma `Interval` `46.686447783(14)` m⁻¹ T⁻¹ from the same section (not an SI defining `Ratio`; not `muB`; not a FormalClaim of `muB/hc`); CODATA 2018 Bohr magneton in K/T `muB_K` is a one-sigma `Interval` `0.67171381563(20)` K T⁻¹ from the same section (not an SI defining `Ratio`; not `muB`; not a FormalClaim of `muB/k`); CODATA 2018 nuclear magneton `muN` is a one-sigma `Interval` `5.0507837461(15)×10^{-27}` J T⁻¹ from the same section (not an SI defining `Ratio`; not `muB`; not neutron magnetic moment `mu_n`; not a FormalClaim of `eℏ/2mp`); CODATA 2018 nuclear magneton in eV/T `muN_eV` is a one-sigma `Interval` `3.15245125844(96)×10^{-8}` eV T⁻¹ from the same section (not an SI defining `Ratio`; not `muN`; not `muB_eV`; not a FormalClaim of `muN/e`; nuclear magneton in inverse meter per tesla is a later ELECTROMAGNETIC row); electron mass is not stored (`10^{42}` overflows `i128`). IAU 2012 `au` is an exact `Ratio` `149597870700` m (BIPM table 8). The electronvolt is an exact `Ratio` `1.602176634×10^{-19}` J from the same table (SI 2019, same decimal as `e`, unit joule). The parsec is `(648000/π) au` and is not a Ratio. IAU 2015 `(GM)_☉^N` is an exact `Ratio` `1.3271244×10^20` m³ s⁻² (AJ 152, 41 table 1): a conversion ruler, not a measured solar mass. IAU 2015 `R_☉^N` is an exact `Ratio` `695700000` m from the same table: a conversion ruler, not a measured photospheric radius. IAU 2015 `L_☉^N` is an exact `Ratio` `3.828×10^26` W from the same table: a conversion ruler, not a measured solar luminosity. `physis constant [name]` independently rebuilds those hashes (`provenance-auditor`; not P3N). Omitted name rebuilds the full LEDGER into one VersionedConstant bundle. Overlapping `physis_model` Qty floats lockstep the ledger: `c`, `au`, `p0`, `atm`, `GM_sun`, `R_sun`, and `L_sun` via integer `to_f64`; `e`/`k`/`eV`/`NAh`/`NAk`/`NAe`/`Vm` via IEEE rounding of the SI decimal (`SciExact::to_f64`, not reduced `Ratio::to_f64`); `n0`/`Vm_atm`/`n0_atm`/`c2`/`KJ`/`RK` via IEEE rounding of the exact `Ratio`; `h`/`c1L` via `SciExact::to_f64`; `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `S0_R`, `S0_R_atm`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`, `muN`, and `muN_eV` Qty values are the CODATA centres inside the hulls. Theories still evaluate with `f64` Qty |

Journal events are hash-linked in memory (`Journal::tip`). Journal
restore of a `prove` event remints through `verify` (never Deserialize)
only when the recorded `challenge_hash` is `Challenge::generate` of the
live FormalClaim (and, when present, the recorded `statement_hash`
matches). A matching slug with a different identity is not that prove.
Lean kernel + nanoda when the pipeline is wired, otherwise the exact
dual expanders. `physis prove` uses the same preference. Restore of
`review` remints only when the recorded `statement_hash` is the live
identity; a slug-only review line is not P3S.

## What is not yet true

- Versioned constants: SI 2019 defining `c`, `Δν_Cs`, `e`, `k`, `N_A`,
  `K_cd` are exact `Ratio`. Planck `h` is SI-exact `SciExact`
  `662607015e-42` (not a `Ratio`). `ħ` is not a terminating decimal.
  CODATA 2018 `G` is a one-sigma `Interval`.   CODATA 2018 `μ₀` is a
  one-sigma `Interval` `1.25663706212(19)×10^{-6}` N A⁻² (JPCRD table
  XXXI UNIVERSAL). CODATA 2018 `ε₀` is a one-sigma `Interval`
  `8.8541878128(13)×10^{-12}` F m⁻¹ (JPCRD table XXXI UNIVERSAL).
  CODATA 2018 `Z₀` is a one-sigma `Interval` `376.730313668(57)` ohm
  (JPCRD table XXXI UNIVERSAL); `Y₀` is not stored.   CODATA 2018 `α` is a
  one-sigma `Interval` `7.2973525693(11)×10^{-3}` (JPCRD table XXXI
  ATOMIC AND NUCLEAR). CODATA 2018 inverse-α is a one-sigma `Interval`
  `137.035999084(21)` from the same table. CODATA 2018 Rydberg frequency
  `cR∞` is a one-sigma `Interval` `3.2898419602508(64)×10^{15}` Hz from
  the same table. CODATA 2018 Rydberg energy equivalent `hcR∞` is a
  one-sigma `Interval` `2.1798723611035(42)×10^{-18}` J from the same
  table; the eV conversion is not stored. CODATA 2018
  Rydberg `R∞` is a
  one-sigma `Interval` `10973731.568160(21)` m⁻¹ from the same table.
  CODATA 2018 Bohr radius `a₀` is a one-sigma `Interval`
  `5.29177210903(80)×10^{-11}` m from the same table. CODATA 2018
  Hartree energy `E_h` is a one-sigma `Interval`
  `4.3597447222071(85)×10^{-18}` J from the same table; the eV
  conversion is not stored. CODATA 2018 electron-muon mass ratio
  `m_e/m_μ` is a one-sigma `Interval` `4.83633169(11)×10^{-3}` (JPCRD
  table XXXI Electron, e-). CODATA 2018 electron-proton mass ratio
  `m_e/m_p` is a one-sigma `Interval` `5.44617021487(33)×10^{-4}` from
  the same section. CODATA 2018 electron-neutron mass ratio
  `m_e/m_n` is a one-sigma `Interval` `5.4386734424(26)×10^{-4}` from
  the same section. CODATA 2018 electron-deuteron mass ratio
  `m_e/m_d` is a one-sigma `Interval` `2.724437107462(96)×10^{-4}` from
  the same section. CODATA 2018 electron-triton mass ratio
  `m_e/m_t` is a one-sigma `Interval` `1.819200062251(90)×10^{-4}` from
  the same section. CODATA 2018 electron-helion mass ratio
  `m_e/m_h` is a one-sigma `Interval` `1.819543074573(79)×10^{-4}` from
  the same section. CODATA 2018 electron to alpha particle mass ratio
  `m_e/m_α` is a one-sigma `Interval` `1.370933554787(45)×10^{-4}` from
  the same section. CODATA 2018 electron charge to mass quotient
  `−e/m_e` is a one-sigma `Interval` `−1.75882001076(53)×10^{11}`
  C kg⁻¹ from the same section. CODATA 2018 electron molar mass
  `M_e` is a one-sigma `Interval` `5.4857990888(17)×10^{-7}`
  kg mol⁻¹ from the same section. CODATA 2018 reduced Compton
  wavelength `ƛ_C` is a one-sigma `Interval` `3.8615926796(12)×10^{-13}`
  m from the same section. CODATA 2018 Compton wavelength `λ_C` is a
  one-sigma `Interval` `2.42631023867(73)×10^{-12}` m from the same
  section. CODATA 2018 classical electron radius `r_e` is a one-sigma
  `Interval` `2.8179403262(13)×10^{-15}` m from the same section.
  CODATA 2018 electron magnetic moment `μ_e` is a one-sigma
  `Interval` `−9.2847647043(28)×10^{-24}` J T⁻¹ from the same section.
  CODATA 2018 electron magnetic moment to Bohr magneton ratio `μ_e/μ_B`
  is a one-sigma `Interval` `−1.00115965218128(18)` from the same section.
  CODATA 2018 electron magnetic moment to nuclear magneton ratio `μ_e/μ_N`
  is a one-sigma `Interval` `−1838.28197188(11)` from the same section.
  CODATA 2018 electron magnetic-moment anomaly `ae` is a one-sigma
  `Interval` `1.15965218128(18)×10^{-3}` from the same section.
  CODATA 2018 electron g-factor `ge` is a one-sigma
  `Interval` `−2.00231930436256(35)` from the same section.
  CODATA 2018 electron-muon magnetic-moment ratio `mu_e_mmu` is a
  one-sigma `Interval` `206.7669883(46)` from the same section.
  CODATA 2018 electron-proton magnetic-moment ratio `mu_e_mup` is a
  one-sigma `Interval` `−658.21068789(20)` from the same section.
  CODATA 2018 electron to shielded-proton magnetic-moment ratio `mu_e_mu0p` is a
  one-sigma `Interval` `−658.2275971(72)` from the same section.
  CODATA 2018 electron-neutron magnetic-moment ratio `mu_e_mun` is a
  one-sigma `Interval` `960.92050(23)` from the same section.
  CODATA 2018 electron-deuteron magnetic-moment ratio `mu_e_mud` is a
  one-sigma `Interval` `−2143.9234915(56)` from the same section.
  CODATA 2018 electron to shielded-helion magnetic-moment ratio `mu_e_mu0h` is a
  one-sigma `Interval` `864.058257(10)` from the same section.
  CODATA 2018 muon mass `m_mu` is a one-sigma `Interval`
  `1.883531627(42)×10^{-28}` kg (JPCRD table XXXI Muon, mu-).
  CODATA 2018 muon mass in u `m_mu_u` is a one-sigma `Interval`
  `0.1134289259(25)` u from the same section.
  CODATA 2018 muon mass energy equivalent `m_mu_c2` is a one-sigma `Interval`
  `1.692833804(38)×10^{-11}` J from the same section.
  CODATA 2018 muon mass energy equivalent in MeV `m_mu_c2_MeV` is a one-sigma `Interval`
  `105.6583755(23)` MeV from the same section.
  CODATA 2018 muon-electron mass ratio `mmu_me` is a one-sigma `Interval`
  `206.7682830(46)` from the same section.
  The muon-tau mass ratio is a PDG reprint of `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored.
  CODATA 2018 muon-proton mass ratio `mmu_mp` is a one-sigma `Interval`
  `0.1126095264(25)` from the same section.
  CODATA 2018 muon-neutron mass ratio `mmu_mn` is a one-sigma `Interval`
  `0.1124545170(25)` from the same section.
  CODATA 2018 muon molar mass `M_mu` is a one-sigma `Interval`
  `1.134289259(25)×10^{-4}` kg mol⁻¹ from the same section.
  The reduced muon Compton wavelength is ħ/m_μc and is not stored.
  CODATA 2018 muon Compton wavelength `lambda_C_mu` is a one-sigma `Interval`
  `1.173444110(26)×10^{-14}` m from the same section.
  CODATA 2018 muon magnetic moment `mu_mu` is a one-sigma `Interval`
  `−4.49044830(10)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 muon magnetic moment to Bohr magneton ratio `mu_mu_muB`
  is a one-sigma `Interval` `−4.84197047(11)×10^{-3}` from the same
  section.
  CODATA 2018 muon magnetic moment to nuclear magneton ratio `mu_mu_muN`
  is a one-sigma `Interval` `−8.89059703(20)` from the same section.
  CODATA 2018 muon magnetic-moment anomaly `amu` is a one-sigma `Interval`
  `1.16592089(63)×10^{-3}` from the same section.
  CODATA 2018 muon g-factor `gmu` is a one-sigma `Interval`
  `−2.0023318418(13)` from the same section.
  CODATA 2018 muon-proton magnetic-moment ratio `mu_mu_mup` is a one-sigma `Interval`
  `−3.183345142(71)` from the same section.
  CODATA 2018 `m_p` is a
  one-sigma `Interval` `1.67262192369(51)×10^{-27}` kg (JPCRD table XXXI
  Proton, p).
  CODATA 2018 proton mass in u `m_p_u` is a one-sigma `Interval`
  `1.007276466621(53)` u from the same section.
  CODATA 2018 proton mass energy equivalent `m_p_c2` is a one-sigma `Interval`
  `1.50327761598(46)×10^{-10}` J from the same section.
  CODATA 2018 proton mass energy equivalent in MeV `m_p_c2_MeV` is a one-sigma `Interval`
  `938.27208816(29)` MeV from the same section.
  CODATA 2018 proton-electron mass ratio `mp_me` is a one-sigma `Interval`
  `1836.15267343(11)` from the same section.
  CODATA 2018 proton-muon mass ratio `mp_mmu` is a one-sigma `Interval`
  `8.88024337(20)` from the same section.
  The proton-tau mass ratio is a PDG reprint of `m_tau c^2` (JPCRD table XXXI footnote e) and is not stored.
  CODATA 2018 proton-neutron mass ratio `mp_mn` is a one-sigma `Interval`
  `0.99862347812(49)` from the same section.
  CODATA 2018 proton charge-to-mass quotient `e_mp` is a one-sigma `Interval`
  `9.5788331560(29)×10^{7}` C kg⁻¹ from the same section.
  CODATA 2018 proton molar mass `M_p` is a one-sigma `Interval`
  `1.00727646627(31)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 proton Compton wavelength `lambda_C_p` is a one-sigma `Interval`
  `1.32140985539(40)×10^{-15}` m from the same section.
  CODATA 2018 proton rms charge radius `rp` is a one-sigma `Interval`
  `8.414(19)×10^{-16}` m from the same section.
  CODATA 2018 proton magnetic moment `mu_p` is a one-sigma `Interval`
  `1.41060679736(60)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 proton magnetic moment to Bohr magneton ratio `mu_p_muB` is a one-sigma `Interval`
  `1.52103220230(46)×10^{-3}` from the same section.
  CODATA 2018 proton magnetic moment to nuclear magneton ratio `mu_p_muN` is a one-sigma `Interval`
  `2.79284734463(82)` from the same section.
  CODATA 2018 proton g-factor `gp` is a one-sigma `Interval`
  `5.5856946893(16)` from the same section.
  CODATA 2018 proton-neutron magnetic-moment ratio `mu_p_mun` is a one-sigma `Interval`
  `−1.45989805(34)` from the same section.
  CODATA 2018 shielded proton magnetic moment `mu0p` is a one-sigma `Interval`
  `1.410570560(15)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 shielded proton magnetic moment to Bohr magneton ratio `mu0p_muB` is a one-sigma `Interval`
  `1.520993128(17)×10^{-3}` from the same section.
  CODATA 2018 shielded proton magnetic moment to nuclear magneton ratio `mu0p_muN` is a one-sigma `Interval`
  `2.792775599(30)` from the same section.
  CODATA 2018 proton magnetic shielding correction `sigma0p` is a one-sigma `Interval`
  `2.5689(11)×10^{-5}` from the same section.
  CODATA 2018 neutron mass `m_n` is a one-sigma `Interval`
  `1.67492749804(95)×10^{-27}` kg (JPCRD table XXXI Neutron, n).
  CODATA 2018 neutron mass in u `m_n_u` is a one-sigma `Interval`
  `1.00866491595(49)` u from the same section.
  CODATA 2018 neutron mass energy equivalent `m_n_c2` is a one-sigma `Interval`
  `1.50534976287(86)×10^{-10}` J from the same section.
  CODATA 2018 neutron mass energy equivalent in MeV `m_n_c2_MeV` is a one-sigma `Interval`
  `939.56542052(54)` MeV from the same section.
  CODATA 2018 neutron-electron mass ratio `mn_me` is a one-sigma `Interval`
  `1838.68366173(89)` from the same section.
  CODATA 2018 neutron-muon mass ratio `mn_mmu` is a one-sigma `Interval`
  `8.89248406(20)` from the same section.
  CODATA 2018 neutron-proton mass ratio `mn_mp` is a one-sigma `Interval`
  `1.00137841931(49)` from the same section.
  CODATA 2018 neutron-proton mass difference `mn_minus_mp` is a one-sigma `Interval`
  `2.30557435(82)×10^{-30}` kg from the same section.
  CODATA 2018 neutron-proton mass difference in u `mn_minus_mp_u` is a one-sigma `Interval`
  `1.38844933(49)×10^{-3}` u from the same section.
  CODATA 2018 neutron-proton mass difference energy equivalent `mn_minus_mp_c2` is a one-sigma `Interval`
  `2.07214689(74)×10^{-13}` J from the same section.
  CODATA 2018 neutron-proton mass difference energy equivalent in MeV `mn_minus_mp_c2_MeV` is a one-sigma `Interval`
  `1.29333236(46)` MeV from the same section.
  CODATA 2018 neutron molar mass `M_n` is a one-sigma `Interval`
  `1.00866491560(57)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 neutron Compton wavelength `lambda_C_n` is a one-sigma `Interval`
  `1.31959090581(75)×10^{-15}` m from the same section.
  CODATA 2018 neutron magnetic moment `mu_n` is a one-sigma `Interval`
  `−9.6623651(23)×10^{-27}` J T⁻¹ from the same section.
  CODATA 2018 neutron magnetic moment to Bohr magneton ratio `mu_n_muB` is a one-sigma `Interval`
  `−1.04187563(25)×10^{-3}` from the same section.
  CODATA 2018 neutron magnetic moment to nuclear magneton ratio `mu_n_muN` is a one-sigma `Interval`
  `−1.91304273(45)` from the same section.
  CODATA 2018 neutron g-factor `gn` is a one-sigma `Interval`
  `−3.82608545(90)` from the same section.
  CODATA 2018 neutron-electron magnetic-moment ratio `mu_n_mue` is a one-sigma `Interval`
  `1.04066882(25)×10^{-3}` from the same section.
  CODATA 2018 neutron-proton magnetic-moment ratio `mu_n_mup` is a one-sigma `Interval`
  `−0.68497934(16)` from the same section.
  CODATA 2018 neutron to shielded-proton magnetic-moment ratio `mu_n_mu0p` is a one-sigma `Interval`
  `−0.68499694(16)` from the same section.
  CODATA 2018 deuteron mass `m_d` is a one-sigma `Interval`
  `3.3435837724(10)×10^{-27}` kg (JPCRD table XXXI Deuteron, d).
  CODATA 2018 deuteron mass in u `m_d_u` is a one-sigma `Interval`
  `2.013553212745(40)` u from the same section.
  CODATA 2018 deuteron mass energy equivalent `m_d_c2` is a one-sigma `Interval`
  `3.00506323102(91)×10^{-10}` J from the same section.
  CODATA 2018 deuteron mass energy equivalent in MeV `m_d_c2_MeV` is a one-sigma `Interval`
  `1875.61294257(57)` MeV from the same section.
  CODATA 2018 deuteron-electron mass ratio `md_me` is a one-sigma `Interval`
  `3670.48296788(13)` from the same section.
  CODATA 2018 deuteron-proton mass ratio `md_mp` is a one-sigma `Interval`
  `1.99900750139(11)` from the same section.
  CODATA 2018 deuteron molar mass `M_d` is a one-sigma `Interval`
  `2.01355321205(61)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 deuteron rms charge radius `rd` is a one-sigma `Interval`
  `2.12799(74)×10^{-15}` m from the same section.
  CODATA 2018 deuteron magnetic moment `mu_d` is a one-sigma `Interval`
  `4.330735094(11)×10^{-27}` J T⁻¹ from the same section.
  CODATA 2018 deuteron magnetic moment to Bohr magneton ratio `mu_d_muB` is a one-sigma `Interval`
  `4.669754570(12)×10^{-4}` from the same section.
  CODATA 2018 deuteron magnetic moment to nuclear magneton ratio `mu_d_muN` is a one-sigma `Interval`
  `0.8574382338(22)` from the same section.
  CODATA 2018 deuteron g-factor `gd` is a one-sigma `Interval`
  `0.8574382338(22)` from the same section.
  CODATA 2018 deuteron-electron magnetic-moment ratio `mu_d_mue` is a one-sigma `Interval`
  `−4.664345551(12)×10^{-4}` from the same section.
  CODATA 2018 deuteron-proton magnetic-moment ratio `mu_d_mup` is a one-sigma `Interval`
  `0.30701220939(79)` from the same section.
  CODATA 2018 deuteron-neutron magnetic-moment ratio `mu_d_mun` is a one-sigma `Interval`
  `−0.44820653(11)` from the same section.
  CODATA 2018 triton mass `m_t` is a one-sigma `Interval`
  `5.0073567446(15)×10^{-27}` kg from JPCRD table XXXI Triton, t.
  CODATA 2018 triton mass in u `m_t_u` is a one-sigma `Interval`
  `3.01550071621(12)` u from the same section.
  CODATA 2018 triton mass energy equivalent `m_t_c2` is a one-sigma `Interval`
  `4.5003878060(14)×10^{-10}` J from the same section.
  CODATA 2018 triton mass energy equivalent in MeV `m_t_c2_MeV` is a one-sigma `Interval`
  `2808.92113298(85)` MeV from the same section.
  CODATA 2018 triton-electron mass ratio `mt_me` is a one-sigma `Interval`
  `5496.92153573(27)` from the same section.
  CODATA 2018 triton-proton mass ratio `mt_mp` is a one-sigma `Interval`
  `2.99371703414(15)` from the same section.
  CODATA 2018 triton molar mass `M_t` is a one-sigma `Interval`
  `3.01550071517(92)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 triton magnetic moment `mu_t` is a one-sigma `Interval`
  `1.5046095202(30)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 triton magnetic moment to Bohr magneton ratio `mu_t_muB` is a one-sigma `Interval`
  `1.6223936651(32)×10^{-3}` from the same section.
  CODATA 2018 triton magnetic moment to nuclear magneton ratio `mu_t_muN` is a one-sigma `Interval`
  `2.9789624656(59)` from the same section.
  CODATA 2018 triton g-factor `gt` is a one-sigma `Interval`
  `5.957924931(12)` from the same section.
  CODATA 2018 helion mass `m_h` is a one-sigma `Interval`
  `5.0064127796(15)×10^{-27}` kg from JPCRD table XXXI Helion, h.
  CODATA 2018 helion mass in u `m_h_u` is a one-sigma `Interval`
  `3.014932247175(97)` u from the same section.
  CODATA 2018 helion mass energy equivalent `m_h_c2` is a one-sigma `Interval`
  `4.4995394125(14)×10^{-10}` J from the same section.
  CODATA 2018 helion mass energy equivalent in MeV `m_h_c2_MeV` is a one-sigma `Interval`
  `2808.39160743(85)` MeV from the same section.
  CODATA 2018 helion-electron mass ratio `mh_me` is a one-sigma `Interval`
  `5495.88528007(24)` from the same section.
  CODATA 2018 helion-proton mass ratio `mh_mp` is a one-sigma `Interval`
  `2.99315267167(13)` from the same section.
  CODATA 2018 helion molar mass `M_h` is a one-sigma `Interval`
  `3.01493224613(91)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 helion magnetic moment `mu_h` is a one-sigma `Interval`
  `−1.074617532(13)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 helion magnetic moment to Bohr magneton ratio `mu_h_muB` is a one-sigma `Interval`
  `−1.158740958(14)×10^{-3}` from the same section.
  CODATA 2018 helion magnetic moment to nuclear magneton ratio `mu_h_muN` is a one-sigma `Interval`
  `−2.127625307(25)` from the same section.
  CODATA 2018 helion g-factor `gh` is a one-sigma `Interval`
  `−4.255250615(50)` from the same section.
  CODATA 2018 shielded helion magnetic moment `mu0h` is a one-sigma `Interval`
  `−1.074553090(13)×10^{-26}` J T⁻¹ from the same section.
  CODATA 2018 shielded helion mag. mom. to Bohr magneton ratio `mu0h_muB` is a one-sigma `Interval`
  `−1.158671471(14)×10^{-3}` from the same section.
  CODATA 2018 shielded helion mag. mom. to nuclear magneton ratio `mu0h_muN` is a one-sigma `Interval`
  `−2.127497719(25)` from the same section.
  CODATA 2018 shielded helion to proton mag. mom. ratio `mu0h_mup` is a one-sigma `Interval`
  `−0.7617665618(89)` from the same section.
  CODATA 2018 shielded helion to shielded proton mag. mom. ratio `mu0h_mu0p` is a one-sigma `Interval`
  `−0.7617861313(33)` from the same section.
  CODATA 2018 alpha particle mass `m_alpha` is a one-sigma `Interval`
  `6.6446573357(20)×10^{-27}` kg from JPCRD table XXXI Alpha particle, a.
  CODATA 2018 alpha particle mass in u `m_alpha_u` is a one-sigma `Interval`
  `4.001506179127(63)` u from the same section.
  CODATA 2018 alpha particle mass energy equivalent `m_alpha_c2` is a one-sigma `Interval`
  `5.9719201914(18)×10^{-10}` J from the same section.
  CODATA 2018 alpha particle mass energy equivalent in MeV `m_alpha_c2_MeV` is a one-sigma `Interval`
  `3727.3794066(11)` MeV from the same section.
  CODATA 2018 alpha particle-electron mass ratio `malpha_me` is a one-sigma `Interval`
  `7294.29954142(24)` from the same section.
  CODATA 2018 alpha particle-proton mass ratio `malpha_mp` is a one-sigma `Interval`
  `3.97259969009(22)` from the same section.
  CODATA 2018 alpha particle molar mass `M_alpha` is a one-sigma `Interval`
  `4.0015061777(12)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 atomic mass constant `m_u` is a one-sigma `Interval`
  `1.66053906660(50)×10^{-27}` kg from JPCRD table XXXI PHYSICOCHEMICAL.
  CODATA 2018 atomic mass constant energy equivalent `m_u_c2` is a one-sigma `Interval`
  `1.49241808560(45)×10^{-10}` J from the same section.
  CODATA 2018 atomic mass constant energy equivalent in MeV `m_u_c2_MeV` is a one-sigma `Interval`
  `931.49410242(28)` MeV from the same section.
  CODATA 2018 molar mass constant `M_u` is a one-sigma `Interval`
  `0.99999999965(30)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 molar mass of carbon-12 `M_12C` is a one-sigma `Interval`
  `11.9999999958(36)×10^{-3}` kg mol⁻¹ from the same section.
  CODATA 2018 molar Planck constant `NAh` is an exact `Ratio`
  `3.99031271289343140×10^{-10}` J Hz⁻¹ mol⁻¹ from the same section.
  CODATA 2018 molar gas constant `NAk` is an exact `Ratio`
  `8.31446261815324` J mol⁻¹ K⁻¹ from the same section.
  CODATA 2018 Faraday constant `NAe` is an exact `Ratio`
  `96485.3321233100184` C mol⁻¹ from the same section.
  CODATA 2018 standard-state pressure `p0` is an exact `Ratio`
  `100000` Pa from the same section.
  CODATA 2018 standard atmosphere `atm` is an exact `Ratio`
  `101325` Pa from the same section.
  CODATA 2018 molar volume of ideal gas `Vm` is an exact `Ratio`
  `0.022710954641485575` m³ mol⁻¹ from the same section at 273.15 K and 100 kPa.
  CODATA 2018 Loschmidt constant `n0` is an exact `Ratio`
  `200000000000000000000000000000000000/7542485487` m⁻³ from the same section at 273.15 K and 100 kPa.
  CODATA 2018 molar volume of ideal gas at 101.325 kPa `Vm_atm` is an exact `Ratio`
  `378515910691426251/16887500000000000000` m³ mol⁻¹ from the same section at 273.15 K.
  CODATA 2018 Loschmidt constant at 101.325 kPa `n0_atm` is an exact `Ratio`
  `67550000000000000000000000000000000/2514161829` m⁻³ from the same section at 273.15 K.
  CODATA 2018 Sackur-Tetrode constant `S0_R` is a one-sigma `Interval`
  `-1.15170753706(45)` from the same section at T1 = 1 K and p0 = 100 kPa.
  CODATA 2018 Sackur-Tetrode constant at 101.325 kPa `S0_R_atm` is a one-sigma `Interval`
  `-1.16487052358(45)` from the same section at T1 = 1 K.
  CODATA 2018 first radiation constant for spectral radiance `c1L` is SI-exact
  `SciExact` `11910429723971884140794892e-41` (not a `Ratio`: `i128` denominator overflow;
  Stefan-Boltzmann cites π and is not stored).
  CODATA 2018 second radiation constant `c2` is an exact `Ratio`
  `272115870842319/18913000000000000` m K from the same section (not a terminating
  `SciExact`: 18913 remains in the denominator; first radiation constant `c1` cites π
  and is not stored).
  CODATA 2018 Josephson constant `KJ` is an exact `Ratio`
  `21362355120000000000000/44173801` Hz V⁻¹ from JPCRD table XXXI ELECTROMAGNETIC (not a
  terminating `SciExact`: 7 and 6310543 remain in the denominator; magnetic flux quantum
  `Phi0` and conductance quantum `G0` cite π and ħ and are not stored).
  CODATA 2018 von Klitzing constant `RK` is an exact `Ratio`
  `5521725125000000000000/213914163877964163` ohm from the same section (not a terminating
  `SciExact`: 3, 19, 389, and 12043 remain in the denominator; JPCRD also writes `2πℏ/e²`
  and that printed formula is not stored).
  CODATA 2018 Bohr magneton `muB` is a one-sigma `Interval`
  `9.2740100783(28)×10^{-24}` J T⁻¹ from the same section (not an SI defining
  `Ratio`; not electron magnetic moment `mu_e`; not a FormalClaim of `eℏ/2me`).
  CODATA 2018 Bohr magneton in eV/T `muB_eV` is a one-sigma `Interval`
  `5.7883818060(17)×10^{-5}` eV T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muB`; not a FormalClaim of `muB/e`).
  CODATA 2018 Bohr magneton in Hz/T `muB_Hz` is a one-sigma `Interval`
  `1.39962449361(42)×10^{10}` Hz T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muB`; not a FormalClaim of `muB/h`).
  CODATA 2018 Bohr magneton in inverse meter per tesla `muB_m` is a one-sigma `Interval`
  `46.686447783(14)` m⁻¹ T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muB`; not a FormalClaim of `muB/hc`).
  CODATA 2018 Bohr magneton in K/T `muB_K` is a one-sigma `Interval`
  `0.67171381563(20)` K T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muB`; not a FormalClaim of `muB/k`).
  CODATA 2018 nuclear magneton `muN` is a one-sigma `Interval`
  `5.0507837461(15)×10^{-27}` J T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muB`; not neutron magnetic moment `mu_n`; not a FormalClaim of
  `eℏ/2mp`).
  CODATA 2018 nuclear magneton in eV/T `muN_eV` is a one-sigma `Interval`
  `3.15245125844(96)×10^{-8}` eV T⁻¹ from the same section (not an SI defining
  `Ratio`; not `muN`; not `muB_eV`; not a FormalClaim of `muN/e`; nuclear magneton
  in inverse meter per tesla is a later ELECTROMAGNETIC row).
  Electron mass is not stored (`10^{42}` overflows `i128`). IAU 2012 `au` is an exact
  `Ratio` `149597870700` m (BIPM table 8). The electronvolt is an exact
  `Ratio` `1.602176634×10^{-19}` J from the same table (SI 2019, same
  decimal as `e`, unit joule). The parsec is
  `(648000/π) au` and is not a Ratio. IAU 2015 `(GM)_☉^N` is an exact
  `Ratio` `1.3271244×10^20` m³ s⁻² (AJ 152, 41 table 1): a conversion
  ruler, not a measured solar mass. IAU 2015 `R_☉^N` is an exact
  `Ratio` `695700000` m from the same table: a conversion ruler, not a
  measured photospheric radius. IAU 2015 `L_☉^N` is an exact
  `Ratio` `3.828×10^26` W from the same table: a conversion ruler, not a
  measured solar luminosity. `physis constant [name]`
  rebuilds those hashes independently; omitted name rebuilds the full
  LEDGER. Overlapping `physis_model` Qty floats lockstep the ledger:
  `c`, `au`, `p0`, `atm`, `GM_sun`, `R_sun`, and `L_sun` via integer `to_f64`; `e`/`k`/`eV`/`NAh`/`NAk`/`NAe`/`Vm` via IEEE rounding of the SI
  decimal (`SciExact::to_f64`, not reduced `Ratio::to_f64`); `n0`/`Vm_atm`/`n0_atm`/`c2`/`KJ`/`RK` via IEEE rounding of the exact `Ratio`; `h`/`c1L` via
  `SciExact::to_f64`; `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, `mu_e_mup`, `mu_e_mu0p`, `mu_e_mun`, `mu_e_mud`, `mu_e_mu0h`, `m_mu`, `m_mu_u`, `m_mu_c2`, `m_mu_c2_MeV`, `mmu_me`, `mmu_mp`, `mmu_mn`, `M_mu`, `lambda_C_mu`, `mu_mu`, `mu_mu_muB`, `mu_mu_muN`, `amu`, `gmu`, `mu_mu_mup`, `m_p`, `m_p_u`, `m_p_c2`, `m_p_c2_MeV`, `mp_me`, `mp_mmu`, `mp_mn`, `e_mp`, `M_p`, `lambda_C_p`, `rp`, `mu_p`, `mu_p_muB`, `mu_p_muN`, `gp`, `mu_p_mun`, `mu0p`, `mu0p_muB`, `mu0p_muN`, `sigma0p`, `m_n`, `m_n_u`, `m_n_c2`, `m_n_c2_MeV`, `mn_me`, `mn_mmu`, `mn_mp`, `mn_minus_mp`, `mn_minus_mp_u`, `mn_minus_mp_c2`, `mn_minus_mp_c2_MeV`, `M_n`, `lambda_C_n`, `mu_n`, `mu_n_muB`, `mu_n_muN`, `gn`, `mu_n_mue`, `mu_n_mup`, `mu_n_mu0p`, `m_d`, `m_d_u`, `m_d_c2`, `m_d_c2_MeV`, `md_me`, `md_mp`, `M_d`, `rd`, `mu_d`, `mu_d_muB`, `mu_d_muN`, `gd`, `mu_d_mue`, `mu_d_mup`, `mu_d_mun`, `m_t`, `m_t_u`, `m_t_c2`, `m_t_c2_MeV`, `mt_me`, `mt_mp`, `M_t`, `mu_t`, `mu_t_muB`, `mu_t_muN`, `gt`, `m_h`, `m_h_u`, `m_h_c2`, `m_h_c2_MeV`, `mh_me`, `mh_mp`, `M_h`, `mu_h`, `mu_h_muB`, `mu_h_muN`, `gh`, `mu0h`, `mu0h_muB`, `mu0h_muN`, `mu0h_mup`, `mu0h_mu0p`, `m_alpha`, `m_alpha_u`, `m_alpha_c2`, `m_alpha_c2_MeV`, `malpha_me`, `malpha_mp`, `M_alpha`, `m_u`, `m_u_c2`, `m_u_c2_MeV`, `M_u`, `M_12C`, `S0_R`, `S0_R_atm`, `muB`, `muB_eV`, `muB_Hz`, `muB_m`, `muB_K`, `muN`, and `muN_eV` Qty values are the CODATA centres
  inside the hulls.
  Theories still evaluate with `physis_model` `f64` Qty constants
- Community-canonical encodings (review tops out at
  `AdversariallyReviewed`; there is no `Canonical` variant to assign)
- P4 independent reproduction (in-process `reproduce` remints and
  **refuses** to assign P4; a distinct implementation is still required)
- Mathlib-scale Physlib; four catalog identities are kernel-checked
  (`d²`, interval, Einstein composition, mass shell). The mass-shell
  polynomial is the interval identity on 4-momentum, not a fifth
  algebraic idea. That is not Mathlib.
- Most other claims still use the encoding-wide domain placeholder
  and only `encoding-is-the-model`. Catalog identities name regimes and
  catalog axioms. `field.second-order-accurate` names `|k a| < 1`.
  GUT mixing-angle and SM P3N cells name unification-scale / `M_Z` /
  one generation / hydrogen. Super-K `p→e+π0` names dim-6 / Super-K 90% CL;
  GUT `Tr Q` stays encoding-wide. `predictivity.unique-vacuum` encodings
  name landscape / program axiom / Einstein–Hilbert / Higgs vacuum and
  stay `Asserted`.
  `dec.hodge-harmonic` names discrete combinatorial Hodge; Euler–Poincaré
  and Poincaré stay encoding-wide. `field.dispersion-continuum-limit` names
  the longest lattice mode; ohm-circuit `em.quasi-static-valid` names
  `λ > 100 ×` circuit size. Maxwell's copy of that slug stays encoding-wide.
  `thermo.high-t-classical` names `T/Θ ≥ 8`; `thermo.debye-t3` names the
  `Θ/20` phonon probe; `thermo.rj-ir-limit` names `hν = 0.01 kT`;
  `gauge.exact-area-law-2d` names 2D plaquette factorization;
  `gauge.local` names nearest-neighbour Wilson plaquettes.
  On `dulong-petit`, `thermo.dulong-petit` names harmonic `U = 3 N k T`;
  on `einstein-solid` and `debye-solid` it stays encoding-wide.
  On `heterotic-e8e8`, `consistency.anomaly-cancellation` names complete
  `E8 x E8`; on `heterotic-so32` it names complete `SO(32)`;
  on `type-i` it names complete `Chan-Paton SO(32)`;
  Type II copies stay encoding-wide.
- Trust tiers do not gate observation or standalone encoding-review.
  They now refuse `reproduce` and the loop's review step without P3F.
  Named *roles* still gate who may issue an op. A proof-searcher cannot
  remint; that is `replication-agent`. An explorer cannot score; that
  is `empirical-analyst`. A proof-searcher cannot independently parse a
  `CertifiedNumeric` enclosure; that is `numerical-verifier` (`physis
  enclose`, a content-addressed NumericCertificate; not a kernel
  receipt, not Canonical, not P4). A reviewer cannot independently
  rehash a `SourceRecord`; that is `provenance-auditor` (`physis cite`;
  not P3S). A reviewer cannot independently rebuild a versioned
  Constant; that is `provenance-auditor` (`physis constant [name]`;
  omitted name rebuilds the full LEDGER; not P3N, not P3S). A reviewer cannot independently round-trip a live theory
  IR package; that is `encoding-auditor` (`physis encode`; combinational
  NAND, Klein–Gordon stencil, and Wilson U(1) plaquettes; a package with
  `lean_ref` must bind the catalog identity tree; encode lists each
  bound identity by claim id, not a kernel proof;
  not P3S). An explorer cannot
  independently rebuild a `from_lab` judgment; that is `judge`
  (`physis judge`; unique-vacuum stays heuristic failed; JSON cannot
  mint `logical proved`).

## Vertical slice

| Item | Status |
|---|---|
| A. `d² = 0` | Dual-expanded identity **and** Lean kernel + nanoda receipt; `physis review` raises semantic |
| B. Lorentz interval | Same backends |
| B2. Einstein composition | Same backends; `|w|<1` over ℝ remains the evaluator |
| B3. Mass shell | Same bilinear form on `(E, p)`; typed rest-mass check remains the evaluator |
| C. Interval-certified numeric | `3/8` as `Ratio`; disjoint from `0.23122` enclosure |
| C2. Exact SM anomalies | Four chiral sums vanish as `Ratio`; hypercharges solved in Q (`checked_sqrt`); hydrogen `Q = T₃+Y` is exactly 0; GUT-scale `3/8` is `Ratio` / P3N, not Lean. Those cells name a `DomainOfValidity`. GUT `Tr Q` is `ΣY` already certified, not a second P3N. GQW at `M_Z` is not P3N |
| D. Empirical comparison | `EmpiricalReceipt` against a versioned PDG-style dataset **and** Super-K `p→e+π0`. Compatible is prediction ⊆ data; overlap without containment is inconclusive (`InsufficientPrecision`), not compatible. PDG `sin²θ_W(M_Z)` is a Gaussian (`μ`, `σ` as Ratio); Super-K is a lower-limit hull, not a Gaussian, not P3N |
| E. Open/conjectural | `predictivity.unique-vacuum` stays `Asserted`; the four encodings name distinct regimes; `prove` and `review` refuse it |

## Pure-Rust rule (revised)

Runtime and unverified physics computation remain unsafe-free Rust.
Unverified external computation is never authoritative. External formal
systems may produce proof artifacts only through isolated
certificate-checking boundaries. Lean kernel compile plus nanoda replay
of `lean4export` is that boundary for catalog identities.

## Related

- `specs/004-theories-and-claims.md`
- `specs/006-agent-protocol.md`
- `AGENTS.md`
