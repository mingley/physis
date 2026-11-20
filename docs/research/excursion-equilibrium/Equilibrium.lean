/- Algebra for exact equilibrium feasibility of three excursion statistics.
   All identities are denominator-cleared and valid over commutative rings. -/
namespace ExcursionEquilibrium
variable {K : Type} [Lean.Grind.CommRing K]

private def chord (a : K) := 1+a-a^2
private def scale (a : K) := (1+a)^2*(1-a^2)^2
private def beta (a : K) := 1+2*a-a^2
private def alpha (a : K) := 2*(chord a)^2-beta a

/-- A reversible isolated hidden edge has m² ≤ r after restoring the denominator. -/
theorem edge_slack (x y : K) :
    x*y*(x+y-2*x*y)^2 - (x*y*(2-x-y))^2 = x*y*(x-y)^2*(1-x*y) := by grind

/-- The common supporting plane touches the geometric branch at x=a². -/
theorem geometric_support (a x : K) :
    scale a*x-(1-x)*(alpha a*x+beta a*x^2-2*a^6) =
    (x-a^2)^2*(beta a*x+2*a^2) := by
  unfold scale alpha beta chord
  grind

/-- The same plane touches the even-return path at s=a³/(1+a-a²). -/
theorem path_support (a s : K) :
    2*scale a*s-(1-s)*((alpha a+beta a)*s-2*a^6) =
    2*(chord a*s-a^3)^2 := by
  unfold scale alpha beta chord
  grind

/-- All mixtures of the two contacts lie on the cubic that selects the plane. -/
theorem contact_chord (a w : K) :
    w*a^4*chord a+(1-w)*a^3 =
    chord a*a^3+(w*a^2*chord a+(1-w)*a^3)*(a^2-a) := by
  unfold chord
  grind

/-- Positive residual mass after matching both count tails in the two-branch family. -/
theorem mixture_capacity (m r y : K) :
    (1-y)*(r-y)-(m-y)^2 = r-m^2-(1-2*m+r)*y := by grind

/-- The pole of the mean precedes y=r in the interior. -/
theorem pole_gap (m r : K) : r*(1-2*m+r)-(r-m^2) = (m-r)^2 := by grind

/-- The polynomial used by the rational decision procedure after y=vz/(1+kz). -/
theorem decision_cubic (m r n z : K) :
    let d := m-r
    let v := r-m^2
    let k := 1-2*m+r
    let e := m^2-n*d
    (m*(1+k*z)-v*z)^2 + 2*d*v*z*(1+k*z) +
      2*d*v*z^2*(r*(1+k*z)-v*z) - n*d*(1+k*z)^2 =
    2*d^3*v*z^3+(k^2*e+v^2-2*r*v*(k-d))*z^2+2*(k*e-r*v)*z+e := by
  dsimp
  grind

/-- The clock constraint for the long geometric branch is linear in its weight. -/
theorem trap_mean (m r n t w : K) :
    (1-w)*(m+r-w*(t+t^2))*(1-t)+w*t*(1-r-w*(1-t^2))-
        n*(1-t)*(1-r-w*(1-t^2)) =
    (1-t)*(m+r-n*(1-r))+w*(t^3-r-m*(1-t)+n*(1-t)*(1-t^2)) := by grind

/-- Exact squared count deficit on the weakly driven branch. -/
theorem trap_deficit (m t w : K) :
    (m-w*t)^2-(1-w)*(m^2-w*t^2) = w*(t-m)^2 := by grind

end ExcursionEquilibrium
