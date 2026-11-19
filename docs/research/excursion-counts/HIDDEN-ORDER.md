# A lower bound when the hidden state count is unknown

Consider a finite irreducible stationary continuous-time Markov chain with
one on state 0. Observe each jump, its time and whether the state is off.
Use natural logarithms, measure $\sigma$ in $k_B$ per model time unit and
let $\nu$ be the excursion frequency. One effective channel per ordered
pair suffices. We assume reciprocal edges; an unpaired positive stationary
flux has infinite entropy production in the usual flux formula.

Group complete excursions by their count $N$ of off-to-off jumps. Write
$m=P(N\ge1)$, $r=P(N\ge2)$ and $q=(1-m,m-r,r)$. For $0<r<m<1$ and
$m^2>r$, define

$$
t=\frac{m+r}{1+m},\qquad b=(1-t,\ t(1-t),\ t^2).
$$

Then, for *any* finite hidden state count,

$$
\frac{\sigma}{\nu}\ \ge\ 4D_{\rm KL}(q\Vert b)
=4\sum_{i=0}^2 q_i\log(q_i/b_i)>0.
$$

If $m^2\le r$, the two tail probabilities and frequency alone admit an
equilibrium three-state construction. For $m^2>r$, this bound is a
guarantee. The sharper minimum derived for a triangle uses its specified
number of states. [Three-state result](RESULT.md)

## What reversibility requires

Take the chain at jump epochs, with transition matrix $P$ and stationary
weights $\mu_i$ proportional to $\pi_i\lambda_i$, where $\pi_i$ and
$\lambda_i$ are the continuous-time occupancy and exit rate. A reversible
chain satisfies detailed balance for these jump-epoch weights. The hidden
submatrix $H=(P_{ij})_{i,j\ne0}$ is self-adjoint for the inner product
weighted by $\mu$ on the hidden states. Its eigenvalues lie in $[-1,1]$
by substochasticity. Let $\mathbf{1}$ denote the vector of ones there.

The entrance probability from 0 into hidden state $i$ equals
$\mu_i[(I-H)\mathbf{1}]_i/\mu_0$ by detailed balance. Thus

$$
P(N\ge j)=
\frac{\langle(I-H)\mathbf{1},H^j\mathbf{1}\rangle_\mu}
{\langle(I-H)\mathbf{1},\mathbf{1}\rangle_\mu}.
$$

In an orthonormal eigenbasis this is the $j$th moment of a probability
measure on $[-1,1]$: if $\mathbf{1}$ has coefficient $c_\ell$ at
eigenvalue $\theta_\ell$, the weight is proportional to
$(1-\theta_\ell)c_\ell^2$. Consequently $r-m^2$ is a variance and is
nonnegative. The use of jump epochs matters because discrete-time
eigenvalues can be negative. Related continuous-time hitting laws under
reversibility have classical spectral representations.
[Aldous and Fill, §3.5](https://www.stat.berkeley.edu/~aldous/RWG/Book_Ralph/Ch3.S5.html)

## How far the counts are from reversibility

For the given chain let $f_{ij}=\pi_i Q_{ij}$ be its stationary flux on
an edge. Replace it by $g_{ij}=(f_{ij}+f_{ji})/2$ and give the comparison
chain rates $g_{ij}/\pi_i$. Conservation at each vertex preserves its
total exit rate, stationary occupancy and $\nu=\pi_0\lambda_0$.
The comparison chain is reversible. Let $\bar q$ be its three count
probabilities.

For one complete microscopic return path starting at 0, its likelihood
ratio against the comparison chain is the product of $f_{ij}/g_{ij}$
over jumps. Both chains have identical exit rates, so holding-time factors
cancel. Expected traversals of $i\to j$ per excursion are $f_{ij}/\nu$ by
regeneration. Coarse-graining a path to its count bin and applying the
log-sum inequality gives

$$
\nu D_{\rm KL}(q\Vert\bar q)
\le \sum_{i\ne j} f_{ij}\log(f_{ij}/g_{ij}) \le \sigma/4.
$$

For the second inequality, take any unordered edge, write its fluxes as
$f,b>0$ with $f\ge b$, and set $h=(f-b)/(f+b)$. The contribution of that
edge to the middle term, divided by $f+b$, is
$\int_0^h \operatorname{atanh}(u)\,du$. Its entropy contribution in the
same units is $2h\operatorname{atanh}(h)$. Convexity of
$\operatorname{atanh}$ on $[0,h]$ bounds the integral above by
$h\operatorname{atanh}(h)/2$. Summing proves the claim.

Finally minimize $D_{\rm KL}(q\Vert b')$ over all count laws $b'$ obeying
the necessary condition $r'\ge(m')^2$. This is a convex region in the
probability simplex. When $m^2>r$, the unconstrained minimizer $q$ is
outside it. A boundary face with a zero bin has infinite divergence for
the stated positive $q$, so the constrained optimum has $r'=(m')^2$.
Set $m'=u$. Maximizing the expected log likelihood along that boundary
amounts to maximizing
$(1-r)\log(1-u)+(m+r)\log u$ over $0<u<1$. Its unique maximizer is
$t=(m+r)/(1+m)$, giving the displayed bound. For $m=2/5$, $r=1/8$,
$\nu=5/9$ one obtains $t=3/8$ and $4\nu D_{\rm KL}(q\Vert b)\simeq
0.01053848$. This argument allows the comparison step and the count
coarse-graining to lose information.

## Fitting three states can overestimate dissipation by an unbounded factor

Use five states, formed by two triangles that share only state 0. At each
departure from 0 choose the first triangle with probability $\alpha$,
the second with probability $1-\alpha$, for $0<\alpha<1$. Conditional on
the choice use the following embedded probabilities in the notation of the
[three-state construction](RESULT.md).

| Triangle | $p$ | $x$ | $y$ | $P(N\ge1)$ | $P(N\ge2)$ | Entropy per excursion in units of $k_B$ |
|---|---:|---:|---:|---:|---:|---:|
| Driven | $3/4$ | $3/4$ | $1/4$ | $5/8$ | $3/16$ | $\tfrac12\log 27$ |
| Reversible | $1/2$ | $t$ | $t$ | $t$ | $t^2$ | $0$ |

Here $t=1-\alpha^2$. Give every state exit rate one. The chain is finite,
irreducible and has reciprocal edges. Each completed excursion stays in its
chosen triangle. In its forward/reverse probability ratio the branch
selection factor cancels. The true entropy per excursion is therefore
$\sigma/\nu=(\alpha/2)\log 27$. The two observed tails are

$$
M=\alpha(5/8)+(1-\alpha)t,\qquad
R=\alpha(3/16)+(1-\alpha)t^2.
$$

Every positive $\alpha$ gives $0<R<M<1$. The inverse map in the three-state
result realizes both tails and the measured $\nu$ exactly. To see what that
fit would imply, take $\alpha\downarrow0$. The three ratios
$(1-R)/\alpha$, $(M-R)/\alpha$ and $(M^2-R)/\alpha$ tend respectively
to $13/16$, $7/16$ and $1/16$. For the last limit use
$M^2-R=\alpha[13/64-(1-\alpha)(5/8-t)^2]$. The three-state floor per
excursion thus tends to

$$
\frac{1}{\sqrt{13}}\log\frac{7+\sqrt{13}}{7-\sqrt{13}}\simeq0.315967,
$$

while the true entropy per excursion tends to zero. The ratio of rates per
unit time is the same as the ratio per excursion since the fit matches
$\nu$. With unit exit rates $\nu$ itself tends to zero.

For $\alpha=1/100$, the true value is about $0.016479$ per excursion and
the three-state minimum is about $0.316395$. An exact separation already
follows from $\log((s+d)/(s-d))\ge 2d/s$ for $0\le d<s$. The inferred
floor is at least $2(M^2-R)/(M-R)>7/25$ while the true value is below
$1/50$ since $\log 27<4$.

The retained measurements here are the first two tails and the frequency.
A counter that retains higher counts could detect this particular model
mismatch: $P(N\ge4)-R^2=\alpha(1-\alpha)(3/16-t^2)^2$, strictly positive
in the small-$\alpha$ regime. Every three-state fit predicts zero for this
difference. Related hidden-order bias has been derived for two individually
visible states with stationary flux and duration data. The observation
protocol determines which lower bound applies.
[Petkov, preprint, version 1](https://hexagonmath.org/2611.00657v1)
