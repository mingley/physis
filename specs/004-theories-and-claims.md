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
- a `ClaimClass`
- a `DerivationAssurance` (never `MachineProved` — that is not an enum)
- an `EmpiricalStatus` and `SemanticAssurance`
- an explicit `AssumptionSet` and `DomainOfValidity`
- a content-addressed `statement_hash`
- an evaluator that returns `Verdict { kind, class, derivation, empirical, semantic, summary, evidence }`

Kinds: `holds`, `fails`, `undecidable`, `inapplicable`.

`Executed` means the evaluator ran inside this encoding. It is not a Lean kernel proof. See `specs/020-proof-carrying.md`.

## Shared claim ids

Defined in `crates/physis-theory/src/claims.rs`. Experiments build matrices by these ids. If you need a new row, add it there first, then implement `evaluate` arms.

## Controls

Every ambitious theory should sit next to **controls**:

- Standard Model: empirical contact, no gravity, not UV-complete
- GR: gravity, no SM, not UV-complete as QFT

A unification story that cannot beat “inapplicable / fails” on empirical rows relative to SM, except by conjecture, has not earned empirical contact in this lab.

## Honesty protocol

When encoding a literature result you did not derive:

- tag `ClaimClass::Phenomenological` (formerly `EncodedFact`)
- derivation `Executed` if the lab actually checks a table or embedding; `Asserted` if it does not
- put the citation-class in evidence or rustdoc
- plan the replacement (`specs/020` M2–M3) if the fact should become a verified theorem

When encoding folklore (landscape ~ 10^500):

- tag `ClaimClass::Heuristic`
- derivation `Asserted`
- make it *knob-sensitive* so agents can explore, not a magic constant

When encoding a program’s *demand* (unique vacuum):

- tag `ClaimClass::Conjecture` if the demand is assumed
- do not tag `Executed` model-internal because the author wished it
- never mint `Verified<T>`
