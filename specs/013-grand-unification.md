# 013 — Grand unification (SU(5))

Status: active
Layer: interaction / particle / effective
Id: `su5-gut`

## Purpose

The Georgi–Glashow SU(5) grand unified theory sits one layer above the Standard
Model and sharpens the lab's central theme — **accommodate vs derive** — with
real empirical stakes. The SM *accommodates* charge quantization and the weak
mixing angle as inputs; SU(5) *derives* both from the single requirement that
one generation of fermions fills a complete SU(5) multiplet (`5̄ ⊕ 10`).
Complete `5bar + 10` lives on the IR package of `su5-gut`. A missing 10
(`add-missing-10`) is not a `supersymmetric` knob.

It is equally a lesson in honest failure: minimal (non-SUSY) SU(5) does **not**
unify the gauge couplings and predicts proton decay at a rate already excluded
by Super-Kamiokande. Those claims `fail`. This is a theory the lab records as
empirically refuted — exactly the epistemic honesty the project is built on.

## Object

| id | object |
|---|---|
| `su5-gut` | Georgi–Glashow SU(5). Complete `5bar + 10` lives on the IR package; `add-missing-10` is not a knob |

## Knob

| knob | effect |
|---|---|
| `supersymmetric` | MSSM matter. Flips `gut.coupling-unification` and `gut.proton-decay-viable` from `fails` to `holds` (as `heuristic`s), at the price of unobserved superpartners. A missing 10 is not this knob: `add-missing-10` is an IR mutation. |

## Claims

| id | meaning | status |
|---|---|---|
| `gut.sm-embedding` | SM fermions fill `5̄ ⊕ 10`. On `su5-gut` this names a complete 5bar + 10 generation. `add-missing-10` appends `missing 10` and the cell fails. That is not a knob. `supersymmetric` stays a knob | `encoded-fact`, holds (verified chain). Domain: complete 5bar + 10 generation |
| `gut.charge-quantization` | `Tr Q = 0` over the multiplet forces quantized charge | **executed** (`ΣY`, already the grav anomaly; not a second P3N) |
| `gut.weinberg-angle` | `sin²θ_W = 3/8` at unification | **exact Ratio** / P3N, not GQW at `M_Z`. Domain: unification-scale |
| `gut.weinberg-angle-mz` | GQW running of that 3/8 down to `M_Z` matches 0.231 | **computed** (one-loop RGE), knob-sensitive, `heuristic`. Domain: `M_Z` |
| `gut.weinberg-angle-mz-interval` | the same one-loop *algebraic* centre, enclosed by sourced PDG 2022 `α_s` / `α_em⁻¹` one-sigma hulls, vs the PDG mixing-angle hull **and** the exact Gaussian NLL of that centre rounded to PDG `10^{-5}` vs PDG σ | **empirical receipt** (interval-subset) plus **statistical computed** (not P3N). Minimal SU(5) **excluded**; one-loop MSSM **inconclusive** (input-σ hull overlaps `10^{-5}` but is not contained). MSSM has the smaller NLL. The heuristic 3% cell can still hold. Super-K is not this Gaussian. Not GUT-scale `3/8`. Domain: `M_Z` |
| `gut.coupling-unification` | the three couplings meet at one scale | **computed** (one-loop RGE), knob-sensitive |
| `gut.coupling-unification-interval` | the same one-loop *algebraic* `α_3(M_Z)` centre, enclosed by sourced PDG 2024 `sin²θ_W` / PDG 2022 `α_em⁻¹` one-sigma hulls, vs the PDG `α_s` hull **and** the exact Gaussian NLL of that centre rounded to PDG `10^{-4}` vs PDG σ | **empirical receipt** (interval-subset) plus **statistical computed** (not P3N). Minimal SU(5) **excluded**; one-loop MSSM **inconclusive** (input-σ hull overlaps `α_s` but is not contained). Complementary to GQW (predicts `α_3` from the mixing angle, not the mixing angle from `α_s`). The heuristic 3% cell can still hold. Not GUT-scale `3/8`. Domain: `M_Z` |
| `gut.proton-decay-viable` | predicted `τ_p` consistent with experiment | knob-sensitive (tied to computed `M_GUT`), `heuristic` |
| `gut.proton-lifetime-sk` | dim-6 `τ/B(p→e+π0)` from `M_GUT^4` vs Super-Kamiokande **dataset** | **empirical receipt**. Takenaka et al. Phys. Rev. D 102, 112011 (2020), 90% CL `> 2.4×10³⁴ yr`. Minimal SU(5) **excluded**; MSSM dim-6 **compatible**. Decade envelope for missing matrix elements. Not P3N, not dim-5, not `p→μ+π0`. Domain: `p→e+π0` / dim-6 / Super-K 90% CL. |

## The two computed theorems

Both numbers are computed from the *same* `SM_WEYL_FIELDS` table the anomalies
use (`crates/physis-theory/src/standard_model.rs`), so there is one source of
fermion truth:

- **Charge quantization** (`gut_trace_charge_exact`): `Q` is a traceless generator of
  SU(5), so `Tr Q = Σ colour·weak·Y = ΣY = 0` over one generation as an exact
  `Ratio`. That sum is the gravitational `[grav]²U(1)` anomaly already
  certified as P3N on `consistency.anomaly-cancellation`. The GUT cell
  stays `executed`: `Q = T₃ + Y` and `Σ T₃ = 0` do not mint a second
  certificate. Charge is still a *consequence* of the embedding, not a
  postulate — but the overlay is not `CertifiedNumeric`.
- **Weak mixing angle** (`gut_weinberg_traces_exact`): because the SU(5) generators are
  equally normalized at unification, `sin²θ_W = Tr(T₃²)/Tr(Q²)` over a complete
  multiplet. Using `Q = T₃ + Y` and `Σ T₃ = 0` per weak multiplet, the sums give
  `ΣT₃² = 2` and `ΣQ² = 16/3`, so `sin²θ_W = 3/8` exactly in Q. Overlay
  `CertifiedNumeric` / P3N. This is **not** Georgi–Quinn–Weinberg running to `M_Z`
  and not the heuristic 3% band.

Honesty note: `3/8 = 0.375` is the *boundary condition at the GUT scale*.
Georgi–Quinn–Weinberg running of that number down to `M_Z` is
`gut.weinberg-angle-mz`: it uses `α_em` and `α_s` only (not the measured
mixing angle), so it is not tautological with `3/8`.

## Gauge-coupling unification is *computed*, not asserted

`gut.coupling-unification` is backed by an actual one-loop
renormalization-group computation (`crates/physis-theory/src/rge.rs`). Each
inverse coupling runs linearly in `t = ln(μ/M_Z)`:

```text
α_i⁻¹(μ) = α_i⁻¹(M_Z) − (b_i / 2π) · t
```

with the standard one-loop coefficients `b = (41/10, −19/6, −7)` for the SM and
`(33/5, 1, −3)` for the MSSM. Starting from the measured electroweak inputs at
`M_Z` (`α_em⁻¹`, `sin²θ_W`, `α_s`, in `physis-model` constants), the code fixes
the unification point from the `α_1`/`α_2` crossing and **predicts** `α_3(M_Z)`:

- **Minimal SM**: predicted `α_3(M_Z) ≈ 0.071` vs measured `0.118` — a ~40%
  miss, with a low `M_GUT ≈ 10¹³ GeV`. The couplings do not meet.
- **MSSM**: predicted `α_3(M_Z) ≈ 0.117` vs measured `0.118` — agreement to
  ~1%, with `M_GUT ≈ 2×10¹⁶ GeV`. The couplings (nearly) meet.

Both verdicts are `Heuristic` (one loop is an approximation; two-loop terms and
SUSY thresholds shift the percent-level numbers, and MSSM unification rests on
unobserved superpartners), but the numbers in the evidence are genuinely
computed. The low SM `M_GUT` is also what feeds `gut.proton-decay-viable`: the
dimension-6 rate scales as `M_GUT⁻⁴`, so a low scale means a short, excluded
lifetime.

`gut.coupling-unification-interval` is the empirical sibling: the same
one-loop algebraic centre (a π-free `Ratio` of recorded PDG `α_em⁻¹` and
`sin²θ_W`) enclosed by the sourced PDG 2024 mixing-angle hull and the
PDG 2022 `α_em⁻¹` hull, compared to the registered PDG `α_s(M_Z)` hull
under the interval-subset rule, **and** the exact Gaussian NLL of that
centre rounded to four decimal places versus the PDG σ. Complementary
to GQW, which uses `α_s` to predict the mixing angle. Compatible means
the prediction lies inside the data. Minimal SU(5) is **excluded**. The
one-loop MSSM enclosure overlaps the PDG `α_s` hull but is not contained
in it, so the cell is **undecidable** / `inconclusive` while the
heuristic 3% cell can still **hold**. Not P3N, not GUT-scale `3/8`.

## Georgi–Quinn–Weinberg: `3/8` run down to `M_Z`

`gut.weinberg-angle-mz` is the low-energy sibling of the `3/8` theorem. Given
`α_em(M_Z)` and `α_s(M_Z)`, one-loop unification `α_1 = α_2 = α_3` at a single
`M_U` **predicts** `sin²θ_W(M_Z)`:

```text
t = 2π (α_em⁻¹ − 8/3 α_s⁻¹) / [(5/3)(b₁−b₃) + (b₂−b₃)]
α₂⁻¹(M_Z) = α_s⁻¹ + (b₂−b₃) t / 2π
sin²θ_W(M_Z) = α₂⁻¹(M_Z) / α_em⁻¹
```

This does **not** use the measured mixing angle (that would recover `3/8` at
the `α_1`/`α_2` crossing by construction). Complementary to
`gut.coupling-unification`, which uses the measured `sin²θ_W` to fix `α_1`/`α_2`
and predicts `α_3`.

- **Minimal SU(5)**: predicted `sin²θ_W(M_Z) ≈ 0.207` vs measured `0.231` — a
  ~10% miss. The claim **fails**.
- **MSSM**: predicted `sin²θ_W(M_Z) ≈ 0.231` vs `0.231` — agreement to a few
  parts per thousand, at `M_U ≈ 2×10¹⁶ GeV`. The claim **holds** as a
  `heuristic`.

`gut.weinberg-angle-mz-interval` is the empirical sibling: the same one-loop
algebraic centre (a π-free `Ratio` of recorded PDG `α_em⁻¹` and `α_s`)
enclosed by the sourced PDG 2022 one-sigma hulls of those inputs
(`α_s(M_Z) = 0.1179(9)`, `α_em⁻¹(M_Z) = 127.951 ± 0.009`), compared to the
registered PDG `sin²θ_W(M_Z)` hull under the interval-subset rule, **and**
the exact Gaussian NLL of that centre rounded to five decimal places versus
the PDG σ. Compatible means the prediction lies inside the data, not that
the intervals merely overlap. The NLL is a `statistical computed` judgment,
not P3N. `physis enclose` independently parses the input-interval endpoints
as a canonical `Interval`; that is not `CertifiedNumeric` and is not P3N.
Minimal SU(5) is **excluded**. The one-loop MSSM enclosure
overlaps the PDG hull but is not contained in it, so the cell is
**undecidable** / `inconclusive` (`InsufficientPrecision`) while the
heuristic 3% cell can still **hold**. The input enclosure is not a
two-loop remainder certificate, and it is not the GUT-scale `3/8`.

`set su5-gut supersymmetric true` flips the heuristic mixing-angle cell
`fails → holds` and the interval cell `fails → undecidable`.

### Two-loop running

The verdict also carries a **two-loop** result: `GaugeRunning` integrates the
coupled two-loop RGEs `d(α_i⁻¹)/dt = −b_i/2π − (1/8π²)·Σ_j b_ij α_j` with RK4
(the gauge two-loop matrices `b_ij` for the SM and MSSM), finds the
`α_1⁻¹ = α_2⁻¹` crossing, and reports the residual `α_3⁻¹` gap there. The
qualitative picture is unchanged and sharpened: minimal SU(5) misses (≈12% gap,
`M_GUT ≈ 10¹³ GeV`), while the MSSM meets to a few percent at
`M_GUT ≈ 3×10¹⁶ GeV` — the phenomenological unification scale. The residual
two-loop gap in the MSSM is the well-known few-percent discrepancy closed by
SUSY threshold corrections, which this milestone does not model.

## The knob → verdict diff

```
physis run su5-gut               # coupling-unification, proton-decay, weinberg-angle-mz: fail
physis hypothesize su5-gut       # add-missing-10 is IR, not set
physis encode su5-gut            # complete 5bar + 10; not P3S, not a kernel proof
physis set su5-gut supersymmetric true
```

`physis hypothesize su5-gut` forks the package with a missing 10
(`equation missing 10`); that is not a knob (`set su5-gut missing_10`
is unknown). The incomplete encoding fails `gut.sm-embedding` while
GUT-scale `3/8` still holds. `supersymmetric` stays a knob. Mutants
are not installed.

`set su5-gut supersymmetric true` flips `gut.coupling-unification`,
`gut.proton-decay-viable`, and
`gut.weinberg-angle-mz` `fails → holds` (as `heuristic`s), because switching the
beta coefficients from SM to MSSM brings the computed `α_3(M_Z)` and
`sin²θ_W(M_Z)` into agreement and raises `M_GUT`. The interval cell
`gut.weinberg-angle-mz-interval` flips `fails → undecidable`: the sourced
PDG input-σ hull now overlaps the mixing-angle measurement but is not
contained in it. `gut.coupling-unification-interval` flips the same way
against the PDG `α_s` hull. The Super-K cell
`gut.proton-lifetime-sk` flips `fails → holds` on the empirical axis
(`excluded → compatible`): the dim-6 `M_GUT^4` envelope sits below the
Takenaka et al. 90% CL hull for minimal SU(5) and inside it for MSSM dim-6.
That is not P3N and not a dimension-5 operator. Minimal SU(5) is falsified;
SUSY SU(5) survives current dim-6 Super-K bounds but requires superpartners
that have not been seen, and its one-loop GQW input-interval enclosure is
too coarse for an empirical support receipt.

## Relation to the string-critique

The heterotic gauge chains verified in `crates/physis-model/src/gauge.rs`
(`E₈ ⊃ E₆ ⊃ SO(10) ⊃ SU(5) ⊃ SM`) pass through SU(5). This GUT layer is where a
string compactification would have to *land* to make contact with observed
physics — and where the "does the alternative earn empirical contact?" question
becomes concrete.

## Non-goals (this milestone)

- A two-loop GQW *prediction* of `sin²θ_W(M_Z)` (the one-loop closed form is
  the historical theorem; two-loop shooting is later).
- SUSY threshold corrections and Yukawa contributions to the two-loop running
  (the gauge-only two-loop RGEs are integrated; thresholds close the residual).
- A dynamical proton-decay *rate* from the dimension-6 operator coefficients
  (the verdict uses the computed `M_GUT` in the order-of-magnitude `M_GUT^4`
  scaling with a decade envelope; not lattice matrix elements).
- Dimension-5 SUSY proton decay (this cell is the dim-6 `p→e+π0` mode only).
- SO(10)/E₆ as separate theories (the embedding chain already reaches them).

## Related

- `specs/005-string-critique.md` — the SM anomaly/hypercharge theorems
- `specs/004-theories-and-claims.md` — the `Theory` trait
- `plans/003-m2-empirical-contact.md` — the empirical-contact milestone
