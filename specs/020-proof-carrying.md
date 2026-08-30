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
  encoding-wide. `dec.hodge-harmonic` names discrete combinatorial Hodge
  on finite simplicial 1-cochains, not the smooth Hodge theorem.
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
  does not yet project a statistical object, and a crate outside
  physis-core cannot mint one. `Verdict` has no `Deserialize` impl: JSON
  cannot mint a `certified-numeric` overlay or an encoding-review tag.
  `Claim` derivation, empirical, and semantic fields are private: a
  theory cannot assign `CertifiedNumeric` on the claim. Overlays live on
  `Verdict`, whose derivation / empirical / semantic / enclosure fields
  are also private: a public assignment cannot mint `CertifiedNumeric`.
  The overlay path is `Verdict::with_certified_numeric`.
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
  running at `M_Z` and the 3% band are not P3N. `CrossChecked` / P2 is earned when two independent
  executable paths agree (`dec.hodge-harmonic`: Laplacian nullity vs
  coboundary `b₁` on this complex; the cell names that discrete regime).
  Euler–Poincaré stays `executed`: with these Betti
  formulas, `b₀−b₁+b₂ ≡ V−E+F` is rank-cancellation, not a second path.
  Neither overlay is a Lean receipt. A P3F
  `Unreviewed` result is labelled dangerous. `physis why` prints a typed
  `Judgment` (evaluator `holds` is `logical undetermined`; a
  `CertifiedNumeric` Holds is `numeric certified` with a display
  enclosure, not a kernel proof; a coarse numeric order is `numeric
  unresolved`, not a failed theorem) and the
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
| 7 | `physis-ir` | Line-oriented theory package (not a Lean replacement) |
| 8 | `physis-audit`, `physis audit` | Red-team corpus must fail to promote |
| 9 | `physis design` | Rank theory pairs by discriminating claim count |
| 10 | `physis loop` | Observe → hypothesize (constrained structural mutation of chosen/fitted knobs) → prove → falsify → replicate → design → audit → review |
| origin | `KnobSpec.origin`, `physis inspect` | Distinguish chosen/fitted knobs from measured ones; invert trust/class/origin/gap |
| gaps | `physis gaps`, `NodeKind::KnowledgeGap` | Live gap graph, content-addressed; rebuilt, not deserialized. `MissingTheorem` only for evaluator-Holds claims without a receipt; Fails is decided, not a missing lemma. `InsufficientPrecision` is overlap without containment on an empirical receipt, **or** a lattice too coarse to certify a numerical order (`field.second-order-accurate`, `|k a| ≥ 1`). `ComputationallyIntractable` is coNP-complete / exponential search, not Rice. `MissingDataset` is an empirical prediction with no registered dataset. Super-K `p→e+π0` is registered; `gut.proton-lifetime-sk` is the dim-6 comparison, not that hole |
| lemmas | `Claim.depends_on` | Live lemma edges in `gaps` / `why`; not statement identity; never deserialized as authority |
| evidence | `physis evidence` | Competing encodings (distinct FormalClaims of one slug) and competing evaluations; confidence is derived TrustProfile, not a numeric score; never Canonical or P4 |
| trust-gate | `Lab::exec` | `reproduce` and loop-review require P3F. Standalone `review` stays encoding-axis. Observation is free |
| roles | `Role`, `ResearchBudget`, `physis formalize` | Named processes propose; only `verify` mints. Explorer cannot prove. Budget is a cap, not a proof |
| semantic | `physis-semantic`, `physis review` | Provenance + independent IR encoding + corpus, bound to the catalog FormalClaim; never `Canonical` |
| constants | `physis-constants` | Versioned `c` (SI 2019 exact) |

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
  GUT `Tr Q` stays encoding-wide.
  `dec.hodge-harmonic` names discrete combinatorial Hodge; Euler–Poincaré
  and Poincaré stay encoding-wide. `field.dispersion-continuum-limit` names
  the longest lattice mode; ohm-circuit `em.quasi-static-valid` names
  `λ > 100 ×` circuit size. Maxwell's copy of that slug stays encoding-wide.
- Trust tiers do not gate observation or standalone encoding-review.
  They now refuse `reproduce` and the loop's review step without P3F.
  Named *roles* still gate who may issue an op.

## Vertical slice

| Item | Status |
|---|---|
| A. `d² = 0` | Dual-expanded identity **and** Lean kernel + nanoda receipt; `physis review` raises semantic |
| B. Lorentz interval | Same backends |
| B2. Einstein composition | Same backends; `|w|<1` over ℝ remains the evaluator |
| B3. Mass shell | Same bilinear form on `(E, p)`; typed rest-mass check remains the evaluator |
| C. Interval-certified numeric | `3/8` as `Ratio`; disjoint from `0.23122` enclosure |
| C2. Exact SM anomalies | Four chiral sums vanish as `Ratio`; hypercharges solved in Q (`checked_sqrt`); hydrogen `Q = T₃+Y` is exactly 0; GUT-scale `3/8` is `Ratio` / P3N, not Lean. Those cells name a `DomainOfValidity`. GUT `Tr Q` is `ΣY` already certified, not a second P3N. GQW at `M_Z` is not P3N |
| D. Empirical comparison | `EmpiricalReceipt` against a versioned PDG-style dataset **and** Super-K `p→e+π0`. Compatible is prediction ⊆ data; overlap without containment is inconclusive (`InsufficientPrecision`), not compatible. Super-K is a lower-limit hull, not P3N |
| E. Open/conjectural | `predictivity.unique-vacuum` stays `Asserted`; `prove` and `review` refuse it |

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
