# physis

A pure-Rust workspace for **mechanically verifiable models of reality**.

Theories are objects. Knobs are typed. Claims return verdicts. Turning a knob is required to produce a **diff of behavioral state** — not a vibe.

```
cargo run -p physis -- experiment string-critique
cargo run -p physis -- set type-iib total_dim 9
```

The second command flips `consistency.critical-dimension` from **holds** to **fails**, and prints the judgment axis (`logical undetermined → logical disproved`). That is the point.

## Why this exists

Modern physics is a stack of layers — spacetime, quantum amplitudes, fields, particles, interactions — with a messy relationship between what is **theorem**, what is **encoded textbook fact**, what is **heuristic**, and what is **open**. Agents (and humans) are bad at keeping those categories apart when the model lives in slides and prose.

Rust is unusually good at this:

- the type system will not let you add kilograms to meters
- enums make illegal states unrepresentable
- a claim carries orthogonal assurance axes (`class`, `derivation`, `empirical`, `semantic`) as seriously as its verdict (`holds` vs `fails`)
- `executed` means the evaluator ran; it is not a kernel proof. `MachineProved` cannot be set as an enum.
- a lab journal is an append-only record of knob turns and verdict diffs

`physis` is a **foundational building block**. The flagship experiment is a laboratory for the public argument that string theory took fundamental physics into a landscape of untestable vacua (the family of critiques associated with, among others, Eric Weinstein). The *same* substrate now also hosts **electromagnetism, computation, thermodynamics, and quantum foundations** — not plugins bolted on, but new layers and theories on the same knobs-and-claims machine.

This repository does **not** decide whether string theory is false. It makes the distinctive structural claims of several theories *inspectable* and *comparable*, over long time horizons, by agents that can only act through a typed protocol.

## What's in the box

| Crate | Role |
|---|---|
| `physis-core` | SI dimensions, quantities, layers, knobs, claims, orthogonal assurance, content-addressed identity |
| `physis-model` | Spacetime, finite Hilbert space, SM spectrum, gauge groups, `World` |
| `physis-theory` | five domains on one substrate: fundamental physics (SM, GR, strings/M, observer-geometry), **electromagnetism**, **computation**, **thermodynamics**, and **quantum foundations** |
| `physis-proof` | trusted challenges and untrusted artifacts (no physics, no mint) |
| `physis-verifier` | the only crate that can mint `Verified<T>`; runs dual checkers |
| `physis-numeric` | exact ratios and interval enclosures |
| `physis-provenance` | source records; slogan locators are rejected |
| `physis-store` | content-addressed artifact DAG |
| `physis-data` | datasets and empirical receipts |
| `physis-ir` | declarative theory packages and constrained mutations |
| `physis-audit` | red-team corpus |
| `physis-semantic` | encoding review from evidence; no `Canonical` variant |
| `physis-constants` | versioned SI 2019 defining constants (`Ratio` / `SciExact` for `h`) and CODATA `G`/`mu0`/`epsilon0`/`Z0`/`alpha`/`inv_alpha`/`cRinf`/`hcRinf`/`Rinf`/`a0`/`Eh`/`me_mmu`/`m_p` (`Interval`); `physis constant [name]` rebuilds; omitted name rebuilds the full LEDGER; overlapping `physis_model` Qty floats lockstep the ledger (e/k via SI decimal, not reduced Ratio::to_f64) |
| `physis-agent` | Lab, protocol v2, hash-linked journal |
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

Read `class` and `derivation` before treating a cell as physics. A `holds` that is a `conjecture` is not the same object as a `holds` that is `executed` model-internal, and neither is a kernel-checked theorem.

## Five domains, one substrate

The same typed knob→verdict machine hosts five sciences (`physis experiments`):

| experiment | what it scrutinizes |
|---|---|
| `string-critique` | string constructions vs SM/GR/observer-geometry — predictivity and the "accommodate vs derive" critique (a `euler_number` knob makes three generations a *choice*, not a derivation) |
| `em-vacuum` | electromagnetism — `1/√(ε₀μ₀)=c` and the Maxwell equations as **computed theorems**; `add-monopole` / `add-proca` are IR, not constitutive knobs; a medium and a lumped circuit as effective limits (`add-tline` / `add-flux` are IR, not frequency knobs) |
| `computation` | a combinational circuit vs a Turing machine — the halting problem and P vs NP as honest `undecidable`/`open` |
| `field-modes` | a Klein–Gordon scalar and a 1D Dirac fermion on a lattice — computed dispersion, a tachyon from `mass² < 0`, unbounded minus-φ⁴ via `add-quartic`, naive doubling lifted by `add-wilson`, and next-nearest hopping via `add-next-nearest` |
| `gauge-lattice` | compact U(1) vs SU(2)/SU(3) — asymptotic freedom, a computed strong-coupling area law, and the 4D mass gap as a `conjecture` |
| `thermo` | a classical ideal gas — equipartition and the second law hold; the third law **fails honestly**; `add-bose` / `add-fermi` are IR, not temperature knobs |
| `blackbody` | cavity radiation — **Rayleigh–Jeans fails** finite energy, T⁴, and Wien's peak; Planck holds them; `set planck quantum false` restores the ultraviolet catastrophe |
| `solid` | lattice oscillators — **Dulong–Petit fails** the third law; Einstein holds it but **fails T³** (exponential freeze-out); Debye holds both the third law and `C_V ∝ T³` |
| `gravity` | solar-system gravity — **Newton fails** Eddington's 1.75″ and Mercury's 43″; GR holds them; Soldner's half-angle is the standing claim GR doubles; `add-r-squared` and `add-brans-dicke` are IR, not a `dim` knob |
| `olbers` | night sky — **static Euclidean fails** finite brightness and a dark sky (`F ∝ R`, `τ → ∞`); a finite-age horizon holds both; Hubble dimming is an independent flip |
| `bell` | a CHSH Bell test — **local realism is refuted** by a computed `S = 2√2 > 2`; `add-product` / `add-pr-box` are IR, not visibility knobs |

Domains also compose: `run landauer-engine` bridges computation and
thermodynamics — erasing a bit costs `k_B·T·ln2` as a typed `Qty<Energy>`
on the live encoding, `set landauer-engine reversible true` flips
`info.thermodynamically-free` `fails → holds`, and `hypothesize landauer-engine`
forks the bound (`add-kt` / `add-demon`) so `info.landauer-cost` `holds → fails`.

The `su5-gut` theory mechanizes "accommodate vs derive" with real stakes:
embedding one SM generation in a complete SU(5) multiplet **derives** charge
quantization (`Tr Q = 0`) and `sin²θ_W = 3/8` as computed theorems.
`hypothesize su5-gut` forks a missing 10 as IR, not the `supersymmetric` knob.
Running that `3/8` down to `M_Z` (Georgi–Quinn–Weinberg, `α_em` and `α_s` only)
**fails** for minimal SU(5) (`≈0.207` vs measured `0.231`) and holds for the
MSSM. Minimal SU(5) is honestly **falsified** — it `fails`
`gut.coupling-unification`, `gut.weinberg-angle-mz`, and
`gut.proton-decay-viable` (excluded by Super-Kamiokande), which a
`supersymmetric` knob revives as heuristics. The empirical sibling
`gut.proton-lifetime-sk` compares the dim-6 `M_GUT^4` scaling to the
registered Super-K `p→e+π0` Dataset (Takenaka et al. 2020): minimal SU(5)
is **excluded**; MSSM dim-6 is **compatible**. That is not P3N. The PDG
mixing-angle cell `gut.weinberg-angle-mz-interval` is a Gaussian NLL
(`statistical computed`) of the algebraic GQW `Ratio` centre versus
`σ = 10^{-5}` (the centre is rounded to that PDG scale for the
likelihood), still
not P3N; Super-K is not that Gaussian.

`physis epistemics` tallies the whole lab by class, derivation, and semantic
assurance (currently hundreds of `executed` model-internal evaluations).
`physis prove dec.d-squared-zero` mints a dual-checked receipt (Lean kernel
+ nanoda when those tools are installed, otherwise the exact expanders).
`physis why <claim>` prints assumptions, the statement hash, and the
receipt or `kernel proof: none`. `physis evidence <claim>` groups those
evaluations by statement hash: a shared slug is not one FormalClaim, and
confidence is a derived TrustProfile (not a numeric score). The command
inserts a content-addressed Evidence DAG; that snapshot is not
deserialized as authority. `physis --json <command>` emits the typed
matrices and verdict diffs for agents.

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
cargo run -p physis -- experiment solid
cargo run -p physis -- set einstein-solid temperature 4000
cargo run -p physis -- set einstein-solid spectrum debye   # T³ fails → holds
cargo run -p physis -- hypothesize debye-solid            # add-2d is IR, not set
cargo run -p physis -- hypothesize dulong-petit           # add-quartic is IR, not set
cargo run -p physis -- hypothesize heterotic-e8e8         # add-missing-e8 is IR, not set
cargo run -p physis -- hypothesize heterotic-so32         # add-so16 is IR, not set
cargo run -p physis -- hypothesize type-i                 # add-chan-paton-16 is IR, not set
cargo run -p physis -- hypothesize standard-model         # add-missing-eR is IR, not set
cargo run -p physis -- hypothesize observer-geometry      # add-missing-spin10 is IR, not set
cargo run -p physis -- experiment gravity
cargo run -p physis -- set general-relativity dim 5
cargo run -p physis -- experiment olbers
cargo run -p physis -- hypothesize olbers-static  # add-tired-light is IR, not set
cargo run -p physis -- set olbers-static finite_age true   # catastrophe fails → holds
cargo run -p physis -- experiment bell
cargo run -p physis -- run de-rham          # d²=0, Betti numbers; set shape disk/circle/torus/klein/sphere
cargo run -p physis -- run special-relativity   # invariants; then flip absolute_time
cargo run -p physis -- hypothesize special-relativity  # add-binomial-gamma and add-minus-uv are IR, not set
cargo run -p physis -- set special-relativity absolute_time true
cargo run -p physis -- hypothesize planck              # add-wien and add-zero-point are IR, not set
cargo run -p physis -- set planck quantum false   # ultraviolet catastrophe
cargo run -p physis -- run su5-gut          # SU(5): 3/8 at M_GUT; GQW misses 0.231 at M_Z
cargo run -p physis -- hypothesize su5-gut  # add-missing-10 is IR, not set
cargo run -p physis -- set su5-gut supersymmetric true   # GQW + unification fail → hold
cargo run -p physis -- score heterotic-e8e8
cargo run -p physis -- epistemics
cargo run -p physis -- why consistency.critical-dimension
cargo run -p physis -- prove dec.d-squared-zero
cargo run -p physis -- prove sr.invariant-interval
cargo run -p physis -- review dec.d-squared-zero
cargo run -p physis -- inspect origin fitted
cargo run -p physis -- inspect class conjecture
cargo run -p physis -- inspect gap missing-theorem
cargo run -p physis -- inspect judgment statistical-computed
cargo run -p physis -- inspect judgment empirical-excluded
cargo run -p physis -- why gut.weinberg-angle-mz-interval
cargo run -p physis -- why gut.proton-lifetime-sk
cargo run -p physis -- --role explorer prove dec.d-squared-zero   # refused
cargo run -p physis -- --role explorer score standard-model       # refused
cargo run -p physis -- --role empirical-analyst score standard-model
cargo run -p physis -- --role numerical-verifier enclose gut.weinberg-angle
cargo run -p physis -- --role explorer enclose gut.weinberg-angle   # refused
cargo run -p physis -- --role provenance-auditor cite gut.proton-lifetime-sk
cargo run -p physis -- --role reviewer cite gut.proton-lifetime-sk  # refused
cargo run -p physis -- --role formalizer formalize dec.d-squared-zero
cargo run -p physis -- prove dec.d-squared-zero
cargo run -p physis -- --role proof-searcher reproduce dec.d-squared-zero  # refused
cargo run -p physis -- --role replication-agent reproduce dec.d-squared-zero  # not P4
cargo run -p physis -- gaps
cargo run -p physis -- loop
cargo run -p physis -- falsify consistency.critical-dimension
cargo run -p physis -- hypothesize type-iib
cargo run -p physis -- hypothesize combinational-circuit
cargo run -p physis -- hypothesize turing-machine  # add-oracle is IR, not set
cargo run -p physis -- hypothesize olbers-static   # add-tired-light is IR, not set
cargo run -p physis -- hypothesize su5-gut         # add-missing-10 is IR, not set
cargo run -p physis -- hypothesize debye-solid     # add-2d is IR, not set
cargo run -p physis -- hypothesize dulong-petit    # add-quartic is IR, not set
cargo run -p physis -- hypothesize heterotic-e8e8  # add-missing-e8 is IR, not set
cargo run -p physis -- hypothesize heterotic-so32  # add-so16 is IR, not set
cargo run -p physis -- hypothesize type-i          # add-chan-paton-16 is IR, not set
cargo run -p physis -- hypothesize standard-model  # add-missing-eR is IR, not set
cargo run -p physis -- hypothesize observer-geometry  # add-missing-spin10 is IR, not set
cargo run -p physis -- evidence predictivity.unique-vacuum
cargo run -p physis -- enclose gut.weinberg-angle
cargo run -p physis -- cite gut.proton-lifetime-sk
cargo run -p physis -- sweep type-iib total_dim 8,9,10,11,12
cargo run -p physis -- audit
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

- Critical dimensions of strings (26 / 10 / 11) are **executed model-internal** claims, not kernel proofs. `physis why consistency.critical-dimension` prints `kernel proof: none`.
- SM embeddings into E₈×E₈, SO(32), SO(10), Spin(10) are **verified by code** — `GaugeGroup::verified_contains_sm` walks the standard maximal-subgroup chain and checks rank/dimension at each step — but remain **encoded facts**, not full root-system branching rules (necessary conditions + an encoded chain, not a proof). Full Dynkin branching is a planned milestone.
- Green–Schwarz anomaly cancellation is a **mechanical predicate** on the gauge group (dimension 496, exactly SO(32) or E₈×E₈), not a menu — see `GaugeGroup::gs_anomaly_free_10d`. It is an **encoded fact**, not a re-derivation of the anomaly polynomial (that is a later milestone).
- Landscape counts are **heuristics**. They exist so uniqueness can *flip* when fluxes and extra dimensions move — not because we computed 10⁵⁰⁰ Calabi–Yau flux vacua.
- Observer-geometry's gauge group is a **conjectural assignment**. The program is here as a contrast class, not as a completed theory.

## License

MIT. See `LICENSE`.
