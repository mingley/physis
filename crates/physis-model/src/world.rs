//! A world snapshot: what a theory currently says exists.

use serde::{Deserialize, Serialize};

use crate::gauge::GaugeGroup;
use crate::particle::Spectrum;
use crate::spacetime::Manifold;

/// Projection of a theory into the mechanical layers.
///
/// Agents compare worlds, not slogans. Two theories with the same knobs
/// may still project different worlds; two different knobs may project
/// indistinguishable low-energy worlds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct World {
    /// Spacetime layer.
    pub spacetime: Manifold,
    /// Gauge structure at the theory's fundamental / GUT-ish level.
    pub gauge: GaugeGroup,
    /// Low-energy particle content the theory claims.
    pub spectrum: Spectrum,
    /// Does the theory include a massless spin-2 (gravity)?
    pub has_gravity: bool,
    /// Does the theory include supersymmetry as a structural ingredient?
    pub supersymmetric: bool,
    /// Rough count of independent continuous parameters after consistency
    /// (a predictivity knob — heuristic).
    pub free_parameter_count: u32,
    /// Log₁₀ of an estimated vacuum count (0 = unique). Heuristic.
    pub landscape_log10: f64,
    /// One-line description of this projection.
    pub note: String,
}

impl World {
    /// Observed world as a target: 4D Lorentzian, SM gauge, SM spectrum,
    /// gravity yes, SUSY not required, many SM parameters, unique spacetime.
    pub fn empirical_target() -> Self {
        Self {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::standard_model(),
            spectrum: Spectrum::standard_model(),
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: 19,
            landscape_log10: 0.0,
            note: "Empirical target: 3+1 Lorentzian, SM, gravity, no required SUSY.".into(),
        }
    }
}
