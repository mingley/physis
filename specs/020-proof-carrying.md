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
| constants | `physis-constants`, `physis constant` | Versioned SI 2019 defining `c`, `Δν_Cs`, `e`, `k`, `N_A`, `K_cd` as exact `Ratio`. Planck `h` is SI-exact `SciExact` `662607015e-42` (not a `Ratio`: `i128` denominator overflow). `ħ` is not stored. CODATA 2018 `G` is a one-sigma `Interval` (JPCRD table XXXI UNIVERSAL). CODATA 2018 `μ₀` is a one-sigma `Interval` `1.25663706212(19)×10^{-6}` N A⁻² (JPCRD table XXXI UNIVERSAL). CODATA 2018 `ε₀` is a one-sigma `Interval` `8.8541878128(13)×10^{-12}` F m⁻¹ (JPCRD table XXXI UNIVERSAL). CODATA 2018 `Z₀` is a one-sigma `Interval` `376.730313668(57)` ohm (JPCRD table XXXI UNIVERSAL); `Y₀` is not stored. CODATA 2018 `α` is a one-sigma `Interval` `7.2973525693(11)×10^{-3}` (JPCRD table XXXI ATOMIC AND NUCLEAR). CODATA 2018 inverse-α is a one-sigma `Interval` `137.035999084(21)` from the same table. CODATA 2018 Rydberg frequency `cR∞` is a one-sigma `Interval` `3.2898419602508(64)×10^{15}` Hz from the same table. CODATA 2018 Rydberg energy equivalent `hcR∞` is a one-sigma `Interval` `2.1798723611035(42)×10^{-18}` J from the same table; the eV conversion is not stored. CODATA 2018 Rydberg `R∞` is a one-sigma `Interval` `10973731.568160(21)` m⁻¹ from the same table. CODATA 2018 Bohr radius `a₀` is a one-sigma `Interval` `5.29177210903(80)×10^{-11}` m from the same table. CODATA 2018 Hartree energy `E_h` is a one-sigma `Interval` `4.3597447222071(85)×10^{-18}` J from the same table; the eV conversion is not stored. CODATA 2018 electron-muon mass ratio `m_e/m_μ` is a one-sigma `Interval` `4.83633169(11)×10^{-3}` (JPCRD table XXXI Electron, e-). CODATA 2018 electron-proton mass ratio `m_e/m_p` is a one-sigma `Interval` `5.44617021487(33)×10^{-4}` from the same section. CODATA 2018 electron-neutron mass ratio `m_e/m_n` is a one-sigma `Interval` `5.4386734424(26)×10^{-4}` from the same section. CODATA 2018 electron-deuteron mass ratio `m_e/m_d` is a one-sigma `Interval` `2.724437107462(96)×10^{-4}` from the same section. CODATA 2018 electron-triton mass ratio `m_e/m_t` is a one-sigma `Interval` `1.819200062251(90)×10^{-4}` from the same section. CODATA 2018 electron-helion mass ratio `m_e/m_h` is a one-sigma `Interval` `1.819543074573(79)×10^{-4}` from the same section. CODATA 2018 electron to alpha particle mass ratio `m_e/m_α` is a one-sigma `Interval` `1.370933554787(45)×10^{-4}` from the same section. CODATA 2018 electron charge to mass quotient `−e/m_e` is a one-sigma `Interval` `−1.75882001076(53)×10^{11}` C kg⁻¹ from the same section. CODATA 2018 electron molar mass `M_e` is a one-sigma `Interval` `5.4857990888(17)×10^{-7}` kg mol⁻¹ from the same section. CODATA 2018 reduced Compton wavelength `ƛ_C` is a one-sigma `Interval` `3.8615926796(12)×10^{-13}` m from the same section. CODATA 2018 Compton wavelength `λ_C` is a one-sigma `Interval` `2.42631023867(73)×10^{-12}` m from the same section. CODATA 2018 classical electron radius `r_e` is a one-sigma `Interval` `2.8179403262(13)×10^{-15}` m from the same section. CODATA 2018 electron magnetic moment `μ_e` is a one-sigma `Interval` `−9.2847647043(28)×10^{-24}` J T⁻¹ from the same section. CODATA 2018 electron magnetic moment to Bohr magneton ratio `μ_e/μ_B` is a one-sigma `Interval` `−1.00115965218128(18)` from the same section. CODATA 2018 electron magnetic moment to nuclear magneton ratio `μ_e/μ_N` is a one-sigma `Interval` `−1838.28197188(11)` from the same section. CODATA 2018 electron magnetic-moment anomaly `ae` is a one-sigma `Interval` `1.15965218128(18)×10^{-3}` from the same section. CODATA 2018 electron g-factor `ge` is a one-sigma `Interval` `−2.00231930436256(35)` from the same section. CODATA 2018 electron-muon magnetic-moment ratio `mu_e_mmu` is a one-sigma `Interval` `206.7669883(46)` from the same section. CODATA 2018 `m_p` is a one-sigma `Interval` `1.67262192369(51)×10^{-27}` kg (JPCRD table XXXI Proton, p); electron mass is not stored (`10^{42}` overflows `i128`). IAU 2012 `au` is an exact `Ratio` `149597870700` m (BIPM table 8). The electronvolt is an exact `Ratio` `1.602176634×10^{-19}` J from the same table (SI 2019, same decimal as `e`, unit joule). The parsec is `(648000/π) au` and is not a Ratio. IAU 2015 `(GM)_☉^N` is an exact `Ratio` `1.3271244×10^20` m³ s⁻² (AJ 152, 41 table 1): a conversion ruler, not a measured solar mass. IAU 2015 `R_☉^N` is an exact `Ratio` `695700000` m from the same table: a conversion ruler, not a measured photospheric radius. IAU 2015 `L_☉^N` is an exact `Ratio` `3.828×10^26` W from the same table: a conversion ruler, not a measured solar luminosity. `physis constant [name]` independently rebuilds those hashes (`provenance-auditor`; not P3N). Omitted name rebuilds the full LEDGER into one VersionedConstant bundle. Overlapping `physis_model` Qty floats lockstep the ledger: `c`, `au`, `GM_sun`, `R_sun`, and `L_sun` via integer `to_f64`; `e`/`k`/`eV` via IEEE rounding of the SI decimal (`SciExact::to_f64`, not reduced `Ratio::to_f64`); `h` via `SciExact::to_f64`; `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, and `m_p` Qty values are the CODATA centres inside the hulls. Theories still evaluate with `f64` Qty |

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
  CODATA 2018 `m_p` is a
  one-sigma `Interval` `1.67262192369(51)×10^{-27}` kg (JPCRD table XXXI
  Proton, p); electron mass is not stored (`10^{42}` overflows `i128`). IAU 2012 `au` is an exact
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
  `c`, `au`, `GM_sun`, `R_sun`, and `L_sun` via integer `to_f64`; `e`/`k`/`eV` via IEEE rounding of the SI
  decimal (`SciExact::to_f64`, not reduced `Ratio::to_f64`); `h` via
  `SciExact::to_f64`; `G`, `mu0`, `epsilon0`, `Z0`, `alpha`, `inv_alpha`, `cRinf`, `hcRinf`, `Rinf`, `a0`, `Eh`, `me_mmu`, `me_mp`, `me_mn`, `me_md`, `me_mt`, `me_mh`, `me_malpha`, `e_me`, `M_e`, `lambdabar_C`, `lambda_C`, `re`, `mu_e`, `mu_e_muB`, `mu_e_muN`, `ae`, `ge`, `mu_e_mmu`, and `m_p` Qty values are the CODATA centres
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
