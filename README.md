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

`physis` is a **foundational building block**. The flagship experiment is a laboratory for the public argument that string theory took fundamental physics into a landscape of untestable vacua (the family of critiques associated with, among others, Eric Weinstein). The *same* substrate now also hosts **electromagnetism, computation, thermodynamics, and quantum foundations** — not plugins bolted on, but new layers and theories on the same knobs-and-claims machine.

This repository does **not** decide whether string theory is false. It makes the distinctive structural claims of several theories *inspectable* and *comparable*, over long time horizons, by agents that can only act through a typed protocol.

## What's in the box

| Crate | Role |
|---|---|
| `physis-core` | SI dimensions, quantities, layers, knobs, claims, verdicts |
| `physis-model` | Spacetime, finite Hilbert space, SM spectrum, gauge groups, `World` |
| `physis-theory` | five domains on one substrate: fundamental physics (SM, GR, strings/M, observer-geometry), **electromagnetism**, **computation**, **thermodynamics**, and **quantum foundations** |
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

## Five domains, one substrate

The same typed knob→verdict machine hosts five sciences (`physis experiments`):

| experiment | what it scrutinizes |
|---|---|
| `string-critique` | string constructions vs SM/GR/observer-geometry — predictivity and the "accommodate vs derive" critique (a `euler_number` knob makes three generations a *choice*, not a derivation) |
| `em-vacuum` | electromagnetism — `1/√(ε₀μ₀)=c` and the Maxwell equations as **computed theorems**; a medium and a lumped circuit as effective limits |
| `computation` | a combinational circuit vs a Turing machine — the halting problem and P vs NP as honest `undecidable`/`open` |
| `field-modes` | a Klein–Gordon scalar field on a lattice — computed dispersion, second-order accuracy, and a tachyon from `mass² < 0` |
| `gauge-lattice` | compact U(1) vs SU(2)/SU(3) — asymptotic freedom, a computed strong-coupling area law, and the 4D mass gap as a `conjecture` |
| `thermo` | a classical ideal gas — equipartition and the second law hold; the third law **fails honestly** (needs quantum statistics) |
| `blackbody` | cavity radiation — **Rayleigh–Jeans fails** finite energy, T⁴, and Wien's peak; Planck holds them; `set planck quantum false` restores the ultraviolet catastrophe |
| `solid` | lattice oscillators — **Dulong–Petit fails** the third law; Einstein holds it but **fails T³** (exponential freeze-out); Debye holds both the third law and `C_V ∝ T³` |
| `gravity` | solar-system gravity — **Newton fails** Eddington's 1.75″ and Mercury's 43″; GR holds them; Soldner's half-angle is the standing claim GR doubles |
| `bell` | a CHSH Bell test — **local realism is refuted** by a computed `S = 2√2 > 2` |

Domains also compose: `run landauer-engine` bridges computation and
thermodynamics — erasing a bit costs `k_B·T·ln2` as a typed `Qty<Energy>`
theorem, and `set landauer-engine reversible true` flips
`info.thermodynamically-free` `fails → holds`.

The `su5-gut` theory mechanizes "accommodate vs derive" with real stakes:
embedding one SM generation in a complete SU(5) multiplet **derives** charge
quantization (`Tr Q = 0`) and `sin²θ_W = 3/8` as computed theorems. Running
that `3/8` down to `M_Z` (Georgi–Quinn–Weinberg, `α_em` and `α_s` only)
**fails** for minimal SU(5) (`≈0.207` vs measured `0.231`) and holds for the
MSSM. Minimal SU(5) is honestly **falsified** — it `fails`
`gut.coupling-unification`, `gut.weinberg-angle-mz`, and
`gut.proton-decay-viable` (excluded by Super-Kamiokande), which a
`supersymmetric` knob revives as heuristics.

`physis epistemics` tallies the whole lab's knowledge state by epistemic tag
(currently ~104 theorems alongside encoded-facts, conjectures, heuristics, and
honestly-`open` problems). `physis --json <command>` emits the typed matrices and
verdict diffs for agents.

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
cargo run -p physis -- experiments
cargo run -p physis -- experiment string-critique
cargo run -p physis -- experiment em-vacuum
cargo run -p physis -- experiment computation
cargo run -p physis -- experiment field-modes
cargo run -p physis -- experiment gauge-lattice
cargo run -p physis -- experiment thermo
cargo run -p physis -- experiment blackbody
cargo run -p physis -- set planck quantum false   # ultraviolet catastrophe
cargo run -p physis -- experiment solid
cargo run -p physis -- set einstein-solid temperature 4000
cargo run -p physis -- set einstein-solid spectrum debye   # T³ fails → holds
cargo run -p physis -- experiment gravity
cargo run -p physis -- set general-relativity dim 5
cargo run -p physis -- experiment bell
cargo run -p physis -- run de-rham          # d²=0, Betti numbers; set shape disk/circle/torus/klein/sphere
cargo run -p physis -- run special-relativity   # invariants; then flip absolute_time
cargo run -p physis -- set special-relativity absolute_time true
cargo run -p physis -- run su5-gut          # SU(5): 3/8 at M_GUT; GQW misses 0.231 at M_Z
cargo run -p physis -- set su5-gut supersymmetric true   # GQW + unification fail → hold
cargo run -p physis -- score heterotic-e8e8
cargo run -p physis -- epistemics
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
