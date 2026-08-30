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
- Every claim has assumptions, a domain, and a SHA-256 statement identity.
- `physis why` / `physis epistemics` do not print a `theorem` tag.
-   `TrustProfile` is derived from receipts and reviews. P3F cannot be set
  as an enum; P4 is not assigned from an in-process remint. P3N is
  earned when an evaluator overlays `CertifiedNumeric` after an exact
  `Ratio` cancellation (Standard Model chiral anomalies). A P3F
  `Unreviewed` result is labelled dangerous. `physis why` prints a typed
  `Judgment` (evaluator `holds` is `logical undetermined`) and the
  transitive axiom closure from `AxiomLedger`.

### Milestone 2 — dual-check receipts (exact + Lean)

Trusted side: `physis-proof::Challenge::generate` from a `FormalClaim`.
Untrusted side: `UntrustedProof`. The only public mint is
`physis_verifier::verify`, which *runs* two checkers.

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
| 6 | `prove falsify sweep branch compare sensitivity` | Structured agent ops |
| 7 | `physis-ir` | Line-oriented theory package (not a Lean replacement) |
| 8 | `physis-audit`, `physis audit` | Red-team corpus must fail to promote |
| 9 | `physis design` | Rank theory pairs by discriminating claim count |
| 10 | `physis loop` | Observe → hypothesize → prove → falsify → replicate → design → audit → review |
| origin | `KnobSpec.origin`, `physis inspect` | Distinguish chosen/fitted knobs from measured ones; invert trust/class/origin/gap |
| gaps | `physis gaps`, `NodeKind::KnowledgeGap` | Live gap graph, content-addressed; rebuilt, not deserialized. `MissingTheorem` only for evaluator-Holds claims without a receipt; Fails is decided, not a missing lemma. `InsufficientPrecision` is overlap without containment on an empirical receipt. `ComputationallyIntractable` is coNP-complete / exponential search, not Rice. `MissingDataset` is an empirical prediction with no registered dataset (`gut.proton-lifetime-sk`); Super-K prose is not a Dataset |
| lemmas | `Claim.depends_on` | Live lemma edges in `gaps` / `why`; not statement identity; never deserialized as authority |
| trust-gate | `Lab::exec` | `reproduce` and loop-review require P3F. Standalone `review` stays encoding-axis. Observation is free |
| roles | `Role`, `ResearchBudget`, `physis formalize` | Named processes propose; only `verify` mints. Explorer cannot prove. Budget is a cap, not a proof |
| semantic | `physis-semantic`, `physis review` | Provenance + independent IR encoding + corpus; never `Canonical` |
| constants | `physis-constants` | Versioned `c` (SI 2019 exact) |

Journal events are hash-linked in memory (`Journal::tip`). Journal
restore of a `prove` event remints through `verify` (never Deserialize):
Lean kernel + nanoda when the pipeline is wired, otherwise the exact
dual expanders. `physis prove` uses the same preference.

## What is not yet true

- `SemanticAssurance::Canonical` (reserved; not agent-mintable)
- P4 independent reproduction (in-process `reproduce` remints and
  **refuses** to assign P4; a distinct implementation is still required)
- Mathlib-scale Physlib; four catalog identities are kernel-checked
  (`d²`, interval, Einstein composition, mass shell). The mass-shell
  polynomial is the interval identity on 4-momentum, not a fifth
  algebraic idea. That is not Mathlib.
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
| C2. Exact SM anomalies | Four chiral sums vanish as `Ratio`; `CertifiedNumeric` / P3N, not Lean |
| D. Empirical comparison | `EmpiricalReceipt` against a versioned PDG-style dataset. Compatible is prediction ⊆ data; overlap without containment is inconclusive (`InsufficientPrecision`), not compatible |
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
