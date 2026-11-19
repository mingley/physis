/- Finite-sum and flux algebra for the excursion activity theorem. Logarithms,
   stochastic paths and equality in the analytic inequalities are in RESULT.md. -/
namespace ExcursionActivity
variable {K : Type} [Lean.Grind.CommRing K]

private def dotSum (f : K × K → K) : List (K × K) → K
  | [] => 0
  | t :: ts => f t + dotSum f ts

private def xx (t : K × K) := t.1 ^ 2
private def yy (t : K × K) := t.2 ^ 2
private def xy (t : K × K) := t.1 * t.2
private def minor (a b : K × K) := (a.1*b.2 - a.2*b.1)^2

private theorem row_of_minors (a : K × K) (ts : List (K × K)) :
    dotSum (minor a) ts =
      xx a * dotSum yy ts + yy a * dotSum xx ts - 2 * xy a * dotSum xy ts := by
  induction ts with
  | nil => simp [dotSum]; grind
  | cons t ts ih =>
    simp only [dotSum, ih, xx, yy, xy, minor]
    grind

private def allMinors : List (K × K) → K
  | [] => 0
  | t :: ts => dotSum (minor t) ts + allMinors ts

/-- The Cauchy slack in either flux estimate is a sum of squares, for any
    finite hidden order. Substitute the two vectors specified in the proof. -/
theorem finite_cauchy_identity (ts : List (K × K)) :
    dotSum xx ts * dotSum yy ts - (dotSum xy ts)^2 = allMinors ts := by
  induction ts with
  | nil => simp [dotSum, allMinors]; grind
  | cons t ts ih =>
    simp only [dotSum, allMinors, row_of_minors, ← ih]
    unfold xx yy xy
    grind

/-- The internal and boundary estimates have the same squared deficit. -/
theorem equal_deficits (n q : K) :
    n^2 - (1+n)*(n-1+q) = 1-(1+n)*q := by grind

/-- Complementing both boundary jump probabilities preserves the product
    information needed for the second Cauchy bound. -/
theorem complement_product (u v : K) :
    (1-u)*(1-v) = 1-u-v+u*v := by grind

/-- Equality at both Cauchy steps restricts every hidden row to two types. -/
theorem equality_row (k u v q : K) (hs : k*(u+v)=2) (hp : k*u*v=q) :
    (k*u-1)^2 = 1-k*q := by grind

/-- Doubled minimizer fluxes balance rowwise and match the immediate return. -/
theorem minimizer_fluxes (n d q : K) (hd : d^2 = 1-(1+n)*q) :
    (1-d)+(n+d) = 1+n ∧ (1+d)+(n-d) = 1+n ∧
    (1+d)*(1-d) = (1+n)*q ∧ (n+d)*(n-d) = (1+n)*(n-1+q) := by grind

/-- A triangle-consistent mean converts the old threshold into the new one. -/
theorem mean_check (k m r : K) (h : k*(1-r) = 1+m) :
    (1-r)*(1-k*(1-m)) = m^2-r := by grind

/-- The two-hidden-state activity ceiling has the sign of the one-hop mass. -/
theorem state_count_slack (n m r : K) (h : n*(1-r) = m+r) :
    (2*m-(1-m)*n)*(1-r) = (1+m)*(m-r) := by grind

/-- The singleton/triangle mixture matches a prescribed zero-floor pair.
    a = pair return chance; w = probability of choosing the pair. -/
theorem reversible_mixture (w a n q : K) (h : w*(1-a)=1-q) (h' : a*n=1-q) :
    (1-w)+w*a=q ∧ w*(1-a)=a*n := by grind

/-- For X = log(f/b) and Y = log(p/(1-p)), the lift adds the indicated
    cost on each directed internal flux f. -/
theorem lift_entropy_increment (f p X Y : K) :
    f*p*(X+Y) + f*(1-p)*(X-Y) = f*X + f*(2*p-1)*Y := by grind

/-- The even-return path mixture reaches the equilibrium activity ceiling at w=1. -/
theorem even_pair_slack (a w r n : K) (h : n*(1-a)=2*r) (hw : w*a=r) :
    (n-(n+2)*r)*(1-a) = 2*r*a*(1-w) := by grind

/-- Exact joint-observation gap on the four-state 4:2:1 ring, with denominators cleared. -/
theorem joint_ring_control (m r n : K) (hm : 49*m=32) (hr : 343*r=164) (hn : n=2) :
    2401*((1+m)^2-(1+2*n)*(1-r))=296 := by grind

end ExcursionActivity
