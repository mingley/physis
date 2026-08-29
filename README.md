# physis

A pure-Rust workspace for **mechanically verifiable models of reality**.

Theories are objects. Knobs are typed. Claims return verdicts. Turning a knob is required to produce a **diff of behavioral state** — not a vibe.

```
cargo run -p physis -- experiment string-critique
cargo run -p physis -- set type-iib total_dim 9
```

The second command flips `consistency.critical-dimension` from **holds** to **fails**. That is the point.

## Why this exists

Modern physics is a stack of layers — spacetime, quantum amplitudes, fields, particles, interactions — with a messy relationship between what is **theorem**, what is **encoded textbook fact**, what is **heuristic**, and what is **open**. Agents (and humans) are bad at keeping those categories apart when the model lives in slides and prose.

Rust is unusually good at this:

- the type system will not let you add kilograms to meters
- enums make illegal states unrepresentable
- a claim can carry an epistemic tag (`theorem` vs `heuristic`) as seriously as its verdict (`holds` vs `fails`)
- a lab journal is an append-only record of knob turns and verdict diffs

`physis` is a **foundational building block**. The first experiment is a laboratory for the public argument that string theory took fundamental physics into a landscape of untestable vacua (the family of critiques associated with, among others, Eric Weinstein). After that, the same substrate is meant to host electricity, computation, and other domains — not as plugins bolted on, but as new layers and theories on the same knobs-and-claims machine.

This repository does **not** decide whether string theory is false. It makes the distinctive structural claims of several theories *inspectable* and *comparable*, over long time horizons, by agents that can only act through a typed protocol.

## What's in the box

| Crate | Role |
|---|---|
| `physis-core` | SI dimensions, quantities, layers, knobs, claims, verdicts |
| `physis-model` | Spacetime, finite Hilbert space, SM spectrum, gauge groups, `World` |
| `physis-theory` | Standard Model, GR, strings/M, observer-geometry scaffold, critique lab, **electromagnetism** and **computation** (domain reuse) |
| `physis-agent` | Lab, protocol, JSONL journal |
| `physis` | Facade + CLI |

Layers, finest first: `mathematical → spacetime → quantum → field → particle → interaction → effective → statistical → information → agent`.

## The first experiment

```
physis experiment string-critique
```

Ten objects sit on one claim matrix:

- **standard-model** — empirically sharp, not a theory of gravity, not UV-complete
- **general-relativity** — gravity, classical, no SM matter
- **type-iib** / **type-iia** / **type-i** / **heterotic-e8e8** / **heterotic-so32** / **bosonic** / **m-theory** — string/M constructions with real theorems (critical dimension) and honest heuristics (landscape count)
- **observer-geometry** — a *scaffold* for unique-geometry programs. **Not Geometric Unity.** Uniqueness is an axiom/conjecture here, not a proof.

Read the `epistemic` column before treating a cell as physics. A `holds` that is a `conjecture` is not the same object as a `holds` that is a `theorem`.

## Smallest level of modern physics

Empirically confirmed description currently bottoms out at **quantum fields of the Standard Model** (quarks, leptons, gauge bosons, Higgs) on a classical 3+1 spacetime, down to the electroweak scale. Planck-scale pictures (strings, loops, causal sets, unique geometries) are first-class *theories* in this workspace. They are not smuggled in as substrate. See `docs/LAYERS.md` and `specs/002-ontology-layers.md`.

## Quick start

```bash
cargo test --workspace
cargo run -p physis -- layers
cargo run -p physis -- theories
cargo run -p physis -- knobs type-iib
cargo run -p physis -- run type-iib
cargo run -p physis -- set type-iib total_dim 9
cargo run -p physis -- experiment string-critique
cargo run -p physis -- experiment em-vacuum
cargo run -p physis -- experiment computation
cargo run -p physis -- score heterotic-e8e8
cargo run -p physis --example kinetic_energy

# record a session across runs, then mechanically verify it replays:
cargo run -p physis -- --journal session.jsonl set type-iib total_dim 9
cargo run -p physis -- replay session.jsonl
```

Adding mass to length is a type error (this doctest is required to *fail* to compile):

```rust,ignore
use physis_core::qty::{kg, meters};
let _ = kg(1.0) + meters(1.0);
```

## Repo map

```
specs/     what the system is, contract-level
plans/     what we build next, in order
docs/      how it works (architecture, layers, knobs, theories, agents)
crates/    the Rust
AGENTS.md  standing orders for long-horizon agents
```

## Honesty

- Critical dimensions of strings (26 / 10 / 11) are encoded as **theorems**.
- SM embeddings into E₈×E₈, SO(32), SO(10), Spin(10) are **verified by code** — `GaugeGroup::verified_contains_sm` walks the standard maximal-subgroup chain and checks rank/dimension at each step — but remain **encoded facts**, not full root-system branching rules (necessary conditions + an encoded chain, not a proof). Full Dynkin branching is a planned milestone.
- Green–Schwarz anomaly cancellation is a **mechanical predicate** on the gauge group (dimension 496, exactly SO(32) or E₈×E₈), not a menu — see `GaugeGroup::gs_anomaly_free_10d`. It is an **encoded fact**, not a re-derivation of the anomaly polynomial (that is a later milestone).
- Landscape counts are **heuristics**. They exist so uniqueness can *flip* when fluxes and extra dimensions move — not because we computed 10⁵⁰⁰ Calabi–Yau flux vacua.
- Observer-geometry's gauge group is a **conjectural assignment**. The program is here as a contrast class, not as a completed theory.

## License

MIT. See `LICENSE`.
