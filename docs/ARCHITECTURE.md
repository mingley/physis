# Architecture

```
┌────────────────────────────────────────────────────────┐
│  physis (CLI / facade)                                  │
│    layers | theories | knobs | run | set | experiment   │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│  physis-agent                                           │
│    Lab  ──  Command/Response  ──  Journal (JSONL)       │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│  physis-theory                                          │
│    Theory + knobs + evaluate(claim) → Verdict           │
│    SM, GR, StringTheory, ObserverGeometry               │
│    experiment: string-critique                          │
└───────────────────────────┬──────────────────────────────┘
                            │ world()
┌───────────────────────────▼──────────────────────────────┐
│  physis-model                                           │
│    Manifold, Ket, Spectrum, GaugeGroup, World           │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│  physis-core                                            │
│    Qty<D>, LayerId, Knob*, Claim, Verdict, Scale        │
└───────────────────────────────────────────────────────────┘
```

## Data flow of a knob turn

1. Agent issues `set type-iib total_dim 9`.
2. Lab looks up the theory and the `KnobSpec`.
3. Token is parsed against the domain (uint 2..=32).
4. All claims are evaluated **before**.
5. `Knobbed::set` mutates the theory.
6. All claims are evaluated **after**.
7. Diff of `VerdictKind` is the result.
8. Journal appends `SetKnob`.

No PDE is solved. The “simulation” is the evaluate functions. That is enough to host long-horizon conceptual work and is the right complexity for v0.

## Why not one crate

Agents and humans should be able to depend on `physis-core` without taking string theories, and on `physis-model` without taking the critique experiment. Domains in M3 may add crates; they should not need to fork core.

## Why not a Python notebook

Python is where knobs become strings and kilograms add to seconds. The whole bet is that the type system *is* the scientific instrument.
