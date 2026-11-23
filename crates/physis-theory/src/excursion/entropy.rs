//! Entropy per excursion inferred from population counts in a stationary,
//! reciprocal finite CTMC. See docs/research/excursion-entropy/RESULT.md.

use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};
use physis_numeric::Ratio;

fn integer(value: i32) -> BigRational {
    BigRational::from_integer(value.into())
}

/// Sharp minimum when P(N=1)=0, m=P(N≥1) and n=E[N]. Returns a numerical
/// display in units of k_B per excursion. Domain and deficit signs are exact.
pub fn one_jump_absent_minimum(m: Ratio, n: Ratio) -> Option<f64> {
    if m.den <= 0 || n.den <= 0 {
        return None;
    }
    let [m, n] = [m, n].map(BigRational::from);
    let one = BigRational::one();
    if m.is_zero() && n.is_zero() {
        return Some(0.0);
    }
    let b = n / integer(2);
    if m <= BigRational::zero() || m >= one || b <= m {
        return None;
    }
    let boundary_gap = (&one + &b) * (&one - &m);
    let deficit = &one - &boundary_gap;
    if deficit <= BigRational::zero() {
        return Some(0.0);
    }
    let internal_gap = (&one + &b) * (&b - m) / b.pow(2);
    let d = deficit.to_f64()?.sqrt();
    // For x near one, use the exact 1-x². Forming 1-x in f64 could
    // erase a strictly positive reverse flux and display infinite entropy.
    let atanh = |x: f64, gap: f64| {
        if x < 0.5 {
            x.atanh()
        } else {
            x.ln_1p() - 0.5 * gap.ln()
        }
    };
    Some(
        4.0 * d
            * (atanh(d, boundary_gap.to_f64()?) + atanh(d / b.to_f64()?, internal_gap.to_f64()?)),
    )
}

/// Rational lower bound on entropy per excursion for a physical (m,r,n),
/// using one reversible supporting plane 0 ≤ a < 1. A zero value means that
/// this plane gives no positive certificate. Uses arbitrary precision.
pub fn support_plane_entropy(m: Ratio, r: Ratio, n: Ratio, a: Ratio) -> Option<BigRational> {
    if [m, r, n, a].iter().any(|x| x.den <= 0) {
        return None;
    }
    let [m, r, n, a] = [m, r, n, a].map(BigRational::from);
    let zero = BigRational::zero();
    let one = BigRational::one();
    if a < zero || a >= one {
        return None;
    }
    if m.is_zero() && r.is_zero() && n.is_zero() {
        return Some(zero);
    }
    if r <= zero || r > m || m >= one || n <= &m + &r {
        return None;
    }
    let d = &one + &a - a.pow(2);
    let beta = &one + integer(2) * &a - a.pow(2);
    let alpha = integer(2) * d.pow(2) - &beta;
    let scale = (&one + &a).pow(2) * (&one - a.pow(2)).pow(2);
    let gap = alpha * m + &beta * r - integer(2) * a.pow(6) - scale * n;
    Some(integer(32) * gap.max(zero) / (integer(11) * beta))
}

/// Rational entropy lower bound from the first two count tails alone.
/// Valid for any finite hidden order and independent of the mean count.
pub fn two_tail_entropy(m: Ratio, r: Ratio) -> Option<BigRational> {
    if m.den <= 0 || r.den <= 0 {
        return None;
    }
    let [m, r] = [m, r].map(BigRational::from);
    let zero = BigRational::zero();
    if m.is_zero() && r.is_zero() {
        return Some(zero);
    }
    if r <= zero || r > m || m >= BigRational::one() {
        return None;
    }
    let gap = (m.pow(2) - r).max(zero);
    let cost = (integer(2) * m).max(integer(11) / integer(8));
    Some(integer(4) * gap / cost)
}
