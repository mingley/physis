# R8 gate evaluation: bounded parallel workers stay deferred

R8's own acceptance sets two requirements: (1) two workers cannot
spend the same budget or publish conflicting state, with one
coordinator committing results; (2) show better
useful-results-per-budget than serial execution *before* adding
queues or chat infrastructure. This note evaluates (2) against
measured serial evidence. It does not clear.

## Serial evidence

- Four acceptance campaigns + one sweep, all recorded in this
  directory with bundles and exports.
- Sweep-1 (largest run): 9 receipts / 9 actions / 4.6s wall —
  1 useful result per action at ~2 results per wall-second.
- Single-op wall (built debug binary): prove ≈ 0.36s,
  hypothesize ≈ 0.02s, cite ≈ 0.06s. Every campaign op is
  sub-second and boot-dominated; per-attempt compute is
  milliseconds.

## Why the gate does not clear

1. **No wall headroom.** Parallel workers win when attempts are
   wall-dominated. Here each attempt is milliseconds of compute
   inside ~0.5s of process boot and lab construction. Two
   workers would each pay their own boot and contend on the
   bundle lock while splitting milliseconds of compute.
2. **No budget headroom.** Useful-results-per-action is already
   1:1 on eligible gaps (every dispatched attempt mints,
   fails honestly, or blocks precisely). Parallelism cannot
   mint more receipts per attempt than the gaps contain.
3. **The substrate says no.** R2 refuses concurrent bundle
   writers by design (`second_writer_refused_until_close`);
   attempt ids come from a sequential counter. R8 needs a
   lock/commit-queue redesign first — exactly the infrastructure
   the gate forbids building before the evidence exists.

## Reopen criteria

Revisit R8 only when all three hold:

1. A campaign op exists whose wall cost dominates boot by 10×
   or more on a real campaign (e.g. reproduce-as-an-op with
   worker-subprocess runs, or hypothesize spaces with genuine
   per-candidate cost).
2. A recorded serial campaign shows wall, not gaps, as the
   binding constraint on useful results.
3. A bundle multi-writer or commit-queue design is specified
   that keeps "no double-charge, no conflicting publish"
   (R8's requirement 1) without weakening R2 recovery.

Until then, serial dispatch stands: it is fast, exactly
budgeted, and every refusal is inspectable.
