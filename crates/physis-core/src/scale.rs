//! Characteristic length scales of modern physics.
//!
//! The "smallest level modern physics currently understands" is not a
//! single object. Empirically confirmed description currently bottoms
//! out at quantum fields of the Standard Model (quarks, leptons, gauge
//! bosons, Higgs) on a classical 3+1 spacetime. Strings, loops, and
//! other Planck-scale pictures are *hypotheses* — first-class theories
//! in this workspace, not smuggled in as substrate.

use crate::dim::Length;
use crate::qty::{meters, Qty};

/// Named scale, with a typical length for orientation (order of magnitude).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scale {
    /// Planck length, ~1.6×10⁻³⁵ m. Quantum gravity. Not empirically accessed.
    Planck,
    /// Grand-unification folklore, ~10⁻³¹ m / ~10¹⁶ GeV. Hypothetical.
    Gut,
    /// Electroweak, ~10⁻¹⁸ m. W, Z, Higgs — empirically confirmed.
    Electroweak,
    /// QCD / nucleon, ~10⁻¹⁵ m.
    Qcd,
    /// Atomic, ~10⁻¹⁰ m.
    Atomic,
    /// Human / everyday, ~1 m.
    Human,
    /// Astronomical (AU-ish), ~10¹¹ m.
    Astronomical,
    /// Cosmological (Gpc-ish), ~10²⁶ m.
    Cosmological,
}

impl Scale {
    /// Typical length (order of magnitude, SI meters).
    pub fn typical_length(self) -> Qty<Length> {
        match self {
            Scale::Planck => meters(1.616255e-35),
            Scale::Gut => meters(1e-31),
            Scale::Electroweak => meters(1e-18),
            Scale::Qcd => meters(1e-15),
            Scale::Atomic => meters(1e-10),
            Scale::Human => meters(1.0),
            Scale::Astronomical => meters(1.5e11),
            Scale::Cosmological => meters(1e26),
        }
    }

    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Scale::Planck => "planck",
            Scale::Gut => "gut",
            Scale::Electroweak => "electroweak",
            Scale::Qcd => "qcd",
            Scale::Atomic => "atomic",
            Scale::Human => "human",
            Scale::Astronomical => "astronomical",
            Scale::Cosmological => "cosmological",
        }
    }

    /// Whether this scale has direct experimental contact today.
    pub const fn empirically_accessed(self) -> bool {
        matches!(
            self,
            Scale::Electroweak
                | Scale::Qcd
                | Scale::Atomic
                | Scale::Human
                | Scale::Astronomical
                | Scale::Cosmological
        )
    }

    /// Finest empirically confirmed scale in this encoding.
    pub const fn finest_empirical() -> Scale {
        Scale::Electroweak
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planck_is_not_empirical() {
        assert!(!Scale::Planck.empirically_accessed());
        assert!(Scale::Electroweak.empirically_accessed());
        assert_eq!(Scale::finest_empirical(), Scale::Electroweak);
    }

    #[test]
    fn ordering_of_lengths() {
        assert!(Scale::Planck.typical_length().value() < Scale::Gut.typical_length().value());
        assert!(Scale::Gut.typical_length().value() < Scale::Electroweak.typical_length().value());
        assert!(
            Scale::Human.typical_length().value() < Scale::Cosmological.typical_length().value()
        );
    }
}
