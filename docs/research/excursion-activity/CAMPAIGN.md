# Replaying the hidden-excursion condition search

The starting point was the five-state counterexample on `main` at `2e366de`.
The triangle fit overestimated dissipation because its two short count tails
did not constrain the mean length of a general excursion. This campaign
varied the mean jump count, cycle direction and branch weight, then tested
the resulting conditions on other graph topologies. [Starting counterexample](../excursion-counts/HIDDEN-ORDER.md) · [Theorem and constructions](RESULT.md)

## Joint conditions in the lab

Run from the repository root. Each invocation starts from the same five-state
model. The first branch has mean 1 and signed winding $1/2$. The second starts
at mean 9999 and winding 0. One percent of departures enter the first branch.

```sh
cargo build --locked -p physis
lab=./target/debug/physis
"$lab" run excursion-counts
"$lab" hypothesize excursion-counts
"$lab" conditions excursion-counts excursion.floor-attained \
  second_mean_jumps=1 second_winding_quarters=-2,2
```

All three claims fail at baseline. The six valid one-knob probes in
`hypothesize` found a change that restores a positive floor and the predicted
mean. None made the original network attain its floor. The joint search
keeps the failed controls for the very same `excursion.floor-attained` claim.

| Second-branch mean | Second-branch winding | Attains the floor |
|---:|---:|---|
| 9999 | 0 | fails at baseline |
| 1 | 0 | fails |
| 9999 | $-1/2$ | fails |
| 1 | $-1/2$ | holds, minimal change set |
| 9999 | $+1/2$ | fails |
| 1 | $+1/2$ | holds, minimal change set |

The following grid evaluates all 150 assignments. Five first-branch weights
times two winding signs give ten holding rows. The other 140 fail. Only two
rows are minimal repairs because changing the weight is unnecessary. The
proof in the result extends this conclusion to every strictly positive mix.

```sh
"$lab" --json conditions excursion-counts excursion.floor-attained \
  second_mean_jumps=1,2,3,15,99,9999 \
  second_winding_quarters=-2,-1,0,1,2 first_percent=1,25,50,75,99 \
  | jq '.conditions.rows | {points:length, holds:map(select(.outcome.verdict.kind=="holds"))|length, repairs:map(select(.minimal_repair)|.changes)}'
"$lab" replay docs/research/excursion-activity/knobs.jsonl
```

The replay reproduces both `set-knob` diffs, including their statement hashes
and assurance axes. The last recorded `run` has three holds. The grid command
returns the baseline and every tested row in JSON. It evaluates the claim on
forks and leaves the live state available for a journaled experiment. [Recorded knob path](knobs.jsonl) · [Factory command](../../../specs/006-agent-protocol.md)

## Searching for a genuinely three-statistic effect

The `excursion-ring` theory starts at equilibrium with all weights one. Its
claim requires all three pairwise reversible fits and a strictly positive
joint dissipation bound. Changing either directional weight alone fails
in this four-point grid. Changing both to 4 clockwise and 2 anticlockwise
while keeping the opposite weight at 1 succeeds. [Recorded ring knob path](ring-knobs.jsonl)

```sh
"$lab" conditions excursion-ring excursion.only-joint-detects clockwise=4 anticlockwise=2
"$lab" replay docs/research/excursion-activity/ring-knobs.jsonl
```

Replaying eight complete slices of 64 settings returns 22 successful rows
among 512. The integration test independently propagates each ring's traffic
matrix and compares its decision to the native factory evaluator at every
point.

```sh
study_dir=$(mktemp -d)
weights=1,2,3,4,5,6,7,8
for cross in 1 2 3 4 5 6 7 8; do
  "$lab" --journal "$study_dir/session.jsonl" set excursion-ring opposite "$cross" > /dev/null
  "$lab" --journal "$study_dir/session.jsonl" --json conditions excursion-ring \
    excursion.only-joint-detects "clockwise=$weights" "anticlockwise=$weights" > "$study_dir/grid-$cross.json"
done
jq -s '[.[].conditions.rows[]] | {points:length, holds:map(select(.outcome.verdict.kind=="holds"))|length}' "$study_dir"/grid-*.json
```

## Independent network checks

```sh
cargo test --locked -p physis-agent conditions::tests
cargo test --locked -p physis-theory --test excursion_activity -- --nocapture
```

The Rust test builds balanced traffic matrices, computes stationary visits,
propagates the hidden jump chain twice, and sums directed edge entropies. The
count predicates use exact rational arithmetic. Logarithmic entropy checks
use floating point, with absolute tolerance at most $10^{-11}$ per excursion.

| Four-state ring weights 1 through 8 | Mean test allows zero | Mean test forces dissipation |
|---|---:|---:|
| Two-tail test allows zero | 238 | 20 |
| Two-tail test forces dissipation | 10 | 244 |

The 512 settings count ordered integer weight triples, including proportional
and reversed choices. A joint test of all three statistics detects 28 of the
238 settings in the upper left. In 22 of those, **all three** possible pairs
admit an equilibrium fit. Tests reconstruct three exact reversible matrices
for the $4,2,1$ ring and check the positive joint bound against the driven
model. [Joint detection and branch constructions](JOINT.md)

A second grid varies three cycle weights on reciprocal
networks with 3 through 7 states. In all 2,560 cases the measured edge entropy
is at least the predicted floor up to tolerance, and in 2,289 it has a
strictly positive floor. Separate fixtures reach both minima and the
two-hidden-state boundary. A lift of the driven four-state example checks
every projected rate and stationary mass exactly with exit rates 2, 3, 5 and
7, and checks the predicted entropy increment at $p=1/2,3/4,99/100$.
[Independent matrix implementation and fixtures](../../../crates/physis-theory/tests/excursion_activity.rs)

## Algebra and its independent typecheck

Lean 4.34.0-rc2 checks eleven public lemmas, including an arbitrary-finite-list
sum-of-squares identity for both Cauchy bounds, the state-count ceiling, the
equality row and the lift's cost decomposition. Standalone nanoda 0.4.16
checked all 2,430 declarations in their joint export. To repeat this, use the
CI-pinned `lean4export`, the workspace-locked `nanoda_bin`, and a temporary
directory containing a copy of `Activity.lean`. [Lean source](Activity.lean)

```sh
lean -DwarningAsError=true -o Activity.olean Activity.lean
LEAN_PATH=. lean4export Activity -- \
  ExcursionActivity.finite_cauchy_identity ExcursionActivity.equal_deficits \
  ExcursionActivity.complement_product ExcursionActivity.equality_row \
  ExcursionActivity.minimizer_fluxes ExcursionActivity.mean_check \
  ExcursionActivity.state_count_slack ExcursionActivity.reversible_mixture \
  ExcursionActivity.lift_entropy_increment ExcursionActivity.even_pair_slack \
  ExcursionActivity.joint_ring_control > activity.ndjson
cat > check.json <<'EOF'
{"export_file_path":"activity.ndjson","permitted_axioms":["propext","Quot.sound","Classical.choice"],"unpermitted_axiom_hard_error":true,"print_success_message":true,"print_axioms":false,"nat_extension":true,"string_extension":true,"num_threads":1}
EOF
nanoda_bin check.json
```

The local legacy `physis-forge` also checked eleven polynomial identities or
inequalities and three exact counterexamples in an eight-worker batch with
caching disabled. Its direct search for the equality-row implication exhausted
256 candidates. The explicit identity

$$
(ku-1)^2-(1-kq)=ku\,[k(u+v)-2]-k\,[kuv-q]
$$

provides the missing algebraic step. The batch checks this identity and Lean
checks the original implication. The four-state counterexample refutes
$(1+n)(1-m)\ge1$ at $m=43/64,r=257/512,n=2$ even with $r\ge m^2$. The
retained five-state example refutes the reverse inequality even with $m^2>r$.
The third case refutes the joint equilibrium inequality at the $4,2,1$ ring
under all three pairwise compatibility conditions, with residual $-296/2401$.
The optional Forge executable used here has SHA-256
`fe12e26fe263c7cd7bd321a51fe99e21994538361e87ff92584be4a762a261bf`.
Its source is absent from this revision. The Rust and Lean replays above run
from the checkout. [Exact Forge inputs, certificates and assignments](forge-replay.jsonl)

```sh
physis-forge batch docs/research/excursion-activity/forge-replay.jsonl \
  --workers 8 --no-cache --compact
```

The stationarity argument, divergence inequality, characterization of equality
and path-law preservation have analytic proofs in the result. The grid and
formal algebra cover the indicated individual steps. All model inputs in
this campaign are synthetic.
