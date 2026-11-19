# Three statistics can force dissipation even when every pair admits equilibrium

For a stationary reciprocal finite jump network with one visible state,
let $S$ be the entropy produced per excursion in units of $k_B$. Measure
$m=P(N\ge1)$, $r=P(N\ge2)$ and $n=E[N]$. Counting every physical jump is
essential. On the four-state ring with clockwise, anticlockwise and opposite
weights $4,2,1$, all three two-statistic projections have explicit equilibrium
realizations. **Every network matching the three statistics together has
$S\ge 700928/466948881\simeq0.00150108$.** All realizations may also match
the excursion frequency by rescaling time. [Model and sharp two-statistic result](RESULT.md)

## A joint equilibrium constraint

At equilibrium write $T$ for the hidden-to-hidden block of the embedded jump
matrix and $h_i$ for the expected visits to hidden state $i$ per excursion.
Detailed balance makes $T$ self-adjoint for the inner product weighted by
$h_i$. The vector $v=(I-T)\mathbf1$ gives the chance of returning to 0 on
the next jump. The entrance probability of $i$ equals $h_i v_i$ by detailed
balance. The finite absorbing hidden chain has spectral radius strictly
less than one because every state can reach 0. For $\ell\ge0$ this gives

$$
P(N\ge\ell)=\langle v,T^\ell\mathbf1\rangle_h
=\sum_t (1-t)\,\|\Pi_t\mathbf1\|_h^2\,t^\ell,
$$

where $\Pi_t$ is the orthogonal projection for eigenvalue $t$. The weights
are nonnegative and sum to one by the $\ell=0$ identity. In this probability
measure on $(-1,1)$, $m=E[t]$, $r=E[t^2]$ and $1+n=E[(1-t)^{-1}]$ by
summing the geometric series. [Spectral representation in the earlier two-tail proof](../excursion-counts/HIDDEN-ORDER.md#what-reversibility-requires)

Two applications of Cauchy–Schwarz, the first
to $t$ and $1$, the second with the positive weight $1+t$, give

$$
r\ge m^2,\qquad
(1+m)^2\le E[(1+t)/(1-t)]\,E[(1+t)(1-t)]=(1+2n)(1-r).
$$

Equality in the second inequality requires a single spectral value on the
support of these weights. Then the tails are geometric, $r=m^2$ and
$n=m/(1-m)$. Define $G=(1+m)^2-(1+2n)(1-r)$. A positive $G$ excludes
equilibrium at every finite hidden order. This is a sufficient test. The
complete minimum at arbitrary triples remains open.

## Converting the violation into an entropy bound

For any allowed directed network, write $f_{ij}$ for its stationary edge
traversals per excursion, and replace them by $\bar f_{ij}=(f_{ij}+f_{ji})/2$.
This reversible comparison network has the same visits $h_i$, departures
from 0 and mean $n$. Normalize each row to obtain its jump probabilities.
Let $P,\bar P$ be the laws of complete state sequences during one excursion.
The expected log likelihood ratio sums over edge traversals. Equal row sums,
finite mean and common support give

$$
D(P\Vert\bar P)=\sum_{ij} f_{ij}\log(f_{ij}/\bar f_{ij})\le S/4.
$$

For an unordered edge of traffic $a,b$, set $z=|a-b|/(a+b)$. Its
contributions to $D$ and $S$, divided by $a+b$, are respectively
$\int_0^z\operatorname{atanh}(x)\,dx$ and $2z\operatorname{atanh}(z)$.
Convexity bounds the integral by $z\operatorname{atanh}(z)/2$.
[Earlier path comparison](../excursion-counts/HIDDEN-ORDER.md#how-far-the-counts-are-from-reversibility)

Pinsker's inequality therefore bounds the absolute change of each tail
probability by $\sqrt{S/8}$. The difference of the two
squared terms $(1+m)^2$ and $(1+\bar m)^2$ is at most $4|m-\bar m|$ in
absolute value. Since $\bar G\le0$, the joint measurements imply

$$
\boxed{S\ \ge\ 8\left(\frac{\max(0,G)}{5+2n}\right)^2.}
$$

The right side need not be the minimum. Its strict positivity already
separates a joint observation from every near-equilibrium fit.

## What the second tail and the mean can determine on their own

For an equilibrium network and $k\ge1$, the same spectral measure gives
$P(N\ge2k)=E[(t^2)^k]\ge r^k$. The preceding odd tail is at least as large.
Summing both tails over $k$ gives $n\ge2r/(1-r)$. Conversely, for
$r>0$ and $r\le n/(n+2)$ put $a=1-2r/n$ and $w=r/a\le1$. With probability
$w$ enter a reversible branch $0\leftrightarrow1\leftrightarrow2$. At 1
make a round trip via 2 with probability $a$ or return to 0. The other branch
is an immediately returning singleton, omitted when $w=1$. Then $r=wa$
and $n=2wa/(1-a)$. Thus **$r\le n/(n+2)$ is necessary and sufficient for
an equilibrium fit to $(r,n)$** on nontrivial feasible data. The same
reversible comparison and Pinsker give $S\ge8[\max(0,r-n/(n+2))]^2$.
For $r>n/(n+2)$, retaining the binary event $N\ge2$ gives the stronger
$S\ge4D_{\rm KL}(\mathrm{Bern}(r)\Vert\mathrm{Bern}(n/(n+2)))$ by data
processing and monotonicity in the comparison probability below $r$.

## Exact four-state witnesses

The driven ring has $m=32/49$, $r=164/343$ and $n=2$. Substitution gives
$G=296/2401$. The lower bound is per excursion. Unit exit rates give
$\nu=1/4$ and at least $0.000375270\,k_B$ per time unit. Here are three
reversible controls. Each table entry is symmetric stationary traffic in
common arbitrary units. Divide each row by its sum to get a jump chain.

| Data matched | $0\leftrightarrow1$ | $0\leftrightarrow2$ | $0\leftrightarrow3$ | $2\leftrightarrow3$ |
|---|---:|---:|---:|---:|
| $m,r$ | 465 | 1920 | 1920 | 5248 |
| $m,n$ | 1 | 16 | 16 | 33 |
| $r,n$ | 15 | 164 | 0 | 179 |

The independent ring sweep found 28 settings in which both the $(m,r)$ and
$(m,n)$ tests allow equilibrium but $G>0$. Of these, 22 also admit an
equilibrium fit to $(r,n)$. The other six already force dissipation through
$(r,n)$. The counts refer to ordered integer weight triples from 1 through
8. [Rust replay](../../../crates/physis-theory/tests/excursion_activity.rs) · [Campaign](CAMPAIGN.md)
