# Standing orders for agents

You are operating inside **physis**, a typed laboratory. You do not have opinions that bypass knobs, claims, journals, and verification receipts.

## What you may do

1. Read `specs/`, `plans/`, `docs/`, and crate-level rustdoc.
2. Run the CLI (`physis layers|theories|knobs|run|set|experiment|journal|epistemics|why|prove|formalize|reproduce|gaps|falsify|sweep|branch|audit|review|inspect|loop`). `--role explorer` (and the other named roles) can observe but cannot mint. `--budget prove=N,review=N,set=N` is a research cap, not a proof. `physis reproduce` remints a stored receipt in-process and is **not** P4; it requires P3F. `physis gaps` rebuilds the knowledge-gap graph from live verdicts and declared lemma edges. A failing evaluation is not a missing theorem. Overlap without containment on an empirical receipt is insufficient-precision, not compatible. A lattice too coarse to certify a numerical order (`field.second-order-accurate`) is also insufficient-precision, not a failed theorem. coNP-complete search is computationally-intractable, not logically undecidable. An empirical prediction with no registered dataset is missing-dataset; Super-K prose is not a Dataset. The research loop will not raise P3S on an unproved identity; standalone `review` is still encoding-axis.
3. Turn knobs through `Lab::set_knob` / `physis set`. Illegal values must be rejected by domain checks.
4. Add tests that demonstrate a knob → verdict diff.
5. Add a new theory as a `Theory` impl with its own knobs and claims. Do not special-case it in the CLI.
6. Add a new scientific domain as layers + theories, following `specs/007-reuse-domains.md`.
7. Propose candidate theorems, counterexamples, and encodings. You may not mint `Verified<T>`. Call `physis_verifier::verify` with a trusted `Challenge`; you still cannot deserialize a `Verified` from JSON.

## What you must not do

- Do not treat `VerdictKind::Holds` as “true of nature”. Verdicts are internal to the encoding.
- Do not treat `DerivationAssurance::Executed` as a kernel proof. It means the evaluator ran.
- Do not invent a `theorem` tag or a `MachineProved` enum variant. Only `physis-verifier` can mint `Verified<T>`, and that mint is crate-private.
- Do not silently upgrade `Asserted` (conjecture/heuristic/open) to `Executed`, or `Unreviewed` to a stronger semantic tag, without encoding an actual check. `physis review` is allowed only because it *runs* provenance, a second encoding, and the red-team corpus. There is no `SemanticAssurance::Canonical` variant.
- Do not implement Geometric Unity, or claim to. `observer-geometry` is a scaffold.
- Do not declare string theory false because `predictivity.unique-vacuum` fails. That cell *is* the landscape objection, labelled heuristic.
- Do not add `unsafe`. The workspace is `#![forbid(unsafe_code)]`.
- Do not add FFI or non-Rust **physics engines**. Unverified external computation is never authoritative. External formal systems may produce proof *artifacts* only through isolated certificate-checking boundaries (`specs/020-proof-carrying.md`). `physis_verifier::verify` on `LeanSource` runs the Lean kernel and nanoda on a `lean4export`; missing tools or export-only bytes are `LeanPipelineNotWired` (no mint). `ExactCertificate` is a dual-expanded identity, not a kernel proof.
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
| `SemanticAssurance` | unreviewed / source-anchored / independently-encoded / adversarially-reviewed |

A theory can `Executed`-derive a prediction that nature `Excluded`. That is a feature.

`physis why <claim>` prints assumptions, a typed `judgment` (evaluator
`holds` is `logical undetermined` until a dual-checked receipt exists;
`CertifiedNumeric` Holds is `numeric certified` with a display enclosure,
not a kernel proof; a coarse lattice is `numeric unresolved`; `Judgment`
has no `Deserialize`, so JSON cannot mint `logical proved`;
`LogicalJudgment` has no public `Proved` constructor;
`NumericJudgment` has no public `Certified` constructor;
`EmpiricalJudgment` has no public `Compatible` constructor;
`HeuristicJudgment` has no public `Suggestive` constructor;
`StatisticalJudgment` has no public `Computed` constructor;
there is no `SemanticAssurance::Canonical` variant; P3S is a review-store
tag, not an evaluator field; `Verdict` has no `Deserialize`, so JSON
cannot mint a `certified-numeric` overlay; `Claim` derivation / empirical /
semantic fields are private; `Verdict` derivation / empirical / semantic /
enclosure fields are private, so a public assignment cannot mint
`CertifiedNumeric`),
a derived `trust` profile (P3F only from a verifier receipt; P3S from
encoding review of the live `statement_hash`, not the slug; P4 is not assigned from an in-process remint), the
statement hash (which commits to quantifiers, units, constants, boundary
conditions, conventions, theory version, definitions, datasets, and
formal-library identity; the lab slug is stable; P3F is a receipt of
that hash, not of the slug; P3S is a review of that hash, not of the
slug), non-default identity
fields, and `kernel proof: none` until `physis prove` records a
receipt. A P3F result that is still `Unreviewed` prints a danger note.
When Lean 4.34 and `lean4export` are on PATH (`LEAN4EXPORT`), `prove` of a
catalog FormalClaim identity mints `FormalBackend::Lean4` (`lean-kernel` + `nanoda`).
The catalog obligation is that identity, not the slug: a matching id with
different commitments, the encoding-wide domain placeholder, or only
`encoding-is-the-model` cannot borrow ExactIdentity or Physlib. `FormalClaim`
is from_claim-only (private fields, no Deserialize; the hash is recomputed
from the live sentence). `Claim::statement_hash` is derived from the live
sentence (no stored field, no Deserialize). `Challenge`
is generate-only (private fields, no Deserialize). Otherwise it mints `ExactCertificate` (dual expanders). Neither is an enum
an agent can set. `physis review` overlays a justified semantic tag from a
trusted dossier bound to the live FormalClaim; journal restore re-runs
review of that identity rather than deserializing the tag, and only when
the journaled `statement_hash` is the live FormalClaim. A slug-only
review line is not P3S. A review of an
older identity that kept the slug is not P3S. `physis inspect trust|class|origin|gap <value>` inverts those
axes: knobs carry a `ParameterOrigin` so a fitted dilaton is not a derived
prediction, and `inspect origin fitted` lists the knobs that accommodate
rather than derive. `inspect trust P3N` lists exact-`Ratio` cancellations
and the exact hypercharge solve (Standard Model chiral anomalies and
`sm.hypercharge-derivation`) and hydrogen neutrality from `Q = T₃ + Y`
(`empirical.charge-quantization`), plus GUT-scale `sin²θ_W = 3/8`
(`gut.weinberg-angle`). Those P3N cells and the GQW / PDG mixing-angle
siblings name a `DomainOfValidity`; Super-K and GUT `Tr Q` stay
encoding-wide. It does not list GUT `Tr Q` (`ΣY` is already
the gravitational anomaly), Georgi–Quinn–Weinberg running at `M_Z`, the
3% band, or a kernel proof. `inspect trust P2` lists Hodge Laplacian-versus-`b₁` agreement
(`dec.hodge-harmonic`), not Euler–Poincaré rank-cancellation, not Poincaré,
and not a kernel proof. That P2 cell names discrete combinatorial Hodge
on finite simplicial 1-cochains; Euler–Poincaré and Poincaré stay
encoding-wide. `exec` checks role, then trust, then budget:
`reproduce` and the loop's review step require P3F. Standalone encoding
review does not.

## First lab

The current flagship is `string-critique`. Before proposing that “we were fundamentally wrong about string theory,” you must:

1. Produce a knob path that flips an **executed model-internal** claim (not a heuristic) to `fails` for every viable string construction, **or**
2. Produce a unique-geometry construction whose empirical-contact claims are `executed` model-internal or phenomenological, not `conjecture`/`open-problem`.

Until then, report the matrix and the diffs. That *is* the work. A kernel proof of a catalog identity is `physis prove` when the Lean pipeline is wired, not an enum.

## Style

- Small crates, documented public items, `cargo fmt`, clippy `-D warnings`.
- Tests name the claim they protect (`turning_iib_dimension_flips_critical_claim`).
- Prefer enums over strings, except at the agent protocol boundary (`KnobValue`).
- When you add a knob, add it to specs if it is load-bearing.
