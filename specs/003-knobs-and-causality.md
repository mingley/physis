# 003 — Knobs and causality

Status: active
Layer: agent / all

## Definition

A **knob** is a named, domain-bounded parameter of a `Knobbed` object. Agents set knobs by name with a raw token. The lab parses the token against the domain and rejects illegal values.

Kinds: `bool`, `int`, `uint`, `float`, `choice`.

## Causality contract

```
set(knob, value) → (old, new, Δverdicts)
```

`Δverdicts` is the list of claims whose `VerdictKind` changed. Epistemic-only changes (holds+heuristic → holds+theorem) are *not* currently diffed; promoting epistemic status is a code change, not a knob turn.

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
- They are not evidence about nature. They are coordinates on a theory's moduli / parameter space.
- They are not free-form JSON. Unknown names are errors.

## Journal

Every successful `set` appends a `SetKnob` event with from/to and diffs. Journals are JSONL, append-only, optionally a file.

## Agent rule

Do not set ten knobs and then look once. Set one, read the diff, then decide. The causal graph is the science.
