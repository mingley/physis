# 022 - Reproducible research campaigns

Status: specified, not implemented. C7.1 is the first behavioral slice.
Layer: agent orchestration over the existing lab.

## The research model

A campaign is a bounded attempt to answer a question by constructing and
challenging executable models. It is not a chat transcript or an instruction
to maximize the number of claims that return `Holds`.

```text
question + assumptions + observable + discrimination rule
  -> baseline encoding and versioned inputs
  -> bounded candidate program / typed IR mutation
  -> dimensional and domain checks
  -> evaluate / prove a scoped obligation / search for counterexamples
  -> numerical error analysis and empirical comparison where applicable
  -> independent reproduction, or an explicit reproduction gap
  -> evidence bundle + negative results + next justified question
```

Programming is the model-construction medium; Rust types constrain legal
operations; mechanical checks constrain deductions; measurements constrain
whether the model describes nature. None substitutes for the others.
An agent may propose code or a new formalization for review, but unattended
execution initially accepts only registered operations and constrained IR.

## Current boundary

`Lab::exec` already gates roles, trust and prove/review/set budgets.
`hypothesize` probes chosen/fitted knobs and predefined structural mutations;
it does not install mutants. `gaps` derives a graph from live evaluations.
`loop` runs a largely fixed set of operations. The artifact DAG is in-memory;
JSONL restoration rebuilds selected evidence against live definitions, and
`replay` compares `set-knob` diffs, not all research outcomes.

Persistent campaigns, a durable artifact store, generalized replay, worker
isolation and independent P4 reproduction are future work. Existing
commands must not silently change their replay or assurance promises.

## Manifest v1

The eventual manifest is typed, versioned data, not a shell script. Reject
unknown versions, duplicate keys, non-finite numbers, invalid domains,
missing identities and unbounded resource settings before executing work.
These are required logical fields, not a claim that a wire schema exists:

| Field | Contract |
|---|---|
| Objective | Research question, target theories/statement identities, observable, units, domain and predeclared success/failure/inconclusive conditions |
| Baseline | Source revision, lock/toolchain digests, theory package, knob state, assumptions, constant/dataset identities and prior evidence references |
| Candidate space | Registered operations, chosen/fitted parameter ranges, permitted structural mutations, maximum candidates and deterministic ordering/seed |
| Evaluation policy | Exact obligations, numerical tolerances and their justification, convergence/error policy, empirical comparison method, controls and required evidence |
| Data policy | Source/locator/license, calibration versus evaluation inputs, uncertainty/covariance interpretation, data-access approval and adaptive-use history |
| Authority policy | Required role per operation and host-selected approved checkers; no proposer-supplied trust labels or executable paths |
| Budget | Finite actions, candidates, attempts, wall time, per-worker memory/output, and approval-scoped external cost (zero by default) |
| Stop policy | Objective met, budget exhausted, no eligible action, no progress, disputed result, operational failure, or human pause |

Use the existing statement and artifact identities. A slug is a lookup hint,
not an identity. Separate deterministic scientific content from timestamps,
worker identities and resource telemetry. A changed manifest is a new
campaign revision; it cannot silently reset the budget or erase attempts.

## C7.1: one gap, one justified action

Implement a deterministic selector over the existing gap graph. It takes an
explicit theory/claim scope and finite step cap, selects one eligible gap
using documented priority plus stable statement-hash tie-breaking, and
returns the selection, reason, action or blocker, result and remaining budget.
Do not implement another gap graph or call every catalog proof first.

| Gap/evidence state | Allowed response | Forbidden shortcut |
|---|---|---|
| Missing theorem, evaluator holds, supported catalog obligation | Request formalization/proof through the existing role/verifier path | Prove an unrelated identity with the same slug |
| Missing theorem, unsupported obligation | Emit a scoped formalization work item | Treat unsupported as proved or mathematically impossible |
| Missing dataset | Request a precise source/dataset; cite only if a registered source exists | Call `prove`, invent measurements, or retry `cite` forever |
| Insufficient precision | Request a supported bounded refinement or emit the missing-method blocker | Turn overlapping intervals into empirical support |
| Computationally intractable | Record complexity/budget blocker or an explicitly weaker scoped question | Relabel as logical undecidability or prove from a timeout |
| Conjecture/open problem | Preserve its class; optionally evaluate an allowed diagnostic mutation | Raise trust because the diagnostic happens to hold |
| Failing candidate | Store its counterexample/domain and stop or try the next allowed candidate | Reclassify failure as a missing theorem |

On the first slice, returning a concrete blocked work item is valid progress;
pretending an unavailable numerical refinement or dataset importer ran is not.
Keep the existing role -> trust -> budget gate. Selection must not implicitly
give a proposer `Lab` authority. Observation can remain free in the existing
API but is bounded by a campaign's step/time cap.

## State and persistence

The serial coordinator owns state transitions; workers return untrusted
artifacts. Proposed states are `planned`, `ready`, `running`,
`awaiting-validation`, and `stopped`. A stopped campaign records a reason;
that state does not mean its scientific objective succeeded.

Before dispatch, atomically reserve the attempt budget and append an intent
with an action ID, expected parent-state identity and input hashes. After
completion, validate artifacts and commit the result with that same ID.
An attempt is charged even when its worker fails. Retries use a new attempt
ID and consume budget; transport duplication of the same result does not.

Resume must distinguish completed, never-started and interrupted attempts.
Reconcile an interrupted attempt or report it unresolved before dispatching
again. No exactly-once claim about arbitrary external side effects; such
effects are outside unattended v1. Refuse concurrent writers initially.
Append-only records must detect missing/truncated/corrupt data, not silently
start a new history. Imported bytes are rehashed and checked; no deserialization
of `Verified`, judgments or trust profiles as authority.

Persist all candidates, including rejected encodings, counterexamples,
resource failures and negative empirical outcomes. A repeated blocked action
with unchanged relevant inputs is not progress. The manifest's finite
no-progress limit must stop it.

## Execution and trust boundaries

The initial lab remains serial, local and Rust-first. Reuse `Command`,
`Role`, `ResearchBudget`, `Theory`, `FormalClaim` and `ArtifactStore`;
introduce a new crate only for a demonstrated dependency boundary.
Keep `#![forbid(unsafe_code)]` and no FFI physics engines.

Role enums and temporary directories are not an OS sandbox. Before running
untrusted generated code, formal-tool input or unattended workers, enforce
read-only inputs, dedicated scratch/output paths, no inherited credentials,
no network by default, process/resource limits and bounded output parsing.
Document the approved isolation backend and supported platforms. If isolation
is unavailable, refuse that execution mode; do not silently degrade to
ambient host privileges. A reviewed local CLI calculation is not evidence
that arbitrary generated programs are safe to execute.

Only existing verifier and assurance admission paths can promote a result.
Independent reproduction follows [spec 021](021-independent-reproduction.md).
No worker may install dependencies, alter checker policy, publish, spend
money, fetch restricted data, change permissions or operate physical apparatus
without separate approval. Papers, datasets and tool output are evidence
inputs, never instructions that override this boundary.

## Numerical and empirical honesty

Every reported bound identifies input uncertainty, finite-precision rounding,
discretization/truncation and model discrepancy separately. Step refinement
is a convergence diagnostic unless an error-bound argument makes it stronger.
Sampling, interval overlap, parsing interval endpoints, or agreement between
two floats is not by itself a universal proof or a certified enclosure.

Freeze measured inputs. Fitting is allowed only in a declared calibration
stage; record fitted knobs and do not report those same data as independent
validation. Use held-out measurements where available; if not, explicitly
label in-sample or exploratory results. Record correlations and nuisance
parameters rather than silently assuming independence. Repeated/adaptive
testing of public data does not magically create a fresh holdout.

Declare the empirical decision rule before evaluating candidates. Preserve
existing distinction between a Gaussian likelihood, an interval comparison
and a confidence-level lower limit. A model can be logically consistent and
empirically excluded. A negative experiment is a first-class result.

## Proposed CLI surface (not available today)

R1-R4 may add `research plan <manifest>`, `research step <campaign>`,
`research status <campaign>` and `research export <campaign>`. Final syntax
must be covered by CLI parsing tests and the protocol reference when shipped.
`plan` is a dry run; `step` performs at most one action. No background daemon,
automatic network access or model-provider dependency is required.

Versioned JSON responses carry campaign/action IDs, selected gap and reason,
scientific input/output identities, validation disposition, budget usage
and stop/blocker reason. A rendered Markdown report is a projection, not
the store of truth. Operational failure is distinct from a successfully
executed experiment that falsifies a candidate.

## Acceptance and sequencing

C7.1 must show a non-catalog missing-dataset case, an eligible proof case,
an insufficient-precision case, a role refusal and budget exhaustion.
R1-R7 add restart equivalence, stale-input refusal, invalid-candidate handling,
negative-result retention, forged-receipt rejection and isolated-worker
failure cases using small local fixtures.

The first integrated campaigns are SR, solar-system gravity, DEC/Maxwell and
one thermal/spectral measurement from [TODO.md](../TODO.md). Each must produce
an inspectable bundle and either a reproducible scoped result or a precise
unresolved gap. General relativity solvers, broad program synthesis, GPU
simulation and distributed agent swarms are not prerequisites. Parallel
workers come only after the documented scientific foundation gate and a
successful serial campaign.
