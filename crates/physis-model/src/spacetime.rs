//! Spacetime: dimension, signature, extra directions, topology hints.
//!
//! Extra dimensions are first-class knobs, not a string-theory-only idea.
//! A 3+1 Lorentzian manifold is the empirically confirmed default.

use physis_core::id::LayerId;
use physis_core::layer::Layer;
use serde::{Deserialize, Serialize};

/// Mostly-plus (`-+++`) or mostly-minus (`+---`) convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignConvention {
    /// Time negative, space positive (`-+++`). Particle-physics common.
    MostlyPlus,
    /// Time positive, space negative (`+---`). GR common.
    MostlyMinus,
}

/// `(time, space)` signature. Dimension is `time + space`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signature {
    /// Number of time directions (almost always 1).
    pub time: u8,
    /// Number of spatial directions (3 observed; more in some theories).
    pub space: u8,
}

impl Signature {
    /// Observed 3+1 Minkowski.
    pub const MINKOWSKI_4: Self = Self { time: 1, space: 3 };

    /// Total dimension.
    pub const fn dim(self) -> u16 {
        self.time as u16 + self.space as u16
    }

    /// Lorentzian: exactly one time direction.
    pub const fn is_lorentzian(self) -> bool {
        self.time == 1 && self.space >= 1
    }

    /// Euclidean: no time direction.
    pub const fn is_euclidean(self) -> bool {
        self.time == 0
    }
}

/// Compactification / topology hint. Not a full atlas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Topology {
    /// R^{t,s}, no compact directions.
    Minkowski,
    /// Product of Minkowski with a compact Riemannian manifold.
    ProductCompact,
    /// Torus of extra directions (the simplest compactification).
    Torus,
    /// Calabi–Yau-like (string compactification folklore).
    CalabiYau,
    /// Unknown / unspecified.
    Unspecified,
}

/// A manifold as this workspace understands it: dimension, signature,
/// how many directions are compact, and a topology hint.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifold {
    /// Total dimension (time + space).
    pub dim: u8,
    /// Signature.
    pub signature: Signature,
    /// Number of compact extra spatial directions (`dim - 4` in usual 4D reductions).
    pub compact_extra: u8,
    /// Compactification radius in Planck lengths. Ignored if `compact_extra == 0`.
    pub compact_radius_planck: f64,
    /// Topology hint.
    pub topology: Topology,
    /// Metric sign convention.
    pub convention: SignConvention,
}

impl Manifold {
    /// Empirically confirmed default: 3+1, nothing compact, Minkowski.
    pub fn observed_4d() -> Self {
        Self {
            dim: 4,
            signature: Signature::MINKOWSKI_4,
            compact_extra: 0,
            compact_radius_planck: 0.0,
            topology: Topology::Minkowski,
            convention: SignConvention::MostlyPlus,
        }
    }

    /// Superstring spacetime: 9+1, 6 compact.
    pub fn superstring_10() -> Self {
        Self {
            dim: 10,
            signature: Signature { time: 1, space: 9 },
            compact_extra: 6,
            compact_radius_planck: 1.0,
            topology: Topology::CalabiYau,
            convention: SignConvention::MostlyPlus,
        }
    }

    /// Bosonic string: 25+1, 22 compact.
    pub fn bosonic_26() -> Self {
        Self {
            dim: 26,
            signature: Signature { time: 1, space: 25 },
            compact_extra: 22,
            compact_radius_planck: 1.0,
            topology: Topology::Torus,
            convention: SignConvention::MostlyPlus,
        }
    }

    /// M-theory: 10+1, 7 compact.
    pub fn m_theory_11() -> Self {
        Self {
            dim: 11,
            signature: Signature { time: 1, space: 10 },
            compact_extra: 7,
            compact_radius_planck: 1.0,
            topology: Topology::Unspecified,
            convention: SignConvention::MostlyPlus,
        }
    }

    /// Observed non-compact dimension (total minus compact extra).
    pub fn observed_dim(self) -> i16 {
        self.dim as i16 - self.compact_extra as i16
    }

    /// Internal consistency of the numbers.
    pub fn structurally_ok(self) -> bool {
        self.signature.dim() == self.dim as u16
            && self.compact_extra < self.dim
            && (self.compact_extra == 0 || self.compact_radius_planck > 0.0)
            && self.signature.is_lorentzian()
    }
}

impl Layer for Manifold {
    const ID: LayerId = LayerId::Spacetime;
    type Observable = Manifold;
    fn observe(&self) -> Self::Observable {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_4d_is_ok() {
        let m = Manifold::observed_4d();
        assert!(m.structurally_ok());
        assert_eq!(m.observed_dim(), 4);
    }

    #[test]
    fn superstring_has_six_extra() {
        let m = Manifold::superstring_10();
        assert!(m.structurally_ok());
        assert_eq!(m.observed_dim(), 4);
        assert_eq!(m.compact_extra, 6);
    }
}
