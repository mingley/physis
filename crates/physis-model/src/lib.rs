//! # physis-model
//!
//! Mechanical layers of a world: spacetime, quantum amplitudes, particles,
//! gauge groups, and a snapshot `World` theories project into.
//!
//! This is not a PDE engine. It is a *typed ontology* with enough dynamics
//! (finite Hilbert spaces, Coulomb force, Lorentz factor) to make knobs
//! produce observable state changes.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod complex;
pub mod constants;
pub mod gauge;
pub mod particle;
pub mod quantum;
pub mod spacetime;
pub mod world;

pub use complex::Complex;
pub use gauge::{Embed, GaugeGroup, SimpleGroup};
pub use particle::{EmpiricalStatus, Flavor, Species, Spectrum};
pub use quantum::{pauli_x, pauli_y, pauli_z, Hilbert, Ket};
pub use spacetime::{Manifold, SignConvention, Signature, Topology};
pub use world::World;
