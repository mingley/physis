# Algorithm-independence rationale (C6.2)

Merely moving code into a second crate would not satisfy spec 021. This
record states what is independently implemented here, what is shared,
and what could still fail in common mode.

## What production does

The P3F exact-certificate path parses an infix string (`parse_expr`),
builds a shared `Expr` AST, and runs two expanders over it
(`expand_recursive`, a recursive AST walk, and `expand_postfix`, a
flatten-then-stack-machine). Both expanders distribute products over
sums into the same `Poly` type and check that the result is the zero
polynomial.

## What this checker does instead

1. **Own parser.** The input is a JSON obligation tree, parsed by the
   strict parser in `src/lib.rs` (`parse_obligation`, `parse_tree`).
   It shares no code with the infix parser, the IR parser, the
   catalog expression constructors, or the production `Expr` type.
   Duplicate keys are rejected through the deserializer itself
   (document-order entries with decoded escapes), not by reusing any
   production scanner.
2. **Own algorithm.** No expansion, no polynomial type, no symbolic
   normalization. The checker recomputes the syntactic total degree
   itself and evaluates the tree directly at every point of the
   `{0..=d}^n` integer grid with checked `i128` arithmetic. All-zero
   means agreement; the first nonzero point is the preserved
   counterexample. Exactness rests on the grid theorem (a nonzero
   total-degree-`d` polynomial cannot vanish on all `(d+1)^n` grid
   points), not on sharing production's ring identities.
3. **Own failure semantics.** Overflow, grid explosion, and degree
   overflow map to `resource-exhausted` (unresolved, never disproof);
   malformed input maps to `invalid`. There is no `Holds` value to
   read and no receipt to trust.

## What is shared, deliberately

- The wire format (`WIRE.md`): both sides must agree on what an
  obligation *is*, or nothing is being reproduced.
- Standard serialization (`serde`, `serde_json`) and hashing (`sha2`).
- Integer arithmetic semantics: both implementations compute over
  mathematical integers (production via its polynomial ring, here via
  checked `i128` grid evaluation).

## Declared common-mode risks

1. **Wire-format misreading.** If `WIRE.md` is ambiguous, both sides
   could agree on a wrong reading. Mitigated by strictness (unknown
   keys/ops/versions are `invalid`, never defaulted) and by the host
   binding obligation bytes to the live challenge independently of
   either parser.
2. **Integer semantics.** Both sides assume two's-complement exact
   integer arithmetic. A platform that violates this breaks both.
3. **Grid theorem.** The checker's exactness claim rests on a theorem
   the production path does not use. An error here fails open only if
   the grid is too small for the degree; the degree is recomputed
   from the tree with checked arithmetic, and any overflow refuses
   with `resource-exhausted`.
4. **JSON parsing.** Both sides parse JSON with `serde_json` (different
   code paths: derived structs on the host, the strict walker here).
   A `serde_json` soundness bug could affect both. The duplicate-key
   walk and the host's byte-equality binding narrow this to full
   document forgery, which the host's hash binding still catches.
