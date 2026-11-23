# Factory replay and performance measurements

The starting revision was `b1fd944f18a3200575d7fc2671de8c313dab8419`. Its
three-count classifier decided when a positive entropy floor exists and left
its value open on the face $P(N=1)=0$. The four-state cycle below tests
equality and excess entropy at the same clock. The proof permits every
finite hidden order. [Sharp floor](RESULT.md) · [General linear certificate](LINEAR.md)

## Reproduce the failed assertion and its repair

From the repository root, start with a fresh journal and the default unit-rate
square. The two integer knobs are offsets in units of $1/24$. In the notation
of the proof, $p=1/2+v$, $z=1/2-v$, $x=1/2+u$, $y=1/2-u$, where $v$ is
the entry bias and $u$ the continuation bias. Every occupation is $1/4$.
The measured first tail is $m=1/2+2uv$, later paired continuations have
probability $1-m$, and $n=2$. The true entropy is
$4(u+v)[\operatorname{atanh}(2u)+\operatorname{atanh}(2v)]$ per excursion.

```sh
cargo build --locked -p physis
lab=./target/debug/physis
study_dir=$(mktemp -d)
"$lab" knobs excursion-square
"$lab" run excursion-square
"$lab" --journal "$study_dir/square.jsonl" set excursion-square entry_bias 9
"$lab" --journal "$study_dir/square.jsonl" set excursion-square continuation_bias 4
"$lab" --journal "$study_dir/square.jsonl" conditions excursion-square \
  excursion.square-attains-minimum entry_bias=6 continuation_bias=6
"$lab" replay docs/research/excursion-entropy/knobs.jsonl
```

| Entry, continuation | First tail | Entropy per excursion | Attains the all-order minimum |
|---|---:|---:|---|
| 9, 4 | $5/8$ | $(13/12)\log 14$ | Fails |
| 6, 4 | $7/12$ | Computed on its changed clock | Fails |
| 9, 6 | $11/16$ | Computed on its changed clock | Fails |
| 6, 6 | $5/8$ | $2\log 3$ | Holds |

The complete observed clock agrees at the two ends. The joint change is a
minimal repair under the factory's exact-subset definition. Varying each
integer from $-11$ through 11 gives 529 settings. The forced-positive claim
holds at 242 and the minimum-attainment claim at 45. The first requires
$uv>0$. At a positive floor the fluxes attain the bound exactly when $u=v$.
Indeed for the two equally visited boundary sites the Cauchy equality
requires $(1/2+v)(1/2-u)=(1/2-v)(1/2+u)$. At a zero floor equality is
detailed balance, $u=-v$. [Recorded knob changes](knobs.jsonl)

```sh
values=$(seq -s, -11 11)
for entry in $(seq -11 11); do
  "$lab" --journal "$study_dir/sweep.jsonl" set excursion-square entry_bias "$entry" >/dev/null
  for claim in square-forces-dissipation square-attains-minimum; do
    "$lab" --journal "$study_dir/sweep.jsonl" --json conditions excursion-square \
      "excursion.$claim" "continuation_bias=$values" > "$study_dir/$claim.$entry.json"
  done
done
for claim in square-forces-dissipation square-attains-minimum; do
  jq -s '[.[].conditions.rows[].outcome.verdict.kind] | {points:length, holds:map(select(.=="holds"))|length}' "$study_dir/$claim".*.json
done
```

## Evaluate other measurements

```sh
cargo run --locked -p physis-theory --example excursion_entropy -- no-one-jump 5/8 2
cargo run --locked -p physis-theory --example excursion_entropy -- no-one-jump 11/18 1805/576
cargo run --locked -p physis-theory --example excursion_entropy -- plane 23/36 11/24 2 9/11
cargo run --locked -p physis-theory --example excursion_entropy -- two-tail 2/5 1/8
cargo test --locked -p physis-theory --test excursion_activity
cargo test --locked -p physis-agent conditions::
```

The first command evaluates the sharp floor. The second evaluates the
odd-count bound using the effective mean $19/6-19/576$. The third prints the
exact certificate $2189228/172485621$. The fourth gives $28/275$ using
only the first two tails. The logarithmic display computes
the deficit and both squared gaps with arbitrary-precision rational arithmetic
before converting to floating point. This keeps finite values near a
vanishing reverse flux representable even when the inferred winding rounds
to one. All model verdicts use exact comparisons.

The Rust tests propagate probability and sum entropy from the directed
traffic matrix independently of the square evaluator. They check all 529
square settings, six fixed-occupation paths across the three regimes,
and 128 networks with longer odd excursions. Another 128 directed networks
with four through seven sites check the two symmetrization identities
exactly and the entropy inequality at three planes each. Four perturbations
of a reversible contact give the predicted quadratic changes in both counts.
The five-state odd-count control and the 3:2:1 ring are tested separately.
[Tests](../../../crates/physis-theory/tests/excursion_activity/entropy.rs)

## Faster exact classifications and condition grids

For a common positive integer denominator $L$, write $m=M_0/L$,
$r=R/L$, $n=N/L$. The classifier forms the integer polynomial $L^2P(Lz)$
and evaluates its discriminant sign. This preserves all roots' signs and
multiplicities, including exact double roots, and removes repeated
fraction reduction. The generic condition search also caches whether every
subset of an assignment failed. Every proper subset omits some nonzero
mixed-radix digit, so checking the cached single-coordinate reversions
suffices. The subset check takes $O(Pk)$ operations over $P$ points and $k$
axes, plus one Boolean per row. Rejections and undecidable verdicts still
block minimality. [Classifier](../../../crates/physis-theory/src/excursion/equilibrium.rs) ·
[Grid search and exhaustive-subset regression](../../../crates/physis-agent/src/conditions.rs)

One AMD EPYC 9V74 logical CPU, rustc 1.99.1, repository release profile.
The table reports medians of seven paired trials after one warm-up, with
the order of the two builds alternating. End-to-end time includes eight
fresh CLI processes, journal restoration, JSON serialization and hashing.

| Workload | Starting revision | This implementation | Speedup |
|---|---:|---:|---:|
| Exact classifier, 512 rings repeated 60 times | 22.84 µs/decision | 0.551 µs/decision | 41.5× |
| Nine rational contacts, means at the contact and $\pm10^{-25}$, repeated 120 times | 106.63 µs/decision | 2.052 µs/decision | 52.0× |
| Eight 64-point ring grids, complete CLI output | 26.65 ms | 14.94 ms | 1.78× |

Both builds returned 192 equilibrium fits and 320 positive floors per ring
pass, and 18 equilibrium fits and nine positive floors per contact pass.
The 512 serialized factory rows and all other grid response fields were
byte-identical. SHA-256 of the concatenated eight responses was
`e1578b0df8a30a22963d3eaafba942b69f6c1284d86414b9ed32524e154eecc9`.
These are local measurements on the two stated workloads.

A separate differential run called the starting revision's rational
classifier and the integer implementation on 54,753 inputs. All four
verdicts agreed point by point. The small grid used $i/24,j/24,k/18$ with
$-1\le i,j\le25$ and $0\le k\le72$. A further 1,536 cases scaled the
numerators and denominators of four triples close to the signed 128-bit
limit and shifted their numerators by $0$ or $\pm1$. The counts were 4,603
equilibrium fits, 54 unattained zero infima, 11,758 positive-floor decisions
and 38,338 invalid inputs.

To compare releases, build the same bench source on both revisions. The
old checkout needs the bench file and the three-line `[[bench]]` stanza
from `crates/physis-theory/Cargo.toml`. Build in separate target directories
and pass the executable paths printed by Cargo to the comparison script.

```sh
cargo build --release --locked -p physis
cargo bench --locked -p physis-theory --bench excursion_equilibrium --no-run
python3 docs/research/excursion-entropy/compare_factory.py \
  /path/to/base/physis /path/to/new/physis /path/to/base/bench /path/to/new/bench
```

[Benchmark corpus](../../../crates/physis-theory/benches/excursion_equilibrium.rs) · [Paired runner](compare_factory.py)

## Algebra and verification scope

Lean 4.34.0-rc2 checks ten denominator-cleared polynomial identities,
including the edge expansion for the one-jump return probability and the
integer rescaling of the decision cubic. Standalone nanoda 0.4.16 checked
2,389 declarations in the export. The analytic divergence estimates,
attainment, jump-process reduction and divergence along the family have
written mathematical proofs. The lab's model verdicts retain executed
derivation assurance. [Lean source](Entropy.lean)

```sh
formal_dir=$(mktemp -d)
cp docs/research/excursion-entropy/Entropy.lean "$formal_dir/"
(
  cd "$formal_dir"
  lean -DwarningAsError=true -o Entropy.olean Entropy.lean
  LEAN_PATH=. "$LEAN4EXPORT" Entropy -- \
    ExcursionEntropy.layer_deficit ExcursionEntropy.reciprocal_margin \
    ExcursionEntropy.fixed_visits ExcursionEntropy.reversible_mean \
    ExcursionEntropy.clock_product ExcursionEntropy.one_jump_shift \
    ExcursionEntropy.boundary_budget ExcursionEntropy.rational_constant \
    ExcursionEntropy.contact_gap ExcursionEntropy.homogeneous_decision > entropy.ndjson
  cat > check.json <<'JSON'
{"export_file_path":"entropy.ndjson","permitted_axioms":["propext","Quot.sound","Classical.choice"],"unpermitted_axiom_hard_error":true,"print_success_message":true,"print_axioms":false,"nat_extension":true,"string_extension":true,"num_threads":1}
JSON
  nanoda_bin check.json
)
```

The available legacy Forge binary separately replayed eight algebraic
identities and two exact counterexamples. Its SHA-256 was
`fe12e26fe263c7cd7bd321a51fe99e21994538361e87ff92584be4a762a261bf`.
Its source is absent from this checkout. The native factory, Rust tests
and Lean identities above can be rebuilt from the repository.
[Forge jobs](forge-replay.jsonl) · [Figure source](plot_clock.py)

```sh
physis-forge batch docs/research/excursion-entropy/forge-replay.jsonl --workers 8 --no-cache --compact
```

The sharp positive entropy infimum at general interior triples, and a sharp
bound when a measured one-jump probability is small and positive, remain open.
