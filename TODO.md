# Agentic physics lab TODOs

Status: implementation queue. Documentation is not implementation.
Baseline: `8a45c9bf978425d6bfb8004719a56911d228cdc4` (2026-09-04 source review).

Build a lab where agents propose executable models, derive consequences,
try to break them, and compare predictions with measurements. The unit of
progress is a reproducible research result, including a negative result,
not another constant, a green `Holds`, or a longer agent conversation.

## Start here

Read [AGENTS.md](AGENTS.md), [the research workflow](docs/RESEARCH-LAB.md),
[the architecture](docs/ARCHITECTURE.md), and the relevant contract.
[Plan 006](plans/006-civilization-lab.md) defines the scientific scope;
this file is the dependency-ordered pickup queue and status index.
The older roadmap is a history of slices, not evidence that the lab is done.

**Next implementation task: none — queue exhausted.** R7 closed
the adversarial campaign regression suite; all four acceptance
campaigns are recorded; R8's gate was evaluated against measured
serial evidence and did not clear (see `docs/campaigns/r8-gate.md`).
The next work is new science or new plan scope, not an open queue item.
All unchecked tasks are unassigned. Select the first task whose dependencies
are complete, state its ID and owned files in the work handoff, and keep one
scientific idea per change. Independent tasks may proceed in separate files;
coordinate changes to `lab.rs`, `protocol.rs`, and shared claim identities.

The dependency column below is a merge gate, not a reason to avoid reading
ahead. Complete means the acceptance condition is demonstrated, with a commit
and relevant command/test recorded; a design document alone cannot close a
behavioral task. If existing code already satisfies a task, record evidence
instead of reimplementing it.

## Completed documentation slices

- [x] C6.1: [independent reproduction contract](specs/021-independent-reproduction.md).
  C6.2 implemented it with P4 runtime support (checker, pipeline, sealed admission).
- [x] C8.1: [architecture](docs/ARCHITECTURE.md) describes the current crates
  and separates application role gates from process isolation.
- [x] C8.2: [README](README.md) links the research path and explains `why`.
- [x] R0: [research campaign contract](specs/022-research-campaigns.md) and
  [agent workflow](docs/RESEARCH-LAB.md). No new commands are implemented.

## Foundation: make the existing physics dependable

`C*` IDs retain their meaning from plan 006. Paths in this table are relative
to `crates/`; each task also updates its affected docs and honesty tests.

| Task | Depends on | Implementation surface | Acceptance condition |
|---|---|---|---|
| [x] **C1.1** Versioned inputs in SR mass shell | None | `physis-theory/src/special_relativity.rs`, `physis-constants` | Read existing `c` and `m_e` entries; expose their identities and uncertainties in evidence. Galilean control still fails; no trust promotion or new table row. |
| [x] **C1.2** Defensible SR numerical bounds | C1.1 | `physis-numeric`, SR evaluator | State the numerical domain and error sources. A threshold-straddling result is unresolved; interval overlap alone is not proof of equality. Exact catalog identity remains distinct from its floating-point evaluation. |
| [x] **C1.3** Gravity error budget | C1.2 | `physis-theory/src/gravity.rs`, `physis-numeric` | Separate input, rounding, and integration/truncation error for deflection and perihelion. Coarse steps cannot become certified by wrapping a point estimate in an interval. |
| [x] **C2.1** Live kinematic receipt binding | C1.2 | `physis-agent/src/lab.rs`, SR tests | In one journal, prove/why bind interval, composition, and mass-shell receipts to their exact statements. A changed assumption/encoding cannot borrow the old receipt. Do not claim a polynomial proves the whole numerical evaluator. |
| [x] **C3.1** Measured light deflection | C1.3 | `physis-data`, `physis-provenance`, gravity | Register a precisely located, licensed measurement with units and uncertainty semantics. Compare both GR and Soldner under the same predeclared rule; report the data-derived outcome, not a hardcoded winner. |
| [x] **C7.1** One gap-driven research step | C2.1, C3.1 | `physis-agent/src/lab.rs`, `role.rs`, `protocol.rs` | Select one eligible gap with a stable tie-break; run an allowed existing operation or emit an explicit blocked result. A non-catalog missing-dataset case does useful work without trying to prove it. No unbounded loop. See spec 022. |
| [x] **C1.4** Reduced Planck constant representation | C1.3 | `physis-constants`, affected docs | Distinguish exact `h/(2*pi)` from a finite tabulated approximation. Document its rounding bound and source; do not invent measurement uncertainty for SI-exact `h` or certify a decimal as exact `hbar`. No sibling constants in this change. |
| [x] **C3.2** Measured Mercury perihelion | C3.1 | `physis-data`, gravity | Bind residual precession, reference frame, time units, subtraction model, and uncertainty to the dataset. Separate measured residual from a textbook GR prediction. |
| [x] **C3.3** One thermal or spectroscopic benchmark | C3.2 | `physis-data`, one existing theory | Use an independently measured observable, not a constant derived from the same model used to predict it. Record fit/calibration inputs separately; retain incompatible and inconclusive outcomes. |

For C3, historical data need not decisively exclude Newton/Soldner. Never
choose an uncertainty or dataset merely to force a verdict. A Gaussian
one-sigma interval is not a rigorous bound or a universal exclusion rule.
Preserve existing receipt semantics; a new statistical interpretation needs
an explicit contract and tests.

## Physics and language: deepen, do not collect examples

| Task | Depends on | Implementation surface | Acceptance condition |
|---|---|---|---|
| [x] **C5.1** Typed catalog expressions in IR | C2.1 | `physis-ir`, `physis-theory` | Round-trip a typed identity alongside legacy tokens. Missing/mismatched trees fail closed; `encode` never claims to have proved them. |
| [x] **C4.1** Computed 2D gauge benchmark | C3.3 | `physis-theory/src/gauge_field.rs` | Use finite enumeration or a transfer matrix with named volume/boundary assumptions; compare an independently derived reference and a failing mutation. Do not claim the 4D mass gap. |
| [x] **C4.2** Maxwell on a live 2-complex | C5.1, C4.1 | `physis-theory/src/em.rs`, `dec.rs` | Shared oriented incidence data drives the field computation; orientation/boundary mutations produce explained diffs. Not a relabeled simplex identity. |
| [x] **C2.2** A relevant Faraday proof obligation | C4.2 | `physis-proof`, `physis-verifier`, `formal/physlib` | Bind a genuinely distinct live EM obligation and reject a sign-mutated artifact. If it is just existing coboundary algebra, document the skip instead of adding a duplicate. **Skip taken:** flux ≡ −`tetrahedron_d2` (test-pinned); see spec 008. |
| [x] **C4.3** A named GR geometry computation | C4.1 | `physis-theory/src/gravity.rs`, `geometry.rs` | Compute a curvature/metric identity on an explicit coordinate domain; singular coordinates and conventions are inputs, not hidden assumptions. Keep encoded identities distinct from Einstein-equation proofs. (Computation in `gravity.rs`, claim on GR in `relativity.rs`; `geometry.rs` is the unification scaffold, untouched.) |
| [x] **C2.4** A substantive GR formalization (later) | C4.3, C5.1 | GR IR, `formal/physlib` | First specify a live tensor expression and its assumptions. Only then formalize a matching obligation; no cosmetic `lean_ref` and no promise to solve open physics. (First Bianchi `R^t_[rθφ]`; contracted `∇·G` needs differential hypotheses — see spec 018.) |
| [x] **C5.2** Gap-query completeness check | C7.1 | Existing `inspect`/`gaps` paths | Demonstrate all gap kinds needed by C7.1 can be queried. Close with evidence if already supported; do not build a second gap graph. (Evidence close: all 7 kinds queryable, one graph; test-pinned.) |
| [x] **C7.2** Typed hypothesis search | C5.1, C7.1 | `physis-ir`, `Theory::structural_mutations` | Explore a bounded mutation family, retain baseline and counterexamples, reject dimensionally invalid candidates, and keep measured inputs frozen. Candidate evaluations cannot modify the trusted baseline. (Depth-2 closure + fork-only knob probes + type gate; `Theory::fork`; test-pinned.) |
| [x] **C6.2** Independent reproduction implementation | C6.1, R2, R3 | New independent checker plus verifier/agent integration | Meet spec 021 on `dec.d-squared-zero`, including sign corruption, wrong statement, copied checker, timeout, and forged agreement cases. Existing `reproduce` remains an in-process remint, never P4. (Checker crate + `physis-verifier::reproduce` + sealed P4 + lab hookup; 72 tests.) |

## Research environment: only after a useful single-step lab

**Foundation gate F:** C1.1-C1.4, C2.1, C3.1-C3.3, and C7.1 are complete;
C2.2 is complete or has the documented plan-006 skip. R1 may build the
small serial campaign contract after C7.1; R2 onward waits for F.
This gate intentionally prevents building throughput before scientific value.

| Task | Depends on | Implementation surface | Acceptance condition |
|---|---|---|---|
| [x] **R1** Versioned campaign manifest | R0, C7.1 | `physis-agent`, CLI | Implement spec 022's typed objective, allowed operations, source/input identities, finite budgets, stop conditions, and schema refusal. A dry-run plan executes nothing and grants no authority. (`campaign.rs` + `research plan`; 44 tests; test-pinned.) |
| [x] **R2** Durable bundles and crash recovery | F, R1 | `physis-store`, `journal.rs`, `replay.rs` | Persist raw content-addressed artifacts and action intents/results. Kill/restart at each write boundary without losing charged work or repeating a completed action. Detect missing/corrupt objects; loading bytes never restores trust. (`DurableStore` + `Bundle`; lab journal/replay deliberately untouched — separate strict history; 23 tests.) |
| [x] **R3** Isolated worker boundary | F, R1 | Host runner and verifier subprocess boundary | Enforce finite time/memory/output limits, read-only inputs, private output directory, no credentials and no network by default. On unsupported isolation, refuse unattended untrusted execution instead of running with ambient privileges. (`physis-core::worker` + lean routing; memory/untrusted refused without a backend; 23 tests.) |
| [x] **R4** Resumable serial campaigns | R2, R3, C7.2 | `physis-agent`, CLI | Implement one-step/resume/status with persisted budgets, deterministic selection and explicit stop reasons. Restore after interruption; do not retry blocked work until a relevant input changes. (`coordinator.rs` + `research step`/`research status`; checkpointed usage/streak/blocked/stop; 21 tests.) |
| [x] **R5** Empirical experiment design | R4, C3.3 | `physis-data`, existing `design`/`sensitivity` | Rank a bounded set of discriminating observables with units, reachable domains and costs. Freeze the comparison rule before evaluation; disclose fitted inputs, covariance assumptions and repeated/adaptive use of data. No automatic data downloads. (Pure ranking core + journaled freeze in `design`; 11 tests.) |
| [x] **R6** Inspectable research report | R4, C6.2 | CLI/report projection over store | Export objective, assumptions, candidate lineage, all attempts, negative results, bounds, dataset IDs, receipts and unresolved gaps. A clean checkout can reproduce the scoped result or get a precise blocker; prose is not evidence. (`report.rs` + `research export [json|markdown]`; 9 tests.) |
| [x] **R7** Adversarial campaign regression suite | R4, R5, R6 | `physis-audit`, existing Rust tests | Exercise stale receipts, malformed manifests, unit/sign errors, dataset leakage, restart/budget bypass, tampered bundles and false checker agreement. None may promote trust. (`campaign_corpus` + `attack_campaign` in `physis-audit`, run by `physis audit`; 7 agent regression pins; no finding promoted; 8 tests.) |
| [ ] **R8** Bounded parallel workers (later; gate evaluated, not cleared) | R7 | Campaign scheduler | Two workers cannot spend the same budget or publish conflicting state. Workers return untrusted artifacts; one coordinator commits results. Show better useful-results-per-budget than serial execution before adding queues or chat infrastructure. (Gate evaluated against sweep-1 + acceptance campaigns: serial is 1 result/action at ~2 results/s with ms-scale attempts and a single-writer bundle, so parallelism cannot win; see `docs/campaigns/r8-gate.md` for evidence and reopen criteria.) |

## Campaigns that demonstrate the approach

These are acceptance scenarios for R4-R7, not claims of new discoveries.
All four are recorded under `docs/campaigns/` (kinematics-1, gravity-1,
field-1, thermal-1) with manifests, stopped bundles, exports, and
serial-baseline notes; see `docs/campaigns/README.md` for the R8 gate input.

| Campaign | Candidate/control | Required result |
|---|---|---|
| Kinematics | Lorentz versus Galilean/truncated boost | Exact algebra and numerical-domain checks remain distinct; invalid candidate or domain is explained, not hidden. |
| Gravity | Newton/Soldner versus Schwarzschild | Bound numerical error, freeze measured inputs, and compare the same deflection/perihelion data. Report inconclusive if the data cannot discriminate. |
| Field structure | DEC/Maxwell with orientation or boundary mutation | Produce a minimal counterexample and identify the changed assumption/encoding, not just a failed boolean. |
| Thermal/spectral model | One C3.3 model and a documented alternative | Separate calibration from evaluation; reproduce an out-of-calibration prediction and retain a failed alternative. |

Success is a bundle someone else can inspect and replay, with a smaller
well-specified uncertainty or a falsified candidate. Proof counts, constant
counts, CLI command counts, and agent throughput are not scientific goals.

## Handoff and definition of done

Use this compact handoff in the change description or active work thread:

```text
Task ID / owned files:
Baseline commit and dependent task evidence:
Question, assumptions, domain and expected scientific-axis change:
Implemented artifact or explicit blocker:
Commands/tests and observed result:
Remaining uncertainty / next task:
```

For behavior changes, add a positive case, a negative control, and the relevant
trust-boundary failure case. Use the smallest existing Rust test selector
first; follow the repository's full gates before publishing executable changes.
For documentation-only work, check links, status consistency, and correspondence
with source; do not mark executable milestones done. Update this queue and
plan 006 together when closing a `C*` task.

Never change spec 020 in these slices, wrap AGENTS item 2, add a cosmetic proof
pointer, add a sibling table row as the primary change, or promote conjecture
by enum assignment. No new physics engine, plugin, service, dataset download,
paid compute, publication, or physical experiment without the relevant approval.
