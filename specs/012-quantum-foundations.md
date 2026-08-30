# 012 — Quantum foundations

Status: active
Layer: quantum
Id: `bell`

## Purpose

A fifth domain that puts an old assumption — **local realism** — on trial, and
mechanically finds it wanting. It also gives the `quantum` layer (finite kets,
Born rule) its first `Theory`, where earlier milestones only used it internally.

## Object

| id | object |
|---|---|
| `bell-test` | a CHSH Bell test on a two-qubit singlet |

## Knobs

| knob | effect |
|---|---|
| `visibility` | Werner-state visibility V ∈ [0,1]; the CHSH value scales as V·2√2 on the singlet. Ket topology is not this knob: `add-product` is an IR mutation. Tsirelson is not this knob: `add-pr-box` is an IR mutation |

## Claims

| id | meaning | how it is decided |
|---|---|---|
| `quantum.born-normalization` | the ket is normalized | computed from the live ket: `⟨ψ|ψ⟩ = 1`, `Σ pᵢ = 1` |
| `quantum.correlator-from-operators` | `E(a,b) = ⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩ = −cos(a−b)` | **derived from the operators** on the singlet. Domain: two-qubit singlet. `add-product` appends `state product` and this cell fails. That is not a knob |
| `quantum.bell-violation` | `|S| > 2` (local realism refuted) | computed CHSH `S` at the optimal angles; holds iff `S > 2`. Domain: two-qubit singlet. A product ket fails. `visibility` still scales the singlet independently |
| `quantum.tsirelson-bound` | `|S| ≤ 2√2` | theorem on `bell-test` (named domain: Hilbert-space CHSH (Tsirelson 2√2)). Live: brute-force over a 90³ angle grid finds `|S|max ≈ 2.827`, never exceeding `2√2`. `add-pr-box` appends `correlator pr-box` and the CHSH combination of `E = (−1)^{xy}` is 4, so this cell fails. That is not a knob |
| `quantum.local-realism-bound` | the LHV maximum of `|S|` is exactly 2 | **derived by enumeration** of all `2⁴` deterministic ±1 strategies; the max is 2 |

## The refutation

The CHSH correlator `S = |E(a,b) − E(a,b′) + E(a′,b) + E(a′,b′)|` with
`E(a,b) = −V·cos(a−b)` and the optimal angles `(0, 90°, 45°, 135°)` evaluates to
`V·2√2`. The correlator `−cos(a−b)` is itself **derived**: the operator
expectation `⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩` is computed on the singlet (building the spin
operators, tensoring them, and applying the 4×4 matrix — all in
`physis-model`), and `quantum.correlator-from-operators` checks it matches the
closed form to machine precision. At full visibility `S = 2√2 ≈ 2.828 > 2`: no
local hidden-variable theory can reproduce it. This is the whole point — a
famous 19th/20th-century intuition (local realism) is refuted by a computation,
with the classical bound (2) and the quantum bound (2√2) both explicit.

Both bounds are **derived, not asserted**. `quantum.local-realism-bound`
enumerates every one of the `2⁴` deterministic outcome assignments a local
hidden-variable model could use and finds the maximum `|S|` is exactly 2 — so
the CHSH threshold falls out of the model, it is not put in by hand.
`quantum.tsirelson-bound` brute-force maximizes `|S|` over a fine grid of
measurement angles and confirms no quantum strategy exceeds `2√2` (the grid
maximum saturates it). Together they mechanize *why* `2 < S ≤ 2√2` is the
signature of quantum nonlocality.

## Knob → verdict

```
physis experiment bell
physis set bell-test visibility 0.5   # S = √2 < 2: a local model now suffices
physis hypothesize bell-test          # add-product and add-pr-box are IR, not set
```

## Non-goals (this milestone)

- A full density-matrix / measurement simulator; the singlet correlator is
  computed in closed form with a visibility parameter. The product-ket fork
  evaluates operator expectations on `|01⟩`, not a circuit simulator.
  The PR-box fork evaluates the bit-table CHSH combination `E = (−1)^{xy}`,
  not a Hilbert-space ket.
- Loophole modelling (detection, locality) — the point here is the ideal bound.

## Related

- `specs/002-ontology-layers.md` (the `quantum` layer)
- `specs/007-reuse-domains.md` (how domains are added)
