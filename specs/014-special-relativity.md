# 014 — Special relativity

Status: active
Layer: spacetime / particle
Id: `special-relativity`

## Purpose

Special relativity is a foundational, spectacularly-confirmed theory — an ideal
target for *mechanical* scrutiny. This theory does not assert Einstein's 1905
kinematics; it **computes** three of its invariants and shows that a single knob
(`absolute_time`) demolishes all of them by reverting to Galilean boosts. It is
the Galilean→Einstein revolution rendered as one mechanical knob turn.

It also exercises the project's core premise — Rust's type system as a physics
guardrail. The mass-shell check builds `pc` and `mc²` from typed quantities, so
the compiler *forces* both to have dimensions of energy; a dimensionally wrong
expression would not compile.

## Object

| id | object |
|---|---|
| `special-relativity` | flat Minkowski kinematics with a Galilean-toggle knob |

## Knob

| knob | effect |
|---|---|
| `absolute_time` | If `true`, boosts are Galilean (time is absolute) instead of Lorentzian. Flips all three claims `holds → fails`. |

## Claims (all computed theorems)

| id | meaning |
|---|---|
| `sr.invariant-interval` | `s² = (cΔt)² − Δx²` is unchanged by the boost |
| `sr.subluminal-composition` | composing two subluminal velocities stays below `c` |
| `sr.energy-momentum-invariant` | the mass shell `E² − (pc)² = (mc²)²` is frame-independent |

## How each is computed

Both the spacetime coordinates `(cΔt, Δx)` and the energy–momentum `(E, pc)` are
4-vectors, so `SpecialRelativity::boost` applies the *same* transform to each —
Lorentz by default, Galilean under the knob:

- **Interval**: a timelike event `(c·10 ns, 2 m)` is boosted by `β = 0.6`. Under
  Lorentz, `s²` is unchanged (holds); under a Galilean boost `x' = x − βct` with
  `t` absolute, `s²` changes (fails).
- **Velocity composition**: `0.8c ⊕ 0.7c`. Relativistic `(u+v)/(1+uv/c²) ≈
  0.9615c < c` (holds); Galilean `u+v = 1.5c ≥ c` (fails).
- **Mass shell**: a particle at rest has `(E, pc) = (mc², 0)`, with `mc²` a typed
  `Qty<Energy>` built from the electron mass and `c`. Boosting the 4-momentum
  and checking `E² − (pc)²` against `(mc²)²` holds under Lorentz and fails under
  a Galilean boost.

## The knob → verdict diff

```
physis run special-relativity            # all three: holds
physis set special-relativity absolute_time true
```

flips every claim `holds → fails`. Absolute time is not a small perturbation of
relativity — it breaks the interval, lets velocities exceed `c`, and destroys
the mass shell, all at once.

## Non-goals (this milestone)

- General Lorentz group representations (only boosts along one axis).
- Thomas precession, relativistic Doppler, and other second-order effects.
- Dynamics (forces, radiation); this is pure kinematics.

## Related

- `specs/001-type-system.md` — the typed quantities the mass shell relies on
- `crates/physis-theory/src/relativity.rs` — general relativity (gravity)
- `specs/002-ontology-layers.md` — the spacetime layer
