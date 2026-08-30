# M3 — Domain reuse (electricity, then computation)

Goal: prove the workspace is not a string-theory toy by hosting a second scientific domain without forking core.

## Electricity (first reuse)

New theory `maxwell-vacuum` (and later `linear-medium`):

- Knobs: none in vacuum except unit system; in a medium, `epsilon_r`, `mu_r`
- Claims:
  - `1/√(ε₀μ₀) = c` as a **theorem** of the encoding
  - Faraday / Ampere structure as encoded facts, then as typed exterior calculus if we get that far
- Control: ✅ `ohm-circuit` — lumped circuit theory as the quasi-static effective limit of Maxwell. Charge conservation = Kirchhoff's current law on a lumped IR netlist (`add-tline` is an IR fork, not a knob); wave propagation dropped (`wave-speed-c` inapplicable), preferred frame (`lorentz-invariance` fails). The `frequency_hz` knob flips `em.quasi-static-valid` when the wavelength stops dwarfing the circuit.

This domain is chosen because it *needs* `Qty` immediately (cannot add volts to amperes) and produces clean knob diffs (`epsilon_r` changes wave speed).

## Computation (second reuse) — ✅ done

New theories `combinational-circuit` and `turing-machine` with the `computation`
experiment (`physis_theory::computation`, `specs/009-computation.md`):

- Knob: `turing-machine tape_bound` (0 = unbounded).
- Claims: halts, turing-complete, deterministic, decidable-equivalence,
  resource-bounded.
- The halting problem is encoded as a genuine `Undecidable` verdict for the
  unbounded machine; `set turing-machine tape_bound 1000` flips halts,
  turing-complete, decidable-equivalence, and resource-bounded.
- ✅ Landauer / reversible computing (`landauer-engine`) on `statistical` +
  `information`: `info.landauer-cost` = `k_B·T·ln2` computed as a typed
  `Qty<Energy>` (a theorem), and `set landauer-engine reversible true` flips
  `info.thermodynamically-free` `fails → holds` — a cross-domain
  (computation ↔ thermodynamics) knob → verdict diff.
- The `World` projection was generalized to `Option<World>`; non-physics
  objects return `None` and describe themselves via `note()`.
- Later: complexity classes as verdicts; actual interpreters/simulators.

## Crate split

If model types get heavy, add `physis-em` / `physis-info` crates. Do not copy `physis-core`.

## Done when

- ✅ `physis experiment em-vacuum` prints a matrix (M3, `physis_theory::em`)
- ✅ At least one electricity knob → verdict diff test (`em::tests::permittivity_knob_flips_the_wave_speed_verdict`)
- ✅ Spec `specs/008-electromagnetism.md` exists
- ✅ `specs/007-reuse-domains.md` is still accurate

## Status

Electricity's reuse is delivered: `maxwell-vacuum`, `linear-medium`, and the
`ohm-circuit` control, the `em-vacuum` experiment, and the `1/√(ε₀μ₀) = c`
theorem checked from typed `ε₀`/`μ₀` constants — all on the same substrate, no
core fork. The computation domain (`combinational-circuit`, `turing-machine`,
`specs/009`) is also delivered, now including `landauer-engine` — Landauer's
principle as a typed-energy theorem bridging computation and thermodynamics. The
experiment machinery was generalized (`report_from_rows`) so domains supply
their own theory list and claim rows, and the physics-shaped `World` projection
was generalized to `Option<World>` for non-physics domains. Still open in M3:
typed exterior calculus for the field equations, and complexity classes as
verdicts.
