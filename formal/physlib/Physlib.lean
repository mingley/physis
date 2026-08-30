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

/-- Minkowski interval identity at `c = 1` (polynomial; no `γ`). -/
theorem invariant_interval (t x β : Int) :
    (t - β * x) ^ 2 - (x - β * t) ^ 2 = (1 - β ^ 2) * (t ^ 2 - x ^ 2) := by
  grind
