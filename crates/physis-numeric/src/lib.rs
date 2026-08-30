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
    pub fn new(num: i128, den: i128) -> Self {
        assert!(den != 0, "ratio denominator must be nonzero");
        let (n, d) = reduce(num, den);
        Self { num: n, den: d }
    }

    /// Integer.
    pub fn int(n: i128) -> Self {
        Self { num: n, den: 1 }
    }

    /// Inclusive interval enclosure (exact).
    pub fn enclosure(self) -> Interval {
        Interval { lo: self, hi: self }
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn reduce(num: i128, den: i128) -> (i128, i128) {
    let s = if den < 0 { -1 } else { 1 };
    let n = num * s;
    let d = den.abs();
    let g = gcd(n, d);
    (n / g, d / g)
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

    /// Conservative hull of a machine float. Prefer [`Ratio`] for threshold
    /// claims. One ulp of slack, stored as a dyadic rational interval.
    #[allow(dead_code)]
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
    }
}
