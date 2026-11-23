/- Denominator-cleared identities for the entropy estimates and the factory.
   The continuous inequalities and the excursion arguments are in RESULT.md
   and LINEAR.md. No imported analytic axioms or external tactics. -/
namespace ExcursionEntropy
variable {K : Type} [Lean.Grind.CommRing K]

/-- The two entropy estimates see the same squared deficit. -/
theorem layer_deficit (b q : K) :
    b^2-(1+b)*(b-1+q) = 1-(1+b)*q := by grind

/-- Reciprocal internal flux stays positive on the physical domain b>m. -/
theorem reciprocal_margin (b m : K) :
    b^2-(1-(1+b)*(1-m)) = (1+b)*(b-m) := by grind

/-- Entry and return probabilities recover the target mean continuation. -/
theorem fixed_visits (c u h m : K) :
    (u*h+m-c)*(c+(1-u)*h)+((1-u)*h-m+c)*(c-u*h) = m*h := by grind

/-- Clear denominators in the mean of the reversible two-branch square. -/
theorem reversible_mean (p x y : K) :
    p*x*(1-y)+(1-p)*y*(1-x) = p*x+(1-p)*y-x*y := by grind

/-- Exact first continuation in the uniform four-state laboratory. -/
theorem clock_product (u v : K) :
    (1+2*v)*(1+2*u)+(1-2*v)*(1-2*u) = 2+8*u*v := by grind

/-- Each hidden edge contributes the two orientations of a one-hop return. -/
theorem one_jump_shift (s j v l g k : K) :
    (s+j)*(g+k)*(v-l)+(v+l)*(g-k)*(s-j)-2*s*g*v =
    -2*j*g*l+2*k*(j*v-s*l) := by grind

/-- The boundary current also pays for the hidden symmetric quadratic term. -/
theorem boundary_budget (s : K) : 1-s*(2-s) = (1-s)^2 := by grind

/-- 11/8 is a rational multiplier valid for the mixed quadratic term. -/
theorem rational_constant (z y : K) :
    (3*z+11*y)^2-128*z*y = (3*z-11*y)^2+4*z*y := by grind

/-- A winding on a contact triangle moves strictly beyond its supporting plane. -/
theorem contact_gap (a : K) :
    2*(1+a-a^2)^2-(1+2*a-a^2)-(1+2*a-a^2)*(1-a^2) =
    a^2*(1-a)^2 := by grind

/-- Integer coefficients evaluate exactly L² P(Lz), including double roots. -/
theorem homogeneous_decision (l d v k r e z : K) :
    2*(l*d)^3*(l^2*v)*z^3+
      ((l*k)^2*(l^2*e)+(l^2*v)^2-2*(l*r)*(l^2*v)*(l*k-l*d))*z^2+
      2*((l*k)*(l^2*e)-(l*r)*(l^2*v))*z+l^2*e =
    l^2*(2*d^3*v*(l*z)^3+(k^2*e+v^2-2*r*v*(k-d))*(l*z)^2+
         2*(k*e-r*v)*(l*z)+e) := by grind

end ExcursionEntropy
