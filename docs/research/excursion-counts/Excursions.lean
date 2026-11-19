/- Algebra for the three-state excursion result. No external library or axioms
   beyond Lean's prelude. Probability, positivity, logarithms and limits are
   treated in RESULT.md. This file is independent of the lab receipt catalog. -/
namespace Excursions

variable {K : Type} [Lean.Grind.CommRing K]

def zeroLeft (p x : K) := p * (1 - x)
def zeroRight (p y : K) := (1 - p) * (1 - y)
def forward (p x y : K) := p * x * (1 - y)
def backward (p x y : K) := (1 - p) * y * (1 - x)
def oneTail (p x y : K) := p * x + (1 - p) * y

theorem zero_sum (p x y : K) :
    zeroLeft p x + zeroRight p y = 1 - oneTail p x y := by
  unfold zeroLeft zeroRight oneTail
  grind

theorem one_sum (p x y : K) :
    forward p x y + backward p x y = oneTail p x y - x*y := by
  unfold forward backward oneTail
  grind

theorem path_product (p x y : K) :
    forward p x y * backward p x y = x*y * zeroLeft p x * zeroRight p y := by
  unfold forward backward zeroLeft zeroRight
  grind

/-- The squared-current inequality has an explicit square as slack. -/
theorem floor_slack (p x y : K) :
    (forward p x y - backward p x y)^2 - (1-x*y)*((oneTail p x y)^2-x*y)
      = x*y * (zeroLeft p x - zeroRight p y)^2 := by
  unfold forward backward oneTail zeroLeft zeroRight
  grind

/-- Cross-multiplied verification of the inverse map's two-hop probability. -/
theorem inverse_two_hops (w v f b r : K) (h : f*b = r*w*v) :
    (f+r*w)*(b+r*v) = r*(w+f)*(v+b) := by
  grind

theorem inverse_first_path (p x y r w f : K)
    (hr : x*y = r) (hw : p*(1-x) = w) (hf : p*(1-r) = w+f) :
    forward p x y = f := by
  unfold forward
  grind

def sample (w v a b : K) (j : Nat) := w*a^j + v*b^j

/-- Two-component exponential mixture, sampled at equally spaced times. -/
theorem moment_determinants (w v a b : K) :
    let h := sample w v a b
    let D := h 0 * h 2 - (h 1)^2
    D = w*v*(a-b)^2 ∧
      h 0 * h 3 - h 1 * h 2 = (a+b)*D ∧
      h 1 * h 3 - (h 2)^2 = a*b*D := by
  dsimp [sample]
  grind

/-- The short-count deficit of a mixture with a reversible geometric branch. -/
theorem mixture_deficit (h m r t : K) :
    (h*m + (1-h)*t)^2 - (h*r + (1-h)*t^2)
      = h*((m^2-r) - (1-h)*(m-t)^2) := by
  grind

/-- The fourth count tail distinguishes branches with different two-hop tails. -/
theorem mixture_fourth (h r u : K) :
    h*r^2 + (1-h)*u^2 - (h*r + (1-h)*u)^2 = h*(1-h)*(r-u)^2 := by
  grind

end Excursions
