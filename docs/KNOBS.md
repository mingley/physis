# Knobs

See `specs/003-knobs-and-causality.md`.

Knobs are the only way an agent is allowed to change a theory. They have:

- a stable name
- a layer
- a domain
- a `ParameterOrigin` (measured, fitted, chosen, …) — so a fitted dilaton
  is not a derived prediction
- a one-line `doc` that says what turning them *means*

`physis knobs [theory]` prints the origin. `physis inspect origin fitted`
lists every fitted knob in the lab.

## Inventory (default lab)

### standard-model
- `generations` (1–4) — **measured** (nature: 3). A missing `e_R` is not this knob: `add-missing-eR` is an IR mutation on `standard-model`.
- `include_higgs` — chosen
- `include_gravity` — chosen
- `neutrino_masses` — chosen; minimal SM stores them as 0; oscillations show they are nonzero (`empirical.neutrino-masses`)

### special-relativity
- `absolute_time` — Galilean vs Lorentz boosts. The Lorentz boost is not this knob: `add-binomial-gamma` is an IR mutation. Collinear composition is not this knob: `add-minus-uv` is an IR mutation

### general-relativity
- `dim` — solar-system tests (Eddington, Mercury) are 4D; other values make them inapplicable. The Einstein–Hilbert action is not this knob: `add-r-squared` is an IR mutation. Brans–Dicke is not this knob: `add-brans-dicke` is an IR mutation
- `cosmological_constant` (Planck units, order-of-magnitude)

### newtonian-gravity
- (none — inverse-square is a single law; the Schwarzschild 3GM u² term is an IR mutation, not a knob. A Yukawa e^{-μr}/r potential is a second IR mutation, not a knob. GR keeps `dim`.)

### bell-test
- `visibility` — Werner-state visibility V. Ket topology is not this knob: `add-product` is an IR mutation. Tsirelson is not this knob: `add-pr-box` is an IR mutation

### maxwell-vacuum
- (none — vacuum is the unit medium; magnetic current is an IR mutation (`add-monopole`), not a knob. A Proca mass term is a second IR mutation (`add-proca`), not a knob. `epsilon_r` / `mu_r` stay on linear-medium.)

### linear-medium
- `epsilon_r` — relative permittivity; raises n = √(ε_r μ_r). Constitutive form is not this knob: `add-tellegen` and `add-chiral` are IR mutations
- `mu_r` — relative permeability

### ohm-circuit
- `frequency_hz` — operating frequency; lumped model holds while c/f dwarfs the circuit. Topology is not this knob: `add-tline` is an IR mutation. Lumped KVL is not this knob: `add-flux` is an IR mutation

### string / M constructions (type-iib, type-iia, type-i, heterotic-e8e8, heterotic-so32, bosonic, m-theory)
- `kind` — chosen. A missing E8 is not this knob: `add-missing-e8` is an IR mutation on `heterotic-e8e8`. SO(16) is not this knob: `add-so16` is an IR mutation on `heterotic-so32`. Chan-Paton SO(16) is not this knob: `add-chan-paton-16` is an IR mutation on `type-i`.
- `total_dim` — chosen (the critical-dimension theorem constrains it). A missing E8 is not this knob: `add-missing-e8` is an IR mutation on `heterotic-e8e8`. SO(16) is not this knob: `add-so16` is an IR mutation on `heterotic-so32`. Chan-Paton SO(16) is not this knob: `add-chan-paton-16` is an IR mutation on `type-i`.
- `observed_dim` — **measured** (empirical target: 4)
- `compact_radius_planck` — **fitted**; overall Kähler volume (size) modulus, in Planck lengths
- `supersymmetry` — chosen
- `flux_bits` — chosen
- `dilaton` — **fitted**; φ; string coupling g_s = e^φ (inflates the effective compact size)
- `h11` — chosen; Kähler (size) moduli count, heuristic stand-in for h^{1,1}
- `h21` — chosen; complex-structure (shape) moduli count, heuristic stand-in for h^{2,1}
- `euler_number` — **chosen** (not derived); Euler characteristic χ of the compactification (0 = unset); chiral generations = |χ|/2

`unique-vacuum` depends on `flux_bits`, `h11`, and `h21` (zero flux **or** zero
moduli ⇒ no landscape ⇒ uniqueness holds). `hidden-extra-dims` depends on the
effective radius `compact_radius_planck · √g_s`, so both the Kähler volume and
the dilaton can expose extra dimensions.

### observer-geometry
- `fibre_dim` — chosen (default 10 — minimal fibre that can host Spin(10); total = observed + fibre). A missing Spin(10) is not this knob: `add-missing-spin10` is an IR mutation on `observer-geometry`.
- `observed_dim` — **measured**
- `derive_gauge` — chosen. A missing Spin(10) is not this knob: `add-missing-spin10` is an IR mutation.
- `unique_vacuum` — chosen (program axiom, not a theorem). A missing Spin(10) is not this knob: `add-missing-spin10` is an IR mutation.

The total geometric dimension is `observed_dim + fibre_dim` (default `4 + 10 = 14`),
not a magic literal. Setting `fibre_dim < 10` with `derive_gauge=true` makes
`empirical.sm-gauge` fail: Spin(10) has no geometric room in a smaller fibre.

### ideal-gas
- `temperature` — gas temperature (K). Statistics are not this knob: `add-bose` and `add-fermi` are IR mutations
- `volume_ratio` — V_f/V_i for an isothermal expansion
- `particles` — number of gas particles N

### combinational-circuit
- (none — NAND netlist lives on the IR package; `add-feedback` is a cycle, not a knob. A second NAND writing the same wire is a second IR mutation (`add-contention`), not the Turing-machine `nondeterministic` knob.)

### turing-machine
- `tape_bound` — tape length in cells; `0` = unbounded. A finite bound makes the machine a finite automaton. A halt oracle is not this knob: `add-oracle` is an IR mutation
- `nondeterministic` — whether the transition relation allows branching; flips `comp.deterministic`

### landauer-engine
- `temperature_k` — bath temperature (K); sets `k_B·T·ln2`. The `ln2` factor is not this knob: `add-kt` is an IR mutation. A Maxwell demon that skips the memory cost is not this knob: `add-demon` is an IR mutation
- `bits_erased` — number of logical bits irreversibly erased
- `reversible` — logical reversibility (Bennett): erases nothing, so the process can be free. This stays a knob.

### klein-gordon
- `sites` — number of lattice sites N
- `mass_squared` — m²; negative values make the zero mode tachyonic. Stencil is not this knob: `add-next-nearest` is an IR mutation. Potential boundedness is not this knob: `add-quartic` is an IR mutation
- `spacing` — lattice spacing a

### dirac-fermion
- `sites` — number of lattice sites N
- `mass` — Dirac mass m. Doubling is not this knob: `add-wilson` is an IR mutation. Hopping range is not this knob: `add-next-nearest` is an IR mutation
- `spacing` — lattice spacing a

### blackbody (`planck`, `rayleigh-jeans`)
- `quantum` — Planck (true) vs Rayleigh–Jeans (false). Turning `planck`'s
  `quantum` off restores the ultraviolet catastrophe. Occupation is not this knob: `add-wien` is an IR mutation on `planck`. Zero-point energy is not this knob: `add-zero-point` is an IR mutation on `planck`
- `temperature` — cavity temperature (K)
- `cutoff_hz` — ultraviolet frequency cutoff (Hz). Classical `u ∝ ν_max³`.

### solid (`einstein-solid`, `dulong-petit`, `debye-solid`)
- `quantum` — Bose occupation (true) vs Dulong–Petit (false). A quartic virial is not this knob: `add-quartic` is an IR mutation on `dulong-petit`.
- `spectrum` — `einstein` (single ω, exponential freeze-out) or `debye` (ω² DOS, T³). `set einstein-solid spectrum debye` flips `thermo.debye-t3` fails → holds. A 2D ω continuum is not this knob: `add-2d` is an IR mutation on `debye-solid`. A quartic virial is not this knob: `add-quartic` is an IR mutation on `dulong-petit`.
- `temperature` — lattice temperature (K). Raising it far above `Θ` recovers Dulong–Petit as correspondence.
- `einstein_temp` — characteristic `Θ` (K): Einstein `Θ_E` or Debye `Θ_D`
- `oscillators` — number of atoms N

### de-rham
- `shape` — `disk`, `circle`, `torus`, `klein`, or `sphere` (tetrahedron `S²`).
  `set de-rham shape sphere` flips `dec.fundamental-class` fails → holds
  without flipping Poincaré (`b₁` stays 0).

### olbers (`olbers-static`, `olbers-horizon`)
- `finite_age` — light-travel horizon at `c t`. `set olbers-static finite_age true`
  flips `astro.sky-finite` and `astro.night-sky-dark` fails → holds without
  touching shell cancellation. Tired light is not this knob: `add-tired-light`
  is an IR mutation on `olbers-static`.
- `expanding` — linear Hubble dimming. `set olbers-static expanding true` flips
  cancellation holds → fails *and* the two catastrophe cells fails → holds.
  Tired light is not this knob: covering still diverges under `add-tired-light`.
- `age_yr` — cosmic age (years). `set olbers-horizon age_yr 1e26` makes `τ ≳ 1`:
  a finite but ancient sky is photosphere-bright.
- `cutoff_m` — radial cutoff (metres). Standing-theory verdicts and notes use
  the improper `R → ∞` limit or `c t` / `c/H`, not this cutoff (weakly live
  in the unbounded-static note as what the verdict is not).

### su5-gut
- `supersymmetric` — MSSM matter. Flips `gut.coupling-unification`,
  `gut.proton-decay-viable`, and `gut.weinberg-angle-mz` fails → holds
  (heuristics). `gut.weinberg-angle-mz-interval` flips fails →
  undecidable (PDG overlap without containment). The GUT-scale `3/8`
  theorem is independent of this knob. A missing 10 is not this knob:
  `add-missing-10` is an IR mutation.

## Dead knobs

If you find a knob that no claim reads, either wire it or delete it. v0 `cosmological_constant` is *weakly* dead (it appears in the world note, not in a verdict). That is an accepted M0 hole; M1 should give Λ a claim or drop the knob.
