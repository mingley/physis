# How much entropy can an unlabelled jump counter recover?

During each visit to a hidden set, count its internal jumps. Let $m$ be the
fraction of visits with at least one and $r$ the fraction with at least two.
At stationarity, **$m^2>r$ rules out detailed balance**, even if the number
of hidden states is unknown. With exactly two hidden states the least
compatible entropy production has a closed form, and every finite rate above
that minimum is possible. A timestamp can remove the ambiguity under a
precise condition.

## Measurement and model

Consider a stationary, time-homogeneous, three-state continuous-time Markov
chain with six strictly positive, finite rates, one channel per ordered pair.
State 0 has the visible label “on”. States 1 and 2 share the label “off”.
A counter detects every physical jump, including those that preserve the
off label. It supplies no state identities. All probabilities below refer
to exact laws over *complete excursions*, sampled at departure from 0.
Length-biased sampling at an arbitrary off time gives a different law.

An excursion starts when the chain leaves 0 and ends at its next return.
Write $N$ for its number of hidden-to-hidden jumps and $\nu$ for the
stationary number of excursions per unit time. Set $m=P(N\ge1)$ and
$r=P(N\ge2)$, so $0<r<m<1$. Write $\sigma$ for the stationary entropy
production rate in units of $k_B$ per model time unit, with natural logarithms.
The exit rates at 0, 1, 2 are $\lambda,k_1,k_2$. The embedded jump chain
has parameters $p=P(0\to1)$, $x=P(1\to2)$ and $y=P(2\to1)$.
Multiplying each row's probabilities by its exit rate recovers its two rates.

| Available observation | Compatible entropy production rates |
|---|---|
| $\nu$ and whether $N=0$, $N=1$, or $N\ge2$ | Exactly $[\sigma_{\min},\infty)$ in the three-state model |
| All timestamps and on/off labels, $k_1=k_2$ | The same interval, even fixing all three exit rates |
| The same count data and three timed survival probabilities for $N=0$, $k_1\ne k_2$ | A single rate |

Here and below the interval contains finite rates only. Resolving every jump
and knowing both individual hidden occupancies are different experiments.
Hidden occupancies are free in this construction.

## Exact range from two counts

Put $s=m-r$ and $d_0=\sqrt{\max(0,(1-r)(m^2-r))}$. Then

$$
\sigma_{\min}=\frac{\nu\,d_0}{1-r}
                 \log\frac{s+d_0}{s-d_0}.
$$

To see why this works, split the zero-hop paths according to their hidden
state, and the one-hop paths according to their orientation. Their
unconditional probabilities per excursion are

$$
w=p(1-x),\quad v=(1-p)(1-y),\qquad
F=px(1-y),\quad B=(1-p)y(1-x).
$$

An extra pair of internal jumps returns to the same hidden state with
probability $r=xy$. Thus $P(N\ge2j)=r^j$ and
$P(N\ge2j+1)=m r^j$, with $m=px+(1-p)y$. In particular
$w+v=1-m$, $F+B=s$, and $FB=rwv$.
Even excursions have zero net winding. Each odd excursion contributes a
forward or backward cycle, with probabilities $F r^j$ and $B r^j$ for
$N=2j+1$. The signed current per unit time is $\nu(F-B)/(1-r)$.
The cycle affinity is $\log(F/B)$ because the three exit-rate factors
cancel from the ratio of forward and reverse rate products. The stationary
flux formula sums to current times cycle affinity on a triangle. Consequently,
with $d=|F-B|$,

$$
\sigma(d)=\frac{\nu\,d}{1-r}\log\frac{s+d}{s-d},\qquad
d^2-(1-r)(m^2-r)=r(w-v)^2.
$$

**Attainment.** Choose any $d_0\le d<s$. Set $F=(s+d)/2$, $B=(s-d)/2$,
and take $w,v$ to be the two roots of
$z^2-(1-m)z+FB/r=0$. Their discriminant is
$[d^2-(1-r)(m^2-r)]/r\ge0$. Their sum and product are positive, hence
both roots are positive. The lower endpoint satisfies $d_0<s$ because
$s^2-(1-r)(m^2-r)=r(1-m)^2>0$ and $s>0$.
One explicit embedded jump chain is

$$
p=\frac{w+F}{1-r},\qquad
x=\frac{F+rw}{w+F},\qquad y=\frac{B+rv}{v+B}.
$$

Since $1-p=(v+B)/(1-r)$, all three parameters lie strictly between 0 and 1.
Substitution gives $xy=r$, $p(1-x)=w$, $(1-p)(1-y)=v$ and the prescribed
one-hop probabilities. To realize any specified $\nu>0$, take equal exit
rates at all states and rescale time. The mean number of jumps per
return from 0 is $1+(1+m)/(1-r)$. Finally $\sigma(d)$ increases
continuously from its minimum to infinity as $d$ tends to $s$.
This proves both endpoints, including the case $m^2=r$.

**Equal exit rates.** Fix $\lambda$ and $k_1=k_2=k$. Conditional on $N$,
the $N+1$ off holding times are independent exponentials of rate $k$.
The following on holding time is independently exponential of rate
$\lambda$. The count law above and these holding times specify the complete
timestamped record between returns. Regeneration then specifies the
stationary record, including the size bias of the initial incomplete cycle.
All members of the construction now have the same frequency
$\nu=[1/\lambda+(1+m)/(k(1-r))]^{-1}$. Even a clock that sees every jump
cannot shrink the interval in this case. The stationary observed record is
also invariant under time reversal. Each off block has an exchangeable list
of durations given its length, and successive off blocks and on holds are
independent.

## Two exact examples

Both use $(\lambda,k_1,k_2)=(1,2,2)$. Each row, with the embedded
probabilities defined above, specifies all six transition rates.

| $p$ | $x$ | $y$ | $m$ | $r$ | $\nu$ | $\sigma$ |
|---:|---:|---:|---:|---:|---:|---:|
| $1/4$ | $1/2$ | $1/4$ | $5/16$ | $1/8$ | $4/7$ | $0$ |
| $1/4$ | $3/4$ | $1/6$ | $5/16$ | $1/8$ | $4/7$ | $(4/49)\log 5$ |
| $3/5$ | $1/2$ | $1/4$ | $2/5$ | $1/8$ | $5/9$ | $(1/9)\log(9/2)$ |

The first two have the same complete clock record. Here $m^2<r$, so that
record also admits arbitrarily large finite dissipation. In the third row
$w=v=3/10$, $F=9/40$ and $B=1/20$. This row attains the strictly positive
minimum, about $0.167120$, for its own time-reversal-invariant record.

## Recovering entropy when the hidden exit rates differ

Retain the off duration $T$ for excursions with $N=0$. At a fixed
$\tau>0$, measure the three additional probabilities
$h_j=P(N=0,T>j\tau)$ for $j=1,2,3$ and put $h_0=1-m$.
Their exact values satisfy $h_j=w a^j+v b^j$, where $a=e^{-k_1\tau}$
and $b=e^{-k_2\tau}$. The observable determinant

$$
D=h_0 h_2-h_1^2=wv(a-b)^2
$$

vanishes exactly when the exit rates coincide. For $D>0$, form

$$
A=\frac{h_0h_3-h_1h_2}{D}=a+b,\qquad
C=\frac{h_1h_3-h_2^2}{D}=ab,\qquad wv=\frac{D}{A^2-4C}.
$$

Now $d=\sqrt{s^2-4r wv}$ and the formula for $\sigma(d)$ gives the
entropy rate. This uses neither a hidden-state label nor a full-order
assumption on the binary signal. All inputs are population quantities.
The denominator $D$ tends to zero quadratically in $a-b$.

**Finite records near equal rates.** Fix a count pair with $m^2\le r$,
and compare a reversible member to a driven member at the same count
pair. Give both the same on rate and hidden rates $k,k(1+\epsilon)$ with
$\epsilon>0$. Start at a regeneration point. Couple the counts exactly
and sample the hidden routes conditional on their common count.
At each corresponding hold, exponentials of the two possible rates
can be coupled by racing a rate-$k$ clock against an independent
rate-$k\epsilon$ clock. Their disagreement probability is at most
$\epsilon/(1+\epsilon)$. There are on average $(1+m)/(1-r)$ hidden holds
per excursion. A union bound for $M$ complete excursions gives total
variation distance at most

$$
\min\left(1,\ M\,\frac{1+m}{1-r}\frac{\epsilon}{1+\epsilon}\right).
$$

Changing exit rates preserves all embedded cycle odds and winding per
excursion. The frequency is at least $[1/\lambda+(1+m)/(k(1-r))]^{-1}$.
Hence for any finite $M$, positive error tolerance and finite target rate,
one can choose distinct hidden exits so that equilibrium and a model above
the target satisfy this tolerance in total variation. A population
identification result alone supplies no uniform finite-record resolution.

## State count, prior work and replay

For an unknown finite number of hidden states, the same two probabilities
still imply a positive, explicit lower bound when $m^2>r$. For the third
row it gives $0.010538$ in the same units. A five-state construction shows
that applying the three-state minimum to these two count probabilities can
overestimate dissipation by an unbounded factor. Measuring the fourth count
tail detects that construction. Sharpness of the general bound across hidden
state counts is open. [Proof and five-state counterexample](HIDDEN-ORDER.md)

Adding the mean number of hidden jumps gives an exact condition under which
the triangle minimum holds at any finite order. The immediate-return fraction
and mean alone also have a sharp minimum of their own. [Mean jump activity and hidden entropy](../excursion-activity/RESULT.md)

Ehrich optimizes over hidden jump networks using return probabilities
indexed by step number, including a two-visible-state unicyclic recovery
result. Skinner and Dunkel bound dissipation using waiting-time statistics.
Wagner and Timmer analyze how equal or nearly equal dwell times affect
detailed-balance testing in aggregated ion-channel models. The specific
claims here concern a singleton on state, unlabelled counts of every
off-to-off jump, and the exact feasible entropy interval and timing criterion
for that measurement. Historical priority and experimental performance
are open. [Ehrich](https://arxiv.org/abs/2105.08803) ·
[Skinner and Dunkel](https://doi.org/10.1103/PhysRevLett.127.198101) ·
[Wagner and Timmer](https://doi.org/10.1016/S0006-3495(00)76529-5)

Replay from the repository root with the project's Lean 4.34.0-rc2 and Rust
toolchains.

```sh
lean -DwarningAsError=true docs/research/excursion-counts/Excursions.lean
cargo test --locked -p physis-theory --test excursion_counts
```

Lean checks nine algebraic lemmas for the path weights, inverse map,
floor, timed moments and five-state mixture. Standalone nanoda 0.4.16
checked their joint export, including 2,413 declarations with dependencies.
The Rust replay compares current and cycle odds
against directed spanning-tree stationary probabilities on all $3^6=729$
generators with rates in $\{1,2,3\}$, checks the explicit witnesses and
propagates the hidden jump chains for four- and five-state controls.
The probabilistic, logarithmic and finite-record arguments are analytic
proofs in these notes. No catalog receipt or experimental dataset is
attached. [Lean source](Excursions.lean) ·
[Rust replay](../../../crates/physis-theory/tests/excursion_counts.rs)

To repeat the independent typecheck, run the following in a fresh temporary
directory containing a copy of `Excursions.lean`, with the CI-pinned
`lean4export` and the workspace-locked `nanoda_bin` on the path.

```sh
lean -DwarningAsError=true -o Excursions.olean Excursions.lean
LEAN_PATH=. lean4export Excursions -- \
  Excursions.zero_sum Excursions.one_sum Excursions.path_product \
  Excursions.floor_slack Excursions.inverse_two_hops Excursions.inverse_first_path \
  Excursions.moment_determinants Excursions.mixture_deficit Excursions.mixture_fourth > identities.ndjson
cat > check.json <<'EOF'
{"export_file_path":"identities.ndjson","permitted_axioms":["propext","Quot.sound","Classical.choice"],"unpermitted_axiom_hard_error":true,"print_success_message":true,"print_axioms":false,"nat_extension":true,"string_extension":true,"num_threads":1}
EOF
nanoda_bin check.json
```
