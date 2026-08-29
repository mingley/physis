# 009 — Computation

Status: active
Layer: information / mathematical
Id: `computation`

## Purpose

Computation is the **second domain reuse** (after electromagnetism), and the
third scientific domain overall. It further proves the substrate is
general: the same knobs, claims, verdicts, `Theory` trait, and experiment
matrix host models of computation.

Its payoff is the **halting problem**. The lab already has an `Undecidable`
verdict kind; here it means what Turing meant. An unbounded-tape Turing
machine's `comp.halts` claim is `Undecidable`, and bounding the tape
mechanically restores decidability — a clean knob → verdict diff.

## Objects

| id | object |
|---|---|
| `combinational-circuit` | a finite, acyclic boolean circuit |
| `turing-machine` | a deterministic Turing machine (`tape_bound` knob) |

## Knobs

| theory | knob | effect |
|---|---|---|
| `turing-machine` | `tape_bound` | tape length in cells; `0` = unbounded. A finite bound makes the machine a finite automaton. |

`combinational-circuit` has no knobs (it is structurally fixed).

## Claims

| id | meaning |
|---|---|
| `comp.halts` | the machine halts on every input |
| `comp.turing-complete` | the model is Turing complete |
| `comp.deterministic` | the transition function is single-valued |
| `comp.decidable-equivalence` | equivalence of two instances is decidable |
| `comp.resource-bounded` | the computation runs within an a priori resource bound |

## The knob → verdict diff

```
physis experiment computation
physis set turing-machine tape_bound 1000
```

Setting `tape_bound: 0 → 1000` flips:

- `comp.halts` `undecidable → holds` (a finite tape has finitely many
  configurations; halting is decidable by cycle detection),
- `comp.turing-complete` `holds → fails` (a bounded machine is a finite
  automaton),
- `comp.decidable-equivalence` `undecidable → holds`,
- `comp.resource-bounded` `fails → holds`.

The combinational circuit always halts and has decidable equivalence, but is
not Turing complete (no memory or feedback).

## Honest limitation: `World` is physics-shaped

The `Theory::world()` projection assumes spacetime/gauge/spectrum, which do not
apply to a computation. These objects therefore return a **degenerate
placeholder world**; only their claims carry meaning. A future milestone may
generalize the projection (e.g. a `Layer`-based observable) so non-physics
domains do not borrow a spacetime they do not have. This is a known rough edge,
not a hidden one.

## Non-goals (this milestone)

- An actual interpreter / simulator of circuits or tape machines.
- Complexity-class claims (P, NP, …) as verdicts — a later milestone.
- Landauer's principle / reversible computing, which needs the `statistical`
  layer (see `plans/004`).

## Related

- `specs/007-reuse-domains.md` — how domains are added
- `specs/008-electromagnetism.md` — the first reuse
- `plans/004-m3-domain-reuse.md` — the milestone
