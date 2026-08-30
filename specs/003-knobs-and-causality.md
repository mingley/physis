# 003 — Knobs and causality

Status: active
Layer: agent / all

## Definition

A **knob** is a named, domain-bounded parameter of a `Knobbed` object. Agents set knobs by name with a raw token. The lab parses the token against the domain and rejects illegal values.

Kinds: `bool`, `int`, `uint`, `float`, `choice`.

Every spec also carries a [`ParameterOrigin`]: `measured` (taken from
nature), `fitted` (adjusted to data), `chosen` (encoder/agent),
`derived`, `fundamental-input`, or `nuisance`. A fitted dilaton is not
a derived prediction. `physis inspect origin fitted` lists the knobs
that accommodate rather than derive.

## Causality contract

```
set(knob, value) → (old, new, Δverdicts)
```

`Δverdicts` is the list of claims whose *scientific axes* changed: evaluator
`VerdictKind`, `DerivationAssurance`, `EmpiricalStatus`, or the projected
`Judgment` label (`Judgment::from_lab` with `dual_checked = false` — a knob
turn does not mint a kernel proof). Each row also carries the claim's
`statement_hash`. `physis set` prints kind always and the other axes only
when they moved.

A coarse lattice (`set klein-gordon spacing 100`) is the load-bearing
example: `field.second-order-accurate` is `holds → undecidable`, empirical
`not-applicable → inconclusive`, judgment `logical undetermined → numeric
unresolved`. That is not a failed theorem.

Pre-axis journals stored only `{claim, from, to}`. Replay still certifies
those records against a live recompute that now emits the extra fields.
A journal that *does* carry axes is not faithful if those strings were
tampered. Promoting an encoding in source (Asserted → Executed by rewriting
`evaluate`) is still a code change, not a knob turn.

If a load-bearing knob produces an empty diff, that is a failing test.

## Examples in v0

| Theory | Knob | Effect |
|---|---|---|
| `type-iib` | `total_dim=9` | `consistency.critical-dimension` holds → fails |
| `type-iib` | `supersymmetry=false` | `consistency.susy-construction` and `no-tachyon` fail |
| `type-iib` | `flux_bits=0` | `predictivity.unique-vacuum` may hold (heuristic) |
| `type-iib` | `h11=0` and `h21=0` | `predictivity.unique-vacuum` fails → holds (no moduli to scan) |
| `type-iib` | `dilaton` large (with large `compact_radius_planck`) | `empirical.hidden-extra-dims` holds → fails (g_s inflates the effective size) |
| `standard-model` | `generations=2` | `empirical.three-generations` holds → fails |
| `observer-geometry` | `unique_vacuum=false` | `predictivity.unique-vacuum` holds → fails |
| `observer-geometry` | `derive_gauge=false` | `empirical.sm-gauge` fails (SM postulated, not derived) |

## What knobs are not

- They are not the laws. Laws are the evaluate functions.
- They are not evidence about nature. They are coordinates on a theory's moduli / parameter space. `ParameterOrigin` records whether a coordinate was measured, fitted, or chosen.
- They are not free-form JSON. Unknown names are errors.

## Journal

Every successful `set` appends a `SetKnob` event with from/to and diffs. Journals are JSONL, append-only, optionally a file.

## Agent rule

Do not set ten knobs and then look once. Set one, read the diff, then decide. The causal graph is the science.
