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
`cosmological_constant`. The solar tests are 4D; `set general-relativity
dim 5` makes them **inapplicable**.

## Claims

| id | meaning | Newton | GR (D=4) |
|---|---|---|---|
| `gr.newton-half-deflection` | grazing δ = `2 GM/(c² R)` ≈ 0.87″ | **holds** | **fails** (twice that) |
| `gr.eddington-deflection` | grazing δ = 1.75″ | **fails** | **holds** |
| `gr.mercury-perihelion` | extra Δω = 43″/century | **fails** (closed ellipses) | **holds** |

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
`4 GM/(c² R)`, and `6π GM/(c² a (1−e²))`.

`GM_☉` is the IAU standard gravitational parameter, so `GM/c²` is a typed
`Qty<Length>` (half the Schwarzschild radius) without folding in the
uncertainty on `G`.

## Knob → verdict

```
physis experiment gravity
physis run newtonian-gravity
physis run general-relativity
physis hypothesize newtonian-gravity   # add-schwarzschild and add-yukawa are IR, not set
physis set general-relativity dim 5   # solar tests become inapplicable
```

## Honesty

- The 43″ is the *remainder* after Newtonian perturbations of the other
  planets (~531″), which this lab does not integrate. The theorem is that
  the Schwarzschild geodesic supplies that remainder, while a 1/r² ellipse
  supplies none.
- Weak-field RK4, not a full numerical relativity evolution.
- Einstein 1911 (equivalence only) agrees with Newton on light; 1915 spatial
  curvature doubles it. This lab's "Newton" column is that half-angle.
- The Schwarzschild Binet fork is a Newton IR mutation, not an install of
  `general-relativity`. The Yukawa potential fork is a second Newton IR
  mutation: it is not GR (Eddington / Mercury still fail). GR remains the
  separate 1915 object.

## Related

- `specs/014-special-relativity.md` (flat-space kinematics; `absolute_time` knob)
- `specs/005-string-critique.md` (GR as a control on the string matrix)
