# M3 — Domain reuse (electricity, then computation)

Goal: prove the workspace is not a string-theory toy by hosting a second scientific domain without forking core.

## Electricity (first reuse)

New theory `maxwell-vacuum` (and later `linear-medium`):

- Knobs: none in vacuum except unit system; in a medium, `epsilon_r`, `mu_r`
- Claims:
  - `1/√(ε₀μ₀) = c` as a **theorem** of the encoding
  - Faraday / Ampere structure as encoded facts, then as typed exterior calculus if we get that far
- Control: `ohm-circuit` as an effective theory of Maxwell

This domain is chosen because it *needs* `Qty` immediately (cannot add volts to amperes) and produces clean knob diffs (`epsilon_r` changes wave speed).

## Computation (second reuse)

New theory `turing-finite` or `combinational-circuit`:

- Knobs: resource bounds
- Claims: invariants, halt (undecidable — good, we already have that verdict kind)
- Later: Landauer, sitting on `statistical` + `information`

## Crate split

If model types get heavy, add `physis-em` / `physis-info` crates. Do not copy `physis-core`.

## Done when

- `physis experiment em-vacuum` prints a matrix
- At least one electricity knob → verdict diff test
- Spec `specs/008-electromagnetism.md` exists
- `specs/007-reuse-domains.md` is still accurate
