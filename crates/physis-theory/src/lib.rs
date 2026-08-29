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

pub mod claims;
pub mod computation;
pub mod continuum;
pub mod critique;
pub mod em;
pub mod framework;
pub mod gauge_field;
pub mod geometry;
pub mod relativity;
pub mod standard_model;
pub mod strings;
pub mod target;
pub mod thermo;

pub use computation::computation;
pub use continuum::field_modes;
pub use critique::{string_critique, ExperimentReport, TheoryReport, VerdictDiff};
pub use em::em_vacuum;
pub use framework::Theory;
pub use gauge_field::gauge_lattice;
pub use geometry::ObserverGeometry;
pub use relativity::GeneralRelativity;
pub use standard_model::StandardModel;
pub use strings::{StringKind, StringTheory};
pub use target::{empirical_target, score, EmpiricalTarget, Scorecard};
pub use thermo::thermodynamics;
