# 006 — Civilization laboratory

Status: active (plan). Subsequent agents execute this document.
Audience: agents
Does not weaken `specs/020-proof-carrying.md`. Calendar estimates are not used.

Task pickup, dependencies and acceptance criteria now live in
[TODO.md](../TODO.md). This plan retains the scientific scope and `C*` IDs;
the TODO queue refines execution order and adds the gated campaign work.
Architecture, entry docs and the C6.1 contract are delivered; implementation
tasks remain open. Writing a contract does not implement its behavior.

## Objective

Make `physis` the laboratory a civilization would actually use for
agentic physics research: a place where an agent can propose, branch,
prove, falsify, compare, and empirically test models of reality, and
where **nothing becomes trusted scientific knowledge because an agent
wrote code that returns Holds**.

Authority comes only from explicit, independently checkable artifacts:
immutable assumptions, source records, hashed datasets, deterministic
computations, dual-checked certificates, and (later) independent
reproduction. Agents may be creative above that layer and receive no
authority below it.

This plan is the post-kernel payload. L3-M1 through L3-M10 in
`plans/000-roadmap.md` are the trust machine. They are not the science
a civilization needs. Do not spend the next increments leftover-flipping
CODATA siblings or hosting another integer polynomial on special
relativity.

## Why this is the job

A notebook plus an LLM is not a laboratory. It cannot tell a proved
identity from a vibe, a fitted parameter from a derivation, or a PDG
reprint from a prediction. `physis` already can. Civilization-useful
means that advantage is attached to **enough real physics** that a
careful researcher would choose this lab over slides, and that an
agent swarm cannot launder a conjecture into a theorem.

The flagship public argument (string landscape vs unique geometry vs
SM/GR) stays. It is not the whole job. The same substrate must host
kinematics, fields, gravity, thermodynamics, gauge theory, and
empirical contact with hashed measurements — with honest open problems
left open.

## What is already world-class (do not redo)

Treat these as load-bearing. Regression is a bug.

- Unforgeable mint: only `physis-verifier` constructs `Verified<T>`.
  There is no `DerivationAssurance::MachineProved` and no
  `Epistemic::Theorem`. JSON cannot mint `FormalClaim`, `Challenge`,
  `Judgment`, `Verdict`, or `Verified`.
- Orthogonal axes on every claim: `ClaimClass`, `DerivationAssurance`,
  `EmpiricalStatus`, `SemanticAssurance`. Typed `Judgment` variants.
  A MachineProved-shaped result that is `Unreviewed` prints as dangerous.
- Content-addressed `FormalClaim` identity. Catalog lookup is by
  statement hash, not slug.
- Dual checking: recursive vs postfix expanders, Lean kernel plus
  nanoda on `lean4export`. Unauthorized `axiom` / `sorry` / `admit`
  cannot promote.
- Roles and budgets: proposers cannot promote their own results.
  Explorer cannot mint. Proof-searcher cannot remint.
- Journals append and remint; they do not deserialize authority.
- Eight catalog identities, of which seven are distinct algebraic
  ideas (mass-shell is the interval polynomial on `(E, p)`). Vertical
  slice A–E exists (`specs/020-proof-carrying.md`).
- Versioned SI/CODATA ledger (`physis constant`) with lockstep Qty
  centres. P3N count stays 4 until a new *exact cancellation* exists.
- String-critique matrix, typed SI quantities, knob → scientific-axis
  diffs, `unsafe`-free Rust, isolated Lean boundary.

## What is not yet civilization-useful (honest)

Evidence is the live tree, not intent.

- Live evaluators still use `physis_model` `f64` `Qty`, except
  special-relativity mass-shell which reads versioned `c` and `m_e`
  (still Executed, not P3N). Threshold Holds/Fails on kinematics,
  gravity integrals, blackbody, solids, and lattices are not
  certificate-first.
- `ħ` is not stored (π). Thomson, Φ₀, G₀, Y₀, quantum of circulation,
  and gyromagnetic reconstructions stay blocked for that reason — do
  not leftover-flip a sibling instead.
- Physlib is a handful of `grind`/`omega` identities. That is not
  Mathlib-scale physics (Einstein tensor, Maxwell as a field theory,
  SM representation theory).
- `physis-ir` is a line-oriented parser. Equations are strings.
- No live G3 mesh. Tet catalog identity is a local 3-simplex, not a
  3-complex.
- Empirical corpus is three PDG Gaussians plus Super-K. Eddington,
  Mercury, blackbody, hydrogen lines, and CODATA-G tests are not
  `EmpiricalReceipt`s.
- P4 does not exist. In-process `reproduce` remints and **refuses**
  P4. A distinct implementation is still required.
- There is no `SemanticAssurance::Canonical` variant. That is correct.
  Community-canonical encodings are a process that has not been built.
- Most claims are encoding-wide with only `encoding-is-the-model`.
- `physis loop` replays the catalog and standing ops. It does not
  propose new claims from the gap graph.
- GR live package is `action einstein-hilbert` without a catalog tree.
  Do not add a cosmetic `lean_ref`.
- Continuum is 1D lattices and encoded gauge facts, not 3+1 dynamics.
- Architecture and research entry docs now describe the crate and trust
  boundaries; persistent campaigns and independent reproduction remain future work.

## End state (testable)

A skeptical engineer would accept the lab as civilization-useful when
**all** of the following are true in the live tree. Do not mark this
plan complete on a subset.

1. **Authority closure.** Every Holds that a CLI user could mistake for
   a theorem is either (a) a dual-checked receipt on a named
   `FormalClaim`, (b) a `CertifiedNumeric` enclosure, (c) an
   `EmpiricalReceipt` against a hashed dataset, or (d) explicitly
   `Asserted` / heuristic / open. Naked `f64` is not authoritative for
   a threshold claim.
2. **Constants are the evaluators' inputs.** Live theories that mention
   `c`, `h`, `k`, `G`, `α`, `m_e`, `m_p` read `physis-constants` (Ratio
   / SciExact / Interval). `physis_model` floats remain lockstep
   centres, not a second source of truth.
3. **Physics in Physlib, not leftover algebra.** At least one identity
   per live domain that already has a computation (SR, DEC, EM, gravity)
   is kernel-checked **and** is the obligation the live claim is about.
   GR may gain a catalog tree only when the live equations include that
   tree. SM anomalies stay Ratio / P3N, not Lean (`specs/020` C2).
4. **Empirical contact a civilization cares about.** Hashed datasets
   and `EmpiricalReceipt`s for: PDG mixing/couplings (done), Super-K
   (done), solar-system light deflection, Mercury perihelion, and at
   least one spectroscopic or thermal measurement already computed in
   the lab. Compatible remains subset; overlap without containment is
   `InsufficientPrecision`.
5. **Independent reproduction.** A second implementation, not a remint
   of the same crate, can replay a stored receipt and is the only path
   that may assign P4. Until it exists, CLI and loop must keep saying
   remint is not P4.
6. **IR is a scientific language.** Packages carry typed trees for
   catalog identities and dimensioned tokens for the rest. Round-trip
   still fails closed without a tree when `lean_ref` names a catalog
   type.
7. **Research loop is a scientist.** `physis loop` (or a successor)
   selects work from `physis gaps` (missing theorem, missing dataset,
   insufficient precision, intractable) and from constrained IR
   mutation. Catalog replay alone is not autonomous research.
8. **Honest open problems stay honest.** Unique-vacuum, 4D Yang–Mills
   mass gap, P vs NP, and observer-geometry uniqueness remain
   `Asserted` / conjecture / open. Prove and review still refuse them.
9. **A human can enter.** README, `docs/ARCHITECTURE.md`, and
   `AGENTS.md` describe the live crate graph and the trust contract
   without implying that Holds is a kernel proof.
10. **Gates stay green.** `cargo fmt --check`, `cargo clippy --workspace
    --all-targets -- -D warnings` (one `--` before `-D`), `cargo test
    --workspace`, CLI honesty (no `theorem` tag, encode of live SR
    lists catalog trees and does not print `receipt`).

P3N count stays 4 unless the slice is a new exact cancellation (not a
reprint, not GQW, not Super-K). Unique-vacuum graph id does not change
unless unique-vacuum encodings change, which this plan does not ask
for.

## Governing invariants (do not weaken)

Copy these into every slice. If a slice needs to break one, the slice
is wrong.

- Runtime and orchestration stay Rust-first and `#![forbid(unsafe_code)]`.
- Unverified external computation is never authoritative. Lean/Physlib
  produce proof artifacts only through isolated certificate-checking
  (Lean kernel + nanoda).
- No public constructor manufactures a kernel proof. No Deserialize of
  `Verified`, `TrustProfile`, `Challenge`, `Judgment`, `FormalClaim`,
  `Claim`, or `Verdict`.
- Do not restore `Epistemic::Theorem`, `DerivationAssurance::MachineProved`,
  `SemanticAssurance::Canonical`, or P4-from-remint.
- FormalClaim identity commits to statement, quantifiers, units,
  constants, assumptions, domain, conventions, theory version,
  definitions, datasets, and formal-library versions. Changing
  forall/exists, sign, units, or boundary conditions is a new statement
  hash; the human-facing slug may remain unchanged.
- Hidden assumptions are bugs. Extrapolation outside `DomainOfValidity`
  is a new claim or a warning.
- Proposers cannot promote their own results.
- `source: textbook` is not a locator.
- Do not implement Geometric Unity. Do not declare string theory false
  because unique-vacuum fails.

## Forbidden increments (read before coding)

These look like progress and are not. An agent that lands one of these
as the primary diff of a turn has failed this plan.

- Leftover-flip of another CODATA / BIPM / PDG table row whose only
  novelty is a sibling of an already-stored listing (`are`, minute,
  hour, litre, tonne, bar, `J_Hz`, `Hz_kg`, `eV_kg`, `kg_eV`, `f_0`,
  d220, `c1`/`σ`/Wien, Thomson, quantum of circulation, Φ₀/G₀, Y₀,
  G/ħc, PDG `m_τ` reprints, second names).
- A ninth catalog polynomial that is a rename or component of an
  existing one: mass-shell-style relabeling, Jacobi y/z, quaternion
  norm of Lagrange, Cayley–Hamilton one matrix entry, 4-simplex leftover
  of tet, 2D leftover of Lagrange.
- A fifth P3N cell that is not a new exact cancellation.
- Cosmetic `lean_ref` on GR/SM/Planck whose live equations are only
  tokens (`action einstein-hilbert`, …) without the catalog tree.
- Wrapping `AGENTS.md` standing-orders item 2 onto two lines.
- Editing `specs/020-proof-carrying.md` at all in this plan's C-slices.
  A Related bullet grows the file past 730 lines. Inserting P4 or ħ
  text shifts Hartree off compact-table line 183. Leave 020 at 730
  lines. C6.1 writes `specs/021-independent-reproduction.md` and
  points from `plans/000-roadmap.md`.
- Restoring forbidden enums. Empty role aliases. DomainOfValidity
  naming patches as the primary work.
- High-throughput agent machinery (queues, multi-agent chat, prompt
  graphs) before slices C1–C3 and C7.1 exist.
- FFI physics engines.

## Workstreams

Use the dependency order in [TODO.md](../TODO.md); the first-turn list below
identifies the initial vertical slices. One scientific idea per commit.
Use `area: imperative summary` and a body explaining the change.
Follow the applicable branch workflow and publish only when authorized;
do not add attribution trailers.

### C1 — Certificate-first evaluators

**Why.** A civilization cannot trust a 1e-9 `f64` tolerance as a
threshold claim. The ledger already exists; theories ignore it.

**C1.1 Live constants, one theory.** Pick `special-relativity` mass
shell (already typed `Qty<Energy>`). Read `c` and `m_e` from
`physis-constants` (Interval centre + hull documented in evidence).
Keep derivation `Executed`. Galilean still Fails. Lockstep tests must
still pass. Do not leftover-flip a new constant.

**C1.2 Interval sample, not naked tolerance.** The SR interval
evaluator reports the domain and justified error bounds, not only a raw
`1e-9` relative `f64` comparison. Overlap of two intervals does not prove
equality. Catalog integer identity remains the kernel obligation.
Parsing an interval does not certify how it was calculated; keep diagnostic
enclosures separate from `CertifiedNumeric` unless the full bound is checked.

**C1.3 Same pattern on gravity.** Eddington / Mercury RK4 results
report an enclosure. Holds/Fails against the *computed* Schwarzschild
vs Soldner factor stays model-internal until C3 attaches datasets.

**C1.4 ħ as a stored listing.** Represent the sourced reduced Planck
constant with an explicit rounding bound. SI-exact `h/(2*pi)` is not a
finite decimal, and its approximation is not a measured uncertainty.
Do not call that decimal a FormalClaim of equality to `h/(2*pi)`.
This unblocks later electromagnetic quantum constants; do not also
leftover-flip Thomson/Φ₀ in the same commit.

**Done when:** `physis why sr.energy-momentum-invariant` evidence cites
the versioned `m_e` / `c` hashes; interval evidence cites an enclosure;
`inspect trust P3N` is still count 4.

### C2 — Physlib proves physics the lab already computes

**Why.** Eight integer polynomials do not make a physics library.
The next identity must be an idea a live theory already evaluates,
with a named domain, and must not be a leftover of tet/Jacobi/Lagrange
/det-product/interval.

**C2.1 Bind receipts to live kinematic claims (no new polynomial).**
`sr.invariant-interval`, composition, and mass-shell already catalog.
Confirm CLI `why` after `prove` in one journal shows `logical proved`
and P3F for those three, and that `absolute_time` still flips the
evaluator without forging a receipt. Add a lab test if missing. This
is wiring, not a new tree.

**C2.2 Discrete Faraday that is not tet leftover.** Only if the
identity is a **different grade or different complex** than
`dec.d-squared-zero` / `dec.d-squared-one` (for example: `dF = 0` on
the live 2-complex faces the Maxwell theory already talks about).
Host on `maxwell-vacuum` or `de-rham`, not SR. `lean_ref` only with
the tree. Axioms explicit. One-byte mutation invalidates.

**C2.3 Stop.** Do not add another SR-hosted degree-4 polynomial in
the same era as C2.2. Mathlib-scale GR/SM is a later era (C2.4+)
and needs a real encoding, not `grind` on eight matrix entries.

**C2.4 (later)** Contracted Bianchi / Einstein `∇·G = 0` in an
encoding the GR package actually carries. Until the live equations
include that tree, GR keeps token `action einstein-hilbert`.

**Done when:** a non-SR, non-leftover catalog identity is dual-checked
and bound on its host theory, **or** C2.1 tests prove kinematic
receipts are the live SR claims' kernel path. C2.2 may be skipped
only with a written reason in CHANGELOG that Faraday-on-2-complex
*is* tet leftover.

### C3 — Empirical corpus

**Why.** Super-K plus three PDG Gaussians is not contact with nature
at civilization scale. The lab already *computes* solar-system numbers.

**C3.1 Light deflection dataset.** Hash a real locator (Dyson/Eddington
1919 and/or a modern VLBI compilation) as `Dataset`. Compare the
computed 1.75″ Schwarzschild prediction as `EmpiricalReceipt`.
Evaluate Newton/Soldner under the same predeclared comparison rule; preserve
compatible or inconclusive outcomes if the selected measurements cannot
exclude it. Do not tune uncertainty to force a winner. Not P3N. Not dim-5.

**C3.2 Mercury perihelion dataset.** Same pattern for 43″/century.
`add-schwarzschild` remains IR, not a knob.

**C3.3 One thermal or spectroscopic receipt.** Planck peak / Stefan
envelope / hydrogen line using stored `Rinf`/`hcRinf` hulls against a
hashed measurement. Not a FormalClaim that `Eh = 2 hcR∞`.

**Done when:** `physis evidence` on those claims shows hashed datasets;
`inspect judgment empirical-excluded` still lists Super-K; new cells
are empirical compatible/excluded/inconclusive, not logical proved.

### C4 — Continuum and gravity as dynamics

**Why.** Fields-as-flags is already rejected. 1D KG and encoded 4D
confinement are not yet a continuum laboratory.

**C4.1** Transfer-matrix or exact enumeration behind 2D Wilson area
law (already named `gauge.exact-area-law-2d`) so the claim is a
computation, not only an encoding.

**C4.2** Live 2-complex Maxwell that shares DEC coboundary (triangle
already proved). Do not install a G3 mesh by leftover 4-simplex.

**C4.3** GR: one computed curvature identity on a named domain
(Schwarzschild `g_tt g_rr = -1` as a checked encoding, or Birkhoff
as open). Token EH action stays until a tree exists.

**Done when:** at least one gauge or EM claim flips by a computed
spectrum/measure, not a tabulated phase diagram, and docs say so.

### C5 — Typed IR (two-tier)

**Why.** `equations: Vec<String>` cannot be the scientific language.

**C5.1** Packages may attach parsed `Expr` for catalog trees while
keeping token strings. `certify_round_trip` still independent.
Encode lists bound identities by claim id, never `receipt`.

**C5.2** Inverse query: `inspect gap missing-dataset` already lists
that hole. Do not reimplement it. C7.1 is the loop hook that *acts*
on those nodes. If inspect is missing a gap kind this plan names,
add the kind; otherwise skip this slice.

**Done when:** live SR encode still pins a content-addressed package
id; missing tree still fails closed without the word `receipt`.

### C6 — P4 independent reproduction

**Why.** Remint is the same binary checking itself.

**C6.1 (documentation complete)** [Independent reproduction](../specs/021-independent-reproduction.md).
The contract is linked from `plans/000-roadmap.md` and this file. Do **not**
edit `specs/020-proof-carrying.md`: adding a Related bullet grows the
file past 730 lines; inserting P4 text shifts Hartree off line 183.
020 already says in-process remint is not P4. That sentence stays.

**C6.2** An independently implemented Rust checker consumes one explicit
catalog obligation under spec 021's isolation and identity contract.
It does not share the production parser, expression constructors or verifier
internals. The host validates agreement and execution evidence before any
P4 admission; printing agree/disagree alone is insufficient. In-process
reproduce stays not P4.

**Done when:** CLI reproduce still refuses P4; a documented second
checker can agree on `dec.d-squared-zero` export.

### C7 — Autonomous research that is not catalog replay

**Why.** Loop proving every catalog slug is a demo, not a scientist.

**C7.1** Loop (or a budgeted subcommand) picks one `physis gaps`
node: missing-dataset -> refuse prove, cite an existing registered source
or emit a concrete missing-source/dataset work item;
missing-theorem on an evaluator-Holds catalog claim → prove;
insufficient-precision → do not upgrade to compatible;
intractable/open → leave Asserted. The selector, no-progress behavior and
role/trust/budget preservation are specified in spec 022.

**C7.2** Hypothesis search: constrained structural mutation remains
the only search. Do not invent a free-form LLM theory generator that
bypasses IR.

**Done when:** a lab test shows loop on a theory with no unproved
catalog identity still does gap-driven work (cite, enclose, or
hypothesize) rather than only `prove sr.*`.

### C8 — Human entry and architecture truth

**C8.1** Rewrite `docs/ARCHITECTURE.md` to include proof, verifier,
numeric, provenance, store, data, semantic, audit, constants,
agent roles. Holds is not a theorem.

**C8.2** README: one paragraph on how to read `physis why` (axes,
receipt vs none, danger note). String-critique remains the first
experiment; civilization lab is the trajectory.

**C8.3** Keep `AGENTS.md` item 2 as **one line**. Do not dump this
plan into that line.

**Done when:** a new agent can find the crate graph and the trust
rules without reading CHANGELOG archaeology.

## First eight agent turns (do these in order)

Architecture (C8.1), README guidance (C8.2) and the P4 specification (C6.1)
are delivered. The next implementation turns, subject to TODO dependencies:

1. C1.1 - SR mass shell reads versioned constants.
2. C1.2 - SR numerical bounds.
3. C1.3 - gravity numerical error budget.
4. C2.1 - kinematic receipt wiring tests if not already complete.
5. C3.1 - measured light-deflection comparison.
6. C7.1 - one gap-driven loop step.
7. C1.4 - reduced Planck constant representation.
8. C3.2 - measured Mercury perihelion.

Continue from the dependency-ready queue, including C3.3 before scaling the
agent machinery. C6.2 requires durable bundles and an isolated independent
checker. Do not start a ninth catalog polynomial instead of these tasks.

## Slice protocol (every turn)

1. Read this file and `AGENTS.md`. Confirm the slice is not forbidden.
2. Write the failing test or CLI assertion first when the slice is
   behavioral.
3. Implement the slice only.
4. `cargo fmt --all && cargo fmt --check` from the workspace root.
5. `cargo clippy --workspace --all-targets -- -D warnings`.
6. `cargo test --workspace`.
7. CLI honesty for any touched command: encode/why/prove/evidence/
   inspect as relevant. Unknown names on stderr. Unique-vacuum uses
   `evidence`/`why` on `predictivity.unique-vacuum`. Domain notes must
   not contain the substring `theory ` if `why_theory_block` would
   split them.
8. CHANGELOG Unreleased bullet. Pin hashes only after a failing
   `PIN_…` test prints the live value.
9. Commit and publish using the authorized repository workflow. Inspect the
   current CI configuration rather than reusing another session's subscription
   or assuming a review service is installed. Do not create a PR unless asked.
10. Update this checklist and TODO.md together. Do not mark the civilization goal
    complete until the end state is evidenced.

Documentation-only changes check links, source correspondence and status
consistency; they do not require the executable gates in steps 2-7.

## Checklist (maintainers / later agents)

- [x] C1.1 SR mass shell from versioned constants
- [x] C1.2 SR interval enclosure
- [ ] C1.3 gravity enclosure
- [ ] C1.4 `ħ` stored as listing
- [ ] C2.1 kinematic receipt wiring
- [ ] C2.2 non-leftover Faraday/DEC-Maxwell or documented skip
- [ ] C3.1 deflection EmpiricalReceipt
- [ ] C3.2 Mercury EmpiricalReceipt
- [ ] C3.3 thermal or spectroscopic EmpiricalReceipt
- [ ] C4.1 computed 2D area law
- [ ] C4.2 Maxwell on live 2-complex
- [ ] C4.3 named GR geometry computation
- [ ] C2.4 substantive GR formalization (later)
- [ ] C5.1 typed catalog trees in IR
- [ ] C5.2 gap-query completeness check (reuse if already implemented)
- [x] C6.1 P4 spec (spec 021; no runtime P4 implementation)
- [ ] C6.2 second implementation
- [ ] C7.1 gap-driven loop
- [ ] C7.2 typed hypothesis search
- [x] C8.1 architecture doc
- [x] C8.2 README trust paragraph
- [ ] C8.3 AGENTS.md item 2 still one line (standing; do not wrap)

## Related

- `plans/000-roadmap.md` — mechanical milestones already landed
- [TODO.md](../TODO.md) - dependency-ready implementation queue and campaign work
- [Independent reproduction](../specs/021-independent-reproduction.md) - C6.1 contract
- [Research campaigns](../specs/022-research-campaigns.md) - C7/R-series contract
- `specs/020-proof-carrying.md` — trust kernel contract (do not edit
  in C-slices; C6.1 is `specs/021-independent-reproduction.md`)
- `specs/000-overview.md` — v0 success (already true; this plan is after v0)
- `AGENTS.md` — standing orders
- `docs/ARCHITECTURE.md` — current crate and trust-boundary guide
