# Architecture

Physis is a Rust laboratory with separate model, evidence and verification
boundaries. A theory returning `Holds` is not the trusted boundary.
This page describes the source at the roadmap baseline in
[TODO.md](../TODO.md); future capabilities are labeled separately.

## Workspace map

This is a responsibility diagram, not a complete Cargo dependency graph.

```text
physis: CLI / facade
  |
physis-agent: Lab, Command/Response, roles, budgets, journal, replay
  |
  +-- model evaluation
  |     physis-theory -> physis-model -> physis-core
  |     physis-ir: declarative packages and constrained mutations
  |
  +-- evidence
  |     physis-constants + physis-numeric: versioned inputs, exact/bounded values
  |     physis-data + physis-provenance: measurements and precise sources
  |     physis-store: content-addressed artifact DAG
  |     physis-semantic + physis-audit: encoding review and adversarial cases
  |
  +-- proof boundary
        physis-proof: trusted challenges, catalog, untrusted proof artifacts
        physis-verifier: dual checks; sole mint of Verified<T>
          +-- recursive and postfix exact polynomial expanders
          +-- Lean kernel + nanoda on lean4export (configured external tools)
```

| Crate | Responsibility |
|---|---|
| `physis-core` | Dimensions/quantities, claims, assumptions, statement identity, assurance axes and typed judgments |
| `physis-model` | Spacetime, quantum state, particles, gauge groups and world projection |
| `physis-theory` | Registered physics/computation models, knobs, evaluators and structural mutations |
| `physis-ir` | Line-oriented theory packages, round-trip checks and bounded package mutations |
| `physis-constants` | Versioned SI/CODATA and other sourced quantities; exact values versus measured/rounded listings |
| `physis-numeric` | Exact ratios and interval/scientific-number representations; a representation alone is not a certificate |
| `physis-data` | Registered datasets, interval comparisons and likelihood evidence |
| `physis-provenance` | Source records with specific locators rather than slogans |
| `physis-proof` | Catalog obligations, expression/artifact formats and challenge generation; cannot mint |
| `physis-verifier` | Validate challenge/artifact binding, run both checkers and mint sealed receipts |
| `physis-store` | In-memory content-addressed nodes, dependency edges and descendant invalidation |
| `physis-semantic` | Evidence-backed review of an encoding bound to its statement identity |
| `physis-audit` | Adversarial examples which must not gain authority |
| `physis-agent` | Lab composition, protocol dispatch, role/trust/budget gates, journal and restoration |
| `physis` | Library facade and CLI; no theory-specific authority shortcuts |

The runtime and physics computations remain unsafe-free Rust. External formal
tools return artifacts through the verification boundary; they are not
unverified FFI physics engines.

## Data flow of a knob turn

1. Parse `set type-iib total_dim 9` into a typed `Command`.
2. `Lab::exec` checks role, then trust prerequisites, then action budget.
3. Resolve the theory and parse the value against its `KnobSpec` domain.
4. Evaluate claims before and after the mutation through `Theory`.
5. Return scientific-axis diffs, including changed judgment/assurance axes,
   and append the `SetKnob` event.

Evaluators range from encoded predicates to finite lattice/kinematic/numerical
calculations. This is not yet a general 3+1 field or spacetime solver.
Many live evaluators still consume `physis-model` floating-point quantities;
certificate-first inputs and numerical error accounting are C1 work, not a
property of every existing evaluator.

## Proof and evidence data flow

A live `Claim` produces a `FormalClaim` identity committing to its sentence,
assumptions, domain, units and other scientific commitments. A trusted
`Challenge` specifies the exact obligation. An untrusted proof artifact must
match that challenge and pass both backend checkers before `physis-verifier`
can mint `Verified<T>`.

The lab combines live evaluation with statement-bound receipts and reviews to
project judgments and trust. Exact algebra, numerical evidence, semantic review
and empirical comparison remain distinct. `ExactCertificate` is not a Lean
kernel proof; a kernel-checked identity is not an empirical endorsement.
`P3F` with unreviewed semantics remains visibly dangerous. No public enum or
JSON import may manufacture `Verified`, a trusted judgment or P4.

## Persistence and replay today

`ArtifactStore` is an in-memory DAG, not an on-disk research database.
`Journal` records JSONL events and maintains a hash-linked in-memory history;
hashing is not authentication or a complete crash-recovery protocol.
The CLI `--journal` path restores knob state and rebuilds selected receipts
and evidence against live definitions rather than deserializing authority.

`replay_journal` checks recorded `set-knob` diffs on a fresh lab. It does not
reproduce every proof, external process or empirical calculation in the
history. `reproduce` remints in-process and explicitly does not assign P4.
Do not share an active journal between concurrent writers.

## Agent boundary today and target

Roles are application-level permissions. They are not authenticated worker
identities, process isolation or an OS sandbox. The existing prove/review/set
budgets are not wall-time, memory, output-size or financial limits.
Likewise, a temporary directory for formal tools is not confinement.

Today, `hypothesize` explores predefined mutations without installing them;
`gaps` exposes research needs; `loop` runs a largely fixed cycle.
The target is a bounded, gap-driven serial campaign with typed proposals,
durable artifacts, explicit failure/stop reasons, enforced worker isolation
and independent reproduction. That target is specified in
[research campaigns](../specs/022-research-campaigns.md) and
[independent reproduction](../specs/021-independent-reproduction.md).
Neither is implemented by this documentation change.

Keep the trusted layer small. Model generation and search may grow above it,
but agent orchestration, report prose and persisted bytes never become new
sources of scientific authority.
