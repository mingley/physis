# The immediate-return fraction and mean activity give a sharp entropy floor

A previous five-state counterexample defeated the minimum inferred by fitting
two short-excursion counts to a triangle. Most departures entered a
slow-returning reversible branch, while a rare driven branch changed the
short counts. The triangle substantially underestimated the mean. Measuring
the mean supplies a sharp lower bound at **any finite hidden state count**.
Moreover, each finite rate above the minimum is attainable. [Counterexample](../excursion-counts/HIDDEN-ORDER.md#fitting-three-states-can-overestimate-dissipation-by-an-unbounded-factor)

## What the counter measures

Consider a finite, irreducible, stationary, time-homogeneous continuous-time
Markov chain. State 0 is individually visible. All other states have the
same label. Every physical jump is detected, edges have reciprocal support,
and each ordered pair has one effective rate. An excursion starts on
departure from 0 and ends at the next return. Sample complete excursions by
departure, with $N$ their number of hidden-to-hidden jumps. Measure

$$
q=P(N=0),\qquad n=E[N],\qquad \nu=\text{excursions per unit time}.
$$

A counter only needs to flag whether each excursion had an internal jump
and record the global jump activity $\mathcal A$. Since every excursion
uses two boundary jumps, $n=\mathcal A/\nu-2$. No hidden-state label,
dwell-time fit or upper bound on the number of states is required.

Write $S=\dot S_{\rm prod}/(k_B\nu)$ for entropy per excursion using
natural logarithms. The nontrivial data domain is $0<q<1$, $n>1-q$.
Indeed $q>0$ because an entrance edge has a reverse. If an internal jump
can occur, following it immediately by its reverse has positive probability,
so $E[N]>P(N\ge1)$. The constructions below realize the entire domain.
The exceptional case $q=1$ has $n=0$ and only zero entropy production.
Its connected support is a star.

## Exact feasible interval

Let $K=1+n$ and $d=\sqrt{\max(0,1-Kq)}$. The compatible values of $S$
over all finite reciprocal networks fill $[L(q,n),\infty)$, including the
lower endpoint and every finite value above it, where

$$
\boxed{L(q,n)=
d\left[2\log\frac{1+d}{1-d}+\log\frac{n+d}{n-d}\right].}
$$

On the nontrivial domain $d<1$ and $d<n$. For a positive deficit the latter
inequality follows from $n^2-d^2=K(n-1+q)>0$. Thus the threshold
**$q(1+n)<1$** is necessary and sufficient for these observations to
force positive dissipation. Multiply $L$ by $k_B\nu$ to obtain a rate.

The same data put a sharp lower bound on the size of the hidden set.

| Observed regime on the nontrivial domain | Minimum entropy per excursion | Fewest hidden states |
|---|---|---|
| $qn<1-q$ | $L(q,n)>0$ | 2 |
| $1-q\le qn<2(1-q)$ | 0 | 2 |
| $qn=2(1-q)$ | 0 | 2, necessarily arranged in a path with 0 at one end |
| $qn>2(1-q)$ | 0 | 3 |

### Why extra hidden states cannot lower the floor

Use stationary edge traversals per excursion, $f_{ij}=\pi_i Q_{ij}/\nu$
for $i\ne j$, with diagonal zero. These fluxes balance at every vertex.
For hidden state $i$ let $e_i=f_{0i}$, $b_i=f_{i0}$ and
$h_i=\sum_j f_{ij}$. Each $h_i>0$ is the expected number of visits there
per excursion. The observed data give four equalities,

$$
\sum_i e_i=\sum_i b_i=1,\quad \sum_i h_i=K,\quad
q=\sum_i \frac{e_i b_i}{h_i}.
$$

The last holds because $e_i$ is the entrance probability and $b_i/h_i$ is
the probability of returning on the next jump. Hidden edge traffic has
row totals $h_i-b_i$, column totals $h_i-e_i$ and total $n$.

For two nonnegative flux vectors $a,b$ of common mass $s>0$ and common
support, let $J(a,b)=\sum_i(a_i-b_i)\log(a_i/b_i)$ and
$C=\sum_i\sqrt{a_i b_i}$. Double-zero coordinates contribute zero.
With $z=\sqrt{s^2-C^2}$,

$$
J(a,b)\ \ge\ 2z\log\frac{s+z}{s-z}.
$$

For a direct proof, use weights $(a_i+b_i)/(2s)$ and arguments
$c_i=2\sqrt{a_i b_i}/(a_i+b_i)$. Define $g(c)=2u\operatorname{atanh}u$
with $u=\sqrt{1-c^2}$. Then $J/(2s)$ is the weighted average of
$g(c_i)$ and the mean of $c_i$ is $C/s$. The function is decreasing
and strictly convex on $0<c<1$, continuous at 1. In fact
$g''(c)=2[u/(1-u^2)-\operatorname{atanh}u]/u^3>0$ by integration of
$1/(1-u^2)$. Jensen's inequality gives the displayed bound. Its right
side increases with $z$.

Apply this twice. The boundary contribution to $S$ is $J(e,b)$.
Cauchy–Schwarz with the vector $(\sqrt{h_i})_i$ gives
$(\sum_i\sqrt{e_i b_i})^2\le Kq$. For the internal edges let
$F=(f_{ij})_{i,j\ne0}$. Their contribution is $J(F,F^{\mathsf T})/2$.
Summing destinations in each row and applying the log-sum inequality in
both directions bounds this below by $J(h-b,h-e)/2$. Another application
of Cauchy–Schwarz gives

$$
\left(\sum_i\sqrt{(h_i-b_i)(h_i-e_i)}\right)^2
\le K\sum_i\frac{(h_i-b_i)(h_i-e_i)}{h_i}
=K(K-2+q).
$$

The two estimates, with masses $1$ and $n$, give the same lower bound
$d$ on $z$, since $n^2-K(K-2+q)=1-Kq$. Their sum is $L(q,n)$.

### Networks attaining the bound

For $Kq<1$, use one triangle. The following expected traversals per
excursion specify every edge.

| Edge pair | Forward flux | Reverse flux |
|---|---:|---:|
| $0\leftrightarrow1$ | $(1+d)/2$ | $(1-d)/2$ |
| $1\leftrightarrow2$ | $(n+d)/2$ | $(n-d)/2$ |
| $2\leftrightarrow0$ | $(1+d)/2$ | $(1-d)/2$ |

They balance, both hidden visit counts equal $K/2$, and the immediate
return probability is $(1-d^2)/K=q$. Summing the three edge contributions
gives exactly $L$. To convert any such balanced flux matrix into a chain at
the measured frequency, normalize each row to jump probabilities and give
every state exit rate $\nu(1+K)$. The stationary probabilities are the row
sums divided by $1+K$.

For $Kq\ge1$ put $a=(1-q)/n$, so $0<a\le q<1$. On leaving 0, choose
a symmetric pair of hidden states with probability $w=(1-q)/(1-a)$ and
choose an immediately returning singleton otherwise. Enter the pair
uniformly. At either member return to 0 with probability $a$, otherwise
jump to its partner. The pair has geometric $N$ of mean $(1-a)/a$.
The mixture has immediate-return fraction $1-w+wa=q$ and mean
$w(1-a)/a=n$. Symmetry makes every stationary edge flux reciprocal in
magnitude. At $w=1$ omit the unused singleton. This gives equilibrium
throughout the zero-floor region. The construction at the end of this note
gives all larger rates in both regions.

### Equality determines the hidden jump probabilities up to a partition

Suppose $Kq<1$ and a network attains the minimum. Both Cauchy inequalities
must be equalities since the divergence bound increases strictly with $z>0$.
Set $u_i=e_i/h_i$ and $v_i=b_i/h_i$, the respective reversed and forward
boundary jump probabilities. Equality gives $u_i v_i=q/K$ and
$(1-u_i)(1-v_i)=(K-2+q)/K$ at every hidden state. Hence $u_i+v_i=2/K$
and the unordered pair is exactly $\{(1+d)/K,(1-d)/K\}$.

Partition the states by the sign of $u_i-v_i$. Equality in the row
log-sum step requires $f_{ij}/f_{ji}=(1-v_i)/(1-u_i)$ on every internal
edge leaving $i$. This ratio is $(n+d)/(n-d)$ in one class and its
reciprocal in the other. Reversing an edge inverts the ratio, so every
internal edge joins opposite classes. Each class has a common probability
of exiting to 0 and a common probability of continuing to the other class.
Also $\sum_i e_i=\sum_i b_i$ forces total visit mass $K/2$ in each class.
The embedded chain on 0 and these two classes is exactly the minimizing
triangle above. Conversely these row probabilities **and** the stated
edge-flux ratio ensure equality. The holding rates inside a class can
still differ. This classifies positive-floor equality at jump epochs.

The factory starts from two triangles joined at 0. On branch $j$ write
$n_j$ for its mean and $d_j$ for its signed winding per excursion. On its
two hidden rows the probabilities $u_i,v_i$ are $(1\pm d_j)/(1+n_j)$ in
opposite orders. Their sum fixes $n_j$ and their product fixes $|d_j|$.
Consequently, if the first branch is driven and both branches have positive
weight, the combined network attains the floor exactly when
$n_2=n_1$ and $|d_2|=|d_1|$. Either winding direction works, for **every**
mixture weight strictly between zero and one. This extends the two repairs
found in the finite knob grid to the whole family of these triangle fluxes.

## When the previously falsified triangle inference becomes safe

Let $m=P(N\ge1)=1-q$ and $r=P(N\ge2)$, with $0<r<m<1$. A triangle
predicts $n_\triangle=(m+r)/(1-r)$ via its alternating geometric count
tails. The triangle result on those two tails is precisely $L(q,n_\triangle)$.
Indeed the minimizer above has those tails in the positive regime since
its two-hop tail is $(n_\triangle^2-d^2)/(1+n_\triangle)^2=r$.
In the zero regime the earlier triangle construction at these $m,r$ is
reversible. [Three-state derivation](../excursion-counts/RESULT.md)

* If the measured $n=n_\triangle$, that triangle minimum is **sharp among
  all finite networks** matching all three statistics. A triangle realizes
  the joint data, and every network is subject to the bound just proved.
* The weaker condition $n\le n_\triangle$ is sufficient to use the old
  value as a lower bound. At fixed $q$, increasing $n$ decreases $d$ down
  to zero and decreases $d/n$. Each positive term in $L$ decreases.
  The same monotonicity in $q$ means that valid simultaneous upper limits
  on $q,n$ yield a conservative bound whenever their product test
  $q_{\rm upper}(1+n_{\rm upper})<1$ passes.
* In the previous five-state example at mixture weight $1/100$, the actual
  mean is $9899.02$ hidden jumps. A triangle fit to the first two tails
  predicts about $238.84$. The measured $(q,n)$ allow a zero floor, while the two tails rule
  out equilibrium. A minimum over these three observables away from the
  consistency surface is a separate optimization problem.

**An example missed by the first two counts.** On four states arranged
in a ring use jump weights 5 clockwise, 2 anticlockwise and 1 to the
opposite site, normalized by 8 in each row. All exits have rate one.
Uniform stationarity gives $\nu=1/4$ and $n=2$. Direct propagation gives
$q=21/64$, $m=43/64$ and $r=257/512$. Since $m^2<r$, the first two
tails and frequency have an equilibrium three-state fit. Measuring the
mean gives $Kq=63/64$ and $d=1/8$. The all-order floor from $(q,n)$ is
$[2\log(9/7)+\log(17/15)]/8\simeq0.078474$ $k_B$ per excursion, or
$0.0196185$ $k_B$ per time unit. The given ring dissipates more than
this floor.

## An exact test for needing a third hidden state

The same observations determine the smallest possible number of hidden
states. On the nontrivial domain $0<q<1$, $n>1-q$, put $m=1-q$. Exactly
two hidden states suffice if and only if $qn\le 2m$. For $qn>2m$ at least
three hidden states are needed, and the reversible four-state construction
above attains that minimum. Thus some records certify extra hidden states
even though they permit equilibrium.

For two hidden states let $x,y$ be their respective probabilities of jumping
to each other and $p$ the entrance probability of the first. Alternation gives
$r=xy\le px+(1-p)y=m$ and $n=(m+r)/(1-r)$. It follows that

$$
(2m-qn)(1-r)=(1+m)(m-r)\ge0.
$$

Conversely, for $qn<2m$ the implied $r=(n-m)/(1+n)$ lies strictly between
zero and $m$. The explicit positive triangle for these two tails realizes
the required mean. A single hidden state has no internal jumps. At
$qn=2m$, the path $0\leftrightarrow1\leftrightarrow2$ realizes the data
by returning from 1 with probability $q$ and from 2 to 1 with probability
one. In fact every two-hidden-state realization at this boundary is that
path up to relabeling and exit rates. Equality forces $r=m$, so a one-hop
excursion has probability zero. If both hidden states were adjacent to 0,
their internal edge would give a positive one-hop excursion. The path has
detailed balance. For example $(q,n)=(1/2,2)$ gives the boundary, while
$(1/2,3)$ requires the extra state. [Positive triangle construction](../excursion-counts/RESULT.md#exact-range-from-two-counts)

## Why even a complete jump clock has no finite upper bound

Start from *any* chain in the model with $n>0$. Keep the singleton 0
and replace each hidden state $i$ by $(i,k)$ with $k\in\mathbb Z/3$.
Split $Q_{0i}$ uniformly among its three copies and use $Q_{i0}$ at each
copy to return to 0. Split each internal rate $Q_{ij}$ between
$(i,k)\to(j,k+1)$ at rate $p Q_{ij}$ and $(i,k)\to(j,k-1)$ at rate
$(1-p)Q_{ij}$, for $1/2\le p<1$. The two destinations are distinct.

Projection back to the original states preserves every outgoing rate to
each destination block, every holding-time law and every jump. This
preserves the complete projected state trajectory **including every
timestamp**. The stationary mass of $(i,k)$ is $\pi_i/3$ by flux balance,
and its exit rate is the original exit rate. All these individual masses
and exit rates stay fixed as $p$ varies on the same lifted graph.
Irreducibility follows by paths to and from 0. Every used edge has its
reverse.

Boundary flux ratios are unchanged. For a directed internal base flux $f$
with reverse $b$, its lifted contributions are
$f p\log[(f/b)p/(1-p)] + f(1-p)\log[(f/b)(1-p)/p]$ after summing the
three copies. The base entropy per excursion therefore increases by

$$
n(2p-1)\log\frac{p}{1-p}.
$$

This runs continuously from zero to infinity, with finite positive rates
at every $p<1$. Applied to either minimizer, it realizes the entire claimed
interval on a fixed finite lifted topology, even at the same complete
projected jump-clock law as that minimizer. The diverging limit has a
vanishing reverse rate.

## Scope and prior work

Kapustin, Ghosal and Bisker explicitly studied an observer who detects
transitions between unresolved hidden states. They grouped consecutive hidden
visits by their count and used these records in trajectory-based estimators.
The observation protocol itself has clear precedent. [Kapustin et al., Fig. 1 and accompanying text](https://arxiv.org/html/2212.13487)

Skinner and Dunkel minimized dissipation using observed transitions and
successive macro-transitions. Nitzan, Ghosal and Bisker developed bounds
conditioned on steady flows and waiting-time statistics. Ehrich studied
minimum entropy via hidden-network fits to step-resolved observations.
Maier, Seifert and van der Meer inferred constraints on hidden paths and
network topology using observable transitions and their waiting times.
Petkov studied state-count-dependent reduction for two individually visible
states with stationary flux and excursion duration data. The closed form
here minimizes entropy for a single visible state at fixed immediate-return
fraction and mean hidden jump count. Historical priority and experimental
applicability remain open. [Skinner and Dunkel](https://doi.org/10.1073/pnas.2024300118) ·
[Nitzan et al.](https://doi.org/10.1103/PhysRevResearch.5.043251) ·
[Ehrich](https://arxiv.org/abs/2105.08803) · [Maier et al.](https://doi.org/10.1103/p4k1-1dvb) · [Petkov, preprint](https://hexagonmath.org/2611.00657v1)

The finite flux algebra, the analytic inequalities and the numerical
search have different coverage. The campaign records which checks ran
and how to reproduce them. [Factory run and replay](CAMPAIGN.md)
