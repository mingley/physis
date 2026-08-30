//! Validated numerics. Raw `f64` is not authoritative for threshold claims.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use serde::{Deserialize, Serialize};

/// Numerical assurance tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NumericTier {
    /// Uncontrolled floating point.
    Approximate,
    /// Extra care (compensated sum, …) but still not a certificate.
    StableFloat,
    /// Arbitrary precision, still a rounding mode.
    HighPrecision,
    /// Interval / ball enclosure.
    IntervalCertified,
    /// Exact integer or rational.
    Exact,
}

impl NumericTier {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            NumericTier::Approximate => "approximate",
            NumericTier::StableFloat => "stable-float",
            NumericTier::HighPrecision => "high-precision",
            NumericTier::IntervalCertified => "interval-certified",
            NumericTier::Exact => "exact",
        }
    }
}

/// Exact rational `num/den` in lowest terms, `den > 0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ratio {
    /// Numerator.
    pub num: i128,
    /// Positive denominator.
    pub den: i128,
}

impl Ratio {
    /// Construct and reduce. Panics if den == 0 (a programming error).
    pub const fn new(num: i128, den: i128) -> Self {
        assert!(den != 0, "ratio denominator must be nonzero");
        let (n, d) = reduce(num, den);
        Self { num: n, den: d }
    }

    /// Integer.
    pub const fn int(n: i128) -> Self {
        Self { num: n, den: 1 }
    }

    /// Inclusive interval enclosure (exact).
    pub fn enclosure(self) -> Interval {
        Interval { lo: self, hi: self }
    }

    /// Non-negative integer power.
    pub fn pow(self, n: u32) -> Self {
        let mut acc = Ratio::int(1);
        for _ in 0..n {
            acc = acc * self;
        }
        acc
    }

    /// True when the numerator is zero.
    pub fn is_zero(self) -> bool {
        self.num == 0
    }

    /// IEEE-754 approximation. Not a certificate.
    pub fn to_f64(self) -> f64 {
        (self.num as f64) / (self.den as f64)
    }

    /// Round `x` to the nearest multiple of `1/den`. Not a certificate of
    /// the float; use when a computed centre must share the dataset's
    /// rational scale before an exact NLL.
    pub fn nearest(x: f64, den: i128) -> Self {
        assert!(den > 0, "nearest-denominator must be positive");
        if !x.is_finite() {
            return Self::int(0);
        }
        let n = (x * (den as f64)).round() as i128;
        Self::new(n, den)
    }

    /// Gaussian negative log-likelihood `(x − μ)² / (2σ²)` as an exact
    /// Ratio. `sigma` must be positive. This is a defined statistical
    /// object, not an LLM-invented confidence.
    pub fn gaussian_nll(self, mu: Self, sigma: Self) -> Self {
        assert!(sigma > Ratio::int(0), "gaussian sigma must be positive");
        let d = self - mu;
        (d * d) / (Ratio::int(2) * sigma * sigma)
    }

    /// Exact square root when both numerator and denominator are perfect
    /// squares. `None` if the radicand is negative or not a square in Q.
    pub fn checked_sqrt(self) -> Option<Self> {
        if self.num < 0 {
            return None;
        }
        let n = isqrt_exact(self.num)?;
        let d = isqrt_exact(self.den)?;
        Some(Ratio::new(n, d))
    }
}

impl std::ops::Add for Ratio {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        add_ratio(self, rhs)
    }
}

impl std::ops::Sub for Ratio {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        sub_ratio(self, rhs)
    }
}

impl std::ops::Mul for Ratio {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        mul_ratio(self, rhs)
    }
}

impl std::ops::Div for Ratio {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(!rhs.is_zero(), "ratio denominator must be nonzero");
        self * Ratio::new(rhs.den, rhs.num)
    }
}

impl std::ops::Neg for Ratio {
    type Output = Self;
    fn neg(self) -> Self {
        Ratio::new(-self.num, self.den)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        cmp_ratio(*self, *other)
    }
}

impl std::fmt::Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

const fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

const fn reduce(num: i128, den: i128) -> (i128, i128) {
    let s = if den < 0 { -1 } else { 1 };
    let n = num * s;
    let d = den.abs();
    let g = gcd(n, d);
    (n / g, d / g)
}

/// Integer square root when `n` is a perfect square.
fn isqrt_exact(n: i128) -> Option<i128> {
    if n < 0 {
        return None;
    }
    if n <= 1 {
        return Some(n);
    }
    let mut x = n;
    while x > n / x {
        let next = x.saturating_add(n / x) / 2;
        if next >= x {
            break;
        }
        x = next;
        if x == 0 {
            return None;
        }
    }
    if x.checked_mul(x) == Some(n) {
        return Some(x);
    }
    // Newton's method can land one below a perfect square.
    let up = x.checked_add(1)?;
    if up.checked_mul(up) == Some(n) {
        Some(up)
    } else {
        None
    }
}

/// Closed interval with exact rational endpoints.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Interval {
    /// Lower bound.
    pub lo: Ratio,
    /// Upper bound.
    pub hi: Ratio,
}

impl Interval {
    /// Point interval.
    pub fn point(r: Ratio) -> Self {
        Self { lo: r, hi: r }
    }

    /// From inclusive rational bounds; swaps if reversed.
    pub fn new(a: Ratio, b: Ratio) -> Self {
        if cmp_ratio(a, b) == std::cmp::Ordering::Greater {
            Self { lo: b, hi: a }
        } else {
            Self { lo: a, hi: b }
        }
    }

    /// True when every point of `self` is less than every point of `other`.
    pub fn strictly_left_of(self, other: Self) -> bool {
        cmp_ratio(self.hi, other.lo) == std::cmp::Ordering::Less
    }

    /// Disjoint closed intervals.
    pub fn disjoint(self, other: Self) -> bool {
        self.strictly_left_of(other) || other.strictly_left_of(self)
    }

    /// True when every point of `other` lies in `self` (closed).
    pub fn contains(self, other: Self) -> bool {
        cmp_ratio(self.lo, other.lo) != std::cmp::Ordering::Greater
            && cmp_ratio(other.hi, self.hi) != std::cmp::Ordering::Greater
    }

    /// Smallest closed interval containing both.
    pub fn hull(self, other: Self) -> Self {
        let lo = if cmp_ratio(self.lo, other.lo) == std::cmp::Ordering::Greater {
            other.lo
        } else {
            self.lo
        };
        let hi = if cmp_ratio(self.hi, other.hi) == std::cmp::Ordering::Less {
            other.hi
        } else {
            self.hi
        };
        Self { lo, hi }
    }

    /// Closed envelope `[(1 − rel) · self, (1 + rel) · self]` for a
    /// non-negative interval. `rel` is a relative half-width (3/100 is ±3%).
    pub fn relative_envelope(self, rel: Ratio) -> Self {
        let one = Ratio::int(1);
        self.scaled(sub_ratio(one, rel))
            .hull(self.scaled(add_ratio(one, rel)))
    }

    fn scaled(self, factor: Ratio) -> Self {
        Self::new(mul_ratio(self.lo, factor), mul_ratio(self.hi, factor))
    }

    /// Conservative hull of a machine float. Prefer [`Ratio`] for threshold
    /// claims. One ulp of slack, stored as a dyadic rational interval.
    pub fn from_f64_approx(x: f64) -> Self {
        if !x.is_finite() {
            return Self::point(Ratio::int(0));
        }
        // x = m * 2^e exactly for a finite f64. Enclose by ± 2^(e-52) which
        // is one ulp at this magnitude (normal numbers).
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i128 } else { 1 };
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        if exp == 0 {
            // subnormal: value = frac * 2^-1074. One ulp is 2^-1074.
            let n = sign * (frac as i128);
            let ulp = Ratio::new(1, 1i128 << 62); // coarser than 2^-1074; we widen extra
            let p = Ratio::new(n, 1i128 << 52);
            return Self::new(sub_ratio(p, ulp), add_ratio(p, ulp));
        }
        let m = (frac as i128) | (1i128 << 52);
        let e = exp - 1023 - 52;
        let p = if e >= 0 {
            Ratio::int(sign * m * (1i128 << e.min(60)))
        } else {
            Ratio::new(sign * m, 1i128 << ((-e) as u32).min(120))
        };
        let ulp = if e >= 0 {
            Ratio::int(1i128 << e.min(60))
        } else {
            Ratio::new(1, 1i128 << ((-e) as u32).min(120))
        };
        Self::new(sub_ratio(p, ulp), add_ratio(p, ulp))
    }
}

fn cmp_ratio(a: Ratio, b: Ratio) -> std::cmp::Ordering {
    // a.num/a.den ? b.num/b.den  →  a.num*b.den ? b.num*a.den
    let left = a.num.saturating_mul(b.den);
    let right = b.num.saturating_mul(a.den);
    left.cmp(&right)
}

fn add_ratio(a: Ratio, b: Ratio) -> Ratio {
    Ratio::new(
        a.num
            .saturating_mul(b.den)
            .saturating_add(b.num.saturating_mul(a.den)),
        a.den.saturating_mul(b.den),
    )
}

fn sub_ratio(a: Ratio, b: Ratio) -> Ratio {
    add_ratio(a, Ratio::new(-b.num, b.den))
}

fn mul_ratio(a: Ratio, b: Ratio) -> Ratio {
    Ratio::new(a.num.saturating_mul(b.num), a.den.saturating_mul(b.den))
}

/// A computation that returns a checkable certificate.
pub trait CertifiedComputation {
    /// Input.
    type Input;
    /// Output.
    type Output;
    /// Certificate.
    type Certificate;
    /// Compute.
    fn compute(input: &Self::Input) -> (Self::Output, Self::Certificate);
    /// Independent check.
    fn verify(input: &Self::Input, output: &Self::Output, certificate: &Self::Certificate) -> bool;
}

/// Hash of a numeric certificate for receipts.
pub fn certificate_hash(bytes: &[u8]) -> ArtifactId {
    ArtifactId::of(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_eighths_does_not_cover_weinberg_at_mz() {
        // GUT-scale sin²θ_W = 3/8. Measured sin²θ_W(M_Z) ≈ 0.23122.
        let gut = Interval::point(Ratio::new(3, 8));
        // 0.23122 as a rational enclosure 23122/100000 ± 1/100000
        let mz = Interval::new(Ratio::new(23121, 100000), Ratio::new(23123, 100000));
        assert!(gut.disjoint(mz));
        // 3/8 = 0.375 is far from 0.231; this threshold cannot flip by ulp noise.
        assert_eq!(Ratio::new(3, 8), Ratio::new(6, 16));
    }

    #[test]
    fn overlapping_intervals_are_not_an_exclusion() {
        let a = Interval::new(Ratio::new(1, 5), Ratio::new(1, 4));
        let b = Interval::new(Ratio::new(22, 100), Ratio::new(24, 100));
        assert!(!a.disjoint(b));
        assert!(a.contains(b));
        assert!(!b.contains(a));
    }

    #[test]
    fn a_wide_band_is_not_contained_in_a_tight_measurement() {
        let mz = Interval::new(Ratio::new(23121, 100000), Ratio::new(23123, 100000));
        let band = Interval::point(Ratio::new(23122, 100000)).relative_envelope(Ratio::new(3, 100));
        assert!(band.contains(mz));
        assert!(!mz.contains(band));
        assert!(!band.disjoint(mz));
    }

    #[test]
    fn sm_hypercharge_cube_cancels_exactly() {
        // One generation, integer colour × weak multiplicities, Y as Ratio.
        let fields: [(i128, i128, Ratio); 5] = [
            (3, 2, Ratio::new(1, 6)),
            (3, 1, Ratio::new(-2, 3)),
            (3, 1, Ratio::new(1, 3)),
            (1, 2, Ratio::new(-1, 2)),
            (1, 1, Ratio::int(1)),
        ];
        let mut cubic = Ratio::int(0);
        for (color, weak, y) in fields {
            cubic = cubic + Ratio::int(color) * Ratio::int(weak) * y.pow(3);
        }
        assert!(cubic.is_zero(), "Σ colour·weak·Y³ = {cubic}");
        let flipped = cubic + Ratio::int(1);
        assert!(!flipped.is_zero());
    }

    #[test]
    fn checked_sqrt_is_exact_or_absent() {
        assert_eq!(Ratio::int(0).checked_sqrt(), Some(Ratio::int(0)));
        assert_eq!(Ratio::int(1).checked_sqrt(), Some(Ratio::int(1)));
        assert_eq!(Ratio::int(9).checked_sqrt(), Some(Ratio::int(3)));
        assert_eq!(Ratio::new(4, 9).checked_sqrt(), Some(Ratio::new(2, 3)));
        assert_eq!(Ratio::new(36, 49).checked_sqrt(), Some(Ratio::new(6, 7)));
        assert_eq!(Ratio::int(2).checked_sqrt(), None);
        assert_eq!(Ratio::int(8).checked_sqrt(), None);
        assert_eq!(Ratio::int(-1).checked_sqrt(), None);
        for k in 0i128..=256 {
            let sq = Ratio::int(k * k);
            assert_eq!(sq.checked_sqrt(), Some(Ratio::int(k)), "sqrt({}^2)", k);
        }
        // Discriminant of the SM {Y_u, Y_d} quadratic t^2 - s t + p = 0.
        let s = Ratio::new(-1, 3);
        let p = Ratio::new(-2, 9);
        let disc = s * s - Ratio::int(4) * p;
        assert_eq!(disc, Ratio::int(1));
        assert_eq!(disc.checked_sqrt(), Some(Ratio::int(1)));
    }

    #[test]
    fn gaussian_nll_of_the_mean_is_zero_and_one_sigma_is_half() {
        let mu = Ratio::new(23122, 100000);
        let sigma = Ratio::new(1, 100000);
        assert_eq!(mu.gaussian_nll(mu, sigma), Ratio::int(0));
        assert_eq!((mu + sigma).gaussian_nll(mu, sigma), Ratio::new(1, 2));
        assert_eq!((mu + sigma + sigma).gaussian_nll(mu, sigma), Ratio::int(2));
        assert_eq!(Ratio::nearest(0.23122, 100_000), mu);
        assert_eq!(Ratio::nearest(0.207, 100_000), Ratio::new(20700, 100000));
    }
}
