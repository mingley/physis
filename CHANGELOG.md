# Changelog

Every change to `physis` is atomic, committed directly to `main`, agentically
reviewed, and recorded here with its rationale and the verification that backs
it. This log is part of the contract: the process is meant to be as inspectable
as the physics.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
The project keeps `unsafe`-free pure Rust and honest epistemic tags.

## [Unreleased]

### Computed theorems

- **Einstein composition is a catalog identity**
  (`sr.subluminal-composition`, Physlib `subluminal_composition`).
  The polynomial `(1+uv)² − (u+v)² − (1−u²)(1−v²) ≡ 0` is dual-expanded
  and kernel-checked. `|w|<1` over ℝ remains the evaluator. Galilean
  addition is not an identity. The claim depends on
  `sr.invariant-interval`. Verified: expander and parse tests, exact
  mint, Lean+nanoda when wired, loop prove, `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **Trust gates reproduce and loop-review**
  (`Lab::exec` trust check). Role, then trust, then budget.
  `reproduce` requires P3F and does not spend prove budget on a
  refusal. The research loop will not raise P3S on an unproved catalog
  identity. Standalone `physis review` stays encoding-axis (orthogonal
  to kernel proof). Not P4. Verified: loop-review-requires-P3F,
  reproduce-does-not-spend-budget, `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Live lemma edges in the gap graph**
  (`Claim::depends_on`, `physis gaps`, `physis why`). Poincaré
  (`dec.closed-equals-exact`) records a live edge to `dec.d-squared-zero`.
  The edge is rebuilt from the theory encoding, never deserialized as
  authority, and is not part of the statement hash. Proving d² flips the
  edge to `have receipt`; Poincaré itself still `needs receipt`. Verified:
  statement-hash invariance unit test, gap-graph lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Failing evaluations are not missing theorems**
  (`gap_for`). `MissingTheorem` is only for evaluator-`Holds`
  mathematical / model-internal / phenomenological claims without a
  dual-checked receipt. Combinational `comp.turing-complete` Fails and
  leaves the gap graph; combinational `comp.halts` Holds and still
  `needs receipt` until proved. Verified: gap_for unit test, inspect
  and `physis gaps` lab tests, `fmt`, `clippy -D warnings`, full suite,
  CLI.

- **Live knowledge-gap graph**
  (`physis gaps`, `NodeKind::KnowledgeGap`). Rebuilt from current
  verdicts and receipts, content-addressed in the artifact DAG, never
  deserialized as authority. A proved catalog identity leaves
  `needs receipt`. Explorer may observe the graph. Verified: gap graph
  hash moves after prove, `fmt`, `clippy -D warnings`, full suite, CLI.

- **In-process reproduce is not P4**
  (`physis reproduce`). Requires a prior receipt, remints through
  `verify`, and matches challenge hash plus checkers. The output says
  `not P4 (same binary, same process)`. `inspect trust P4` stays 0.
  Explorer cannot reproduce. Verified: lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Agent roles propose; they do not mint**
  (`Role`, `ResearchBudget`, `physis formalize`). Explorer / formalizer /
  proof-searcher / falsifier / reviewer / auditor are processes that
  `exec` may refuse. `formalize` prints the catalog encoding as
  untrusted bytes and does not call `verify`. `--budget prove=N` is a
  research cap: a spent slot cannot mint. `loop` and `replay` stay
  lab-only. Journal restore reconstitutes as the lab, then the live
  command is role-gated. `loop` spends prove/review slots for each
  inner remint; a zero prove budget cannot mint through the cycle.
  P4 is still not assigned. Verified: role permit tests,
  explorer-does-not-mint, formalizer-without-receipt, spent-budget
  second prove, loop-respects-zero-prove-budget, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Chosen knobs vs measured ones, and inverse inspect**
  (`ParameterOrigin` on every `KnobSpec`, `physis inspect`). String
  `observed_dim` and SM `generations` are measured; compact radius and
  dilaton are fitted; `euler_number` stays chosen (accommodate, not
  derive). `physis knobs` prints the origin. `physis inspect
  trust|class|origin|gap <value>` lists matching claims or knobs. P3F
  is empty until `prove`. A proved catalog identity leaves the
  `missing-theorem` gap. Information-layer Undecidable (halting, Rice)
  is `logically-undecidable`; other undecidable evaluations are
  `unsupported-formal-primitive`. The TM equivalence claim lives on the
  information layer with halting (Rice). P vs NP stays a scientific open
  problem. CLI `loop` is wired (it was listed in usage but not parsed).
  Verified: origin unit tests, inspect lab test, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Derived trust profiles and axiom closure**
  (`physis-core` TrustProfile / Judgment projection, `AxiomLedger`
  defaults, `physis why` / `epistemics`). P3F is earned only by a
  dual-checked verifier receipt, P3S by encoding review, P0 by asserted
  conjectures. Evaluator `holds` is `logical undetermined`, not proved.
  A kernel proof that is still Unreviewed prints a danger note. P4 is
  not assigned from an in-process remint. `propose` cannot mark an axiom
  Accepted. Verified: compile-fail against TrustProfile literals and
  Deserialize, P3F-requires-receipt unit tests, why/epistemics CLI.

- **Level-3 Lean kernel + nanoda dual replay**
  (`formal/physlib`, `physis-proof` Physlib type matching, `physis-verifier`
  lake sandbox). `verify` on clean Lean source whose theorem type matches
  the trusted challenge compiles with Lean 4.34, exports that declaration
  with lean4export 3.1.0, and typechecks the export with nanoda 0.4.16.
  Vacuous `True` is `StatementMismatch`. Export bytes alone stay
  `LeanPipelineNotWired`. Receipts list `propext`, `Quot.sound`, and
  `Classical.choice`. `physis prove` prefers this backend when the tools
  are present; otherwise it still mints `ExactCertificate`. CI installs
  elan and lean4export. MSRV is 1.85 (nanoda / edition 2024). Verified:
  Physlib type match, dual-kernel mint for d² and the Lorentz interval,
  CLI `prove`, `fmt`, `clippy -D warnings`, full suite.

- **Level-3 semantic review and research loop**
  (`physis-semantic`, `physis-agent`, `physis-proof` infix parser).
  `physis review` raises `SemanticAssurance` only from a trusted dossier:
  a precise source record, a second IR encoding whose unexpanded tree
  matches the catalog identity (vacuous `0` is not `d² = 0`), and a
  passing red-team corpus. `Canonical` is never assigned. Journal restore
  re-runs review; it does not deserialize the tag. `physis loop` is one
  scheduled cycle: observe, hypothesize unproved catalog identities,
  prove, falsify, replicate receipts, design, audit, review. Sweep and
  sensitivity probes are not journaled. Lean kernel + nanoda replay is
  still unwired. Verified: compile-fail against SemanticRecord literals
  and Deserialize, vacuous-zero rejection, review restore, loop restores
  `total_dim`, journal tip test uses frozen timestamps (wall-clock `t` is
  in the Merkle payload), `fmt`, `clippy -D warnings`, full suite, CLI.

- **Level-3 dual-check receipts and protocol v2**
  (`physis-proof`, `physis-verifier`, `physis-audit`, `physis-numeric`,
  `physis-provenance`, `physis-store`, `physis-data`, `physis-ir`,
  `physis-constants`, `physis-agent`). `Verified` is not Deserialize, so
  JSON cannot forge a kernel proof. `verify` runs two independent expanders
  on catalogued identities (discrete d squared = 0, Lorentz interval) and
  refuses Lean source that contains axiom, sorry, or admit, or that lacks
  dual kernels. `physis prove` records a receipt; `physis why` then prints
  it. Conjectures cannot be proved by the exact backend. `physis falsify`,
  `sweep`, `branch`, `compare`, `design`, `sensitivity`, and `audit` are
  first-class ops. Provenance rejects textbook slogans. Intervals exclude
  SU(5) 3/8 from the MZ mixing-angle enclosure. The artifact DAG invalidates
  only descendants. Lean kernel + nanoda replay is typed but not wired.
  Verified: compile-fail against Deserialize of Verified, identity mutation
  and sorry/axiom rejection, red-team corpus, prove/why/falsify/sweep/audit
  lab tests, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Level-3 Milestone 1: theorem is no longer an enum**
  (`crates/physis-core` assurance/assumption/artifact/axiom/formal,
  `crates/physis-verifier`, `specs/020-proof-carrying.md`). Former
  `Epistemic::Theorem` is `ClaimClass::ModelInternal` (or mathematical /
  phenomenological) with `DerivationAssurance::Executed`. There is no
  `MachineProved` variant; `Verified<T>` can be minted only inside
  `physis-verifier` (`pub(crate)`). Every claim has a SHA-256 statement
  identity, a non-empty assumption set, and a domain of validity.
  Semantic assurance starts `Unreviewed`. `physis epistemics` prints
  derivation/class/semantic ledgers and an explicit zero kernel-proof
  row. `physis why` prints assumptions and `kernel proof: none`.
  Existing labs still evaluate; they are not silently promoted.
  Verified: compile-fail against `DerivationAssurance::MachineProved` and
  `Verified` struct literals, statement-hash sensitivity, axiom propose
  is unreviewed, no lab verdict is a kernel proof, `fmt`,
  `clippy -D warnings`, full suite, CLI `epistemics` / `why`.

- **Olbers' paradox: infinite static Euclidean starlight on trial**
  (`crates/physis-theory/src/olbers.rs`, `specs/019-olbers.md`, typed
  `Irradiance` / `LuminosityDensity` in `physis-core`). A standing
  19th-century cosmology (`olbers-static`) holds inverse-square shell
  cancellation (`dF = ρ_L dr`) and **fails** a finite sky and a dark night:
  the improper integral is `F ∝ R` (`F(2R)/F(R) = 2` at a Hubble-time probe,
  independent of cutoff) and `τ = n σ R → ∞`. A finite-age horizon
  (`olbers-horizon`) keeps cancellation and holds both observations
  (`F = ρ_L c t`, `τ ~ 10⁻¹⁵`). Hubble dimming is an independent knob:
  `dF = ρ_L dr / (1+z)²` saturates at `ρ_L c/H`, so cancellation fails and
  the sky stays dark. `set olbers-static finite_age true` flips the two
  catastrophe cells; `set olbers-horizon age_yr 1e26` makes `τ ≳ 1` (a
  merely finite universe is not automatically dark). Flux is
  `Qty<Irradiance>`, not energy. Linear Hubble `z = H r/c` is not a full
  FLRW integral; `ρ_L` is a cosmic mean, not the solar neighbourhood packed
  to infinity. Verified: shell ratio 1 vs Hubble fall-off, flux doubling,
  expanding saturation ≈ 2 (static 100), Hubble-time `τ ~ 10⁻¹⁵`, ancient
  `τ ≳ 1`, cutoff independence, typed `ρ_L × length`, lab knob-diffs,
  `fmt`, `clippy -D warnings`, full suite, and the CLI experiment. Bugbot
  follow-up: `note()` and `astro.night-sky-dark` share `verdict_radius`
  (`c t` or `c/H`, never the cutoff), and `τ = n σ R` is a typed
  `Qty<Dimensionless>` rather than an `f64` after `.value()`.

- **Rayleigh–Jeans vs Planck: the ultraviolet catastrophe as a computed theorem**
  (`crates/physis-theory/src/blackbody.rs`, `specs/016-blackbody.md`, typed
  `EnergyDensity` / `StefanBoltzmann` in `physis-core`). Classical equipartition
  of electromagnetic cavity modes is a first-class theory (`rayleigh-jeans`):
  it *holds* `thermo.mode-equipartition` and **fails** finite energy, the T⁴
  law, and Wien's peak — `u(2ν_max)/u(ν_max) = 8` and `u(2T)/u(T) = 2`, not 16.
  Planck's Bose occupation (`planck`) reverses the matrix: UV modes freeze out,
  `u = a T⁴` matches the typed Stefan–Boltzmann constant derived from exact SI
  `h`, `k_B`, `c`, and `λ_max T` matches `hc/(k x)`. `set planck quantum false`
  restores the catastrophe. Verified: Bose integral `π⁴/15`, numeric `u` vs
  analytic `aT⁴`, RJ octupling, Wien temperature-independence, lab knob-diff,
  `fmt`, `clippy -D warnings`, full suite, and the CLI experiment. Bugbot
  follow-up: Planck's `uv-finite` / `stefan-boltzmann` verdicts use the
  improper integral `u_∞ = a T⁴` (not the current cutoff, which can sit in
  the infrared), and Wien's classical failure is a sampled absence of an
  interior peak.

- **Georgi–Quinn–Weinberg: run 3/8 down to M_Z**
  (`crates/physis-theory/src/rge.rs`, `crates/physis-theory/src/gut.rs`,
  `specs/013-grand-unification.md`). The GUT-scale theorem `sin²θ_W = 3/8` is
  a boundary condition. Predicting `sin²θ_W(M_Z)` from `α_em` and `α_s`
  (no measured mixing angle) is a one-loop computation:
  minimal SU(5) lands at ≈0.207 and **fails** `gut.weinberg-angle-mz`; the
  MSSM lands on 0.231 at `M_U ≈ 2×10¹⁶ GeV` and holds as a heuristic.
  `set su5-gut supersymmetric true` flips that cell with unification and
  proton decay. The 3/8 hold evidence no longer quotes the M_Z measurement.
  Verified: SM GQW ≈0.207 vs 0.231, MSSM match, lab knob-diff, `fmt`,
  `clippy -D warnings`, full suite, and `run su5-gut`.

- **2-sphere: χ = 2, b₂ = 1, Poincaré still holds**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). The
  boundary of a tetrahedron is a first-class `sphere` shape: 4 vertices, 6
  edges, 4 triangles, every edge bordering two faces. It shares `b₁ = 0`
  with the disk (closed = exact) but **holds** `dec.fundamental-class`
  (`b₂ = 1`, `χ = 2`) which the disk, circle, and Klein bottle fail. `set
  de-rham shape sphere` flips that claim fails → holds without touching
  Poincaré — the homology contrast "a 2-cycle, not a 1-hole." Verified:
  tetrahedron Betti numbers and closed-surface check, disk vs S² share b₁
  but not b₂/χ, lab knob-diff, `fmt`, `clippy -D warnings`, full suite, and
  `run de-rham` / `set de-rham shape sphere`.

- **Einstein vs Debye: exponential freeze-out on trial against T³**
  (`crates/physis-theory/src/solid.rs`, `specs/017-einstein-solid.md`, typed
  `HeatCapacity` in `physis-core`). Einstein's 1907 Bose oscillators hold the
  third law but **fail** the observed low-T phonon law: `C_V(2T)/C_V(T)` at
  `Θ/20` is exponential (≫ 8), not 8. Debye's 1912 `ω²` density of states
  (`debye-solid`) reverses that cell: the improper Bose integrals recover
  `π⁴/15` and `4π⁴/15`, and `C_V = (12/5) π⁴ N k (T/Θ_D)³` is a sampled
  theorem. `set einstein-solid spectrum debye` flips `thermo.debye-t3`
  fails → holds without restoring Dulong–Petit. `C_V` is `Qty<HeatCapacity>`,
  not energy. The `thermo` experiment now shares the third-law row across
  ideal gas, Dulong–Petit, Einstein, and Debye. Verified: Debye integrals vs
  `π⁴/15` and `4π⁴/15`, C_V vs dU/dT, Einstein over-freeze at T/Θ=0.2,
  spectrum knob-diff, `fmt`, `clippy -D warnings`, full suite, and the CLI
  experiment.

- **Dulong–Petit vs Einstein: classical solid heat capacity on trial**
  (`crates/physis-theory/src/solid.rs`, `specs/017-einstein-solid.md`). The
  1819 standing theory `C_V = 3 N k` independent of T is a first-class object
  (`dulong-petit`): it holds that axiom and **fails** the third law. Einstein's
  Bose oscillators (`einstein-solid`) reverse the matrix at `T/Θ_E = 0.2`:
  `C_V/(3Nk) ≈ 0.17`, `C_V(Θ_E/40) → 0`. Raising `temperature` to 4000 K
  recovers Dulong–Petit as correspondence without resurrecting the third-law
  failure. The `thermo` experiment now shares the third-law row across ideal
  gas, Dulong–Petit, and Einstein.

- **Newton vs Einstein: Eddington 1.75″ and Mercury 43″ as computed theorems**
  (`crates/physis-theory/src/gravity.rs`, `specs/018-light-deflection.md`).
  Inverse-square gravity is a first-class theory (`newtonian-gravity`): RK4 on
  the Binet equation holds Soldner's `2 GM/(c² R) ≈ 0.87″` and a closed
  Mercury ellipse, and **fails** Eddington's 1.75″ and the 43″/century
  remainder. Schwarzschild geodesics reverse the matrix (`u'' + u = 3 (GM/c²)
  u²` for light; Kepler plus that term for Mercury). `GM/c²` is a typed
  length from the IAU solar `GM`. `set general-relativity dim 5` makes the
  4D solar tests inapplicable. Bugbot follow-up: Mercury's RK4 now starts at
  the Kepler perihelion `u = 1/(a(1−e))`, not `(1+e)/a`.

- **Two-loop RG running for gauge-coupling unification**
  (`crates/physis-theory/src/rge.rs`, `crates/physis-theory/src/gut.rs`,
  `specs/013-grand-unification.md`). `GaugeRunning` now integrates the *coupled
  two-loop* RGEs `d(α_i⁻¹)/dt = −b_i/2π − (1/8π²)·Σ_j b_ij α_j` with a hand-rolled
  RK4 stepper (the standard gauge two-loop matrices `b_ij` for the SM and MSSM),
  finds the `α_1⁻¹ = α_2⁻¹` crossing, and reports the residual `α_3⁻¹` gap and the
  two-loop `M_GUT`. The `gut.coupling-unification` verdict now shows both loops:
  minimal SU(5) misses (≈12% gap at `M_GUT ≈ 10¹³ GeV`), the MSSM meets to a few
  percent at `M_GUT ≈ 3×10¹⁶ GeV`. Verified: two new tests (MSSM unifies far
  better than the SM at two loops; two-loop scale refines the one-loop estimate),
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Klein bottle: non-orientability and torsion-invisibility over ℝ**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Added
  `Complex::klein_bottle()` (a 4×4 grid glued into a torus one way and with a flip
  the other) and a `klein` option to the `shape` knob. It is the sharpest
  homology contrast in the lab: same Euler characteristic as the torus (`χ = 0`)
  but `b₁ = 1` (not 2 — the `ℤ/2` torsion in `H₁(K;ℤ)` is invisible to real
  coefficients) and `b₂ = 0` (not 1 — non-orientable surfaces have no fundamental
  class over ℝ). Added `Complex::is_closed_surface()` (every edge borders exactly
  two triangles) as a validity check for the surface constructions. Verified:
  two new tests (Klein real homology + closed-surface check, and via the knob),
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Exact 2D SU(N) confinement from the quadratic Casimir**
  (`crates/physis-theory/src/gauge_field.rs`, `specs/010-continuum.md`). Extended
  `gauge.exact-area-law-2d` to the non-abelian `wilson-su2`/`wilson-su3`
  theories. Two-dimensional Yang–Mills is exactly solvable, so the fundamental
  Wilson loop has string tension `σ = (g²/2)·C₂(fund) = (N²−1)/(2β)` (from the
  quadratic Casimir `C₂(fund) = (N²−1)/(2N)`), positive at every finite `β` — 2D
  SU(N) confines at *all* couplings, a **theorem**, in pointed contrast to the 4D
  mass gap, which stays a `conjecture`. `set wilson-su3 dimension 2` flips the
  claim `inapplicable → holds` (σ = 2/3 at β=6). Added `su_casimir_fundamental`
  and `exact_2d_string_tension_sun`. Verified: two new tests (SU(2)/SU(3) confine
  for β ∈ [0.5, 50], Casimir values, 2D-only applicability), `fmt`,
  `clippy -D warnings`, full suite, and the CLI knob diff.

- **Torus homology: a non-trivial `b₁ = 2` check for the DEC machinery**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Added
  `Complex::torus()` — a triangulated 3×3 flat torus (9 vertices, 27 edges, 18
  triangles) — and generalized the `de-rham` knob from a `filled` bool to a
  `shape` **choice** (`disk`/`circle`/`torus`, exercising the `Choice` knob
  domain). The torus is a genuine stress test of the Betti/Hodge code beyond the
  minimal disk/circle: all invariants come out to the textbook values `b₀ = 1`,
  `b₁ = 2`, `b₂ = 1`, `χ = 0`, with the Hodge Laplacian's harmonic dimension
  matching `b₁ = 2`. `set de-rham shape torus` flips `closed-equals-exact` to
  `fails` (two independent 1-cycles). Verified: two new tests (torus invariants,
  torus via the knob incl. rejecting an unknown shape) plus updated knob tests,
  `fmt`, `clippy -D warnings`, full suite, and the CLI knob diff.

- **Euler–Poincaré and Hodge theorems on the de Rham complex**
  (`crates/physis-theory/src/dec.rs`, `specs/015-exterior-calculus.md`). Two more
  computed theorems on `de-rham`, each cross-checking a classical invariant two
  independent ways: `dec.euler-poincare` confirms `χ = V−E+F = b₀−b₁+b₂` (Euler
  characteristic from cell counts vs. from Betti numbers), and
  `dec.hodge-harmonic` confirms `dim(harmonic 1-forms) = b₁` by computing the
  nullity of the combinatorial Hodge Laplacian `Δ₁ = d₀d₀ᵀ + d₁ᵀd₁` and matching
  it to the first Betti number (harmonic representatives ≅ cohomology). Added
  `transpose`/`matmul`/`matadd` helpers and `betti2`/`euler_from_cells`/
  `euler_from_betti`/`harmonic1_dim` to `Complex`. Disk: `χ=1`, harmonic dim `0`;
  circle: `χ=0`, harmonic dim `1`. Verified: four new tests, `fmt`,
  `clippy -D warnings`, full suite, and `run de-rham` across the `filled` knob.

- **Discrete exterior calculus: `d²=0` and Betti numbers with type-level grade**
  (new `crates/physis-theory/src/dec.rs`, new `specs/015-exterior-calculus.md`).
  New `de-rham` theory on the mathematical layer. Differential-form grade is a
  Rust type parameter (`Cochain<G0/G1/G2>`), so the exterior derivative `d`
  provably raises grade by one and mixing grades is a compile error (guarded by a
  `compile_fail` doctest, mirroring the `Qty<D>` contracts). Three computed
  theorems: `dec.d-squared-zero` (`d₁∘d₀ = 0` exactly — the `curl grad = 0` /
  `dF=0`-from-`F=dA` identity), `dec.first-betti-number` (holes counted as
  `n_edges − rank(d₁) − rank(d₀)` via Gaussian elimination), and
  `dec.closed-equals-exact` (the Poincaré lemma). A `filled` knob removes the
  triangle's face, turning the disk into a circle: `b₁` jumps `0 → 1` and
  `closed-equals-exact` flips `holds → fails` — topology detected mechanically by
  linear algebra on the coboundary. Registered in `Lab::standard()`. Verified:
  five new tests, a compile-fail doctest, `fmt`, `clippy -D warnings`, full
  suite, and the `run de-rham` + knob diff.

- **CHSH correlator derived from the two-qubit operators**
  (`crates/physis-model/src/quantum.rs`, `crates/physis-theory/src/quantum.rs`,
  `specs/012-quantum-foundations.md`). Added `spin_measurement(θ) = cos θ·σ_z +
  sin θ·σ_x`, a Kronecker product `tensor2`, a 4×4 `apply_mat4`, and
  `expectation4` to `physis-model`. The Bell test's correlator `E(a,b) =
  −cos(a−b)` is now the genuine operator expectation `⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩`,
  verified against the closed form by a new `quantum.correlator-from-operators`
  theorem — so the quantum prediction *emerges from the formalism* rather than
  being an assumed cosine. The CHSH angle convention was updated accordingly
  `(0, 90°, 45°, 135°)`, still saturating `2√2`. Verified: three new
  `physis-model` tests (spin eigenvalues ±1, singlet correlator = −cos Δ) and a
  `physis-theory` test, `fmt`, `clippy -D warnings`, full suite, and `run
  bell-test` (now five holding theorems).

- **CHSH bounds derived, not asserted (Tsirelson by maximization, classical by
  enumeration)** (`crates/physis-theory/src/quantum.rs`,
  `specs/012-quantum-foundations.md`). `quantum.tsirelson-bound` is now computed:
  a brute-force maximization of `|S|` over a 90³ grid of measurement angles finds
  `|S|max ≈ 2.827`, confirming no quantum strategy exceeds `2√2`. A new
  `quantum.local-realism-bound` claim *derives* the classical CHSH bound by
  enumerating all `2⁴` deterministic ±1 strategies and finding the maximum is
  exactly 2 — the threshold falls out of the model rather than being asserted.
  Together they mechanize why `2 < S ≤ 2√2` is the signature of quantum
  nonlocality. Verified: three new tests (Tsirelson maximization, classical
  enumeration, quantum-beats-classical), `fmt`, `clippy -D warnings`, full suite,
  and the `bell` experiment matrix (now four rows).

- **Exact 2D lattice-gauge confinement from the Bessel-function ratio**
  (`crates/physis-theory/src/gauge_field.rs`, `specs/010-continuum.md`). New
  `gauge.exact-area-law-2d` claim on `wilson-u1`: in two dimensions the gauge
  integral factorizes plaquette by plaquette, so the Wilson loop is *exactly*
  `⟨W⟩ = (I₁(β)/I₀(β))^Area` with string tension `σ = −ln(I₁(β)/I₀(β))`. Since
  `0 < I₁/I₀ < 1` for every finite `β`, `σ > 0` always — 2D compact U(1) confines
  at **all** couplings, a theorem (not the strong-coupling approximation). The
  modified Bessel ratio is computed by a convergent, overflow-free series
  (`bessel_i1_over_i0`); the claim is `inapplicable` in D > 2, honestly leaving
  4D as the open mass-gap problem. `set wilson-u1 dimension 2` flips it
  `inapplicable → holds` (σ = 0.807 at β=1, 0.053 at β=10, both > 0). Verified:
  four new tests (Bessel values, 2D confinement at β ∈ [0.1, 50], 2D-only
  applicability, monotone tension), `fmt`, `clippy -D warnings`, full suite, and
  the CLI knob diff.

- **Special relativity: the Galilean→Einstein revolution as one knob**
  (new `crates/physis-theory/src/special_relativity.rs`, new
  `specs/014-special-relativity.md`). New `special-relativity` theory with three
  computed theorems — `sr.invariant-interval` (`s² = (cΔt)² − Δx²` unchanged by
  a boost), `sr.subluminal-composition` (`0.8c ⊕ 0.7c ≈ 0.9615c < c`), and
  `sr.energy-momentum-invariant` (`E² − (pc)² = (mc²)²`, with `pc` and `mc²`
  built from *typed* `Qty<Energy>` so the dimensions are compiler-checked). An
  `absolute_time` knob replaces Lorentz boosts with Galilean ones and flips all
  three `holds → fails` at once — the pre-1905 worldview, mechanized. Added a
  `Momentum` (`M L T⁻¹`) type alias to `physis-core`. Registered in
  `Lab::standard()`. Verified: four new tests, `fmt`, `clippy -D warnings`, full
  suite, and the `run special-relativity` + knob diff.

- **Gauge-coupling unification computed by one-loop RG running**
  (new `crates/physis-theory/src/rge.rs`, `specs/013-grand-unification.md`).
  `gut.coupling-unification` is no longer an asserted sentence: `GaugeRunning`
  runs the three inverse couplings `α_i⁻¹(μ) = α_i⁻¹(M_Z) − (b_i/2π)·ln(μ/M_Z)`
  from the measured electroweak inputs at `M_Z`, fixes the unification point
  from the `α_1`/`α_2` crossing, and **predicts** `α_3(M_Z)`. The minimal SM
  misses by ~40% (`M_GUT ≈ 10¹³ GeV`); the MSSM agrees to ~1% (`M_GUT ≈
  2×10¹⁶ GeV`) — the celebrated near-success, now computed. The
  `supersymmetric` knob switches the beta coefficients `(41/10,−19/6,−7) →
  (33/5,1,−3)`, flipping the verdict `fails → holds`; the same computed `M_GUT`
  feeds the proton-decay verdict (rate ∝ `M_GUT⁻⁴`). Verdicts stay `Heuristic`
  (one loop is approximate) but carry the genuinely computed numbers as
  evidence. New PDG constants `inverse_alpha_em_mz`, `weak_mixing_angle_sin2_mz`,
  `z_mass_gev`. Verified: four new tests (SM misses, MSSM unifies, SUSY beats
  SM, verdict carries computed numbers), `fmt`, `clippy -D warnings`, full
  suite, and the `run su5-gut` + knob diff.

- **SU(5) grand unification: `sin²θ_W = 3/8` and charge quantization derived**
  (new `crates/physis-theory/src/gut.rs`, new `specs/013-grand-unification.md`).
  New `su5-gut` theory (Georgi–Glashow SU(5)) one layer above the SM. Two
  computed theorems from embedding one generation in a complete SU(5) multiplet,
  both from the same `SM_WEYL_FIELDS` table the anomalies use: `Tr Q = 0` forces
  charge quantization (`gut.charge-quantization`), and `sin²θ_W = Tr(T₃²)/Tr(Q²)
  = 3/8` at the unification scale (`gut.weinberg-angle`, with an honesty note
  that the measured `M_Z` value differs by RG running). It is also honest about
  failure: minimal (non-SUSY) SU(5) `fails` both `gut.coupling-unification` and
  `gut.proton-decay-viable` (excluded by Super-Kamiokande), and a
  `supersymmetric` knob flips both `fails → holds` as heuristics. New helpers
  `gut_weinberg_sin2` / `gut_trace_charge` in `standard_model.rs`; registered in
  `Lab::standard()`. Verified: four new tests, `fmt`, `clippy -D warnings`, full
  suite, and the `run su5-gut` + SUSY knob diff.

- **Standard Model hypercharges *derived* from anomaly cancellation**
  (`crates/physis-theory/src/standard_model.rs`, `specs/005-string-critique.md`).
  New `sm.hypercharge-derivation` claim (computed `theorem`): fixing only the
  normalization `Y_Q = 1/6`, the code solves the four anomaly conditions and
  recovers every hypercharge — `Y_L = −1/2`, `Y_e = 1`, and the `[U(1)]³` cubic
  forces `{Y_u, Y_d} = {−2/3, 1/3}` (`StandardModel::derive_hypercharges`). The
  charges are a *consequence* of consistency, not an input — the mechanized form
  of "accommodate vs derive". The fermion content was refactored into a richer
  `WeylField` table (separate SU(3)/SU(2) dimensions), and
  `consistency.anomaly-cancellation` was strengthened to check **all four**
  gauge anomalies (`[SU(3)]²U(1)`, `[SU(2)]²U(1)`, `[grav]²U(1)`, `[U(1)]³`)
  rather than only the two hypercharge sums. Verified: three new tests
  (all-four-anomalies, hypercharges-derived, derivation-claim-holds), `fmt`,
  `clippy -D warnings`, full suite, and `run standard-model`.

### Domain reuse

- **Landauer's principle: a computation ↔ thermodynamics bridge**
  (`crates/physis-theory/src/computation.rs`, `specs/009-computation.md`).
  New `landauer-engine` theory — the first object that reuses substrate from two
  domains at once. Erasing a logical bit dissipates at least `k_B·T·ln2` of
  energy (Landauer 1961); a logically reversible computation erases nothing and
  can be free (Bennett 1973). The bound is **computed from the typed Boltzmann
  constant**, so its units are checked at compile time: `k_boltzmann()` (J/K) ×
  `kelvin(T)` (K) × `N·ln2` is a `Qty<Energy>`. `info.landauer-cost` holds as a
  theorem with the computed floor as evidence (one bit at 300 K = `2.871e-21 J`),
  and `set landauer-engine reversible true` flips `info.thermodynamically-free`
  `fails → holds` — a cross-domain knob → verdict diff. Knobs: `temperature_k`,
  `bits_erased`, `reversible`. Registered in `Lab::standard()`. Verified: five
  new tests (typed-energy bound, erasure forces dissipation, reversibility knob
  removes it, linear scaling in bits and temperature), `fmt`, `clippy -D
  warnings`, full workspace suite, and the CLI knob diff.

- **Quantum foundations: a fifth domain (CHSH Bell test)**
  (`crates/physis-theory/src/quantum.rs`, new `specs/012-quantum-foundations.md`).
  New `bell-test` theory and `physis experiment bell`, giving the `quantum` layer
  its first `Theory`. Puts local realism on trial and mechanically refutes it:
  the CHSH correlator `S = |E(a,b) − E(a,b′) + E(a′,b) + E(a′,b′)|` with
  `E(a,b) = −V·cos(2(a−b))` and the optimal angles computes to `V·2√2`, so at full
  visibility `quantum.bell-violation` holds (`S = 2√2 > 2`, refuting local hidden
  variables) while `quantum.tsirelson-bound` holds (`S ≤ 2√2`) and
  `quantum.born-normalization` is checked from the singlet ket. A `visibility`
  knob turns the violation off below `1/√2`. Registered in `Lab::standard()` and
  the experiments list.

- **Thermodynamics: a fourth domain on the statistical layer**
  (`crates/physis-theory/src/thermo.rs`, new `specs/011-thermodynamics.md`).
  New `ideal-gas` theory (monatomic classical ideal gas) and `physis experiment
  thermo`, populating the previously-empty `statistical` layer. Exercises the
  type system on `Qty<Temperature>` vs `Qty<Energy>` (`k_B·T` is an energy by
  construction). Claims: `thermo.equipartition` (computed `C_v = dU/dT =
  (3/2)Nk`), `thermo.second-law` (computed `ΔS = Nk ln(V_f/V_i) ≥ 0`,
  knob-sensitive — a compression flips it to `fails`), and `thermo.third-law`
  which **fails honestly**: a classical ideal gas has `S ∝ ln T → −∞`, so it
  cannot satisfy the third law without quantum statistics. Adds
  `k_boltzmann` to `physis-model::constants`. Registered in `Lab::standard()`
  and the experiments list.

### Type system

- **More compile-fail contracts** (`crates/physis-core/src/lib.rs`). Added two
  `compile_fail` doctests to the "illegal states are unrepresentable" proof set:
  subtracting a length from an energy, and assigning `mass × time` to a
  `Qty<Energy>`. With the original mass+length example, the type system's
  dimensional safety is now proven by three compile-fail contracts.

### Documentation

- **README refresh for five domains.** The top-level README now presents physis
  as a five-domain laboratory (fundamental physics, electromagnetism,
  computation, thermodynamics, quantum foundations) with a "Five domains, one
  substrate" table of the seven experiments and what each scrutinizes, plus the
  `epistemics` ledger and `--json` structured output — while keeping
  string-critique as the flagship.

### Tooling

- **`physis epistemics` knowledge ledger** (`crates/physis-agent`, `crates/physis`).
  Tallies every verdict across all lab theories by epistemic tag
  (theorem / encoded-fact / conjecture / heuristic / open) and verdict kind — the
  mission's core metric, mechanically counted. Current state: 54 theorems, 75
  encoded-facts, 13 conjectures, 21 heuristics, 11 open (all 11 honestly
  `undecidable`), over 174 claim-evaluations. Composes with `--json`; tested.

- **`--json` structured output** (`crates/physis`). A global `--json` flag makes
  the CLI emit the full typed `Response` as JSON — status, text, and the
  structured `report` (claim matrix) / `diffs` (verdict changes) — so a
  long-horizon agent consumes typed data instead of parsing prose. Works with
  any command and composes with `--journal`. A test asserts the response
  serializes with the matrix and knob-diff content.

- **`physis experiments` command** (`crates/physis-agent`, `crates/physis`). Lists
  the five available experiments (string-critique, em-vacuum, computation,
  field-modes, gauge-lattice) with one-line descriptions, so the growing set of
  labs is discoverable without reading the source. A consistency test asserts
  every listed experiment actually runs.

### M4 — Continuum

- **Charge conservation backed by a computed identity** (`crates/physis-theory/src/em.rs`).
  `em.charge-conservation` (already a theorem) now carries computed evidence: the
  vector-calculus identity `∇·(∇×A) = 0` — the mechanism behind the continuity
  equation `∂ρ/∂t + ∇·J = 0` — is verified numerically (4-point mixed-partial
  stencil) to residual ≈ 0. Test included.

- **Gauss's law verified on a Coulomb field** (`crates/physis-theory/src/em.rs`).
  `em.gauss` is now a **computed theorem** in vacuum: a Coulomb field `E = r̂/r²`
  is checked by central finite differences to have `∇·E = 0` away from the source
  (residual ≲ 1e-4). With this, all three vacuum Maxwell laws (`gauss`,
  `faraday`, `ampere`) are computed theorems; a medium keeps the encoded-fact
  macroscopic forms. Test asserts the residual and both the vacuum-theorem and
  medium-encoded-fact tags.

- **Maxwell homogeneous equations verified numerically** (`crates/physis-theory/src/em.rs`).
  `em.faraday` and `em.ampere` are now **computed theorems** in vacuum: a plane
  wave `E = ŷ cos(x−t)`, `B = ẑ cos(x−t)` is checked by central finite differences
  to satisfy `∂B/∂t + ∇×E = 0` and `∂E/∂t − ∇×B = 0` to residual ≲ 1e-6,
  promoting them from encoded facts. In a medium they revert to encoded facts
  (macroscopic form). Tests assert the residuals and the vacuum epistemic tag.

- **M4 computed strong-coupling area law** (`crates/physis-theory/src/gauge_field.rs`).
  New `gauge.strong-coupling-area-law` claim backed by a real computation: the
  leading strong-coupling string tension `σ = −ln(β/2N²)` (first term of the
  convergent Wilson-loop expansion) for both compact U(1) and SU(N). `σ > 0` is a
  genuine area-law theorem; it fails once the coupling is too weak for the
  expansion. This is the *computed* companion to the physical (heuristic/
  conjecture) `gauge.confining` verdict. Knob-sensitive: `set wilson-su3 beta
  100` flips it holds→fails. Tests check the knob diff and the closed form.

- **M4 second-order accuracy, computed** (`crates/physis-theory/src/continuum.rs`).
  New `field.second-order-accurate` claim for `klein-gordon`: the empirical
  convergence order `p = log2(err(a)/err(a/2))` of the discrete Laplacian at a
  fixed physical wavenumber is *computed* and verified to be ≈ 2 (error ∝ a²).
  This promotes the continuum limit from asserted to a computed numerical-order
  theorem. `set klein-gordon spacing 100` leaves the second-order regime and the
  claim fails. Tests included.

- **M4 non-abelian gauge fields (QED vs QCD)** (`crates/physis-theory/src/gauge_field.rs`).
  New `WilsonSun` theories `wilson-su2` and `wilson-su3` contrast with compact
  U(1) in the `gauge-lattice` matrix. New `gauge.asymptotic-freedom` claim: U(1)
  `fails` (Landau pole), SU(N) `holds` (Gross–Wilczek–Politzer). SU(N) 4D
  confinement `holds` as a **conjecture** — the Yang–Mills mass-gap Millennium
  Problem — while U(1) deconfines in 4D above β≈1.01. Registered in
  `Lab::standard()`; tests cover the asymptotic-freedom contrast and the
  conjecture tag.

- **Architecture: `Theory::world()` returns `Option<World>`** (`framework.rs` and
  all theories). Non-physics domains no longer borrow a physics-shaped
  spacetime. Computation (`combinational-circuit`, `turing-machine`) returns
  `None` and describes itself via the new `Theory::note()`; the scalar field
  reports an honest 1+1 D world instead of 3+1 Minkowski. `physis score` now
  reports a non-physics theory as such rather than faking a physics score, and
  `critique::report_of` handles the optional world. Removes the placeholder-world
  rough edge documented in `specs/009`/`specs/010`. All existing tests stay
  green.

- **M4 gauge field on links** (`crates/physis-theory/src/gauge_field.rs`,
  `specs/010`, `plans/005`). New theory `wilson-u1`: compact U(1) lattice gauge
  theory whose degrees of freedom live on links, with the Wilson plaquette
  action. `gauge.invariant` and `gauge.local` are structural theorems;
  `gauge.confining` is a theorem (encoded) at all β in 2D/3D and a knob-sensitive
  heuristic across the 4D transition near β ≈ 1.01. New `physis experiment
  gauge-lattice`; registered in `Lab::standard()`. `set wilson-u1 beta 2`
  deconfines the 4D theory (Coulomb phase); `set wilson-u1 dimension 3` confines
  at any β. Tests cover both.

- **M4 seed: a field as an actual local object** (`crates/physis-theory/src/continuum.rs`,
  new `specs/010-continuum.md`, `plans/005-m4-continuum.md`). `klein-gordon` is a
  real scalar field on a finite 1D periodic lattice — N sites coupled by a
  nearest-neighbour discrete Laplacian — so its normal modes
  `ω_j² = m² + (4/a²) sin²(π j / N)` are **computed, not tabulated**. Claims are
  theorems of that computation: `field.finite-modes`, `field.dispersion-continuum-limit`
  (long-wavelength error < 5%), `field.stable` (min ω² ≥ 0), `field.causal`
  (group velocity ≤ c), `field.local`. New `physis experiment field-modes`.
  `set klein-gordon mass_squared -1` produces a genuine computed tachyon:
  `field.stable` and `field.causal` both flip to `fails` — the same instability
  notion as the bosonic-string tachyon, here computed from the spectrum.

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

- **M3 Computation: complexity claims** (`crates/physis-theory/src/computation.rs`).
  Added `comp.p-equals-np`, encoded as `undecidable`/`open` for the Turing
  machine — the lab honestly refuses to record holds/fails for a famous open
  problem — and `inapplicable` for a fixed circuit. Added a `nondeterministic`
  knob to `turing-machine` that flips `comp.deterministic`. Tests cover both.

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

- **Charge quantization computed from the catalog** (`crates/physis-theory/src/standard_model.rs`).
  New `empirical.charge-quantization` claim (a computed `theorem`): the net charge
  of a hydrogen atom (proton `uud` + electron) is summed from the particle
  catalog's electric charges and is exactly zero — `2·Q(u) + Q(d) + Q(e⁻) = 0`
  in units of e/3. Shown on `run standard-model`; test asserts neutrality and the
  theorem tag.

- **SM anomaly cancellation is now computed** (`crates/physis-theory/src/standard_model.rs`).
  `consistency.anomaly-cancellation` for the Standard Model was an `encoded-fact`;
  it is now a **computed `theorem`**. The hypercharges of one generation's
  left-handed Weyl fermions are summed in code: `ΣY = 0` (grav/mixed anomalies)
  and `ΣY³ = 0` (the [U(1)]³ anomaly), with an even SU(2) doublet count (Witten).
  Tests assert both the vanishing sums and the theorem tag.

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

- **Critical dimension derived from the conformal anomaly** (`crates/physis-theory/src/strings.rs`).
  `consistency.critical-dimension` no longer relies only on a table: the critical
  dimension is computed from central-charge cancellation `c_matter·D + c_ghost = 0`
  (`StringKind::worldsheet_central_charge` / `critical_dim_from_anomaly`), giving
  26 for the bosonic string (`1·D − 26`) and 10 for the superstring
  (`(3/2)·D − 15`). The verdict carries the cancellation equation as evidence; a
  test asserts the derived value matches the table for every worldsheet kind, and
  M-theory (11D SUGRA) has no worldsheet anomaly.

- **Unified tachyon stability notion across domains** (`crates/physis-theory/src/strings.rs`).
  `consistency.no-tachyon` is now computed from a string ground-state mass²
  (`α'm²`): bosonic `−1` (tachyon), superstring with GSO `0` (no tachyon),
  GSO off `−1/2` (tachyon returns). It fails exactly when `m² < 0` — the *same*
  criterion as the scalar field's `field.stable` (`min ω² < 0`), so the string
  and continuum labs share one notion of tachyonic instability (per `plans/005`).
  Verdicts unchanged; now computed with the mass² in the evidence.

- **"Why three generations?" made mechanical** (`crates/physis-theory/src/strings.rs`).
  New `euler_number` knob (Calabi–Yau Euler characteristic χ). `empirical.three-generations`
  is `undecidable` until a topology is chosen, then it is the computed topological
  count `|χ|/2`: `set heterotic-e8e8 euler_number 6` → 3 generations (`holds`);
  `euler_number 8` → 4 (`fails`). This encodes the accommodate-vs-derive critique
  directly — string theory can *fit* three generations by choosing χ = ±6, but
  nothing *derives* why χ = ±6; the knob is the unexplained choice. Docs:
  docs/KNOBS, docs/STRING-EXPERIMENT. Knob-diff test included.

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
