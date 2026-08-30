# 020 — Proof-carrying Physis (Level 3)

Status: active (Milestone 1 implemented; Milestones 2–10 planned)
Layer: all
Id: `proof-carrying`

## Objective

Nothing gains authority merely because an agent wrote code that returns
`Holds`. Authority comes from explicit, independently checkable artifacts.

This spec is the trust kernel of Level-3 Physis. The full laboratory
(Lean/Physlib, provenance, certified numerics, agent protocol v2, IR,
adversarial roles, experiment design) is ordered so that a fast lab cannot
accumulate incorrectly encoded “knowledge.”

## What shipped in Milestone 1

### Three orthogonal questions

Every claim and verdict carries:

| Axis | Type | M1 default for former `Epistemic::Theorem` |
|---|---|---|
| What kind of claim? | `ClaimClass` | `ModelInternal` (or `Mathematical` / `Phenomenological` when that is what the encoding actually is) |
| How was the deduction checked? | `DerivationAssurance` | `Executed` |
| What does observation say? | `EmpiricalStatus` | `NotApplicable` or `Untested` |
| Is the encoding the intended physics? | `SemanticAssurance` | `Unreviewed` |

Former `Conjecture` / `Heuristic` / `Open` map to the matching `ClaimClass`
with `DerivationAssurance::Asserted`.

This is **not** a judgement that the science is doubtful. It records that
Physis has not mechanically established a kernel proof.

### `MachineProved` is unforgeable

There is no `DerivationAssurance::MachineProved` and no `Epistemic::Theorem`.

A kernel-checked result is `physis_verifier::Verified<T>`. Fields are
private. The mint function is `pub(crate)`. External crates cannot
construct it (compile-fail). The public `ReceiptStore` starts empty.

### Formal identity

`Claim.statement_hash` is a SHA-256 of a canonical listing: id, statement,
class, layer, assumption-set id, domain id. Changing the sentence (∀ vs ∃,
a sign, a unit) yields a new hash. `FormalClaim` is that identity object.

### Assumptions and domain

Every claim has a non-empty `AssumptionSet` (M1 default:
`encoding-is-the-model`) and a `DomainOfValidity` (M1 default:
encoding-wide, with an explicit note that silent extrapolation is a new
claim). `AxiomLedger::propose` always stores `Unreviewed`.

### CLI

```
physis epistemics   # derivation / class / semantic ledgers; no theorem row
physis why <claim>  # assumptions, identity hash, kernel proof: none
```

`physis run` prints `executed` / `asserted`, not `theorem`.

## What is not yet true (later milestones)

| Milestone | Work |
|---|---|
| 2 | Lean 4 + Physlib + Lean kernel + independent checker (nanoda). Immutable challenge. Dual replay. `ProofReceipt`. One-byte statement mutation invalidates the receipt. Unauthorized `axiom` / `sorry` / `admit` blocks promotion. |
| 3 | `physis-provenance`, source locks, semantic review workflow |
| 4 | Validated numerics; naked `f64` is not authoritative for threshold claims |
| 5 | Content-addressed artifact DAG and incremental invalidation |
| 6 | Agent protocol v2 (`branch`, `sweep`, `falsify`, `prove`, …) |
| 7 | Declarative `physis-ir` theory packages |
| 8 | Adversarial laboratory and red-team corpus |
| 9 | Experimental design engine |
| 10 | Autonomous research loop |

## Pure-Rust rule (revised)

Runtime, orchestration, and unverified physics computation remain
unsafe-free Rust. Unverified external computation is never authoritative.
External formal systems may produce **proof artifacts** only through
isolated certificate-checking boundaries. Lean is not asked to simulate
the physics. The final proof can be independently checked on the Rust
side (nanoda).

## Vertical slice (after M2+)

A. `d² = 0` (mathematical)
B. Lorentz interval preservation (model-internal)
C. Interval-certified numeric prediction
D. Empirical comparison with a versioned dataset
E. An honest open/conjectural result

## Related

- `specs/004-theories-and-claims.md`
- `specs/006-agent-protocol.md`
- `AGENTS.md`
