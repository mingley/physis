# 020 — Proof-carrying Physis (Level 3)

Status: active (Milestones 1–9 sliced; Lean/nanoda dual kernel still open)
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

### Milestone 2 — exact dual-check (Lean still open)

Trusted side: `physis-proof::Challenge::generate` from a `FormalClaim`.
Untrusted side: `UntrustedProof`. The only public mint is
`physis_verifier::verify`, which *runs* two checkers.

Catalogued polynomial identities, dual-expanded (recursive AST vs postfix
stack):

- `dec.d-squared-zero`: `(b−a)−(c−a)+(c−b) ≡ 0`
- `sr.invariant-interval`: `(t−βx)² − (x−βt)² − (1−β²)(t²−x²) ≡ 0`

A one-byte mutation of the challenge bytes is `ChallengeTampered`.
A sign flip of the identity fails both expanders. `axiom` / `sorry` /
`admit` in Lean source is `UnauthorizedAxiom`. Clean Lean without two
kernels is `LeanPipelineNotWired` — **refuses to mint**.

`physis prove dec.d-squared-zero` records a `FormalBackend::ExactCertificate`
receipt. That is not a Lean kernel proof; the receipt says so.

Still open: Lean 4 + Physlib + Lean kernel + nanoda on the same export.

### Milestones 3–9 — first slices

| Slice | Crate / CLI | What it does |
|---|---|---|
| 3 | `physis-provenance` | Rejects `source: textbook`; requires a page/equation/… locator |
| 4 | `physis-numeric`, `physis-data` | Exact `Ratio` / `Interval`; SU(5) `3/8` disjoint from PDG `sin²θ_W(M_Z)` |
| 5 | `physis-store` | Content-addressed DAG; descendants only are invalidated |
| 6 | `prove falsify sweep branch compare sensitivity` | Structured agent ops |
| 7 | `physis-ir` | Line-oriented theory package (not a Lean replacement) |
| 8 | `physis-audit`, `physis audit` | Red-team corpus must fail to promote |
| 9 | `physis design` | Rank theory pairs by discriminating claim count |
| constants | `physis-constants` | Versioned `c` (SI 2019 exact) |

Journal events are hash-linked in memory (`Journal::tip`).

## What is not yet true

- Lean kernel replay + nanoda on a `lean4export` file
- Semantic review workflow that can raise `SemanticAssurance` above `Unreviewed`
- Full autonomous research loop (observe → hypothesize → prove → falsify →
  replicate → design next experiment) as a single scheduled orchestrator
- Agents other than the lab protocol (Explorer, Formalizer, … as processes)

## Vertical slice

| Item | Status |
|---|---|
| A. `d² = 0` | Dual-expanded exact identity; `physis prove` mints a receipt |
| B. Lorentz interval | Same backend |
| C. Interval-certified numeric | `3/8` as `Ratio`; disjoint from `0.23122` enclosure |
| D. Empirical comparison | `EmpiricalReceipt` against a versioned PDG-style dataset |
| E. Open/conjectural | `predictivity.unique-vacuum` stays `Asserted`; `prove` refuses it |

## Pure-Rust rule (revised)

Runtime and unverified physics computation remain unsafe-free Rust.
Unverified external computation is never authoritative. External formal
systems may produce proof artifacts only through isolated
certificate-checking boundaries. That Lean/nanoda boundary is typed but
not wired.

## Related

- `specs/004-theories-and-claims.md`
- `specs/006-agent-protocol.md`
- `AGENTS.md`
