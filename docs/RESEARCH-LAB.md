# Working in the agentic physics lab

Physis aims to make research a sequence of inspectable programs and evidence,
not a sequence of persuasive answers. An agent proposes an encoding, asks what
it predicts, searches for a counterexample, and records what survived. Rust
constrains the program; a verifier checks a scoped deduction; measurements
test the physical interpretation.

This is a research environment under construction. The current CLI supports
typed experiments, mutations, receipts and journaled knob changes. Durable
campaign orchestration and independent reproduction are specified, not shipped.
Start implementation work from [TODO.md](../TODO.md).

## What makes a useful research task

State the question, baseline and competing encoding, assumptions, domain,
observable, units, decision rule and finite budget. Name what would falsify
the candidate or leave the question unresolved. Do not begin with "prove this
theory right."

For example, a solar light-deflection task should compare computed
Newton/Soldner and Schwarzschild predictions against the *same* located
measurement. It needs numerical error bounds and the measurement's uncertainty
model, not just a check that two textbook numbers differ. A kinematics task
can establish an exact polynomial identity without thereby establishing
empirical Lorentz invariance or validating every floating-point evaluation.

## A small session using existing commands

From the repository root, these commands use a new journal filename in a
writable working directory. Do not reuse somebody else's active journal.

```bash
# Observe a supported exact identity and its current evidence.
cargo run -p physis -- why dec.d-squared-zero
cargo run -p physis -- --journal triangle-session.jsonl prove dec.d-squared-zero
cargo run -p physis -- --journal triangle-session.jsonl why dec.d-squared-zero

# Explore an encoding mutation without installing it as the new baseline.
cargo run -p physis -- --json --role explorer hypothesize special-relativity

# Find a research gap instead of proving another catalog identity.
cargo run -p physis -- --json gaps
cargo run -p physis -- --json inspect gap missing-dataset
```

Use only the repository's approved toolchain/dependencies. `prove` reports
the backend actually used: Lean kernel/nanoda when the configured tools are
available, otherwise the exact-certificate path. Neither a missing tool nor
an unsupported claim justifies inventing a success.

For knob-session replay, use a separate journal:

```bash
cargo run -p physis -- --journal knob-session.jsonl set type-iib total_dim 9
cargo run -p physis -- replay knob-session.jsonl
```

That checks recorded `set-knob` diffs. It does not certify the complete
research history, external execution, independent replication or nature.
Likewise, `reproduce` is an in-process remint, not P4.

## Read the evidence before the verdict

| Question | Inspect |
|---|---|
| What sentence was evaluated? | Statement hash, theory encoding, assumptions and domain in `why` |
| What did the evaluator do? | Claim class, derivation axis and typed judgment |
| Was an exact obligation checked? | Receipt, backend, axioms and its matching statement identity |
| Are numbers defensible? | Units, versioned inputs, numerical bounds and their justification |
| Was anything measured? | Dataset identity, locator, calibration history and statistical interpretation |
| Does the encoding mean what we say? | Semantic review, independent encoding and unresolved assumptions |
| Can another implementation reproduce it? | Currently a gap; follow spec 021 rather than calling a remint independent |

`Holds` is local to an encoding. A checked theorem can concern a poor model.
An empirically excluded model can have impeccable algebra. A conjecture that
survives a finite search is still a conjecture.

## Working contract for implementation agents

Select one ready TODO, identify the files you own, and keep the baseline
observable while developing a candidate. Add a negative control alongside
the successful case. Reuse existing typed protocol and evidence machinery;
do not add a special CLI escape hatch that bypasses the verifier.

Record the task ID, source revision, changed scientific axes, commands/results,
remaining uncertainty and next task in the handoff. A blocked formalization,
insufficient precision or unavailable dataset is an acceptable honest result.
Do not silently skip it, retune measured inputs, or replace it with an easier
but irrelevant theorem.

The target workflow, artifact contract, budgets and stop conditions are in
[spec 022](../specs/022-research-campaigns.md). Independent checking is in
[spec 021](../specs/021-independent-reproduction.md). Neither document grants
permission to execute arbitrary generated code, install unreviewed tools,
publish a result, spend money or run a physical experiment.
