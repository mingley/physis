# A linear lower bound from violations of equilibrium

Use the same stationary reciprocal network and population counts
$m=P(N\ge1)$, $r=P(N\ge2)$, $n=E[N]$. A reversible supporting plane
at $0\le a<1$ has coefficients

$$
D=1+a-a^2,\quad B=1+2a-a^2,\quad A=2D^2-B,\quad M=(1+a)^2(1-a^2)^2.
$$

Define its violation by $H=Am+Br-2a^6-Mn$. Every compatible network
satisfies the following bound in units of $k_B$ per excursion,

$$
\boxed{S\ge\frac{8[H]_+}{(1+\sqrt3)B}\ \ge\ \frac{32[H]_+}{11B}.}
$$

It holds at arbitrary finite hidden order and at any strength of driving.
The last expression gives a rational certificate for rational data and
$a$. The dependence on $H$ is sharp in order along the contact families
below. The optimal prefactor is open. [Equilibrium planes and their contacts](../excursion-equilibrium/RESULT.md#a-supporting-plane-that-a-finite-network-attains)

For the previously studied ring with jump weights 3, 2 and 1, use
$(m,r,n)=(23/36,11/24,2)$ and $a=9/11$. Then
$B=238/121$ and $H=547307/63776196$. The radical bound is approximately
$0.01277562$. The exact rational certificate is
$2189228/172485621>0.0126922$. The former quadratic bound at this same
plane gave about $0.00008458$. All three two-statistic comparisons still
admit equilibrium. [Previous bound](../excursion-equilibrium/BOUNDARY.md#when-a-failed-fit-does-guarantee-a-positive-floor)

The plane also propagates simultaneous uncertainty intervals. If the three
population inputs are within $e_m,e_r,e_n$ of their stated values, replace
$H$ by $H-|A|e_m-Be_r-Me_n$. At the same ring, absolute errors of at most
$1/1000$ in each statistic still certify
$177971224/21560702625>0.0082544$ per excursion. This statement uses supplied
error bounds. Estimating them from a particular data stream requires a
sampling model.

## Two exact changes under flux symmetrization

Use per-excursion flux $f_{ij}=\pi_iQ_{ij}/\nu$ with zero diagonal,
$h_i=\sum_jf_{ij}$, and $\bar f=(f+f^\top)/2$. This comparison is
reversible and preserves every visit count and $n$. All indices below are
hidden. Set

$$
s_i=\frac{f_{0i}+f_{i0}}{2h_i},\qquad j_i=\frac{f_{0i}-f_{i0}}{2h_i},
\qquad G_{ik}=\tfrac12(f_{ik}+f_{ki}),\quad T_{ik}=\tfrac12(f_{ik}-f_{ki}).
$$

Here $0\le s_i\le1$, $G$ is symmetric and $T$ is antisymmetric. Flux
balance gives $\sum_k G_{ik}=h_i(1-s_i)$ and $\sum_k T_{ik}=h_i j_i$.
Write $X=\sum_i h_i j_i^2$ and let bars on counts denote the reversible
comparison. Immediate returns give $1-m=\sum_i h_i(s_i^2-j_i^2)$, hence
$m-\bar m=X$. The one-jump probability is
$m-r=(s+j)^\top(G+T)(s-j)$. Expanding it and using $j^\top T{\bf1}=X$
gives the other exact identity

$$
r-\bar r=j^\top G j-2j^\top T(s-\tfrac12{\bf1}).
$$

Both changes are quadratic in the antisymmetric currents at fixed symmetric
traffic. This gives a stronger estimate near a reversible contact.

## Paying for both changes with edge entropy

Write
$Z=\sum_i h_i(1-s_i)j_i^2$,
$E=\sum_{s_i>0} h_i j_i^2/s_i$, and
$Y=\sum_{i<k:G_{ik}>0} T_{ik}^2/G_{ik}$. Sites with $s_i=0$ have $j_i=0$.
On an edge with symmetric flux $g>0$ and signed fraction $u$, the entropy
is $4g u\operatorname{atanh}u\ge4gu^2$. Summing boundary and hidden edges
gives $S\ge4(E+Y)$. Pointwise $s_i(2-s_i)\le1$, so $X+Z\le E$.

Because $G$ is nonnegative and symmetric, $2|j_i j_k|\le j_i^2+j_k^2$
gives $|j^\top G j|\le Z$. Also $|s_k-1/2|\le1/2$. Cauchy–Schwarz over
ordered internal pairs gives
$2|j^\top T(s-\tfrac12{\bf1})|\le\sqrt{2YZ}$.
The reversible comparison obeys the supporting plane. Using
$A=B-2a^3(2-a)\le B$, $B>0$, it follows that

$$
H\le B\big(X+Z+\sqrt{2YZ}\big)
  \le \tfrac{1+\sqrt3}{2}B(E+Y).
$$

For the last step the largest eigenvalue of the symmetric matrix with
diagonal $(1,0)$ and off-diagonal $1/\sqrt2$ is $(1+\sqrt3)/2$. Thus
$Z+\sqrt{2YZ}\le(1+\sqrt3)(Z+Y)/2$. Substitution of the edge entropy
estimate proves the claim. For the rational version the same factor is at
most $11/8$.

## Two tails suffice for another linear certificate

Even when the mean is unavailable, the same current estimates give

$$
S\ge \frac{4[m^2-r]_+}{\max\{2m,(1+\sqrt3)/2\}}
\ge \frac{4[m^2-r]_+}{\max\{2m,11/8\}}
\ge 2[m^2-r]_+ .
$$

Indeed the reversible comparison obeys $\bar r\ge\bar m^2$. Therefore
$m^2-r\le(m+\bar m)X+\bar r-r\le2mX+Z+\sqrt{2YZ}$.
Bounding this last sum by $\max\{2m,(1+\sqrt3)/2\}(E+Y)$ gives the claim.
This bound depends only on the two measured probabilities and holds for any
number of hidden states. At $m=2/5,r=1/8$ it certifies at least
$28/275$ per excursion. For frequency $5/9$ this gives $28/495$ per unit
time. The earlier count-divergence bound gave about $0.0105385$ per unit
time on this example. [Reversible variance and earlier example](../excursion-counts/HIDDEN-ORDER.md)

The linear order in the count deficit is attainable near every pure geometric
branch $G(t)$ with $0<t<1$. Use just the driven triangle of the next
construction with $w=1$. Then $m=t+X$, $r=t^2-(1-t)X$, so exactly
$m^2-r=(1+t)X+X^2$ while $S$ is of order $X$. The optimal uniform
coefficient is a separate question.

## Why the exponent cannot be improved

Take any $0<a<1$ and $0<w<1$. Begin with the equilibrium contact
$wG(a^2)+(1-w)P(a^3/D)$, using the geometric triangle and even-return
path from the equilibrium construction. Write $t=a^2$. The symmetric
per-excursion fluxes of the triangle are $w/2,w t/[2(1-t)],w/2$ around
its three edges. Add $\epsilon$ in the cyclic direction and subtract it
in reverse, keeping $|\epsilon|$ smaller than all three fluxes. This
preserves stationarity and all visit counts, and the symmetrization recovers
the contact exactly.

The two hidden sites in the driven branch have $s_i=1-t$ and opposite
$j_i=\pm2(1-t)\epsilon/w$. Direct substitution gives

$$
X=4(1-t)\epsilon^2/w,\qquad
j^\top G j=-tX,\qquad
2j^\top T(s-\tfrac12{\bf1})=(1-2t)X .
$$

Hence $m-\bar m=X$, $r-\bar r=-(1-t)X$, and
$H=(A-B(1-t))X=a^2(1-a)^2 X>0$ whenever $\epsilon\ne0$. The realizing
network has $S=\sum_g4\epsilon\operatorname{atanh}(\epsilon/g)$ over
the three displayed fluxes. As $\epsilon\to0$, $S/\epsilon^2\to4\sum_g1/g$
is finite and positive. The entropy infimum over all compatible networks
is therefore bounded above and below by positive constants times $H$ on
these triples. A lower bound proportional to $H^p$ for $p<1$, with a
fixed positive coefficient on this family, is impossible. Uniform exit
rates keep every occupancy and every exit rate fixed during the perturbation.
[Exact checks of the two tail changes and the contact](../../../crates/physis-theory/tests/excursion_activity/entropy.rs)
