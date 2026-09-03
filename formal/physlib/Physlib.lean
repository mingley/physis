/-
  Physlib: kernel-checked obligations for the Level-3 vertical slice.

  These theorems are *untrusted source* as far as physis-verifier is
  concerned. Authority comes only from Lean kernel compile plus nanoda
  replay of the lean4export of this module. `grind` / `omega` may produce
  the proof terms; they do not mint a receipt.
-/

/-- Discrete coboundary on a triangle: `d₁ ∘ d₀ = 0`. -/
theorem d_squared_zero (a b c : Int) :
    (b - a) - (c - a) + (c - b) = 0 := by
  omega

/-- Discrete coboundary on a tetrahedron: `d₂ ∘ d₁ = 0`.
Edge values `ab, ac, ad, bc, bd, cd`. Face coboundaries are
`ω_bc − ω_ac + ω_ab` and cyclic; `d₂` of those four faces is
identically zero. Not the triangle 0-form identity. -/
theorem d_squared_one (ab ac ad bc bd cd : Int) :
    (((((cd - bd) + bc) - ((cd - ad) + ac)) + ((bd - ad) + ab))
      - ((bc - ac) + ab)) = 0 := by
  omega

/-- Minkowski interval identity at `c = 1` (polynomial; no `γ`). -/
theorem invariant_interval (t x β : Int) :
    (t - β * x) ^ 2 - (x - β * t) ^ 2 = (1 - β ^ 2) * (t ^ 2 - x ^ 2) := by
  grind

/-- Einstein velocity addition: `1 − w²` shares the sign of `(1 − u²)(1 − v²)`
when `w = (u + v) / (1 + u v)`. Polynomial form; the inequality over ℝ is
still the lab evaluator. -/
theorem subluminal_composition (u v : Int) :
    (1 + u * v) ^ 2 - (u + v) ^ 2 = (1 - u ^ 2) * (1 - v ^ 2) := by
  grind

/-- Mass shell at `c = 1`: the Minkowski form on 4-momentum. Algebraically
the interval identity with `(t, x) → (E, p)`; not a new postulate. The
typed rest-mass check remains the lab evaluator. -/
theorem energy_momentum_invariant (E p β : Int) :
    (E - β * p) ^ 2 - (p - β * E) ^ 2 = (1 - β ^ 2) * (E ^ 2 - p ^ 2) := by
  grind

/-- Jacobi identity for the R^3 cross product: the x-component of
`a × (b × c) + b × (c × a) + c × (a × b)` is identically zero.
Not the Minkowski interval polynomial and not a boost identity. -/
theorem cross_product_jacobi (a1 a2 a3 b1 b2 b3 c1 c2 c3 : Int) :
    ((((a2 * ((b1 * c2) - (b2 * c1))) - (a3 * ((b3 * c1) - (b1 * c3))))
        + ((b2 * ((c1 * a2) - (c2 * a1))) - (b3 * ((c3 * a1) - (c1 * a3)))))
      + ((c2 * ((a1 * b2) - (a2 * b1))) - (c3 * ((a3 * b1) - (a1 * b3)))))
      = 0 := by
  grind
