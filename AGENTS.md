# Standing orders for agents

You are operating inside **physis**, a typed laboratory. You do not have opinions that bypass knobs, claims, journals, and verification receipts.

## What you may do

1. Read `specs/`, `plans/`, `docs/`, and crate-level rustdoc.
2. Run the CLI (`physis layers|theories|knobs|run|set|experiment|journal|epistemics|why|evidence|prove|formalize|reproduce|gaps|enclose|cite|encode|judge|falsify|hypothesize|sweep|branch|audit|review|inspect|loop`). `--role explorer` (and the other named roles) can observe but cannot mint. `proof-searcher` cannot remint; that is `replication-agent` (still not P4). `explorer` cannot `score`; that is `empirical-analyst`. `proof-searcher` cannot independently parse a `CertifiedNumeric` enclosure; that is `numerical-verifier` (`physis enclose`; not a kernel receipt, not Canonical, not P4; `inspect trust P3N` stays count 4). `--journal` restore rebuilds `NumericCertificate` nodes from live overlay strings; a recorded `certificate_hash` is not deserialized. A reviewer cannot independently rehash a `SourceRecord`; that is `provenance-auditor` (`physis cite`; datasets and catalog dossiers; not P3S, not Canonical, not P4). A reviewer cannot independently round-trip a live theory IR package; that is `encoding-auditor` (`physis encode`; combinational NAND netlist, Klein–Gordon stencil, Wilson U(1)/SU(2)/SU(3) plaquettes, ohm-circuit lumped branches, bell-test singlet ket, newtonian-gravity inverse-square Binet rhs, linear-medium isotropic-linear constitutive law, maxwell-vacuum source-free homogeneous Faraday, ideal-gas Maxwell-Boltzmann statistics, landauer-engine kT ln2 bound, and dirac-fermion naive 1D operator; not P3S, not Canonical, not P4). An explorer cannot independently rebuild a `from_lab` judgment; that is `judge` (`physis judge`; unique-vacuum stays heuristic failed; JSON cannot mint `logical proved`; not Canonical, not P4). `physis hypothesize [theory]` probes chosen/fitted knobs for scientific-axis diffs and restores; measured knobs (generations, observed_dim) are frozen — they are not hypotheses about the encoding. It also applies IR package mutations (`Theory::structural_mutations`): `combinational-circuit` is a NAND netlist, and `add-feedback` and `add-contention` are not knobs; `klein-gordon` is a nearest-neighbour Laplacian with a quadratic potential, and `add-next-nearest` and `add-quartic` are not knobs; `wilson-u1`, `wilson-su2`, and `wilson-su3` are unimproved 1×1 Wilson stencils, and `add-rectangle` is not a knob; `ohm-circuit` is a lumped Kirchhoff netlist, and `add-tline` and `add-flux` are not knobs; `bell-test` is a two-qubit singlet ket, and `add-product` and `add-pr-box` are not knobs; `newtonian-gravity` is an inverse-square Binet rhs, and `add-schwarzschild` and `add-yukawa` are not knobs; `linear-medium` is an isotropic-linear constitutive law, and `add-tellegen` is not a knob; `maxwell-vacuum` is source-free homogeneous Faraday, and `add-monopole` and `add-proca` are not knobs; `ideal-gas` is Maxwell-Boltzmann statistics, and `add-bose` and `add-fermi` are not knobs; `landauer-engine` is a kT ln2 Landauer bound, and `add-kt` and `add-demon` are not knobs; `dirac-fermion` is a naive 1D Dirac operator, and `add-wilson` and `add-next-nearest` are not knobs. Mutants are not installed. `physis evidence <claim>` groups live evaluations by statement hash: a shared slug is not one FormalClaim; confidence is a derived TrustProfile, not a numeric score. It inserts a content-addressed Evidence graph (Statement and Evaluation nodes) and journals `JournalEvent::Evidence`. `--journal` restore rebuilds that DAG from live evaluations; a recorded `graph_hash` is not deserialized as the graph and is not Canonical or P4. `physis replay` still certifies only `set-knob`. `--budget prove=N,review=N,set=N` is a research cap, not a proof. `physis reproduce` remints a stored receipt in-process and is **not** P4; it requires P3F. `physis gaps` rebuilds the knowledge-gap graph from live verdicts and declared lemma edges. A failing evaluation is not a missing theorem. Overlap without containment on an empirical receipt is insufficient-precision, not compatible. A lattice too coarse to certify a numerical order (`field.second-order-accurate`) is also insufficient-precision, not a failed theorem. coNP-complete search is computationally-intractable, not logically undecidable. An empirical prediction with no registered dataset is missing-dataset. Super-K `p→e+π0` is a Dataset (Takenaka et al., Phys. Rev. D 102, 112011); `gut.proton-lifetime-sk` is the dim-6 `M_GUT^4` comparison (excluded for minimal SU(5), compatible for MSSM dim-6), not P3N and not a dimension-5 operator. PDG `sin²θ_W(M_Z)` is a Gaussian; `gut.weinberg-angle-mz-interval` is `statistical computed` from an exact NLL, still not P3N. The research loop will not raise P3S on an unproved identity; standalone `review` is still encoding-axis.
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
- Do not declare string theory false because `predictivity.unique-vacuum` fails. That cell *is* the landscape objection, labelled heuristic. The four encodings name distinct regimes (flux/moduli landscape, observer-geometry program axiom, classical Einstein–Hilbert plus Λ, SM Higgs vacuum); they are not one FormalClaim and not Canonical.
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
3. read the verdict diff (kind **and** derivation / empirical / judgment when those axes move)
4. record (the lab journals this)

If a change produces **no** scientific-axis diff and you expected one, either the claim is missing or the mapping is a bug. Add a test. `set klein-gordon spacing 100` must show `holds → undecidable` **and** `not-applicable → inconclusive` / `logical undetermined → numeric unresolved`, not a failed theorem.

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
`from_lab` projects `statistical computed` only from an exact Gaussian
NLL overlay on an empirical or measurement claim (PDG `sin²θ_W(M_Z)`);
Super-K interval-subset stays empirical;
there is no `SemanticAssurance::Canonical` variant; P3S is a review-store
tag, not an evaluator field; `Verdict` has no `Deserialize`, so JSON
cannot mint a `certified-numeric` overlay; `Claim` derivation / empirical /
semantic fields are private; `Verdict` derivation / empirical / semantic /
enclosure / NLL fields are private, so a public assignment cannot mint
`CertifiedNumeric` or a Gaussian NLL),
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
sentence (`Claim.statement`, id, class, layer, assumptions, domain, and
commitments are private; lemma edges stay public; no stored hash
field, no Deserialize). `Challenge`
is generate-only (private fields, no Deserialize). Otherwise it mints `ExactCertificate` (dual expanders). Neither is an enum
an agent can set. `physis review` overlays a justified semantic tag from a
trusted dossier bound to the live FormalClaim; journal restore re-runs
review of that identity rather than deserializing the tag, and only when
the journaled `statement_hash` is the live FormalClaim. A slug-only
review line is not P3S. A review of an
older identity that kept the slug is not P3S. `physis inspect trust|class|origin|gap|judgment <value>` inverts those
axes: knobs carry a `ParameterOrigin` so a fitted dilaton is not a derived
prediction, and `inspect origin fitted` lists the knobs that accommodate
rather than derive. `inspect judgment statistical-computed` lists the PDG
GQW NLL cell; `inspect judgment empirical-excluded` lists Super-K (not that
Gaussian); `inspect judgment logical-proved` is empty until a dual-checked
receipt exists. `inspect trust P3N` lists exact-`Ratio` cancellations
and the exact hypercharge solve (Standard Model chiral anomalies and
`sm.hypercharge-derivation`) and hydrogen neutrality from `Q = T₃ + Y`
(`empirical.charge-quantization`), plus GUT-scale `sin²θ_W = 3/8`
(`gut.weinberg-angle`). Those P3N cells and the GQW / PDG mixing-angle
siblings name a `DomainOfValidity`. Super-K `p→e+π0` names the dim-6 /
90% CL regime; GUT `Tr Q` stays encoding-wide. Super-K is not P3N. `thermo.high-t-classical` names `T/Θ ≥ 8`; `thermo.debye-t3` names the `Θ/20` probe; `thermo.rj-ir-limit` names `hν = 0.01 kT`; `gauge.exact-area-law-2d` names 2D. Dulong–Petit at the current T stays encoding-wide. It does not list GUT `Tr Q` (`ΣY` is already
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
