//! Exact equilibrium feasibility for two hidden-jump tails and their mean.
//! The cubic decision and the constructive theorem are derived in
//! docs/research/excursion-equilibrium/RESULT.md.

use num_rational::BigRational;
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
    // The discriminant of a ring in the supported grid already exceeds i128.
    let [m, r, n] = [m, r, n].map(BigRational::from);
    let int = |x: i32| BigRational::from_integer(x.into());
    let zero = int(0);
    let one = int(1);
    if m == zero && r == zero && n == zero {
        return Attained;
    }
    if m <= zero || m >= one || r <= zero || r > m || n <= &m + &r {
        return InvalidCounts;
    }
    let variance = &r - m.pow(2);
    if variance < zero {
        return PositiveFloor;
    }
    if variance == zero {
        return match (&n * (&one - &m)).cmp(&m) {
            Ordering::Less => PositiveFloor,
            Ordering::Equal => Attained,
            Ordering::Greater => ZeroInfimum,
        };
    }
    let d = &m - &r;
    if d == zero {
        return if &n * (&one - &m) >= int(2) * &m {
            Attained
        } else {
            PositiveFloor
        };
    }

    // The mean of a geometric branch mixed with an immediate return is m²/d.
    // Above the minimum the second branch can continuously lengthen the mean.
    let constant = m.pow(2) - &n * &d;
    if constant <= zero {
        return Attained;
    }
    let k = one - int(2) * &m + &r;
    let a = int(2) * d.pow(3) * &variance;
    let b = k.pow(2) * &constant + variance.pow(2) - int(2) * &r * &variance * (&k - &d);
    let c = int(2) * (&k * &constant - &r * &variance);
    if b >= zero && c >= zero {
        return PositiveFloor;
    }
    // Positive leading and constant coefficients force one negative root.
    // With a negative intermediate coefficient, the other two are positive
    // iff all roots are real. A repeated positive root attains the frontier.
    let discriminant = int(18) * &a * &b * &c * &constant - int(4) * b.pow(3) * &constant
        + b.pow(2) * c.pow(2)
        - int(4) * &a * c.pow(3)
        - int(27) * a.pow(2) * constant.pow(2);
    if discriminant >= zero {
        Attained
    } else {
        PositiveFloor
    }
}
