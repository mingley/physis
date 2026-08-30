//! # physis-core
//!
//! The substrate: quantities the compiler will not let you add incorrectly,
//! layers of description, knobs that theories expose, and claims that can
//! be judged against a world.
//!
//! Adding mass to length is a type error:
//!
//! ```compile_fail
//! use physis_core::qty::{kg, meters};
//! let _ = kg(1.0) + meters(1.0);
//! ```
//!
//! Subtracting a length from an energy is also rejected:
//!
//! ```compile_fail
//! use physis_core::qty::{joule, meters};
//! let _ = joule(1.0) - meters(1.0);
//! ```
//!
//! And a wrong-dimension result cannot be assigned: mass × time is not energy:
//!
//! ```compile_fail
//! use physis_core::dim::Energy;
//! use physis_core::qty::{kg, seconds, Qty};
//! let _: Qty<Energy> = kg(1.0) * seconds(1.0);
//! ```
//!
//! Energy density (J/m³) is not energy (J):
//!
//! ```compile_fail
//! use physis_core::dim::{Energy, EnergyDensity};
//! use physis_core::qty::Qty;
//! let _: Qty<Energy> = Qty::<EnergyDensity>::new(1.0);
//! ```
//!
//! Heat capacity (J/K) is not energy (J):
//!
//! ```compile_fail
//! use physis_core::dim::{Energy, HeatCapacity};
//! use physis_core::qty::Qty;
//! let _: Qty<Energy> = Qty::<HeatCapacity>::new(1.0);
//! ```
//!
//! Irradiance (W/m²) is not energy (J):
//!
//! ```compile_fail
//! use physis_core::dim::{Energy, Irradiance};
//! use physis_core::qty::Qty;
//! let _: Qty<Energy> = Qty::<Irradiance>::new(1.0);
//! ```
//!
//! Luminosity density (W/m³) is not irradiance (W/m²):
//!
//! ```compile_fail
//! use physis_core::dim::{Irradiance, LuminosityDensity};
//! use physis_core::qty::Qty;
//! let _: Qty<Irradiance> = Qty::<LuminosityDensity>::new(1.0);
//! ```
//!
//! Energy from mass and velocity is a type success:
//!
//! ```
//! use physis_core::qty::{joule, kg, meters_per_second};
//! let m = kg(2.0);
//! let v = meters_per_second(3.0);
//! let k = m * v * v * 0.5;
//! assert!((k.value() - joule(9.0).value()).abs() < 1e-12);
//! ```
//!
//! There is no `Epistemic::Theorem` and no `DerivationAssurance::MachineProved`.
//! A kernel proof cannot be created by setting an enum:
//!
//! ```compile_fail
//! use physis_core::assurance::DerivationAssurance;
//! let _ = DerivationAssurance::MachineProved;
//! ```
//!
//! ```compile_fail
//! use physis_core::claim::Epistemic;
//! let _ = Epistemic::Theorem;
//! ```
//!
//! P3F cannot be deserialized or written as a struct literal:
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_core::judgment::TrustProfile>();
//! }
//! ```

#![forbid(unsafe_code)]

pub mod artifact;
pub mod assumption;
pub mod assurance;
pub mod axiom;
pub mod claim;
pub mod dim;
pub mod error;
pub mod formal;
pub mod id;
pub mod judgment;
pub mod knob;
pub mod layer;
pub mod qty;
pub mod scale;

pub use artifact::ArtifactId;
pub use assumption::{Assumption, AssumptionSet, AssumptionSetId, DomainOfValidity};
pub use assurance::{ClaimClass, DerivationAssurance, EmpiricalStatus, SemanticAssurance};
pub use axiom::{AxiomClass, AxiomId, AxiomLedger, AxiomRecord, ReviewStatus};
pub use claim::{Claim, Verdict, VerdictKind};
pub use dim::{
    Acceleration, Action, Amount, Charge, Current, Dimensionless, Energy, EnergyDensity, Force,
    Frequency, HeatCapacity, Irradiance, Length, LengthTemperature, LuminosityDensity, Luminous,
    Mass, Momentum, Power, Pressure, RadiationConstant, SpectralEnergyDensity, StefanBoltzmann,
    Temperature, Time, Velocity, SI,
};
pub use error::CoreError;
pub use formal::FormalClaim;
pub use id::{ClaimId, KnobId, LayerId, TheoryId};
pub use judgment::{
    EmpiricalJudgment, GapReason, HeuristicJudgment, Judgment, LogicalJudgment, NumericJudgment,
    ParameterOrigin, StatisticalJudgment, TrustEvidence, TrustProfile, TrustTier,
};
pub use knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
pub use layer::Layer;
pub use qty::Qty;
pub use scale::Scale;
