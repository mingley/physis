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
| `visibility` | Werner-state visibility V ∈ [0,1]; the CHSH value scales as V·2√2 |

## Claims

| id | meaning | how it is decided |
|---|---|---|
| `quantum.born-normalization` | the singlet is normalized | computed from the ket: `⟨ψ|ψ⟩ = 1`, `Σ pᵢ = 1` |
| `quantum.bell-violation` | `|S| > 2` (local realism refuted) | computed CHSH `S` at the optimal angles; holds iff `S > 2` |
| `quantum.tsirelson-bound` | `|S| ≤ 2√2` | computed; quantum cannot exceed Tsirelson |

## The refutation

The CHSH correlator `S = |E(a,b) − E(a,b′) + E(a′,b) + E(a′,b′)|` with
`E(a,b) = −V·cos(2(a−b))` and the optimal angles `(0, 45°, 22.5°, 67.5°)`
evaluates to `V·2√2`. At full visibility `S = 2√2 ≈ 2.828 > 2`: no local
hidden-variable theory can reproduce it. This is the whole point — a famous
19th/20th-century intuition (local realism) is refuted by a computation, with
the classical bound (2) and the quantum bound (2√2) both explicit.

## Knob → verdict

```
physis experiment bell
physis set bell-test visibility 0.5   # S = √2 < 2: a local model now suffices
```

## Non-goals (this milestone)

- A full density-matrix / measurement simulator; the correlator is computed in
  closed form with a visibility parameter.
- Loophole modelling (detection, locality) — the point here is the ideal bound.

## Related

- `specs/002-ontology-layers.md` (the `quantum` layer)
- `specs/007-reuse-domains.md` (how domains are added)
