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
//! Energy from mass and velocity is a type success:
//!
//! ```
//! use physis_core::qty::{joule, kg, meters_per_second};
//! let m = kg(2.0);
//! let v = meters_per_second(3.0);
//! let k = m * v * v * 0.5;
//! assert!((k.value() - joule(9.0).value()).abs() < 1e-12);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod claim;
pub mod dim;
pub mod error;
pub mod id;
pub mod knob;
pub mod layer;
pub mod qty;
pub mod scale;

pub use claim::{Claim, Epistemic, Verdict, VerdictKind};
pub use dim::{
    Acceleration, Amount, Charge, Current, Dimensionless, Energy, Force, Frequency, Length,
    Luminous, Mass, Power, Pressure, Temperature, Time, Velocity, SI,
};
pub use error::CoreError;
pub use id::{ClaimId, KnobId, LayerId, TheoryId};
pub use knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
pub use layer::Layer;
pub use qty::Qty;
pub use scale::Scale;
