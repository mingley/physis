# Wire format: integer obligations and verdicts (v1)

This document specifies field ordering, integer/text encoding, and
normalization. It exists before any fixture.

## Obligation file

A UTF-8 JSON object with exactly these keys (hosts emit them in
lexicographic order with minimal separators, no trailing newline; the
checker parses semantically and hashes the bytes it read):

| Key | Type | Meaning |
|---|---|---|
| `schema` | string | Must be `physis-int-obligation`. |
| `version` | integer | Must be `1`. |
| `run_id` | non-empty string | Opaque run binding, echoed in the verdict. |
| `symbols` | sorted unique non-empty list of names | Declared indeterminates. |
| `expect` | string | Must be `zero`. Anything else is `unsupported`. |
| `tree` | node | The integer expression claimed to be zero. |

Names match `[A-Za-z_][A-Za-z0-9_]*`. `symbols` must be sorted and
duplicate-free, or the obligation is `invalid`.

Nodes are exactly one of:

- `{"const": INT}` — a JSON integer literal (no fraction, no exponent)
  within i64/u64 range, widened to i128 for evaluation. Floats,
  exponents, and out-of-range literals are `invalid` (literals beyond
  u64 arrive as floats and cannot be trusted exactly).
- `{"var": NAME}` — a declared symbol. Undeclared names are `invalid`.
- `{"op": OP, "args": [...]}` — `add`/`sub`/`mul` take exactly 2 args,
  `neg` exactly 1, `pow` exactly 1 arg plus `"exp": N` with `N` a
  non-negative integer literal. Anything else is `invalid`.

Limits (all `invalid` when exceeded, except grid evaluation which is
`resource-exhausted`): file `<= 1_000_000` bytes, depth `<= 32` tree
levels along any path, `<= 100_000` total nodes, `<= 16` symbols.

Duplicate object keys anywhere are `invalid`. Unknown top-level or node
keys are `invalid`. Missing required keys are `invalid`. A JSON syntax
error is `invalid`. These are schema violations in untrusted bytes, not
checker bugs.

## Verdict file

Written by the checker on every parsed run, including `disagree`:

| Key | Type | Meaning |
|---|---|---|
| `schema` | string | `physis-int-verdict`. |
| `version` | integer | `1`. |
| `run_id` | string | Echo of the obligation's `run_id` (empty when the id did not parse). |
| `obligation_sha256` | hex | SHA-256 of the obligation file bytes as read. |
| `exe_sha256` | hex-or-empty | Echo of `WORKER_EXE_DIGEST`. |
| `checker_id` | string | `dec-int-checker`. |
| `checker_version` | integer | `1`. |
| `disposition` | string | `agree`, `disagree`, `unsupported`, `invalid`, or `resource-exhausted`. |
| `detail` | string | Human diagnostic (witness for `disagree`). |
| `evaluated_points` | integer | Grid points evaluated (`0` unless learnt). |
| `grid_degree` | integer-or-null | Syntactic degree used, when computed. |
| `witness` | object-or-null | First counterexample `{symbol: value}`, when found. |

Exit code `0` means a verdict file was written (any disposition).
Nonzero means no verdict exists (bad argv, unreadable input, unwritable
output): the host maps that to `execution-failed` and trusts nothing.

## Evaluation semantics

Integers are mathematical integers computed in checked i128. Overflow
is `resource-exhausted`, never wrap, never `disagree`. The checker
evaluates the tree at every point of `{0..=d}^n` where `d` is the
syntactic total degree it recomputes itself and `n` is the symbol
count. More than `1_000_000` points is `resource-exhausted` before any
evaluation. All-zero means `agree`; the first nonzero point is the
`disagree` witness. Agreement is exact: no floating point anywhere.
