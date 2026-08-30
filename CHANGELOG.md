# Changelog

Every change to `physis` is atomic, committed directly to `main`, agentically
reviewed, and recorded here with its rationale and the verification that backs
it. This log is part of the contract: the process is meant to be as inspectable
as the physics.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
The project keeps `unsafe`-free pure Rust and honest epistemic tags.

## [Unreleased]

### Computed theorems

- **Wilson fundamental Higgs is an IR mutation**
  (`wilson-u1`, `wilson-su2`, `wilson-su3`, `add-higgs`). Unimproved
  1×1 plaquettes are the live encoding (`wilson-plaquette 1x1`);
  appending `higgs fundamental` screens static charges and flips
  `gauge.confining` holds to fails. The residual is a Yukawa mass
  m_H = 1 in lattice units and is evidence, not the encoding: v → 0
  recovers pure-gauge confinement and the cell still fails, including
  SU(N) at weak coupling where the live encoding still holds. That is
  not a knob. Locality still holds (Higgs is not a 2×1 rectangle).
  The gauge-action strong-coupling area law still holds at default β.
  Rectangle 2×1 remains a separate locality fork (`add-rectangle`).
  `dimension` / `beta` / `sites_per_side` stay knobs. Confining names
  pure Wilson gauge field. Mutants stay the same Wilson object; they
  are not a silent install of another group. Mutants are not
  installed, not journaled, and not Canonical or P4. Catalog d² hash
  unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pins unchanged. Verified: IR round-trip; set higgs is
  unknown; hypothesize wilson-u1 / wilson-su3; live 1×1 restored;
  encode pins
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`
  (U(1)),
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`
  (SU(2)),
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`
  (SU(3)).

- **Linear-medium Pasteur chirality is an IR mutation**
  (`linear-medium`, `add-chiral`). Isotropic linear D = εE, B = μH is
  the live encoding (`constitutive isotropic-linear`); appending
  `constitutive chiral` makes circular birefringence n_L ≠ n_R and
  flips `em.constitutive-linear` holds to fails. The residual is
  2κ = 0.2 and is evidence, not the encoding: κ → 0 recovers a unique
  index and the cell still fails. That is not a knob. Gauss and
  charge conservation still hold. Tellegen magnetoelectric mixing
  remains a separate n₊ ≠ n₋ fork (`add-tellegen`). `epsilon_r` /
  `mu_r` stay knobs. Constitutive names isotropic linear D = εE,
  B = μH. Mutants stay `linear-medium`; they are not a silent
  Maxwell-vacuum or ohm-circuit install. Mutants are not installed,
  not journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode
  pin unchanged. Verified: IR round-trip; set chiral is unknown;
  hypothesize linear-medium; live isotropic-linear restored; encode pin
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.

- **Landauer Maxwell demon is an IR mutation**
  (`landauer-engine`, `add-demon`). The kT ln2 bound is the live
  encoding (`erase kT ln2`); appending `erase demon` makes the
  encoding dissipate 0 while bits are still erased and flips
  `info.landauer-cost` holds to fails. The residual is the unpaid
  N kT ln2 floor (~2.87e-21 J at 300 K) and is evidence, not the
  encoding: T = 0 makes that floor 0 and the cell still fails. That
  is not a knob. Thermodynamic freedom still fails (bits are erased).
  Dropped ln2 remains a separate cost fork (`add-kt`). `reversible` /
  `bits_erased` / `temperature_k` stay knobs. Cost names kT ln2
  Landauer bound. Mutants stay `landauer-engine`; they are not a
  silent Turing-machine install. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode
  pin unchanged. Verified: IR round-trip; set demon is unknown;
  hypothesize landauer-engine; live kT ln2 restored; encode pin
  `94e8b44c1e141f6e4cbff91a409b805361e5fe00a925121348b62cdbc3e187a9`.

- **Combinational multi-driven net is an IR mutation**
  (`combinational-circuit`, `add-contention`). Unique NAND drivers are
  the live encoding (`nand 0 1 -> 2`); appending `nand 0 0 -> 2` puts a
  second driver on the same wire and flips `comp.deterministic` holds
  to fails. The truth-table disagreement is 0.25 (NAND vs NOT on 1 of
  4 rows) and is evidence, not the encoding: two identical NANDs agree
  everywhere and the cell still fails. That is not a knob. Acyclicity
  and combinational halting still hold on the mutant. Feedback remains
  a separate cycle fork (`add-feedback`). `turing-machine`
  `nondeterministic` stays a knob. Determinism names unique NAND
  drivers; TM determinism stays encoding-wide. Mutants stay
  `combinational-circuit`; they are not a silent Turing-machine
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Live encode pin unchanged. Verified: IR
  round-trip; set contention is unknown; hypothesize
  combinational-circuit; live unique-driver netlist restored; encode
  pin
  `762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea`.

- **Newtonian Yukawa potential is an IR mutation**
  (`newtonian-gravity`, `add-yukawa`). Inverse-square Binet is the live
  encoding (`binet inverse-square`); appending `potential yukawa` makes
  the impulse-approximation Soldner factor μR K₁(μR) equal 0.601907 at
  grazing μR = 1 and flips `gr.newton-half-deflection` holds to fails.
  The residual 1 − μR K₁ is 0.398 and is evidence, not the encoding:
  μ → 0 recovers Newton and the cell still fails. That is not a knob.
  Eddington and Mercury still fail (Yukawa is not GR). Schwarzschild
  3GM u² remains a separate Eddington/Mercury fork (`add-schwarzschild`).
  `general-relativity` keeps `dim`. Half-angle names inverse-square
  Binet rhs; GR solar cells stay encoding-wide. Mutants stay
  `newtonian-gravity`; they are not a silent GR install. Mutants are
  not installed, not journaled, and not Canonical or P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pin unchanged. Verified: IR round-trip; set yukawa is
  unknown; hypothesize newtonian-gravity; live inverse-square restored;
  encode pin
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.

- **Dirac next-nearest hopping is an IR mutation**
  (`dirac-fermion`, `add-next-nearest`). Nearest-neighbour naive hopping
  is the live encoding (`dirac naive`); appending `dirac nnn` includes
  distance-2 hopping and flips `field.local` holds to fails. The sampled
  residual max |c sin(2ka)/a| is 0.5 on the default lattice and is
  evidence, not the encoding: on N = 4 every lattice mode has sin(2ka) = 0
  and the cell still fails. That is not a knob. Doubling still fails
  (sin(ka) and sin(2ka) share the Brillouin-edge zero). Wilson r remains
  a separate doubling fork (`add-wilson`). `sites` / `mass` / `spacing`
  stay knobs. Locality names nearest-neighbour 1D lattice Dirac;
  Klein–Gordon locality stays nearest-neighbour 1D periodic lattice.
  Mutants stay `dirac-fermion`; they are not a silent Klein–Gordon
  install. Mutants are not installed, not journaled, and not Canonical
  or P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Live encode pin unchanged. Verified: IR round-trip;
  set next_nearest is unknown; hypothesize dirac-fermion; live naive
  hopping restored; encode pin
  `62ea25b78eaf5a7d934db096943e401135acf490c4594fc8a0621478581a521a`.

- **Ohm-circuit unlumped mesh flux is an IR mutation**
  (`ohm-circuit`, `add-flux`). Lumped Kirchhoff voltage is the live
  encoding (`branch R 0 1`); appending `loop dPhi/dt` makes the Faraday
  residual of ∮E·dl + dΦ/dt equal dB/dt × L² = 0.01 and flips
  `em.faraday` holds to fails. That is not a knob. KCL and the
  quasi-static cell still hold on the mutant. Transmission-line delay
  remains a separate KCL fork (`add-tline`). `frequency_hz` stays a
  knob and still flips `em.quasi-static-valid`. Faraday names lumped
  Kirchhoff voltage; Maxwell Faraday stays source-free homogeneous
  dF=0; linear-medium Faraday stays encoding-wide. Mutants stay
  `ohm-circuit`. Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Live encode pin unchanged. Verified:
  IR round-trip; set flux is unknown; hypothesize ohm-circuit; live
  lumped KVL restored; encode pin
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.

- **Bell-test PR-box correlator is an IR mutation**
  (`bell-test`, `add-pr-box`). Hilbert-space CHSH is the live encoding
  (`state singlet`); appending `correlator pr-box` makes the CHSH
  combination of `E = (−1)^{xy}` equal 4, which exceeds Tsirelson's
  `2√2` and flips `quantum.tsirelson-bound` holds to fails. That is
  not a knob. Bell violation still holds (`S = 4 > 2`). The operator
  correlator fails (PR E is not `−cos(a−b)`). Product ket remains a
  separate Bell-violation fork (`add-product`). `visibility` stays a
  Werner mixedness knob and still flips `quantum.bell-violation` on
  the singlet. Tsirelson names Hilbert-space CHSH (Tsirelson 2√2).
  Mutants stay `bell-test`. Mutants are not installed, not journaled,
  and not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Live encode pin unchanged.
  Verified: IR round-trip; set prbox is unknown; hypothesize
  bell-test; live singlet restored; encode pin
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.

- **Maxwell vacuum Proca mass is an IR mutation**
  (`maxwell-vacuum`, `add-proca`). Massless Coulomb Gauss is the live
  encoding; appending `proca m2 A` makes the Coulomb residual of
  ∇·E + m²φ the Proca mass term and flips `em.gauss` holds to fails.
  That is not a knob. Faraday, Ampère, wave-speed, Lorentz, charge
  conservation, and constitutive-linear still hold on the mutant.
  Magnetic current remains a separate Faraday fork (`add-monopole`).
  Mutants stay `maxwell-vacuum`; they are not a silent linear-medium
  install. `epsilon_r` / `mu_r` stay on linear-medium. Maxwell Gauss
  names source-free massless Maxwell; linear-medium Gauss and
  ohm-circuit Gauss stay encoding-wide. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Live encode pin
  unchanged. Verified: IR round-trip; set proca is unknown; hypothesize
  maxwell-vacuum; live massless Gauss restored; encode pin
  `f6f47f600c798018d8cea30121512950f0066f56406aa7be34575f4fae034cc3`.

- **Ideal-gas degenerate Fermi statistics is an IR mutation**
  (`ideal-gas`, `add-fermi`). Maxwell–Boltzmann statistics are the live
  encoding (`gas maxwell-boltzmann`); appending `gas fermi` makes the
  Sommerfeld heat capacity `C_V = (π²/2) N k (T/T_F)` instead of
  `(3/2) N k` and flips `thermo.equipartition` holds to fails. That is
  not a knob. Third-law also fails to holds (Fermi S/Nk → 0). The second
  law still holds on the default expansion. `temperature` /
  `volume_ratio` / `particles` stay knobs; `volume_ratio` still flips
  `thermo.second-law`. Bose statistics remain a separate fork
  (`add-bose`). Mutants stay `ideal-gas`; they are not a silent
  Einstein-solid install. Equipartition names classical C_V = 3/2 Nk.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Live encode pin unchanged. Verified: IR round-trip;
  set fermi is unknown; hypothesize ideal-gas; live Maxwell–Boltzmann
  restored; encode pin
  `fb1dbc123bf6f00bc62cb49b4ba5df49a6b22aba81c6d9434e817c714ea18e06`.

- **Klein–Gordon unbounded minus-φ⁴ is an IR mutation**
  (`klein-gordon`, `add-quartic`). The live scalar potential is
  quadratic; appending `potential minus-phi4` makes
  `V(φ) = ½ m² φ² − φ⁴/4` run to −∞ and flips `field.stable` holds to
  fails. That is not a knob. `sites` / `mass_squared` / `spacing` stay
  knobs; `mass_squared: -1` still produces a tachyon. Next-nearest
  locality is still a separate fork (`add-next-nearest`). Stability
  names the quadratic Klein-Gordon potential. Mutants stay
  `klein-gordon`; they are not a silent Dirac install. Mutants are
  not installed, not journaled, and not Canonical or P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. P3N count stays 4.
  Live encode pin unchanged. Verified: IR round-trip; set quartic is
  unknown; hypothesize klein-gordon; live quadratic potential restored;
  encode pin
  `32b0997d38afb977615e8fc6527ee5d766271e8a31fb5c882912ca740a3b4e4f`.

- **Naive Dirac doubling is an IR mutation**
  (`dirac-fermion`, `add-wilson`). Naive 1D lattice Dirac is the live
  encoding (`dirac naive`); `sin(ka) = 0` at `k = 0` and `k = π/a` so
  `fermion.no-doublers` fails. Appending `dirac wilson` lifts the edge
  copy to mass `m + 2r/a` and flips that cell fails to holds. That is
  not a knob. `sites` / `mass` / `spacing` stay knobs. Mutants stay
  `dirac-fermion`; they are not a silent Klein–Gordon install.
  `mass_squared` stays on klein-gordon. Dirac no-doublers names naive
  1D lattice Dirac; Klein–Gordon locality stays named nearest-neighbour;
  Dirac locality stays encoding-wide (both encodings are nearest-
  neighbour). Mutants are not installed, not journaled, and not
  Canonical or P4. Catalog d² hash unchanged. Unique-vacuum graph id
  unchanged. P3N count stays 4. Verified: IR round-trip; set wilson
  is unknown; hypothesize dirac-fermion; live naive operator restored;
  encode pin
  `62ea25b78eaf5a7d934db096943e401135acf490c4594fc8a0621478581a521a`.

- **Landauer dropped-ln2 bound is an IR mutation**
  (`landauer-engine`, `add-kt`). `kT ln2` is the live
  encoding (`erase kT ln2`); appending `erase kT` makes the
  encoding energy `N kT` instead of `N kT ln2` and flips
  `info.landauer-cost` holds to fails. That is not a knob.
  Thermodynamically-free still fails on the default irreversible
  one-bit mutant. `temperature_k` / `bits_erased` / `reversible`
  stay knobs; `reversible` still flips `info.thermodynamically-free`.
  Mutants stay `landauer-engine`; they are not a silent
  Turing-machine install. Landauer cost names the kT ln2 bound.
  Mutants are not installed, not journaled, and not Canonical or
  P4. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  P3N count stays 4. Verified: IR round-trip; set kt is unknown;
  hypothesize landauer-engine; live kT ln2 restored; encode pin
  `94e8b44c1e141f6e4cbff91a409b805361e5fe00a925121348b62cdbc3e187a9`.

- **Ideal-gas Bose statistics is an IR mutation**
  (`ideal-gas`, `add-bose`). Maxwell–Boltzmann statistics are the live
  encoding (`gas maxwell-boltzmann`); appending `gas bose` makes the
  low-T Bose entropy S/Nk ∝ (T/T_c)^{3/2} vanish and flips
  `thermo.third-law` fails to holds. That is not a knob.
  Equipartition at 300 K and the second law still hold on the mutant.
  Mutants stay `ideal-gas`; they are not a silent Einstein-solid
  install. `temperature` / `volume_ratio` / `particles` stay knobs.
  Ideal-gas third law names classical Sackur–Tetrode; Einstein-solid
  third law stays encoding-wide. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set bose is unknown; hypothesize ideal-gas; live
  Maxwell–Boltzmann restored; encode pin
  `fb1dbc123bf6f00bc62cb49b4ba5df49a6b22aba81c6d9434e817c714ea18e06`.

- **Maxwell vacuum magnetic current is an IR mutation**
  (`maxwell-vacuum`, `add-monopole`). Homogeneous Faraday is the live
  Bianchi encoding (`maxwell dF=0`); appending `dF = *j_m` makes the
  plane-wave residual of ∇×E + ∂B/∂t + J_m the uniform magnetic
  current and flips `em.faraday` holds to fails. That is not a knob.
  Gauss, Ampère, wave-speed, Lorentz, charge conservation, and
  constitutive-linear still hold on the mutant. Mutants stay
  `maxwell-vacuum`; they are not a silent linear-medium install.
  `epsilon_r` / `mu_r` stay on linear-medium. Maxwell Faraday names
  source-free homogeneous dF=0; linear-medium Faraday and ohm-circuit
  KVL stay encoding-wide. Mutants are not installed, not journaled,
  and not Canonical or P4. Catalog d² hash unchanged. Unique-vacuum
  graph id unchanged. P3N count stays 4. Verified: IR round-trip; set
  monopole is unknown; hypothesize maxwell-vacuum; live homogeneous
  Faraday restored; encode pin
  `f6f47f600c798018d8cea30121512950f0066f56406aa7be34575f4fae034cc3`.

- **Linear-medium Tellegen mixing is an IR mutation**
  (`linear-medium`, `add-tellegen`). The constitutive law is isotropic
  linear (`constitutive isotropic-linear`); appending
  `constitutive tellegen` splits the unique index n = √(ε_r μ_r) into
  n₊ ≠ n₋ and flips `em.constitutive-linear` holds to fails. That is
  not a knob. `epsilon_r` / `mu_r` still flip `em.wave-speed-c` and
  `em.lorentz-invariance` independently. Maxwell vacuum Holds
  encoding-wide (unit medium); ohm-circuit is inapplicable. Mutants
  are not installed, not journaled, and not Canonical or P4. Catalog
  d² hash unchanged. Unique-vacuum graph id unchanged. P3N count
  stays 4. Verified: IR round-trip; set tellegen is unknown;
  hypothesize linear-medium; live constitutive law restored; encode pin
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.

- **Newtonian 3GM u² Binet term is an IR mutation**
  (`newtonian-gravity`, `add-schwarzschild`). Inverse-square light and
  orbits are the live Binet rhs (`binet inverse-square`); appending
  `binet 3GM u^2` flips `gr.newton-half-deflection` holds to fails and
  `gr.eddington-deflection` / `gr.mercury-perihelion` fails to holds.
  That is not a knob. Mutants stay `newtonian-gravity`; they are not a
  silent GR install. `general-relativity` keeps `dim`. Mutants are not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set schwarzschild is unknown; hypothesize
  newtonian-gravity; live inverse-square restored; encode pin
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.

- **Bell-test product ket is an IR mutation**
  (`bell-test`, `add-product`). The CHSH lab ket is the two-qubit singlet
  (`state singlet`); appending `state product` flips
  `quantum.bell-violation` and `quantum.correlator-from-operators` holds
  to fails. That is not a knob. `visibility` still scales the singlet
  Werner mixture independently. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. P3N count stays 4. Verified: IR
  round-trip; set product is unknown; hypothesize bell-test; live
  singlet restored; encode pin
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.

- **Ohm-circuit transmission-line delay is an IR mutation**
  (`ohm-circuit`, `add-tline`). Kirchhoff current law is the lumped
  branch netlist (`branch R 0 1`); appending `tline 0 1` flips
  `em.charge-conservation` holds to fails. That is not a knob.
  `frequency_hz` still flips `em.quasi-static-valid` independently.
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set tline is unknown;
  hypothesize ohm-circuit; live netlist restored; encode pin
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.

- **Wilson SU(N) 2×1 rectangle is an IR mutation**
  (`wilson-su2`, `wilson-su3`, `add-rectangle`). Same stencil dialect as
  U(1): locality is the unimproved 1×1 Wilson stencil; appending
  `wilson-rectangle 2x1` flips `gauge.local` holds to fails. Exact 2D
  area-law factorization does not apply to the rectangle encoding.
  That is not a knob. Mutants are not installed, not journaled, and not
  Canonical or P4. 4D SU(N) confinement stays a conjecture Holds.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set rectangle is unknown;
  hypothesize wilson-su3; live stencil restored; encode pins
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`
  (SU(2)) and
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`
  (SU(3)).

- **Wilson U(1) 2×1 rectangle is an IR mutation**
  (`wilson-u1`, `add-rectangle`). Locality is the unimproved 1×1
  Wilson stencil; appending `wilson-rectangle 2x1` flips `gauge.local`
  holds to fails. Exact 2D area-law factorization does not apply to the
  rectangle encoding. That is not a knob. Mutants are not installed,
  not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Unique-vacuum graph id unchanged. P3N
  count stays 4. Verified: IR round-trip; set rectangle is unknown;
  hypothesize wilson-u1; live stencil restored; encode pin
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`.

- **Independent from_lab judge is a unique judge op**
  (`physis judge`, `Role::Judge`). Rebuilds `Judgment::from_lab` from live
  evaluator axes and receipts into a JudgmentProjection DAG. Unique-vacuum
  is heuristic failed, not logical proved. Super-K is empirical excluded.
  GQW NLL is statistical computed. GUT-scale 3/8 is numeric certified.
  Poincaré stays logical undetermined. Catalog d² is logical proved only
  after a live dual-check. A forged `projection_hash` cannot mint.
  Not P3S, not Canonical, not P4. Loop judges after encode. Unique-vacuum
  graph id unchanged. P3N count stays 4. Unique-vacuum projection
  `0dadce8d7bfc005efc32e47917f75b4c17ea77900ec9f6592010fd81f0f1ea76`.
  GUT-scale 3/8 projection
  `40c991698dbff52a5614093b98edcc3478a3702ddcb5cc545f9818af4a6448ae`.
  Verified: role gates; journal restore; JSONL cannot mint proved.

- **Independent IR package encode is a unique encoding-auditor op**
  (`physis encode`, `Role::EncodingAuditor`). Parses, round-trips, and
  reconstructs live theory IR packages (`combinational-circuit` NAND
  netlist, `klein-gordon` nearest-neighbour stencil, `wilson-u1` /
  `wilson-su2` / `wilson-su3` 1×1 plaquettes, `ohm-circuit` lumped
  branches, `bell-test` singlet ket, `newtonian-gravity` inverse-square
  Binet rhs, `linear-medium` isotropic-linear constitutive law). A forged
  `package_hash` cannot mint. Refuses theories with no package.
  Hypothesize mutants are not installed. Not P3S, not a kernel receipt,
  not Canonical, not P4. Loop encodes after cite. Unique-vacuum graph
  id unchanged. P3N count stays 4. Combinational-circuit package id
  `762aa72d9eace0c61026eca6ebf71b37f26608797a6786c60b92ba06af4ad8ea`.
  Klein-Gordon package id
  `32b0997d38afb977615e8fc6527ee5d766271e8a31fb5c882912ca740a3b4e4f`.
  Wilson U(1) package id
  `d9644435e8775eeb95d5e81638ad61a589686d65ff6929caf0ec3c2769d4423a`.
  Wilson SU(2) package id
  `32f36c4b5c3dc442b1c1fa970c1949c12fd0601b640f6c784d2317fcb742897a`.
  Wilson SU(3) package id
  `03bd82af34a6e36ee04985c243a0e2a35ab9fe56a1b28d3ad0bb63ea8461d8d3`.
  Ohm-circuit package id
  `fb14d2c8a8cf2c51fe67c2f334a9307860c6ebb5cfbeca1c35467d61f1387af1`.
  Bell-test package id
  `4a54aa1db88b053ef04a53593732c435331a71dcc0f8ad3749e7cbb6786990dc`.
  Newtonian-gravity package id
  `e6e7c4222c571adcf6f526a27ab5e0572fb41d92361c7f3ce393e71e23184078`.
  Linear-medium package id
  `35df991eb0911875613084efff07327ed6821b5580bfbccb85dd08387c3722eb`.
  Verified: role gates; journal restore; hypothesize does not change
  the live package id.

- **Independent SourceRecord cite is a unique provenance-auditor op**
  (`physis cite`, `Role::ProvenanceAuditor`). Rebuilds live dataset
  (PDG, Super-K) and catalog-dossier locators via `SourceRecord::recheck`.
  A forged `source_hash` cannot mint. Not P3S, not a kernel receipt,
  not Canonical, not P4. Unique-vacuum and GUT-scale 3/8 refuse. Loop
  cites after enclose. Catalog d² hash unchanged. Unique-vacuum graph
  id unchanged. P3N count stays 4. Super-K source id
  `26467998781b7d501f90a1dc762d3c16ae636f867ea61152923c505e1ad3bbef`.
  Verified: recheck rejects a tampered
  hash; role gates; journal restore.

- **Independent Ratio enclose is a unique numerical-verifier op**
  (`physis enclose`, `Role::NumericalVerifier`). Parses live
  `CertifiedNumeric` overlay strings as canonical `Ratio` (`3/8`, `0`,
  `-1/2`) and stores a content-addressed `NumericCertificate`. Succeeds
  on the four P3N cells; refuses unique-vacuum, Super-K, GQW NLL, and
  Poincaré. Restore rebuilds from live strings; a forged
  `certificate_hash` cannot mint. Not a kernel receipt, not Canonical,
  not P4. `inspect trust P3N` stays count 4. Loop runs enclose after
  falsify. Catalog d² hash unchanged. Unique-vacuum graph id unchanged.
  GUT-scale 3/8 certificate
  `0967e9f42ec9ff0fd8e29fecc5bb5a3ed9aba4974ac77b0e5217a4bb634ec202`.
  Verified: parse_display rejects `6/16` and `0.23122`; role gates;
  journal restore; P3N count.

- **Correspondence cells name a DomainOfValidity**
  (`thermo.high-t-classical`, `thermo.debye-t3`, `thermo.rj-ir-limit`,
  `gauge.exact-area-law-2d`). High-T is `T/Θ ≥ 8`; Debye T³ is the
  `Θ/20` probe; Rayleigh–Jeans correspondence is `hν = 0.01 kT`; the
  exact area law is 2D plaquette factorization. Dulong–Petit at the
  current T stays encoding-wide. Poincaré, Maxwell's quasi-static copy,
  and GUT `Tr Q` stay encoding-wide. Not Canonical, not P4. Catalog d²
  hash unchanged. Unique-vacuum graph id unchanged. Verified: unit
  domain tests; why prints the regimes; encoding-wide cells stay wide.

- **Klein–Gordon next-nearest coupling is an IR mutation**
  (`klein-gordon`, `add-next-nearest`). Locality is nearest-neighbour on
  a 1D periodic lattice; appending `laplacian nnn` flips `field.local`
  holds to fails. That is not a knob. Mutants are not installed, not
  journaled, and not Canonical or P4. Catalog d² hash unchanged.
  Unique-vacuum graph id unchanged. Verified: IR round-trip; set
  next_nearest is unknown; hypothesize klein-gordon; live stencil
  restored.

- **Evidence graphs persist across `--journal` restore**
  (`JournalEvent::Evidence`). Restore rebuilds the DAG from live
  evaluations. A tampered `graph_hash` cannot mint the snapshot.
  Restore does not journal again. `replay_journal` still certifies only
  `set-knob`. Not Canonical, not P4. Catalog d² hash unchanged.
  Verified: unique-vacuum reconstitutes graph
  `6ee50cdc3de02838465b178b47061d8d5b36d6c135baf40f80988ff640a36bc9`;
  a forged hash is absent from the store; two-process CLI prints the
  same id; dump-only restore does not append.

- **Inverse query over projected judgments**
  (`physis inspect judgment`). `statistical-computed` lists the PDG
  GQW cell; `empirical-excluded` lists Super-K `p→e+π0`;
  `logical-proved` is empty until a dual-checked receipt exists.
  Explorer may observe. Does not mint. Catalog d² hash unchanged.
  Verified: count 1 statistical; Super-K is not that row; prove d²
  appears under logical-proved; unknown labels refuse.

- **PDG mixing angle carries a Gaussian likelihood**
  (`LikelihoodModel::Gaussian`, `Verdict::with_statistical_nll`).
  `gut.weinberg-angle-mz-interval` keeps interval-subset on the empirical
  axis and projects `statistical computed` from the exact NLL of the
  five-decimal GQW centre versus PDG σ = 10^{-5}. Super-K `p→e+π0` stays
  a one-sided hull: `from_lab` without an NLL overlay is still
  `empirical excluded` / `empirical compatible`. Not P3N, not a kernel
  proof, not an LLM score. Catalog d² hash unchanged. Verified: NLL of
  μ is 0 and of μ+σ is 1/2; Super-K compare_gaussian ignores a point;
  why prints `nll`; MSSM NLL is smaller than minimal SU(5); P3N count
  stays 4.

- **Proposers cannot remint their own receipts**
  (`Role::ReplicationAgent`, `Role::EmpiricalAnalyst`).
  `proof-searcher` may `prove` and cannot `reproduce`. `explorer` may
  observe and cannot `score`. `replication-agent` remints in-process
  (still not P4). `empirical-analyst` scores the empirical-target
  fixture. Lab still runs the full protocol. Catalog d² hash unchanged.
  Verified: role unit tests; lab prove-then-reproduce refusal;
  explorer cannot score.

- **Hypothesis search mutates IR packages, not only knobs**
  (`physis hypothesize`, `physis-ir::apply_mutation`).
  `combinational-circuit` is a NAND netlist in a theory package.
  `add-feedback` appends a gate equation; that is not a knob.
  Cycle detection flips `comp.acyclic` holds to fails and takes
  `comp.halts` out of the combinational domain (inapplicable).
  Mutants are not installed, not journaled, and not Canonical or P4.
  Catalog d² hash unchanged. Verified: IR round-trip; set feedback
  is unknown; hypothesize combinational-circuit; live netlist restored.

- **Evidence graphs are a store DAG**
  (`physis evidence <claim>`, `NodeKind::Evidence`). Each FormalClaim
  identity is a Statement node; each theory evaluation is an Evaluation
  parented by that statement; the Evidence root's parents are the
  evaluation ids. A verdict flip is a new graph. Same live lab is the
  same graph id. The snapshot is not deserialized as authority, not
  Canonical, and not P4. Catalog d² hash unchanged. Verified: unique-vacuum
  has 4 statements / 10 evaluations / 1 graph; descendants of the
  string statement exclude the SM evaluation; turning unique_vacuum off
  mints a new graph while the old one remains a descendant of the old
  observer-geometry eval; Super-K and quasi-static still print their
  encoding contract.

- **Unique-vacuum encodings name a DomainOfValidity**
  (`predictivity.unique-vacuum`). String constructions share the
  flux/moduli landscape regime; observer-geometry names the
  unique_vacuum program axiom; GR names classical Einstein-Hilbert
  plus Λ; the SM names the Higgs vacuum given its parameters. They
  remain Asserted (heuristic/conjecture). Not Canonical, not P3N,
  not a kernel proof. Catalog d² hash unchanged. Verified: the four
  hashes stay distinct; string kinds share one hash; evidence prints
  named regimes and not encoding-wide.

- **Super-K p→e+π0 is a Dataset**
  (`sk-2020-p-e-pi0`, Takenaka et al., Phys. Rev. D 102, 112011).
  `gut.proton-lifetime-sk` compares the dim-6 M_GUT^4 lifetime scaling
  to that 90% CL allowed hull. The 2.4×10^34 yr figure is the published
  lower limit, not an invented number. Minimal SU(5) is excluded; MSSM
  dim-6 is compatible. Decade envelope stands in for missing matrix
  elements. Not P3N, not a kernel proof, not dimension-5 operators.
  Domain names p→e+π0 / dim-6 / Super-K 90% CL. Catalog d² hash
  unchanged. Verified: unit tests on the Dataset and SU(5); CLI why /
  set / inspect gap / evidence; P3N count stays 4.

- **Evidence graphs group by FormalClaim, not slug**
  (`physis evidence <claim>`). Distinct statement hashes that share a
  lab id are competing encodings, not one theorem. Competing
  evaluations are listed per identity. Confidence is the derived
  TrustProfile; there is no numeric score, no Canonical, and no P4.
  `predictivity.unique-vacuum` and `em.quasi-static-valid` are the
  load-bearing examples. Catalog d² hash unchanged. Verified: unique
  vacuum has multiple encodings; ohm-circuit names λ, Maxwell stays
  encoding-wide; explorer may observe.

- **Hypothesis search is constrained structural mutation**
  (`physis hypothesize [theory]` probes chosen and fitted knobs for
  scientific-axis diffs and restores). Measured knobs (generations,
  observed_dim) stay frozen: they are nature, not a hypothesis about
  the encoding. Fitted probes are tagged accommodate. Explorer may
  observe; the command does not journal, persist, or mint. The research
  loop uses this search instead of listing unproved catalog slugs.
  Catalog d² hash unchanged. Verified: type-iib finds critical-dimension;
  standard-model does not propose generations; klein-gordon coarse
  spacing is numeric unresolved; knobs restored; explorer permitted.

- **Knob diffs are scientific-axis, not kind-only**
  (`VerdictDiff` records derivation, empirical, and projected judgment
  labels plus the statement hash; `diff_verdicts` emits a row when any
  of those axes move). `set klein-gordon spacing 100` is
  `holds → undecidable` **and** `not-applicable → inconclusive` /
  `logical undetermined → numeric unresolved`, not a failed theorem.
  `set su5-gut supersymmetric true` moves the PDG interval from empirical
  excluded to inconclusive. Pre-axis journals still replay: extra fields
  compare only when the record carries them; a forged empirical string
  does not certify. Set-time judgment uses `from_lab` without a receipt.
  Catalog d² hash unchanged. Verified: unit tests on Klein–Gordon and
  SU(5); legacy JSONL strip; tampered empirical axis; CLI `set`.

- **Claim cannot rebind the lab slug**
  (`Claim.id` is private; `id()` / `id_str()` are the getters).
  A public assignment cannot attach a kernel receipt to a different
  slug. Lemma edges stay public and are not in the statement hash.
  Catalog d² hash unchanged. Verified: compile-fail against assigning
  `Claim.id`. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Claim cannot assign class or rebind commitments**
  (`Claim` class, layer, assumptions, domain, and commitments are
  private). Overlays remain `with_commitments`, `with_domain`, and
  `with_assumptions`. The slug and lemma edges stay public. Catalog d²
  hash unchanged. Verified: compile-fail against assigning
  `Claim.class`. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Claim cannot rebind the hashed sentence**
  (`Claim.statement` is private; `statement()` is the getter).
  Same-module mutation still cannot keep a stale hash; a public
  assignment cannot rebind a kernel receipt. Catalog d² hash unchanged.
  Verified: compile-fail against `Claim.statement.push_str`. Live prove
  of catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Verdict cannot assign CertifiedNumeric**
  (`Verdict` derivation, empirical, semantic, and enclosure fields are
  private). Overlay builders (`with_certified_numeric`,
  `with_cross_checked`, `with_empirical`, `with_intractable`) remain the
  only assignment path. Catalog d² hash unchanged. Verified: compile-fail
  against assigning `Verdict.derivation`. Live prove of catalog d² is
  still Lean+nanoda; `why` of GUT-scale 3/8 is still `numeric certified`
  with enclosure `[3/8, 3/8]`. `fmt`, `clippy -D warnings`, full suite,
  CLI.

- **Claim cannot assign CertifiedNumeric**
  (`Claim` derivation, empirical, and semantic fields are private).
  Constructors tag derivation from class; `CertifiedNumeric` and
  encoding-review overlays live on `Verdict`. Catalog d² hash unchanged.
  Verified: compile-fail against assigning `Claim.derivation` and
  `Claim.semantic`. Live prove of catalog d² is still Lean+nanoda.
  `fmt`, `clippy -D warnings`, full suite, CLI.

- **Verdict is not JSON-mintable**
  (`Verdict` has no Deserialize). Journal diffs still store
  `VerdictKind` only. JSON cannot mint a `certified-numeric` overlay or
  an encoding-review tag. Catalog d² hash unchanged. Verified:
  compile-fail against Verdict Deserialize. Live prove of catalog d² is
  still Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Canonical is not a mintable semantic tag**
  (`SemanticAssurance` has no `Canonical` variant; P3S is taken from the
  review store of the live statement hash, not `Verdict.semantic`).
  Encoding review still tops out at adversarially-reviewed. Catalog d²
  hash unchanged. Verified: compile-fail against
  `SemanticAssurance::Canonical`; inspect P3S is empty until review.
  Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **StatisticalJudgment cannot mint computed**
  (`StatisticalJudgment` is a transparent wrapper with a private kind).
  There is no public `Computed` constructor. `from_lab` does not project
  a statistical object for any claim class. A reserved crate-private
  computed value is `statistical computed`, not `logical proved`. Catalog
  d² hash unchanged. Verified: compile-fail against
  `StatisticalJudgment::Computed` and a kind struct literal; from_lab
  never returns Statistical. Live prove of catalog d² is still
  Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **HeuristicJudgment cannot mint suggestive**
  (`HeuristicJudgment` is a transparent wrapper with a private kind).
  Suggestive / failed are produced only by `from_lab`. A heuristic Holds
  is `heuristic suggestive`, not `logical proved`. Catalog d² hash
  unchanged. Verified: compile-fail against `HeuristicJudgment::Suggestive`
  and a kind struct literal; heuristic Holds is suggestive. Live prove
  of catalog d² is still Lean+nanoda; `why` of GQW at `M_Z` is still
  heuristic. `fmt`, `clippy -D warnings`, full suite, CLI.

- **EmpiricalJudgment cannot mint compatible**
  (`EmpiricalJudgment` is a transparent wrapper with a private kind).
  Compatible / excluded are produced only by `from_lab` from a
  registered empirical overlay. Evaluator Holds with `Untested` stays
  `empirical inconclusive`. Catalog d² hash unchanged. Verified:
  compile-fail against `EmpiricalJudgment::Compatible` and a kind
  struct literal; untested Holds is not compatible. Live prove of
  catalog d² is still Lean+nanoda; `why` of the PDG mixing-angle
  interval is still empirical excluded / inconclusive as encoded.
  `fmt`, `clippy -D warnings`, full suite, CLI.

- **NumericJudgment cannot mint certified**
  (`NumericJudgment` is a transparent wrapper with a private kind).
  A certified enclosure is produced only by `from_lab` from a
  `CertifiedNumeric` Holds. JSON still cannot mint it; a Rust
  `Certified` variant cannot be constructed either. Catalog d² hash
  unchanged. Verified: compile-fail against `NumericJudgment::Certified`
  and a kind struct literal; from_lab CertifiedNumeric Holds is
  `numeric certified` with the display enclosure. Live prove of catalog
  d² is still Lean+nanoda; `why` of GUT-scale 3/8 is still `numeric
  certified`. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Claim statement hash is derived**
  (`Claim::statement_hash` is a getter; there is no stored field and
  no Deserialize). Mutating the English statement cannot keep a stale
  catalog hash attached to a kernel receipt. JSON cannot mint a Claim
  identity. Catalog d² hash unchanged. Verified: compile-fail against
  assigning `statement_hash` and Deserialize; mutating the sentence
  changes the getter; from_claim follows the live sentence. Live prove
  of catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **LogicalJudgment cannot mint proved**
  (`LogicalJudgment` is a transparent wrapper with a private kind).
  JSON still cannot mint `logical proved`; a Rust `Proved` variant
  cannot be constructed outside `from_lab` either. Catalog d² hash
  unchanged. Verified: compile-fail against `LogicalJudgment::Proved`
  and a kind struct literal; from_lab Holds is not proved until a
  receipt. Live prove of catalog d² is still Lean+nanoda; why after
  prove is still `logical proved`. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **FormalClaim is not JSON-mintable**
  (`FormalClaim` private fields, no Deserialize). `from_claim` is the
  only constructor and recomputes the statement hash from the live
  sentence: a forged hash on `Claim` is not copied through. JSON cannot
  mint a catalog identity. Catalog d² hash unchanged. Verified:
  compile-fail against FormalClaim literals and Deserialize;
  `from_claim` restores an honest hash. Live prove of catalog d² is
  still Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Judgment is not JSON-mintable**
  (`Judgment` and `LogicalJudgment` have no Deserialize). `from_lab`
  projects evaluator plus receipts; JSON cannot mint `logical proved`.
  Evaluator Holds without a dual-checked receipt stays undetermined.
  Catalog d² hash unchanged. Verified: compile-fail against Judgment
  Deserialize; from_lab Holds is not Proved until a receipt. Live
  prove of catalog d² is still Lean+nanoda; why after prove is still
  `logical proved`. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Long-wavelength cells name a DomainOfValidity**
  (`field.dispersion-continuum-limit`, ohm-circuit `em.quasi-static-valid`).
  Encoding-wide "long-wavelength" is a hidden regime. Dispersion is the
  longest non-zero lattice mode, not the Nyquist mode and not the
  Richardson `|k a| < 1` probe. Lumped validity is `λ > 100 ×` circuit
  size; Maxwell's inapplicable copy stays encoding-wide. Catalog d² hash
  unchanged. Verified: those cells are not encoding-wide; `field.stable`
  and Maxwell quasi-static are; `why` names the regime. Live prove of
  catalog d² is still Lean+nanoda. `fmt`, `clippy -D warnings`, full
  suite, CLI.

- **Hodge P2 names a discrete DomainOfValidity**
  (`dec.hodge-harmonic`). Encoding-wide "Hodge theorem" is a hidden
  regime: the cell is combinatorial Laplacian nullity vs coboundary
  `b₁` on a finite simplicial complex, not the smooth Hodge theorem.
  Euler–Poincaré and Poincaré stay encoding-wide. Catalog d² hash
  unchanged. Verified: Hodge is not encoding-wide; Euler and Poincaré
  are; `inspect trust P2` stays count 1; `why` names finite simplicial
  1-cochains. Live prove of catalog d² is still Lean+nanoda. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **GUT mixing-angle and SM P3N cells name a DomainOfValidity**
  Encoding-wide with a filled boundary or dataset is a hidden regime.
  `gut.weinberg-angle` is unification-scale, not `sin²θ_W(M_Z)`. GQW
  and the PDG interval name `M_Z`. SM anomalies, hypercharge, and
  hydrogen name one generation / hydrogen. Super-K proton lifetime
  stays encoding-wide until Super-K is a Dataset. Verified: those
  cells are not encoding-wide; `why` names the regime; Poincaré, Tr Q,
  and Super-K stay encoding-wide. Catalog d² hash unchanged. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Catalog claims name their axioms as AssumptionSet**
  (`IdentitySpec::assumption_set`, `Claim::with_assumptions`). Hidden
  coboundary / Minkowski / Einstein-addition hypotheses are a new
  identity, not a silent `encoding-is-the-model`. Lean kernel axioms
  stay on the receipt. Poincaré does not inherit the catalog coboundary
  set. Verified: named domain without catalog axioms cannot mint;
  `why` lists `discrete-coboundary` before prove. Live prove is still
  Lean+nanoda. `fmt`, `clippy -D warnings`, full suite, CLI.

- **Catalog identities name a DomainOfValidity**
  (`IdentitySpec.domain`). Encoding-wide Physlib forall is not the
  catalog FormalClaim: filling in the regime is a new hash. `why`
  prints regimes and approximations. Poincaré stays encoding-wide.
  Verified: encoding-wide physlib d² cannot mint ExactIdentity;
  live catalog claims are not encoding-wide; `why` shows d² simplex
  coboundary and interval `|β| < 1`. Live prove is still Lean+nanoda
  (new challenge hash: domain is in the FormalClaim). `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Journal restore binds prove/review to FormalClaim identity**
  (`statement_hash` on prove/review events). Restore remints a prove
  only when the recorded challenge is `Challenge::generate` of the live
  identity, and a review only when the recorded statement hash is that
  identity. A matching slug with unspecified commitments, a wrong
  challenge hash, or a slug-only legacy review line does not mint P3F
  or P3S. Verified: stale-identity journal prove/review stay
  unproved/unreviewed; live hashes still restore. Live d² prove/review
  journal restores P3F+P3S; forged hashes do not. `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Challenge is generate-only**
  (`Challenge` private fields, no Deserialize). The solver cannot
  construct the obligation it is judged against. `generate` fills the
  Lean type and polynomial from the matching catalog FormalClaim.
  Verified: compile-fail against Challenge literals and Deserialize;
  a generated challenge is hash-consistent; unspecified slug still
  cannot mint ExactIdentity. Live `prove` of catalog d² is still
  Lean+nanoda; challenge hash unchanged. `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **Catalog obligation is the FormalClaim, not the slug**
  (`IdentitySpec::lab_claim`, `lookup_matching`, `bind_catalog`).
  ExactIdentity and encoding review mint only when the live identity is
  the catalog sentence (quantifier, units, constants, conventions,
  Physlib). The same slug with unspecified commitments cannot borrow
  the polynomial, the Lean type, or the dossier. Live theories host
  catalog claims from the spec so hashes cannot drift. Verified:
  unspecified d² ExactIdentity is NoExactIdentity even if the catalog
  polynomial is attached; review is WrongIdentity; live prove/review
  still mint. `fmt`, `clippy -D warnings`, full suite, CLI (live d²
  Lean+nanoda P3F+P3S; identity hash unchanged).

- **P3S keys to statement hash, not slug**
  (`SemanticRecord.statement_hash`, `Lab::semantic_tag`). Encoding
  review mints against the live `FormalClaim`. Changing commitments
  keeps the lab slug and is a new identity: a stale `by_claim` lookup
  is not P3S. `why`, epistemics, inspect, and journal restore match.
  Verified: stale unspecified d² review does not inspect as P3S;
  reviewing the live identity does. `fmt`, `clippy -D warnings`, full
  suite, CLI (`review` identity hash matches `why`).

- **P3F keys to statement hash, not slug**
  (`Lab::has_live_receipt`). A dual-checked receipt proves the live
  `statement_hash`. Changing commitments keeps the lab slug and is a
  new identity: a stale `by_claim` lookup is not P3F, not a closed
  MissingTheorem gap, and not a reproduce. `why` already judged by
  hash; trust, inspect, gaps, the loop, and reproduce now match.
  Verified: stale unspecified d² receipt does not inspect as P3F;
  proving the live identity does. `fmt`, `clippy -D warnings`, suite.

- **FormalClaim identity is first-class**
  (`ClaimCommitments`, `Quantifier`). The statement hash commits to
  quantifiers, units, constants, boundary conditions, conventions,
  theory version, definitions, datasets, and formal-library identity,
  not just English prose. Changing forall/exists, a sign, a unit, a
  constant, or a boundary is a new hash; the lab slug is unchanged.
  Catalog identities (`d²`, interval, composition, mass shell) are
  `forall` in `physlib:unversioned`. Poincaré is not a catalog
  polynomial. `field.second-order-accurate` names `|k a| < 1` as its
  domain. GUT-scale `3/8` commits to the unification-scale boundary;
  the PDG interval cell commits to `pdg-2024-sin2theta`. Lean compiler
  versions stay on the receipt. Verified: unit tests per axis, why
  identity fields, `fmt`, `clippy -D warnings`, full suite, CLI.

- **P3N why is numeric certified**
  (`Judgment::from_lab`, `Verdict.numeric_lo` / `numeric_hi`).
  `CertifiedNumeric` Holds projects `numeric certified` with a display
  enclosure (`[0, 0]`, `[-1/2, -1/2]`, `[3/8, 3/8]`), not `logical
  undetermined`. Evaluator Holds without a receipt stays logical
  undetermined; dual-checked catalog stays `logical proved`; a coarse
  lattice stays `numeric unresolved`. Enclosure strings are display,
  not authority. Not Lean, not P3F, not P4. Verified: unit tests,
  why/inspect, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Coarse second-order is insufficient precision**
  (`field.second-order-accurate`). The O(a²) identity is a long-wavelength
  statement (`|k a| < 1` at the probe). Outside that domain, Richardson
  `p` is not a stencil verdict: the cell is `undecidable` /
  `inconclusive`, the gap is `InsufficientPrecision`, and `why`
  projects `numeric unresolved`. Not a failed theorem, not P3N, not
  Lean, not P3F, not P4. `set klein-gordon spacing 100` is the knob
  diff. Verified: unit tests, inspect/why, `fmt`, `clippy -D warnings`,
  full suite, CLI.

- **GUT Tr Q does not earn P3N**
  (`gut.charge-quantization`). `Tr Q = ΣY` is the gravitational
  `[grav]²U(1)` sum already certified on
  `consistency.anomaly-cancellation`. `Q = T₃ + Y` and `Σ T₃ = 0` make
  that an interpretation, not a second identity. The cell stays
  `executed`. GUT-scale `3/8` stays `CertifiedNumeric` / P3N.
  `inspect trust P3N` lists four cells. Verified: unit tests,
  inspect/why, `fmt`, `clippy -D warnings`, full suite, CLI.

- **GUT-scale 3/8 and Tr Q earn P3N**
  (`gut.weinberg-angle`, `gut.charge-quantization`). Weyl colour and weak
  dimensions are integers. `Tr(T₃²)/Tr(Q²) = 2 / (16/3) = 3/8` in Q, and
  `ΣY = 0`. Those overlays are `CertifiedNumeric` / P3N, not Lean, not
  P3F, not P4, and not Georgi–Quinn–Weinberg running at `M_Z` or the 3%
  band. `inspect trust P3N` lists five cells. Verified: exact Ratio
  tests, inspect/why P3N, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Hydrogen neutrality from T3+Y earns P3N**
  (`empirical.charge-quantization`, `Q = T₃ + Y`). Derived hypercharges
  give `Q_u = 2/3`, `Q_d = −1/3`, `Q_e = −1`; left-handed `T₃+Y` matches
  `−Y` of conjugate singlets; `2 Q_u + Q_d + Q_e = 0`. That overlay is
  `CertifiedNumeric` / P3N, not a catalog lookup of charge-thirds, not
  GUT `Tr Q`, not Lean, not P3F, not P4. A left/right mismatch Fails
  without P3N. `inspect trust P3N` lists three SM cells. Verified: unit
  tests, inspect/why P3N, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Hodge earns P2; Euler–Poincaré does not**
  (`dec.hodge-harmonic`, `Verdict::with_cross_checked`). Laplacian
  nullity versus coboundary `b₁` is a second matrix: forgetting the up
  or down term disagrees. That overlay is `CrossChecked` / P2, not a
  Lean receipt, not P3N, not P3F, not P4. Euler–Poincaré stays
  `executed`: `b₀−b₁+b₂ ≡ V−E+F` is rank-cancellation of the Betti
  formulas, not a second path. Poincaré stays `executed`.
  `inspect trust P2` lists Hodge (count 1). Verified: unit tests,
  inspect/why P2, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Exact SM hypercharge solve earns P3N**
  (`sm.hypercharge-derivation`, `Ratio::checked_sqrt`,
  `Verdict::with_certified_numeric`). Fixing `Y_Q = 1/6`, the four
  anomaly conditions are solved in Q. The `{Y_u, Y_d}` quadratic has
  discriminant `1`, a square; the roots are `{−2/3, 1/3}`. That overlay
  is `CertifiedNumeric` / P3N, not a Lean receipt, not P3F, not P4, and
  not the heuristic 3% GQW band. A non-square discriminant Fails without
  P3N. `inspect trust P3N` lists both SM cells (count 2). Verified:
  Ratio sqrt tests, exact derivation tests, inspect/why P3N, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Exact rational square roots**
  (`Ratio::checked_sqrt`, `Div`/`Neg`/`Ord`). `Some` only when numerator
  and denominator are perfect squares. Used by the hypercharge quadratic.
  Not a kernel proof. Verified: unit tests including the SM discriminant
  `1`, `fmt`, `clippy -D warnings`.

- **Exact SM anomalies earn P3N**
  (`consistency.anomaly-cancellation`, `Ratio`, `Verdict::with_certified_numeric`).
  The four chiral gauge sums over one generation are exact rationals
  that vanish, plus an even Witten doublet count. That overlay is
  `CertifiedNumeric` / P3N, not a Lean receipt, not P3F, not P4, and
  not the heuristic 3% GQW band. Green-Schwarz stays encoded. Verified:
  Ratio cubic identity, SM unit tests, inspect/why P3N, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Mass shell is a catalog identity**
  (`sr.energy-momentum-invariant`, Physlib `energy_momentum_invariant`).
  The polynomial `(E−βp)² − (p−βE)² − (1−β²)(E²−p²) ≡ 0` is the
  Minkowski bilinear form on 4-momentum: the interval identity with
  `(t,x) → (E,p)`, not a new postulate. Dual-expanded and
  kernel-checked. The typed rest-mass check remains the evaluator.
  Galilean `E' = E`, `p' = p − βE` is not an identity. The claim
  depends on `sr.invariant-interval`. Not Mathlib-scale. Not P4.
  Verified: expander and parse tests, exact mint, Lean+nanoda when
  wired, loop prove, `fmt`, `clippy -D warnings`, full suite, CLI.

- **Super-K prose is not a dataset**
  (`gut.proton-lifetime-sk`, `super_kamiokande_proton_lifetime`).
  The empirical proton-lifetime cell stays `untested` because no
  Super-Kamiokande Dataset is registered. The heuristic
  `gut.proton-decay-viable` still quotes Super-K as M_GUT prose and
  still flips with SUSY. Do not mint a lifetime number to close the
  gap. `inspect gap missing-dataset` lists the cell. Not P4.
  Verified: physis-data None test, GUT and lab tests, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Decidable is not feasible**
  (`comp.feasible-decision`, `gap_for`). Circuit equivalence is
  coNP-complete; a bounded tape has a finite configuration graph this
  lab does not enumerate. Those evaluations are `ComputationallyIntractable`,
  not Rice / the halting problem. Unbounded TM feasible-decision is
  inapplicable (the obstruction is computability). Combinational
  `comp.decidable-equivalence` still Holds. No simulator. Not P vs NP
  as holds/fails. Not P4. Verified: computation and gap_for unit tests,
  inspect/gaps lab tests, tape_bound knob diff, `fmt`,
  `clippy -D warnings`, full suite, CLI.

- **Overlap without containment is insufficient precision**
  (`EmpiricalReceipt`, `gap_for`, `gut.weinberg-angle-mz-interval`).
  Compatible now means the prediction interval is a subset of the data
  hull, not merely overlapping. A wide theory envelope that overlaps
  PDG `sin²θ_W(M_Z)` is `inconclusive`, not compatible. Minimal SU(5)
  remains excluded. Turning on SUSY makes the 3% GQW band overlap
  without fitting inside the PDG hull, so `inspect gap
  insufficient-precision` lists the empirical cell; the heuristic GQW
  cell can still hold. The 3% band is the existing heuristic threshold,
  not a remainder certificate, and is not GUT-scale `3/8`. Not P4.
  Verified: interval contains tests, three-way receipt tests, gap_for
  unit test, GUT and lab tests, `fmt`, `clippy -D warnings`, full
  suite, CLI.

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
