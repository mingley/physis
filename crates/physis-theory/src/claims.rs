//! Shared claim identifiers so experiments can build comparison matrices.

use physis_core::claim::{Claim, Epistemic};
use physis_core::id::LayerId;

/// Spacetime numbers are internally consistent (dim = time + space, …).
pub const SPACETIME_STRUCTURE: &str = "consistency.spacetime-structure";
/// Critical dimension of a worldsheet / membrane theory.
pub const CRITICAL_DIMENSION: &str = "consistency.critical-dimension";
/// Superstring construction requires supersymmetry.
pub const SUSY_CONSTRUCTION: &str = "consistency.susy-construction";
/// No tachyon in the spectrum of the construction.
pub const NO_TACHYON: &str = "consistency.no-tachyon";
/// Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D).
pub const ANOMALY_CANCELLATION: &str = "consistency.anomaly-cancellation";

/// Macroscopic spacetime is 3+1.
pub const OBSERVED_4D: &str = "empirical.observed-4d";
/// Extra dimensions are hidden at currently accessed scales.
pub const HIDDEN_EXTRA_DIMS: &str = "empirical.hidden-extra-dims";
/// Low-energy fermions exist.
pub const FERMIONS: &str = "empirical.fermions";
/// Gauge group contains the Standard Model.
pub const SM_GAUGE: &str = "empirical.sm-gauge";
/// Three generations of charged leptons.
pub const THREE_GENERATIONS: &str = "empirical.three-generations";
/// Neutrinos have nonzero mass (as oscillation experiments require).
pub const NEUTRINO_MASSES: &str = "empirical.neutrino-masses";
/// Gravity (massless spin-2) is present.
pub const GRAVITY: &str = "empirical.gravity";

/// Unique vacuum / no landscape.
pub const UNIQUE_VACUUM: &str = "predictivity.unique-vacuum";
/// Number of free parameters is small.
pub const FEW_PARAMETERS: &str = "predictivity.few-parameters";
/// The theory is a UV completion of gravity + QFT.
pub const UV_COMPLETION: &str = "predictivity.uv-completion";

/// Build a claim with a shared id.
pub fn c(id: &str, statement: &str, layer: LayerId, epistemic: Epistemic) -> Claim {
    Claim::new(id, statement, layer, epistemic)
}

/// The comparison rows used by the string-critique lab.
pub fn critique_rows() -> [&'static str; 13] {
    [
        SPACETIME_STRUCTURE,
        CRITICAL_DIMENSION,
        SUSY_CONSTRUCTION,
        NO_TACHYON,
        ANOMALY_CANCELLATION,
        OBSERVED_4D,
        HIDDEN_EXTRA_DIMS,
        FERMIONS,
        SM_GAUGE,
        THREE_GENERATIONS,
        GRAVITY,
        UNIQUE_VACUUM,
        UV_COMPLETION,
    ]
}
