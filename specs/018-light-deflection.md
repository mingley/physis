# 018 — Solar-system gravity (Newton vs Einstein)

Status: active
Layer: spacetime
Id: `gravity`

## Purpose

Put inverse-square gravity on trial with the two solar-system numbers that
ended its monopoly: Eddington's 1.75″ solar light deflection and Mercury's
43″/century perihelion remainder. Both are RK4 integrals of the Binet
equation, not slogans.

Newtonian corpuscular light (Soldner 1801; Einstein's 1911 equivalence
result) deflects by `2 GM/(c² R)`. Schwarzschild null geodesics deflect by
`4 GM/(c² R)`. Inverse-square bound orbits are closed ellipses; the
Schwarzschild term `3 (GM/c²) u²` advances perihelion by
`6π GM / (c² a (1−e²))` per orbit.

## Objects

| id | object |
|---|---|
| `newtonian-gravity` | inverse-square gravity, corpuscular light (Binet IR) |
| `general-relativity` | Einstein gravity (also in `string-critique`) |

## Knobs

`newtonian-gravity` has none: topology is not a knob. The standing encoding
is inverse-square Binet (`binet inverse-square`). `add-schwarzschild`
appends `binet 3GM u^2` and is an IR mutation: the half-angle fails and
Eddington / Mercury hold on that fork. `add-yukawa` appends
`potential yukawa` and is a second IR mutation: `μR K_1(μR)` suppresses
the inverse-square Soldner angle and the half-angle fails, while
Eddington / Mercury still fail. That is still `newtonian-gravity`,
not a silent GR install. `general-relativity` keeps `dim` and
`cosmological_constant`. The Einstein–Hilbert action is a live IR
package (`action einstein-hilbert`). `add-r-squared` appends
`action r-squared` and is an IR mutation: uniqueness of Einstein
gravity plus Λ fails. `add-brans-dicke` appends `action brans-dicke`
and is a second IR mutation: PPN `γ = (ω+1)/(ω+2)` at `ω = 1` is not
1, so Eddington and Mercury fail while uniqueness also fails. That is
still `general-relativity`, not a silent Newton install. The solar
tests are 4D; `set general-relativity dim 5` makes them **inapplicable**.
GR also keeps the `chart` knob (`schwarzschild` or
`eddington-finkelstein`) and the `r_min` knob (inner domain edge in
units of `M`, 0.5–9; outer edge fixed at 10M): conventions and domain
are inputs of the reciprocal check, not hidden assumptions.

## Claims

| id | meaning | Newton | GR (D=4) |
|---|---|---|---|
| `gr.newton-half-deflection` | grazing δ = `2 GM/(c² R)` ≈ 0.87″ | **holds** | **fails** (twice that) |
| `gr.eddington-deflection` | grazing δ = 1.75″ | **fails** | **holds** |
| `gr.mercury-perihelion` | extra Δω = 43″/century | **fails** (closed ellipses) | **holds** |
| `gr.schwarzschild-reciprocal` | `g_tt·g_rr = −1` on the exterior domain | n/a (GR object only) | **holds** (Schwarzschild chart, `r_min > 2M`); **fails** (EF chart, `g_rr = 0`); **inapplicable** (`r_min ≤ 2M` in Schwarzschild chart, or D ≠ 4) |
| `gr.first-bianchi` | computed `R^ρ_[σμν] = 0` on the domain | n/a (GR object only) | **holds** (both charts, both sides of the EF horizon); **inapplicable** (`r_min ≤ 2M` in Schwarzschild chart, or D ≠ 4). Catalog identity + physlib `first_bianchi`, bound on the live package |

On `newtonian-gravity`, those three cells name DomainOfValidity inverse-square
Binet. `add-schwarzschild` appends `binet 3GM u^2` and the half-angle fails
while Eddington / Mercury hold. `add-yukawa` appends `potential yukawa` and
the half-angle fails while Eddington / Mercury still fail. That is not a
knob. GR's copies stay encoding-wide. `set general-relativity dim 5` is
still the 4D inapplicable knob, not this fork.

## What is computed

Shared RK4 on `u'' + u = rhs(u)` (`u = 1/r`):

| problem | Newtonian `rhs` | GR `rhs` |
|---|---|---|
| light, periapsis `u = 1/R` | `(GM/c²)/R²` | `3 (GM/c²) u²` |
| Mercury, perihelion `u = 1/(a(1−e))` | `1/(a(1−e²))` | Kepler + `3 (GM/c²) u²` |

Light: integrate from periapsis until `u = 0`; deflection is
`2 (φ_∞ − π/2)`. Mercury: next perihelion minus `2π`, times orbits per
Julian century, in arcseconds. Both are checked against `2 GM/(c² R)`,
`4 GM/(c² R)`, and `6π GM/(c² a (1−e²))`. Default RK4 uses
`n = 80_000` (`h = π/(2n)`) for light and `n = 120_000` (`h = 2π/n`)
for Mercury. Evidence names three error sources: IAU `GM_sun` / `R_sun`
exact Ratio conversion rulers (not measured solar mass/radius); Mercury
`a,e` point Qty with no hull (not a dataset); `f64` RK4 /
`NumericTier::Approximate`; and that step count. The RK4 remainder is
**not** a certificate. A coarse `n` whose truncation hull overlaps the
relative 3% Eddington band or the 1.5″ Mercury band without containment
is `Undecidable` (`numeric unresolved`), not a theorem and not
`CertifiedNumeric`. Wrapping a point estimate in an interval does not
certify the integral. 1.75″ compares to the hashed Sobral 1919 dataset
(`dyson-1920-sobral-deflection`: 1.98″ ± 0.12″ probable error, σ = 0.18″,
public domain) under the predeclared two-sigma rule
(`EmpiricalReceipt::compare_two_sigma`: interval-subset against the μ±2σ
hull plus the exact Gaussian NLL), applied identically to the GR and
Soldner predictions; a numerically undecided sample withholds comparison.
The 1σ hull is honestly stored and is not the exclusion rule. 42.98″
compares to the hashed Clemence 1947 residual dataset
(`clemence-1947-mercury-residual`: 42.56 ± 0.94 probable error per
Julian century, moving equinox, σ = 1880/1349) under the same
predeclared two-sigma rule, applied identically to the GR and Newtonian
predictions; a numerically undecided sample withholds comparison. The
dataset is the measured residual (observed 5599.74 minus computed
Newtonian 5557.18), not the total and not Clemence's Table I 43.03.

`GM_☉` is the IAU standard gravitational parameter, so `GM/c²` is a typed
`Qty<Length>` (half the Schwarzschild radius) without folding in the
uncertainty on `G`.

The reciprocal check samples `g_tt·g_rr` at 8 points over
`[r_min, 10M]` (geometric units, `M = 1`, signature `−+++`) in the
selected chart: `−(1−2M/r)` times `1/(1−2M/r)` in Schwarzschild
coordinates (product `−1`), `−(1−2M/r)` times `0` in advanced EF
coordinates (product `0`). A Jacobian cross-check (`t = v − r*`,
`g'_rr = (∂t/∂r)²g_tt + g_rr → 0`) reproduces the direct EF `g_rr`
away from the horizon (`|r − 2M| > 0.1M`); disagreement is
`undecidable` numerics. A Schwarzschild-chart domain with
`r_min ≤ 2M` is refused (the coordinate singularity is an input,
not a silent extrapolation); EF coordinates are regular at the
horizon and compute. The `r = 0` curvature singularity is never
sampled. The identity holds for any static spherical `f(r)`, so it
is chart geometry — a checked encoding, never an Einstein-equation
proof.

The live tensor engine derives Γ from numeric metric derivatives and
Riemann from numeric connection derivatives (central differences,
`h = 1e-4`, generic 4×4 inverse for the EF off-diagonal), contracts
the first-Bianchi cyclic sum, and anchors against Kretschmann
`K = 48M²/r⁶` (both charts agree: it is an invariant). Assumptions:
4D, `M = 1` geometric units, `Λ = 0` Schwarzschild, exterior domain
shared with the reciprocal check, torsion-free Levi-Civita by
construction, fixed probe angles (poles excluded). The residual
bound is finite-difference truncation, not a certificate; a ΓΓ
sign mutation breaks it. Unlike the reciprocal identity, Bianchi is
tensorial: it holds in both charts, including through the EF
horizon.

The matching formal obligation is the `R^t_[rθφ]` Bianchi component
as an integer polynomial in folded `(Γ, ∂Γ)` indeterminates (lower
Γ pairs shared: torsion-free), 21 indeterminates and 30 terms,
vanishing by the ΓΓ pairing. It is a catalog identity
(`gr.first-bianchi`, `integer-arithmetic`) with a physlib
`first_bianchi` theorem closed by `ring`; the live GR package
carries the generated tree plus a bound `lean_ref`, and
sign-mutated artifacts are rejected by expansion and at bind time.
The contracted Bianchi `∇·G = 0` is not formalized: its proof needs
differential hypotheses (symmetry of mixed partials) outside the
bare-polynomial fragment — Mathlib-scale GR, a later era. No
Einstein-equation proof is claimed. The EH action itself stays a
token; only the Bianchi tree rides the package.

## Knob → verdict

```
physis experiment gravity
physis run newtonian-gravity
physis run general-relativity
physis hypothesize newtonian-gravity   # add-schwarzschild and add-yukawa are IR, not set
physis hypothesize general-relativity  # add-r-squared and add-brans-dicke are IR, not set
physis set general-relativity dim 5   # solar tests become inapplicable
physis set general-relativity chart eddington-finkelstein  # reciprocal holds → fails (g_rr = 0)
physis set general-relativity r_min 1.5  # Schwarzschild chart: reciprocal holds → inapplicable (horizon)
```

## Honesty

- The 43″ is the *remainder* after Newtonian perturbations of the other
  planets (~531″), which this lab does not integrate. The theorem is that
  the Schwarzschild geodesic supplies that remainder, while a 1/r² ellipse
  supplies none. 43″/century is the planetary remainder this encoding
  compares to the hashed Clemence 1947 residual (C3.2). The Eddington
  cell's dataset is Sobral 4-inch only; the companion Principe result
  (1.61″ ± 0.30″ probable error) is documented here and not registered.
  The Mercury cell's dataset is the Clemence residual only; modern PPN
  ephemeris fits constrain different observables and are not this
  comparison.
- Weak-field RK4, not a full numerical relativity evolution. The RK4
  remainder is not a certificate: a coarse-step point estimate wrapped in
  an interval is not `CertifiedNumeric`.
- Einstein 1911 (equivalence only) agrees with Newton on light; 1915 spatial
  curvature doubles it. This lab's "Newton" column is that half-angle.
- The Schwarzschild Binet fork is a Newton IR mutation, not an install of
  `general-relativity`. The Yukawa potential fork is a second Newton IR
  mutation: it is not GR (Eddington / Mercury still fail). GR remains the
  separate 1915 object. Quadratic curvature on GR is an IR mutation of
  the Einstein–Hilbert action, not a Newton Binet term and not `dim`.
  Brans–Dicke on GR is a second IR mutation: PPN `γ` is not 1, so
  Eddington and Mercury fail. That is not Soldner and not `dim`.
- The reciprocal identity is a checked chart encoding, not evidence
  that Schwarzschild solves the Einstein equations (Ricci-flatness is
  not computed here). It is also convention-relative by construction:
  EF coordinates honestly fail it while remaining regular at the
  horizon where Schwarzschild coordinates refuse.

## Related

- `specs/014-special-relativity.md` (flat-space kinematics; `absolute_time` knob)
- `specs/005-string-critique.md` (GR as a control on the string matrix)
