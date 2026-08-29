# Standing orders for agents

You are operating inside **physis**, a typed laboratory. You do not have opinions that bypass knobs, claims, and journals.

## What you may do

1. Read `specs/`, `plans/`, `docs/`, and crate-level rustdoc.
2. Run the CLI (`physis layers|theories|knobs|run|set|experiment|journal`).
3. Turn knobs through `Lab::set_knob` / `physis set`. Illegal values must be rejected by domain checks.
4. Add tests that demonstrate a knob → verdict diff.
5. Promote a heuristic to a theorem *only* by encoding an actual derivation, and retagging `Epistemic`.
6. Add a new theory as a `Theory` impl with its own knobs and claims. Do not special-case it in the CLI.
7. Add a new scientific domain (electricity, computation, …) as layers + theories, following `specs/007-reuse-domains.md`.

## What you must not do

- Do not treat `VerdictKind::Holds` as “true of nature”. Verdicts are internal to the encoding.
- Do not silently upgrade `Heuristic` or `Conjecture` to `Theorem`.
- Do not implement Geometric Unity, or claim to. `observer-geometry` is a scaffold.
- Do not declare string theory false because `predictivity.unique-vacuum` fails. That cell *is* the landscape objection, labelled heuristic.
- Do not add `unsafe`. The workspace is `#![forbid(unsafe_code)]`.
- Do not add FFI or non-Rust physics engines. Pure Rust is a constraint, not a preference.
- Do not paper over type errors with `f64` bags. If two quantities should not add, they must not share a type.
- Do not invent particles, groups, or critical dimensions. Cite, or mark `Open`.
- Do not rewrite history in the journal. Append only.

## Protocol

Every interesting action is:

1. observe (`run`, `knobs`, `layers`)
2. set a knob
3. read the verdict diff
4. record (the lab journals this)

If a change produces **no** verdict diff and you expected one, either the claim is missing or the mapping is a bug. Add a test.

## Epistemic tags

| Tag | Meaning |
|---|---|
| `theorem` | Proven in this model, or a standard theorem encoded as such |
| `encoded-fact` | Textbook result stored as data (e.g. SM ⊂ E₈×E₈) |
| `conjecture` | The theory claims it; we have not derived it |
| `heuristic` | Order-of-magnitude / folklore (landscape counts) |
| `open` | The encoding cannot decide |

## First lab

The current flagship is `string-critique`. Before proposing that “we were fundamentally wrong about string theory,” you must:

1. Produce a knob path that flips a **theorem** (not a heuristic) to `fails` for every viable string construction, **or**
2. Produce a unique-geometry construction whose empirical-contact claims are `theorem` or `encoded-fact`, not `conjecture`/`open`.

Until then, report the matrix and the diffs. That *is* the work.

## Style

- Small crates, documented public items, `cargo fmt`, clippy `-D warnings`.
- Tests name the claim they protect (`turning_iib_dimension_flips_critical_claim`).
- Prefer enums over strings, except at the agent protocol boundary (`KnobValue`).
- When you add a knob, add it to specs if it is load-bearing.
