//! # physis-theory
//!
//! Theories as first-class objects. A theory is a bundle of knobs, a
//! projection into a [`physis_model::World`], and a list of claims it
//! is willing to be judged on.
//!
//! The first lab is `critique::string_critique`: superstring constructions,
//! the bosonic string, the Standard Model, GR, and an observer-geometry
//! scaffold that encodes the *shape* of landscape critiques (including
//! those associated with Eric Weinstein) without pretending to implement
//! Geometric Unity.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod blackbody;
pub mod claims;
pub mod computation;
pub mod continuum;
pub mod critique;
pub mod dec;
pub mod em;
pub mod framework;
pub mod gauge_field;
pub mod geometry;
pub mod gravity;
pub mod gut;
pub mod quantum;
pub mod relativity;
pub mod rge;
pub mod solid;
pub mod special_relativity;
pub mod standard_model;
pub mod strings;
pub mod target;
pub mod thermo;

pub use blackbody::{blackbody, Blackbody};
pub use computation::computation;
pub use continuum::field_modes;
pub use critique::{string_critique, ExperimentReport, TheoryReport, VerdictDiff};
pub use dec::DeRham;
pub use em::em_vacuum;
pub use framework::Theory;
pub use gauge_field::gauge_lattice;
pub use geometry::ObserverGeometry;
pub use gravity::{gravity, NewtonianGravity};
pub use gut::Su5Gut;
pub use quantum::bell;
pub use relativity::GeneralRelativity;
pub use solid::{solid, EinsteinSolid};
pub use special_relativity::SpecialRelativity;
pub use standard_model::StandardModel;
pub use strings::{StringKind, StringTheory};
pub use target::{empirical_target, score, EmpiricalTarget, Scorecard};
pub use thermo::thermodynamics;
