# Ruling out equilibrium can still leave a zero entropy infimum

Use the [measurement protocol and notation of the theorem](RESULT.md).
At $r=m^2$, a reversible network has geometric count tails $P(N\ge j)=m^j$,
so its only possible mean is $n_0=m/(1-m)$. Nevertheless, for every
$0<m<1$ and every finite $n>n_0$, **the exact triple $(m,m^2,n)$ permits
every strictly positive finite entropy per excursion**. A sequence on five
states approaches zero while preserving the triple and excursion frequency.

## A sequence with exactly the prescribed counts

Set $r=m^2$. Mix a reversible geometric branch of continuation $t$ and
departure weight $w$ with a three-state triangle sharing the same visible 0.
Take $t<1$ sufficiently close to one and set

$$
w=\frac{(1-t)[n(1-r)-m-r]}
        {t^3-r-m(1-t)+n(1-t)(1-t^2)},\qquad
m_A=\frac{m-wt}{1-w},\quad r_A=\frac{r-wt^2}{1-w}.
$$

The denominator tends to $1-r>0$. The numerator is positive because
$n>n_0=(m+r)/(1-r)$. Thus $0<w<1$ eventually and $w\to0$. The triangle
has $0<r_A<m_A<1$ by continuity. Use the triangle of least entropy at
$m_A,r_A$ from the earlier construction. Its mean is
$n_A=(m_A+r_A)/(1-r_A)$. Both tails match by definition. Clearing the
denominator in $(1-w)n_A+wt/(1-t)=n$ leaves the linear equation for $w$
displayed above. Thus the mean also matches exactly at every such $t$.
[Explicit triangle rates](../excursion-counts/RESULT.md#exact-range-from-two-counts)

The driven branch has

$$
m_A^2-r_A=\frac{w(t-m)^2}{(1-w)^2}>0 .
$$

Writing $b=m_A-r_A$ and $d=\sqrt{(1-r_A)(m_A^2-r_A)}$, its minimum entropy
per excursion is $\frac{d}{1-r_A}\log[(b+d)/(b-d)]$. Here $d\to0$ and
$b\to m(1-m)>0$. Multiplying by the branch weight $1-w$ gives a strictly
positive total entropy tending to zero. The other branch accounts for
the finite excess mean through rarer and longer excursions, with
$wt/(1-t)\to n-n_0$. For example $m=1/2,r=1/4,n=2$ gives
$w=3(1-t)/(12t^3-8t^2-6t+5)$.

The two branches have four hidden sites in total. Equal exit rates on all
five sites can be fixed throughout at $\nu(n+2)$ to give any specified
excursion frequency $\nu$. For any desired $S>0$, first take a member of
the sequence below $S$, then use the finite hidden-state lift that adds a
continuously tunable entropy at the same projected clock. This gives the
entire open interval. [Entropy-increasing lift](../excursion-activity/RESULT.md#why-even-a-complete-jump-clock-has-no-finite-upper-bound)

## When a failed fit does guarantee a positive floor

For an interior triple below the frontier choose any $a\in[0,1)$ with
$H=Am+Br-2a^6-Mn>0$, using the coefficients in the theorem. The maximizing
value from the theorem gives strict separation, and a sufficiently close
rational $a$ does too. Symmetrize stationary edge fluxes per excursion via
$\bar f_{ij}=(f_{ij}+f_{ji})/2$. This preserves visits and $n$. Relative
entropy of complete excursion paths against this equilibrium comparison is
at most $S/4$. Hence each tail differs from its comparison value by at most
$\sqrt{S/8}$ by Pinsker's inequality. The supporting plane for the comparison
then gives

$$
S\ \ge\ 8\left(\frac{H}{|A|+|B|}\right)^2.
$$

This is a computable lower bound for any compatible chain. At $r<m^2$
the earlier two-tail estimate already gives a positive bound. When $r=m^2$
and $n<n_0$, the immediate-return fraction and mean give a positive bound.
When $r=m$ and $n<2m/(1-m)$, the second-tail and mean estimate gives one.
These cover every positive-floor entry in the table.
[Path comparison](../excursion-activity/JOINT.md#converting-the-violation-into-an-entropy-bound) · [Two-tail estimate](../excursion-counts/HIDDEN-ORDER.md) · [Immediate-return and mean bound](../excursion-activity/RESULT.md)
