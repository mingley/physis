//! Exact equilibrium feasibility for two hidden-jump tails and their mean.
//! The cubic decision and the constructive theorem are derived in
//! docs/research/excursion-equilibrium/RESULT.md.

use num_bigint::BigInt;
use physis_numeric::Ratio;
use std::cmp::Ordering;

/// Conclusion for a finite, reciprocal, stationary chain with one visible state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EquilibriumFit {
    /// A finite reversible network realizes the three observations exactly.
    Attained,
    /// Exact fits can approach zero dissipation arbitrarily closely. Each dissipates.
    ZeroInfimum,
    /// Every physical realization has an entropy floor bounded away from zero.
    PositiveFloor,
    /// The inputs violate elementary count constraints.
    InvalidCounts,
}

impl EquilibriumFit {
    /// Stable text for the research replay.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attained => "equilibrium-attained",
            Self::ZeroInfimum => "zero-infimum-unattained",
            Self::PositiveFloor => "positive-entropy-floor",
            Self::InvalidCounts => "invalid-counts",
        }
    }
}

/// Classify exact population inputs m=P(N≥1), r=P(N≥2), n=E[N].
/// PositiveFloor is conditional on physical realizability. The domain checks
/// are necessary count constraints, not a decision procedure for arbitrary driven fits.
pub fn equilibrium_fit(m: Ratio, r: Ratio, n: Ratio) -> EquilibriumFit {
    use EquilibriumFit::*;
    if [m, r, n].iter().any(|x| x.den <= 0) {
        return InvalidCounts;
    }
    // Work in a single positive integer scale. If P is the rational
    // decision cubic, the coefficients below represent L² P(Lz).
    // Root signs and multiplicities survive the rescaling, without a GCD
    // after every product in the discriminant.
    let mr = BigInt::from(m.den) * r.den;
    let scale = &mr * n.den;
    let (m, r, n) = (
        BigInt::from(m.num) * r.den * n.den,
        BigInt::from(r.num) * m.den * n.den,
        BigInt::from(n.num) * mr,
    );
    let zero = BigInt::ZERO;
    if m == zero && r == zero && n == zero {
        return Attained;
    }
    if m <= zero || m >= scale || r <= zero || r > m || n <= &m + &r {
        return InvalidCounts;
    }
    let squared = m.pow(2);
    let variance = &scale * &r - &squared;
    if variance < zero {
        return PositiveFloor;
    }
    if variance == zero {
        return match (&n * (&scale - &m)).cmp(&(&m * &scale)) {
            Ordering::Less => PositiveFloor,
            Ordering::Equal => Attained,
            Ordering::Greater => ZeroInfimum,
        };
    }
    let d = &m - &r;
    if d == zero {
        return if &n * (&scale - &m) >= 2 * &m * &scale {
            Attained
        } else {
            PositiveFloor
        };
    }
    let e = squared - n * &d;
    if e <= zero {
        return Attained;
    }
    let k: BigInt = scale - 2 * m + &r;
    let rv = r * &variance;
    let a: BigInt = 2 * d.pow(3) * &variance;
    let b: BigInt = k.pow(2) * &e + variance.pow(2) - 2 * &rv * (&k - d);
    let c: BigInt = 2 * (k * &e - rv);
    if b >= zero && c >= zero {
        return PositiveFloor;
    }
    // There is one negative root. With a negative intermediate coefficient,
    // a nonnegative discriminant is equivalent to two positive real roots
    // counted with multiplicity. A double root attains the frontier.
    let discriminant = 18 * &a * &b * &c * &e - 4 * b.pow(3) * &e + b.pow(2) * c.pow(2)
        - 4 * &a * c.pow(3)
        - 27 * a.pow(2) * e.pow(2);
    if discriminant >= zero {
        Attained
    } else {
        PositiveFloor
    }
}
