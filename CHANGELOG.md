# Changelog

Every change to `physis` is atomic, committed directly to `main`, agentically
reviewed, and recorded here with its rationale and the verification that backs
it. This log is part of the contract: the process is meant to be as inspectable
as the physics.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
The project keeps `unsafe`-free pure Rust and honest epistemic tags.

## [Unreleased]

### M3 — Domain reuse

- **M3 Computation, the third domain** (`crates/physis-theory/src/computation.rs`,
  new `specs/009-computation.md`). A second reuse on the same substrate. New
  theories `combinational-circuit` and `turing-machine` (`tape_bound` knob) and
  the `physis experiment computation` matrix.
  - **The halting problem as an honest `Undecidable`:** the unbounded Turing
    machine's `comp.halts` is `undecidable` (Turing 1936); `comp.decidable-equivalence`
    is `undecidable` (Rice). Setting `tape_bound` to a finite value flips halts,
    turing-complete, decidable-equivalence, and resource-bounded — a finite
    automaton decides halting but loses Turing completeness.
  - Registered in `Lab::standard()`; `run`/`knobs`/`set` work from the CLI.
  - Documented rough edge: `World` is physics-shaped, so computational objects
    use a degenerate placeholder world (generalizing the projection is future
    work).

- **M3 Ohm-circuit control** (`crates/physis-theory/src/em.rs`). Added
  `ohm-circuit`, lumped circuit theory as the quasi-static effective limit of
  Maxwell, to the `em-vacuum` matrix (now three EM objects). Charge conservation
  is Kirchhoff's current law (`holds`), wave propagation is dropped
  (`em.wave-speed-c` inapplicable), and the theory has a preferred frame
  (`em.lorentz-invariance` fails). New `em.quasi-static-valid` claim: the
  `frequency_hz` knob flips it via typed `Qty<Length>` wavelengths when `c/f`
  stops dwarfing the circuit. Registered in `Lab::standard()`.

- **M3 Electromagnetism, the second domain** (`crates/physis-theory/src/em.rs`,
  `crates/physis-model/src/constants.rs`, new `specs/008-electromagnetism.md`).
  Proves the workspace hosts a second science without forking core: `Qty`,
  layers, knobs, claims, verdicts, the `Theory` trait, and the experiment matrix
  all carry classical electromagnetism.
  - New theories `maxwell-vacuum` and `linear-medium` (`ε_r`, `μ_r` knobs) and
    the `physis experiment em-vacuum` matrix.
  - **A real theorem:** typed `ε₀` and `μ₀` constants give `ε₀·μ₀·c² = 1` as a
    `Qty<Dimensionless>` that type-checks and evaluates to 1 — i.e.
    `1/√(ε₀μ₀) = c`, checked, not tabulated.
  - Knob → verdict: a linear medium with `n = √(ε_r μ_r) > 1` slows light and
    selects a rest frame, so `em.wave-speed-c` and `em.lorentz-invariance` fail;
    setting `epsilon_r = 1` flips them back to hold.
  - The experiment builder was generalized (`critique::report_from_rows`,
    `ExperimentReport` gained `rows`/`notes`) so each domain supplies its own
    theory list and claim rows — string-critique and em-vacuum share the
    machinery.

### M2 — Empirical contact

- **M2.5 Empirical target as data** (`data/empirical-world.json`,
  `crates/physis-theory/src/target.rs`, new `physis score` CLI verb). The
  low-energy requirements (observed 3+1, gauge ⊃ SM, chiral fermions, three
  generations, gravity) live in a checked-in JSON fixture parsed by serde, not a
  hand-written `empirical_target()` body. `score(target, theory)` grades any
  theory's projected `World`; `physis score <theory>` prints the scorecard.
  Honest, illustrative results: heterotic E₈×E₈ 5/5, Standard Model 4/5 (misses
  only gravity), Type IIB 4/5 (no perturbative SM gauge), GR 2/5. Tests cover
  each case. **M2 complete** (all five items, both "Done when" criteria).

- **M2.3 Coupling constants as typed quantities** (`crates/physis-model/src/constants.rs`).
  `fine_structure_constant` (α) and `strong_coupling_mz` (α_s) are
  `Qty<Dimensionless>`; `fermi_coupling` (G_F) is a typed `energy⁻²` quantity
  (SI J⁻²). The dimension is mechanically enforced: `G_F · E · E` type-checks to
  `Qty<Dimensionless>` by construction (test), and multiplying G_F by anything
  else is a compile error. Existence only — running the couplings is M4.

- **M2.2 Neutrino masses as a knob + claim** (`crates/physis-theory/src/standard_model.rs`).
  Added the `neutrino_masses` knob and the `empirical.neutrino-masses` claim.
  The minimal SM (default) now *fails* it honestly ("stores neutrino masses as
  0, but oscillations prove they are nonzero") instead of silently storing 0;
  setting the knob makes it hold. Addresses the `specs/002` "known lie". This
  claim lives on the Standard Model object (visible via `run standard-model`),
  not in the string-critique matrix rows. Knob-diff test included.

- **M2.1 SM embedding verified by code** (`crates/physis-model/src/gauge.rs`).
  `GaugeGroup::sm_embed` no longer asks "is this group literally SU(5)?" via an
  equality table. It now calls `verified_contains_sm`, which walks the standard
  maximal-subgroup chain (E₈ ⊃ E₆ ⊃ SO(10) ⊃ SU(5) ⊃ SM, plus SO(32) ⊃ SO(10)
  and Spin(10) ⊃ SU(5)) and checks the necessary rank and dimension
  inequalities at each step, backed by `SimpleGroup::dimension`/`rank`. The
  `empirical.sm-gauge` verdicts (strings + observer-geometry) now carry the
  verified chain as evidence. Retag policy honored: still `EncodedFact` (the
  check is necessary-but-not-sufficient and the chain is encoded), not
  `Theorem`. Satisfies M2's "SM ⊂ SU(5) verified by code" criterion. Tests in
  `gauge.rs` (chains verify, off-chain groups do not, monotonic rank/dim).

- **M2.4 Typed lengths for hidden extra dimensions** (`crates/physis-theory/src/strings.rs`).
  `empirical.hidden-extra-dims` no longer compares a raw float to a magic
  `1e16`. It now builds a typed effective radius `Qty<Length>` (Kähler volume ×
  √g_s × Planck length via `StringTheory::effective_radius`) and compares it to
  `Scale::Electroweak.typical_length()` — the shortest length we currently
  probe. The threshold is physics (an electroweak probe length), not a
  hand-tuned constant. Satisfies one of M2's "Done when" criteria.

### M1 — String lab

- **M1.5 Retire observer-geometry's magic 14** (`crates/physis-theory/src/geometry.rs`).
  The total dimension is now `observed_dim + fibre_dim` rather than a literal
  14. The `total_dim` knob is replaced by `fibre_dim` (default 10). The 10 is
  justified by a toy constraint: Spin(10) acts on a 10-dimensional space, so a
  fibre smaller than 10 cannot host the conjectured gauge group — setting
  `fibre_dim < 10` with `derive_gauge=true` flips `empirical.sm-gauge` to
  `fails`. So `14 = 4 + 10` is the minimal geometric carrier, not a magic
  number. (Knob rename: `observer-geometry total_dim` → `fibre_dim`.)

- **M1.4 Moduli as knobs** (`crates/physis-theory/src/strings.rs`). Added
  `dilaton` (string coupling g_s = e^φ) and heuristic moduli counts `h11`
  (Kähler / size) and `h21` (complex structure / shape). `unique-vacuum` now
  scales as `flux_bits × (h11 + h21)` — zeroing either the flux or the moduli
  collapses the landscape and restores uniqueness — and `hidden-extra-dims`
  uses the effective radius `compact_radius_planck · √g_s`, so the dilaton and
  the Kähler volume can both make extra dimensions visible. Constructors were
  refactored through a shared `StringTheory::new`. Knob→verdict-diff tests:
  moduli drive the landscape, zero flux restores uniqueness, and the
  Kähler volume + dilaton expose extra dimensions.

- **M1.3 Anomaly cancellation as a claim** (`crates/physis-model/src/gauge.rs`,
  `crates/physis-theory/src/{claims,strings,standard_model}.rs`). New matrix row
  `consistency.anomaly-cancellation`. The Green–Schwarz condition is a mechanical
  predicate — `GaugeGroup::gs_anomaly_free_10d`, backed by a real
  `GaugeGroup::dimension` computation — that holds for exactly SO(32) and E₈×E₈
  (dimension 496) and rejects a fake `SU(3)`/`E8`/SM gauge choice. So heterotic
  gauge groups are "not a menu; Green–Schwarz is the reason." Type II and
  M-theory hold for their own (non-GS) reasons; the SM holds (per-generation
  cancellation); the bosonic string is inapplicable (non-chiral); and off the
  critical dimension the claim is `undecidable`. Encoded as `EncodedFact`; a
  typed anomaly polynomial is deferred. Tests in `gauge.rs` (predicate),
  `strings.rs`, and `standard_model.rs`.

- **M1.2 More constructions first-class** (`crates/physis-theory/src/strings.rs`,
  `critique.rs`, `lab.rs`). Type I, Type IIA, heterotic SO(32), and M-theory are
  now constructed and registered in both the default lab and the `string-critique`
  matrix — ten objects in all. Their distinctive verdicts are pinned by tests:
  SO(32) constructions (Type I, heterotic SO(32)) carry an encoded SM embedding
  (`sm-gauge` holds); Type IIA and M-theory have no perturbative GUT group
  (`sm-gauge` undecidable); M-theory sits at critical dimension 11; and every
  default string construction fails `unique-vacuum` as a heuristic.

- **M1.1 Journal replay** (`crates/physis-agent/src/replay.rs`).
  Deterministic replay of a recorded JSONL journal onto a fresh
  `Lab::standard()`. `replay_journal` re-applies every `set-knob`, recomputes
  the verdict diffs, and checks them against what was recorded. A faithful
  replay is a mechanical proof of reproducibility; the CLI `physis replay
  <file.jsonl>` exits non-zero on any mismatch or failed turn.
  - New: `Journal::from_jsonl`, `ReplayReport`/`ReplayStep`, `Command::Replay`.
  - `VerdictDiff` gained `PartialEq`/`Eq` so recorded and recomputed diffs are
    comparable.
  - Tests: round-trip faithfulness, tamper detection, failed-turn reporting,
    empty-journal, and multi-run resume (`crates/physis-agent/src/replay.rs`).

- **M1.6 Session persistence** (`crates/physis/src/main.rs`).
  `physis --journal <file.jsonl>` records a session across process runs. On
  each run the lab loads the file and **restores prior state**
  (`Lab::restore_from_journal`) before applying the new turn, so a multi-run
  session is a single coherent, replayable session rather than a bag of
  independent one-shot diffs.

### Fixed

- **Journals were silently unreadable from disk.** Journal event timestamps
  were `u128`, and serde's internally tagged enum representation drops 128-bit
  integers on deserialization — so every event written by `Journal::file` was
  dropped when the file was reloaded. Timestamps are now `u64` Unix millis
  (range good for ~584 million years). Caught by the M1.1 replay round-trip
  test, not by inspection.
- **Journal events recorded `t: 0`.** `set-knob`/`run`/`experiment` events were
  stamped with `0` instead of the real time. They now use stamping constructors
  (`JournalEvent::set_knob` / `run` / `experiment`).

### Verification

Every change above is covered by `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, and
`cargo test --workspace` (all suites green — unit tests + doctests, including
the compile-fail proof that mass cannot be added to length and the replay
round-trip), plus the CLI record→replay loop demonstrated end-to-end.
Agentically reviewed with bugbot; the malformed/empty-journal certification
gap it flagged is fixed and covered by tests.

## Environment

- **Cloud Agent environment** (`.cursor/environment.json`). Repo-managed config
  for the pure-Rust workspace: toolchain from `rust-toolchain.toml`, `install`
  fetches and warm-builds all targets. Validated on a fresh build + fresh
  Cloud Agent.
