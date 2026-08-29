# 004 — Theories and claims

Status: active
Layer: all

## A theory

```
Theory = id + knobs + world() + claims + evaluate
```

`world()` is a projection into `physis-model::World` (spacetime, gauge, spectrum, gravity flag, SUSY flag, parameter count, landscape log10). Worlds are comparable. Slogans are not.

## A claim

A claim is a sentence with:

- a stable id (shared across theories when the sentence is the same)
- a layer
- a default epistemic tag
- an evaluator that returns `Verdict { kind, epistemic, summary, evidence }`

Kinds: `holds`, `fails`, `undecidable`, `inapplicable`.

## Shared claim ids

Defined in `crates/physis-theory/src/claims.rs`. Experiments build matrices by these ids. If you need a new row, add it there first, then implement `evaluate` arms.

## Controls

Every ambitious theory should sit next to **controls**:

- Standard Model: empirical contact, no gravity, not UV-complete
- GR: gravity, no SM, not UV-complete as QFT

A unification story that cannot beat “inapplicable / fails” on empirical rows relative to SM, except by conjecture, has not earned empirical contact in this lab.

## Honesty protocol

When encoding a literature result you did not derive:

- tag `EncodedFact`
- put the citation-class in evidence or rustdoc
- plan the replacement (`plans/` milestone) if the fact should become a theorem

When encoding folklore (landscape ~ 10^500):

- tag `Heuristic`
- make it *knob-sensitive* so agents can explore, not a magic constant

When encoding a program’s *demand* (unique vacuum):

- tag `Conjecture` if the demand is assumed
- do not tag `Theorem` because the author wished it
