# Standing orders for agents

You are operating inside **physis**, a typed laboratory. You do not have opinions that bypass knobs, claims, journals, and verification receipts.

## What you may do

1. Read `specs/`, `plans/`, `docs/`, and crate-level rustdoc.
2. Run the CLI (`physis layers|theories|knobs|run|set|experiment|journal|epistemics|why`).
3. Turn knobs through `Lab::set_knob` / `physis set`. Illegal values must be rejected by domain checks.
4. Add tests that demonstrate a knob → verdict diff.
5. Add a new theory as a `Theory` impl with its own knobs and claims. Do not special-case it in the CLI.
6. Add a new scientific domain as layers + theories, following `specs/007-reuse-domains.md`.
7. Propose candidate theorems, counterexamples, and encodings. You may not mint `Verified<T>`.

## What you must not do

- Do not treat `VerdictKind::Holds` as “true of nature”. Verdicts are internal to the encoding.
- Do not treat `DerivationAssurance::Executed` as a kernel proof. It means the evaluator ran.
- Do not invent a `theorem` tag or a `MachineProved` enum variant. Only `physis-verifier` can mint `Verified<T>`, and that mint is crate-private.
- Do not silently upgrade `Asserted` (conjecture/heuristic/open) to `Executed`, or `Unreviewed` to a stronger semantic tag, without encoding an actual check.
- Do not implement Geometric Unity, or claim to. `observer-geometry` is a scaffold.
- Do not declare string theory false because `predictivity.unique-vacuum` fails. That cell *is* the landscape objection, labelled heuristic.
- Do not add `unsafe`. The workspace is `#![forbid(unsafe_code)]`.
- Do not add FFI or non-Rust **physics engines**. Unverified external computation is never authoritative. External formal systems may later produce proof *artifacts* only through isolated certificate-checking boundaries (`specs/020-proof-carrying.md`). That pipeline is not wired yet.
- Do not paper over type errors with `f64` bags. If two quantities should not add, they must not share a type.
- Do not invent particles, groups, or critical dimensions. Cite, or mark `OpenProblem`.
- Do not rewrite history in the journal. Append only.
- Do not treat generated prose as scientific authority.

## Protocol

Every interesting action is:

1. observe (`run`, `knobs`, `layers`, `why`)
2. set a knob
3. read the verdict diff
4. record (the lab journals this)

If a change produces **no** verdict diff and you expected one, either the claim is missing or the mapping is a bug. Add a test.

## Assurance axes

Every claim answers four questions. They are different Rust types.

| Axis | Meaning |
|---|---|
| `ClaimClass` | mathematical / model-internal / phenomenological / empirical-prediction / measurement / conjecture / heuristic / open-problem |
| `DerivationAssurance` | asserted / executed / cross-checked / certified-numeric. **No MachineProved variant.** |
| `EmpiricalStatus` | not-applicable / untested / compatible / supported / tension / excluded / inconclusive |
| `SemanticAssurance` | unreviewed / source-anchored / independently-encoded / adversarially-reviewed / canonical |

A theory can `Executed`-derive a prediction that nature `Excluded`. That is a feature.

`physis why <claim>` prints assumptions, the statement hash, and `kernel proof: none` until a receipt exists.

## First lab

The current flagship is `string-critique`. Before proposing that “we were fundamentally wrong about string theory,” you must:

1. Produce a knob path that flips an **executed model-internal** claim (not a heuristic) to `fails` for every viable string construction, **or**
2. Produce a unique-geometry construction whose empirical-contact claims are `executed` model-internal or phenomenological, not `conjecture`/`open-problem`.

Until then, report the matrix and the diffs. That *is* the work. A kernel proof of any of this is Milestone 2, not an enum.

## Style

- Small crates, documented public items, `cargo fmt`, clippy `-D warnings`.
- Tests name the claim they protect (`turning_iib_dimension_flips_critical_claim`).
- Prefer enums over strings, except at the agent protocol boundary (`KnobValue`).
- When you add a knob, add it to specs if it is load-bearing.
