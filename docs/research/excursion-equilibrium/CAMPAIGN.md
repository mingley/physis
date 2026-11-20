# Exact decisions, independent witnesses and replay

This campaign started at `c705559`. The earlier factory detected 22 four-state
ring settings where all three count pairs admit equilibrium but a joint
inequality fails. We kept that claim and its journal. A second claim asks
whether even that joint inequality passes while the exact test still forces
a positive entropy floor. [Theorem and assumptions](RESULT.md)

## Turning the factory knobs

```sh
cargo build --locked -p physis
lab=./target/debug/physis
"$lab" conditions excursion-ring excursion.beyond-joint-bound clockwise=3 anticlockwise=2
"$lab" replay docs/research/excursion-equilibrium/knobs.jsonl
```

The baseline and both one-knob controls fail. Changing both directional
weights succeeds with the opposite jump weight fixed at one. The previous
joint test passes on this ring. [Recorded knob path](knobs.jsonl)

```sh
study_dir=$(mktemp -d)
weights=1,2,3,4,5,6,7,8
for opposite in 1 2 3 4 5 6 7 8; do
  "$lab" --journal "$study_dir/state.jsonl" set excursion-ring opposite "$opposite" > /dev/null
  "$lab" --journal "$study_dir/state.jsonl" --json conditions excursion-ring \
    excursion.beyond-joint-bound "clockwise=$weights" "anticlockwise=$weights" > "$study_dir/grid-$opposite.json"
done
jq -s '[.[].conditions.rows[]] | {points:length, holds:map(select(.outcome.verdict.kind=="holds"))|length}' "$study_dir"/grid-*.json
```

There are 18 additional detections in 512 ordered weight settings, for a total
of 40 where all three pairs admit equilibrium and the triple forces positive
dissipation. The exact classifier excludes equilibrium in 320 overall. Of the
192 compatible settings, 64 are reversible rings and 128 are driven rings
whose measurements admit a reversible alternative. These counts include
rescalings and reversed choices.

## Evaluating arbitrary rational data

The research example takes $m=P(N\ge1)$, $r=P(N\ge2)$, $n=E[N]$, as
integers or reduced fractions in the project's signed 128-bit input format.
It uses arbitrary-precision arithmetic for the decision.

```sh
cargo run --locked -p physis-theory --example excursion_equilibrium -- 23/36 11/24 2
cargo run --locked -p physis-theory --example excursion_equilibrium -- 7/40 13/160 5/18
cargo run --locked -p physis-theory --example excursion_equilibrium -- 1/2 1/4 2
```

The respective results are `positive-entropy-floor`, `equilibrium-attained`
and `zero-infimum-unattained`. `invalid-counts` flags violations of the
elementary probability and mean constraints. A positive-floor result applies
to every physically realizable model with those inputs.

For exact comparison with an algebraic frontier the program avoids rounding
its root. In the interior define $d=m-r$, $v=r-m^2$, $k=1-2m+r$ and
$e=m^2-nd$. The constructive mean $F(y)$ in the theorem attains every value
above its minimum. For $e\le0$ we already have $F(0)\le n$. Otherwise
substitute $y=vz/(1+kz)$, $z\ge0$. Multiplying $F(y)-n$ by the positive
$d(1+kz)^2$ gives $P(z)=az^3+bz^2+cz+e$, where

$$
a=2d^3v,\quad b=k^2 e+v^2-2rv(k-d),\quad c=2(ke-rv).
$$

Here $a,e>0$. A nonpositive value on the positive axis exists exactly when
the cubic has positive real roots, including a repeated positive root. This
is equivalent to a nonnegative discriminant and at least one of $b,c$ being
negative. Indeed with all roots real their product is negative. Three
negative roots would give strictly positive coefficients, and nonnegative
coefficients exclude a positive root. The native classifier uses these sign
tests. [Rust implementation](../../../crates/physis-theory/src/excursion/equilibrium.rs)

## Independent checks and arithmetic repair

```sh
cargo test --locked -p physis-numeric
cargo test --locked -p physis-theory --test excursion_activity
```

For every one of the 512 rings the test propagates its actual traffic matrix.
For each interior count pair it independently finds either a rational
separating plane or a rational branch mixture with mean at most the supplied
mean. Sixteen trial parameters per family suffice on this grid. The test also
checks 384 connected reversible networks with 3 through 8 states, including
edges whose endpoints both have zero boundary flux. Nine exact interior
frontier mixtures test a zero discriminant and mean perturbations of
$\pm10^{-25}$. Four five-state boundary examples reproduce the three input
statistics and compare the entropy from reconstructed edge traffic with the
analytic expression. At slow-branch continuation $1-10^{-5}$ the entropy is
about $0.0000200003\,k_B$ per excursion. [Network tests](../../../crates/physis-theory/tests/excursion_activity.rs)

For the $3,2,1$ ring, $a=20513/25000$ certifies the lower end of the quoted
interval for $n_*$. The mixture parameter $y=863/10000$ certifies the upper
end. The rational plane at $a=9/11$ in the entropy estimate gives more than
$0.0000845\,k_B$ per excursion. The plotted frontier is an illustration of
the analytic formula. [Figure source](plot_frontier.py)

Two of the new regression tests exposed silent saturation in the original
fixed-width `Ratio` operations. Intermediate products overflowed even when
the reduced answer fit. For $k=2^{127}-1$, the earlier implementation of
$1/k+1/k$ returned one. The reduced answer is $2/k$. The fixed-width fast path now uses checked products
and sums, and larger intermediates are reduced at arbitrary precision. The
`checked_add`, `checked_sub`, `checked_mul` and `checked_div` methods report
unrepresentable results as `None`. Arithmetic operators fail explicitly in
that case. [Numeric implementation and regression](../../../crates/physis-numeric/src/lib.rs)

## Formal check coverage

Lean 4.34.0-rc2 checks nine polynomial identities, including the two support
factorizations, the contact cubic, the exact decision polynomial and both
identities used in the zero-infimum construction. Standalone nanoda 0.4.16
checked their export of 2,391 declarations. In a temporary directory containing
`Equilibrium.lean`, with the CI-pinned exporter and workspace-locked nanoda,
run the following. [Lean source](Equilibrium.lean)

```sh
lean -DwarningAsError=true -o Equilibrium.olean Equilibrium.lean
LEAN_PATH=. lean4export Equilibrium -- \
  ExcursionEquilibrium.edge_slack ExcursionEquilibrium.geometric_support \
  ExcursionEquilibrium.path_support ExcursionEquilibrium.contact_chord \
  ExcursionEquilibrium.mixture_capacity ExcursionEquilibrium.pole_gap \
  ExcursionEquilibrium.decision_cubic ExcursionEquilibrium.trap_mean \
  ExcursionEquilibrium.trap_deficit > equilibrium.ndjson
cat > check.json <<'EOF'
{"export_file_path":"equilibrium.ndjson","permitted_axioms":["propext","Quot.sound","Classical.choice"],"unpermitted_axiom_hard_error":true,"print_success_message":true,"print_axioms":false,"nat_extension":true,"string_extension":true,"num_threads":1}
EOF
nanoda_bin check.json
```

The local legacy Forge executable separately checked eight ordered-ring
identities and the $3,2,1$ counterexample under all preceding tests. Its
SHA-256 is `fe12e26fe263c7cd7bd321a51fe99e21994538361e87ff92584be4a762a261bf`.
Its source is absent from this revision. The Rust and Lean replays run from
the checkout. The probabilistic decomposition, equality rigidity, continuous
attainment and relative-entropy arguments have written analytic proofs.
Historical priority and experimental relevance remain open. [Forge replay jobs](forge-replay.jsonl)

```sh
physis-forge batch docs/research/excursion-equilibrium/forge-replay.jsonl --workers 8 --no-cache --compact
```
