# 005 — String critique experiment

Status: active
Layer: experiment
Id: `string-critique`

## Question

Which structural claims of string constructions, the Standard Model, GR, and a unique-geometry scaffold hold under default knobs — and which flip when knobs move?

In particular:

1. Are there **theorems** of string constructions (critical dimension, tachyons of the bosonic string) we can encode faithfully?
2. Is the **landscape / uniqueness** objection mechanical in this encoding, or only rhetorical?
3. Does a unique-geometry program **earn** empirical contact, or only assert it?

## What this is not

This experiment does **not** prove that “we were fundamentally wrong about string theory.” It builds the machine on which that question can be *worked*, over long time horizons, without losing the difference between a theorem and a vibe.

Eric Weinstein’s public critique (string theory as a wrong turn; uniqueness and geometry as a better starting point; Geometric Unity as his own program) is the *motivation*. Geometric Unity is **not implemented**. `observer-geometry` is a contrast-class scaffold.

## Objects

| id | object |
|---|---|
| `standard-model` | SM QFT control |
| `general-relativity` | classical gravity control |
| `type-iib` | Type IIB superstring (chiral N=2, no perturbative GUT group) |
| `type-iia` | Type IIA superstring (non-chiral N=2, no perturbative GUT group) |
| `type-i` | Type I superstring (open + closed, SO(32)) |
| `heterotic-e8e8` | Heterotic E₈×E₈ (SM embedding is an encoded fact) |
| `heterotic-so32` | Heterotic SO(32) (SM embedding is an encoded fact) |
| `bosonic` | 26D bosonic string (tachyon, no fermions) |
| `m-theory` | 11D M-theory (membrane; critical dimension 11) |
| `observer-geometry` | unique-geometry scaffold (total D = observed 4 + fibre 10; the fibre 10 is the minimal carrier of Spin(10), so 14 is a toy constraint, not a magic number) |

All five superstring/M constructions plus the bosonic string are in the default
lab and matrix. Type II and M have no perturbative 10D/11D GUT group, so their
`empirical.sm-gauge` is `undecidable`; the SO(32)/E₈×E₈ constructions carry an
encoded SM embedding, so theirs `holds` (as `encoded-fact`).

## Distinctive string facts encoded as theorems / encoded facts

- Bosonic critical dimension 26
- Superstring critical dimension 10
- M-theory dimension 11
- Bosonic tachyon
- Superstring tachyon absence when SUSY is on
- Closed strings include a graviton
- Heterotic 10D gauge groups SO(32) and E₈×E₈, with SM embeddings as encoded facts
- Type II 10D theories have no GUT group; SM from compactification/branes is `undecidable` here
- Green–Schwarz anomaly cancellation selects exactly SO(32) and E₈×E₈ in 10D N=1 (both dimension 496). `consistency.anomaly-cancellation` encodes this via `GaugeGroup::gs_anomaly_free_10d` — an `encoded-fact`, not a re-derivation of the anomaly polynomial. It is knob-sensitive: off the critical dimension the claim is `undecidable`. The Standard Model row also `holds` (anomalies cancel per generation); the bosonic string is `inapplicable` (non-chiral).

## Distinctive critique facts encoded as heuristics / conjectures

- Landscape cardinality grows with extra dimensions and flux bits
- Uniqueness therefore fails for default superstring knobs
- Observer-geometry uniqueness *holds* as a conjectural axiom of the program
- Observer-geometry SM gauge “derivation” is a Spin(10) assignment, `conjecture`

## Pass criteria for a future agent that wants to claim “strings are the wrong turn”

Not v0. A serious claim would need at least one of:

- A **theorem-level** obstruction that applies to all viable constructions (not just the bosonic tachyon, not just a heuristic landscape)
- A competing construction whose empirical rows are `theorem` or `encoded-fact` and whose uniqueness is not an axiom

Until then, the output is the matrix plus knob diffs. That is already more than a podcast.

## CLI

```
physis experiment string-critique
physis set type-iib total_dim 9
physis set type-iib flux_bits 0
physis set observer-geometry derive_gauge false
```
